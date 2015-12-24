//! Trait for loadable from configuration file objects.

use std::rc::Rc;
use std::path::Path;
use config::reader;
use config::types::{Config, Value};
use ::{Error, Result};

/// Shared configuration
#[derive(Debug, Clone)]
pub struct Conf {
	instance: Rc<Config>,
}


impl Conf {
	/// `from_file` loads configuration from file.
	pub fn from_file(path: &Path) -> Result<Conf> {
		match reader::from_file(path) {
			Ok(config) => Ok(Conf {
				instance: Rc::new(config),
			}),
			Err(err) => make_err!(err)
		}
	}

	/// Get the configuration instance
	pub fn get(&self) -> &Config {
		&self.instance
	}

	/// Resolve the `xpath` provided. Resolution rules are:  
	/// * If the element at `xpath` is the scalar value then it's the reference, return the value of that element
	/// * If the element at `xpath` is the group then return `xpath`
	/// * Otherwise `xpath` is invalid
	pub fn resolve_reference(&self, xpath: &str) -> Option<String> {
		match self.instance.lookup(xpath) {
			Some(&Value::Svalue(ref target)) => Some(target.to_string()),
			Some(&Value::Group(_)) => Some(xpath.to_string()),
			_ => None,
		}
	}
}


/// Loadable from configuration file.  
/// The trait is implement for Vec so it's very simple to load the list of similar settings into the vector.
pub trait FromConfig: Sized {
	/// Load settings from `config`uration file using `xpath` as the base.
	fn from_config(config: &Conf, xpath: &str) -> Result<Self>;
}


impl<T: FromConfig> FromConfig for Vec<T> {
	fn from_config(config: &Conf, xpath: &str) -> Result<Vec<T>> {
		// Test if at the xpath is the array and get the numer of elements in it
		let count = match config.get().lookup(xpath) {
			Some(&Value::List(ref collection)) => collection.len(),
			_ => return make_err!(format!("Expected array at '{}' but found nothing", xpath)),
		};

		// Read array items
		let mut items: Vec<T> = Vec::new();

		for i in 0..count {
			match T::from_config(config, &format!("{}.[{}]", xpath, i)) {
				Ok(item) => items.push(item),
				Err(msg) => return Err(msg),
			};
		}

		Ok(items)
	}
}
