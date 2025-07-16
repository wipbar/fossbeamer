use clap::Parser;
use config::Config;
use eyre::Context;
use std::{path::PathBuf, time::Duration};
use tracing::{debug, info, warn};

use fossbeamer::{
    browser::BrowserWindow,
    display::{self, Display},
};

mod common;
mod config;
mod listener;
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

    #[arg(long, default_value = "screens")]
    mqtt_topic_prefix: String,
}

fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;
    setup_tracing();

    let cli = Cli::parse();

    // Try peeking at the EDID data of the connected display.
    // This is currently hardcoded to a single display at card0-HDMI-A-1,
    // as that's what's running on the CM3's.
    // FUTUREWORK: enumerate monitors somehow, and handle multiple
    let display_info = match display::display_info_drm(std::path::Path::new(
        "/sys/class/drm/card0/card0-HDMI-A-1",
    )) {
        Ok(display_info) => display_info,
        Err(err) => {
            warn!(%err, "unable to read edid from DRM, fallback using machine ID as serial");
            let machine_id = system::get_machine_id().wrap_err("getting machine id")?;
            display::Info {
                make: "Unknown".into(),
                modes: vec![],
                model: "Unknown".into(),
                name: "Unknown".into(),
                serial: machine_id,
            }
        }
    };

    info!(
        machine.make=%display_info.make,
        machine.model=%display_info.model,
        machine.name=%display_info.name,
        machine.serial=%display_info.serial,
        "created DisplayInfo"
    );

    debug!("Loading the config");
    let config =
        Config::load(cli.default_config_path.map(PathBuf::from)).context("loading config")?;

    // Initialize the event loop

    // Initialize browser window
    let browser_window = BrowserWindow::new();
    info!(url=%cli.url, "Opening URL");
    browser_window
        .run_scenario(display::Scenario::URL { url: cli.url })
        .wrap_err("running scenario")?;

    // Initialize MQTT listener
    let listener = listener::MQTT::new(
        config.id.unwrap_or_else(|| display_info.serial.clone()),
        config.host,
        config.port,
        cli.mqtt_topic_prefix,
    )?;

    // Register the display
    listener
        .add_display(browser_window, &display_info)
        .context("adding display")?;

    loop {
        std::thread::sleep(Duration::from_secs(1));
    }
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
