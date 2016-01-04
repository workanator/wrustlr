//! Utilities for concurrent programming.

mod ready_flag;
mod notify;

pub use self::notify::Notify;
pub use self::ready_flag::ReadyFlag;
