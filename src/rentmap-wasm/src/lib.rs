use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["google", "maps"], js_name = Map)]
    pub type GoogleMap;

    #[wasm_bindgen(constructor, js_namespace = ["google", "maps"], js_name = Map)]
    pub fn new(element: &HtmlElement, options: &JsValue) -> GoogleMap;

    #[wasm_bindgen(method, js_name = addListener)]
    pub fn add_listener(this: &GoogleMap, event_name: &str, handler: &js_sys::Function);

    #[wasm_bindgen(js_namespace = ["google", "maps"], js_name = Marker)]
    pub type GoogleMapsMarker;

    #[wasm_bindgen(constructor, js_namespace = ["google", "maps"], js_name = Marker)]
    pub fn new_marker(options: &JsValue) -> GoogleMapsMarker;

    #[wasm_bindgen(js_namespace = ["google", "maps"], js_name = LatLng)]
    pub type LatLng;

    #[wasm_bindgen(constructor, js_namespace = ["google", "maps"], js_name = LatLng)]
    pub fn new_latlng(lat: f64, lng: f64) -> LatLng;
}

// High-level wrapper functions for easier use
#[wasm_bindgen]
pub fn add_marker_to_map(
    map: &GoogleMap,
    lat: f64,
    lng: f64,
    title: Option<String>,
) -> GoogleMapsMarker {
    let position = LatLng::new_latlng(lat, lng);
    let options = js_sys::Object::new();

    js_sys::Reflect::set(&options, &"position".into(), &position).unwrap();
    js_sys::Reflect::set(&options, &"map".into(), map).unwrap();

    if let Some(title) = title {
        js_sys::Reflect::set(&options, &"title".into(), &title.into()).unwrap();
    }

    GoogleMapsMarker::new_marker(&options.into())
}

#[wasm_bindgen]
pub fn add_click_listener(map: &GoogleMap, callback: &js_sys::Function) {
    map.add_listener("click", callback);
}
