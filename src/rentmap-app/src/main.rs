mod components;

use components::document::DocumentMeta;
use components::header::Header;
use components::icons::SpinnerIcon;
use components::rentmap::RentMap;
use dioxus::document::Script;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info};

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

    let state_cached = use_server_cached(|| {
        let result = state();

        match &result {
            Some(Ok(_)) => info!("Load Google Maps API state"),
            Some(Err(e)) => error!(error = %e, "Fail to load Google Maps API state"),
            None => debug!("Load Google Maps API state..."),
        }

        result
    });

    rsx! {
        DocumentMeta {}

        div { class: "h-screen bg-gray-50 flex flex-col",
            div { class: "flex-shrink-0",
                Header { property_count: 1247, geocoded_count: 892 }
            }

            main { class: "flex-1 max-w-7xl mx-auto px-6 py-4 w-full min-h-0",
                div { class: "w-full h-full rounded-lg overflow-hidden border border-gray-300 relative flex",
                    RentMap { center: (25.0330, 121.5654), zoom: 11 }
                }
            }
        }

        match &state_cached {
            Some(Ok(state)) => rsx! {
                Script { src: "https://maps.googleapis.com/maps/api/js?key={state.api_key}&libraries=maps,marker&v=weekly&loading=async" }
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
