//! Core configuration

use wrust_types::{Error, Result};
use wrust_types::conf::{Conf, FromConfig};
use ::conf::LogConf;


/// Core settings
pub struct CoreConf {
	/// Worker count
	pub worker_count: u16,
	/// Logger settings
	pub log: LogConf,
}


impl FromConfig for CoreConf {
	// Load settings from the config
	fn from_config(config: &Conf, xpath: &str) -> Result<Self> {
		// Check if core section exists
		if None == config.get().lookup(xpath) {
			return Err(Error::general("Cannot load core configuration").because(format!("Group does not exist at path '{}'", xpath)));
		}

		// Read work queue settings
		let worker_count = match config.get().lookup_integer32(&format!("{}.worker_count", xpath)) {
			Some(count) => count as u16,
			None => return Err(Error::general("Cannot load core configuration").because(format!("Worker Count is required at '{}'", xpath))),
		};

		// Read logging config
		let log_conf = try!(LogConf::from_config(config, &format!("{}.log", xpath)));

		Ok(CoreConf {
			worker_count: worker_count,
			log: log_conf,
		})
	}
}
