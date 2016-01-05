//! Client socket I/O operation `Processor`. 

use std::sync::Arc;
use std::thread;
use std::sync::atomic::Ordering;
use deque::Stolen;
use mio::{TryRead, TryWrite, EventSet};
use wrust_types::{Result, Error};
use wrust_types::net::Protocol;
use wrust_types::net::connection::State;
use wrust_types::module::stream::{Behavior, Intention};
use ::net::{EventChannel, Request};
use ::net::client::Client;
use ::net::server::Server;
use super::{Queue, Parcel};


/// I/O event processor  
/// New `Worker`s can be spawned using method `worker` on the live queue.
pub struct Worker {
	_child: thread::JoinHandle<()>,
	_id: usize,
}


impl Worker {
    /// Create a new `Worker` which handles client socket I/O operations.
	pub fn run(queue: &Queue, id: usize, event_channel: EventChannel) -> Self {
		let stealer = queue.stealer();
		let ready = queue.ready();
		let counter = queue.worker_count();

		let child = thread::spawn(move || {
			debug!("Worker {} started", id);

			let mut done = false;

			// The main loop where the worker tries to steal parcels from the deque and process them
			while !done {
				match stealer.steal() {
					Stolen::Empty => {
						// No items in the deque, the worker can sleep
						ready.wait();
					},
					Stolen::Abort => {
						// Do nothing, just try to steal again
					},
					Stolen::Data(parcel) => {
						match parcel {
							Parcel::Shutdown => {
								done = true;
							},
							Parcel::Open { server, client } => {
								debug!("{} -> {:?} opens {:?}", id, *server, *client);
								Worker::open(&server, &client, &event_channel);
							},
							Parcel::Close { server, client } => {
								debug!("{} -> {:?} closes {:?}", id, *server, *client);
								Worker::close(&server, &client, &event_channel);
							},
							Parcel::Ready { server, client, events } => {
								debug!("{} -> {:?} processes {:?} for {:?}", id, *server, *client, events);

								match client.state() {
									State::Reading => {
										assert!(events.is_readable(), "unexpected events; events={:?}", events);
										Worker::read(&server, &client, &event_channel);
									},
									State::Writing => {
										assert!(events.is_writable(), "unexpected events; events={:?}", events);
										Worker::write(&server, &client, &event_channel);
									},
									State::Flushing => {
										assert!(events.is_writable(), "unexpected events; events={:?}", events);
										Worker::flush(&server, &client, &event_channel);
									},
									_ => unimplemented!(),
								};
							},
						};
					},
				};
			}

			// Decrease the number of running processors
			counter.fetch_sub(1, Ordering::SeqCst);

			debug!("Worker {} finished", id);
		});

		Worker {
			_child: child,
			_id: id,
		}
	}

	fn open(server: &Arc<Server>, client: &Arc<Client>, event_channel: &EventChannel) {
		// Ask the stream processing module what to do next
		let further_action = server.forward()
			.open(client.descriptor());

		// Close the client connection if the stream processing module said to
		// or register in the event loop
		if let Intention::Close(err) = further_action {
			if err.is_some() {
				error!("{}", err.unwrap());
			}

			event_channel
				.send(Request::Close { client_token: *client.token() })
				.unwrap();
		}
		else {
			// Change the client state
			client.set_state(further_action.as_state());

			event_channel
				.send(Request::Register {
						client_token: *client.token(),
						events: further_action.as_event_set(),
					})
				.unwrap();
		};
	}

	fn close(server: &Arc<Server>, client: &Arc<Client>, event_channel: &EventChannel) {
		// Ask the stream processing to free resources associated with the connection
		server.forward()
			.close(client.descriptor());

		// Send the event loop request to close the connection
		event_channel
			.send(Request::Close { client_token: *client.token() })
			.unwrap();
	}

	fn reregister(client: &Arc<Client>, event_channel: &EventChannel, intention: Intention) {
		// Close the client connection if the stream processing module said to
		// or reregister in the event loop
		if let Intention::Close(err) = intention {
			if err.is_some() {
				error!("{}", err.unwrap());
			}

			event_channel
				.send(Request::Close { client_token: *client.token() })
				.unwrap();
		}
		else {
			// Change the client state
			client.set_state(intention.as_state());

			event_channel
				.send(Request::Reregister {
						client_token: *client.token(),
						events: intention.as_event_set(),
					})
				.unwrap();
		};
	}

	fn read(server: &Arc<Server>, client: &Arc<Client>, event_channel: &EventChannel) {
		// Read data from the socket
		let mut buf: Vec<u8> = Vec::new();
		let read_result = Worker::try_read_buf(client, &mut buf);

		// Check what we'v got
		match read_result {
			Ok(Some(0)) => {
				// If there is any data buffered up, attempt to write it back
				// to the client. Either the socket is currently closed, in
				// which case writing will result in an error, or the client
				// only shutdown half of the socket and is still expecting to
				// receive the buffered data back. See
				// test_handling_client_shutdown() for an illustration
				println!("    read 0 bytes from client; buffered={}", buf.len());

				// Change the client state
				client.set_state(State::Flushing);

				event_channel
					.send(Request::Register {
							client_token: *client.token(),
							events: EventSet::writable(),
						})
					.unwrap();
			},
			Ok(Some(n)) => {
				println!("read {} bytes", n);

				// Pass read data to the stream processing module
				let further_action = server.forward()
					.read(client.descriptor(), &mut buf);

				// Re-register the socket with the event loop. The current
				// state is used to determine whether we are currently reading
				// or writing.
				Worker::reregister(client, event_channel, further_action);
			},
			Ok(None) => {
				event_channel
					.send(Request::Reregister {
							client_token: *client.token(),
							events: EventSet::readable(),
						})
					.unwrap();
			},
			Err(e) => {
				panic!("got an error trying to read; err={:?}", e);
			}
		}
	}

	fn write(server: &Arc<Server>, client: &Arc<Client>, event_channel: &EventChannel) {
		// Get data from the stream processing module and write to the stream
		let mut buf: Vec<u8> = Vec::new();
		let further_action = server.forward()
			.write(client.descriptor(), &mut buf);
		let write_result = Worker::try_write_buf(client, &mut buf);

		match write_result {
			Ok(Some(_)) => {
				// Re-register the socket with the event loop.
				Worker::reregister(client, event_channel, further_action);
			}
			Ok(None) => {
				// The socket wasn't actually ready, re-register the socket
				// with the event loop
				event_channel
					.send(Request::Reregister {
							client_token: *client.token(),
							events: EventSet::writable(),
						})
					.unwrap();
			}
			Err(e) => {
				panic!("got an error trying to write; err={:?}", e);
			}
		}
	}

	fn flush(server: &Arc<Server>, client: &Arc<Client>, event_channel: &EventChannel) {
		// Get data from the stream processing module and write to the stream
		let mut buf: Vec<u8> = Vec::new();
		let should_close = server.forward()
			.flush(client.descriptor(), &mut buf);
		let write_result = Worker::try_write_buf(client, &mut buf);

		match write_result {
			Ok(Some(_)) => {
				// Re-register the socket with the event loop.
				if should_close {
					Worker::reregister(client, event_channel, Intention::Close(None));
				}
				else {
					Worker::reregister(client, event_channel, Intention::Write);
				}
			}
			Ok(None) => {
				// The socket wasn't actually ready, re-register the socket
				// with the event loop
				event_channel
					.send(Request::Reregister {
							client_token: *client.token(),
							events: EventSet::writable(),
						})
					.unwrap();
			}
			Err(e) => {
				panic!("got an error trying to write; err={:?}", e);
			}
		}
	}

	fn try_read_buf(client: &Arc<Client>, buf: &mut Vec<u8>) -> Result<Option<usize>> {
		client.then_on_socket(|sock| -> Result<Option<usize>> {
			match sock {
				&mut Protocol::Tcp(ref mut stream) => match stream.try_read_buf(buf) {
					Ok(count) => Ok(count),
					Err(msg) => make_err!(msg)
				},
				&mut Protocol::Unix(ref mut stream) => match stream.try_read_buf(buf) {
					Ok(count) => Ok(count),
					Err(msg) => make_err!(msg)
				},
				_ => Err(Error::general("Cannot read from client socket").because("UDP is not supported"))
			}
		})
	}

	fn try_write_buf(client: &Arc<Client>, buf: &mut Vec<u8>) -> Result<Option<usize>> {
		client.then_on_socket(|sock| -> Result<Option<usize>> {
			match sock {
				&mut Protocol::Tcp(ref mut stream) => match stream.try_write(buf) {
					Ok(count) => Ok(count),
					Err(msg) => make_err!(msg)
				},
				&mut Protocol::Unix(ref mut stream) => match stream.try_write(buf) {
					Ok(count) => Ok(count),
					Err(msg) => make_err!(msg)
				},
				_ => Err(Error::general("Cannot write to client socket").because("UDP is not supported"))
			}
		})
	}
}