use std::{env, sync::mpsc::channel};

use config::Config;

mod mqtt;
mod config;

fn main() -> wry::Result<()> {
    if let Some(url) = env::args().nth(1) {
        println!("Loading the config...");
        let config = Config::load().unwrap();

        println!("Opening {}...", url);

        let (sender, receiver) = channel();

        let listener = mqtt::Listener {
            id: config.id.unwrap_or("screen".to_string()),
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
