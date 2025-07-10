use std::sync::LazyLock;

use scraper::{ElementRef, Html};
use url::Url;

use super::ViewError;
use crate::define_selectors;
use crate::scraper::ElementExt;
use crate::sites::rent591::{ItemUrl, RentItem};

define_selectors! {
    ItemSelectors,
    root: "#__nuxt",
    title: ".title h1",
    house_label_item: ".house-label .label-item",
    pattern: ".pattern span",
    content: ".main-content > :not(.info-board):not(.question)",
    area: ".pattern wc-obfuscate-c-area ~ img.printing-show",
    floor: ".pattern wc-obfuscate-c-floor ~ img.printing-show",
    price: ".house-price wc-obfuscate-c-price ~ img.printing-show",
    address: ".address wc-obfuscate-rent-map-address ~ img.printing-show",
    phone: ".phone > a > button > span.icon + span > span",
    album: ".album img.common-img",
}

static ITEM_SELECTORS: LazyLock<ItemSelectors> = LazyLock::new(ItemSelectors::new);

pub struct ItemView {
    pub document: Html,
}

impl ItemView {
    pub fn new(document: Html) -> Self {
        Self { document }
    }

    fn extract_title_from_root(&self, root: &ElementRef) -> Option<String> {
        let selector = &ITEM_SELECTORS.title;
        root.select_text_concat(selector).next()
    }

    fn extract_house_labels(&self, root: &ElementRef) -> Vec<String> {
        let selector = &ITEM_SELECTORS.house_label_item;
        root.select_text_concat(selector).collect()
    }

    fn extract_patterns(&self, root: &ElementRef) -> Vec<String> {
        let selector = &ITEM_SELECTORS.pattern;
        root.select_text_concat(selector).collect()
    }

    fn extract_content(&self, root: &ElementRef) -> String {
        let selector = &ITEM_SELECTORS.content;
        root.select_content(selector)
    }

    fn extract_phone(&self, root: &ElementRef) -> Option<String> {
        let selector = &ITEM_SELECTORS.phone;
        root.select_text_concat(selector).next()
    }

    fn extract_album(&self, root: &ElementRef) -> Vec<Url> {
        let selector = &ITEM_SELECTORS.album;
        root.select_url(selector, "data-src").collect()
    }

    fn extract_area(&self, root: &ElementRef) -> Option<Url> {
        let selector = &ITEM_SELECTORS.area;
        root.select_url(selector, "src").next()
    }

    fn extract_floor(&self, root: &ElementRef) -> Option<Url> {
        let selector = &ITEM_SELECTORS.floor;
        root.select_url(selector, "src").next()
    }

    fn extract_price(&self, root: &ElementRef) -> Option<Url> {
        let selector = &ITEM_SELECTORS.price;
        root.select_url(selector, "src").next()
    }

    fn extract_address(&self, root: &ElementRef) -> Option<Url> {
        let selector = &ITEM_SELECTORS.address;
        root.select_url(selector, "src").next()
    }

    pub fn extract_rent_item(&self, url: ItemUrl) -> Result<RentItem, ViewError> {
        let selector = &ITEM_SELECTORS.root;

        self.document
            .select(selector)
            .next()
            .ok_or(ViewError::NoItem)
            .map(|root| RentItem {
                url,
                title: self.extract_title_from_root(&root),
                labels: self.extract_house_labels(&root),
                patterns: self.extract_patterns(&root),
                content: self.extract_content(&root),
                phone: self.extract_phone(&root),
                album: self.extract_album(&root),
                area: self.extract_area(&root),
                floor: self.extract_floor(&root),
                price: self.extract_price(&root),
                address: self.extract_address(&root),
            })
    }
}
