use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;

use chrono::{DateTime, Utc};
use clap::Args;
use serde::Serialize;
use serde::de::DeserializeOwned;
use tracing::debug;

use super::FileError;
use super::ops::{find_latest_file, load_json, make_directory, save_html, save_json};
use super::url::url_to_file_name;
use crate::web::page::Page;

#[derive(Clone, Debug, Args)]
pub struct Workspace {
    /// The root directory of the workspace
    #[arg(long = "workspace", short = 'w', default_value = ".rentmap")]
    pub root: PathBuf,
}

impl Workspace {
    pub fn data(&self) -> PathBuf {
        self.root.join("data")
    }

    pub fn html(&self) -> PathBuf {
        self.root.join("html")
    }

    pub fn init(&self) -> Result<(), FileError> {
        make_directory(&self.root)
    }

    pub fn save_page(&self, page: &Page) -> Result<(), FileError> {
        let file_name = url_to_file_name(&page.url_final);
        debug!(file_name = %file_name.display(), "url to file name");

        let path = &self.html().join(file_name);
        if let Some(parent) = path.parent() {
            make_directory(parent)?;
        }

        save_html(&page.html, path)
    }

    pub fn save_data_now<V, K>(&self, data: &V, key: &K) -> Result<DateTime<Utc>, FileError>
    where
        V: Serialize,
        K: Hash,
    {
        let now = Utc::now();
        self.save_data_at(data, key, &now)?;
        Ok(now)
    }

    pub fn save_data_at<V, K>(
        &self,
        data: &V,
        key: &K,
        timestamp: &DateTime<Utc>,
    ) -> Result<(), FileError>
    where
        V: Serialize,
        K: Hash,
    {
        let timestamp_str = timestamp.format("%Y-%m-%dT%H-%M-%S").to_string();

        let mut path = self.data();
        path.push(hash(key));

        make_directory(&path)?;
        // if success, update metadata

        path.push(timestamp_str);
        path.set_extension("json");
        save_json(data, &path)
    }

    pub fn load_data_latest<T, K>(&self, key: &K) -> Result<(DateTime<Utc>, T), FileError>
    where
        T: DeserializeOwned,
        K: Hash,
    {
        let mut dir = self.data();
        dir.push(hash(key));
        find_latest_file(&dir, "json").and_then(|(ts, path)| load_json(path).map(|data| (ts, data)))
    }

    pub fn load_data_at<T, K>(&self, key: &K, timestamp: DateTime<Utc>) -> Result<T, FileError>
    where
        T: DeserializeOwned,
        K: Hash,
    {
        let mut path = self.data();
        path.push(hash(key));
        path.push(timestamp.format("%Y-%m-%dT%H-%M-%S").to_string());
        path.set_extension("json");
        load_json(path)
    }

    // TODO: Add metadata persistence for hash -> URL mapping
}

fn hash<T: Hash>(value: &T) -> String {
    let mut state = DefaultHasher::new();
    value.hash(&mut state);
    format!("{:08x}", state.finish())
}
