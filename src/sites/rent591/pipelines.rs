use super::model::{RentItem, RentList};
use super::view::ListView;
use crate::scraping::fetcher::Fetcher;
use crate::sites::rent591::model::RentLists;
use crate::sites::rent591::view::ItemView;
use tracing::{debug, error, info, warn};
use url::Url;

fn build_page_url(base_url: &Url, page_number: u32) -> Url {
    let mut page_url = base_url.clone();
    page_url
        .query_pairs_mut()
        .append_pair("page", &page_number.to_string());
    page_url
}

async fn scrape_rent_list(fetcher: &Fetcher, url: Url) -> Option<RentList> {
    let response = match fetcher.try_fetch(&url).await {
        Ok(response) => response,
        Err(error) => {
            warn!(%url, %error);
            return None;
        }
    };

    let list_view: ListView = response.into();
    let rent_list_items = list_view.extract_items();

    debug!(%url, item_count = rent_list_items.len());

    Some(RentList {
        url,
        items: rent_list_items,
    })
}

pub async fn scrape_rent_lists(
    fetcher: &Fetcher,
    base_url: Url,
    limit: Option<u32>,
) -> Result<RentLists, crate::scraping::error::Error> {
    let url = build_page_url(&base_url, 1);

    let first_list = fetcher
        .try_fetch(&url)
        .await
        .inspect_err(|error| error!(%base_url, %error))?;

    let first_list_view: ListView = first_list.into();

    let page_count = match first_list_view.extract_page_count() {
        Some(page_count) => {
            debug!(%base_url, page_count);
            page_count
        }
        None => {
            debug!(%base_url, "no page count, fallback to 1");
            1
        }
    };

    let item_count = match first_list_view.extract_item_count() {
        Some(item_count) => {
            debug!(%base_url, item_count);
            item_count
        }
        None => {
            debug!(%base_url, "no item count, fallback to 0");
            0
        }
    };

    let rent_list_items = first_list_view.extract_items();
    debug!(%url, item_count = rent_list_items.len());

    let max_pages = match limit {
        Some(limit) => std::cmp::min(page_count, limit),
        None => page_count,
    };

    let rent_list = Some(RentList {
        url,
        items: rent_list_items,
    });
    let mut rent_lists = Vec::with_capacity(max_pages as usize);
    rent_lists.push(rent_list);

    for page_number in 2..=max_pages {
        let url = build_page_url(&base_url, page_number);
        let rent_list = scrape_rent_list(fetcher, url).await;
        rent_lists.push(rent_list);
    }

    let err = rent_lists.iter().filter(|p| p.is_none()).count();
    let ok = rent_lists.len() - err;

    match err {
        0 => info!(%base_url, ok, "all lists scraped successfully"),
        _ => warn!(%base_url, ok, err, "some lists failed to scrape"),
    }

    Ok(RentLists {
        base_url,
        page_count,
        item_count,
        lists: rent_lists,
    })
}

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
