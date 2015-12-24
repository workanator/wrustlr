//! Communication channels.

use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver, SendError, RecvError, TryRecvError};

/// One-way communication `Channel`.
pub struct Channel<T> {
	sender: Sender<T>,
	receiver: Receiver<T>,
}


impl<T> Channel<T> {
	/// Create a new `Channel`
	pub fn new() -> Channel<T> {
		let (tx, rx) = mpsc::channel::<T>();

		Channel {
			sender: tx,
			receiver: rx,
		}
	}

	/// Get that channel `Sender`.
	pub fn sender(&self) -> &Sender<T> {
		&self.sender
	}

	/// Get that channel `Receiver`.
	pub fn receiver(&self) -> &Receiver<T> {
		&self.receiver
	}

	/// Send data.
	pub fn send(&self, data: T) -> Result<(), SendError<T>> {
		self.sender.send(data)
	}

	/// Receive some data and block if no data is available yet.
	pub fn recv(&self) -> Result<T, RecvError> {
		self.receiver.recv()
	}

	/// Try to receive some data and return without blocking if
	/// no data is available yet.
	pub fn try_recv(&self) -> Result<T, TryRecvError> {
		self.receiver.try_recv()
	}

	/// Destructure `self` to the `Sender` and the `Receiver`.
	pub fn split(self) -> (Sender<T>, Receiver<T>) {
		( self.sender, self.receiver )
	}
}


/// Two-way communication `DuplexChannel`.
pub struct DuplexChannel<T> {
	first: Channel<T>,
	second: Channel<T>,
}


impl<T> DuplexChannel<T> {
	/// Create a new `DuplexChannel`
	pub fn new() -> DuplexChannel<T> {
		let (tx1, rx1) = mpsc::channel::<T>();
		let (tx2, rx2) = mpsc::channel::<T>();

		DuplexChannel {
			first: Channel {
				sender: tx1,
				receiver: rx2,
			},
			second: Channel {
				sender: tx2,
				receiver: rx1,
			},
		}
	}

	/// Destructure `self` to two `Channel`s where the first channel is used
	/// to send data forward and the second channels is used the send data backward.
	pub fn split(self) -> (Channel<T>, Channel<T>) {
		( self.first, self.second )
	}
}
