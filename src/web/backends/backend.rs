use url::Url;

use crate::web::{Page, WebError};

#[derive(Default)]
pub enum BackendType {
    #[default]
    Spider,
}

impl BackendType {
    pub async fn fetch_page(&self, url: &Url) -> Result<Page, WebError> {
        match self {
            Self::Spider => super::spider::fetch_page(url).await,
        }
    }
}
