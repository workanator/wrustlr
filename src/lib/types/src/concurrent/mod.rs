//! Usefull utilities for concurrent programming.

mod ready_flag;

pub use self::ready_flag::ReadyFlag;

/// Notification target
#[derive(Debug)]
pub enum Target {
	/// Notify no one thread
	None,
	/// Notify one thread waiting
	One,
	/// Notify all threads waiting
	All,
}
