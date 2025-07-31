mod args;
mod backend;
mod backends;
mod error;
mod fetcher;
mod page;

pub use args::FetcherArgs;
pub use backend::{Backend, BackendType};
pub use backends::{SpiderChromeArgs, SpiderChromeBackend, SpiderChromeError, SpiderError};
pub use error::WebError;
pub use fetcher::Fetcher;
pub use page::Page;
