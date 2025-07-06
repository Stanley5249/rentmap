use clap::Parser;
use miette::Result;
use tracing::debug;
use url::Url;

use crate::cli::fetcher::{FetcherArgs, setup_fetcher};
use crate::file::{UrlExt, Workspace};
use crate::sites::rent591::scrapers::scrape_rent_list;
use crate::sites::rent591::url::ListUrl;

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

pub async fn run(mut args: Args) -> Result<()> {
    args.url.normalize();

    debug!(?args);

    args.workspace.init()?;

    let fetcher = setup_fetcher(&args.fetcher, args.workspace.clone());

    let url = ListUrl::try_from(args.url)?;

    let list_record = scrape_rent_list(&fetcher, &url, args.limit).await?;

    args.workspace
        .add_record(list_record, "rent591_lists.json")?;

    Ok(())
}
