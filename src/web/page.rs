use sqlx::prelude::FromRow;
use sqlx::types::Json;
use url::Url;

#[derive(Debug, Clone, FromRow)]
pub struct Page {
    pub url: Json<Url>,
    pub html: String,
}

impl Page {
    pub fn new(url: Url, html: String) -> Self {
        Self {
            url: Json(url),
            html,
        }
    }
}
