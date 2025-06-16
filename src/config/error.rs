use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
#[error("API key not found")]
#[diagnostic(
    code(config::api_key_not_found),
    help(
        "Provide your Google API key via environment variable GOOGLE_API_KEY, command line --api-key flag, or config file with 'api_key = \"your-key\"' in rentmap.toml. Make sure to enable both Maps API and Vision API in Google Cloud Platform."
    ),
    url("https://developers.google.com/maps/documentation/geocoding/get-api-key")
)]
pub struct ApiKeyNotFound;
