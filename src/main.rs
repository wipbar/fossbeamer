use std::{env, sync::mpsc::channel};

use config::Config;

mod common;
mod config;
mod mqtt;
mod system;

fn main() -> wry::Result<()> {
    if let Some(url) = env::args().nth(1) {
        print!("Acquiring the CPU serial number...\r");
        let serial = system::get_cpu_serial().unwrap();

        println!("The CPU serial number is {}", serial);

        println!("Loading the config...");
        let config = Config::load().unwrap();

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
