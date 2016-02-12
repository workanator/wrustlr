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
	/// `Close` the client connection behind the `client_token`.
	Close { client_token: mio::Token },

	/// `Register` the client connection for further I/O events.
	Register { client_token: mio::Token, events: mio::EventSet },

	/// `Reregister` the client connection for further I/O events.
	Reregister { client_token: mio::Token, events: mio::EventSet },
}
