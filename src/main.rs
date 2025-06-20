//! Main entry point for RentMap CLI

use clap::{Parser, Subcommand};
use miette::Result;
use rentmap::cli::commands::{fetch, geocoding, items, lists, ocr};
use tracing::error;
use tracing_subscriber::{self, EnvFilter};

/// Rental data scraping and processing toolkit
#[derive(Debug, Parser)]
#[command(name = "rentmap", version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Lists(lists::Args),
    Items(items::Args),
    Fetch(fetch::Args),
    Geocoding(geocoding::Args),
    Ocr(ocr::Args),
}

/// Initialize tracing for logging
pub fn setup_tracing() {
    let filter = EnvFilter::from_default_env();

    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter(filter)
        .with_writer(std::io::stderr)
        .init();
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    setup_tracing();

    let cli = Cli::parse();
    match cli.command {
        Commands::Lists(args) => lists::run(args).await,
        Commands::Items(args) => items::run(args).await,
        Commands::Fetch(args) => fetch::run(args).await,
        Commands::Geocoding(args) => geocoding::run(args).await,
        Commands::Ocr(args) => ocr::run(args).await,
    }
    .inspect_err(|report| error!(%report))
}
