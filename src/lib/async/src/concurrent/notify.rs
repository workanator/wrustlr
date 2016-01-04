use std::sync::Condvar;

/// Notification target
#[derive(Debug)]
pub enum Notify {
	/// Notify no one thread
	None,
	/// Notify one thread waiting
	One,
	/// Notify all threads waiting
	All,
}


impl Notify {
	/// Invoke appropriate notification method of `Condvar`.
	pub fn notify(&self, cvar: &Condvar) {
		match self {
			&Notify::None => (),
			&Notify::One => cvar.notify_one(),
			&Notify::All => cvar.notify_all(),
		};
	}
}