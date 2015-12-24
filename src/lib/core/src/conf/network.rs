//! Network configuration

use std::str::FromStr;
use wrust_types::{Error, Result};
use wrust_types::conf::{Conf, FromConfig};
use wrust_types::net::Protocol;


/// Socket configuration
#[derive(Clone)]
pub struct SocketConf {
	/// Network protocol with address associated
	pub protocol: Protocol<NetSocketConf, NetSocketConf, UnixSocketConf>,
}


impl FromConfig for SocketConf {
	// Load settings from the config
	fn from_config(config: &Conf, xpath: &str) -> Result<Self> {
		// Determine where xpath is reference to
		let xpath = match config.resolve_reference(xpath) {
			Some(path) => path,
			None => return Err(Error::general("Cannot load network socket configuration").because(format!("Reference or group is not found at '{}'", xpath))),
		};

		// Read protocol name
		let protocol = match config.get().lookup_str(&format!("{}.protocol", &xpath)) {
			Some(name) => name.to_uppercase(),
			None => return Err(Error::general("Cannot load network socket configuration").because(format!("Protocol is undefined at '{}'", xpath))),
		};

		// Read other settings based on protocol type
		let protocol = match Protocol::from_str(&protocol) {
			Ok(Protocol::Tcp(_)) => match NetSocketConf::from_config(config, &xpath) {
				Ok(addr) => Protocol::Tcp(addr),
				Err(msg) => return Err(msg),
			},
			Ok(Protocol::Udp(_)) => match NetSocketConf::from_config(config, &xpath) {
				Ok(addr) => Protocol::Udp(addr),
				Err(msg) => return Err(msg),
			},
			Ok(Protocol::Unix(_)) => match UnixSocketConf::from_config(config, &xpath) {
				Ok(addr) => Protocol::Unix(addr),
				Err(msg) => return Err(msg),
			},
			Err(error) => return Err(Error::general("Cannot load network socket configuration").because(format!("Invalid protocol at '{}': {}", xpath, error))),
		};

		Ok(SocketConf {
			protocol: protocol,
		})
	}
}


/// Network socket configuration
#[derive(Clone)]
pub struct NetSocketConf {
	/// IP address
	pub address: String,
	/// Port
	pub port: u16,
}


impl FromConfig for NetSocketConf {
	// Load settings from the config
	fn from_config(config: &Conf, xpath: &str) -> Result<Self> {
		Ok(NetSocketConf {
			address: match config.get().lookup_str(&format!("{}.address", xpath)) {
				Some(ip) => {
					if ip == "*" {
						"0.0.0.0".to_string()
					}
					else {
						ip.to_string()
					}
				},
				None => return Err(Error::general("Cannot load network socket configuration").because(format!("Address is required at '{}'", xpath))),
			},
			port: match config.get().lookup_integer32(&format!("{}.port", xpath)) {
				Some(port) => port as u16,
				None => return Err(Error::general("Cannot load network socket configuration").because(format!("Port is required at '{}'", xpath))),
			}
		})
	}
}


/// UNIX socket configuration
#[derive(Clone)]
pub struct UnixSocketConf {
	/// Filesystem path
	pub path: String,
}


impl FromConfig for UnixSocketConf {
	// Load settings from the config
	fn from_config(config: &Conf, xpath: &str) -> Result<Self> {
		Ok(UnixSocketConf {
			path: match config.get().lookup_str(&format!("{}.path", xpath)) {
				Some(path) => path.to_string(),
				None => return Err(Error::general("Cannot load UNIX socket configuration").because(format!("Path is required at '{}'", xpath))),
			}
		})
	}
}
