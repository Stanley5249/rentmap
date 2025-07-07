use miette::Diagnostic;
use thiserror::Error;
use url::Url;

use super::rent591;

#[derive(Debug, Error, Diagnostic)]
pub enum Error {
    #[error("unsupported or missing domain: {:?}", .0.domain())]
    #[diagnostic(
        code(sites::site::unsupported_domain),
        help("supported domains: rent.591.com.tw")
    )]
    UnsupportedDomain(Url),

    #[error(transparent)]
    #[diagnostic(transparent)]
    Rent591(#[from] rent591::UrlError),
}

pub enum SiteUrl {
    /// rent.591.com.tw
    Rent591(rent591::Rent591Url),
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
            Some("rent.591.com.tw") => {
                let domain = rent591::Rent591Domain(url);
                let url = rent591::Rent591Url::try_from(domain)?;
                Ok(Self::Rent591(url))
            }
            _ => Err(Self::Error::UnsupportedDomain(url)),
        }
    }
}
