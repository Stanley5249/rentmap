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
    workspace: &Workspace,
    fetcher: &Fetcher,
    url: ListUrl,
    limit: Option<u32>,
) -> Result<()> {
    let list_records: TimedRecords<RentList> = workspace.load_records("rent591_lists.json")?;

    let list_record: TimedRecord<RentList> = list_records
        .into_iter()
        .filter(|record| record.data.url == *url)
        .next_back()
        .ok_or(Error::NoRentList)?;

    let update_func = async |mut records: TimedRecords<RentItem>| {
        let existing_urls: BTreeSet<_> = records.iter().map(|item| &item.data.url).collect();

        let pending_urls = list_record
            .data
            .item_urls()
            .filter(|url| !existing_urls.contains(url));

        let pending_urls: Vec<_> = match limit {
            Some(limit) => pending_urls.take(limit as usize).collect(),
            None => pending_urls.collect(),
        };

        let mut new_records = scrape_rent_items(fetcher, pending_urls).await;

        records.append(&mut new_records);

        Ok(records)
    };

    workspace
        .update_records_async("rent591_items.json", update_func)
        .await?;

    Ok(())
}

/// Handle Rent591 item URLs
async fn handle_item(workspace: &Workspace, fetcher: &Fetcher, url: ItemUrl) -> Result<()> {
    let update_func = async |mut records: TimedRecords<RentItem>| {
        if records.iter().any(|item| item.data.url == *url) {
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
            handle_list(&args.workspace, &fetcher, list_url, args.limit).await?;
        }
        Rent591Url::Item(item_url) => {
            handle_item(&args.workspace, &fetcher, item_url).await?;
        }
    }

    Ok(())
}
