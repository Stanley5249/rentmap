use clap::Parser;
use miette::Result;
use tracing::{debug, info, warn};
use url::Url;

use super::error::Error;
use crate::sites::rent591::{Rent591Url, scrape_item, scrape_items};
use crate::url::UrlExt;
use crate::web::{Fetcher, FetcherArgs};
use crate::workspace::{Workspace, WorkspaceArgs};

/// Augment existing rental list with detailed item data
#[derive(Debug, Parser)]
pub struct Args {
    /// Target URL for rent.591.com.tw search results or rental items
    pub url: Url,

    /// Whether to refresh the record even if it already exists
    #[arg(long, short)]
    pub refresh: bool,

    /// Maximum items to scrape
    #[arg(long, short)]
    pub limit: Option<u32>,

    #[clap(flatten)]
    pub workspace: WorkspaceArgs,

    #[clap(flatten)]
    pub fetcher: FetcherArgs,
}

/// Handle Rent591 list URLs
async fn handle_list(
    url: Url,
    refresh: bool,
    limit: Option<u32>,
    workspace: &Workspace,
    fetcher: &Fetcher,
) -> Result<()> {
    miette::ensure!(workspace.list_exists(&url).await?, Error::NoRentList);

    let urls = workspace
        .select_item_urls_with(&url, refresh, limit)
        .await?;

    match urls.len() {
        0 => warn!("no items found"),
        n => {
            info!(count = n, "find items");
            let items = scrape_items(fetcher, urls.into_iter().map(|url| url.0)).await?;
            workspace.insert_items(&items).await?;
        }
    }

    Ok(())
}

/// Handle Rent591 item URLs
async fn handle_item(
    url: Url,
    refresh: bool,
    workspace: &Workspace,
    fetcher: &Fetcher,
) -> Result<()> {
    if !refresh && workspace.item_exists(&url).await? {
        info!("skip existing item");
        return Ok(());
    }

    let item = scrape_item(fetcher, url).await?;

    workspace.insert_items(&[item]).await?;

    Ok(())
}

pub async fn run(mut args: Args) -> Result<()> {
    args.url.normalize();

    debug!(?args);

    let workspace = args.workspace.build().await?;

    let fetcher = args.fetcher.build(workspace.clone()).await?;

    match Rent591Url::try_from(args.url)? {
        Rent591Url::List(url) => {
            handle_list(url, args.refresh, args.limit, &workspace, &fetcher).await?;
        }
        Rent591Url::Item(url) => {
            handle_item(url, args.refresh, &workspace, &fetcher).await?;
        }
    }

    fetcher.shutdown().await;

    Ok(())
}
