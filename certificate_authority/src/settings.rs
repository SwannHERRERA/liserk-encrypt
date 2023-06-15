use std::env;

use config::{Config, ConfigError};
use lazy_static::lazy_static;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Logging {
    pub level: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Cipher {
    pub key: Vec<u8>,
    pub certificates_path: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    pub logging: Logging,
    pub cipher: Cipher,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
        println!("{:?}", std::env::current_dir().unwrap());
        let settings = Config::builder()
            .add_source(
                config::File::with_name(&format!("config/{}", run_mode)).required(false),
            )
            .build()?;
        settings.try_deserialize()
    }
}

lazy_static! {
    pub static ref SETTINGS: Settings = Settings::new().expect("config error");
}
