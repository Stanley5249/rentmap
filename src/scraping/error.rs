use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
pub enum Error {
    #[error("Failed to build website")]
    #[diagnostic(help("{0}"))]
    Website(#[source] spider::website::Website),

    #[error("Failed to retrieve pages from website")]
    PagesRetrieval,

    #[error("No pages found after scraping")]
    NoPages,
}
