//! Geocoding command implementation

use crate::cli::error::ApiKeyMissingError;
use crate::config::model::{Config, GeocodingConfig, load_config};
use crate::pretty::ToPrettyString;
use clap::Parser;
use google_maps::prelude::*;
use miette::Result;

#[derive(Debug, Parser)]
pub struct Args {
    /// Address or location to geocode
    pub query: String,

    /// Google Maps API key
    #[arg(long, env = "GOOGLE_MAPS_API_KEY")]
    pub api_key: Option<String>,

    #[clap(flatten)]
    pub config: GeocodingConfig,
}

fn merge_args(mut args: Args, config: Config) -> Args {
    args.api_key = args.api_key.or(config.api_key);

    if let Some(geocoding) = config.geocoding {
        args.config.language = args.config.language.or(geocoding.language);
        args.config.region = args.config.region.or(geocoding.region);
    }

    args
}

/// Run the CLI application
pub async fn run(mut args: Args) -> Result<()> {
    if let Some(config) = load_config() {
        args = merge_args(args, config);
    }

    let api_key = args.api_key.ok_or(ApiKeyMissingError)?;

    let client = Client::try_new(api_key)?;

    let mut builder = client.geocoding().with_address(&args.query);

    if let Some(language) = &args.config.language {
        builder = builder.with_language(language);
    }

    if let Some(region) = &args.config.region {
        builder = builder.with_region(region);
    }

    let response = builder.execute().await?;
    println!("{}", response.to_pretty_string());

    Ok(())
}
