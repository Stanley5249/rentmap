//! Fetch command implementation

use crate::cli::fetcher::{FetcherArgs, setup_fetcher};
use crate::file::make_directory;
use clap::Parser;
use miette::Result;
use tracing::{error, info};
use url::Url;

#[derive(Debug, Parser)]
pub struct Args {
    /// Target URL to fetch and process
    pub url: Url,

    #[command(flatten)]
    pub fetcher: FetcherArgs,
}

#[tracing::instrument(skip_all)]
pub async fn run(args: Args) -> Result<()> {
    info!(?args);

    make_directory(&args.fetcher.out_dir).inspect_err(|error| error!(%error))?;

    let fetcher = setup_fetcher(&args.fetcher);

    let _ = fetcher
        .try_fetch(&args.url)
        .await
        .inspect_err(|error| error!(%error))?;

    Ok(())
}
