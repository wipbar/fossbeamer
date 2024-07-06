use std::{env, sync::mpsc::channel};

mod mqtt;

fn main() -> wry::Result<()> {
    if let Some(url) = env::args().nth(1) {
        println!("Opening {}", url);

        let (sender, receiver) = channel();

        let listener = mqtt::Listener {
            id: "screen".to_string(),
            host: "localhost".to_string(),
            port: 1883,
            sender,
        };

        listener.start().unwrap();
        fossbeamer::spawn_browser(url, Some(receiver))
    } else {
        println!("BornScreen requires a URL as a parameter.");
        Ok(())
    }
}
