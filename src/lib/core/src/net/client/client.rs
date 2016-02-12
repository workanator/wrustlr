use std::fmt;
use std::sync::Mutex;
use std::cell::{RefCell, UnsafeCell};
use std::ptr;
use wrust_io::mio;
use wrust_io::mio::tcp::*;
use wrust_io::mio::unix::*;
use wrust_types::Result;
use wrust_types::net::Protocol;
use wrust_types::net::connection::{State, Descriptor};
use wrust_module::stream::{Intention, Flush};


pub type ClientProtocol = Protocol<TcpStream, (), UnixStream>;


pub struct LeftData {
	data: Vec<u8>,
	intention: Intention,
	flush: Flush,
}


unsafe impl Send for LeftData {}
unsafe impl Sync for LeftData {}


impl LeftData {
	pub fn new(data: Vec<u8>, intention: Intention, flush: Flush) -> LeftData {
		LeftData {
			data: data,
			intention: intention,
			flush: flush,
		}
	}

	pub fn data(&self) -> &Vec<u8> {
		&self.data
	}

	pub fn intention(&self) -> &Intention {
		&self.intention
	}

	pub fn flush(&self) -> &Flush {
		&self.flush
	}

	pub fn consume(self) -> (Vec<u8>, (Intention, Flush)) {
		(self.data, (self.intention, self.flush))
	}
}


pub struct Client {
	server_token: mio::Token,
	token: mio::Token,
	socket: Mutex<RefCell<ClientProtocol>>,
	state: Mutex<RefCell<State>>,
	descriptor: Descriptor,
	left_data: Mutex<UnsafeCell<Option<LeftData>>>,
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
			left_data: Mutex::new(UnsafeCell::new(None)),
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

	pub fn left_data(&self) -> Option<LeftData> {
		let cell = self.left_data.lock().unwrap();
		let mut left_data: Option<LeftData> = None;
		unsafe { ptr::swap(&mut left_data, cell.get()); }

		left_data
	}

	pub fn set_left_data(&self, data: Option<LeftData>) {
		let cell = self.left_data.lock().unwrap();
		let mut left_data = data;

		unsafe { ptr::swap(&mut left_data, cell.get()); }
	}
}


impl fmt::Debug for Client {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Client #{:?}", self.token)
	}
}
