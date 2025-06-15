use crate::selectors;
use crate::sites::rent591::model::RentListItem;
use scraper::Html;
use url::Url;

selectors! {
    PAGE_SELECTOR: "ul.paging > li:last-child > a",
    ITEM_SELECTOR: ".item",
    LINK_SELECTOR: ".item-info-title a.link",
    TAG_SELECTOR: ".item-info-tag .tag",
    INFO_SELECTOR: ".item-info-txt",
    IMAGE_SELECTOR: "ul.image-list img.common-img"
}

pub struct ListView {
    pub document: Html,
}

impl ListView {
    pub fn new(document: Html) -> Self {
        Self { document }
    }

    pub fn extract_page_count(&self) -> Result<u32, super::error::Error> {
        let page_selector = &*PAGE_SELECTOR;
        let count = self
            .document
            .select(page_selector)
            .next()
            .ok_or(super::error::Error::NoPageLinks)?
            .text()
            .next()
            .ok_or(super::error::Error::NoPageText)?
            .parse::<u32>()?;

        Ok(count)
    }

    fn extract_item_link(&self, item: &scraper::ElementRef) -> Option<Url> {
        let link_selector = &*LINK_SELECTOR;
        item.select(link_selector)
            .next()
            .and_then(|e| e.value().attr("href"))
            .and_then(|href| Url::parse(href).ok())
    }

    fn extract_item_title(&self, item: &scraper::ElementRef) -> Option<String> {
        let link_selector = &*LINK_SELECTOR;
        item.select(link_selector)
            .next()
            .and_then(|e| e.value().attr("title"))
            .map(String::from)
    }

    fn extract_item_tags(&self, item: &scraper::ElementRef) -> Vec<String> {
        let tag_selector = &*TAG_SELECTOR;
        item.select(tag_selector)
            .map(|el| el.text().collect::<String>().trim().to_string())
            .collect()
    }

    fn extract_item_info(&self, item: &scraper::ElementRef) -> Vec<String> {
        let info_selector = &*INFO_SELECTOR;
        item.select(info_selector)
            .map(|el| el.text().collect::<String>().trim().to_string())
            .collect()
    }

    fn extract_item_images(&self, item: &scraper::ElementRef) -> Vec<Url> {
        let selector = &*IMAGE_SELECTOR;
        item.select(selector)
            .filter_map(|img| img.value().attr("data-src"))
            .filter_map(|src| Url::parse(src).ok())
            .collect()
    }

    pub fn extract_rent_items(&self) -> Result<Vec<RentListItem>, super::error::Error> {
        let selector = &*ITEM_SELECTOR;
        // Use a functional approach with iterators
        let items = self
            .document
            .select(selector)
            .filter_map(|item| {
                // Only proceed if we can extract a valid link
                self.extract_item_link(&item).map(|link| RentListItem {
                    link,
                    title: self.extract_item_title(&item),
                    tags: self.extract_item_tags(&item),
                    info: self.extract_item_info(&item),
                    images: self.extract_item_images(&item),
                })
            })
            .collect();

        Ok(items)
    }
}

impl From<Html> for ListView {
    fn from(document: Html) -> Self {
        Self::new(document)
    }
}
