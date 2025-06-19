use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
pub enum Error {
    #[error(transparent)]
    #[diagnostic(
        code(apis::vision::google_cloud_vision_v1_error),
        help("check your Google Cloud Vision API key is valid and has the necessary permissions")
    )]
    GoogleCloud(#[from] google_cloud_vision_v1::Error),

    #[error("no text annotation found in response")]
    #[diagnostic(
        code(apis::vision::no_text),
        help("ensure the image contains readable text or try a different image")
    )]
    NoText,

    #[error("image count ({images}) and response count ({responses}) do not match")]
    #[diagnostic(
        code(apis::vision::count_mismatch),
        help("this is likely an internal error - please contact the author if this persists")
    )]
    CountMismatch { images: usize, responses: usize },
}
