use futures::stream::StreamExt;
use miette::Result;
use tracing::{debug, error, info, instrument, warn};
use url::Url;

use crate::error::TraceReport;
use crate::sites::rent591::{ListUrlExt, ListView, RentList, RentListPage};
use crate::web::Fetcher;

#[instrument(skip_all, fields(%url, %page))]
async fn scrape_list_page(fetcher: &Fetcher, url: &Url, page: u32) -> Result<RentListPage> {
    let url = url.with_page(page);

    let response = fetcher.try_fetch(&url).await?;

    let list_view = ListView::new(response);

    let list_page = list_view.extract_list_page()?;

    debug!(item_count = list_page.items.len());

    Ok(list_page)
}

#[instrument(skip_all, fields(%url))]
async fn scrape_list(fetcher: &Fetcher, url: &Url) -> Result<RentList> {
    use crate::sites::rent591::ListUrlExt;

    let url = url.without_page();

    let response = fetcher.try_fetch(&url).await?;

    let list_view = ListView::new(response);

    let rent_list = list_view.extract_list(url)?;

    if let Some(page_count) = rent_list.page_count {
        debug!(page_count);
    }

    if let Some(item_count) = rent_list.item_count {
        debug!(item_count);
    }

    Ok(rent_list)
}

pub async fn scrape_list_and_pages(
    fetcher: &Fetcher,
    url: &Url,
    limit: Option<u32>,
) -> Result<RentList> {
    let mut rent_list = scrape_list(fetcher, url).await?;

    match rent_list.page_count {
        Some(page_count) => {
            let max_pages = limit.map_or(page_count, |x| x.min(page_count));

            let future = async |page_number| {
                scrape_list_page(fetcher, url, page_number)
                    .await
                    .trace_report()
                    .ok()
            };

            let mut pages: Vec<_> = futures::stream::iter(2..=max_pages)
                .map(future)
                .buffer_unordered(10)
                .filter_map(std::future::ready)
                .collect()
                .await;

            rent_list.pages.append(&mut pages);

            let ok = rent_list.pages.len() as u32;

            match max_pages.saturating_sub(ok) {
                0 => info!(ok, "scrape all pages"),
                err => error!(ok, err, "scrape pages with errors"),
            }
        }
        _ => warn!(page_count = "none", "scrape first page only"),
    }

    Ok(rent_list)
}
