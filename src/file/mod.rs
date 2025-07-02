mod error;
mod ops;
mod url;
mod workspace;

pub use error::{FileError, PathError};
pub use ops::{
    exists_and_nonempty, load_image, load_json, load_toml, make_directory, save_html, save_json,
};
pub use url::{normalize_url, url_to_file_name};
pub use workspace::{SortByTimestamp, TimedRecord, TimedRecords, Workspace};

#[macro_export]
macro_rules! url_wrapper {
    ($(#[$attr:meta])* $name:ident) => {
        $(#[$attr])*
        pub struct $name(url::Url);

        impl $name {
            pub fn new(url: url::Url) -> Self {
                Self(url)
            }
        }

        impl From<$name> for url::Url {
            fn from(wrapper: $name) -> url::Url {
                wrapper.0
            }
        }

        impl std::ops::Deref for $name {
            type Target = url::Url;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}
