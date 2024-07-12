#[derive(Debug)]
pub(crate) enum Error {
    NoConfigFileFound,
    FileIoError,
    ParsingError,
    SerializationError,
}