use dioxus::prelude::*;
use tracing::debug;

use crate::components::rentmap::MarkerData;

#[component]
pub fn MarkerCard(marker: MarkerData, id: u32, on_remove: EventHandler<u32>) -> Element {
    rsx! {
        div { class: "p-3 border border-gray-200 rounded-lg bg-gray-50 hover:bg-gray-100 transition-colors duration-150",
            div { class: "flex items-start justify-between",
                div { class: "flex-1 min-w-0",
                    h5 {
                        class: "font-medium text-sm text-gray-800 truncate",
                        title: "{marker.title}",
                        "{marker.title}"
                    }
                    p { class: "text-xs text-gray-500 mt-1 font-mono",
                        "{marker.format_coordinates(4)}"
                    }
                }
                button {
                    class: "text-red-500 hover:text-red-700 hover:bg-red-50 text-lg font-bold p-1 rounded transition-colors duration-150 ml-2 flex-shrink-0 cursor-pointer",
                    onclick: move |_| {
                        debug!(id, ? marker, "Remove marker from card");
                        on_remove.call(id);
                    },
                    title: "Remove marker",
                    "×"
                }
            }
        }
    }
}
