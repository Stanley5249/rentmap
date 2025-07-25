use clap::Parser;
use miette::Result;
use tracing::{debug, info};
use url::Url;

use crate::cli::fetcher::FetcherArgs;
use crate::sites::rent591::scrape_list_and_pages;
use crate::url::UrlExt;
use crate::web::Fetcher;
use crate::workspace::{Workspace, WorkspaceArgs};

/// Scrape rental listings from rent.591.com.tw and save as JSON
#[derive(Debug, Parser)]
pub struct Args {
    /// Target URL for rent.591.com.tw search results
    pub url: Url,

    /// Whether to refresh the record even if it already exists
    #[arg(long, short)]
    pub refresh: bool,

    /// Maximum pages to scrape
    #[arg(long, short)]
    pub limit: Option<u32>,

    #[clap(flatten)]
    pub workspace: WorkspaceArgs,

    #[clap(flatten)]
    pub fetcher: FetcherArgs,
}

async fn handle_list(
    url: Url,
    refresh: bool,
    limit: Option<u32>,
    workspace: &Workspace,
    fetcher: &Fetcher,
) -> Result<()> {
    if !refresh && workspace.list_exists(&url).await? {
        info!("skip existing list");
        return Ok(());
    }

    let list = scrape_list_and_pages(fetcher, &url, limit).await?;
    workspace.insert_list(&list).await?;

    Ok(())
}

pub async fn run(mut args: Args) -> Result<()> {
    args.url.normalize();

    debug!(?args);

    let workspace = args.workspace.build().await?;

    let fetcher = args.fetcher.build(workspace.clone());

    handle_list(args.url, args.refresh, args.limit, &workspace, &fetcher).await?;

    Ok(())
}
