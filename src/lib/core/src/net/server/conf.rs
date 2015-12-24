//! Server socket configuration

use std::net::SocketAddr;
use wrust_types::{Error, Result};
use wrust_types::conf::{Conf, FromConfig};
use wrust_types::net::Protocol;
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
				Err(_) => Err(Error::general("Invalid socket address").because(addr))
			}
		}
		else if let Protocol::Udp(ref details) = self.listen.protocol {
			let addr = format!("{}:{}", details.address, details.port);
			match addr.parse() {
				Ok(sockaddr) => Ok(sockaddr),
				Err(_) => Err(Error::general("Invalid socket address").because(addr))
			}
		}
		else {
			Err(Error::general("Non-TCP or non-UDP address"))
		}
	}
}


impl FromConfig for ServerConf {
	// Load settings from the config
	fn from_config(config: &Conf, xpath: &str) -> Result<Self> {
		// Determine where xpath is reference to
		let xpath = match config.resolve_reference(xpath) {
			Some(path) => path,
			None => return Err(Error::general("Cannot load server configuration").because(format!("Reference or group is not found at '{}'", xpath))),
		};

		// Read listening socket configuration
		let listen_conf = try!(SocketConf::from_config(&config, &format!("{}.listen", xpath)));
		// Read traffic forward target
		let forward_conf = try!(ModuleConf::from_config(&config, &format!("{}.forward", xpath)));

		Ok(ServerConf {
			listen: listen_conf,
			forward: forward_conf,
		})
	}
}
