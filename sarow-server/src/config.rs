use byte_unit::Byte;
use config::{ConfigError, Environment, File};
use humantime_serde::Serde;
use serde::Deserialize;
use std::net::Ipv4Addr;
use std::time::Duration;
use crate::auth::BasicAuth;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub ip: Ipv4Addr,
    pub port: u16,
    pub log_level: String,
    pub work_dir: String,
    pub max_size: Byte,
    #[serde(with = "humantime_serde")]
    pub max_file_age: Duration,
    #[serde(with = "humantime_serde")]
    pub clean_interval: Duration,
    pub auth: Option<BasicAuth>
}

impl Config {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = config::Config::new();

        s.merge(File::with_name("config/default"))?;

        let config_path = std::env::var("CONFIG_PATH").unwrap_or_else(|_| "config".into());

        s.merge(File::with_name(&config_path).required(false))?;

        s.merge(File::with_name("config/development").required(false))?;

        s.merge(Environment::default())?;

        s.try_into()
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            ip: Ipv4Addr::new(0, 0, 0, 0),
            port: 8080,
            log_level: String::from("info"),
            work_dir: String::from("."),
            max_size: Byte::from(u64::MAX),
            max_file_age: Duration::from_nanos(u64::MAX),
            clean_interval: Duration::from_nanos(u64::MAX),
            auth: None
        }
    }
}
