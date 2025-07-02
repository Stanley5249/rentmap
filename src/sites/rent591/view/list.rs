use std::sync::LazyLock;

use scraper::Html;
use url::Url;

use super::error::Error;
use crate::define_selectors;
use crate::sites::rent591::model::RentItemSummary;

define_selectors! {
    ListSelectors,
    page_count: "ul.paging > li:last-child > a",
    item_count: ".list-sort .total strong",
    item: ".item",
    item_info_link: ".item-info-title a.link",
    item_info_price: ".item-info-price",
    item_info_tag: ".item-info-tag .tag",
    item_info_txt: ".item-info-txt",
    image: "ul.image-list img.common-img"
}

static LIST_SELECTORS: LazyLock<ListSelectors> = LazyLock::new(ListSelectors::new);

pub struct ListView {
    pub document: Html,
}

impl ListView {
    pub fn new(document: Html) -> Self {
        Self { document }
    }

    pub fn extract_page_count(&self) -> Option<u32> {
        let selector = &LIST_SELECTORS.page_count;
        self.document
            .select(selector)
            .next()
            .and_then(|e| e.text().next().and_then(|s| s.parse::<u32>().ok()))
    }

    pub fn extract_item_count(&self) -> Option<u32> {
        let selector = &LIST_SELECTORS.item_count;
        self.document
            .select(selector)
            .next()
            .and_then(|e| e.text().next().and_then(|s| s.parse::<u32>().ok()))
    }

    fn extract_item_link_and_title(
        &self,
        item: &scraper::ElementRef,
    ) -> (Option<Url>, Option<String>) {
        let selector = &LIST_SELECTORS.item_info_link;
        item.select(selector)
            .next()
            .map(|e| {
                let value = e.value();
                let link = value.attr("href").and_then(|s| Url::parse(s).ok());
                let title = value.attr("title").map(String::from);
                (link, title)
            })
            .unwrap_or((None, None))
    }

    fn extract_item_info_price(&self, item: &scraper::ElementRef) -> Option<String> {
        let selector = &LIST_SELECTORS.item_info_price;
        item.select(selector).next().map(|e| {
            e.text()
                .map(|t| t.trim())
                .filter(|t| !t.is_empty())
                .collect::<Vec<_>>()
                .join(" ")
        })
    }

    fn extract_item_info_tags(&self, item: &scraper::ElementRef) -> Vec<String> {
        let selector = &LIST_SELECTORS.item_info_tag;
        item.select(selector)
            .map(|e| e.text().collect::<String>().trim().to_string())
            .collect()
    }

    fn extract_item_info_txts(&self, item: &scraper::ElementRef) -> Vec<String> {
        let selector = &LIST_SELECTORS.item_info_txt;
        item.select(selector)
            .map(|e| {
                e.text()
                    .map(|t| t.trim())
                    .filter(|t| !t.is_empty())
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .collect()
    }

    fn extract_item_images(&self, item: &scraper::ElementRef) -> Vec<Url> {
        let selector = &LIST_SELECTORS.image;
        item.select(selector)
            .filter_map(|img| img.value().attr("data-src"))
            .filter_map(|src| Url::parse(src).ok())
            .collect()
    }

    pub fn extract_item_summaries(&self) -> Result<Vec<RentItemSummary>, Error> {
        let selector = &LIST_SELECTORS.item;
        let items = self
            .document
            .select(selector)
            .filter_map(|item| {
                let (url, title) = self.extract_item_link_and_title(&item);
                url.map(|url| RentItemSummary {
                    url,
                    title,
                    price: self.extract_item_info_price(&item),
                    tags: self.extract_item_info_tags(&item),
                    txts: self.extract_item_info_txts(&item),
                    images: self.extract_item_images(&item),
                })
            })
            .collect::<Vec<_>>();
        if items.is_empty() {
            Err(Error::NoItems)
        } else {
            Ok(items)
        }
    }
}

impl From<Html> for ListView {
    fn from(document: Html) -> Self {
        Self::new(document)
    }
}
