use std::path::PathBuf;

use sanitise_file_name::sanitise;
use url::Url;

/// Normalize URL by removing fragments, sorting query parameters, and cleaning paths
pub fn normalize_url(mut url: Url) -> Url {
    url.set_fragment(None);

    if url.query().is_some() {
        let pairs: Vec<(String, String)> = url
            .query_pairs()
            .filter(|(_, value)| !value.is_empty())
            .map(|(k, v)| (k.into_owned(), v.into_owned()))
            .collect();

        if !pairs.is_empty() {
            let mut sorted_pairs = pairs;
            sorted_pairs.sort_unstable_by(|a, b| a.0.cmp(&b.0));
            url.query_pairs_mut().clear().extend_pairs(sorted_pairs);
        } else {
            url.set_query(None);
        }
    }

    if let Ok(mut segments) = url.path_segments_mut() {
        segments.pop_if_empty();
    }

    url
}

pub fn url_to_file_name(url: &Url) -> PathBuf {
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
    use std::path::PathBuf;

    use url::Url;

    use super::url_to_file_name;

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
