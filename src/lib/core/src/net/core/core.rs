use std::thread;
use std::fs;
use std::path::Path;
use wrust_io::mio;
use wrust_io::mio::{EventSet, PollOpt};
use wrust_types::{Error, Result};
use wrust_types::channel::DuplexChannel;
use wrust_types::net::Protocol;
use ::net::{Request, CommandChannel};
use ::net::core::{CoreConf};
use ::net::server::{ServerConf, Registry as ServerRegistry};
use ::net::client::{Registry as ClientRegistry};
use ::net::work::{Queue, Parcel};
use ::module::Factory;

#[derive(Debug, PartialEq)]
enum Stage {
	Init,
	Listen,
	Shutdown,
}

pub struct Core {
	stage: Stage,
	channel: CommandChannel,
	servers: ServerRegistry,
	clients: ClientRegistry,
	queue: Queue,
}


/// Wrustlr `Core`
impl Core {
	pub fn clients(&mut self) -> &mut ClientRegistry {
		&mut self.clients
	}

	pub fn start(conf: CoreConf, module_factory: &Factory, servers: Vec<ServerConf>) -> Result<CommandChannel> {
		// Create TCP listeners from the configuration
		let mut server_reg = ServerRegistry::new(0);

		for config in &servers {
			try!(server_reg.add(module_factory, config));
		}

		// Create duplex channel to communicate with the server
		let (request_channel, response_channel) = DuplexChannel::new().split();

		let slab = ClientRegistry::new(server_reg.len(), 1024);
		let mut instance = Core {
			stage: Stage::Init,
			channel: response_channel,
			servers: server_reg,
			clients: slab,
			queue: Queue::new(conf.worker_count as usize),
		};

		// Create and initialize event loop
		let loop_config = mio::EventLoopConfig::default();
		let mut event_loop = match mio::EventLoop::configured(loop_config) {
			Ok(event_loop) => event_loop,
			Err(msg) => return Error::new("Event loop failed to initialize").because(msg).result(),
		};

		// .. register servers
		let err = instance.servers.each(|ref serv| -> Option<Error> {
			match *serv.socket() {
				Protocol::Tcp(ref listener) => {
					match event_loop.register(listener, *serv.token(), EventSet::all(), PollOpt::edge()) {
						Ok(_) => {
							if let Protocol::Tcp(ref details) = serv.config().listen.protocol {
								info!("Listen on {}:{} using TCP", details.address, details.port);
							}

							None
						},
						Err(msg) => Some(Error::new("TCP listener registration failed").because(msg))
					}
				},
				Protocol::Unix(ref listener) => {
					match event_loop.register(listener, *serv.token(), EventSet::all(), PollOpt::edge()) {
						Ok(_) => {
							if let Protocol::Unix(ref details) = serv.config().listen.protocol {
								info!("Listen on {} using UNIX", details.path);
							}

							None
						},
						Err(msg) => Some(Error::new("UNIX listener registration failed because {}").because(msg))
					}
				},
				Protocol::Udp(_) => Some(Error::new("UDP is not supported"))
			}
		});

		if let Some(msg) = err {
			return Err(msg);
		}

		// .. run the loop
		thread::spawn(move || {
			instance.stage = Stage::Listen;

			let mut success = true;
			while event_loop.is_running() {
				// Execute ticks as long as the event loop is running
				if let Err(msg) = event_loop.run_once(&mut instance, Some(100)) {
					error!("Shutting down the loop immediately because of {}", msg);
					event_loop.shutdown();
					success = false;
				}
			};

			if !success {
				info!("Normal server shutdown");
			}
		});

		// Server are ready and running
		Ok(request_channel)
	}

	fn cleanup(&mut self) {
		// Clean resources
		self.servers.each(|ref serv| -> Option<Error> {
			// Remove unix socket files
			if let Protocol::Unix(ref details) = serv.config().listen.protocol {
				let path = Path::new(&details.path);
				let _ = fs::remove_file(&path);
			}

			None
		});
	}
}


impl mio::Handler for Core {
	type Timeout = ();
	type Message = Request;

	fn ready(&mut self, event_loop: &mut mio::EventLoop<Self>, token: mio::Token, events: mio::EventSet) {
		let index = token.as_usize();

		// Server socket has a connection request
		if index < self.servers.len() {
			// Do not accept new connections unless the listener is on the listen stage
			if self.stage != Stage::Listen {
				return;
			}

			if events.is_readable() || events.is_writable() {
				// Accept connection
				let client_token: Result<Option<mio::Token>> = self.servers.then_with(index, &mut self.clients, |serv, clients| {
					match *serv.socket() {
						Protocol::Tcp(ref sock) => {
							// Accept TCP the client connection
							match accept(sock, event_loop) {
								Ok(Some(client_socket)) => match clients.add(token, Protocol::Tcp(client_socket)) {
									Ok(client_token) => Ok(Some(client_token)),
									Err(msg) => Err(msg)
								},
								Ok(None) => Ok(None),
								_ => Error::new("Cannot accept TCP client connection").result()
							}
						},
						Protocol::Unix(ref sock) => {
							// Accept UNIX the client connection
							match accept(sock, event_loop) {
								Ok(Some(client_socket)) => match clients.add(token, Protocol::Unix(client_socket)) {
									Ok(client_token) => Ok(Some(client_token)),
									Err(msg) => Err(msg)
								},
								Ok(None) => Ok(None),
								_ => Error::new("Cannot accept UNIX client connection").result()
							}
						},
						Protocol::Udp(_) => Error::new("UDP is not supported").result()
					}});

				match client_token {
					Ok(Some(client_token)) => {
						// Push Open event in the queue
						self.queue.push(Parcel::Open {
							server: self.servers[token].clone(),
							client: self.clients[client_token].clone(),
						});
					},
					Err(err) => {
						error!("{}", err);
						return;
					},
					_ => ()
				};
			}
		}
		else {
			// Push Ready event in the queue
			self.queue.push(Parcel::Ready {
				server: self.servers[*self.clients[token].server_token()].clone(),
				client: self.clients[token].clone(),
				events: events,
			});
		}
	}

	fn tick(&mut self, event_loop: &mut mio::EventLoop<Self>) {
		self.queue.awake(|| {
			(event_loop.channel())
		});

		if let Ok(command) = self.channel.try_recv() {
			if command == "shutdown" {
				info!("Received SHUTDOWN command");
				self.stage = Stage::Shutdown;
				self.queue.shutdown(true);
				event_loop.shutdown();
				self.cleanup();

				if let Err(msg) = self.channel.send("ok") {
					error!("{}", msg);
				}
			}
		}
	}

	fn notify(&mut self, event_loop: &mut mio::EventLoop<Self>, msg: Self::Message) {
		match msg {
			Request::Close { client_token } => {
				debug!("Request::Close {:?}", client_token);
				// Deregister the client connection
				let _ = self.clients[client_token].then_on_socket(|socket| {
					socket
						.tcp_and_then(|sock| {
							if let Err(msg) = event_loop.deregister(sock) {
								error!("{}", msg);
							}
						})
						.unix_and_then(|sock| {
							if let Err(msg) = event_loop.deregister(sock) {
								error!("{}", msg);
							}
						});

					Ok(())
				});

				// Remove the client connection from the registry
				self.clients
					.remove(client_token);
			},
			Request::Register { client_token, events } => {
				debug!("Request::Register {:?} for {:?}", client_token, events);
				// Register the client connection for the new events
				let _ = self.clients[client_token].then_on_socket(|socket| {
					socket
						.tcp_and_then(|sock| {
							if let Err(msg) = event_loop.register(sock, client_token, events, PollOpt::edge() | PollOpt::oneshot()) {
								error!("{}", msg);
							}
						})
						.unix_and_then(|sock| {
							if let Err(msg) = event_loop.register(sock, client_token, events, PollOpt::edge() | PollOpt::oneshot()) {
								error!("{}", msg);
							}
						});

					Ok(())
				});
			},
			Request::Reregister { client_token, events } => {
				// Reregister the client connection for the new events
				debug!("Request::Reregister {:?} for {:?}", client_token, events);
				let _ = self.clients[client_token].then_on_socket(|socket| {
					socket
						.tcp_and_then(|sock| {
							if let Err(msg) = event_loop.reregister(sock, client_token, events, PollOpt::edge() | PollOpt::oneshot()) {
								error!("{}", msg);
							}
						})
						.unix_and_then(|sock| {
							if let Err(msg) = event_loop.reregister(sock, client_token, events, PollOpt::edge() | PollOpt::oneshot()) {
								error!("{}", msg);
							}
						});

					Ok(())
				});
			},
		};
	}
}


impl Drop for Core {
	fn drop(&mut self) {
		self.cleanup();
	}
}


fn accept<Sock> (sock: &Sock, event_loop: &mut mio::EventLoop<Core>) -> Result<Option<Sock::Output>>
	where Sock: mio::TryAccept {
	match sock.accept() {
		Ok(Some(connection)) => {
			Ok(Some(connection))
		}
		Ok(None) => {
			Ok(None)
		}
		Err(e) => {
			error!("Server socket accept failed with error: {}", e);
			event_loop.shutdown();

			Error::new("Failed to accept client connection").because(e).result()
		}
	}
}
