use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
pub enum BackendError {
    #[error("failed to build spider website")]
    #[diagnostic(
        code(web::backend::spider),
        help(
            "spider library has poor error handling - check if the URL is valid and accessible, or contact the author if this persists"
        )
    )]
    Spider(#[source] Box<spider::website::Website>),
}

impl BackendError {
    pub fn spider(website: spider::website::Website) -> Self {
        Self::Spider(Box::new(website))
    }
}
