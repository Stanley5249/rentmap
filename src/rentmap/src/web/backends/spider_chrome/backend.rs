use std::time::Duration;

use futures::StreamExt;
use miette::IntoDiagnostic;
use spider_chrome::browser::{Browser, BrowserConfig};
use tokio::task::JoinHandle;
use url::Url;

use super::args::SpiderChromeArgs;
use super::error::SpiderChromeError;
use super::utils::wait_for_network_idle;
use crate::error::TraceReport;
use crate::web::{Backend, Page};

/// Chrome-based web scraping backend using spider_chrome.
#[must_use = "SpiderChromeBackend holds browser resources that must be shut down with `shutdown()`"]
pub struct SpiderChromeBackend {
    browser: Browser,
    join_handle: JoinHandle<()>,
}

impl SpiderChromeBackend {
    pub async fn new(config: BrowserConfig) -> Result<Self, SpiderChromeError> {
        let (browser, mut handler) = Browser::launch(config).await?;

        let join_handle = tokio::spawn(async move {
            while let Some(result) = handler.next().await {
                result.into_diagnostic().trace_report().ok();
            }
        });

        Ok(Self {
            browser,
            join_handle,
        })
    }

    pub async fn default() -> Result<Self, SpiderChromeError> {
        let config = SpiderChromeArgs::default()
            .try_into()
            .expect("spider-chrome default config should always be valid");

        Self::new(config).await
    }

    pub async fn fetch_page(&self, url: &Url) -> Result<Page, SpiderChromeError> {
        let page = self.browser.new_page(url.as_str()).await?;

        let max_wait_duration = Duration::from_secs(20);
        let network_idle_duration = Duration::from_millis(500);

        wait_for_network_idle(&page, network_idle_duration, max_wait_duration).await?;

        let final_url: Url = page
            .url()
            .await?
            .ok_or(SpiderChromeError::NoPageUrl)?
            .parse()?;

        let html = page.content().await?;

        page.close().await?;

        Ok(Page::new(final_url, html))
    }

    /// Gracefully shutdown the browser and cleanup resources.
    pub async fn shutdown(mut self) -> Result<(), SpiderChromeError> {
        self.browser.close().await?;
        self.browser.wait().await?;
        self.join_handle.await?;
        Ok(())
    }
}

impl From<SpiderChromeBackend> for Backend {
    fn from(backend: SpiderChromeBackend) -> Self {
        Self::SpiderChrome(Box::new(backend))
    }
}
