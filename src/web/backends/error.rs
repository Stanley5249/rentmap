use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
pub enum Backend {
    #[error("Spider backend error: {0}")]
    Spider(#[source] spider::website::Website),
}
