//! Fetch command implementation

use crate::cli::fetcher::{FetcherArgs, setup_fetcher};
use crate::file::make_directory;
use clap::Parser;
use miette::Result;
use tracing::info;
use url::Url;

#[derive(Debug, Parser)]
pub struct Args {
    /// URL to fetch and process
    pub url: Url,

    #[command(flatten)]
    pub fetcher: FetcherArgs,
}

pub async fn run(args: Args) -> Result<()> {
    info!("starting fetch command");

    make_directory(&args.fetcher.out_dir)?;

    let fetcher = setup_fetcher(&args.fetcher);

    let _ = fetcher.try_fetch(&args.url).await?;

    info!("fetch completed successfully");

    Ok(())
}
