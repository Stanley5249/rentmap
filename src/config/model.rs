use crate::file::load_toml;
use clap::Parser;
use google_maps::prelude::*;
use serde::Deserialize;
use std::env;
use std::path::{Path, PathBuf};
use tracing::error;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub api_key: Option<String>,

    pub geocoding: Option<GeocodingConfig>,
}

#[derive(Debug, Deserialize, Parser)]
pub struct GeocodingConfig {
    #[arg(short, long)]
    pub language: Option<Language>,

    #[arg(short, long)]
    pub region: Option<Region>,
}

pub fn find_config<P>(file_name: &P) -> Option<PathBuf>
where
    P: AsRef<Path>,
{
    [env::current_dir().ok(), dirs::home_dir()]
        .into_iter()
        .flatten()
        .map(|dir| dir.join(file_name))
        .find(|path| path.exists())
}

pub fn load_config() -> Option<Config> {
    find_config(&"rentmap.toml")
        .and_then(|path| load_toml(&path).inspect_err(|error| error!(%error)).ok())
}
