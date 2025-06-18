pub mod error;
pub mod spider;

use crate::web::page::Page;
use url::Url;

pub enum FetcherBackend {
    Spider,
}

impl FetcherBackend {
    pub async fn fetch_page(&self, url: &Url) -> Result<Page, super::error::Error> {
        match self {
            FetcherBackend::Spider => spider::fetch_page(url).await,
        }
    }
}
