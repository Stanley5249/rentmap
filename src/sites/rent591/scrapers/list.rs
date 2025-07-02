use miette::Result;
use tracing::{debug, info, instrument, warn};
use url::Url;

use crate::error::TraceReport;
use crate::sites::rent591::model::{RentList, RentListPage};
use crate::sites::rent591::view::ListView;
use crate::web::fetcher::Fetcher;

fn build_page_url(base_url: &Url, page_number: u32) -> Url {
    let mut page_url = base_url.clone();
    page_url
        .query_pairs_mut()
        .append_pair("page", &page_number.to_string());
    page_url
}

async fn scrape_rent_list_page(
    fetcher: &Fetcher,
    base_url: &Url,
    page: u32,
) -> Result<(ListView, RentListPage)> {
    let url = build_page_url(base_url, page);

    let response = fetcher.try_fetch(&url).await?;

    let list_view: ListView = response.into();
    let rent_list_items = list_view.extract_item_summaries()?;

    debug!(%url, item_count = rent_list_items.len());

    let rent_list = RentListPage {
        page,
        items: rent_list_items,
    };

    Ok((list_view, rent_list))
}

#[instrument(skip_all, fields(%url))]
pub async fn scrape_rent_list(fetcher: &Fetcher, url: Url, limit: Option<u32>) -> Result<RentList> {
    // Scrape first page
    let (first_list_view, first_list) = scrape_rent_list_page(fetcher, &url, 1).await?;

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

    let mut rent_list_pages = Vec::with_capacity(max_pages as usize);

    rent_list_pages.push(Some(first_list));

    // Scrape remaining pages
    for page_number in 2..=max_pages {
        let list_page = scrape_rent_list_page(fetcher, &url, page_number)
            .await
            .trace_report()
            .ok()
            .map(|(_, list)| list);
        rent_list_pages.push(list_page);
    }

    match rent_list_pages.iter().filter(|p| p.is_none()).count() {
        0 => info!(ok = max_pages, "all pages scraped successfully"),
        err => warn!(
            ok = rent_list_pages.len() - err,
            err, "some pages failed to scrape"
        ),
    }

    Ok(RentList {
        url,
        page_count,
        item_count,
        pages: rent_list_pages,
    })
}
