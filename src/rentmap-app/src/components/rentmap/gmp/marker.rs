use dioxus::prelude::*;

#[component]
pub fn GoogleMapMarker(lat: f64, lng: f64, title: String) -> Element {
    rsx! {
        gmp-advanced-marker { position: "{lat},{lng}", title: "{title}" }
    }
}
