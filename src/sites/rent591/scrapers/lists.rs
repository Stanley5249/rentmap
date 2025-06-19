use miette::{Report, Result};
use tracing::{debug, error, info, instrument, warn};
use url::Url;

use crate::sites::rent591::model::{RentList, RentLists};
use crate::sites::rent591::view::ListView;
use crate::web::fetcher::Fetcher;

fn build_page_url(base_url: &Url, page_number: u32) -> Url {
    let mut page_url = base_url.clone();
    page_url
        .query_pairs_mut()
        .append_pair("page", &page_number.to_string());
    page_url
}

async fn scrape_rent_list(
    fetcher: &Fetcher,
    base_url: &Url,
    page_number: u32,
) -> Result<(ListView, RentList), Report> {
    let url = build_page_url(base_url, page_number);

    let response = fetcher.try_fetch(&url).await?;

    let list_view: ListView = response.into();
    let rent_list_items = list_view.extract_items()?;

    debug!(%url, item_count = rent_list_items.len());

    let rent_list = RentList {
        url,
        items: rent_list_items,
    };

    Ok((list_view, rent_list))
}

#[instrument(skip_all, fields(%base_url))]
pub async fn scrape_rent_lists(
    fetcher: &Fetcher,
    base_url: Url,
    limit: Option<u32>,
) -> Result<RentLists, Report> {
    // Scrape first page
    let (first_list_view, first_list) = scrape_rent_list(fetcher, &base_url, 1).await?;

    // Extract metadata from first page
    let page_count = match first_list_view.extract_page_count() {
        Some(page_count) => {
            debug!(page_count);
            page_count
        }
        None => {
            warn!(page_count = "none", fallback = 1);
            1
        }
    };

    let item_count = match first_list_view.extract_item_count() {
        Some(item_count) => {
            debug!(item_count);
            item_count
        }
        None => {
            warn!(item_count = "none", fallback = 0);
            0
        }
    };

    let max_pages = match limit {
        Some(limit) => std::cmp::min(page_count, limit),
        None => page_count,
    };

    let mut rent_lists = Vec::with_capacity(max_pages as usize);

    rent_lists.push(Some(first_list));

    // Scrape remaining pages
    for page_number in 2..=max_pages {
        let rent_list = match scrape_rent_list(fetcher, &base_url, page_number).await {
            Ok((_, list)) => Some(list),

            Err(report) => {
                error!(%page_number, %report);
                eprintln!("{:?}", report);
                None
            }
        };

        rent_lists.push(rent_list);
    }

    match rent_lists.iter().filter(|p| p.is_none()).count() {
        0 => info!(ok = max_pages, "all lists scraped successfully"),
        err => warn!(
            ok = rent_lists.len() - err,
            err, "some lists failed to scrape"
        ),
    }

    Ok(RentLists {
        base_url,
        page_count,
        item_count,
        lists: rent_lists,
    })
}
