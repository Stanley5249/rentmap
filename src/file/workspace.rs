use std::path::{Path, PathBuf};

use clap::Args;
use serde::Serialize;
use serde::de::DeserializeOwned;
use tracing::debug;

use super::FileError;
use super::ops::{load_json, make_directory, save_html, save_json, url_to_file_name};
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
        let file_name = url_to_file_name(&page.url_final);
        debug!(file_name = %file_name.display(), "url to file name");

        let path = &self.html().join(file_name);
        if let Some(parent) = path.parent() {
            make_directory(&parent)?;
        }

        save_html(&page.html, path)
    }
}
