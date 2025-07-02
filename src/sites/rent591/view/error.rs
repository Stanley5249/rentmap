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

    #[error("no rental item found")]
    #[diagnostic(
        code(sites::rent591::view::no_rent_item),
        help(
            "verify the URL is a valid rent.591.com.tw item page or contact the author if the scraper needs updating"
        )
    )]
    NoRentItem,
}
