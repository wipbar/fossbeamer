use std::sync::mpsc::channel;
use std::{io::Cursor, path::PathBuf};

use clap::Parser;

use config::Config;
use eyre::eyre;
use eyre::Context;
use fossbeamer::{Info, Mode};
use tracing::{debug, info, warn};

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

    #[arg(long)]
    mqtt_topic_prefix: Option<String>,
}

// Try to read information about the display via DRM.
fn display_info_drm(base_path: &std::path::Path) -> eyre::Result<Info> {
    let edid_data = std::fs::read(base_path.join("edid")).wrap_err("reading edid data")?;

    // parse edid
    let edid = edid_rs::parse(&mut Cursor::new(edid_data))
        .map_err(|err| eyre!("failed to parse edid: {err}"))?;

    // parse modes.
    let modes_data =
        std::fs::read_to_string(base_path.join("modes")).wrap_err("failed to read modes")?;

    let modes: Vec<_> = modes_data
        .lines()
        .filter_map(|line| match Mode::parse_x_y(line) {
            Some(mode) => Some(mode),
            None => {
                warn!(%line, "failed to parse mode line");
                None
            }
        })
        .collect();

    Ok(Info::from_edid_and_modes(edid, modes))
}

fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;
    setup_tracing();

    let cli = Cli::parse();

    // Try peeking at the EDID data of the connected display.
    // This is currently hardcoded to a single display at card0-HDMI-A-1,
    // as that's what's running on the CM3's.
    let display_info =
        match display_info_drm(std::path::Path::new("/sys/class/drm/card0/card0-HDMI-A-1")) {
            Ok(display_info) => display_info,
            Err(err) => {
                warn!(%err, "unable to read edid from DRM, fallback using machine ID as serial");
                let machine_id = system::get_machine_id().wrap_err("getting machine id")?;
                Info {
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

    info!(url=%cli.url, "Opening URL");

    let (tx, rx) = channel();

    let listener = mqtt::Listener::new(
        config.id.unwrap_or_else(|| display_info.serial.clone()),
        config.host,
        config.port,
        cli.mqtt_topic_prefix
            .unwrap_or_else(|| "screens".to_string()),
    )?;

    // register our display
    // FUTURWORK: multiple display support
    listener
        .add_display(&display_info, tx)
        .context("adding display")?;

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
