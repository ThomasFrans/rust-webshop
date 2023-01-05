use rocket::serde::Deserialize;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::{env, fs};

/// Configuration for the webshop. Option values have a sensible default and thus aren't required
/// to be specified. Other values are required for the webshop to run.
///
/// Configuration reading order:
///     - Configuration file (webshop.toml);
///     - Environment variables;
#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub webserver_address: Option<String>,
    pub webserver_port: Option<u16>,
    /// URL to the PostgreSQL database for the webshop.
    pub database_url: String,
    pub secret_key: String,
}

#[derive(Debug)]
pub enum CreationError {
    /// The configuration file is invalid.
    InvalidConfigurationFile,
    InvalidEnvironmentVariable(String),
    /// The user didn't set a configuration anywhere.
    NoConfiguration,
}

impl Display for CreationError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            CreationError::InvalidConfigurationFile => write!(f, "Configuration file is invalid."),
            CreationError::InvalidEnvironmentVariable(variable) => {
                write!(f, "Invalid environment variable: {variable}.")
            }
            CreationError::NoConfiguration => write!(f, "No configuration specified."),
        }
    }
}

impl Error for CreationError {}

impl Configuration {
    /// Generate the webshop configuration from all sources.
    ///
    /// # Priority
    /// Environment variables > config file.
    pub fn new() -> Result<Self, CreationError> {
        let mut configuration_builder = ConfigurationBuilder::default();
        configuration_builder.add_cwd_config_file()?.add_env()?;
        configuration_builder.build()
    }
}

#[derive(Default, Deserialize)]
struct ConfigurationBuilder {
    pub webserver_address: Option<String>,
    pub webserver_port: Option<u16>,
    pub database_url: Option<String>,
    pub secret_key: Option<String>,
}

impl ConfigurationBuilder {
    pub fn build(self) -> Result<Configuration, CreationError> {
        Ok(Configuration {
            webserver_address: self.webserver_address,
            webserver_port: self.webserver_port,
            database_url: self.database_url.ok_or(CreationError::NoConfiguration)?,
            secret_key: self.secret_key.ok_or(CreationError::NoConfiguration)?,
        })
    }

    pub fn add_env(&mut self) -> Result<&mut Self, CreationError> {
        if let Ok(webserver_address) = env::var("WEBSHOP_ADDRESS") {
            self.webserver_address = Some(webserver_address);
        }
        if let Ok(webserver_port) = env::var("WEBSHOP_PORT") {
            self.webserver_port = Some(webserver_port.parse().map_err(|_| {
                CreationError::InvalidEnvironmentVariable(String::from("WEBSHOP_PORT"))
            })?);
        }
        if let Ok(database_url) = env::var("WEBSHOP_DATABASE_URL") {
            self.database_url = Some(database_url);
        }
        if let Ok(secret_key) = env::var("WEBSHOP_SECRET_KEY") {
            self.secret_key = Some(secret_key);
        }
        Ok(self)
    }

    pub fn add_cwd_config_file(&mut self) -> Result<&mut Self, CreationError> {
        if let Ok(config_file) = fs::read_to_string("webshop.toml") {
            let config = toml::from_str::<ConfigurationBuilder>(&config_file)
                .map_err(|_| CreationError::InvalidConfigurationFile)?;
            if let Some(webserver_address) = config.webserver_address {
                self.webserver_address = Some(webserver_address);
            }
            if let Some(webserver_port) = config.webserver_port {
                self.webserver_port = Some(webserver_port);
            }
            if let Some(database_url) = config.database_url {
                self.database_url = Some(database_url);
            }
            if let Some(secret_key) = config.secret_key {
                self.secret_key = Some(secret_key);
            }
        }
        Ok(self)
    }
}
