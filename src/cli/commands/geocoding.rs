//! Geocoding command implementation

use crate::config::geocoding::GeocodingConfig;
use crate::config::google::GoogleConfig;
use crate::config::model::{Config, load_config};
use crate::pretty::ToPrettyString;
use clap::Parser;
use colored::Colorize;
use google_maps::prelude::*;
use miette::Result;
use tracing::{error, info};

#[derive(Debug, Parser)]
pub struct Args {
    /// Address or location to geocode
    pub query: String,

    #[clap(flatten)]
    pub config: GeocodingConfig,

    #[clap(flatten)]
    pub google: GoogleConfig,
}

fn merge_args(mut args: Args, config: Config) -> Args {
    if let Some(google_config) = config.google {
        args.google.api_key = args.google.api_key.or(google_config.api_key);
    }

    if let Some(geocoding) = config.geocoding {
        args.config.language = args.config.language.or(geocoding.language);
        args.config.region = args.config.region.or(geocoding.region);
    }

    args
}

fn format_args(args: &Args) -> String {
    let title = "Args:".bold().underline();
    let table = args.to_pretty_string();
    format!("{}\n{}", title, table)
}

fn format_geocoding_response(response: &GeocodingResponse) -> String {
    let title = "Response:".bold().underline();

    let table = response.to_pretty_string();

    let summary = match response.results.len() {
        0 => "No locations found.".red(), // unreachable
        1 => "Found 1 location.".bright_green(),
        n => format!("Found {} locations.", n).bright_green(),
    };

    format!("{}\n{}\n{}", title, table, summary)
}

/// Run the CLI application
#[tracing::instrument(skip_all)]
pub async fn run(args: Args) -> Result<()> {
    let args = match load_config() {
        Some(config) => merge_args(args, config),
        None => args,
    };
    info!(?args);

    println!("{}", format_args(&args));

    let api_key = args
        .google
        .get_api_key()
        .inspect_err(|error| error!(%error))?;

    let client = Client::try_new(api_key).inspect_err(|error| error!(%error))?;

    let mut builder = client.geocoding().with_address(&args.query);

    if let Some(language) = &args.config.language {
        builder = builder.with_language(language);
    }

    if let Some(region) = &args.config.region {
        builder = builder.with_region(region);
    }

    let response = builder
        .execute()
        .await
        .inspect_err(|error| error!(%error))?;

    println!("\n{}", format_geocoding_response(&response));

    Ok(())
}
