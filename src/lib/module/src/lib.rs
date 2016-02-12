//! Wruet Module alma mater.

extern crate wrust_types;
extern crate wrust_io;
extern crate wrust_conf;

pub mod stream;

use wrust_conf::Conf;


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


