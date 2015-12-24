//! Module alma mater.

pub mod stream;

use ::conf::Conf;


/// Module category
#[derive(Debug)]
pub enum Category {
	/// Stream processing module
	Stream,
}


/// The way Wrustlr can obtain information about the module
/// and create new instances of it.
pub trait Facility {
	/// Create a new instance of the module using the configuration
	/// from `config` and using `xpath_base` as a base for all XPath requests.
	fn new(config: &Conf, xpath_base: &String) -> Self;

	/// The module unique name which is used to identify modules
	/// in the configuration.
	fn name() -> String;

	/// The module version.
	fn version() -> String;

	/// The module category
	fn category() -> Category;
}


