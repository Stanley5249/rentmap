#[macro_export]
macro_rules! define_selectors {
    ($struct_name:ident, $($field:ident: $selector:literal),* $(,)?) => {
        struct $struct_name {
            $($field: ::scraper::Selector,)*
        }
        impl $struct_name {
            fn new() -> Self {
                Self {
                    $($field: ::scraper::Selector::parse($selector).unwrap(),)*
                }
            }
        }
    };
}
