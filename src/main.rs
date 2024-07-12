use std::path::PathBuf;
use std::sync::mpsc::channel;

use clap::Parser;

use common::Config;

mod browser;
mod common;
mod config;
mod error;
mod mqtt;
mod system;

#[derive(Parser)]
#[command(
    version,
    about,
    long_about = "Screen software developed for the bar at BornHack"
)]
struct Cli {
    url: String,

    #[arg(long = "default-config")]
    default_config_path: Option<String>,
}

fn main() -> wry::Result<()> {
    let cli = Cli::parse();

    print!("Acquiring the CPU serial number...\r");
    let serial = system::get_cpu_serial().unwrap();
    println!(" {}", serial);

    println!("The CPU serial number is {}", serial);

    let default_config_path = cli.default_config_path.map(|p| PathBuf::from(p));

    println!("Opening {}...", cli.url);

    let (sender, receiver) = channel();

    let listener = mqtt::Listener {
        get_config: move || Config::load(default_config_path.clone()).unwrap(),
        sender,
    };

    listener.start();
    browser::spawn_browser(cli.url, Some(receiver))
}
