mod error;
mod ops;
mod url;
mod workspace;

pub use error::{FileError, PathError};
pub use ops::{
    exists_and_non_empty, load_image, load_json, load_toml, make_directory, save_html, save_json,
};
pub use url::UrlExt;
pub use workspace::{SortByTimestamp, TimedRecord, TimedRecords, Workspace};

#[macro_export]
macro_rules! url_wrapper {
    ($(#[$attr:meta])* $name:ident) => {
        $(#[$attr])*
        #[derive(::std::clone::Clone)]
        pub struct $name(::url::Url);

        impl $name {
            pub fn new(url: ::url::Url) -> Self {
                Self(url)
            }
        }

        impl ::std::convert::From<$name> for ::url::Url {
            fn from(wrapper: $name) -> ::url::Url {
                wrapper.0
            }
        }

        impl ::std::ops::Deref for $name {
            type Target = ::url::Url;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl ::std::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                self.0.fmt(f)
            }
        }
    };
}
