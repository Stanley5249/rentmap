//! Fetch command implementation

use clap::Parser;
use miette::Result;
use tracing::debug;
use url::Url;

use crate::cli::fetcher::{FetcherArgs, setup_fetcher};
use crate::file::WorkspaceArgs;

/// Download and clean HTML pages
#[derive(Debug, Parser)]
pub struct Args {
    /// Target URL to fetch and process
    pub url: Url,

    #[clap(flatten)]
    pub workspace: WorkspaceArgs,

    #[command(flatten)]
    pub fetcher: FetcherArgs,
}

pub async fn run(args: Args) -> Result<()> {
    debug!(?args);

    let workspace = args.workspace.build().await?;

    let fetcher = setup_fetcher(&args.fetcher, workspace);

    let _ = fetcher.try_fetch(&args.url).await?;

    Ok(())
}
