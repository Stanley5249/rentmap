use super::icons::MapIcon;
use dioxus::prelude::*;

#[component]
pub fn Header(
    property_count: u32,
    geocoded_count: u32,
    title: Option<String>,
    subtitle: Option<String>,
) -> Element {
    let title = title.unwrap_or_else(|| "RentMap Dashboard".to_string());
    let subtitle = subtitle.unwrap_or_else(|| "Geo Data Scraping Visualization".to_string());

    rsx! {
        header { class: "bg-white shadow-sm border-b border-gray-200 flex-shrink-0",
            div { class: "max-w-7xl mx-auto px-6 py-3",
                div { class: "flex items-center justify-between",
                    div { class: "flex items-center space-x-3",
                        div { class: "w-8 h-8 bg-blue-600 rounded-lg flex items-center justify-center",
                            MapIcon { class: "w-5 h-5 text-white".to_string() }
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
