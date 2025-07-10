use std::sync::LazyLock;

use scraper::{ElementRef, Html};
use url::Url;

use super::ViewError;
use crate::define_selectors;
use crate::scraper::ElementExt;
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

    fn extract_url_from_item(&self, item: &ElementRef) -> Option<ItemUrl> {
        let selector = &LIST_SELECTORS.item_info_link;
        item.select_url(selector, "href")
            .filter_map(|url| ItemUrl::try_from(url).ok())
            .next()
    }

    fn extract_title_from_item(&self, item: &ElementRef) -> Option<String> {
        let selector = &LIST_SELECTORS.item_info_link;
        item.select(selector)
            .filter_map(|e| e.attr("title"))
            .next()
            .map(|s| s.trim().to_string())
    }

    fn extract_info_price_from_item(&self, item: &ElementRef) -> Option<String> {
        let selector = &LIST_SELECTORS.item_info_price;
        item.select_text_join(selector, " ").next()
    }

    fn extract_info_tags_from_item(&self, item: &ElementRef) -> Vec<String> {
        let selector = &LIST_SELECTORS.item_info_tag;
        item.select_text_concat(selector).collect()
    }

    fn extract_info_txts_from_item(&self, item: &ElementRef) -> Vec<String> {
        let selector = &LIST_SELECTORS.item_info_txt;
        item.select_text_join(selector, " ").collect()
    }

    fn extract_images_from_item(&self, item: &ElementRef) -> Vec<Url> {
        let selector = &LIST_SELECTORS.image;
        item.select_url(selector, "data-src").collect()
    }

    fn extract_summary_from_item(&self, item: &ElementRef) -> RentItemSummary {
        RentItemSummary {
            url: self.extract_url_from_item(item),
            title: self.extract_title_from_item(item),
            price: self.extract_info_price_from_item(item),
            tags: self.extract_info_tags_from_item(item),
            txts: self.extract_info_txts_from_item(item),
            images: self.extract_images_from_item(item),
        }
    }

    fn select_root(&self) -> Result<ElementRef<'_>, ViewError> {
        let root_selector = &LIST_SELECTORS.root;
        self.document
            .select(root_selector)
            .next()
            .ok_or(ViewError::NoList)
    }

    fn extract_page_count_from_root(&self, root: &ElementRef) -> Option<u32> {
        let selector = &LIST_SELECTORS.page_count;
        root.select_from_str(selector).next()
    }

    fn extract_item_count_from_root(&self, root: &ElementRef) -> Option<u32> {
        let selector = &LIST_SELECTORS.item_count;
        root.select_from_str(selector).next()
    }

    fn extract_page_from_root(&self, root: &ElementRef) -> Result<RentListPage, ViewError> {
        let item_selector = &LIST_SELECTORS.item;

        let items: Vec<_> = root
            .select(item_selector)
            .map(|item| self.extract_summary_from_item(&item))
            .collect();

        if items.is_empty() {
            Err(ViewError::NoItemSummaries)
        } else {
            Ok(RentListPage { items })
        }
    }

    pub fn extract_rent_list_page(&self) -> Result<RentListPage, ViewError> {
        let root = self.select_root()?;
        self.extract_page_from_root(&root)
    }

    pub fn extract_rent_list(&self, url: ListUrl) -> Result<RentList, ViewError> {
        let root = self.select_root()?;
        self.extract_page_from_root(&root).map(|page| RentList {
            url,
            item_count: self.extract_item_count_from_root(&root),
            page_count: self.extract_page_count_from_root(&root),
            pages: vec![Some(page)],
        })
    }
}
