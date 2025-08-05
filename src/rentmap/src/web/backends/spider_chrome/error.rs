use miette::Diagnostic;
use spider_chrome::error::CdpError;
use thiserror::Error;
use url::ParseError;

#[derive(Debug, Error, Diagnostic)]
pub enum SpiderChromeError {
    #[error("browser configuration failed: {0}")]
    #[diagnostic(
        code(web::backends::spider_chrome::config),
        help("check Chrome/Chromium installation and browser configuration parameters")
    )]
    Config(String),

    #[error(transparent)]
    #[diagnostic(
        code(web::backends::spider_chrome::cdp),
        help(
            "Chrome browser may have crashed or become unresponsive - try restarting the application"
        )
    )]
    Cdp(#[from] CdpError),

    #[error(transparent)]
    #[diagnostic(
        code(web::backends::spider_chrome::url_parse),
        help("ensure the URL is properly formatted (e.g., https://example.com)")
    )]
    UrlParse(#[from] ParseError),

    #[error("page URL not available")]
    #[diagnostic(
        code(web::backends::spider_chrome::no_page_url),
        help("browser page did not provide a valid URL - this may indicate a navigation failure")
    )]
    NoPageUrl,

    #[error(transparent)]
    #[diagnostic(
        code(web::backends::spider_chrome::io),
        help("check file permissions and available disk space")
    )]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    #[diagnostic(
        code(web::backends::spider_chrome::task_join),
        help("browser handler task panicked or was cancelled - this indicates an internal error")
    )]
    Join(#[from] tokio::task::JoinError),
}
