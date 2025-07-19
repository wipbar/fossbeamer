use std::process::Stdio;

use clap::Parser;
use eyre::Context;
use fossbeamer::display::display_info_drm;
use fossbeamer::setup_tracing;
use tracing::info;

#[derive(Parser)]
#[command(version, about, long_about = "Set the hostname via EDID data")]
struct Cli {
    path: std::path::PathBuf,
}

fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;
    setup_tracing();

    let cli = Cli::parse();

    let info = display_info_drm(&cli.path)?;
    info!(?info, "collected info");

    // use serial
    let serial = info.serial.to_string();

    let mut command = std::process::Command::new("hostnamectl");
    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());
    command.args(["hostname", "--transient", &serial]);
    let output = command.output().wrap_err("spawning process")?;

    if !output.status.success() {
        eyre::bail!("nonzero exit code from hostnamectl");
    }

    Ok(())
}
