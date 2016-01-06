//! Module configuration

use wrust_types::{Error, Result};
use wrust_conf::{Conf, FromConf};


/// Module configuration
#[derive(Clone)]
pub struct ModuleConf {
	/// Module name
	pub name: String,
	/// Base XPath where to start to read settings
	pub xpath: String,
}


impl FromConf for ModuleConf {
	// Load settings from the config
	fn from_conf(config: &Conf, xpath: &str) -> Result<Self> {
		// Determine where xpath is reference to
		let xpath = match config.resolve_reference(xpath) {
			Some(path) => path,
			None => return Error::new(format!("Reference or group is not found at '{}'", xpath)).result(),
		};

		// Read module name
		let module = match config.lookup_str(&format!("{}.module", xpath)) {
			Some(name) => name.to_string(),
			None => return Error::new(format!("Module name is required at '{}'", xpath)).result(),
		};

		Ok(ModuleConf {
			name: module,
			xpath: xpath,
		})
	}
}
