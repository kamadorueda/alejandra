use std::panic;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    Ok(())
}

#[wasm_bindgen]
pub fn format(before: String, path: String) -> String {
    let config = alejandra_engine::config::Config::default();

    alejandra_engine::format::string_or_passthrough(&config, path, before)
}
