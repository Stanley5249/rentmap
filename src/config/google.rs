use super::error::ApiKeyNotFound;
use clap::Args;
use serde::Deserialize;
use std::fmt::{self, Debug, Formatter};
use std::ops::Deref;

/// Google API configuration for cloud services
#[derive(Debug, Deserialize, Args)]
pub struct GoogleConfig {
    /// Google API key for cloud services
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
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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

impl Deref for SecretString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
