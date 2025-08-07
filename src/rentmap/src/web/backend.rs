use clap::ValueEnum;
use tracing::error;
use url::Url;

use super::backends::SpiderChromeBackend;
use super::{Page, WebError};

#[derive(Clone, Debug, Default, ValueEnum)]
pub enum BackendType {
    #[default]
    #[value(name = "spider-chrome")]
    SpiderChrome,
}

/// Backend instance that owns the actual backend implementations
/// This allows each backend to maintain its own state (like browser instances)
#[must_use = "backends hold resources that must be shut down with `shutdown()`"]
pub enum Backend {
    SpiderChrome(Box<SpiderChromeBackend>),
}

impl Backend {
    /// Create a default backend (spider-chrome)
    pub async fn default() -> Result<Self, WebError> {
        Ok(SpiderChromeBackend::default().await?.into())
    }
    /// Fetch a page using this backend instance
    pub async fn fetch_page(&self, url: &Url) -> Result<Page, WebError> {
        match self {
            Self::SpiderChrome(backend) => Ok(backend.fetch_page(url).await?),
        }
    }

    /// Shutdown the backend and cleanup resources
    pub async fn shutdown(self) {
        match self {
            Self::SpiderChrome(backend) => {
                if let Err(e) = backend.shutdown().await {
                    error!(?e, "failed to shutdown spider-chrome backend");
                }
            }
        }
    }
}
