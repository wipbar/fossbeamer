use std::path::PathBuf;
use std::{fs::File, io::BufReader};

use serde::Deserialize;

use crate::common::Error;

#[derive(Deserialize)]
pub(crate) struct Config {
    pub id: Option<String>,
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn load(default_config_path: Option<PathBuf>) -> Result<Self, Error> {
        let config_path = {
            let override_config_path = PathBuf::from("config.json");

            if override_config_path.is_file() {
                Some(override_config_path)
            } else {
                None
            }
        }
        .or(default_config_path)
        .ok_or(Error::NoConfigFileFound)?;

        let file =
            File::open(&config_path).map_err(|e| Error::FileIoError(config_path.clone(), e))?;

        serde_json::from_reader(BufReader::new(file)).or(Err(Error::ParsingError(config_path)))
    }
}
