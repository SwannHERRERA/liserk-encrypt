use config::ConfigError;
use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("...")]
pub enum Error {
    IoError(#[from] io::Error),
    ConfigError(#[from] ConfigError),
    SerializationError(#[from] toml::ser::Error),
}
