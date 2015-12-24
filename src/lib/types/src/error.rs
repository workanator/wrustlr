use std;
use std::fmt;
use std::convert::From;
use std::string::ToString;
use std::io;
use config;


/// Shorthand to produce `Err(Error)` from compatible type
#[macro_export]
macro_rules! make_err {
	($what:expr) => ({
		use $crate::MakeErr;
		Error::make_err($what)
	});
}


/// Error type
#[derive(Debug)]
pub enum Error {
	/// General error with text details
	General(String, Option<String>),
	/// Configuration is the source of the error
	Config(config::error::ConfigError),
	/// I/O operation is the source of the error
	Io(io::Error),
}


/// Make `Err(Error)` from compatible type
pub trait MakeErr<E> {
	fn make_err<T>(E) -> Result<T, Error>;
}


impl Error {
	/// Construct general `Error` with message only
	pub fn general<Msg>(what: Msg) -> Error
		where Msg: ToString {
		Error::General(what.to_string(), None)
	}

	/// Convert `self` to the general `Error` with details provided if it is of type `General`
	/// or return `self` unmodified.
	pub fn because<Details>(self, details: Details) -> Error
		where Details: ToString {
		match self {
			Error::General(msg, None) => Error::General(msg, Some(details.to_string())),
			Error::General(msg, Some(dets)) => Error::General(msg, Some(format!("{}, {}", dets, details.to_string()))),
			_ => self
		}
	}

	/// Consumes self and retuns `Err` variant of `Result<T, Error>`.
	pub fn result<T>(self) -> Result<T, Self> {
		Err(self)
	}
}


impl std::error::Error for Error {
	fn description(&self) -> &str {
		match *self {
			Error::General(ref msg, _) => &msg,
			Error::Config(ref err) => err.description(),
			Error::Io(ref err) => err.description(),
		}
	}

	fn cause(&self) -> Option<&std::error::Error> {
		match *self {
			Error::General(_, _) => None,
			Error::Config(ref err) => Some(err),
			Error::Io(ref err) => Some(err),
		}
	}
}


impl<'a> From<&'a str> for Error {
	fn from(src: &'a str) -> Self {
		Error::General(src.to_string(), None)
	}
}

impl From<String> for Error {
	fn from(src: String) -> Self {
		Error::General(src, None)
	}
}

impl From<config::error::ConfigError> for Error {
	fn from(src: config::error::ConfigError) -> Self {
		Error::Config(src)
	}
}

impl From<io::Error> for Error {
	fn from(src: io::Error) -> Self {
		Error::Io(src)
	}
}

impl<'a> MakeErr<&'a str> for Error {
	fn make_err<T>(what: &'a str) -> Result<T, Error> {
		Err(Error::from(what.to_string()))
	}
}

impl MakeErr<String> for Error {
	fn make_err<T>(what: String) -> Result<T, Error> {
		Err(Error::from(what))
	}
}

impl MakeErr<config::error::ConfigError> for Error {
	fn make_err<T>(what: config::error::ConfigError) -> Result<T, Error> {
		Err(Error::from(what))
	}
}

impl MakeErr<io::Error> for Error {
	fn make_err<T>(what: io::Error) -> Result<T, Error> {
		Err(Error::from(what))
	}
}

impl fmt::Display for Error
 {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Error::General(ref msg, None) => write!(f, "General error: {}", msg),
			Error::General(ref msg, Some(ref details)) => write!(f, "General error: {} because {}", msg, details),
			Error::Config(ref msg) => write!(f, "Configuration error: {}", msg),
			Error::Io(ref msg) => write!(f, "I/O error: {}", msg),
		}
    }
}
