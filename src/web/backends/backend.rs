use url::Url;

use crate::web::{Page, WebError};

#[derive(Default, Clone, Copy)]
pub enum BackendType {
    #[default]
    Spider,
}

impl BackendType {
    /// Fetch a page using the selected backend
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
            Self::Spider => super::spider::fetch_page(url).await,
        }
    }
}
