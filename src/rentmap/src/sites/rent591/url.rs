use std::ops::Deref;

use miette::Diagnostic;
use thiserror::Error;
use url::Url;

#[derive(Debug, Error, Diagnostic)]
pub enum UrlError {
    #[error("invalid rent 591 domain {:?}", .0.domain())]
    #[diagnostic(
        code(sites::rent591::url::invalid_domain),
        help("domain must be exactly rent.591.com.tw")
    )]
    InvalidDomain(Url),

    #[error("invalid rent 591 path {:?}", .0.path())]
    #[diagnostic(
        code(sites::rent591::url::invalid_path),
        help("path must be either a list page (/list) or item page (/<id>)")
    )]
    InvalidPath(Url),

    #[error("expected a rent 591 list page {:?}", .0.path())]
    #[diagnostic(
        code(sites::rent591::url::expect_list),
        help("path must be a list page (/list)")
    )]
    ExpectList(Url),

    #[error("expected a rent 591 item page {:?}", .0.path())]
    #[diagnostic(
        code(sites::rent591::url::expect_item),
        help("path must be an item page (/<id>)")
    )]
    ExpectItem(Url),
}

impl From<UrlError> for Url {
    fn from(err: UrlError) -> Self {
        match err {
            UrlError::InvalidDomain(url)
            | UrlError::InvalidPath(url)
            | UrlError::ExpectList(url)
            | UrlError::ExpectItem(url) => url,
        }
    }
}

/// URL path type for rent.591.com.tw
pub enum Rent591Url {
    /// Rental listing URLs with query parameter support
    ///
    /// Common query parameters:
    /// - `region`: Location ID (e.g., `15` for specific district)
    /// - `kind`: Property type (`1`=all, `2`=apartment, etc.)
    /// - `school`: School district ID (e.g., `2670`)
    /// - `price`: Price range (`$start_end$` format, e.g., `$_8000$` for up to 8000)
    /// - `other`: Features (comma-separated: `newPost,near_subway,pet,cook,cartplace,lift,balcony_1,lease`)
    /// - `shape`: Room configuration (`1,2,3,4`)
    /// - `acreage`: Size range (`0_10,10_20,20_30,30_40,40_50,50_` format)
    /// - `floor`: Floor range (`1_1,2_6,6_12,13_` format)
    /// - `bathroom`: Bathroom count (`1,2,3,4_`)
    /// - `option`: Equipment (comma-separated: `cold,washer,icebox,hotwater,naturalgas,broadband,bed`)
    /// - `fitment`: Decoration level (`99,3,4`)
    /// - `notice`: Tenant restrictions (`all_sex,boy,girl,not_cover`)
    List(Url),

    /// rental item URLs Item page: `/<id>` where id is numeric
    Item(Url),
}

impl From<Rent591Url> for Url {
    fn from(path_type: Rent591Url) -> Url {
        match path_type {
            Rent591Url::List(url) => url,
            Rent591Url::Item(url) => url,
        }
    }
}

impl Deref for Rent591Url {
    type Target = Url;

    fn deref(&self) -> &Self::Target {
        match self {
            Rent591Url::List(url) => url,
            Rent591Url::Item(url) => url,
        }
    }
}

impl TryFrom<Url> for Rent591Url {
    type Error = UrlError;

    fn try_from(url: Url) -> Result<Self, Self::Error> {
        match url.domain() {
            Some("rent.591.com.tw") => Rent591Url::try_from_matched_domain(url),
            _ => Err(UrlError::InvalidDomain(url)),
        }
    }
}

impl Rent591Url {
    pub(in crate::sites) fn try_from_matched_domain(url: Url) -> Result<Self, UrlError> {
        let segments: Vec<_> = match url.path_segments() {
            Some(v) => v,
            None => return Err(UrlError::InvalidPath(url)),
        }
        .collect();

        match segments.as_slice() {
            ["" | "list"] => Ok(Self::List(url)),

            [id] if is_ascii_digits(id) => Ok(Self::Item(url)),

            _ => Err(UrlError::InvalidPath(url)),
        }
    }
}

fn is_ascii_digits(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_digit())
}

pub trait ListUrlExt {
    fn page(&self) -> Option<u32>;
    fn add_page(&mut self, page: u32);
    fn del_page(&mut self);
    fn with_page(&self, page: u32) -> Self
    where
        Self: Sized;
    fn without_page(&self) -> Self
    where
        Self: Sized;
}

impl ListUrlExt for Url {
    fn page(&self) -> Option<u32> {
        self.query_pairs()
            .find(|(key, _)| key == "page")
            .and_then(|(_, value)| value.parse().ok())
    }

    fn add_page(&mut self, page: u32) {
        let mut pairs: Vec<(String, String)> = self
            .query_pairs()
            .map(|(k, v)| (k.into_owned(), v.into_owned()))
            .collect();
        pairs.retain(|(k, _)| k != "page");
        pairs.push(("page".to_string(), page.to_string()));
        pairs.sort();
        self.query_pairs_mut().clear().extend_pairs(pairs);
    }

    fn del_page(&mut self) {
        let pairs: Vec<(String, String)> = self
            .query_pairs()
            .filter(|(key, _)| key != "page")
            .map(|(k, v)| (k.into_owned(), v.into_owned()))
            .collect();

        self.query_pairs_mut().clear().extend_pairs(pairs);
    }

    fn with_page(&self, page: u32) -> Self {
        let mut url = self.clone();
        url.add_page(page);
        url
    }

    fn without_page(&self) -> Self {
        let mut url = self.clone();
        url.del_page();
        url
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_from_list_url() {
        let url = Url::parse("https://rent.591.com.tw/list?region=1").unwrap();
        assert!(matches!(Rent591Url::try_from(url), Ok(Rent591Url::List(_)),));
    }

    #[test]
    fn test_try_from_item_url() {
        let url = Url::parse("https://rent.591.com.tw/123456").unwrap();
        assert!(matches!(Rent591Url::try_from(url), Ok(Rent591Url::Item(_))));
    }

    #[test]
    fn test_try_from_invalid_path() {
        let url = Url::parse("https://rent.591.com.tw/foo/bar").unwrap();
        assert!(matches!(
            Rent591Url::try_from(url),
            Err(UrlError::InvalidPath(_))
        ));
    }

    #[test]
    fn test_try_from_empty_path() {
        let url = Url::parse("https://rent.591.com.tw/").unwrap();
        assert!(matches!(Rent591Url::try_from(url), Ok(Rent591Url::List(_))));
    }
}
