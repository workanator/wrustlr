use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Mutex;
use wrust_types::Error;
use wrust_types::net::connection::Descriptor;
use wrust_conf::Conf;
use wrust_module::{Facility, Category};
use wrust_module::stream::{Behavior, Intention, Flush};

const MOD_NAME: &'static str = "echo";

pub struct Module {
	client: Mutex<RefCell<HashMap<u32, Vec<u8>>>>,
	reverse: bool,
}

#[inline(never)]
impl Facility for Module {
	fn new(config: &Conf, xpath: &String) -> Self {
		// Read configuration
		let reverse = config.lookup_boolean_or(&format!("{}.reverse", xpath), false);

		Module {
			client: Mutex::new(RefCell::new(HashMap::new())),
			reverse: reverse,
		}
	}

	fn name() -> String {
		MOD_NAME.to_string()
	}

	fn version() -> String {
		format!("{}.{}.{}", env!("CARGO_PKG_VERSION_MAJOR"), env!("CARGO_PKG_VERSION_MINOR"), env!("CARGO_PKG_VERSION_PATCH")).to_string()
	}

	fn category() -> Category {
		Category::Stream
	}
}


#[inline(never)]
impl Behavior for Module {
	fn open(self: &Self, desc: &Descriptor) -> Intention {
		let cell = self.client.lock().unwrap();
		cell.borrow_mut()
			.insert(desc.id(), Vec::new());

		Intention::Read
	}

	fn read(self: &Self, desc: &Descriptor, buf: &Vec<u8>) -> Intention {
		let cell = self.client.lock().unwrap();
		let mut cell_buf = cell.borrow_mut();
		let mut_buf = cell_buf.get_mut(&desc.id());

		match mut_buf {
			Some(client_buf) => {
				client_buf.extend(buf.iter());

				Intention::Write
			},
			None => Intention::Close(Some(Error::new("Client buffer is undefined")))
		}
	}

	fn write(self: &Self, desc: &Descriptor, buf: &mut Vec<u8>) -> (Intention, Flush) {
		let cell = self.client.lock().unwrap();
		let mut cell_buf = cell.borrow_mut();
		let mut_buf = cell_buf.get_mut(&desc.id());

		match mut_buf {
			Some(client_buf) => {
				let should_close = match client_buf.first() {
					Some(&b'Q') => true,
					_ => false,
				};

				if self.reverse {
					if let Some(c) = client_buf.pop() {
						client_buf.reverse();
						if c == b'\n' {
							client_buf.push(c);
						}
						else {
							client_buf.insert(0, c);
						}
					}
				}

				buf.append(client_buf);

				if should_close {
					(Intention::Close(None), Flush::Auto)
				}
				else {
					(Intention::Read, Flush::Force)
				}
			},
			None => (Intention::Close(Some(Error::new("Client buffer is undefined"))), Flush::Auto)
		}
	}

	fn close(self: &Self, desc: &Descriptor) {
		let cell = self.client.lock().unwrap();
		cell.borrow_mut()
			.remove(&desc.id());
	}
}
