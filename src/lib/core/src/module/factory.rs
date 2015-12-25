use std::collections::HashMap;
use wrust_types::{Error, Result};
use wrust_types::conf::Conf;
use wrust_types::module::Category;
use wrust_types::module::stream;

pub struct Factory {
	config: Conf,
	streams: HashMap<String, Box<Fn(&Conf, &String) -> Box<stream::Behavior>>>,
}

impl Factory {
	pub fn new(config: &Conf) -> Factory {
		Factory {
			config: config.clone(),
			streams: HashMap::new(),
		}
	}

	pub fn register<F: 'static>(&mut self, category: Category, name: String, version: String, producer: F)
		where F: Fn(&Conf, &String) -> Box<stream::Behavior> {
		info!("Registered module {:?}:{} v{}", category, name, version);
		match category {
			Category::Stream => self.streams.insert(name, Box::new(producer)),
		};
	}

	pub fn produce(&self, category: Category, name: &String, xpath_base: &String) -> Result<Box<stream::Behavior>> {
		debug!("Instantiate module {:?}:{} using XPath base '{}'", category, name, xpath_base);
		match category {
			Category::Stream => match self.streams.get(name) {
				Some(new) => Ok(new(&self.config, xpath_base)),
				None => Err(Error::general("Cannot instantiate module").because(format!("Module {:?}:{} is not registered", category, name))),
			},
		}
	}
}
