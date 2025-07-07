use std::time::Duration;

use spider::configuration::{Configuration, WaitForDelay, WaitForIdleNetwork, WaitForSelector};
use spider::features::chrome_common::RequestInterceptConfiguration;
use spider::website::Website;
use url::Url;

use super::BackendError;
use crate::web::Page;
use crate::web::WebError;

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

fn build_website(url: &Url) -> Result<Box<Website>, BackendError> {
    let config = build_config();

    let website = Website::new(url.as_str())
        .with_config(config)
        .build()
        .map_err(BackendError::spider)?;

    Ok(Box::new(website))
}

pub async fn fetch_page(url: &Url) -> Result<Page, WebError> {
    let mut website = build_website(url)?;

    Box::pin(website.scrape()).await;

    Ok(website
        .get_pages()
        .and_then(|pages| pages.first())
        .ok_or(WebError::NoPages)?
        .into())
}
