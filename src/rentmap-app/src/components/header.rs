use dioxus::prelude::*;

use super::icons::MapIcon;

#[component]
pub fn Header(
    property_count: u32,
    geocoded_count: u32,
    #[props(default = "RentMap Dashboard".to_string())] title: String,
    #[props(default = "Geo Data Scraping Visualization".to_string())] subtitle: String,
) -> Element {
    rsx! {
        header { class: "bg-white shadow-sm border-b border-gray-200",
            div { class: "max-w-7xl mx-auto px-6 py-3",
                div { class: "flex items-center justify-between",
                    div { class: "flex items-center space-x-3",
                        div { class: "w-8 h-8 bg-blue-600 rounded-lg flex items-center justify-center",
                            MapIcon { class: "w-5 h-5 text-white" }
                        }
                        div {
                            h1 { class: "text-xl font-semibold text-gray-900", "{title}" }
                            p { class: "text-sm text-gray-500", "{subtitle}" }
                        }
                    }
                    div { class: "flex items-center space-x-4 text-sm text-gray-600",
                        span { "{property_count} properties" }
                        span { "•" }
                        span { "{geocoded_count} geocoded" }
                    }
                }
            }
        }
    }
}
