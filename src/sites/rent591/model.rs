use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RentLists {
    pub url: Url,
    pub page_count: u32,
    pub item_count: u32,
    pub lists: Vec<Option<RentList>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RentList {
    pub page: u32,
    pub items: Vec<RentListItem>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RentListItem {
    pub link: Url,
    pub title: Option<String>,
    pub price: Option<String>,
    pub tags: Vec<String>,
    pub txts: Vec<String>,
    pub images: Vec<Url>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RentItemDetail {
    pub title: Option<String>,
    pub price: Option<Url>,
    pub location: Option<Url>,
    pub area: Option<Url>,
    pub room_type: Option<Url>,
    pub description: Option<String>,
    pub contact_info: Option<String>,
    pub images: Vec<Url>,
}
