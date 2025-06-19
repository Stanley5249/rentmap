use miette::{Report, Result};
use tracing::{debug, info, instrument, warn};

use crate::sites::rent591::model::RentLists;
use crate::sites::rent591::view::ItemView;
use crate::web::fetcher::Fetcher;

#[instrument(skip_all)]
pub async fn scrape_rent_items(
    fetcher: &Fetcher,
    mut rent_lists: RentLists,
) -> Result<RentLists, Report> {
    let mut total_items = 0;
    let mut total_urls = 0;

    for list in rent_lists.lists.iter_mut().flatten() {
        for item in &mut list.items {
            total_urls += 1;
            if item.detail.is_none() {
                match fetcher.try_fetch(&item.link).await {
                    Ok(document) => {
                        let item_view: ItemView = document.into();
                        match item_view.extract_rent_item() {
                            Ok(detail) => {
                                debug!(%item.link, "item detail scraped");
                                item.detail = Some(detail);
                                total_items += 1;
                            }
                            Err(extract_error) => {
                                warn!(%item.link, %extract_error, "failed to extract item data");
                            }
                        }
                    }
                    Err(report) => {
                        warn!(%item.link, %report);
                        eprintln!("{:?}", report);
                    }
                }
            } else {
                total_items += 1;
            }
        }
    }

    let skipped = total_urls - total_items;
    match skipped {
        0 => info!(
            scraped = total_items,
            "all item details scraped successfully"
        ),
        _ => warn!(
            scraped = total_items,
            skipped, "some item details failed to scrape"
        ),
    }

    Ok(rent_lists)
}
