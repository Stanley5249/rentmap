mod components;

use components::document::DocumentMeta;
use components::gmap::GoogleMap;
use components::header::Header;
use components::icons::SpinnerIcon;
use dioxus::document::Script;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct State {
    pub api_key: String,
}

impl State {
    #[cfg(feature = "server")]
    pub fn from_args() -> Self {
        let args = rentmap::app::get_args().expect("Failed to get args");
        Self {
            api_key: args.google.get_api_key().unwrap(),
        }
    }
}

fn main() {
    dioxus::LaunchBuilder::new()
        .with_context(server_only! { State::from_args() })
        .launch(App);
}

#[server]
pub async fn get_server_state() -> Result<State, ServerFnError> {
    let FromContext(value) = extract().await?;
    Ok(value)
}

#[component]
pub fn App() -> Element {
    let state = use_server_future(get_server_state)?;
    let state = use_server_cached(|| state());

    rsx! {
        // Document metadata
        DocumentMeta {}
        // Main application container
        div { class: "h-screen bg-gray-50 flex flex-col",
            // Header component
            Header {
                property_count: 1247,
                geocoded_count: 892,
                title: None,
                subtitle: None,
            }

            // Main content - takes remaining space
            main { class: "flex-1 max-w-7xl mx-auto px-6 py-4 w-full",
                GoogleMap {
                    center: (25.0330, 121.5654),
                    zoom: 11,
                    class: "w-full h-full",
                }
            }
        }

        // Google Maps API Script
        match state {
            Some(Ok(state)) => rsx! {
                Script { src: "https://maps.googleapis.com/maps/api/js?key={state.api_key}&libraries=maps&v=weekly&loading=async" }
            },
            Some(Err(e)) => rsx! {
                div { class: "fixed top-4 right-4 bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded-lg shadow-lg",
                    strong { "Error: " }
                    "Failed to load Google Maps API - {e}"
                }
            },
            None => rsx! {
                div { class: "fixed top-4 right-4 bg-blue-100 border border-blue-400 text-blue-700 px-4 py-3 rounded-lg shadow-lg",
                    div { class: "flex items-center space-x-2",
                        SpinnerIcon { class: "animate-spin h-4 w-4" }
                        span { "Loading Google Maps..." }
                    }
                }
            },
        }
    }
}
