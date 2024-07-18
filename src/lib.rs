use serde::{Deserialize, Serialize};

mod browser;
pub use browser::spawn as spawn_browser;
pub use browser::Command;
use tracing::debug;

/// Represents the current configuration for this display.
#[derive(Debug, Deserialize, Serialize)]
pub struct State {
    pub enabled: bool,
    pub mode: Mode,
    pub power: bool,
    pub scale: f64,
    pub transform: String,
    pub scenario: Scenario,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Mode {
    pub width: u64,
    pub height: u64,
    pub refresh: f64,
    pub picture_aspect_ratio: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "name", content = "args", rename_all = "lowercase")]
pub enum Scenario {
    URL { url: String },
    Blank,
    Video { url: String },
}

// Contains more general information about the display
// (make, model, serial, supported modes)
#[derive(Debug, Deserialize, Serialize)]
pub struct Info {
    pub make: String,
    pub model: String,
    pub modes: Vec<Mode>,
    pub name: String,
    pub serial: String,
}
