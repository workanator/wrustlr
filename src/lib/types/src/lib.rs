//! Wrust Types is a support library for Wrustlr server. Here you can find a number of types, traits, enums
//! common in Wrustlr module development.

extern crate wrust_io;

mod error;
pub mod channel;
pub mod net;

pub use self::error::Error;
pub type Result<T> = std::result::Result<T, self::error::Error>;
