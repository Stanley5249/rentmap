use futures::StreamExt;
use miette::{Diagnostic, IntoDiagnostic};
use spider_chrome::browser::{Browser, BrowserConfig};
use spider_chrome::error::CdpError;
use spider_chrome::handler::viewport::Viewport;
use thiserror::Error;
use tokio::sync::OnceCell;
use tokio::task::JoinHandle;
use url::{ParseError, Url};

use crate::error::TraceReport;
use crate::web::Page;

#[derive(Debug, Error, Diagnostic)]
pub enum SpiderChromeError {
    #[error("browser configuration failed: {0}")]
    #[diagnostic(
        code(web::backends::spider_chrome::config),
        help("check Chrome/Chromium installation and browser configuration parameters")
    )]
    Config(String),

    #[error(transparent)]
    #[diagnostic(
        code(web::backends::spider_chrome::cdp),
        help(
            "Chrome browser may have crashed or become unresponsive - try restarting the application"
        )
    )]
    Cdp(#[from] CdpError),

    #[error(transparent)]
    #[diagnostic(
        code(web::backends::spider_chrome::url_parse),
        help("ensure the URL is properly formatted (e.g., https://example.com)")
    )]
    UrlParse(#[from] ParseError),

    #[error("page URL not available")]
    #[diagnostic(
        code(web::backends::spider_chrome::no_page_url),
        help("browser page did not provide a valid URL - this may indicate a navigation failure")
    )]
    NoPageUrl,

    #[error(transparent)]
    #[diagnostic(
        code(web::backends::spider_chrome::io),
        help("check file permissions and available disk space")
    )]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    #[diagnostic(
        code(web::backends::spider_chrome::task_join),
        help("browser handler task panicked or was cancelled - this indicates an internal error")
    )]
    Join(#[from] tokio::task::JoinError),
}
/// Browser session that manages the Chrome instance and message handler.
struct Session {
    browser: Browser,
    join_handle: JoinHandle<()>,
}

impl Session {
    async fn new(config: Option<BrowserConfig>) -> Result<Self, SpiderChromeError> {
        let config = match config {
            Some(config) => config,
            None => Self::default_config()?,
        };

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

    fn default_config() -> Result<BrowserConfig, SpiderChromeError> {
        BrowserConfig::builder()
            .viewport(Some(Viewport {
                width: 1280,
                height: 720,
                device_scale_factor: None,
                emulating_mobile: false,
                is_landscape: false,
                has_touch: false,
            }))
            .with_head()
            .build()
            .map_err(SpiderChromeError::Config)
    }
}

/// Chrome-based web scraping backend using spider_chrome.
#[derive(Default)]
pub struct SpiderChromeBackend {
    session: OnceCell<Session>,
}

impl SpiderChromeBackend {
    pub fn new() -> Self {
        Self::default()
    }

    async fn session(&self) -> Result<&Session, SpiderChromeError> {
        self.session
            .get_or_try_init(async || Session::new(None).await)
            .await
    }

    pub async fn fetch_page(&self, url: &Url) -> Result<Page, SpiderChromeError> {
        let inner = self.session().await?;

        let page = inner.browser.new_page(url.as_str()).await?;

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
    pub async fn shutdown(&mut self) -> Result<(), SpiderChromeError> {
        if let Some(mut inner) = self.session.take() {
            inner.browser.close().await?;
            inner.browser.wait().await?;
            inner.join_handle.await?;
        };

        Ok(())
    }
}
