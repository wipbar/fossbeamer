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

impl From<edid_rs::EDID> for Info {
    fn from(value: edid_rs::EDID) -> Self {
        let mut display_info = Info {
            make: format!(
                "{}{}{}",
                value.product.manufacturer_id.0,
                value.product.manufacturer_id.1,
                value.product.manufacturer_id.2
            ),
            modes: vec![],
            model: format!("{}", value.product.product_code),
            name: "Unknown".to_string(),
            serial: "Unknown".to_string(),
        };

        for descriptor in value.descriptors.0 {
            match descriptor {
                edid_rs::MonitorDescriptor::SerialNumber(sn) => display_info.serial = sn,
                edid_rs::MonitorDescriptor::OtherString(s) => {
                    debug!(%s, "MonitorDescriptor::OtherString")
                }
                edid_rs::MonitorDescriptor::RangeLimits { .. } => {}
                edid_rs::MonitorDescriptor::MonitorName(name) => display_info.name = name,
                edid_rs::MonitorDescriptor::Undefined(_, _) => {}
                edid_rs::MonitorDescriptor::ManufacturerDefined(_, _) => {}
            }
        }

        display_info
    }
}
