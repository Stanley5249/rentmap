use super::error::ApiKeyNotFound;
use clap::Parser;
use serde::Deserialize;
use std::fmt::{self, Debug};

/// Google API key arguments (for Maps, Vision, etc.)
#[derive(Debug, Deserialize, Parser)]
pub struct GoogleConfig {
    /// The Google API key to use for requests
    #[arg(long, env = "GOOGLE_API_KEY")]
    pub api_key: Option<SecretString>,
}

impl GoogleConfig {
    /// Returns the API key as a string, or an error if it's missing
    pub fn get_api_key(self) -> Result<String, ApiKeyNotFound> {
        self.api_key.map(Into::into).ok_or(ApiKeyNotFound)
    }
}

/// A wrapper for secret strings that masks the value in debug output
#[derive(Clone, Deserialize)]
pub struct SecretString(String);

impl SecretString {
    pub fn new(value: String) -> Self {
        Self(value)
    }
}

impl Debug for SecretString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<hidden>")
    }
}

impl From<String> for SecretString {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<SecretString> for String {
    fn from(val: SecretString) -> Self {
        val.0
    }
}
