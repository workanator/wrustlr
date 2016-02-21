//! Core configuration

use wrust_types::{Error, Result};
use wrust_conf::{Conf, FromConf};


/// Core settings
pub struct CoreConf {
	/// Worker count
	pub worker_count: u16,
}


impl FromConf for CoreConf {
	// Load settings from the config
	fn from_conf(config: &Conf, xpath: &str) -> Result<Self> {
		// Check if core section exists
		if None == config.lookup(xpath) {
			return Error::new(format!("Group does not exist at path '{}'", xpath)).result();
		}

		// Read work queue settings
		let worker_count = match config.lookup_integer32(&format!("{}.worker_count", xpath)) {
			Some(count) => count as u16,
			None => return Error::new(format!("Worker Count is required at '{}'", xpath)).result(),
		};

		Ok(CoreConf {
			worker_count: worker_count,
		})
	}
}
