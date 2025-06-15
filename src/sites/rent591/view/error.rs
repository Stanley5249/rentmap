use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
pub enum Error {
    #[error("Could not extract page links")]
    NoPageLinks,

    #[error("Could not extract max page number text")]
    NoPageText,

    #[error("Could not parse max page number")]
    ParseInt(#[from] std::num::ParseIntError),
}
