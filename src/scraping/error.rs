use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
pub enum Error {
    #[error("Failed to build website: {0}")]
    Website(#[source] spider::website::Website),

    #[error("Failed to get pages from website")]
    NoPages,
}
