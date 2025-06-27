use scraper::Html;
use url::Url;

use crate::selectors;
use crate::sites::rent591::model::RentListItem;

selectors! {
    PAGE_COUNT_SELECTOR: "ul.paging > li:last-child > a",
    ITEM_COUNT_SELECTOR: ".list-sort .total strong",
    ITEM_SELECTOR: ".item",
    ITEM_INFO_LINK_SELECTOR: ".item-info-title a.link",
    ITEM_INFO_PRICE_SELECTOR: ".item-info-price",
    ITEM_INFO_TAG_SELECTOR: ".item-info-tag .tag",
    ITEM_INFO_TXT_SELECTOR: ".item-info-txt",
    IMAGE_SELECTOR: "ul.image-list img.common-img"
}

pub struct ListView {
    pub document: Html,
}

impl ListView {
    pub fn new(document: Html) -> Self {
        Self { document }
    }

    pub fn extract_page_count(&self) -> Option<u32> {
        let selector = &*PAGE_COUNT_SELECTOR;

        self.document
            .select(selector)
            .next()
            .and_then(|e| e.text().next().and_then(|s| s.parse::<u32>().ok()))
    }

    pub fn extract_item_count(&self) -> Option<u32> {
        let selector = &*ITEM_COUNT_SELECTOR;

        self.document
            .select(selector)
            .next()
            .and_then(|e| e.text().next().and_then(|s| s.parse::<u32>().ok()))
    }

    fn extract_item_link_and_title(
        &self,
        item: &scraper::ElementRef,
    ) -> (Option<Url>, Option<String>) {
        let selector = &*ITEM_INFO_LINK_SELECTOR;

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
        let selector = &*ITEM_INFO_PRICE_SELECTOR;

        item.select(selector).next().map(|e| {
            e.text()
                .map(|t| t.trim())
                .filter(|t| !t.is_empty())
                .collect::<Vec<_>>()
                .join(" ")
        })
    }

    fn extract_item_info_tags(&self, item: &scraper::ElementRef) -> Vec<String> {
        let selector = &*ITEM_INFO_TAG_SELECTOR;

        item.select(selector)
            .map(|e| e.text().collect::<String>().trim().to_string())
            .collect()
    }

    fn extract_item_info_txts(&self, item: &scraper::ElementRef) -> Vec<String> {
        let selector = &*ITEM_INFO_TXT_SELECTOR;

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
        let selector = &*IMAGE_SELECTOR;

        item.select(selector)
            .filter_map(|img| img.value().attr("data-src"))
            .filter_map(|src| Url::parse(src).ok())
            .collect()
    }

    pub fn extract_items(&self) -> Result<Vec<RentListItem>, super::error::Error> {
        let selector = &*ITEM_SELECTOR;

        let items = self
            .document
            .select(selector)
            .filter_map(|item| {
                let (link, title) = self.extract_item_link_and_title(&item);
                link.map(|link| RentListItem {
                    link,
                    title,
                    price: self.extract_item_info_price(&item),
                    tags: self.extract_item_info_tags(&item),
                    txts: self.extract_item_info_txts(&item),
                    images: self.extract_item_images(&item),
                })
            })
            .collect::<Vec<_>>();

        if items.is_empty() {
            Err(super::error::Error::NoItems)
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
