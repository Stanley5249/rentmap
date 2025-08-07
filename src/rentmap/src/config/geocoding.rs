use clap::Args;
use google_maps::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize, Args)]
#[command(next_help_heading = "Geocoding")]
pub struct GeocodingConfig {
    /// Language for geocoding results
    ///
    /// See: https://developers.google.com/maps/faq#languagesupport
    #[arg(short, long)]
    pub language: Option<Language>,

    /// Region bias for geocoding results
    ///
    /// See: https://developers.google.com/maps/documentation/geocoding/requests-geocoding#RegionCodes
    #[arg(short, long)]
    pub region: Option<Region>,
}
