//! Fetch command implementation

use clap::Parser;
use miette::Result;
use tracing::debug;
use url::Url;

use crate::cli::fetcher::{FetcherArgs, setup_fetcher};
use crate::file::Workspace;

/// Download and clean HTML pages
#[derive(Debug, Parser)]
pub struct Args {
    /// Target URL to fetch and process
    pub url: Url,

    #[clap(flatten)]
    pub workspace: Workspace,

    #[command(flatten)]
    pub fetcher: FetcherArgs,
}

#[tracing::instrument(skip_all)]
pub async fn run(args: Args) -> Result<()> {
    debug!(?args);

    args.workspace.init()?;

    let fetcher = setup_fetcher(&args.fetcher, args.workspace);

    let _ = fetcher.try_fetch(&args.url).await?;

    Ok(())
}
