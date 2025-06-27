use miette::{Report, Result};
use tracing::{debug, instrument, warn};

use crate::sites::rent591::model::{RentItemDetail, RentLists};
use crate::sites::rent591::view::ItemView;
use crate::web::fetcher::Fetcher;

#[instrument(skip_all)]
pub async fn scrape_rent_items(
    fetcher: &Fetcher,
    rent_lists: &RentLists,
) -> Result<Vec<RentItemDetail>, Report> {
    let urls = rent_lists
        .lists
        .iter()
        .flatten()
        .flat_map(|a| a.items.iter().map(|item| &item.link));

    let mut items = Vec::with_capacity(urls.size_hint().0);

    for url in urls {
        match fetcher.try_fetch(url).await {
            Ok(document) => {
                let item_view: ItemView = document.into();

                match item_view.extract_rent_item() {
                    Ok(detail) => {
                        debug!(%url, "item detail scraped");
                        items.push(detail);
                    }
                    Err(extract_error) => {
                        warn!(%url, %extract_error, "failed to extract item data");
                    }
                }
            }
            Err(report) => {
                warn!(%url, %report);
                eprintln!("{:?}", report);
            }
        }
    }

    Ok(items)
}
