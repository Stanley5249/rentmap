use std::path::PathBuf;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub url: Url,
    pub path: PathBuf,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
