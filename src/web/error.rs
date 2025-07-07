use miette::Diagnostic;
use thiserror::Error;

use super::backends::BackendError;

#[derive(Debug, Error, Diagnostic)]
pub enum WebError {
    #[error(transparent)]
    #[diagnostic(transparent)]
    Backend(#[from] BackendError),

    #[error("no pages found")]
    #[diagnostic(
        code(web::no_pages),
        help("verify the website URL is accessible and contains the expected content")
    )]
    NoPages,
}
