use clap::Parser;
use miette::Result;
use tracing::debug;
use url::Url;

use crate::cli::fetcher::{FetcherArgs, setup_fetcher};
use crate::file::Workspace;
use crate::sites::rent591::scrapers::scrape_rent_items;

/// Augment existing rental lists with detailed item data
#[derive(Debug, Parser)]
pub struct Args {
    /// Target URL for rent.591.com.tw search results
    pub url: Url,

    #[clap(flatten)]
    pub workspace: Workspace,

    #[clap(flatten)]
    pub fetcher: FetcherArgs,
}

#[tracing::instrument(skip_all)]
pub async fn run(args: Args) -> Result<()> {
    debug!(?args);

    args.workspace.init()?;

    let fetcher = setup_fetcher(&args.fetcher, args.workspace.clone());

    let (ts, rent_lists) = args.workspace.load_data_latest(&args.url)?;

    let rent_lists = scrape_rent_items(&fetcher, rent_lists).await?;

    args.workspace.save_data_at(&rent_lists, &args.url, &ts)?;

    Ok(())
}
