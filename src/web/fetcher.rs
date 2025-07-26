use miette::{IntoDiagnostic, Result};
use scraper::Html;
use url::Url;

use super::{Backend, BackendType};
use crate::error::TraceReport;
use crate::scraper::HtmlExt;
use crate::web::Page;
use crate::workspace::Workspace;

pub struct Fetcher {
    pub cache: bool,
    pub clean: bool,
    pub workspace: Workspace,
    pub backend: Backend,
}

impl Fetcher {
    pub fn new(workspace: Workspace) -> Self {
        Self {
            cache: false,
            clean: false,
            workspace,
            backend: Backend::from(BackendType::default()),
        }
    }

    pub fn with_clean(mut self) -> Self {
        self.clean = true;
        self
    }

    pub fn with_cache(mut self) -> Self {
        self.cache = true;
        self
    }

    pub fn with_workspace(mut self, workspace: Workspace) -> Self {
        self.workspace = workspace;
        self
    }

    pub fn with_backend<T>(mut self, backend: T) -> Self
    where
        T: Into<Backend>,
    {
        self.backend = backend.into();
        self
    }

    async fn try_fetch_page(&self, url: &Url) -> Result<Page> {
        if self.cache {
            if let Some(page) = self.workspace.get_cached_page(url).await? {
                return Ok(page);
            }
        }

        let page = self.backend.fetch_page(url).await?;

        self.workspace
            .cache_page(&page)
            .await
            .into_diagnostic()
            .trace_report()
            .ok();

        Ok(page)
    }

    pub async fn try_fetch(&self, url: &Url) -> Result<Html> {
        let page = self.try_fetch_page(url).await?;

        let mut document = Html::parse_document(&page.html);

        if self.clean {
            document.hide_scripts();
        }

        Ok(document)
    }

    /// Shutdown the backend and cleanup resources
    /// This should be called when the fetcher is no longer needed
    pub async fn shutdown(&mut self) {
        self.backend.shutdown().await;
    }
}
