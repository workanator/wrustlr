//! Stream processing module facility and behavior.

mod intention;
mod flush;

use wrust_types::net::connection::Descriptor;

pub use self::intention::Intention;
pub use self::flush::Flush;

/// Each stream processing module must folow the `Behavior`.
pub trait Behavior: Send + Sync {
	/// When a new client connection is accepted `open` method is executed where
	/// the stream processing module must decide what it intents to do next.
	/// The processing module can close the client connection immediately
	/// returning `Intention::Close` and in that case `close` method of
	/// the implemented trait will not be called.
	fn open(self: &Self, desc: &Descriptor) -> Intention;

	/// A new data chunk has been read from the client connection into `buf` and
	/// the stream processing module can handle it.
	fn read(self: &Self, desc: &Descriptor, buf: &Vec<u8>) -> Intention;

	/// The stream processing module is ready to output some data in `buf`.
	fn write(self: &Self, desc: &Descriptor, buf: &mut Vec<u8>) -> (Intention, Flush);

	/// The client connection is going to be close and the stream processing module has a chance
	/// to free related resources.
	fn close(self: &Self, desc: &Descriptor);
}
