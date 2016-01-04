//! Wrust Types is a support library for Wrustlr server. Here you can find a number of types, traits, enums
//! common in Wrustlr module development.

extern crate bytes;
extern crate mio;
extern crate config;
extern crate libc;

#[macro_use] mod error;
pub mod channel;
pub mod net;
pub mod module;
pub mod conf;

pub use self::error::Error;
pub use self::error::MakeErr;
pub type Result<T> = std::result::Result<T, self::error::Error>;
