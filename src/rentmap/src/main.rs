//! Main entry point for RentMap CLI

use clap::{Parser, Subcommand};
use miette::Result;
use rentmap::cli::commands::{fetch, geocoding, item, list, ocr, server};
use rentmap::error::TraceReport;
use tracing_subscriber::{self, EnvFilter};

/// Rental data scraping and processing toolkit
#[derive(Debug, Parser)]
#[command(name = "rentmap", version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    List(list::Args),
    Item(item::Args),
    Fetch(fetch::Args),
    Geocoding(geocoding::Args),
    Ocr(ocr::Args),
    Server(server::Args),
}

impl Default for Commands {
    fn default() -> Self {
        Commands::Server(server::Args::default())
    }
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

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    setup_tracing();

    let cli = Cli::parse();

    match cli.command.unwrap_or_default() {
        Commands::List(args) => list::run(args).await,
        Commands::Item(args) => item::run(args).await,
        Commands::Fetch(args) => fetch::run(args).await,
        Commands::Geocoding(args) => geocoding::run(args).await,
        Commands::Ocr(args) => ocr::run(args).await,
        Commands::Server(args) => server::run(args).await,
    }
    .trace()
}
