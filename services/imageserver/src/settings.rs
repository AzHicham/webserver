use config::{Config, ConfigError, Environment, File, FileFormat};
use serde::Deserialize;
use std::env;
use std::path::PathBuf;

#[derive(Clone, Debug, Deserialize)]
pub struct Server {
    pub host: String,
    pub port: u16,
    pub workers: u16,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ImageServer {
    pub slide_dir: PathBuf,
    pub compatible_file_extension: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Logger {
    pub url: Option<PathBuf>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Settings {
    pub server: Server,
    pub imageserver: ImageServer,
    pub logger: Logger,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("WEBSERVER_CONFIG_MODE").unwrap_or_else(|_| "production".into());
        let config_mode_filename = format!("config/settings.{run_mode}");

        let s = Config::builder()
            // Start off by merging in the "default" configuration file
            .add_source(File::new("config/settings", FileFormat::Toml))
            // Add in the current environment file
            // Default to 'production' env
            // Note that this file is _optional_
            .add_source(File::new(config_mode_filename.as_str(), FileFormat::Toml).required(false))
            // Add in a local configuration file
            // This file shouldn't be checked in to git
            .add_source(File::new("config/settings.local", FileFormat::Toml).required(false))
            // Add in settings from the environment (with a prefix of APP)
            // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
            .add_source(Environment::with_prefix("WEBSERVER"))
            .build()?;

        s.try_deserialize()
    }
}
