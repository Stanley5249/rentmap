use clap::Parser;
use miette::Result;
use tracing::debug;
use url::Url;

use crate::cli::fetcher::{FetcherArgs, setup_fetcher};
use crate::error::TraceReport;
use crate::file::{TimedRecords, UrlExt, Workspace};
use crate::sites::rent591::{ListUrl, RentList, scrape_rent_list};
use crate::web::fetcher::Fetcher;

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
    pub workspace: Workspace,

    #[clap(flatten)]
    pub fetcher: FetcherArgs,
}

async fn handle_list(
    url: ListUrl,
    refresh: bool,
    limit: Option<u32>,
    workspace: &Workspace,
    fetcher: &Fetcher,
) -> Result<()> {
    let update_func = async |mut records: TimedRecords<RentList>| {
        if !refresh && records.iter().any(|record| record.data.url == *url) {
            debug!(%url, "find existing record");
            return Err(records);
        }

        match scrape_rent_list(fetcher, &url, limit).await.trace_report() {
            Ok(record) => {
                records.push(record);
                Ok(records)
            }
            Err(_) => Err(records),
        }
    };

    workspace
        .update_records_async("rent591_lists.json", update_func)
        .await?;

    Ok(())
}

pub async fn run(mut args: Args) -> Result<()> {
    args.url.normalize();

    debug!(?args);

    args.workspace.init()?;

    let fetcher = setup_fetcher(&args.fetcher, args.workspace.clone());

    let url = ListUrl::try_from(args.url)?;

    handle_list(url, args.refresh, args.limit, &args.workspace, &fetcher).await?;

    Ok(())
}
