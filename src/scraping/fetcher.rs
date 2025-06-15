use super::page::Page;
use crate::file::save_page;
use crate::scraping::dom::clean_html;
use scraper::Html;
use std::path::{Path, PathBuf};
use tracing::error;
use url::Url;
enum FetcherBackend {
    Spider,
}

impl FetcherBackend {
    async fn fetch_page(&self, url: &Url) -> Result<Page, super::error::Error> {
        match self {
            FetcherBackend::Spider => super::spider::fetch_page(url).await,
        }
    }
}

pub struct Fetcher {
    pub save: Option<PathBuf>,
    pub transforms: Vec<fn(&mut Html)>,
    backend: FetcherBackend,
}

impl Fetcher {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_save<P: AsRef<Path>>(mut self, output_dir: P) -> Self {
        self.save = Some(output_dir.as_ref().to_path_buf());
        self
    }

    pub fn with_clean(mut self) -> Self {
        self.transforms.push(clean_html);
        self
    }

    pub async fn try_fetch(&self, url: &Url) -> Result<Html, super::error::Error> {
        let mut page = self.backend.fetch_page(url).await?;

        let mut document = Html::parse_document(&page.html);

        if !self.transforms.is_empty() {
            for transform in &self.transforms {
                transform(&mut document);
            }
            page.html = document.html();
        }

        if let Some(output_dir) = &self.save {
            if let Err(error) = save_page(&page, output_dir) {
                error!(%error);
            };
        }

        Ok(document)
    }
}

impl Default for Fetcher {
    fn default() -> Self {
        Self {
            save: None,
            transforms: vec![],
            backend: FetcherBackend::Spider,
        }
    }
}
