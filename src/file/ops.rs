use std::fs;
use std::path::{Path, PathBuf};

use bytes::Bytes;
use chrono::{DateTime, Utc};
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
    debug!(path = %path.display(), "directory already exists");
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

/// Find latest file by timestamp. Expects filename format: %Y-%m-%dT%H-%M-%S.ext
pub fn find_latest_file<P>(dir: P, extension: &str) -> Result<(DateTime<Utc>, PathBuf), FileError>
where
    P: AsRef<Path>,
{
    iter_files_in_directory(dir)?
        .filter(|path| path.extension().is_some_and(|e| e == extension))
        .filter_map(|path| parse_file_timestamp(&path).map(|ts| (ts, path)))
        .max_by_key(|(ts, _)| *ts)
        .ok_or(FileError::NotFound)
}

fn parse_file_timestamp(path: &Path) -> Option<DateTime<Utc>> {
    let timestamp_str = path.file_stem()?.to_str()?;
    DateTime::parse_from_str(timestamp_str, "%Y-%m-%dT%H-%M-%S")
        .ok()
        .map(|dt| dt.with_timezone(&Utc))
}

pub fn iter_files_in_directory<P>(dir: P) -> Result<impl Iterator<Item = PathBuf>, FileError>
where
    P: AsRef<Path>,
{
    let dir = dir.as_ref();
    let paths = fs::read_dir(dir)
        .map_err(|source| PathError::new(dir, source))?
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            entry
                .file_type()
                .ok()
                .and_then(|ft| ft.is_file().then_some(entry.path()))
        });
    Ok(paths)
}
