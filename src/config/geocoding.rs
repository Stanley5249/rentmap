use clap::Parser;
use google_maps::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize, Parser)]
pub struct GeocodingConfig {
    #[arg(short, long)]
    pub language: Option<Language>,

    #[arg(short, long)]
    pub region: Option<Region>,
}
