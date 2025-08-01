mod error;
mod ops;

pub use error::{FileError, PathError};
pub use ops::{
    exists_and_non_empty, load_image, load_json, load_toml, make_directory, save_html, save_json,
};
