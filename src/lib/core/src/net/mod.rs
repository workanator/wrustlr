pub mod client;
pub mod server;
pub mod work;
pub mod core;

use wrust_io::mio;
use wrust_types::channel::{Channel};

pub type CommandChannel = Channel<&'static str>;
pub type EventChannel = mio::Sender<Request>;

/// Listener event loop messaging enum.
pub enum Request {
	/// `Close` the client connection.
	Close { client_token: mio::Token },

	/// `Open` the client connection.
	Open { client_token: mio::Token, events: mio::EventSet },

	/// Push the client connection into the queue to `Wait` for further I/O events.
	Wait { client_token: mio::Token, events: mio::EventSet },
}
