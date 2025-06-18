use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
pub enum Error {
    #[error("Backend error: {0}")]
    Backend(#[from] crate::web::backends::error::Backend),

    #[error("Failed to get pages from website")]
    NoPages,
}
