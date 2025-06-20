use std::path::{Path, PathBuf};

use clap::Args;
use serde::Serialize;
use serde::de::DeserializeOwned;
use url::Url;

use super::FileError;
use super::ops::{load_json, make_directory, save_json, save_page};
use crate::web::page::Page;

#[derive(Debug, Args)]
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

    pub fn ensure(&self) -> Result<(), FileError> {
        make_directory(&self.root)
    }

    pub fn save_data_json<P, V>(&self, file_name: &P, value: &V) -> Result<(), FileError>
    where
        P: AsRef<Path>,
        V: Serialize,
    {
        let dir = &self.data();
        make_directory(dir)?;
        let path = &dir.join(file_name);
        save_json(value, path)
    }

    pub fn load_data_json<P, T>(&self, file_name: &P) -> Result<T, FileError>
    where
        P: AsRef<Path>,
        T: DeserializeOwned,
    {
        let path = &self.data().join(file_name);
        load_json(path)
    }

    pub fn save_html_page(&self, page: &Page) -> Result<(), FileError> {
        let dir = &self.html();
        make_directory(dir)?;
        let file_name = url_to_file_name(&page.url_final);
        let path = &dir.join(file_name);
        save_page(page, path)
    }
}

fn url_to_file_name(url: &Url) -> String {
    const INVALID_CHARS: [char; 9] = ['/', '\\', ':', '*', '?', '"', '<', '>', '|'];
    let mut components = Vec::new();
    if let Some(host) = url.host_str() {
        components.push(host);
    }
    if let Some(segment) = url.path_segments() {
        components.extend(segment);
    }
    if let Some(query) = url.query() {
        components.push(query);
    }
    let mut file_name = components
        .into_iter()
        .map(|x| x.replace(INVALID_CHARS, "_"))
        .collect::<Vec<_>>()
        .join("_");
    if !file_name.ends_with(".html") {
        file_name.push_str(".html");
    }
    file_name
}
