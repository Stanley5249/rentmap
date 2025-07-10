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
) -> Result<RentListPage> {
    let url = url.with_page(page);

    let response = fetcher.try_fetch(&url).await?;

    let list_view = ListView::new(response);

    let list_page = list_view.extract_rent_list_page()?;

    debug!(item_count = list_page.items.len());

    Ok(list_page)
}

#[instrument(skip_all, fields(%url))]
async fn scrape_rent_list(fetcher: &Fetcher, url: &ListUrl) -> Result<RentList> {
    let url = url.without_page();

    let response = fetcher.try_fetch(&url).await?;

    let list_view = ListView::new(response);

    let rent_list = list_view.extract_rent_list(url)?;

    if let Some(page_count) = rent_list.page_count {
        debug!(page_count);
    }

    if let Some(item_count) = rent_list.item_count {
        debug!(item_count);
    }

    Ok(rent_list)
}

pub async fn scrape_rent_list_and_pages(
    fetcher: &Fetcher,
    url: &ListUrl,
    limit: Option<u32>,
) -> Result<TimedRecord<RentList>> {
    let mut rent_list = scrape_rent_list(fetcher, url).await?;

    match rent_list.page_count {
        Some(page_count) => {
            let max_pages = limit.map_or(page_count, |x| x.min(page_count));

            rent_list.pages.reserve(max_pages as usize - 1);

            for page_number in 2..=max_pages {
                let list_page = scrape_rent_list_page(fetcher, url, page_number)
                    .await
                    .trace_report()
                    .ok();

                rent_list.pages.push(list_page);
            }

            let ok = rent_list.pages.iter().filter(|p| p.is_some()).count();
            let err = rent_list.pages.len() - ok;

            if err == 0 {
                info!(ok, "all pages scraped successfully")
            } else {
                warn!(ok, err, "some pages failed to scrape")
            }
        }
        _ => warn!(page_count = "none", "scrape only the first page"),
    }

    Ok(rent_list.into())
}
