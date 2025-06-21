use std::env;
use std::path::{Path, PathBuf};

use miette::IntoDiagnostic;
use serde::Deserialize;
use tracing::error;

use crate::config::geocoding::GeocodingConfig;
use crate::config::google::GoogleConfig;
use crate::config::ocr::OcrConfig;
use crate::file::load_toml;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(flatten)]
    pub google: Option<GoogleConfig>,

    pub geocoding: Option<GeocodingConfig>,

    pub ocr: Option<OcrConfig>,
}

pub fn find_config<P>(file_name: P) -> Option<PathBuf>
where
    P: AsRef<Path>,
{
    [env::current_dir().ok(), dirs::home_dir()]
        .into_iter()
        .flatten()
        .map(|dir| dir.join(&file_name))
        .find(|path| path.exists())
}

pub fn load_config() -> Option<Config> {
    find_config("rentmap.toml").and_then(|path| {
        load_toml(&path)
            .into_diagnostic()
            .inspect_err(|report| {
                error!(%report);
                eprintln!("{:?}", report);
            })
            .ok()
    })
}
