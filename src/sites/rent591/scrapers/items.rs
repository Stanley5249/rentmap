use tracing::{debug, info, warn};
use url::Url;

use crate::sites::rent591::model::RentItem;
use crate::sites::rent591::view::ItemView;
use crate::web::fetcher::Fetcher;

pub async fn scrape_rent_items(fetcher: &Fetcher, item_urls: Vec<Url>) -> Vec<Option<RentItem>> {
    if item_urls.is_empty() {
        warn!("empty item URLs");
        return Vec::new();
    }

    debug!(item_count = item_urls.len());

    let mut rent_items = Vec::with_capacity(item_urls.len());

    for url in item_urls {
        let result = match fetcher.try_fetch(&url).await {
            Ok(document) => {
                let item_view: ItemView = document.into();
                let item = item_view.extract_rent_item();
                debug!(%url, "item scraped");
                Some(item)
            }
            Err(error) => {
                warn!(%url, %error);
                None
            }
        };
        rent_items.push(result);
    }

    let err = rent_items.iter().filter(|p| p.is_none()).count();
    let ok = rent_items.len() - err;

    match err {
        0 => info!(ok, "all items scraped successfully"),
        _ => warn!(ok, err, "some items failed to scrape"),
    }

    rent_items
}
