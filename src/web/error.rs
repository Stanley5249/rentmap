use miette::Diagnostic;
use thiserror::Error;

use super::backends::BackendError;

#[derive(Debug, Error, Diagnostic)]
pub enum WebError {
    #[error(transparent)]
    #[diagnostic(transparent)]
    Backend(#[from] BackendError),

    #[error("no pages found")]
    #[diagnostic(
        code(web::no_pages),
        help(
            "the website returned no content or chrome failed to load the page

common causes:
- invalid or inaccessible website URL
- chrome browser initialization failed (often after previous crashes)
- network connectivity issues or website is down
- website requires authentication or blocks automated access

troubleshooting steps:
1. verify the URL is correct and accessible in a regular browser
2. if chrome failed to initialize, kill existing chrome processes:
   - windows: Stop-Process -Name chrome
   - linux/mac: pkill chrome
3. check your internet connection
4. retry the command after killing chrome processes"
        )
    )]
    NoPages,
}
