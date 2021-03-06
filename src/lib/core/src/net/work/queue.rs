//! Client socket I/O operation `Queue`.

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;
use wrust_io::mio;
use wrust_async::crossbeam::sync::chase_lev;
use wrust_async::concurrent::{Notify, ReadyFlag};
use ::net::EventChannel;
use ::net::server::Server;
use ::net::client::Client;
use super::Worker;


/// Queue parcel
#[allow(dead_code)]
#[derive(Debug)]
pub enum Parcel {
	/// Shutdown request. The processor must shutdown as soon as possible.
	Shutdown,
	/// Open new connection.
	Open { server: Arc<Server>, client: Arc<Client> },
	/// Close connection.
	Close { server: Arc<Server>, client: Arc<Client> },
	/// I/O ready event.
	Ready { server: Arc<Server>, client: Arc<Client>, events: mio::EventSet },
}


/// I/O processor event queue
pub struct Queue {
	worker: chase_lev::Worker<Parcel>,
	stealer: chase_lev::Stealer<Parcel>,
	ready: ReadyFlag,
	worker_count: Arc<AtomicUsize>,
	worker_next_id: Arc<AtomicUsize>,
	worker_count_max: usize,
}


impl Queue {
	/// Create a new event queue.
	pub fn new(worker_count_max: usize) -> Queue {
		let (worker, stealer) = chase_lev::deque();

		Queue {
			worker: worker,
			stealer: stealer,
			ready: ReadyFlag::new(),
			worker_count: Arc::new(AtomicUsize::new(0)),
			worker_next_id: Arc::new(AtomicUsize::new(0)),
			worker_count_max: worker_count_max,
		}
	}

	/// Get clone of the stealer
	pub fn stealer(&self) -> chase_lev::Stealer<Parcel> {
		self.stealer.clone()
	}

	/// Get clone of the ready flag
	pub fn ready(&self) -> ReadyFlag {
		self.ready.clone()
	}

	/// Get clone of the worker counter
	pub fn worker_count(&self) -> Arc<AtomicUsize> {
		self.worker_count.clone()
	}

	/// Awake one thread eventually
	pub fn awake<F>(&self, factory: F)
		where F: Fn() -> (EventChannel) {
		self.ready.raise(Notify::One);

		let count_diff = self.worker_count_max - self.worker_count.load(Ordering::SeqCst);
		for _ in 0..count_diff {
			let channel = factory();
			let _ = self.worker(channel);
		}
	}

	/// Push shutdown request in the queue. If `fast` is `true` then the queue
	/// will be cleaned first.
	pub fn shutdown(&mut self, fast: bool) {
		// Clean the deque first because immediate shutdown is requested
		if fast {
			loop {
				match self.worker.try_pop() {
					Some(_) => (),
					None => break,
				};
			}
		}

		// Push so much shutdown requests in the deque so many live workers we have and raise the ready flag
		for _ in 0..self.worker_count.load(Ordering::SeqCst) {
			self.worker.push(Parcel::Shutdown);
		}
		// .. wait until all workers done
		while self.worker_count.load(Ordering::SeqCst) > 0 {
			self.ready.raise(Notify::All);
			thread::sleep(Duration::from_millis(100));
		}
	}

	/// Push event in the qeueue and notify worker that new parcel is available.
	pub fn push(&mut self, parcel: Parcel) {
		// Push the parcel in the deque and raise the ready flag
		self.worker.push(parcel);
		self.ready.raise(Notify::All);
	}

	/// Create a new `Worker` which handles client socket I/O operations.
	pub fn worker(&self, event_channel: EventChannel) -> Worker {
		let worker_id = self.worker_next_id.fetch_add(1, Ordering::SeqCst);
		let worker = Worker::run(self, worker_id, event_channel);

		// Increase the number of running workers
		self.worker_count.fetch_add(1, Ordering::SeqCst);

		worker
	}
}
