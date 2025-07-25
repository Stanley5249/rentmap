use miette::{IntoDiagnostic, Result, WrapErr};
use scraper::Html;
use url::Url;

use super::backends::BackendType;
use crate::error::TraceReport;
use crate::file::Workspace;
use crate::scraper::HtmlExt;
use crate::web::Page;

pub struct Fetcher {
    pub cache: bool,
    pub clean: bool,
    pub workspace: Workspace,
    backend: BackendType,
}

impl Fetcher {
    pub fn new(workspace: Workspace) -> Self {
        Self {
            cache: false,
            clean: false,
            workspace,
            backend: BackendType::default(),
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

    async fn try_fetch_page(&self, url: &Url) -> Result<Page> {
        if self.cache {
            if let Some(page) = self.workspace.get_cached_page(url).await? {
                return Ok(page);
            }
        }

        let future = {
            let backend = self.backend;
            let url = url.clone();
            async move { backend.fetch_page(&url).await }
        };

        let page = tokio::spawn(future)
            .await
            .into_diagnostic()
            .wrap_err("fetch task was cancelled or panicked")??;

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
}
