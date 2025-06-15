use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
#[error("API key is missing")]
#[diagnostic(
    code(cli::api_key_missing),
    help(
        "Provide your API key via environment variable GOOGLE_MAPS_API_KEY, command line --api-key flag, or config file with 'api_key = \"your-key\"' in rentmap.toml"
    ),
    url("https://developers.google.com/maps/documentation/geocoding/get-api-key")
)]
pub struct ApiKeyMissingError;
