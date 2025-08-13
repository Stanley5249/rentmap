use std::collections::BTreeMap;

use dioxus::prelude::*;
use tracing::debug;

use super::card::MarkerCard;
use crate::components::rentmap::model::MarkerData;

#[component]
pub fn MarkerPanel(
    markers: Signal<BTreeMap<u32, MarkerData>>,
    marker_count: Memo<usize>,
    on_remove: EventHandler<MarkerData>,
    on_clear: EventHandler<usize>,
) -> Element {
    let handle_remove = move |id: u32| {
        if let Some(marker) = markers.write().remove(&id) {
            debug!(id, ?marker, "Remove marker");
            on_remove.call(marker);
        }
    };

    let handle_clear_all = move |_| {
        let mut markers = markers.write();
        let count = markers.len();
        markers.clear();

        debug!(count, "Clear all markers");
        on_clear.call(count);
    };

    rsx! {
        div { class: "flex flex-col h-full",
            div { class: "flex items-center justify-between mb-3 flex-shrink-0",
                h4 { class: "text-md font-medium text-gray-800", "Markers" }
                span { class: "text-sm text-gray-500 bg-gray-100 px-2 py-1 rounded-full",
                    "({marker_count})"
                }
            }

            div { class: "flex-1 overflow-y-auto min-h-0",
                if marker_count() == 0 {
                    EmptyState {}
                } else {
                    div { class: "space-y-2",
                        for (id , marker) in markers.read().iter() {
                            MarkerCard {
                                key: "{id}",
                                marker: marker.clone(),
                                id: *id,
                                on_remove: handle_remove,
                            }
                        }
                    }
                }
            }

            if marker_count() > 0 {
                div { class: "mt-4 pt-3 border-t border-gray-200 flex-shrink-0",
                    button {
                        class: "w-full px-3 py-2 bg-gray-100 hover:bg-gray-200 text-gray-700 text-sm font-medium rounded-md transition-colors duration-200 focus:outline-none focus:ring-2 focus:ring-gray-300 cursor-pointer",
                        onclick: handle_clear_all,
                        "Clear All Markers"
                    }
                }
            }
        }
    }
}

#[component]
fn EmptyState() -> Element {
    rsx! {
        div { class: "text-center py-8 text-gray-500",
            div { class: "text-4xl mb-2", "📍" }
            p { class: "text-sm font-medium", "No markers yet" }
            p { class: "text-xs mt-1 text-gray-400", "Click 'Add Marker' to start" }
        }
    }
}
