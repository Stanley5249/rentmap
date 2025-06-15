use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Deserialize, Serialize)]
pub struct RentLists {
    pub base_url: Url,
    pub page_count: u32,
    pub lists: Vec<Option<RentList>>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct RentList {
    pub page_number: u32,
    pub items: Vec<RentListItem>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct RentListItem {
    pub link: Url,
    pub title: Option<String>,
    pub tags: Vec<String>,
    pub info: Vec<String>,
    pub images: Vec<Url>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct RentItem {
    pub url: Url,
    pub title: Option<String>,
    pub price: Option<String>,
    pub location: Option<String>,
    pub area: Option<String>,
    pub room_type: Option<String>,
    pub floor: Option<String>,
    pub description: Option<String>,
    pub contact_info: Option<String>,
    pub images: Vec<Url>,
}
