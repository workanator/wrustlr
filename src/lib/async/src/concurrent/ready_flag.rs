use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, Condvar};
use super::Notify;

/// `ReadyFlag` is usefull to signal that some data is ready or process is done.  
/// The flag is implemented using `AtomicBool` guarded by `Mutex` and `Condvar` wrapped
/// into `Arc` what makes it thread-safe and clonnable.
///
/// # Examples
///
/// ```
/// use std::thread;
/// use wrust_async::concurrent::{ReadyFlag, Notify};
///
/// let go = ReadyFlag::new();
///
/// {
///   let go = go.clone();
///   thread::spawn(move || {
///     go.wait(); // Wait when flag will be up
///     println!("Started!");
///   });
/// }
///
/// thread::sleep_ms(500);
/// go.raise(Notify::All); // Raise the flag and notify all waiting threads
/// ```

#[derive(Clone)]
pub struct ReadyFlag {
	flag: Arc<(Mutex<AtomicBool>, Condvar)>,
}


impl ReadyFlag {
	/// Create a new `ReadyFlag` initially set to lowered state.
	pub fn new() -> ReadyFlag {
		ReadyFlag {
			flag: Arc::new((Mutex::new(AtomicBool::new(false)), Condvar::new())),
		}
	}

	/// Test if flag is risen.
	pub fn is_up(&self) -> bool {
		let &(ref lock, _) = &*self.flag;
		let value = lock.lock().unwrap();
		value.load(Ordering::Acquire)
	}

	/// Lower the flag (set to `false`).
	pub fn lower(&self) {
		let &(ref lock, _) = &*self.flag;
		let value = lock.lock().unwrap();
		value.store(false, Ordering::Release);
	}

	/// Raise the flag (set to `true`) and notify thread(s) waiting for it.
	pub fn raise(&self, target: Notify) {
		let &(ref lock, ref cvar) = &*self.flag;
		let value = lock.lock().unwrap();
		value.store(true, Ordering::Release);
		target.notify(cvar);
	}

	/// Wait for the flag to be risen or return if the flag is already risen. When the waiting
	/// thread returns from `wait` it lowers the flag automatically.
	pub fn wait(&self) {
		let &(ref lock, ref cvar) = &*self.flag;
		let mut value = lock.lock().unwrap();
		while !value.compare_and_swap(true, false, Ordering::Acquire) {
			value = cvar.wait(value).unwrap();
		}
	}
}


#[cfg(test)]
mod tests {
	use std::thread;
	use ::concurrent::*;

	#[test]
	fn test_ready_flag() {
		let rf = ReadyFlag::new();
		assert_eq!(rf.is_up(), false);

		rf.raise(Notify::None);
		assert_eq!(rf.is_up(), true);

		rf.lower();
		assert_eq!(rf.is_up(), false);

		rf.raise(Notify::None);
		rf.wait();
		assert_eq!(rf.is_up(), false);
	}

	#[test]
	fn test_ready_flag_threading() {
		let rf = ReadyFlag::new();
		let rf_done = ReadyFlag::new();

		{
			let rf = rf.clone();
			let rf_done = rf_done.clone();

			thread::spawn(move || {
				rf.wait();
				rf_done.raise(Notify::All);
			});
		}

		thread::sleep_ms(500);
		rf.raise(Notify::All);

		rf_done.wait();
	}
}
