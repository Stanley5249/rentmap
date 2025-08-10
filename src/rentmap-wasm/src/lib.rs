use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["google", "maps"], js_name = Map)]
    pub type GoogleMap;

    #[wasm_bindgen(constructor, js_namespace = ["google", "maps"], js_name = Map)]
    pub fn new(element: &HtmlElement, options: &JsValue) -> GoogleMap;
}
