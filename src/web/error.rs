use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
pub enum Error {
    #[error(transparent)]
    #[diagnostic(transparent)]
    Backend(#[from] crate::web::backends::error::Backend),

    #[error("no pages found")]
    #[diagnostic(
        code(web::no_pages),
        help("verify the website URL is accessible and contains the expected content")
    )]
    NoPages,
}
