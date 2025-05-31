use crate::config::{Config, find_config_file};
use crate::{Error, GeocodingRequest, PrettyPrintable, run_geocoding};
use clap::Parser;
use google_maps::prelude::*;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "geocoding")]
#[command(about = "Geocode addresses using Google Maps API")]
pub struct Args {
    pub query: String,
    #[arg(long, env = "GOOGLE_MAPS_API_KEY")]
    pub api_key: Option<String>,
    #[arg(short, long)]
    pub language: Option<Language>,
    #[arg(short, long)]
    pub region: Option<Region>,
}

/// Resolve CLI arguments and config file into a geocoding request
fn resolve_geocoding_request(
    args: Args,
    config_path: Option<PathBuf>,
) -> Result<GeocodingRequest, Error> {
    let config = config_path.map(|p| Config::from_file(&p)).transpose()?;

    let api_key = args
        .api_key
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

/// Run the CLI application
pub fn run_cli() -> miette::Result<()> {
    let args = Args::parse();

    let config_path = find_config_file();
    println!("{}\n", config_path.to_pretty_string());

    let request = resolve_geocoding_request(args, config_path)?;
    println!("{}\n", request.to_pretty_string());

    let response = match run_geocoding(request) {
        Ok(response) => response,
        // GoogleMapsError did not make diagnostic transparent
        Err(Error::GoogleMaps(GoogleMapsError::Geocoding(e))) => return Err(e.into()),
        Err(e) => return Err(e.into()),
    };
    println!("{}", response.to_pretty_string());
    Ok(())
}
