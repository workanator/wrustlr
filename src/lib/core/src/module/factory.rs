use std::collections::HashMap;
use wrust_types::{Error, Result};
use wrust_types::conf::Conf;
use wrust_types::module::Category;
use wrust_types::module::stream;

pub struct Factory {
	config: Conf,
	streaming: HashMap<String, Box<Fn(&Conf, &String) -> Box<stream::Behavior>>>,
}

impl Factory {
	pub fn new(config: &Conf) -> Factory {
		Factory {
			config: config.clone(),
			streaming: HashMap::new(),
		}
	}

	pub fn register_stream<F: 'static>(&mut self, category: Category, name: String, version: String, producer: F)
		where F: Fn(&Conf, &String) -> Box<stream::Behavior> {
		info!("Registered module {:?}:{} v{}", category, name, version);
		self.streaming.insert(name, Box::new(producer));
	}

	pub fn produce_stream(&self, name: &String, xpath_base: &String) -> Result<Box<stream::Behavior>> {
		match self.streaming.get(name) {
			Some(new) => {
				debug!("Instantiate the streaming module '{}' using XPath base '{}'", name, xpath_base);
				Ok(new(&self.config, xpath_base))
			},
			None => Err(Error::general("Cannot instantiate module").because(format!("The streaming module '{}' is not registered", name))),
		}
	}
}
