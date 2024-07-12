use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub id: Option<String>,
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "kind")]
pub enum Command {
    SetConfig(Config),
    LoadUrl { url: String },
    Reload,
    Stop,
}
