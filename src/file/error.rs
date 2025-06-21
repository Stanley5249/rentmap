use std::path::{Path, PathBuf};

use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
#[error("file operation failed at `{path}`")]
#[diagnostic(code(file::path_error))]
pub struct PathError {
    path: PathBuf,
    #[source]
    source: std::io::Error,
}

impl PathError {
    pub fn new<P>(path: P, source: std::io::Error) -> Self
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref().to_path_buf();
        Self { path, source }
    }
}

#[derive(Debug, Error, Diagnostic)]
pub enum FileError {
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),

    #[error(transparent)]
    SerdeToml(#[from] toml::de::Error),

    #[error(transparent)]
    #[diagnostic(transparent)]
    Path(#[from] PathError),

    #[error("file not found")]
    #[diagnostic(code(file::not_found))]
    NotFound,
}
