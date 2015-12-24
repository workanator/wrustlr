//! Module configuration

use wrust_types::{Error, Result};
use wrust_types::conf::{Conf, FromConfig};


/// Module configuration
#[derive(Clone)]
pub struct ModuleConf {
	/// Module name
	pub name: String,
	/// Base XPath where to start to read settings
	pub xpath: String,
}


impl FromConfig for ModuleConf {
	// Load settings from the config
	fn from_config(config: &Conf, xpath: &str) -> Result<Self> {
		// Determine where xpath is reference to
		let xpath = match config.resolve_reference(xpath) {
			Some(path) => path,
			None => return Err(Error::general("Cannot load module configuration").because(format!("Reference or group is not found at '{}'", xpath))),
		};

		// Read module name
		let module = match config.get().lookup_str(&format!("{}.module", xpath)) {
			Some(name) => name.to_string(),
			None => return Err(Error::general("Cannot load module configuration").because(format!("Module name is required at '{}'", xpath))),
		};

		Ok(ModuleConf {
			name: module,
			xpath: xpath,
		})
	}
}
