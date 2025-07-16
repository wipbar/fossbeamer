use std::io::Cursor;

use eyre::eyre;
use eyre::Context;
use tracing::{debug, instrument, warn};

use crate::display::Info;
use crate::display::Mode;
use crate::get_ips;

// Try to read information about the display via DRM.
pub fn display_info_drm(base_path: &std::path::Path) -> eyre::Result<Info> {
    let edid_data = std::fs::read(base_path.join("edid")).wrap_err("reading edid data")?;

    // parse edid
    let edid = edid_rs::parse(&mut Cursor::new(edid_data))
        .map_err(|err| eyre!("failed to parse edid: {err}"))?;

    // parse modes.
    let modes_data =
        std::fs::read_to_string(base_path.join("modes")).wrap_err("failed to read modes")?;

    let modes: Vec<_> = modes_data
        .lines()
        .filter_map(|line| match parse_x_y(line) {
            Some(mode) => Some(mode),
            None => {
                warn!(%line, "failed to parse mode line");
                None
            }
        })
        .collect();

    Ok(gen_info(edid, modes))
}

/// Assembles [Info] from edid and modes.
fn gen_info(edid: edid_rs::EDID, modes: Vec<Mode>) -> Info {
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
        extra: Some(serde_json::json!({
            "ip_addrs": get_ips(),
        })),
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

#[instrument()]
fn parse_x_y(s: &str) -> Option<Mode> {
    let (x, y) = s.split_once('x')?;

    Some(Mode {
        width: x.parse::<u64>().ok()?,
        height: y.parse::<u64>().ok()?,
    })
}
#[cfg(test)]
mod test {
    use crate::display::Mode;

    use super::parse_x_y;

    #[test]
    fn test_mode() {
        assert_eq!(
            Mode {
                width: 1920,
                height: 1080
            },
            parse_x_y("1920x1080").expect("must parse")
        );
    }
}
