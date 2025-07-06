use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
pub enum Error {
    #[error("no rental list found in workspace, please run `rentmap list` first")]
    #[diagnostic(
        code(rentmap::item::no_rent_list),
        help("run `rentmap list` to fetch rental list")
    )]
    NoRentList,

    #[error("expected a rental listing page URL as input")]
    #[diagnostic(
        code(rentmap::item::expect_list_url),
        help("provide a valid rental listing page URL, e.g. https://rent.591.com.tw/list")
    )]
    ExpectListUrl,
}
