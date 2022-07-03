use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::env;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize)]
pub struct Server {
    pub host: String,
    pub port: u16,
    pub workers: u16,
}

#[derive(Debug, Deserialize)]
pub struct Logger {
    pub file: Option<PathBuf>,
}

#[derive(Debug, Deserialize)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub server: Server,
    pub database: Database,
    pub logger: Logger,
    pub broker: Broker,
}

#[derive(Debug, Deserialize)]
pub struct Broker {
    pub address: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("WEBSERVER_CONFIG_MODE").unwrap_or_else(|_| "development".into());
        let config_dir = env::var("WEBSERVER_CONFIG_DIR").unwrap_or_else(|_| "config".into());

        let s = Config::builder()
            // Start off by merging in the "default" configuration file
            .add_source(File::with_name(&format!("{}/default", config_dir)))
            // Add in the current environment file
            // Default to 'development' env
            // Note that this file is _optional_
            .add_source(File::with_name(&format!("{}/{}", config_dir, run_mode)).required(false))
            // Add in a local configuration file
            // This file shouldn't be checked in to git
            .add_source(File::with_name(&format!("{}/local", config_dir)).required(false))
            // Add in settings from the environment (with a prefix of APP)
            // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
            .add_source(Environment::with_prefix("WEBSERVER"))
            .build()?;

        s.try_deserialize()
    }
}
