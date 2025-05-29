use miette::Diagnostic;
use std::io;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
pub enum Error {
    #[error("API key not found")]
    #[diagnostic(
        help(
            "Set API key via:\n- Environment: $env:GOOGLE_MAPS_API_KEY='your-key'\n- Command line: --api-key your-key\n- Config file: add 'api_key = \"your-key\"' to geocoding.toml"
        ),
        url("https://developers.google.com/maps/documentation/geocoding/get-api-key")
    )]
    ApiKeyNotFound,

    #[error(transparent)]
    #[diagnostic(help("Check your file system permissions and paths"))]
    Io(#[from] io::Error),

    #[error(transparent)]
    #[diagnostic(help("Verify your geocoding.toml file has valid TOML syntax"))]
    Config(#[from] toml::de::Error),

    #[error(transparent)]
    #[diagnostic(transparent)]
    GoogleMaps(#[from] google_maps::Error),
}
