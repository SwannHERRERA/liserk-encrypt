use config::ConfigError;
use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("...")]
pub enum Error {
    Io(#[from] io::Error),
    Config(#[from] ConfigError),
    Serialization(#[from] toml::ser::Error),
}
