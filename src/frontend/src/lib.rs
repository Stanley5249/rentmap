use app::App;
use tracing_subscriber::filter::LevelFilter;

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();

    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
