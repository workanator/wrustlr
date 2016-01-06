//! Server socket configuration

use std::net::SocketAddr;
use wrust_types::{Error, Result};
use wrust_types::net::Protocol;
use wrust_conf::{Conf, FromConf};
use ::conf::{ModuleConf, SocketConf};


/// Server socket configuration
#[derive(Clone)]
pub struct ServerConf {
	/// Listening socket configuration
	pub listen: SocketConf,
	/// Stream forwarding module
	pub forward: ModuleConf,
}


/// Server socket configuration helper functions
impl ServerConf {
	/// Produce socket address from settings
	pub fn socket_address(&self) -> Result<SocketAddr> {
		if let Protocol::Tcp(ref details) = self.listen.protocol {
			let addr = format!("{}:{}", details.address, details.port);
			match addr.parse() {
				Ok(sockaddr) => Ok(sockaddr),
				Err(_) => Error::new(format!("Invalid socket address {}", addr)).result()
			}
		}
		else if let Protocol::Udp(ref details) = self.listen.protocol {
			let addr = format!("{}:{}", details.address, details.port);
			match addr.parse() {
				Ok(sockaddr) => Ok(sockaddr),
				Err(_) => Error::new(format!("Invalid socket address {}", addr)).result()
			}
		}
		else {
			Error::new("Non-TCP or non-UDP address").result()
		}
	}
}


impl FromConf for ServerConf {
	// Load settings from the config
	fn from_conf(config: &Conf, xpath: &str) -> Result<Self> {
		// Determine where xpath is reference to
		let xpath = match config.resolve_reference(xpath) {
			Some(path) => path,
			None => return Error::new(format!("Reference or group is not found at '{}'", xpath)).result(),
		};

		// Read listening socket configuration
		let listen_conf = try!(SocketConf::from_conf(&config, &format!("{}.listen", xpath)));
		// Read traffic forward target
		let forward_conf = try!(ModuleConf::from_conf(&config, &format!("{}.forward", xpath)));

		Ok(ServerConf {
			listen: listen_conf,
			forward: forward_conf,
		})
	}
}
