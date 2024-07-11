use std::path::PathBuf;
use std::{env, sync::mpsc::channel};

use config::Config;

mod common;
mod config;
mod mqtt;
mod system;

fn main() -> wry::Result<()> {
    let command_parameters = parse_command_parameters();

    if let Some(url) = command_parameters.address {
        print!("Acquiring the CPU serial number...\r");
        let serial = system::get_cpu_serial().unwrap();

        println!("The CPU serial number is {}", serial);

        println!("Loading the config...");
        let config = Config::load(command_parameters.default_config_path).unwrap();

        println!("Opening {}...", url);

        let (sender, receiver) = channel();

        let listener = mqtt::Listener {
            id: config.id.unwrap_or(serial),
            host: config.host,
            port: config.port,
            sender,
        };

        listener.start().unwrap();
        fossbeamer::spawn_browser(url, Some(receiver))
    } else {
        println!("Fossbeamer requires a URL as a parameter.");
        Ok(())
    }
}

fn parse_command_parameters() -> CommandParameters {
    let mut default_config_path: Option<PathBuf> = None;
    let mut address: Option<String> = None;
    for string in env::args() {
        if string.starts_with("http") {
            address = Some(string)
        } else if string.starts_with("--default-config=") {
            let dropped_str = &string.as_str()[17..string.len() - 1];
            default_config_path = Some(PathBuf::from(dropped_str.to_string()))
        }
    }
    CommandParameters {
        default_config_path,
        address,
    }
}

struct CommandParameters {
    default_config_path: Option<PathBuf>,
    address: Option<String>,
}
