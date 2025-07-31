use std::fmt::Debug;
use std::time::Duration;

use clap::Args;
use futures::StreamExt;
use miette::{Diagnostic, IntoDiagnostic};
use spider_chrome::Page as ChromiumPage;
use spider_chrome::browser::{Browser, BrowserConfig};
use spider_chrome::cdp::IntoEventKind;
use spider_chrome::cdp::browser_protocol::network::EventLoadingFinished;
use spider_chrome::error::CdpError;
use spider_chrome::handler::viewport::Viewport;
use thiserror::Error;
use tokio::task::JoinHandle;
use tracing::trace;
use url::{ParseError, Url};

use crate::error::TraceReport;
use crate::web::{Backend, Page};

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

#[derive(Debug, Default, Args)]
#[command(next_help_heading = "Spider Chrome")]
pub struct SpiderChromeArgs {
    /// Run browser in head mode (non-headless)
    #[arg(long)]
    pub head: bool,
}

impl TryFrom<SpiderChromeArgs> for BrowserConfig {
    type Error = SpiderChromeError;

    fn try_from(args: SpiderChromeArgs) -> Result<Self, Self::Error> {
        let mut config = BrowserConfig::builder()
            .viewport(Some(Viewport {
                width: 1920,
                height: 1080,
                ..Default::default()
            }))
            .enable_request_intercept();

        if args.head {
            config = config.with_head();
        }

        let mut browser_config = config.build().map_err(SpiderChromeError::Config)?;

        browser_config.ignore_ads = true;
        browser_config.ignore_analytics = true;
        browser_config.ignore_javascript = false;
        browser_config.ignore_stylesheets = false;
        browser_config.ignore_visuals = false;

        Ok(browser_config)
    }
}

/// Chrome-based web scraping backend using spider_chrome.
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

#[allow(dead_code)]
async fn wait_for_event<T>(page: &ChromiumPage, timeout_duration: Duration) -> Result<(), CdpError>
where
    T: Debug + Unpin + IntoEventKind,
{
    let mut events = page.event_listener::<T>().await?;

    let future = async {
        match events.next().await {
            Some(event) => trace!(?event, "received event"),
            None => trace!("event stream closed before receiving event"),
        };
    };

    if (tokio::time::timeout(timeout_duration, future).await).is_err() {
        trace!("timed out waiting for event");
    }

    Ok(())
}

/// Wait for network to be idle (no network events for `network_idle_duration`) or timeout after `timeout_duration`.
async fn wait_for_network_idle(
    page: &ChromiumPage,
    idle_duration: Duration,
    timeout_duration: Duration,
) -> Result<(), CdpError> {
    let mut events = page.event_listener::<EventLoadingFinished>().await?;

    let future = async {
        loop {
            tokio::select! {
                _ = tokio::time::sleep(idle_duration) => {
                    trace!(duration = ?idle_duration, "network idle");
                    break;
                },
                Some(event) = events.next() => {
                    trace!(?event, "received network event");
                },
                else => {
                    trace!("event stream closed before network idle");
                    break;
                },
            }
        }
    };

    if (tokio::time::timeout(timeout_duration, future).await).is_err() {
        trace!("timed out waiting for network idle");
    }

    Ok(())
}
