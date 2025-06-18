use std::fmt::Debug;
use std::fs;
use std::path::{Path, PathBuf};

use bytes::Bytes;
use miette::Diagnostic;
use serde::Serialize;
use serde::de::DeserializeOwned;
use thiserror::Error;
use tracing::{debug, info};
use url::Url;

use crate::web::page::Page;

// macro to generate FileError constructor methods
macro_rules! file_error_ctors {
    { $( $fn_name:ident => $variant:ident ),* $(,)? } => {
        $(
            pub fn $fn_name<P>(path: P, source: std::io::Error) -> Self
            where P: AsRef<Path> {
                Self::$variant(PathError::new(path, source))
            }
        )*
    };
}

#[derive(Debug, Error, Diagnostic)]
#[error("'{path}': {source}")]
pub struct PathError {
    path: PathBuf,
    #[source]
    source: std::io::Error,
}

impl PathError {
    pub fn new<P>(path: P, source: std::io::Error) -> Self
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref().to_path_buf();
        Self { path, source }
    }
}

#[derive(Debug, Error, Diagnostic)]
pub enum FileError {
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),

    #[error(transparent)]
    SerdeToml(#[from] toml::de::Error),

    #[error("failed to make directory at {0}")]
    MakeDirectory(PathError),

    #[error("failed to save file at {0}")]
    SaveFile(PathError),

    #[error("failed to load file at {0}")]
    LoadFile(PathError),
}

impl FileError {
    file_error_ctors! {
        make_directory => MakeDirectory,
        save_file => SaveFile,
        load_file => LoadFile,
    }
}

pub fn make_directory<P>(path: &P) -> Result<(), FileError>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();

    if !path.exists() {
        fs::create_dir_all(path).map_err(|source| FileError::make_directory(path, source))?;

        info!(
            path = %path.display(),
            "make directory"
        );
    }

    debug!(
        path = %path.display(),
        "directory already exists"
    );

    Ok(())
}

pub fn save_json<V, F, P>(value: &V, file_name: &F, output_dir: &P) -> Result<(), FileError>
where
    V: Serialize,
    F: AsRef<Path>,
    P: AsRef<Path>,
{
    let path = output_dir.as_ref().join(file_name);
    let json = serde_json::to_string(value).map_err(FileError::SerdeJson)?;

    fs::write(&path, &json).map_err(|source| FileError::save_file(&path, source))?;

    info!(
        path = %path.display(),
        length = json.len(),
        "save JSON file"
    );

    Ok(())
}

pub fn save_page<P>(page: &Page, output_dir: &P) -> Result<(), FileError>
where
    P: AsRef<Path>,
{
    let name = url_to_file_name(&page.url_final);
    let path = output_dir.as_ref().join(name);

    fs::write(&path, &page.html).map_err(|source| FileError::save_file(&path, source))?;

    info!(
        path = %path.display(),
        length = page.html.len(),
        "save HTML file"
    );

    Ok(())
}

pub fn load_toml<P, T>(path: &P) -> Result<T, FileError>
where
    P: AsRef<Path>,
    T: Debug + DeserializeOwned,
{
    let path = path.as_ref();

    let content = fs::read_to_string(path).map_err(|source| FileError::load_file(path, source))?;

    let value: T = toml::from_str(&content)?;

    info!(
        path = %path.display(),
        length = content.len(),
        "load TOML file"
    );

    Ok(value)
}

pub fn load_image<P>(path: &P) -> Result<Bytes, FileError>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();

    let bytes: Bytes = fs::read(path)
        .map_err(|source| FileError::save_file(path, source))?
        .into();

    info!(
        path = %path.display(),
        length = bytes.len(),
        "load image file"
    );

    Ok(bytes)
}

/// Generate safe filename from URL path segments
pub fn url_to_file_name(url: &Url) -> String {
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

    let mut filename = components
        .into_iter()
        .map(|x| x.replace(INVALID_CHARS, "_"))
        .collect::<Vec<_>>()
        .join("_");

    if !filename.ends_with(".html") {
        filename.push_str(".html");
    }

    filename
}
