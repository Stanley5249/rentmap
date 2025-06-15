use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
pub enum Error {
    #[error(transparent)]
    GoogleCloud(#[from] google_cloud_vision_v1::Error),

    #[error("No text annotation found in response")]
    TextAnnotationMissing,

    #[error("Number of images ({images}) and responses ({responses}) do not match")]
    ImageResponseCountMismatch { images: usize, responses: usize },
}
