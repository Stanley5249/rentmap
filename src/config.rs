use google_maps::prelude::*;
use serde::Deserialize;
use std::env;
use std::fs;
use std::path::PathBuf;

// Re-import types needed for this module
use crate::cli::Args;
use crate::error::Error;
use crate::geocoding::GeocodingRequest;

#[derive(Deserialize)]
struct Config {
    api_key: Option<String>,
    language: Option<Language>,
    region: Option<Region>,
}

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

pub fn resolve_geocoding_request(
    args: Args,
    path: Option<PathBuf>,
) -> Result<GeocodingRequest, Error> {
    let config = path
        .map(|p| -> Result<Config, Error> {
            let text = fs::read_to_string(&p)?;
            let config = toml::from_str(&text)?;
            Ok(config)
        })
        .transpose()?;

    let api_key = args
        .api_key
        .as_ref()
        .cloned()
        .or_else(|| config.as_ref().and_then(|c| c.api_key.clone()))
        .ok_or(Error::ApiKeyNotFound)?;

    let language = args
        .language
        .or_else(|| config.as_ref().and_then(|c| c.language));

    let region = args
        .region
        .or_else(|| config.as_ref().and_then(|c| c.region));

    Ok(GeocodingRequest {
        query: args.query,
        api_key,
        language,
        region,
    })
}
