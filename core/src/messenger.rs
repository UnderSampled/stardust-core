//! Symmetrical messenger for both client and server.

use crate::scenegraph;
use rustc_hash::FxHashMap;
use stardust_xr_schemas::flat::flatbuffers::{self, InvalidFlatbuffer};
use stardust_xr_schemas::flat::message::{root_as_message, Message as FlatMessage, MessageArgs};
use stardust_xr_schemas::flex::flexbuffers;
use std::future::Future;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use thiserror::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::unix::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::UnixStream;
use tokio::sync::{mpsc, oneshot};
use tracing::instrument;

#[instrument(level = "trace", skip_all)]
fn debug_call(
	incoming: bool,
	call_type: u8,
	id: Option<u64>,
	path: Option<&str>,
	method: Option<&str>,
	err: Option<&str>,
	data: &[u8],
) {
	let level = match call_type {
		0 => tracing::Level::WARN,
		_ => tracing::Level::DEBUG,
	};

	if tracing::level_enabled!(level) {
		let call_type = match call_type {
			0 => "error",
			1 => "signal",
			2 => "method call",
			3 => "method return",
			_ => "unknown",
		};
		let data = match flexbuffers::Reader::get_root(data) {
			Ok(root) => root.to_string(),
			Err(_) => String::from_utf8_lossy(data).into_owned(),
		};

		match level {
			tracing::Level::WARN => {
				tracing::warn!(
					source = match incoming {
						true => "remote",
						false => "local",
					},
					id,
					path,
					method,
					err,
					data,
					"Stardust error",
				)
			}
			_ => {
				tracing::trace!(
					direction = match incoming {
						true => "incoming",
						false => "outgoing",
					},
					call_type,
					id,
					path,
					method,
					err,
					data,
					"Stardust message",
				)
			}
		}
	}
}

/// Error for all messenger-related failures.
#[derive(Error, Debug)]
pub enum MessengerError {
	/// The MessageReceiver has been dropped with pending futures
	#[error("Receiver has been dropped")]
	ReceiverDropped,
	#[error("IO Error: {0}")]
	IOError(std::io::Error),
	/// The incoming message is corrupted
	#[error("Invalid flatbuffer {0}")]
	InvalidFlatbuffer(InvalidFlatbuffer),
	/// The message type u8 is greater than method return (3)
	#[error("Message type is out of bounds")]
	MessageTypeOutOfBounds,
}
impl From<std::io::Error> for MessengerError {
	fn from(e: std::io::Error) -> Self {
		MessengerError::IOError(e)
	}
}
impl From<InvalidFlatbuffer> for MessengerError {
	fn from(e: InvalidFlatbuffer) -> Self {
		MessengerError::InvalidFlatbuffer(e)
	}
}

/// Wrapper for messages after being serialized, for type safety.
pub struct Message {
	data: Vec<u8>,
}
impl Message {
	/// Get the data inside to send over the socket.
	pub fn into_data(self) -> Vec<u8> {
		self.data
	}
}

type PendingFuture = oneshot::Sender<Result<Vec<u8>, String>>;
type PendingFutureSender = mpsc::UnboundedSender<(u64, PendingFuture)>;
type PendingFutureReceiver = mpsc::UnboundedReceiver<(u64, PendingFuture)>;

/// Receiving half of the messenger.
pub struct MessageReceiver {
	read: OwnedReadHalf,
	pending_futures: FxHashMap<u64, PendingFuture>,
	pending_future_rx: PendingFutureReceiver,
	send_handle: MessageSenderHandle,
}
impl MessageReceiver {
	fn new(
		read: OwnedReadHalf,
		pending_future_rx: PendingFutureReceiver,
		send_handle: MessageSenderHandle,
	) -> Self {
		MessageReceiver {
			read,
			pending_futures: Default::default(),
			pending_future_rx,
			send_handle,
		}
	}
	/// Take all the pending futures in the queue from method calls and store them for when the other side sends a method return.
	pub fn update_pending_futures(&mut self) {
		while let Ok((id, future)) = self.pending_future_rx.try_recv() {
			let _ = self.pending_futures.insert(id, future);
		}
	}
	/// Await a message from the socket, parse it, and handle it.
	pub async fn dispatch<S: scenegraph::Scenegraph>(
		&mut self,
		scenegraph: &S,
	) -> Result<(), MessengerError> {
		let mut message_length_buffer: [u8; 4] = [0; 4];
		self.read.read_exact(&mut message_length_buffer).await?;
		let message_length: u32 = u32::from_ne_bytes(message_length_buffer);

		let mut message_buffer: Vec<u8> = std::vec::from_elem(0_u8, message_length as usize);
		self.read.read_exact(message_buffer.as_mut_slice()).await?;

		self.update_pending_futures();
		self.handle_message(message_buffer, scenegraph)
	}

	#[instrument(level = "trace", skip_all)]
	fn handle_message<S: scenegraph::Scenegraph>(
		&mut self,
		message: Vec<u8>,
		scenegraph: &S,
	) -> Result<(), MessengerError> {
		let message = root_as_message(&message)?;
		let message_type = message.type_();

		debug_call(
			true,
			message_type,
			Some(message.id()),
			message.object(),
			message.method(),
			message.error(),
			message.data().map(|d| d.bytes()).unwrap_or(&[]),
		);
		let path = message.object().unwrap_or("unknown");
		let method = message.method().unwrap_or("unknown");
		let data = message.data().unwrap_or_default().bytes();
		match message_type {
			// Errors
			0 => {
				let future_opt = self.pending_futures.remove(&message.id());
				if let Some(future) = future_opt {
					let _ = future.send(Err(message.error().unwrap_or("unknown").to_string()));
				}
			}
			// Signals
			1 => {
				let signal_status = scenegraph.send_signal(path, method, data);
				if let Err(e) = signal_status {
					self.send_handle.error(path, method, e, data)?;
				}
			}
			// Method called
			2 => {
				let method_result = scenegraph.execute_method(path, method, data);
				match method_result {
					Ok(return_value) => self.send_handle.send(serialize_call(
						3,
						Some(message.id()),
						path,
						method,
						None,
						&return_value,
					))?,
					Err(error) => self.send_handle.error(path, method, error, data)?,
				};
			}
			// Method return
			3 => {
				let future_opt = self.pending_futures.remove(&message.id());
				match future_opt {
					None => {
						self.send_handle.error(
							path,
							method,
							"Method return without method call".to_string(),
							data,
						)?;
					}
					Some(future) => {
						let _ = future.send(Ok(data.to_vec()));
					}
				}
			}
			_ => println!("Type is wayyy off"),
		}
		Ok(())
	}
}

/// Generate an error message from arguments.
pub fn serialize_error<T: std::fmt::Display>(
	object: &str,
	method: &str,
	err: T,
	data: &[u8],
) -> Message {
	let error = format!("{}", err);
	serialize_call(0, None, object, method, Some(error.as_str()), data)
}
/// Generate a signal message from arguments.
pub fn serialize_signal_call(object: &str, method: &str, data: &[u8]) -> Message {
	serialize_call(1, None, object, method, None, data)
}
/// Generate a method message from arguments.
pub fn serialize_method_call(id: u64, object: &str, method: &str, data: &[u8]) -> Message {
	serialize_call(2, Some(id), object, method, None, data)
}
#[instrument(level = "trace", skip_all)]
fn serialize_call(
	call_type: u8,
	id: Option<u64>,
	path: &str,
	method: &str,
	err: Option<&str>,
	data: &[u8],
) -> Message {
	debug_call(false, call_type, id, Some(path), Some(method), err, data);

	let mut fbb = flatbuffers::FlatBufferBuilder::with_capacity(1024);
	let flex_path = fbb.create_string(path);
	let flex_method = fbb.create_string(method);
	let flex_err = err.map(|s| fbb.create_string(s));
	let flex_data = fbb.create_vector(data);

	let message_constructed = FlatMessage::create(
		&mut fbb,
		&MessageArgs {
			type_: call_type,
			id: id.unwrap_or(0),
			object: Some(flex_path),
			method: Some(flex_method),
			error: flex_err,
			data: Some(flex_data),
		},
	);
	fbb.finish(message_constructed, None);
	Message {
		data: fbb.finished_data().to_vec(),
	}
}

/// Sender half of the messenger
pub struct MessageSender {
	write: OwnedWriteHalf,
	handle: MessageSenderHandle,
	message_rx: mpsc::UnboundedReceiver<Message>,
	pending_future_tx: PendingFutureSender,
	max_message_id: Arc<AtomicU64>,
}
impl MessageSender {
	fn new(write: OwnedWriteHalf, pending_future_tx: PendingFutureSender) -> Self {
		let (message_tx, message_rx) = mpsc::unbounded_channel();
		let max_message_id = Arc::new(AtomicU64::new(0));
		MessageSender {
			write,
			handle: MessageSenderHandle {
				message_tx,
				pending_future_tx: pending_future_tx.clone(),
				max_message_id: max_message_id.clone(),
			},
			message_rx,
			pending_future_tx,
			max_message_id,
		}
	}
	/// Send all the queued messages from the handles
	pub async fn flush(&mut self) -> Result<(), MessengerError> {
		while let Some(message) = self.message_rx.recv().await {
			self.send(message).await?;
		}
		Ok(())
	}
	/// Send a message and await until sent.
	pub async fn send(&mut self, message: Message) -> Result<(), MessengerError> {
		let message = message.into_data();
		let message_length = message.len() as u32;
		self.write.write_all(&message_length.to_ne_bytes()).await?;
		self.write.write_all(&message).await?;
		Ok(())
	}
	/// Get a handle to send messages from anywhere.
	pub fn handle(&self) -> MessageSenderHandle {
		self.handle.clone()
	}

	/// Send an error immediately, await until sent.
	pub async fn error<E: std::fmt::Display>(
		&mut self,
		node_path: &str,
		method_name: &str,
		err: E,
		data: &[u8],
	) -> Result<(), MessengerError> {
		self.send(serialize_error(node_path, method_name, err, data))
			.await
	}
	/// Send a signal immediately, await until sent.
	pub async fn signal(
		&mut self,
		node_path: &str,
		signal_name: &str,
		data: &[u8],
	) -> Result<(), MessengerError> {
		self.send(serialize_signal_call(node_path, signal_name, data))
			.await
	}
	/// Call a method immediately, await until other side sends back a response or the message fails to send.
	pub async fn method(
		&mut self,
		node_path: &str,
		method: &str,
		data: &[u8],
	) -> Result<Result<Vec<u8>, String>, MessengerError> {
		let (tx, rx) = oneshot::channel();
		let id = self.max_message_id.load(Ordering::Relaxed);
		self.pending_future_tx
			.send((id, tx))
			.map_err(|_| MessengerError::ReceiverDropped)?;
		self.send(serialize_method_call(id, node_path, method, data))
			.await?;
		self.max_message_id.store(id + 1, Ordering::Relaxed);
		rx.await.map_err(|_| MessengerError::ReceiverDropped)
	}
}

/// Handle to the message sender, so you can synchronously send messages from anywhere without blocking.
#[derive(Clone)]
pub struct MessageSenderHandle {
	message_tx: mpsc::UnboundedSender<Message>,
	pending_future_tx: PendingFutureSender,
	max_message_id: Arc<AtomicU64>,
}
impl MessageSenderHandle {
	/// Queue up an error to be sent.
	pub fn error<E: std::fmt::Display>(
		&self,
		node_path: &str,
		method_name: &str,
		err: E,
		data: &[u8],
	) -> Result<(), MessengerError> {
		self.send(serialize_error(node_path, method_name, err, data))
	}
	/// Queue up a signal to be sent.
	pub fn signal(
		&self,
		node_path: &str,
		signal_name: &str,
		data: &[u8],
	) -> Result<(), MessengerError> {
		self.send(serialize_signal_call(node_path, signal_name, data))
	}
	/// Queue up a method to be sent and get back a future for when a response is returned.
	pub fn method(
		&self,
		node_path: &str,
		method: &str,
		data: &[u8],
	) -> Result<impl Future<Output = Result<Vec<u8>, String>>, MessengerError> {
		let (tx, rx) = oneshot::channel();
		let id = self.max_message_id.load(Ordering::Relaxed);
		self.pending_future_tx
			.send((id, tx))
			.map_err(|_| MessengerError::ReceiverDropped)?;
		self.send(serialize_method_call(id, node_path, method, data))?;
		self.max_message_id.store(id + 1, Ordering::Relaxed);
		Ok(async move { rx.await.map_err(|e| e.to_string())? })
	}

	#[instrument(level = "trace", skip_all)]
	fn send(&self, message: Message) -> Result<(), MessengerError> {
		self.message_tx
			.send(message)
			.map_err(|_| MessengerError::ReceiverDropped)
	}
}

/// Create 2 messenger halves from a connection to a stardust client or server.
pub fn create(connection: UnixStream) -> (MessageSender, MessageReceiver) {
	let (read, write) = connection.into_split();
	let (pending_future_tx, pending_future_rx) = mpsc::unbounded_channel();
	let sender = MessageSender::new(write, pending_future_tx);
	let receiver = MessageReceiver::new(read, pending_future_rx, sender.handle());
	(sender, receiver)
}
