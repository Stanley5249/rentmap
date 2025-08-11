use dioxus::prelude::*;

#[component]
pub fn GoogleMap(center: (f64, f64), zoom: u8, class: Option<String>) -> Element {
    let center = format!("{},{}", center.0, center.1);
    let container_class = format!(
        "rounded-lg overflow-hidden border border-gray-300 {}",
        class.unwrap_or_default()
    );

    rsx! {
        div { class: "{container_class}",
            gmp-map { center, zoom, style: "height: 100%; width: 100%;" }
        }
    }
}
