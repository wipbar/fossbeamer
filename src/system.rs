use tracing::{instrument, warn};

const MACHINE_ID_PATH: &'static str = "/etc/machine-id";

/// Returns the system machine id
/// (https://www.freedesktop.org/software/systemd/man/latest/machine-id.html)
/// Falls back to a random uuid if the file doesn't exist, or another error
/// occurs during retrieval. It logs a warning in this case.
#[instrument(ret, err)]
pub(crate) fn get_machine_id() -> std::io::Result<String> {
    if let Ok(machine_id) = std::fs::read_to_string(MACHINE_ID_PATH) {
        Ok(machine_id.trim().to_string())
    } else {
        warn!("unable to read machine id, setting a random one");
        Ok(uuid::Uuid::new_v4().to_string())
    }
}
