use std::panic;

use wasm_bindgen::prelude::*;

use alejandra::config;
use alejandra::format::Status;

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

    let (status, after) = alejandra::format::in_memory(path.clone(), before, cfg);
    match status {
        Status::Changed(_) => Ok(after),
        Status::Error(error) => Err(format!("{}: {}", path, error)),
    }
}
#[cfg(test)]
mod tests {
    use super::format;

    #[test]
    fn format_propagates_invalid_nix_error_with_path() {
        let error = format("{ invalid".to_owned(), "example.nix".to_owned(), None).expect_err("invalid Nix should fail");
        assert!(error.starts_with("example.nix: "));
        assert!(error.contains("wanted any of"));
    }
}
