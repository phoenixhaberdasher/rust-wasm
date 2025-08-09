use wasm_bindgen::prelude::*;
use console_error_panic_hook;

/// Called automatically when the WASM module loads
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    web_sys::console::log_1(&"🚀 WASM module initialized!".into());
}

/// Simple function exposed to JavaScript
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
