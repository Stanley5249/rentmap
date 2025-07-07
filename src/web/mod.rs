mod backends;
mod error;
mod fetcher;
mod page;

pub use backends::{BackendError, BackendType};
pub use error::WebError;
pub use fetcher::Fetcher;
pub use page::Page;
