pub mod backends;
pub mod dom;
pub mod error;
pub mod fetcher;
pub mod page;

#[macro_export]
macro_rules! define_selectors {
    ($struct_name:ident, $($field:ident: $selector:literal),* $(,)?) => {
        struct $struct_name {
            $(pub $field: ::scraper::Selector,)*
        }
        impl $struct_name {
            pub fn new() -> Self {
                Self {
                    $($field: ::scraper::Selector::parse($selector).unwrap(),)*
                }
            }
        }
    };
}
