use std::ops::Deref;

use miette::Diagnostic;
use thiserror::Error;
use url::Url;

use crate::url_wrapper;

#[derive(Debug, Error, Diagnostic)]
pub enum UrlError {
    #[error("invalid rent.591.com.tw URL path")]
    #[diagnostic(
        code(sites::rent591::url::invalid_path),
        help("URL must be either a list page (/list) or item page (/<id>)")
    )]
    InvalidPath,
}

/// URL path type for rent.591.com.tw
pub enum PathType {
    /// List page: `/list` with optional query parameters
    List(ListUrl),
    /// Item detail page: `/<id>` where id is numeric
    Item(ItemUrl),
}

impl TryFrom<Url> for PathType {
    type Error = UrlError;

    fn try_from(url: Url) -> Result<Self, Self::Error> {
        let mut segments = url.path_segments().ok_or(UrlError::InvalidPath)?;

        match (segments.next(), segments.next()) {
            (Some("list"), None) => Ok(PathType::List(ListUrl(url))),

            (Some(id), None) if !id.is_empty() && id.chars().all(|c| c.is_ascii_digit()) => {
                Ok(PathType::Item(ItemUrl(url)))
            }
            _ => Err(UrlError::InvalidPath),
        }
    }
}

impl From<PathType> for Url {
    fn from(path_type: PathType) -> Url {
        match path_type {
            PathType::List(url) => url.0,
            PathType::Item(url) => url.0,
        }
    }
}

impl Deref for PathType {
    type Target = Url;

    fn deref(&self) -> &Self::Target {
        match self {
            PathType::List(url) => &url.0,
            PathType::Item(url) => &url.0,
        }
    }
}

url_wrapper! {
    /// Wrapper for rental listing URLs with query parameter support
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
    ListUrl
}

url_wrapper! {
    /// Wrapper for rental item detail URLs
    ItemUrl
}
