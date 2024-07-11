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
        let override_config_path = PathBuf::from("config.json");

        let final_path = if override_config_path.is_file() {
            override_config_path
        } else if let Some(path) = default_config_path {
            if !path.is_file() {
                return Err(Error::NoConfigFileFound);
            }
            path
        } else {
            return Err(Error::NoConfigFileFound);
        };

        let Ok(file) = File::open(final_path) else {
            return Err(Error::FileIoError);
        };

        serde_json::from_reader(BufReader::new(file)).or(Err(Error::ParsingError))
    }
}
