//! Fetch command implementation

use clap::Parser;
use miette::Result;
use tracing::debug;
use url::Url;

use crate::cli::fetcher::{FetcherArgs, setup_fetcher};
use crate::file::make_directory;

#[derive(Debug, Parser)]
pub struct Args {
    /// Target URL to fetch and process
    pub url: Url,

    #[command(flatten)]
    pub fetcher: FetcherArgs,
}

#[tracing::instrument(skip_all)]
pub async fn run(args: Args) -> Result<()> {
    debug!(?args);

    make_directory(&args.fetcher.out_dir)?;

    let fetcher = setup_fetcher(&args.fetcher);

    let _ = fetcher.try_fetch(&args.url).await?;

    Ok(())
}
