use miette::Result;
use tracing::{debug, info, instrument, warn};

use crate::error::TraceReport;
use crate::file::TimedRecord;
use crate::sites::rent591::{ListUrl, ListView, RentList, RentListPage};
use crate::web::Fetcher;

#[instrument(skip_all, fields(%url, %page))]
async fn scrape_rent_list_page(
    fetcher: &Fetcher,
    url: &ListUrl,
    page: u32,
) -> Result<(ListView, RentListPage)> {
    let url = url.with_page(page);

    let response = fetcher.try_fetch(&url).await?;

    let list_view: ListView = response.into();
    let rent_list_items = list_view.extract_item_summaries()?;

    debug!(item_count = rent_list_items.len());

    let rent_list = RentListPage {
        page,
        items: rent_list_items,
    };

    Ok((list_view, rent_list))
}

pub async fn scrape_rent_list(
    fetcher: &Fetcher,
    url: &ListUrl,
    limit: Option<u32>,
) -> Result<TimedRecord<RentList>> {
    let url = url.without_page();

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
    }
    .into())
}
