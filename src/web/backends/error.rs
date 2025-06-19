use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
pub enum Backend {
    #[error("failed to build spider website")]
    #[diagnostic(
        code(web::backend::spider),
        help(
            "spider library has poor error handling - check if the URL is valid and accessible, or contact the author if this persists"
        )
    )]
    Spider(#[source] spider::website::Website),
}
