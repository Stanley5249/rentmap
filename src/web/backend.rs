use clap::ValueEnum;
use tracing::error;
use url::Url;

use super::backends::SpiderChromeBackend;
use super::{Page, WebError};

#[derive(Clone, Debug, Default, ValueEnum)]
pub enum BackendType {
    #[value(name = "spider")]
    Spider,

    #[default]
    #[value(name = "spider-chrome")]
    SpiderChrome,
}

/// Backend instance that owns the actual backend implementations
/// This allows each backend to maintain its own state (like browser instances)
pub enum Backend {
    Spider,
    SpiderChrome(Box<SpiderChromeBackend>),
}

impl Backend {
    /// Fetch a page using this backend instance
    ///
    /// # Stack Overflow Warning
    /// The Spider backend may cause stack overflow for large DOM trees.
    /// Use `tokio::spawn()` when using Spider backend:
    ///
    /// ```rust
    /// let handle = tokio::spawn(backend.fetch_page(&url));
    /// let page = handle.await??;
    /// ```
    pub async fn fetch_page(&self, url: &Url) -> Result<Page, WebError> {
        match self {
            Self::Spider => super::backends::spider_fetch_page(url)
                .await
                .map_err(WebError::Spider),

            Self::SpiderChrome(backend) => backend
                .fetch_page(url)
                .await
                .map_err(WebError::SpiderChrome),
        }
    }

    /// Shutdown the backend and cleanup resources
    pub async fn shutdown(&mut self) {
        match self {
            Self::Spider => {
                // Spider backend is stateless, no cleanup needed
            }
            Self::SpiderChrome(backend) => {
                if let Err(e) = backend.shutdown().await {
                    error!("Failed to shutdown SpiderChrome backend: {}", e);
                }
            }
        }
    }
}

impl From<BackendType> for Backend {
    fn from(backend_type: BackendType) -> Self {
        match backend_type {
            BackendType::Spider => Self::Spider,
            BackendType::SpiderChrome => Self::SpiderChrome(Default::default()),
        }
    }
}
