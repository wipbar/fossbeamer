use std::io::BufRead;
use std::sync::mpsc::channel;
use std::{io::Cursor, path::PathBuf};

use clap::Parser;

use config::Config;
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
}

fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;
    setup_tracing();

    let cli = Cli::parse();

    let mut display_info: Option<Info> = None;
    // Try peeking at the EDID data of the connected display.
    // This is currently hardcoded to a single display at card0-HDMI-A-1,
    // as that's what's running on the CM3's.
    if let Ok(edid_data) = std::fs::read("/sys/class/drm/card0/card0-HDMI-A-1/edid") {
        match edid_rs::parse(&mut Cursor::new(edid_data)) {
            Ok(edid) => {
                // construct a new display_info by converting the EDID.
                let mut di: Info = edid.into();

                // parse modes from the `modes` file in the same dir.
                di.modes = match std::fs::read("/sys/class/drm/card0/card0-HDMI-A-1/modes") {
                    Ok(modes_data) => {
                        modes_data
                            .lines()
                            .flat_map(|x| {
                                let x = x.unwrap();

                                let xys = x.split('x').collect::<Vec<_>>();

                                if xys.len() != 2 {
                                    warn!("invalid mode line");
                                    return None;
                                }
                                let (x, y) = (xys[0].parse::<u64>(), xys[1].parse::<u64>());

                                Some(match (x, y) {
                                    (Ok(width), Ok(height)) => Mode {
                                        width,
                                        height,
                                        refresh: 0.0,
                                        picture_aspect_ratio: (width as f64 / height as f64)
                                            .to_string(), // TODO: why do we have this?
                                    },
                                    _ => {
                                        warn!("invalid mode line");
                                        return None;
                                    }
                                })
                            })
                            .collect()
                    }
                    Err(e) => {
                        warn!(err=%e, "failed to read modes");
                        // this should really not fail, but if it does, set the modes to an empty list.
                        vec![]
                    }
                };

                display_info = Some(di)
            }
            Err(e) => {
                warn!(err=%e, "failed to parse edid");
            }
        }
    }

    // If we couldn't read the EDID, use the machine ID as serial.
    let display_info = match display_info {
        Some(display_info) => display_info,
        None => {
            let machine_id = system::get_machine_id().context("getting machine id")?;
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

    let listener = mqtt::Listener {
        id: config.id.unwrap_or(display_info.serial),
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
