//! Log configuration

use wrust_types::{Error, Result};
use wrust_types::conf::{Conf, FromConfig};


/// Logger settings
#[derive(Clone)]
pub struct LogConf {
	/// Logging level
	pub level: String,
	/// Colorize output
	pub colorize: bool,
}


impl FromConfig for LogConf {
	// Load settings from the config
	fn from_config(config: &Conf, xpath: &str) -> Result<Self> {
		// Determine where xpath is reference to
		let xpath = match config.resolve_reference(xpath) {
			Some(path) => path,
			None => return Err(Error::general("Cannot load logger configuration").because(format!("Reference or group is not found at '{}'", xpath))),
		};

		// Get logging level
		let level = config.get().lookup_str_or(&format!("{}.level", xpath), "info");
		// Get colorize option
		let colorize = config.get().lookup_boolean_or(&format!("{}.colorize", xpath), false);

		Ok(LogConf {
			level: level.to_string(),
			colorize: colorize,
		})
	}
}
