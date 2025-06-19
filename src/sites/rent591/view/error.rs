use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum Error {
    #[error("no rental items found")]
    #[diagnostic(
        code(sites::rent591::view::no_items),
        help(
            "verify the URL is a valid rent.591.com.tw list page or contact the author if the scraper needs updating"
        )
    )]
    NoItems,
}
