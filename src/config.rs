use std::{fs::File, io::BufReader};

use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct Config {
    pub id: Option<String>,
    pub host: String,
    pub port: u16,
}

#[derive(Debug)]
pub(crate) enum Error {
    FileIoError,
    ParsingError,
}

impl Config {
    pub fn load() -> Result<Self, Error> {
        let Ok(file) = File::open("config.json") else {
            return Err(Error::FileIoError);
        };
        let reader = BufReader::new(file);

        serde_json::from_reader(reader).or(Err(Error::ParsingError))
    }
}
