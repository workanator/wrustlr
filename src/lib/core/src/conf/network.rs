//! Network configuration

use std::str::FromStr;
use wrust_types::{Error, Result};
use wrust_types::net::Protocol;
use wrust_conf::{Conf, FromConf};


/// Socket configuration
#[derive(Clone)]
pub struct SocketConf {
	/// Network protocol with address associated
	pub protocol: Protocol<NetSocketConf, NetSocketConf, UnixSocketConf>,
}


impl FromConf for SocketConf {
	// Load settings from the config
	fn from_conf(config: &Conf, xpath: &str) -> Result<Self> {
		// Determine where xpath is reference to
		let xpath = match config.resolve_reference(xpath) {
			Some(path) => path,
			None => return Error::new(format!("Reference or group is not found at '{}'", xpath)).result(),
		};

		// Read protocol name
		let protocol = match config.lookup_str(&format!("{}.protocol", &xpath)) {
			Some(name) => name.to_uppercase(),
			None => return Error::new(format!("Protocol is undefined at '{}'", xpath)).result(),
		};

		// Read other settings based on protocol type
		let protocol = match Protocol::from_str(&protocol) {
			Ok(Protocol::Tcp(_)) => match NetSocketConf::from_conf(config, &xpath) {
				Ok(addr) => Protocol::Tcp(addr),
				Err(msg) => return Err(msg),
			},
			Ok(Protocol::Udp(_)) => match NetSocketConf::from_conf(config, &xpath) {
				Ok(addr) => Protocol::Udp(addr),
				Err(msg) => return Err(msg),
			},
			Ok(Protocol::Unix(_)) => match UnixSocketConf::from_conf(config, &xpath) {
				Ok(addr) => Protocol::Unix(addr),
				Err(msg) => return Err(msg),
			},
			Err(error) => return Error::new(format!("Invalid protocol at '{}': {}", xpath, error)).result(),
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


impl FromConf for NetSocketConf {
	// Load settings from the config
	fn from_conf(config: &Conf, xpath: &str) -> Result<Self> {
		Ok(NetSocketConf {
			address: match config.lookup_str(&format!("{}.address", xpath)) {
				Some(ip) => {
					if ip == "*" {
						"0.0.0.0".to_string()
					}
					else {
						ip.to_string()
					}
				},
				None => return Error::new(format!("Address is required at '{}'", xpath)).result(),
			},
			port: match config.lookup_integer32(&format!("{}.port", xpath)) {
				Some(port) => port as u16,
				None => return Error::new(format!("Port is required at '{}'", xpath)).result(),
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


impl FromConf for UnixSocketConf {
	// Load settings from the config
	fn from_conf(config: &Conf, xpath: &str) -> Result<Self> {
		Ok(UnixSocketConf {
			path: match config.lookup_str(&format!("{}.path", xpath)) {
				Some(path) => path.to_string(),
				None => return Error::new(format!("Path is required at '{}'", xpath)).result(),
			}
		})
	}
}
