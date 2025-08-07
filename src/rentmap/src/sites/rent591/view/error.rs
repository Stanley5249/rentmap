use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum ViewError {
    #[error("no item found")]
    #[diagnostic(
        code(sites::rent591::view::no_item),
        help("verify the URL is a valid rent.591.com.tw item page")
    )]
    NoItem,

    #[error("no list found")]
    #[diagnostic(
        code(sites::rent591::view::no_list),
        help("verify the URL is a valid rent.591.com.tw list page")
    )]
    NoList,

    #[error("no item summaries found")]
    #[diagnostic(
        code(sites::rent591::view::no_item_summaries),
        help("verify the URL is a valid rent.591.com.tw list page")
    )]
    NoItemSummaries,
}
