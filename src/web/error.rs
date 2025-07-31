use miette::Diagnostic;
use thiserror::Error;

use super::backends::SpiderChromeError;

#[derive(Debug, Error, Diagnostic)]
pub enum WebError {
    #[error("SpiderChrome backend error")]
    #[diagnostic(transparent)]
    SpiderChrome(#[from] SpiderChromeError),
}
