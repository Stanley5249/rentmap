use std::time::Duration;

use miette::Diagnostic;
use spider::configuration::{Configuration, WaitForDelay, WaitForIdleNetwork, WaitForSelector};
use spider::features::chrome_common::RequestInterceptConfiguration;
use spider::website::Website;
use thiserror::Error;
use url::Url;

use crate::web::Page;

#[derive(Debug, Error, Diagnostic)]
pub enum SpiderError {
    #[error("failed to build spider website")]
    #[diagnostic(
        code(web::backend::spider::build),
        help(
            "spider library has poor error handling - check if the URL is valid and accessible, or contact the author if this persists"
        )
    )]
    Build(#[source] Box<Website>),

    #[error("no pages found")]
    #[diagnostic(
        code(web::backend::spider::no_pages),
        help(
            "the spider backend returned no content

common causes:
- invalid or inaccessible website URL
- network connectivity issues or website is down
- website requires authentication or blocks automated access
- the website's structure has changed

troubleshooting steps:
1. verify the URL is correct and accessible in a regular browser
2. check your internet connection
3. try using the spider-chrome backend instead: --backend spider-chrome
4. check if the website requires authentication"
        )
    )]
    NoPages,
}

impl SpiderError {
    pub fn build(website: Website) -> Self {
        Self::Build(Box::new(website))
    }
}

fn build_config() -> Configuration {
    let intercept_config = RequestInterceptConfiguration::new(true);

    let timeout = Some(Duration::from_secs(30));
    let selector = "body".to_string();
    let wait_for_idle_dom = Some(WaitForSelector::new(timeout, selector));

    let wait_for_idle_network = Some(WaitForIdleNetwork::new(timeout));

    let delay: Option<Duration> = Some(Duration::from_millis(500));
    let wait_for_delay = Some(WaitForDelay::new(delay));

    Configuration::new()
        .with_chrome_intercept(intercept_config, &None)
        .with_wait_for_idle_dom(wait_for_idle_dom)
        .with_wait_for_idle_network(wait_for_idle_network)
        .with_wait_for_delay(wait_for_delay)
        .with_stealth(true)
        .with_caching(true)
        .with_limit(1)
        .with_retry(1)
        .build()
}

fn build_website(url: &Url) -> Result<Box<Website>, SpiderError> {
    let config = build_config();

    let website = Website::new(url.as_str())
        .with_config(config)
        .build()
        .map_err(SpiderError::build)?;

    Ok(Box::new(website))
}

/// Fetch a page using Spider backend
///
/// Note: This operation may require deep stack recursion for large DOM trees.
/// Consider using `tokio::spawn()` when calling this function to prevent stack overflow:
///
/// ```rust
/// let handle = tokio::spawn(spider::fetch_page(&url));
/// let page = handle.await??;
/// ```
pub async fn fetch_page(url: &Url) -> Result<Page, SpiderError> {
    let mut website = build_website(url)?;

    Box::pin(website.scrape()).await;

    website
        .get_pages()
        .and_then(|pages| pages.first())
        .map(|page| page.into())
        .ok_or(SpiderError::NoPages)
}
