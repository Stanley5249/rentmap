use dioxus::document::{Link, Meta, Title};
use dioxus::prelude::*;

static FAVICON: Asset = asset!("/assets/favicon.ico");
static TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[component]
pub fn DocumentMeta() -> Element {
    rsx! {
        // Page title
        Title { "RentMap Dashboard" }
        // Basic meta tags for better accessibility and SEO
        Meta {
            name: "description",
            content: "Geo data scraping visualization dashboard for rental properties",
        }
        Meta { name: "language", content: "en" }
        // Favicon
        Link { rel: "icon", href: FAVICON }
        // Stylesheet
        Link { rel: "stylesheet", href: TAILWIND_CSS }
    }
}
