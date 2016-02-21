#[macro_use] extern crate log;
extern crate chan_signal;
extern crate wrust_types;
extern crate wrust_conf;
extern crate wrust_module;
extern crate wrust_log;
extern crate wrust_core;
extern crate wrust_mod_echo;

mod constants;

use std::path::Path;
use chan_signal::Signal;
use wrust_conf::{Conf, FromConf};
use wrust_module::Facility;
use wrust_core::net::core::{CoreConf, Core};
use wrust_core::net::server::ServerConf;
use wrust_core::module::Factory;
use wrust_mod_echo as wmod_echo;
use constants::{CONFIG_DIRECTORY, SERVER_CONFIG_NAME};

macro_rules! config_failed {
	($msg:expr, $details:expr) => ({
		error!($msg, $details);
		panic!($msg, $details);
	});
}

fn main() {
	// Load configuration
	let server_config_path = Path::new(CONFIG_DIRECTORY).join(SERVER_CONFIG_NAME);
	let server_config = match Conf::from_file(server_config_path.as_path()) {
		Ok(config) => config,
		Err(msg) => {
			config_failed!("Server configuration load failed with message '{:?}'", msg)
		}
	};

	// Initialize logger
	if let Err(msg) = wrust_log::init_from_conf(&server_config, "log") {
		config_failed!("Server configuration load failed with message '{:?}'", msg)
	}

	// Print welcome message
	info!("Wrustlr v{} ", env!("CARGO_PKG_VERSION"));
	
	// Parse configuration
	let core_settings = match CoreConf::from_conf(&server_config, "core") {
		Ok(settings) => settings,
		Err(msg) => config_failed!("Core settings parse failed with message '{}'", msg)
	};

	let servers: Vec<ServerConf> = match Vec::from_conf(&server_config, "servers") {
		Ok(collection) => collection,
		Err(msg) => config_failed!("Servers parse failed with message '{}'", msg)
	};

	// Load and register modules
	let mut module_factory = Factory::new(&server_config);
	// + echo
	module_factory.register(
		wmod_echo::Module::category(),
		wmod_echo::Module::name(),
		wmod_echo::Module::version(),
		|c: &Conf, xp: &String| { Box::new(wmod_echo::Module::new(c, xp)) });

	// Subscribe to signals we'd like to catch
	let signal_listener = chan_signal::notify(&[Signal::INT, Signal::TERM]);

	// Startup the server
    info!("Normal server startup");

	match Core::start(core_settings, &module_factory, servers) {
		Ok(channel) => {
		    info!("Server ready and listening. Send INT or TERM signal to terminate.");
		    loop {
		    	match signal_listener.recv().unwrap() {
		    		Signal::INT | Signal::TERM => {
					    if let Err(msg) = channel.send("shutdown") {
					    	error!("Send SHUTDOWN request failed with message {}", msg);
					    }
					    else {
							if let Err(msg) = channel.recv() {
								error!("Read SHUTDOWN response failed with message {}", msg);
							}
							else {
							}
					    }

					    break;
		    		},
		    		_ => unreachable!()
		    	}
		    }
		},
		Err(e) => {
			error!("{}", e);
		}
	}
}
