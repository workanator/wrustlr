use std::fmt;
use std::sync::Mutex;
use std::cell::RefCell;
use mio;
use mio::tcp::*;
use mio::unix::*;
use wrust_types::Result;
use wrust_types::net::Protocol;
use wrust_types::net::connection::{State, Descriptor};

pub type ClientProtocol = Protocol<TcpStream, (), UnixStream>;

pub struct Client {
	server_token: mio::Token,
	token: mio::Token,
	socket: Mutex<RefCell<ClientProtocol>>,
	state: Mutex<RefCell<State>>,
	descriptor: Descriptor,
	buffer: Vec<u8>,
}

impl Client {
	pub fn new(server_token: mio::Token, token: mio::Token, socket: ClientProtocol) -> Client {
		let descriptor = Descriptor::new(
			token.as_usize() as u32,
			match socket {
				Protocol::Tcp(ref s) => Some(s.peer_addr().unwrap()),
				Protocol::Udp(_) => None,
				Protocol::Unix(_) => None,
			}
		);

		Client {
			server_token: server_token,
			token: token,
			socket: Mutex::new(RefCell::new(socket)),
			state: Mutex::new(RefCell::new(State::Opened)),
			descriptor: descriptor,
			buffer: Vec::new(),
		}
	}

	pub fn server_token(&self) -> &mio::Token {
		&self.server_token
	}

	pub fn token(&self) -> &mio::Token {
		&self.token
	}

	pub fn state(&self) -> State {
		let guard = self.state.lock().unwrap();
		let cell = guard.borrow();
		cell.clone()
	}

	pub fn set_state(&self, state: State) {
		let guard = self.state.lock().unwrap();
		let mut cell = guard.borrow_mut();
		*cell = state;
	}

	pub fn descriptor(&self) -> &Descriptor {
		&self.descriptor
	}

	pub fn then_on_socket<F, T>(&self, mut func: F) -> Result<T>
		where F: FnMut(&mut ClientProtocol) -> Result<T> {
		let guard = self.socket.lock().unwrap();
		let mut cell = guard.borrow_mut();
		func(&mut *cell)
	}
}


impl fmt::Debug for Client {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Client #{:?}", self.token)
	}
}
