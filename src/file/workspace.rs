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

fn url_to_file_name(url: &Url) -> PathBuf {
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
