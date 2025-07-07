use std::collections::BTreeSet;

use clap::Parser;
use miette::Result;
use tracing::debug;
use url::Url;

use super::error::Error;
use crate::cli::fetcher::{FetcherArgs, setup_fetcher};
use crate::error::TraceReport;
use crate::file::{TimedRecord, TimedRecords, UrlExt, Workspace};
use crate::sites::rent591::model::{RentItem, RentList};
use crate::sites::rent591::scrapers::{scrape_rent_item, scrape_rent_items};
use crate::sites::rent591::url::{ItemUrl, ListUrl, Rent591Url};
use crate::web::fetcher::Fetcher;

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
    pub workspace: Workspace,

    #[clap(flatten)]
    pub fetcher: FetcherArgs,
}

/// Handle Rent591 list URLs
async fn handle_list(
    url: ListUrl,
    refresh: bool,
    limit: Option<u32>,
    workspace: &Workspace,
    fetcher: &Fetcher,
) -> Result<()> {
    let list_records: TimedRecords<RentList> = workspace.load_records("rent591_lists.json")?;

    let list_record: TimedRecord<RentList> = list_records
        .into_iter()
        .filter(|record| record.data.url == *url)
        .next_back()
        .ok_or(Error::NoRentList)?;

    let update_func = async move |mut item_records: TimedRecords<RentItem>| {
        let mut urls: Vec<_> = if refresh {
            list_record.data.item_urls().collect()
        } else {
            let existing_urls: BTreeSet<_> =
                item_records.iter().map(|item| &item.data.url).collect();

            list_record
                .data
                .item_urls()
                .filter(|url| !existing_urls.contains(url))
                .collect()
        };

        if let Some(limit) = limit {
            urls.truncate(limit as usize)
        };

        if urls.is_empty() {
            debug!(%url, "all records already exist");
            Err(item_records)
        } else {
            let mut new_item_records = scrape_rent_items(fetcher, urls).await;

            item_records.append(&mut new_item_records);

            Ok(item_records)
        }
    };

    workspace
        .update_records_async("rent591_items.json", update_func)
        .await?;

    Ok(())
}

/// Handle Rent591 item URLs
async fn handle_item(
    url: ItemUrl,
    refresh: bool,
    workspace: &Workspace,
    fetcher: &Fetcher,
) -> Result<()> {
    let update_func = async move |mut records: TimedRecords<RentItem>| {
        if !refresh && records.iter().any(|item| item.data.url == *url) {
            debug!(%url, "find existing record");
            return Err(records);
        }
        match scrape_rent_item(fetcher, &url).await.trace_report() {
            Ok(record) => {
                records.push(record);
                Ok(records)
            }
            Err(_) => Err(records),
        }
    };

    workspace
        .update_records_async("rent591_items.json", update_func)
        .await?;

    Ok(())
}

pub async fn run(mut args: Args) -> Result<()> {
    args.url.normalize();

    debug!(?args);

    args.workspace.init()?;

    let fetcher = setup_fetcher(&args.fetcher, args.workspace.clone());

    match Rent591Url::try_from(args.url)? {
        Rent591Url::List(list_url) => {
            handle_list(
                list_url,
                args.refresh,
                args.limit,
                &args.workspace,
                &fetcher,
            )
            .await?;
        }
        Rent591Url::Item(item_url) => {
            handle_item(item_url, args.refresh, &args.workspace, &fetcher).await?;
        }
    }

    Ok(())
}
