use std::fs;
use std::path::{Path, PathBuf};

use bytes::Bytes;
use serde::Serialize;
use serde::de::DeserializeOwned;
use tracing::{debug, info};
use url::Url;

use super::FileError;
use crate::file::PathError;

pub fn make_directory<P>(path: &P) -> Result<(), FileError>
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

pub fn save_json<V, P>(value: &V, path: &P) -> Result<(), FileError>
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

pub fn load_json<P, T>(path: &P) -> Result<T, FileError>
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

pub fn load_toml<P, T>(path: &P) -> Result<T, FileError>
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

pub fn load_image<P>(path: &P) -> Result<Bytes, FileError>
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

pub fn save_html<P>(html: &String, path: &P) -> Result<(), FileError>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    fs::write(path, html).map_err(|source| PathError::new(path, source))?;
    info!(path = %path.display(), length = html.len(), "save HTML file");
    Ok(())
}

pub fn url_to_file_name(url: &Url) -> PathBuf {
    use sanitise_file_name::sanitise;

    let mut file_name = PathBuf::new();

    if let Some(host) = url.host_str() {
        file_name.push(sanitise(host));
    }

    match (url.path_segments(), url.query()) {
        (Some(segments), Some(query)) => {
            // When `path_segments` returns `Some`, it always contains at least one string (which may be empty).
            let segments: Vec<_> = segments.filter(|s| !s.is_empty()).collect();

            if let Some((last, rest)) = segments.split_last() {
                file_name.extend(rest.iter().map(|s| sanitise(s)));
                file_name.push(sanitise(&format!("{}_{}", last, query)));
            } else {
                file_name.push(sanitise(query));
            }
        }
        (Some(segments), None) => {
            let segments = segments.map(|s| match s {
                "" => "index".to_string(),
                _ => sanitise(s),
            });
            file_name.extend(segments);
        }
        (None, Some(query)) => {
            file_name.push(sanitise(query));
        }
        (None, None) => file_name.push("index"),
    };

    file_name.set_extension("html");
    file_name
}

#[cfg(test)]
mod tests {
    use super::url_to_file_name;
    use std::path::PathBuf;
    use url::Url;

    macro_rules! test_url_to_file_name {
        ($($test_name:ident, $url:expr, $expected:expr);* $(;)?) => {
            $(
                #[test]
                fn $test_name() {
                    let url = Url::parse($url).unwrap();
                    let result = url_to_file_name(&url);
                    assert_eq!(result, PathBuf::from($expected));
                }
            )*
        };
    }

    test_url_to_file_name! {
        host_no_path_no_query, "https://example.com", "example.com/index.html";
        host_with_path_no_query, "https://example.com/rent/listings", "example.com/rent/listings.html";
        host_no_path_with_query, "https://example.com?q=apartment&city=taipei", "example.com/q=apartment&city=taipei.html";
        host_with_path_and_query, "https://example.com/rent/listings?city=taipei&type=apartment", "example.com/rent/listings_city=taipei&type=apartment.html";
        no_host_no_path_no_query, "file:///", "index.html";
        no_host_no_path_with_query, "file:///?search=test", "search=test.html";
        host_empty_path_with_query, "https://example.com/?search=test", "example.com/search=test.html";
        host_single_path_with_query, "https://example.com/page?q=test", "example.com/page_q=test.html";
    }

    #[test]
    fn test_url_with_invalid_chars() {
        let url = Url::parse("https://example.com/path?query=test").unwrap();
        let result = url_to_file_name(&url);
        let result_str = result.to_string_lossy();
        assert!(result_str.ends_with(".html"));
    }
}
