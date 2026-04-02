use std::panic;

use wasm_bindgen::prelude::*;

use alejandra::config;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    Ok(())
}

#[wasm_bindgen]
pub fn format(before: String, path: String, config_json: Option<String>) -> Result<String, String> {
    let cfg: config::Config = match config_json {
        Some(json) => serde_json::from_str(&json)
            .map_err(|e| format!("InvalidConfig: {}", e))?,
        None => Default::default(),
    };

    Ok(alejandra::format::in_memory(path, before, cfg).1)
}
