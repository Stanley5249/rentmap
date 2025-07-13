use miette::Result;
use tracing::{info, instrument, warn};
use url::Url;

use crate::error::TraceReport;
use crate::sites::rent591::{ItemView, RentItem};
use crate::web::Fetcher;

#[instrument(skip_all, fields(%url))]
pub async fn scrape_item(fetcher: &Fetcher, url: &Url) -> Result<RentItem> {
    let document = fetcher.try_fetch(url).await?;
    let item_view = ItemView::new(document);
    let rent_item = item_view.extract_item(url.clone())?;

    info!("scrape item");

    Ok(rent_item)
}

pub async fn scrape_items<I, T>(fetcher: &Fetcher, urls: I) -> Vec<RentItem>
where
    I: IntoIterator<Item = T>,
    T: AsRef<Url>,
{
    let iter = urls.into_iter();
    let mut results = Vec::with_capacity(iter.size_hint().0);

    let mut err = 0;

    for url in iter {
        match scrape_item(fetcher, url.as_ref()).await {
            Ok(item) => {
                results.push(item);
            }
            Err(report) => {
                err += 1;
                report.trace_report();
            }
        }
    }

    let ok = results.len();

    match err {
        0 => info!(ok, "scrape all items"),
        err => warn!(ok, err, "scrape items with errors"),
    }

    results
}
