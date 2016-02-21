//! Log configuration

use std::str::FromStr;
use log::LogLevelFilter;
use wrust_types::{Error, Result};
use wrust_conf::{Conf, FromConf};


/// Logging device
#[derive(Debug, Clone)]
pub enum LogDevice {
	/// Log to stderr with colorize option
	Stderr(bool),
	/// Log to file(s) in directory given and respect rotation size if set
	File(Option<String>, Option<usize>)
}


/// Logger settings
#[derive(Debug, Clone)]
pub struct LogConf {
	/// Logging device
	pub device: LogDevice,
	/// Logging level
	pub level: LogLevelFilter,
}


impl FromConf for LogConf {
	// Load settings from the config
	fn from_conf(config: &Conf, xpath: &str) -> Result<Self> {
		// Check if core section exists
		if None == config.lookup(xpath) {
			return Error::new(format!("Group does not exist at path '{}'", xpath)).result();
		}

		// Get device option
		let device = match config.lookup_str(&format!("{}.device", xpath)) {
			Some(device) => device.trim().to_lowercase(),
			None => return Error::new(format!("Logging device is required at '{}'", xpath)).result()
		};

		let log_device: LogDevice = if device == "stderr" {
			// Get colorize option
			let colorize = config.lookup_boolean_or(&format!("{}.colorize", xpath), false);

			LogDevice::Stderr(colorize)
		}
		else if device == "file" {
			// Get directory option
			let directory = match config.lookup_str(&format!("{}.directory", xpath)) {
				Some(directory) => Some(directory.to_string()),
				None => None
			};

			// Get rotate size
			let rotate_size = match config.lookup_integer64(&format!("{}.rotate_size", xpath)) {
				Some(size) => Some(size as usize),
				None => None
			};

			LogDevice::File(directory, rotate_size)
		}
		else {
			return Error::new(format!("Unknown logging device {} at '{}.device'", device, xpath)).result()	
		};

		// Get logging level
		let level = config.lookup_str_or(&format!("{}.level", xpath), "info");
		let log_level: LogLevelFilter = match LogLevelFilter::from_str(level) {
			Ok(log_level) => log_level,
			Err(_) => return Error::new(format!("Invalid loging level at '{}'.level", xpath)).result()
		};

		Ok(LogConf {
			device: log_device,
			level: log_level,
		})
	}
}
