use std::fmt;
use wrust_io::mio;
use wrust_io::mio::tcp::TcpListener;
use wrust_io::mio::unix::UnixListener;
use wrust_types::net::Protocol;
use wrust_types::net::connection::Descriptor;
use wrust_module::stream::{Behavior, Intention, Flush};
use super::ServerConf;


pub type ServerProtocol = Protocol<TcpListener, (), UnixListener>;


#[allow(dead_code)]
pub struct Server {
	token: mio::Token,
	config: ServerConf,
	socket: ServerProtocol,
	forward: ForwardProxy,
}


unsafe impl Sync for Server {}


pub struct ForwardProxy {
	instance: Box<Behavior>,
}

impl ForwardProxy {
	pub fn new(instance: Box<Behavior>) -> ForwardProxy {
		ForwardProxy {
			instance: instance
		}
	}
}

impl Behavior for ForwardProxy {
	fn open(self: &Self, desc: &Descriptor) -> Intention {
		self.instance
			.open(desc)
	}

	fn read(self: &Self, desc: &Descriptor, buf: &Vec<u8>) -> Intention {
		self.instance
			.read(desc, buf)
	}

	fn write(self: &Self, desc: &Descriptor, buf: &mut Vec<u8>) -> (Intention, Flush) {
		self.instance
			.write(desc, buf)
	}

	fn close(self: &Self, desc: &Descriptor) {
		self.instance
			.close(desc)
	}
}


impl Server {
	pub fn new(token: mio::Token, config: ServerConf, socket: ServerProtocol, forward: Box<Behavior>) -> Server {
		Server {
			token: token,
			config: config,
			socket: socket,
			forward: ForwardProxy::new(forward),
		}
	}

	pub fn token(&self) -> &mio::Token {
		&self.token
	}

	pub fn config(&self) -> &ServerConf {
		&self.config
	}

	pub fn socket(&self) -> &ServerProtocol {
		&self.socket
	}

	pub fn forward(&self) -> &ForwardProxy {
		&self.forward
	}
}


impl fmt::Debug for Server {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self.socket {
			Protocol::Tcp(_) => write!(f, "TCP Server #{:?}", self.token),
			Protocol::Udp(_) => write!(f, "UDP Server #{:?}", self.token),
			Protocol::Unix(_) => write!(f, "UNIX Server #{:?}", self.token),
		}
	}
}
