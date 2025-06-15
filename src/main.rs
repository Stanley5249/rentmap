//! Main entry point for RentMap CLI

use clap::{Parser, Subcommand};
use miette::Result;
use rentmap::cli::commands::{fetch, geocoding, lists, ocr};
use std::str::FromStr;
use tracing_subscriber::filter::Directive;
use tracing_subscriber::{self, EnvFilter};

#[derive(Debug, Parser)]
#[command(
    name = "rentmap",
    version,
    about = "RentMap CLI Tool",
    long_about = None,
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Scrape listings from rent.591.com.tw and save as JSON.
    Lists(lists::Args),
    /// Download and clean HTML pages
    Fetch(fetch::Args),
    /// Geocoding operations
    Geocoding(geocoding::Args),
    /// Perform OCR (text detection) on an image using Google Vision API
    Ocr(ocr::Args),
}

/// Initialize tracing for logging
pub fn setup_tracing() {
    let directive = Directive::from_str("rentmap=debug").unwrap();

    let filter = EnvFilter::from_default_env().add_directive(directive);

    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter(filter)
        .init();
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    setup_tracing();

    let cli = Cli::parse();

    match cli.command {
        Commands::Lists(args) => lists::run(args).await,
        Commands::Fetch(args) => fetch::run(args).await,
        Commands::Geocoding(args) => geocoding::run(args).await,
        Commands::Ocr(args) => ocr::run(args).await,
    }
}
