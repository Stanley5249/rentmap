use std::ops::Deref;

use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::Json;
use url::Url;

#[derive(Clone, Debug, Deserialize, Serialize, FromRow)]
pub struct RentList {
    pub created_at: NaiveDateTime,
    pub url: Json<Url>,
    pub page_count: Option<u32>,
    pub item_count: Option<u32>,
    #[sqlx(skip)]
    pub pages: Vec<Option<RentListPage>>,
}

impl RentList {
    /// Create a new RentList with current timestamp
    pub fn new(
        url: Url,
        page_count: Option<u32>,
        item_count: Option<u32>,
        pages: Vec<Option<RentListPage>>,
    ) -> Self {
        Self {
            created_at: Utc::now().naive_utc(),
            url: Json(url),
            page_count,
            item_count,
            pages,
        }
    }

    /// Returns an iterator over all pages in the list
    pub fn item_summaries(&self) -> impl Iterator<Item = &RentItemSummary> {
        self.pages
            .iter()
            .filter_map(|list| list.as_ref())
            .flat_map(|list| list.items.iter())
    }

    /// Returns an iterator over all item URLs in all pages
    pub fn item_urls(&self) -> impl Iterator<Item = &Url> {
        self.item_summaries().map(|item| item.url.deref())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RentListPage {
    pub items: Json<Vec<RentItemSummary>>,
}

impl RentListPage {
    /// Create a new RentListPage with Json wrapper
    pub fn new(items: Vec<RentItemSummary>) -> Self {
        Self { items: Json(items) }
    }

    /// Returns an iterator over the item URLs in this list
    pub fn item_urls(&self) -> impl Iterator<Item = &Url> {
        self.items.iter().map(|item| item.url.deref())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RentItemSummary {
    pub url: Json<Url>,
    pub title: Option<String>,
    pub price: Option<String>,
    pub tags: Json<Vec<String>>,
    pub txts: Json<Vec<String>>,
    pub images: Json<Vec<Url>>,
}

impl RentItemSummary {
    /// Create a new RentItemSummary with Json wrappers
    pub fn new(
        url: Url,
        title: Option<String>,
        price: Option<String>,
        tags: Vec<String>,
        txts: Vec<String>,
        images: Vec<Url>,
    ) -> Self {
        Self {
            url: Json(url),
            title,
            price,
            tags: Json(tags),
            txts: Json(txts),
            images: Json(images),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, FromRow)]
pub struct RentItem {
    pub created_at: NaiveDateTime,
    pub url: Json<Url>,
    pub title: Option<String>,
    pub labels: Json<Vec<String>>,
    pub patterns: Json<Vec<String>>,
    pub content: String,
    pub phone: Option<String>,
    pub album: Json<Vec<Url>>,
    pub area: Option<Json<Url>>,
    pub floor: Option<Json<Url>>,
    pub price: Option<Json<Url>>,
    pub address: Option<Json<Url>>,
}

impl RentItem {
    #[allow(clippy::too_many_arguments)]
    /// Create a new RentItem with current timestamp
    pub fn new(
        url: Url,
        title: Option<String>,
        labels: Vec<String>,
        patterns: Vec<String>,
        content: String,
        phone: Option<String>,
        album: Vec<Url>,
        area: Option<Url>,
        floor: Option<Url>,
        price: Option<Url>,
        address: Option<Url>,
    ) -> Self {
        Self {
            created_at: Utc::now().naive_utc(),
            url: Json(url),
            title,
            labels: Json(labels),
            patterns: Json(patterns),
            content,
            phone,
            album: Json(album),
            area: area.map(Json),
            floor: floor.map(Json),
            price: price.map(Json),
            address: address.map(Json),
        }
    }
}
