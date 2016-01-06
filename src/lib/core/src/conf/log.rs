//! Log configuration

use wrust_types::{Error, Result};
use wrust_conf::{Conf, FromConf};


/// Logger settings
#[derive(Clone)]
pub struct LogConf {
	/// Logging level
	pub level: String,
	/// Colorize output
	pub colorize: bool,
}


impl FromConf for LogConf {
	// Load settings from the config
	fn from_conf(config: &Conf, xpath: &str) -> Result<Self> {
		// Determine where xpath is reference to
		let xpath = match config.resolve_reference(xpath) {
			Some(path) => path,
			None => return Error::new(format!("Reference or group is not found at '{}'", xpath)).result(),
		};

		// Get logging level
		let level = config.lookup_str_or(&format!("{}.level", xpath), "info");
		// Get colorize option
		let colorize = config.lookup_boolean_or(&format!("{}.colorize", xpath), false);

		Ok(LogConf {
			level: level.to_string(),
			colorize: colorize,
		})
	}
}
