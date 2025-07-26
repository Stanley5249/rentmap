use miette::Diagnostic;
use thiserror::Error;

use super::backends::{SpiderChromeError, SpiderError};

#[derive(Debug, Error, Diagnostic)]
pub enum WebError {
    #[error("Spider backend error")]
    #[diagnostic(transparent)]
    Spider(#[from] SpiderError),

    #[error("SpiderChrome backend error")]
    #[diagnostic(transparent)]
    SpiderChrome(#[from] SpiderChromeError),
}
