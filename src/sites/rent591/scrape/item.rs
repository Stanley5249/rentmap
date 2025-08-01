use futures::stream::StreamExt;
use miette::Result;
use tracing::{error, info};
use url::Url;

use crate::error::TraceReport;
use crate::sites::rent591::{ItemView, RentItem};
use crate::web::Fetcher;

pub async fn scrape_item(fetcher: &Fetcher, url: Url) -> Result<RentItem> {
    let document = fetcher.try_fetch(&url).await?;
    let item_view = ItemView::new(document);
    let rent_item = item_view.extract_item(url)?;
    Ok(rent_item)
}

pub async fn scrape_items<I>(fetcher: &Fetcher, urls: I) -> Result<Vec<RentItem>>
where
    I: IntoIterator<Item = Url>,
{
    let future = async |url| scrape_item(fetcher, url).await.trace_report();

    let results: Vec<_> = futures::stream::iter(urls)
        .map(future)
        .buffer_unordered(10)
        .collect::<Vec<_>>()
        .await;

    let total = results.len();

    let items: Vec<_> = results
        .into_iter()
        .filter_map(|result| result.ok())
        .collect();

    let ok = items.len();

    match total - ok {
        0 => info!(ok, "scrape all items"),
        err => error!(ok, err, "scrape items with errors"),
    }

    Ok(items)
}
