use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
#[error("api key not found")]
#[diagnostic(
    code(config::no_api_key),
    help(
        "set GOOGLE_API_KEY environment variable, use --api-key flag, or add 'api_key = \"your-key\"' to rentmap.toml"
    ),
    url("https://github.com/Stanley5249/rentmap#setup-your-google-api-key")
)]
pub struct NoApiKey;
