use std::ops::{Index, IndexMut};
use std::sync::Arc;
use mio;
use mio::util::Slab;
use wrust_types::{Result, Error};
use super::Client;
use super::client::ClientProtocol;

pub struct Registry {
	items: Slab<Arc<Client>>,
}


impl Registry {
	pub fn new(start_from: usize, capacity: usize) -> Registry {
		Registry {
			items: Slab::new_starting_at(mio::Token(start_from), capacity)
		}
	}

	pub fn add(&mut self, server_token: mio::Token, socket: ClientProtocol) -> Result<mio::Token> {
		let token = self.items
			.insert_with(|token| {
					Arc::new(Client::new(
						server_token,
						token,
						socket))
				});

		match token {
			Some(token) => Ok(token),
			None => Error::new("Cannot add the Client to the Registry").result()
		}
	}

	pub fn remove(&mut self, index: mio::Token) {
		self.items.remove(index);
	}
}


impl Index<mio::Token> for Registry {
	type Output = Arc<Client>;

	fn index(&self, index: mio::Token) -> &Self::Output {
		&self.items[index]
	}
}

impl IndexMut<mio::Token> for Registry {
	fn index_mut(&mut self, index: mio::Token) -> &mut Arc<Client> {
		&mut self.items[index]
	}
}
