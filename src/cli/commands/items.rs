use std::path::PathBuf;

use clap::Parser;
use miette::Result;
use tracing::debug;

use crate::cli::fetcher::{FetcherArgs, setup_fetcher};
use crate::file::{load_json, make_directory, save_json};
use crate::sites::rent591::scrapers::scrape_rent_items;

#[derive(Debug, Parser)]
pub struct Args {
    /// Input JSON file containing RentLists data
    pub input_file: PathBuf,

    /// Output JSON filename
    #[arg(long = "out-file", short = 'f')]
    pub out_file: Option<String>,

    /// Input directory for JSON file
    #[arg(long = "in-dir", short = 'i', default_value = "output")]
    pub in_dir: PathBuf,

    #[clap(flatten)]
    pub fetcher: FetcherArgs,
}

#[tracing::instrument(skip_all)]
pub async fn run(args: Args) -> Result<()> {
    debug!(?args);

    make_directory(&args.fetcher.out_dir)?;

    let fetcher = setup_fetcher(&args.fetcher);

    let rent_lists = load_json(&args.input_file, &args.in_dir)?;

    let augmented_lists = scrape_rent_items(&fetcher, rent_lists).await?;

    let out_file = args.out_file.unwrap_or_else(|| {
        args.input_file
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("rent591_items.json")
            .to_string()
    });

    save_json(&augmented_lists, &out_file, &args.fetcher.out_dir)?;

    Ok(())
}
