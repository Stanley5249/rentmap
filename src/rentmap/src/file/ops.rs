use std::fs;
use std::path::Path;

use bytes::Bytes;
use serde::Serialize;
use serde::de::DeserializeOwned;
use tracing::{debug, info};

use super::FileError;
use crate::file::PathError;

pub fn make_directory<P>(path: P) -> Result<(), FileError>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    if !path.exists() {
        fs::create_dir_all(path).map_err(|source| PathError::new(path, source))?;
        info!(path = %path.display(), "make directory");
    }
    debug!(path = %path.display(), "directory exists");
    Ok(())
}

pub fn save_json<V, P>(value: &V, path: P) -> Result<(), FileError>
where
    V: Serialize,
    P: AsRef<Path>,
{
    let path = path.as_ref();
    let json = serde_json::to_string(value).map_err(FileError::SerdeJson)?;
    fs::write(path, &json).map_err(|source| PathError::new(path, source))?;
    info!(path = %path.display(), length = json.len(), "save JSON file");
    Ok(())
}

pub fn load_json<P, T>(path: P) -> Result<T, FileError>
where
    P: AsRef<Path>,
    T: DeserializeOwned,
{
    let path = path.as_ref();
    let content = fs::read_to_string(path).map_err(|source| PathError::new(path, source))?;
    let value: T = serde_json::from_str(&content)?;
    info!(path = %path.display(), length = content.len(), "load JSON file");
    Ok(value)
}

pub fn load_toml<P, T>(path: P) -> Result<T, FileError>
where
    P: AsRef<Path>,
    T: DeserializeOwned,
{
    let path = path.as_ref();
    let content = fs::read_to_string(path).map_err(|source| PathError::new(path, source))?;
    let value: T = toml::from_str(&content)?;
    info!(path = %path.display(), length = content.len(), "load TOML file");
    Ok(value)
}

pub fn load_image<P>(path: P) -> Result<Bytes, FileError>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    let bytes: Bytes = fs::read(path)
        .map_err(|source| PathError::new(path, source))?
        .into();
    info!(path = %path.display(), length = bytes.len(), "load image file");
    Ok(bytes)
}

pub fn save_html<P>(html: &String, path: P) -> Result<(), FileError>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    fs::write(path, html).map_err(|source| PathError::new(path, source))?;
    info!(path = %path.display(), length = html.len(), "save HTML file");
    Ok(())
}

pub fn exists_and_non_empty<P>(path: P) -> bool
where
    P: AsRef<Path>,
{
    fs::metadata(path).is_ok_and(|m| m.len() > 0)
}
