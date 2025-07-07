use miette::IntoDiagnostic;
use scraper::Html;
use url::Url;

use super::backends::BackendType;
use super::error::WebError;
use crate::error::TraceReport;
use crate::file::Workspace;
use crate::scraper::clean_html;

type Transform = Box<dyn Fn(&mut Html)>;

pub struct Fetcher {
    pub workspace: Option<Workspace>,
    pub transforms: Vec<Transform>,
    backend: BackendType,
}

impl Fetcher {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_workspace(mut self, workspace: Workspace) -> Self {
        self.workspace = Some(workspace);
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

    pub async fn try_fetch(&self, url: &Url) -> Result<Html, WebError> {
        let mut page = self.backend.fetch_page(url).await?;

        let mut document = Html::parse_document(&page.html);

        if !self.transforms.is_empty() {
            for transform in &self.transforms {
                transform(&mut document);
            }
            page.html = document.html();
        }

        if let Some(workspace) = &self.workspace {
            workspace
                .save_page(&page)
                .into_diagnostic()
                .trace_report()
                .ok();
        }

        Ok(document)
    }
}

impl Default for Fetcher {
    fn default() -> Self {
        Self {
            workspace: None,
            transforms: Vec::new(),
            backend: BackendType::Spider,
        }
    }
}
