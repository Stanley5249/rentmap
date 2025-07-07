use std::path::PathBuf;

use sanitise_file_name::sanitise;
use url::Url;

#[macro_export]
macro_rules! url_wrapper {
    ($(#[$attr:meta])* $name:ident) => {
        $(#[$attr])*
        #[derive(::std::clone::Clone, ::std::fmt::Debug)]
        pub struct $name(pub(crate) ::url::Url);

        impl $name {
            pub fn url(&self) -> &::url::Url {
                &self.0
            }
        }

        impl ::std::convert::From<$name> for ::url::Url {
            fn from(wrapper: $name) -> ::url::Url {
                wrapper.0
            }
        }

        impl ::std::ops::Deref for $name {
            type Target = ::url::Url;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl ::std::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl ::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                self.0.serialize(serializer)
            }
        }

        impl<'de> ::serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                let url = ::url::Url::deserialize(deserializer)?;
                Ok(Self(url))
            }
        }
    };
}

pub trait UrlExt {
    fn query_pairs_owned(&self) -> impl Iterator<Item = (String, String)>;
    fn sort_query_pairs(&mut self);
    fn normalize(&mut self);
    fn to_path_buf(&self) -> PathBuf;
}

impl UrlExt for Url {
    /// Convert query parameters to owned strings
    fn query_pairs_owned(&self) -> impl Iterator<Item = (String, String)> {
        self.query_pairs()
            .map(|(k, v)| (k.into_owned(), v.into_owned()))
    }

    /// Sort query parameters in a URL by key, removing empty values
    fn sort_query_pairs(&mut self) {
        let mut pairs: Vec<(String, String)> = self
            .query_pairs()
            .filter(|(_, value)| !value.is_empty())
            .map(|(k, v)| (k.into_owned(), v.into_owned()))
            .collect();

        pairs.sort();

        self.query_pairs_mut().clear().extend_pairs(pairs);
    }

    /// Normalize URL by removing fragments, sorting query parameters, and cleaning paths
    fn normalize(&mut self) {
        self.set_fragment(None);

        self.sort_query_pairs();

        if let Ok(mut segments) = self.path_segments_mut() {
            segments.pop_if_empty();
        }
    }

    fn to_path_buf(&self) -> PathBuf {
        let mut components = Vec::new();

        if let Some(host) = self.host_str() {
            components.push(host.to_string());
        }

        match self.path_segments() {
            Some(path_segments) => {
                components.extend(path_segments.map(|s| s.to_string()));
            }
            None => {
                components.push(self.path().to_string());
            }
        }

        if let Some(last) = components.last_mut() {
            if last.is_empty() {
                *last = "index".to_string();
            }
        }

        if let Some(query) = self.query() {
            if let Some(last) = components.last_mut() {
                if !last.is_empty() {
                    last.reserve(query.len() + 1);
                    last.push('_');
                    last.push_str(query);
                }
            } else {
                components.push(query.to_string());
            }
        }

        let mut path = PathBuf::from_iter(components.iter().map(|s| sanitise(s)));

        path.set_extension("html");

        path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_url_to_pathbuf(url_str: &str, expected: &str) {
        let url = Url::parse(url_str).unwrap();
        let result = url.to_path_buf();
        assert_eq!(PathBuf::from(expected), result);
    }

    #[test]
    fn host_no_path_no_query() {
        assert_url_to_pathbuf("https://example.com", "example.com/index.html");
    }
    #[test]
    fn host_with_path_no_query() {
        assert_url_to_pathbuf(
            "https://example.com/rent/listings",
            "example.com/rent/listings.html",
        );
    }
    #[test]
    fn host_no_path_with_query() {
        assert_url_to_pathbuf(
            "https://example.com?q=apartment&city=taipei",
            "example.com/index_q=apartment&city=taipei.html",
        );
    }
    #[test]
    fn host_with_path_and_query() {
        assert_url_to_pathbuf(
            "https://example.com/rent/listings?city=taipei&type=apartment",
            "example.com/rent/listings_city=taipei&type=apartment.html",
        );
    }
    #[test]
    fn no_host_no_path_no_query() {
        assert_url_to_pathbuf("file:///", "index.html");
    }
    #[test]
    fn no_host_no_path_with_query() {
        assert_url_to_pathbuf("file:///?search=test", "index_search=test.html");
    }
    #[test]
    fn host_empty_path_with_query() {
        assert_url_to_pathbuf(
            "https://example.com/?search=test",
            "example.com/index_search=test.html",
        );
    }
    #[test]
    fn host_single_path_with_query() {
        assert_url_to_pathbuf(
            "https://example.com/page?q=test",
            "example.com/page_q=test.html",
        );
    }
}
