use std::panic;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    Ok(())
}

#[wasm_bindgen]
pub fn format(before: String, path: String) -> String {
    alejandra::format::in_memory(path, before, Default::default()).1
}
