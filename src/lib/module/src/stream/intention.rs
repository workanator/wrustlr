use std::fmt;
use std::cmp;
use mio;
use wrust_types::Error;
use wrust_types::net::connection::State;


/// What the stream processing module is going to do next with the stream.
#[derive(Debug)]
pub enum Intention {
	/// The stream processing module is going to read more data from the stream.
	Read,
	/// The stream processing module is going to write more data to the stream.
	Write,
	/// The stream processing module is going to close the stream.
	Close(Option<Error>),
}


impl Intention {
	/// Convert `self` to MIO `EventSet`.
	pub fn as_event_set(&self) -> mio::EventSet {
		match *self {
			Intention::Read => mio::EventSet::readable(),
			Intention::Write => mio::EventSet::writable(),
			Intention::Close(_) => mio::EventSet::none(),
		}
	}

	/// Convert `self` to connection `State`.
	pub fn as_state(&self) -> State {
		match *self {
			Intention::Read => State::Reading,
			Intention::Write => State::Writing,
			Intention::Close(_) => State::Closed,
		}
	}
}


impl fmt::Display for Intention {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Intention::Read => write!(f, "Read"),
			Intention::Write => write!(f, "Write"),
			Intention::Close(Some(ref err)) => write!(f, "Close with error {}", err),
			Intention::Close(None) => write!(f, "Close"),
		}
    }
}


impl cmp::PartialEq for Intention {
	fn eq(&self, other: &Self) -> bool {
		match *self {
			Intention::Read => {
				match *other {
					Intention::Read => true,
					_ => false
				}
			},
			Intention::Write => {
				match *other {
					Intention::Write => true,
					_ => false
				}
			},
			Intention::Close(_) => {
				match *other {
					Intention::Close(_) => true,
					_ => false
				}
			}
		}
	}
}
