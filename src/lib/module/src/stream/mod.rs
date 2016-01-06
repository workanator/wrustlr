//! Stream processing module facility and behavior.

mod intention;

use wrust_types::net::connection::Descriptor;

pub use self::intention::Intention;

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
	/// Mutability is required for module chaining.
	fn read(self: &Self, desc: &Descriptor, buf: &mut Vec<u8>) -> Intention;

	/// The stream processing module is ready to output some data in `buf`.
	fn write(self: &Self, desc: &Descriptor, buf: &mut Vec<u8>) -> Intention;

	/// The `flush`ing happens when the client connection is probably half shutdown or even
	/// closed and the stream processing module has the last chance to write any data left.  
	/// The method must return `true` if the client connection should be closed after delivery
	/// of the data and `false` if there are more data to output.
	fn flush(self: &Self, desc: &Descriptor, buf: &mut Vec<u8>) -> bool;

	/// The client connection is going to be close and the stream processing module has a chance
	/// to free related resources.
	fn close(self: &Self, desc: &Descriptor);
}
