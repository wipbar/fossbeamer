use std::path::PathBuf;
use std::sync::mpsc::channel;

use clap::Parser;

use config::Config;
use eyre::Context;
use tracing::{debug, info};

mod common;
mod config;
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

fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;
    setup_tracing();

    let cli = Cli::parse();

    let machine_id = system::get_machine_id().context("getting machine id")?;

    debug!("Loading the config");
    let config =
        Config::load(cli.default_config_path.map(PathBuf::from)).context("loading config")?;

    info!(url=%cli.url, "Opening URL");

    let (tx, rx) = channel();

    let listener = mqtt::Listener {
        id: config.id.unwrap_or(machine_id),
        host: config.host,
        port: config.port,
        sender: tx,
    };

    listener.start().context("starting the mqtt listener")?;
    fossbeamer::spawn_browser(cli.url, rx)?;

    Ok(())
}

pub fn setup_tracing() {
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
    let subscriber = tracing_subscriber::registry()
        .with(
            EnvFilter::builder()
                .with_default_directive(tracing::Level::INFO.into())
                .from_env()
                .expect("Invalid RUST_LOG"),
        )
        .with(
            tracing_subscriber::fmt::Layer::new()
                .with_writer(std::io::stderr)
                .compact(),
        );

    subscriber.try_init().expect("failed to setup tracing");
}
