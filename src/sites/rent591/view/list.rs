use std::sync::LazyLock;

use scraper::{ElementRef, Html};
use url::Url;

use super::ViewError;
use crate::define_selectors;
use crate::sites::rent591::{ItemUrl, ListUrl, RentItemSummary, RentList, RentListPage};

define_selectors! {
    ListSelectors,
    root: "#__nuxt",
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
            .and_then(|e| e.text().collect::<String>().trim().parse().ok())
    }

    pub fn extract_item_count(&self) -> Option<u32> {
        let selector = &LIST_SELECTORS.item_count;
        self.document
            .select(selector)
            .next()
            .and_then(|e| e.text().collect::<String>().trim().parse().ok())
    }

    fn extract_item_url(&self, item: &ElementRef) -> Option<ItemUrl> {
        let selector = &LIST_SELECTORS.item_info_link;
        item.select(selector)
            .next()
            .and_then(|e| e.value().attr("href"))
            .and_then(|s| Url::parse(s).ok())
            .and_then(|url| ItemUrl::try_from(url).ok())
    }

    fn extract_item_title(&self, item: &ElementRef) -> Option<String> {
        let selector = &LIST_SELECTORS.item_info_link;
        item.select(selector)
            .next()
            .and_then(|e| e.value().attr("title"))
            .map(|s| s.to_string())
    }

    fn extract_item_info_price(&self, item: &ElementRef) -> Option<String> {
        let selector = &LIST_SELECTORS.item_info_price;
        item.select(selector).next().map(|e| {
            e.text()
                .map(|t| t.trim())
                .filter(|t| !t.is_empty())
                .collect::<Vec<_>>()
                .join(" ")
        })
    }

    fn extract_item_info_tags(&self, item: &ElementRef) -> Vec<String> {
        let selector = &LIST_SELECTORS.item_info_tag;
        item.select(selector)
            .map(|e| e.text().collect::<String>().trim().to_string())
            .collect()
    }

    fn extract_item_info_txts(&self, item: &ElementRef) -> Vec<String> {
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

    fn extract_item_images(&self, item: &ElementRef) -> Vec<Url> {
        let selector = &LIST_SELECTORS.image;
        item.select(selector)
            .filter_map(|img| img.value().attr("data-src"))
            .filter_map(|src| Url::parse(src).ok())
            .collect()
    }

    fn extract_item_summary(&self, item: &ElementRef) -> Option<RentItemSummary> {
        self.extract_item_url(item).map(|url| RentItemSummary {
            url,
            title: self.extract_item_title(item),
            price: self.extract_item_info_price(item),
            tags: self.extract_item_info_tags(item),
            txts: self.extract_item_info_txts(item),
            images: self.extract_item_images(item),
        })
    }

    fn select_root(&self) -> Result<ElementRef<'_>, ViewError> {
        let root_selector = &LIST_SELECTORS.root;
        self.document
            .select(root_selector)
            .next()
            .ok_or(ViewError::NoList)
    }

    fn extract_list_page_from_root(&self, root: &ElementRef) -> Result<RentListPage, ViewError> {
        let item_selector = &LIST_SELECTORS.item;

        let items = root
            .select(item_selector)
            .map(|item| self.extract_item_summary(&item))
            .collect::<Vec<_>>();

        if items.is_empty() {
            Err(ViewError::NoItemSummaries)
        } else {
            Ok(RentListPage { items })
        }
    }

    pub fn extract_list_page(&self) -> Result<RentListPage, ViewError> {
        let root = self.select_root()?;
        self.extract_list_page_from_root(&root)
    }

    pub fn extract_list(&self, url: ListUrl) -> Result<RentList, ViewError> {
        let root = self.select_root()?;
        self.extract_list_page_from_root(&root)
            .map(|page| RentList {
                url,
                item_count: self.extract_item_count(),
                page_count: self.extract_page_count(),
                pages: vec![Some(page)],
            })
    }
}

impl From<Html> for ListView {
    fn from(document: Html) -> Self {
        Self::new(document)
    }
}
