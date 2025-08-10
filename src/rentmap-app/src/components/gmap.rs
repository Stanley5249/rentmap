use dioxus::prelude::*;

#[component]
pub fn GoogleMap(center: (f64, f64), zoom: u8) -> Element {
    let center = format!("{},{}", center.0, center.1);

    rsx! {
        gmp-map { center, zoom, style: "height: 400px" }
    }
}
