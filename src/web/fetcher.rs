use std::path::PathBuf;

use miette::IntoDiagnostic;
use scraper::Html;
use tracing::warn;
use url::Url;

use crate::file::save_page;
use crate::web::backends::FetcherBackend;
use crate::web::dom::clean_html;

type Transform = Box<dyn Fn(&mut Html)>;

pub struct Fetcher {
    pub save: Option<PathBuf>,
    pub transforms: Vec<Transform>,
    backend: FetcherBackend,
}

impl Fetcher {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_save(mut self, out_dir: PathBuf) -> Self {
        self.save = Some(out_dir);
        self
    }

    pub fn with_clean(mut self) -> Self {
        self.transforms.push(Box::new(clean_html));
        self
    }

    pub fn with_transform<F>(mut self, transform: F) -> Self
    where
        F: 'static + Fn(&mut Html),
    {
        self.transforms.push(Box::new(transform));
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

        if let Some(out_dir) = &self.save {
            if let Err(report) = save_page(&page, out_dir).into_diagnostic() {
                warn!(%report);
                eprintln!("{:?}", report);
            }
        }

        Ok(document)
    }
}

impl Default for Fetcher {
    fn default() -> Self {
        Self {
            save: None,
            transforms: Vec::new(),
            backend: FetcherBackend::Spider,
        }
    }
}
