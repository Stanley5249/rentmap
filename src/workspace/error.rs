use miette::Diagnostic;
use thiserror::Error;

use crate::file::FileError;

#[derive(Debug, Error, Diagnostic)]
pub enum WorkspaceError {
    #[error(transparent)]
    #[diagnostic(transparent)]
    File(#[from] FileError),

    #[error(transparent)]
    #[diagnostic(code(file::database_error))]
    Sqlx(#[from] sqlx::Error),

    #[error(transparent)]
    #[diagnostic(code(file::migration_error))]
    Migration(#[from] sqlx::migrate::MigrateError),
}
