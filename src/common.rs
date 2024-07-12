use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    #[error("No config file found")]
    NoConfigFileFound,
    #[error("Error during file IO at {0}: {1}")]
    FileIoError(PathBuf, std::io::Error),
    #[error("Error during parsing of {0}")]
    ParsingError(PathBuf),
}
