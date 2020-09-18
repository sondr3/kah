use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum KahError {
    #[error("The file {0} already exists")]
    FileExists(String),
}
