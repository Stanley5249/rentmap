use crate::scraping::page::Page;
use spider::configuration::{Configuration, WaitForDelay, WaitForIdleNetwork, WaitForSelector};
use spider::features::chrome_common::RequestInterceptConfiguration;
use spider::website::Website;
use std::time::Duration;
use url::Url;

fn build_config() -> Configuration {
    let intercept_config = RequestInterceptConfiguration::new(true);

    let timeout = Some(Duration::from_secs(30));
    let selector = "body".to_string();
    let wait_for_idle_dom = Some(WaitForSelector::new(timeout, selector));

    let wait_for_idle_network = Some(WaitForIdleNetwork::new(timeout));

    let delay: Option<Duration> = Some(Duration::from_millis(500));
    let wait_for_delay = Some(WaitForDelay::new(delay));

    let config = Configuration::new()
        .with_chrome_intercept(intercept_config, &None)
        .with_wait_for_idle_dom(wait_for_idle_dom)
        .with_wait_for_idle_network(wait_for_idle_network)
        .with_wait_for_delay(wait_for_delay)
        .with_stealth(true)
        .with_caching(true)
        .with_limit(1)
        .build();

    config
}

fn build_website(url: &Url) -> Result<Box<Website>, super::error::Error> {
    let config = build_config();

    let website = Website::new(url.as_str())
        .with_config(config)
        .build()
        .map_err(super::error::Error::Website)?;

    Ok(Box::new(website))
}

pub async fn fetch_page(url: &Url) -> Result<Page, super::error::Error> {
    let mut website = build_website(url)?;

    Box::pin(website.scrape()).await;

    let page = website
        .get_pages()
        .ok_or(super::error::Error::PagesRetrieval)?
        .first()
        .ok_or(super::error::Error::NoPages)?
        .into();

    Ok(page)
}
