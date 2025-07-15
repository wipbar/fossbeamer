use serde::{Deserialize, Serialize};

mod browser;
pub use browser::spawn as spawn_browser;
pub use browser::Command;
use tracing::debug;
use tracing::instrument;

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

impl Mode {
    #[instrument()]
    pub fn parse_x_y(s: &str) -> Option<Self> {
        let (x, y) = s.split_once('x')?;

        Some(Mode {
            width: x.parse::<u64>().ok()?,
            height: y.parse::<u64>().ok()?,
        })
    }
}

#[cfg(test)]
mod test {
    use super::Mode;

    #[test]
    fn test_mode() {
        assert_eq!(
            Mode {
                width: 1920,
                height: 1080
            },
            Mode::parse_x_y("1920x1080").expect("must parse")
        );
    }
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

impl Info {
    pub fn from_edid_and_modes(edid: edid_rs::EDID, modes: Vec<Mode>) -> Self {
        let mut value = Info {
            make: format!(
                "{}{}{}",
                edid.product.manufacturer_id.0,
                edid.product.manufacturer_id.1,
                edid.product.manufacturer_id.2
            ),
            modes,
            model: format!("{}", edid.product.product_code),
            name: "Unknown".to_string(),
            serial: "Unknown".to_string(),
        };

        for descriptor in edid.descriptors.0 {
            match descriptor {
                edid_rs::MonitorDescriptor::SerialNumber(sn) => value.serial = sn,
                edid_rs::MonitorDescriptor::OtherString(s) => {
                    debug!(%s, "MonitorDescriptor::OtherString")
                }
                edid_rs::MonitorDescriptor::RangeLimits { .. } => {}
                edid_rs::MonitorDescriptor::MonitorName(name) => value.name = name,
                edid_rs::MonitorDescriptor::Undefined(_, _) => {}
                edid_rs::MonitorDescriptor::ManufacturerDefined(_, _) => {}
            }
        }

        value
    }
}
