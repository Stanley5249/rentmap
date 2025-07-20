//! Main entry point for RentMap CLI

use clap::{Parser, Subcommand};
use rentmap::cli::commands::{fetch, geocoding, item, list, ocr};
use rentmap::error::TraceReport;
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
    List(list::Args),
    Item(item::Args),
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
        .with_file(false)
        .with_line_number(false)
        .init();
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    setup_tracing();

    let cli = Cli::parse();

    match cli.command {
        Commands::List(args) => list::run(args).await,
        Commands::Item(args) => item::run(args).await,
        Commands::Fetch(args) => fetch::run(args).await,
        Commands::Geocoding(args) => geocoding::run(args).await,
        Commands::Ocr(args) => ocr::run(args).await,
    }
    .trace_report()
    .ok();
}
