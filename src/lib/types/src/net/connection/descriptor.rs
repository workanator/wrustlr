use std::net::SocketAddr;


/// Client connection descriptor
#[derive(Debug)]
pub struct Descriptor {
	/// Identifier
	id: u32,
	/// Peer address if available
	addr: Option<SocketAddr>,
}


impl Descriptor {
	/// Create a new connection `Descriptor`.
	pub fn new(id: u32, addr: Option<SocketAddr>) -> Descriptor {
		Descriptor {
			id: id,
			addr: addr,
		}
	}

	/// Get identifier of the connection.
	pub fn id(&self) -> u32 {
		self.id
	}

	/// Get peer address of the connection.
	pub fn addr(&self) -> Option<SocketAddr> {
		self.addr
	}
}

#[test]
fn test_descriptor() {
	let desc = Descriptor::new(1, None);

	let id = desc.id();
	assert_eq!(id, 1);

	let addr = desc.addr();
	assert_eq!(addr, None);
}