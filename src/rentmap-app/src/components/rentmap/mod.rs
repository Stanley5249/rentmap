mod gmp;
mod model;
mod sidebar;

use std::collections::BTreeMap;

use dioxus::prelude::*;
use tracing::debug;

#[allow(unused_imports)]
pub use gmp::{GoogleMap, GoogleMapMarker};
pub use model::MarkerData;
pub use sidebar::{MarkerPanel, MarkerToolbar};

#[component]
pub fn RentMap(center: (f64, f64), zoom: u8) -> Element {
    let mut markers = use_signal(BTreeMap::<u32, MarkerData>::new);
    let mut marker_id = use_signal(|| 0u32);
    let mut picker_mode = use_signal(|| false);

    let marker_count = use_memo(move || markers.read().len());

    let handle_map_click = move |(lat, lng): (f64, f64)| {
        let id = marker_id();
        marker_id += 1;

        let marker = MarkerData::new(lat, lng, format!("Marker {}", marker_count() + 1));

        debug!(id, ?marker, "Add marker from map click");
        markers.write().insert(id, marker);
        picker_mode.set(false);
    };

    rsx! {
        div { class: "flex h-full w-full overflow-hidden",
            div { class: "flex-1 relative",
                GoogleMap {
                    center,
                    zoom,
                    markers,
                    picker_mode,
                    on_map_click: handle_map_click,
                }
            }

            div { class: "w-80 bg-white border-l border-gray-300 flex flex-col shadow-lg h-full overflow-hidden",
                div { class: "p-4 space-y-4 flex flex-col h-full",
                    h3 { class: "text-lg font-semibold text-gray-900 flex-shrink-0",
                        "Map Controls"
                    }

                    div { class: "flex-shrink-0",
                        MarkerToolbar {
                            markers,
                            marker_id,
                            picker_mode,
                            marker_count,
                        }
                    }

                    div { class: "flex-1 min-h-0",
                        MarkerPanel {
                            markers,
                            marker_count,
                            on_remove: move |marker: MarkerData| {
                                debug!(? marker, "Remove marker from panel");
                            },
                            on_clear: move |count: usize| {
                                debug!(count, "Clear all markers from panel");
                            },
                        }
                    }
                }
            }
        }
    }
}
