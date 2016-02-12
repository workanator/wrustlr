use std::ops::Index;
use std::sync::Arc;
use std::path::Path;
use wrust_io::mio;
use wrust_io::mio::tcp::TcpListener;
use wrust_io::mio::unix::UnixListener;
use wrust_types::{Error, Result};
use wrust_types::net::Protocol;
use wrust_module::Category;
use ::module::Factory;
use super::{Server, ServerConf};

pub struct Registry {
	start_from: usize,
	items: Vec<Arc<Server>>,
}


impl Registry {
	pub fn new(start_from: usize) -> Registry {
		Registry {
			start_from: start_from,
			items: Vec::new(),
		}
	}

	pub fn add(&mut self, module_factory: &Factory, config: &ServerConf) -> Result<mio::Token> {
		let socket = match config.listen.protocol {
			Protocol::Tcp(ref _details) => {
				let addr = try!(config.socket_address());

				match TcpListener::bind(&addr) {
					Ok(listener) => Protocol::Tcp(listener),
					Err(msg) => return Error::new("TCP Server socket binding failed").because(msg).result()
				}
			},
			Protocol::Unix(ref details) => {
				let path = Path::new(&details.path);

				match UnixListener::bind(&path) {
					Ok(listener) => Protocol::Unix(listener),
					Err(msg) => return Error::new("UNIX Server socket binding failed").because(msg).result()
				}
			},
			_ => return Error::new("Cannot bind UDP socket because the protocol is unsupported").result()
		};

		let forward = try!(module_factory.produce(Category::Stream, &config.forward.name, &config.forward.xpath));
		let token = mio::Token(self.start_from + self.items.len());

		self.items.push(Arc::new(Server::new(token, config.clone(), socket, forward)));

		Ok(token)
	}

	pub fn len(&self) -> usize {
		self.items.len()
	}

	pub fn each<Func, R>(&self, mut func: Func) -> Option<R>
		where Func: FnMut(&Server) -> Option<R> {
		for item in &self.items {
			let result = func(item);
			if result.is_some() {
				return result;
			}
		}

		None
	}

	pub fn then_with<F, Ctx, T>(&self, index: usize, context: &mut Ctx, mut func: F) -> Result<T>
		where F: FnMut(&Arc<Server>, &mut Ctx) -> Result<T> {
		if index < self.items.len() {
			func(&self.items[index], context)
		}
		else {
			Error::new(format!("Server index is out of bounds ({} not in [0;{}))", index, self.items.len())).result()
		}
	}
}

impl Index<usize> for Registry {
	type Output = Arc<Server>;

	fn index(&self, index: usize) -> &Self::Output {
		&self.items[index]
	}
}

impl Index<mio::Token> for Registry {
	type Output = Arc<Server>;

	fn index(&self, index: mio::Token) -> &Self::Output {
		&self.items[index.as_usize()]
	}
}
