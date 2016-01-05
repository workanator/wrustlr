use std;
use std::fmt;
use std::ops::Deref;


/// Error type
#[derive(Debug)]
pub struct Error {
	// Description
	what: String,
	// Cause
	cause: Option<Box<std::error::Error + Sized>>,
}


impl Error {
	/// Construct `Error` with message only
	pub fn new<S>(what: S) -> Self
		where S: Into<String> {
		Error {
			what: what.into(),
			cause: None,
		}
	}

	/// Consumes self and constructs `Error` with message and cause
	pub fn because<E>(self, cause: E) -> Self
		where E: 'static + std::error::Error + Sized {
		Error {
			what: self.what,
			cause: Some(Box::new(cause)),
		}
	}

	/// Consumes self and retuns `Err` variant of `Result<T, Error>`.
	pub fn result<T>(self) -> Result<T, Self> {
		Err(self)
	}
}


impl std::error::Error for Error {
	fn description(&self) -> &str {
		&self.what
	}

	fn cause(&self) -> Option<&std::error::Error> {
		match self.cause {
			None => None,
			Some(ref boxed) => Some(boxed.deref())
		}
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self.cause {
			None => write!(f, "{}", self.what),
			Some(ref boxed) => write!(f, "{} because {}", self.what, &boxed),
		}
    }
}
