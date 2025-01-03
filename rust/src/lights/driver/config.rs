use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use log::{trace, debug, info, warn, error};
use anyhow::{Context, Result};

const DRIVER_CONFIG_FILE: &'static str = if cfg!(debug_assertions) {
    "tests/driver_config.toml"
} else {
    "settings/driver_config.toml"
};

/// Configuration for the lights
#[derive(Copy, Clone, Deserialize, Serialize)]
pub struct DriverConfig {
    /// number of lights in the left strip (GPIO 12 - Pin 32)
    pub left: usize,
    /// number of lights in the right strip (GPIO 13 - Pin 33)
    pub right: usize,
    /// maybe max overall brightness?
    pub brightness: u8,
}

impl Default for DriverConfig {
    fn default() -> Self {
        DriverConfig {
            left: 100,
            right: 300,
            brightness: 255,
        }
    }
}

impl DriverConfig {
    /// Attempt to read the config from a file, otherwise return default
    pub fn init() -> Self {
        let input_str = match std::fs::read_to_string(DRIVER_CONFIG_FILE) {
            Ok(input_str) => input_str,
            Err(e) => {
                warn!("Error reading config file {e:?}");
                return DriverConfig::default();
            }
        };
        match toml::from_str(&input_str) {
            Ok(config) => config,
            Err(e) => {
                warn! ("Error reading/parsing config file {e:?}");
                DriverConfig::default()
            }
        }
    }

    /// Write to a file
    pub async fn save(&self) -> Result<()> {
        let output_str = toml::to_string(self).context("Could convert config to toml")?;
        tokio::fs::write(DRIVER_CONFIG_FILE, &output_str).await.context("Error writing file")
    }
}
