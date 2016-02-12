use std::fmt;
use wrust_io::mio;

/// The current state of the client connection.
#[derive(Debug, Clone, PartialEq)]
pub enum State {
	/// Connection is opened and waiting for the further change state.
	Opened,
	/// Connection is reading data from stream.
	Reading,
	/// Connection is writing data into the stream.
	Writing,
	/// Connection is flushing left data into the stream.
	Flushing,
	/// Connection is closed.
	Closed,
}


impl State {
	/// Convert `self` to MIO `EventSet`.
	pub fn as_event_set(&self) -> mio::EventSet {
		match *self {
			State::Reading => mio::EventSet::readable(),
			State::Writing | State::Flushing => mio::EventSet::writable(),
			_ => mio::EventSet::none(),
		}
	}
}

impl fmt::Display for State {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			State::Opened => write!(f, "Opened"),
			State::Reading => write!(f, "Reading"),
			State::Writing => write!(f, "Writing"),
			State::Flushing => write!(f, "Flushing"),
			State::Closed => write!(f, "Closed"),
		}
    }
}
