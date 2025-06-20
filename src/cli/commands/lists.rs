use clap::Parser;
use miette::Result;
use tracing::debug;
use url::Url;

use crate::cli::fetcher::{FetcherArgs, setup_fetcher};
use crate::file::Workspace;
use crate::sites::rent591::scrapers::scrape_rent_lists;

/// Scrape rental listings from rent.591.com.tw and save as JSON
#[derive(Debug, Parser)]
pub struct Args {
    /// Target URL for rent.591.com.tw search results
    pub url: Url,

    /// Maximum pages to scrape
    #[arg(long, short)]
    pub limit: Option<u32>,

    #[clap(flatten)]
    pub workspace: Workspace,

    #[clap(flatten)]
    pub fetcher: FetcherArgs,
}

#[tracing::instrument(skip_all)]
pub async fn run(args: Args) -> Result<()> {
    debug!(?args);

    args.workspace.ensure()?;

    let fetcher = setup_fetcher(&args.fetcher, &args.workspace);

    let rent_lists = scrape_rent_lists(&fetcher, args.url, args.limit).await?;

    args.workspace
        .save_data_json(&"rent591_lists.json", &rent_lists)?;

    Ok(())
}
