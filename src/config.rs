use google_maps::prelude::*;
use serde::Deserialize;
use std::env;
use std::fs;
use std::path::PathBuf;

use crate::error::Error;

/// Configuration structure that can be loaded from a TOML file
#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub api_key: Option<String>,
    pub language: Option<Language>,
    pub region: Option<Region>,
}

impl Config {
    /// Load configuration from a file path
    pub fn from_file(path: &PathBuf) -> Result<Self, Error> {
        let text = fs::read_to_string(path)?;
        let config = toml::from_str(&text)?;
        Ok(config)
    }
}

/// Find the geocoding configuration file
/// Searches in current directory first, then home directory
pub fn find_config_file() -> Option<PathBuf> {
    let config_path = env::current_dir().ok()?.join("geocoding.toml");
    if config_path.exists() {
        return Some(config_path);
    }

    let config_path = dirs::home_dir()?.join("geocoding.toml");
    if config_path.exists() {
        return Some(config_path);
    }

    None
}
