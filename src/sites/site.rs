use miette::Diagnostic;
use thiserror::Error;
use url::Url;

#[derive(Debug, Error, Diagnostic)]
pub enum Error {
    #[error("invalid URL domain")]
    #[diagnostic(code(sites::site::invalid_domain))]
    InvalidDomain,

    #[error("site '{0}' is not supported")]
    #[diagnostic(code(sites::site::invalid_site))]
    InvalidSite(String),

    #[error(transparent)]
    #[diagnostic(transparent)]
    Rent591(#[from] super::rent591::url::Error),
}

pub enum SiteUrl {
    /// rent.591.com.tw
    Rent591(super::rent591::url::PathUrl),
}

impl From<super::rent591::url::PathUrl> for SiteUrl {
    fn from(url: super::rent591::url::PathUrl) -> Self {
        SiteUrl::Rent591(url)
    }
}

impl From<SiteUrl> for Url {
    fn from(site_url: SiteUrl) -> Self {
        match site_url {
            SiteUrl::Rent591(url) => url.into(),
        }
    }
}

impl TryFrom<Url> for SiteUrl {
    type Error = Error;

    fn try_from(url: Url) -> Result<Self, Self::Error> {
        match url.domain() {
            Some("rent.591.com.tw") => Ok(super::rent591::url::PathUrl::try_from(url)?.into()),
            Some(domain) => Err(Error::InvalidSite(domain.into())),
            None => Err(Error::InvalidDomain),
        }
    }
}
