use crate::config::geocoding::GeocodingConfig;
use crate::config::google::GoogleConfig;
use crate::config::ocr::OcrConfig;
use crate::file::load_toml;
use serde::Deserialize;
use std::env;
use std::path::{Path, PathBuf};
use tracing::warn;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(flatten)]
    pub google: Option<GoogleConfig>,

    pub geocoding: Option<GeocodingConfig>,

    pub ocr: Option<OcrConfig>,
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
        .and_then(|path| load_toml(&path).inspect_err(|error| warn!(%error)).ok())
}
