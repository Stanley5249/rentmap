use clap::Parser;
use miette::{Diagnostic, Result};
use thiserror::Error;
use tracing::debug;
use url::Url;

use crate::cli::fetcher::{FetcherArgs, setup_fetcher};
use crate::file::Workspace;
use crate::sites::rent591::scrapers::scrape_rent_items;

#[derive(Debug, Error, Diagnostic)]
pub enum Error {
    #[error("no rental lists found in workspace, please run `rentmap lists` first")]
    #[diagnostic(
        code(rentmap::items::no_rental_lists),
        help("run `rentmap lists` to fetch rental lists")
    )]
    NoRentalListsFound,
}
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

    let records = args.workspace.load_timed_records("rent591_lists.json")?;

    let record = records.last().ok_or(Error::NoRentalListsFound)?;

    let item_details = scrape_rent_items(&fetcher, &record.data).await?;

    args.workspace
        .add_timed_record(item_details, "rent591_items.json")?;

    Ok(())
}
