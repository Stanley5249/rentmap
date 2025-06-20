use clap::Parser;
use miette::Result;
use tracing::debug;

use crate::cli::fetcher::{FetcherArgs, setup_fetcher};
use crate::file::Workspace;
use crate::sites::rent591::scrapers::scrape_rent_items;

/// Augment existing rental lists with detailed item data
#[derive(Debug, Parser)]
pub struct Args {
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

    let rent_lists = args.workspace.load_data_json(&"rent591_lists.json")?;

    let augmented_lists = scrape_rent_items(&fetcher, rent_lists).await?;

    args.workspace
        .save_data_json(&"rent591_items.json", &augmented_lists)?;

    Ok(())
}
