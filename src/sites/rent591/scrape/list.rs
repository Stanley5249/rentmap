use miette::Result;
use tracing::{debug, info, instrument, warn};
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

            rent_list.pages.reserve(max_pages as usize - 1);

            for page_number in 2..=max_pages {
                let list_page = scrape_list_page(fetcher, url, page_number)
                    .await
                    .trace_report()
                    .ok();

                rent_list.pages.push(list_page);
            }

            let ok = rent_list.pages.iter().filter(|p| p.is_some()).count();
            let err = rent_list.pages.len() - ok;

            if err == 0 {
                info!(ok, "scrape all pages")
            } else {
                warn!(ok, err, "scrape pages with errors")
            }
        }
        _ => warn!(page_count = "none", "scrape first page only"),
    }

    Ok(rent_list)
}
