use ::std::sync::LazyLock;
use scraper::{ElementRef, Html, Selector};
use url::Url;

use super::ViewError;
use crate::define_selectors;
use crate::sites::rent591::RentItem;

define_selectors! {
    ItemSelectors,
    root: "#__nuxt",
    title: ".title h1",
    house_label_item: ".house-label .label-item",
    pattern: ".pattern span",
    service: ".service-cate",
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

    fn extract_title(&self, root: &ElementRef) -> Option<String> {
        let selector = &ITEM_SELECTORS.title;
        root.select(selector)
            .next()
            .and_then(|element| element.text().next().map(|text| text.trim().to_string()))
    }

    fn extract_house_labels(&self, root: &ElementRef) -> Vec<String> {
        let selector = &ITEM_SELECTORS.house_label_item;
        root.select(selector)
            .filter_map(|element| element.text().next().map(|text| text.trim().to_string()))
            .collect()
    }

    fn extract_patterns(&self, root: &ElementRef) -> Vec<String> {
        let selector = &ITEM_SELECTORS.pattern;
        root.select(selector)
            .filter_map(|element| {
                element
                    .text()
                    .next()
                    .map(|text| text.trim().to_string())
                    .filter(|text| !text.is_empty())
            })
            .collect()
    }

    fn extract_services(&self, root: &ElementRef) -> String {
        let selector = &ITEM_SELECTORS.service;
        root.select(selector)
            .flat_map(|e| e.text())
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn extract_phone(&self, root: &ElementRef) -> Option<String> {
        let selector = &ITEM_SELECTORS.phone;
        root.select(selector)
            .next()
            .and_then(|element| element.text().next().map(|text| text.trim().to_string()))
    }

    fn extract_album(&self, root: &ElementRef) -> Vec<Url> {
        let selector = &ITEM_SELECTORS.album;
        root.select(selector)
            .filter_map(|element| {
                element
                    .value()
                    .attr("data-src")
                    .and_then(|src| Url::parse(src).ok())
            })
            .collect()
    }

    fn extract_area(&self, root: &ElementRef) -> Option<Url> {
        let selector = &ITEM_SELECTORS.area;
        extract_obfuscated_img_src_from(root, selector)
    }

    fn extract_floor(&self, root: &ElementRef) -> Option<Url> {
        let selector = &ITEM_SELECTORS.floor;
        extract_obfuscated_img_src_from(root, selector)
    }

    fn extract_price(&self, root: &ElementRef) -> Option<Url> {
        let selector = &ITEM_SELECTORS.price;
        extract_obfuscated_img_src_from(root, selector)
    }

    fn extract_address(&self, root: &ElementRef) -> Option<Url> {
        let selector = &ITEM_SELECTORS.address;
        extract_obfuscated_img_src_from(root, selector)
    }

    pub fn extract_rent_item(&self, url: Url) -> Result<RentItem, ViewError> {
        let selector = &ITEM_SELECTORS.root;

        let root = self
            .document
            .select(selector)
            .next()
            .ok_or(ViewError::NoItem)?;

        let title = self.extract_title(&root);
        let labels = self.extract_house_labels(&root);
        let patterns = self.extract_patterns(&root);
        let services = self.extract_services(&root);
        let phone = self.extract_phone(&root);
        let album = self.extract_album(&root);
        let area = self.extract_area(&root);
        let floor = self.extract_floor(&root);
        let price = self.extract_price(&root);
        let address = self.extract_address(&root);

        let rent_item = RentItem {
            url,
            title,
            labels,
            patterns,
            services,
            phone,
            album,
            area,
            floor,
            price,
            address,
        };

        Ok(rent_item)
    }
}

// Standard From trait - your brilliant design pattern!
impl From<Html> for ItemView {
    fn from(document: Html) -> Self {
        Self::new(document)
    }
}

fn extract_obfuscated_img_src_from(root: &ElementRef, selector: &Selector) -> Option<Url> {
    root.select(selector).next().and_then(|element| {
        element
            .value()
            .attr("src")
            .and_then(|src| Url::parse(src).ok())
    })
}
