use miette::Result;
use tracing::{info, instrument, warn};
use url::Url;

use crate::error::TraceReport;
use crate::file::{TimedRecord, TimedRecords};
use crate::sites::rent591::model::RentItem;
use crate::sites::rent591::view::ItemView;
use crate::web::fetcher::Fetcher;

#[instrument(skip_all, fields(%url))]
pub async fn scrape_rent_item(fetcher: &Fetcher, url: &Url) -> Result<TimedRecord<RentItem>> {
    let document = fetcher.try_fetch(url).await?;
    let item_view: ItemView = document.into();
    let rent_item = item_view.extract_rent_item(url.clone())?;

    info!("success");

    Ok(rent_item.into())
}

pub async fn scrape_rent_items<'a, I>(fetcher: &Fetcher, urls: I) -> TimedRecords<RentItem>
where
    I: IntoIterator<Item = &'a Url>,
{
    let urls = urls.into_iter();
    let mut results = TimedRecords::with_capacity(urls.size_hint().0);

    let mut err = 0;

    for url in urls {
        match scrape_rent_item(fetcher, url).await.trace_report() {
            Ok(item) => {
                results.push(item);
            }
            Err(_) => {
                err += 1;
            }
        }
    }

    let ok = results.len();

    match err {
        0 => info!(ok, "all rent items scraped successfully"),
        err => warn!(ok, err, "some rent items failed to scrape"),
    }

    results
}
