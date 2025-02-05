use thiserror::Error;

/// Error for all scenegraph-related things.
#[derive(Error, Debug)]
pub enum ScenegraphError {
	#[error("Node not found")]
	NodeNotFound,
	#[error("Alias has broken")]
	BrokenAlias,
	#[error("Signal not found")]
	SignalNotFound,
	#[error("Method not found")]
	MethodNotFound,
	#[error("Signal error: {error}")]
	SignalError { error: String },
	#[error("Method error: {error}")]
	MethodError { error: String },
}

/// Handles node signals and method calls for the messenger.
pub trait Scenegraph {
	fn send_signal(&self, path: &str, method: &str, data: &[u8]) -> Result<(), ScenegraphError> {
		self.execute_method(path, method, data).map(|_| ())
	}
	fn execute_method(
		&self,
		path: &str,
		method: &str,
		data: &[u8],
	) -> Result<Vec<u8>, ScenegraphError>;
}
