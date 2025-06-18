pub mod backends;
pub mod dom;
pub mod error;
pub mod fetcher;
pub mod page;

#[macro_export]
macro_rules! selectors {
    {$($name:ident: $str:expr),* $(,)?} => {
        $(static $name: std::sync::LazyLock<scraper::Selector> = std::sync::LazyLock::new(|| scraper::Selector::parse($str).unwrap());)*
    };
}
