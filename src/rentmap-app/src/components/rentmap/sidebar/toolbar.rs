use std::collections::BTreeMap;

use dioxus::prelude::*;
use tracing::debug;

use crate::components::rentmap::model::MarkerData;

#[component]
pub fn MarkerToolbar(
    markers: Signal<BTreeMap<u32, MarkerData>>,
    marker_id: Signal<u32>,
    picker_mode: Signal<bool>,
    marker_count: Memo<usize>,
) -> Element {
    let handle_add_marker = move |_| {
        picker_mode.set(true);
        debug!(picker.mode = "active", "Enter picker mode");
    };

    let handle_cancel_picker = move |_| {
        picker_mode.set(false);
        debug!(picker.mode = "cancelled", "Cancel picker mode");
    };

    rsx! {
        div { class: "space-y-3",
            if !picker_mode() {
                button {
                    class: "w-full px-4 py-3 bg-blue-500 hover:bg-blue-600 text-white font-medium rounded-lg shadow-sm transition-all duration-200 flex items-center justify-center gap-3 cursor-pointer focus:outline-none focus:ring-2 focus:ring-blue-300",
                    onclick: handle_add_marker,
                    span { class: "text-lg", "📍" }
                    span { "Add Marker" }
                }
            }
            if picker_mode() {
                button {
                    class: "w-full px-4 py-3 bg-red-500 hover:bg-red-600 text-white font-medium rounded-lg shadow-sm transition-all duration-200 flex items-center justify-center gap-3 cursor-pointer focus:outline-none focus:ring-2 focus:ring-red-300",
                    onclick: handle_cancel_picker,
                    span { class: "text-lg", "✕" }
                    span { "Cancel" }
                }
            }

            if picker_mode() {
                div { class: "p-3 bg-blue-50 border border-blue-200 rounded-lg",
                    p { class: "text-xs text-blue-700 text-center font-medium",
                        "🎯 Click anywhere on the map to place a marker"
                    }
                    p { class: "text-xs text-blue-600 text-center mt-1",
                        "Click Cancel to exit without adding"
                    }
                }
            }
        }
    }
}
