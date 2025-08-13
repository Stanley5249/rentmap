use dioxus::document::{Link, Meta, Stylesheet, Title};
use dioxus::prelude::*;

static FAVICON: Asset = asset!("/assets/favicon.ico");
static TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[component]
pub fn DocumentMeta() -> Element {
    rsx! {
        Title { "RentMap Dashboard" }
        Meta {
            name: "description",
            content: "Geo data scraping visualization dashboard for rental properties",
        }
        Meta { name: "language", content: "en" }
        Link { rel: "icon", href: FAVICON }
        Stylesheet { href: TAILWIND_CSS }
    }
}
