use std::{fmt, fs};

use error_stack::{Context, Report, Result};
use serde::{Deserialize, Serialize};

static CONFIG_FILE_NAME: &'static str = "config.toml";

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigFile {
    #[serde(rename = "discord-token")]
    pub discord_token: String,
    pub guilds: Vec<i64>,
    #[serde(rename = "api-url")]
    pub api_url: String,
    #[serde(rename = "api-key")]
    pub api_key: String,
}

#[derive(Debug)]
pub struct ConfigFileError;

impl fmt::Display for ConfigFileError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Error validating inner values")
    }
}

impl Context for ConfigFileError {}

impl ConfigFile {
    pub fn read() -> Result<ConfigFile, ConfigFileError> {
        let config_file = fs::read(format!("./{}", CONFIG_FILE_NAME)).map_err(|e| {
            Report::from(e)
                .attach_printable("Failed to read config file (./config.toml)")
                .change_context(ConfigFileError)
        })?;

        let encoded_config_file = String::from_utf8(config_file).map_err(|e| {
            Report::from(e)
                .attach_printable(
                    "Failed to encode config file to UTF-8 (Ensure their is no unicode)",
                )
                .change_context(ConfigFileError)
        })?;

        let decoded_config: ConfigFile =
            toml::from_str(encoded_config_file.as_str()).map_err(|e| {
                Report::from(e)
                    .attach_printable("Failed to decode config file. Its likely invalid.")
                    .change_context(ConfigFileError)
            })?;

        Ok(decoded_config)
    }
}
