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
}
