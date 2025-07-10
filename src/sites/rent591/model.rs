use serde::{Deserialize, Serialize};
use url::Url;

use crate::sites::rent591::{ItemUrl, ListUrl};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RentList {
    pub url: ListUrl,
    pub page_count: Option<u32>,
    pub item_count: Option<u32>,
    pub pages: Vec<Option<RentListPage>>,
}

impl RentList {
    /// Returns an iterator over all item URLs in all pages
    pub fn item_urls(&self) -> impl Iterator<Item = &ItemUrl> {
        self.pages
            .iter()
            .filter_map(|list| list.as_ref())
            .flat_map(|list| list.item_urls())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RentListPage {
    pub items: Vec<RentItemSummary>,
}

impl RentListPage {
    /// Returns an iterator over the item URLs in this list
    pub fn item_urls(&self) -> impl Iterator<Item = &ItemUrl> {
        self.items.iter().filter_map(|item| item.url.as_ref())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RentItemSummary {
    pub url: Option<ItemUrl>,
    pub title: Option<String>,
    pub price: Option<String>,
    pub tags: Vec<String>,
    pub txts: Vec<String>,
    pub images: Vec<Url>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RentItem {
    pub url: ItemUrl,
    pub title: Option<String>,
    pub labels: Vec<String>,
    pub patterns: Vec<String>,
    pub content: String,
    pub phone: Option<String>,
    pub album: Vec<Url>,
    pub area: Option<Url>,
    pub floor: Option<Url>,
    pub price: Option<Url>,
    pub address: Option<Url>,
}
