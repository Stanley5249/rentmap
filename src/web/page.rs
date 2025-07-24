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

impl From<&spider::page::Page> for Page {
    fn from(page: &spider::page::Page) -> Self {
        Self::new(
            page.get_url_final()
                .parse()
                .expect("expected valid URL from spider"),
            page.get_html(),
        )
    }
}
