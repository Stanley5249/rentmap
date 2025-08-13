use dioxus::prelude::*;
use tracing::debug;

use super::marker::GoogleMapMarker;
use crate::components::rentmap::model::MarkerData;

#[component]
pub fn GoogleMap(
    center: (f64, f64),
    zoom: u8,
    markers: Signal<std::collections::BTreeMap<u32, MarkerData>>,
    picker_mode: Signal<bool>,
    on_map_click: EventHandler<(f64, f64)>,
) -> Element {
    let (lat, lng) = center;

    let handle_map_click = move |_| {
        if picker_mode() {
            debug!("Map clicked in picker mode, using center coordinates as placeholder");
            on_map_click.call((lat, lng));
        }
    };

    rsx! {
        gmp-map {
            center: "{lat},{lng}",
            zoom,
            "map-id": "ffcd8a40917c8b96c20e9d5d",
            style: "height: 100%; width: 100%; display: block;",
            onclick: handle_map_click,
            for (id , marker) in markers.read().iter() {
                GoogleMapMarker {
                    key: "{id}",
                    lat: marker.lat,
                    lng: marker.lng,
                    title: marker.title.clone(),
                }
            }
        }
    }
}
