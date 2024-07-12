use std::path::PathBuf;
use std::{fs::File, io::BufReader};

use crate::{common::Config, error::Error};

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

        let file = File::open(config_path).or(Err(Error::FileIoError))?;

        serde_json::from_reader(BufReader::new(file)).or(Err(Error::ParsingError))
    }

    pub fn save(&self) -> Result<(), Error> {
        let file = File::options()
            .write(true)
            .truncate(true)
            .open("config.json")
            .or(Err(Error::FileIoError))?;

        serde_json::to_writer(file, self).or(Err(Error::SerializationError))
    }
}
