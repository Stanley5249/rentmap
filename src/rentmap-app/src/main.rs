mod components;

use components::gmap::GoogleMap;
use dioxus::document::{Link, Script};
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

static FAVICON: Asset = asset!("/assets/favicon.ico");

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
        Link { rel: "icon", href: FAVICON }

        h1 { "Dioxus Google Maps Integration" }
        p { "This example demonstrates a robust, idiomatic way to embed multiple Google Maps." }
        p {
            small { "Currently showing maps for Tainan, Taiwan and Tokyo, Japan." }
        }
        GoogleMap { center: (22.9971, 120.2126), zoom: 12 }
        GoogleMap { center: (35.6762, 139.6503), zoom: 14 }

        match state {
            Some(Ok(state)) => rsx! {
                Script { src: "https://maps.googleapis.com/maps/api/js?key={state.api_key}&libraries=maps&v=weekly&loading=async" }
            },
            Some(Err(e)) => rsx! {
                p { "Error fetching state {e}" }
            },
            None => rsx! {
                p { "Loading..." }
            },
        }
    }
}
