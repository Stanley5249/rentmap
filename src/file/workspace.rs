use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::{PathBuf, absolute};

use chrono::{DateTime, Utc};
use clap::Args;
use serde::Serialize;
use serde::de::DeserializeOwned;
use tracing::debug;
use url::Url;

use super::FileError;
use super::metadata::Metadata;
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

    pub fn save_data<T>(&self, data: &T, url: Url) -> Result<DateTime<Utc>, FileError>
    where
        T: Serialize,
    {
        let now = Utc::now();
        self.save_data_at(data, url, now)?;
        Ok(now)
    }

    pub fn save_data_at<T>(
        &self,
        data: &T,
        url: Url,
        timestamp: DateTime<Utc>,
    ) -> Result<(), FileError>
    where
        T: Serialize,
    {
        let file_name = timestamp.format("%Y-%m-%dT%H-%M-%S").to_string();

        let mut path = self.data();
        path.push(hash(&url));

        make_directory(&path)?;

        path.push(file_name);
        path.set_extension("json");
        save_json(data, &path)?;

        self.update_metadata(url, path, timestamp)?;

        Ok(())
    }

    pub fn load_data<T>(&self, url: &Url) -> Result<(T, DateTime<Utc>), FileError>
    where
        T: DeserializeOwned,
    {
        let mut dir = self.data();
        dir.push(hash(url));
        find_latest_file(&dir, "json").and_then(|(ts, path)| load_json(path).map(|data| (data, ts)))
    }

    pub fn load_data_at<T>(&self, url: &Url, timestamp: DateTime<Utc>) -> Result<T, FileError>
    where
        T: DeserializeOwned,
    {
        let mut path = self.data();
        path.push(hash(url));
        path.push(timestamp.format("%Y-%m-%dT%H-%M-%S").to_string());
        path.set_extension("json");
        load_json(path)
    }

    fn update_metadata(
        &self,
        url: Url,
        path: PathBuf,
        timestamp: DateTime<Utc>,
    ) -> Result<Metadata, FileError> {
        let metadata_path = self.data().join("metadata.json");

        let mut metadata_list: Vec<Metadata> = load_json(&metadata_path).unwrap_or_default();

        let index = match metadata_list.iter().position(|item| item.url == url) {
            Some(i) => {
                metadata_list[i].updated_at = Some(timestamp);
                i
            }
            None => {
                let i = metadata_list.len();
                let path = absolute(&path).unwrap_or(path);
                let metadata = Metadata {
                    url,
                    path,
                    created_at: timestamp,
                    updated_at: None,
                };
                metadata_list.push(metadata);
                i
            }
        };

        save_json(&metadata_list, &metadata_path)?;

        Ok(metadata_list.swap_remove(index))
    }
}

fn hash<T: Hash>(value: &T) -> String {
    let mut state = DefaultHasher::new();
    value.hash(&mut state);
    format!("{:08x}", state.finish())
}
