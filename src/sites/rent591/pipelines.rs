use super::model::{RentItem, RentList};
use super::view::ListView;
use crate::scraping::fetcher::Fetcher;
use crate::sites::rent591::model::RentLists;
use crate::sites::rent591::view::ItemView;
use tracing::{error, info, warn};
use url::Url;

fn query_page(base_url: &Url, page_number: u32) -> Url {
    let mut page_url = base_url.clone();
    page_url
        .query_pairs_mut()
        .append_pair("page", &page_number.to_string());
    page_url
}

async fn scrape_rent_list_page(
    fetcher: &Fetcher,
    base_url: &Url,
    page_number: u32,
) -> Option<RentList> {
    let page_url = query_page(base_url, page_number);

    let response = match fetcher.try_fetch(&page_url).await {
        Ok(response) => response,
        Err(e) => {
            error!(page = page_number, %e);
            return None;
        }
    };

    let list_view: ListView = response.into();
    let items = match list_view.extract_rent_items() {
        Ok(items) => items,
        Err(e) => {
            error!(page = page_number, %e);
            return None;
        }
    };

    info!(page = page_number, items = items.len(), "page scraped");
    Some(RentList { page_number, items })
}

pub async fn scrape_rent_lists(fetcher: &Fetcher, base_url: Url, limit: Option<u32>) -> RentLists {
    info!(%base_url, "scraping target");

    let mut lists = Vec::new();

    let first_page_url = query_page(&base_url, 1);

    let first_list = match fetcher.try_fetch(&first_page_url).await {
        Ok(response) => response,
        Err(e) => {
            error!(%e);
            return RentLists {
                base_url,
                page_count: 0,
                lists,
            };
        }
    };

    let first_list_view: ListView = first_list.into();

    let page_count = match first_list_view.extract_page_count() {
        Ok(count) => count,
        Err(e) => {
            error!(%e);
            return RentLists {
                base_url,
                page_count: 0,
                lists,
            };
        }
    };

    info!(page_count, "scraping pages");

    let first_list_items = match first_list_view.extract_rent_items() {
        Ok(items) => {
            info!(page = 1, items = items.len(), "page scraped");
            Some(RentList {
                page_number: 1,
                items,
            })
        }
        Err(e) => {
            error!(page = 1, %e);
            None
        }
    };

    lists.push(first_list_items);

    // Determine the actual number of pages to scrape
    let max_pages = match limit {
        Some(limit) => std::cmp::min(page_count, limit),
        None => page_count,
    };

    for page_number in 2..=max_pages {
        let page_result = scrape_rent_list_page(fetcher, &base_url, page_number).await;
        lists.push(page_result);
    }

    let successful = lists.iter().filter(|p| p.is_some()).count();
    info!(
        successful,
        failed = lists.len() - successful,
        "scraping completed"
    );

    RentLists {
        base_url,
        page_count,
        lists,
    }
}

pub async fn scrape_rent_items_pages(
    fetcher: &Fetcher,
    item_urls: Vec<Url>,
) -> Vec<Option<RentItem>> {
    if item_urls.is_empty() {
        warn!("no item urls found");
        return Vec::new();
    }

    info!(count = item_urls.len(), "scraping items");

    let mut results = Vec::with_capacity(item_urls.len());

    for url in item_urls {
        let result = match fetcher.try_fetch(&url).await {
            Ok(document) => {
                let item_view: ItemView = document.into();
                match item_view.extract_rent_item() {
                    Ok(item) => {
                        info!(%url, "item scraped");
                        Some(item)
                    }
                    Err(e) => {
                        error!(%url, %e);
                        None
                    }
                }
            }
            Err(error) => {
                error!(%url, %error);
                None
            }
        };
        results.push(result);
    }

    let successful = results.iter().filter(|item| item.is_some()).count();
    info!(
        successful,
        failed = results.len() - successful,
        "scraping completed"
    );

    results
}
