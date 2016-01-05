//! Trait for loadable from configuration file objects.

use std::ops::Deref;
use std::rc::Rc;
use std::path::Path;
use config::reader;
use config::types::{Config, Value};
use wrust_types::{Error, Result};

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
			Err(err) => Error::new("Error reading config file").because(err).result()
		}
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


impl Deref for Conf {
	type Target = Config;

	fn deref(&self) -> &Config {
		&self.instance
	}
}


/// Loadable from configuration file.  
/// The trait is implement for Vec so it's very simple to load the list of similar settings into the vector.
pub trait FromConf: Sized {
	/// Load settings from `config`uration file using `xpath` as the base.
	fn from_conf(config: &Conf, xpath: &str) -> Result<Self>;
}


impl<T: FromConf> FromConf for Vec<T> {
	fn from_conf(config: &Conf, xpath: &str) -> Result<Vec<T>> {
		// Test if at the xpath is the array and get the numer of elements in it
		let count = match config.lookup(xpath) {
			Some(&Value::List(ref collection)) => collection.len(),
			_ => return Error::new(format!("Expected array at '{}' but found nothing", xpath)).result(),
		};

		// Read array items
		let mut items: Vec<T> = Vec::new();

		for i in 0..count {
			match T::from_conf(config, &format!("{}.[{}]", xpath, i)) {
				Ok(item) => items.push(item),
				Err(msg) => return Err(msg),
			};
		}

		Ok(items)
	}
}
