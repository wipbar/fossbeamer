use serde::{Deserialize, Serialize};

mod drm;

pub use drm::display_info_drm;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "name", content = "args", rename_all = "lowercase")]
pub enum Scenario {
    URL { url: String },
    Blank,
    Video { url: String },
}
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

#[derive(Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct Mode {
    pub width: u64,
    pub height: u64,
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

pub trait Display: Send + Sync {
    /// Runs the given scenario on the Display.
    fn run_scenario(&self, scenario: Scenario) -> eyre::Result<()>;
}
