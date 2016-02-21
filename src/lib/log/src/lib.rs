extern crate log;
extern crate flexi_logger;
extern crate ansi_term;
extern crate wrust_types;
extern crate wrust_conf;

pub mod conf;

use log::{LogLevel, LogRecord};
use wrust_types::{Error, Result};
use wrust_conf::{Conf, FromConf};
use conf::{LogConf, LogDevice};


/// Initialize logging system using configuration given
pub fn init(config: LogConf) -> Result<()> {
	use flexi_logger::{self, LogConfig};

	// Start from the default logger configuration
	let mut flexi_config = LogConfig::new();
	flexi_config.print_message = false;
	flexi_config.duplicate_error = false;
	flexi_config.duplicate_info = false;

	// Setup flexi logger based on log device
	match config.device {
		LogDevice::Stderr(colorize) => {
			flexi_config.format = if colorize {
				colorized_format
			}
			else {
				simple_format
			};
		},
		LogDevice::File(directory_opt, rotate_size_opt) => {
			flexi_config.format = simple_format;
			flexi_config.log_to_file = true;
			flexi_config.directory = directory_opt;
			flexi_config.rotate_over_size = rotate_size_opt;
		},
	};

	// Initialize logger
	match flexi_logger::init(flexi_config, Some(config.level.to_string())) {
		Ok(_) => Ok(()),
		Err(_) => Error::new("Logger initialization failed").result()
	}
}


pub fn init_from_conf(config: &Conf, xpath: &str) -> Result<()> {
	// Initialize logger
	match LogConf::from_conf(&config, xpath) {
		Ok(settings) => init(settings),
		Err(err) => Error::new("Logger configuration failed").because(err).result()
	}
}


fn colorized_format(record: &LogRecord) -> String {
	use ansi_term::Colour::{Red, Green, Yellow, Purple, Cyan};

	match record.level() {
		LogLevel::Error => format!("{} {} in {} ({}:{})", Red.bold().paint("[!]"), record.args(), record.location().module_path(), record.location().file(), record.location().line()),
		LogLevel::Warn => format!("{} {} in {} ({}:{})", Yellow.paint("[W]"), record.args(), record.location().module_path(), record.location().file(), record.location().line()),
		LogLevel::Info => format!("{}: {}", Green.paint("[I]"), record.args()),
		LogLevel::Debug => format!("{}: {}", Purple.paint("[D]"), record.args()),
		LogLevel::Trace => format!("{}: {}", Cyan.paint("[T]"), record.args()),
	}
}


fn simple_format(record: &LogRecord) -> String {
	match record.level() {
		LogLevel::Error => format!("[!] {} in {} ({}:{})", record.args(), record.location().module_path(), record.location().file(), record.location().line()),
		LogLevel::Warn => format!("[W] {} in {} ({}:{})", record.args(), record.location().module_path(), record.location().file(), record.location().line()),
		LogLevel::Info => format!("[I]: {}", record.args()),
		LogLevel::Debug => format!("[D]: {}", record.args()),
		LogLevel::Trace => format!("[E]: {}", record.args()),
	}
}
