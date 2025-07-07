use url::Url;

use crate::web::Page;
use crate::web::WebError;

pub enum BackendType {
    Spider,
}

impl BackendType {
    pub async fn fetch_page(&self, url: &Url) -> Result<Page, WebError> {
        match self {
            Self::Spider => super::spider::fetch_page(url).await,
        }
    }
}
