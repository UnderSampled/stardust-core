use anyhow::anyhow;
use slotmap::{DefaultKey, Key, KeyData, SlotMap};
use std::io::Result;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::unix::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::UnixStream;
use tokio::runtime::Handle;
use tokio::sync::Mutex;
use tokio::sync::{mpsc, oneshot};

use crate::{
	scenegraph,
	schemas::message::{root_as_message, Message, MessageArgs},
};

pub struct Messenger {
	async_rt: Handle,
	read: Mutex<OwnedReadHalf>,
	write: Mutex<OwnedWriteHalf>,
	send_queue_tx: mpsc::UnboundedSender<Vec<u8>>,
	send_queue_rx: Mutex<mpsc::UnboundedReceiver<Vec<u8>>>,
	pending_method_futures: Mutex<SlotMap<DefaultKey, oneshot::Sender<anyhow::Result<Vec<u8>>>>>,
}

impl Messenger {
	pub fn new(async_rt: Handle, connection: UnixStream) -> Self {
		let (read, write) = connection.into_split();
		let (send_queue_tx, send_queue_rx) = mpsc::unbounded_channel();
		Self {
			async_rt,
			read: Mutex::new(read),
			write: Mutex::new(write),
			send_queue_tx,
			send_queue_rx: Mutex::new(send_queue_rx),
			pending_method_futures: Mutex::new(Default::default()),
		}
	}

	//let flex_root = flexbuffers::Reader::get_root(message.unwrap()).unwrap();
	pub fn error<T: std::fmt::Display>(&self, object: &str, method: &str, err: T) -> Result<()> {
		let error = format!("{}", err);
		self.send_call(0, None, object, method, Some(error.as_str()), None)
	}
	pub fn send_remote_signal(&self, object: &str, method: &str, data: &[u8]) -> Result<()> {
		self.send_call(1, None, object, method, None, Some(data))
	}
	pub async fn execute_remote_method(
		&self,
		object: &str,
		method: &str,
		data: &[u8],
	) -> anyhow::Result<Vec<u8>> {
		let (tx, rx) = oneshot::channel();
		let id = self.pending_method_futures.lock().await.insert(tx);
		let num_id = id.data().as_ffi();
		if let Err(err) = self.send_call(2, Some(num_id), object, method, None, Some(data)) {
			let _ = self
				.pending_method_futures
				.lock()
				.await
				.remove(id)
				.unwrap()
				.send(Err(err.into()));
		}
		rx.await?
	}
	fn send_call(
		&self,
		call_type: u8,
		id: Option<u64>,
		path: &str,
		method: &str,
		err: Option<&str>,
		data: Option<&[u8]>,
	) -> Result<()> {
		let mut fbb = flatbuffers::FlatBufferBuilder::with_capacity(1024);
		let flex_path = fbb.create_string(path);
		let flex_method = fbb.create_string(method);
		let flex_err = err.map(|s| fbb.create_string(s));
		let flex_data = data.map(|s| fbb.create_vector(s));

		let message_constructed = Message::create(
			&mut fbb,
			&MessageArgs {
				type_: call_type,
				id: id.unwrap_or(0),
				object: Some(flex_path),
				method: Some(flex_method),
				error: flex_err,
				data: flex_data,
			},
		);
		fbb.finish(message_constructed, None);

		let message_length = fbb.finished_data().len() as u32;
		let message_length = message_length.to_ne_bytes().to_vec();
		let message = fbb.finished_data().to_vec();

		self.send_queue_tx.send(message_length).unwrap();
		self.send_queue_tx.send(message).unwrap();
		Ok(())
	}

	async fn handle_message(
		&self,
		message: Vec<u8>,
		scenegraph: &impl scenegraph::Scenegraph,
	) -> Result<()> {
		let message = root_as_message(&message).unwrap();
		let message_type = message.type_();
		match message_type {
			// Errors
			0 => {
				let key: DefaultKey = KeyData::from_ffi(message.id()).into();
				let future_opt = self.pending_method_futures.lock().await.remove(key);
				match future_opt {
					None => {
						eprintln!(
							"[Stardust XR][{}:{}] {}",
							message.object().unwrap_or("unknown"),
							message.method().unwrap_or("unknown"),
							message.error().unwrap_or("unknown"),
						)
					}
					Some(future) => {
						let _ = future.send(Err(anyhow!(message
							.error()
							.unwrap_or("unknown")
							.to_string())));
					}
				}
			}
			// Signals
			1 => {
				let signal_status = scenegraph.send_signal(
					message.object().unwrap(),
					message.method().unwrap(),
					message.data().unwrap(),
				);
				if let Err(e) = signal_status {
					self.error(message.object().unwrap(), message.method().unwrap(), e)
						.ok();
				}
			}
			// Method called
			2 => {
				let method_result = scenegraph.execute_method(
					message.object().unwrap(),
					message.method().unwrap(),
					message.data().unwrap(),
				);
				match method_result {
					Ok(return_value) => self.send_call(
						3,
						Some(message.id()),
						message.object().unwrap(),
						message.method().unwrap(),
						None,
						Some(&return_value),
					)?,
					Err(error) => {
						self.error(message.object().unwrap(), message.method().unwrap(), error)
							.ok();
					}
				};
			}
			// Method return
			3 => {
				let key: DefaultKey = KeyData::from_ffi(message.id()).into();
				let future_opt = self.pending_method_futures.lock().await.remove(key);
				match future_opt {
					None => {
						self.error(
							message.object().unwrap(),
							message.method().unwrap(),
							anyhow!("Method return without method call"),
						)?;
					}
					Some(future) => {
						let _ = future.send(Ok(message.data().unwrap().to_vec()));
					}
				}
			}
			_ => println!("Type is wayyy off"),
		}
		Ok(())
	}

	pub async fn dispatch(&self, scenegraph: &impl scenegraph::Scenegraph) -> Result<()> {
		let mut connection = self.read.lock().await;

		let mut message_length_buffer: [u8; 4] = [0; 4];
		connection.read_exact(&mut message_length_buffer).await?;
		let message_length: u32 = u32::from_ne_bytes(message_length_buffer);

		let mut message_buffer: Vec<u8> = std::vec::from_elem(0_u8, message_length as usize);
		connection.read_exact(message_buffer.as_mut_slice()).await?;

		drop(connection);
		self.handle_message(message_buffer, scenegraph).await?;
		Ok(())
	}

	pub async fn flush(&self) -> Result<()> {
		let (mut write, mut send_queue) = tokio::join! {
			self.write.lock(),
			self.send_queue_rx.lock()
		};
		while let Some(message) = send_queue.recv().await {
			write.write_all(message.as_slice()).await?
		}
		Ok(())
	}
}
