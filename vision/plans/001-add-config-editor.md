# Add Configuration Editor to Alejandra Frontend

## Status
`ready`

## Priority
`P1-high`

## Context

Alejandra supports a `Config` struct with an `Indentation` enum (`TwoSpaces`, `FourSpaces`, `Tabs`). The WASM bridge currently hardcodes `Default::default()`, ignoring config. The frontend has no way for users to select formatting options. We need to:
1. Update the WASM crate (on `main` branch) to accept a full JSON config
2. Update the frontend (on `front` branch, feature branch `claude/add-config-editor-Kbxnx`) to expose a config editor UI

## Branch Structure

- **`main`** — Rust source code (alejandra lib, alejandra_cli, alejandra_wasm)
- **`front`** — Frontend React/TypeScript app (this working directory)
- **`claude/add-config-editor-Kbxnx`** — Feature branch off `front` for this work

## Part 1: WASM Changes (on `main` branch via GitHub)

### File: `src/alejandra_wasm/Cargo.toml`
- Add `serde_json` dependency

### File: `src/alejandra_wasm/src/lib.rs`

Modify the `format` function to accept a JSON config string and deserialize it into the `Config` struct. This is future-proof — as new config fields are added to the Rust `Config` struct, the WASM interface doesn't need to change.

```rust
#[wasm_bindgen]
pub fn format(before: String, path: String, config_json: String) -> String {
    let config: alejandra::config::Config = serde_json::from_str(&config_json)
        .unwrap_or_default();
    alejandra::format::in_memory(path, before, config).1
}
```

**Note:** Since we can't rebuild WASM in this environment, the frontend will include a post-processing fallback so the feature works even with the old WASM binary (2-arg `format`).

## Part 2: Frontend Changes (on `claude/add-config-editor-Kbxnx` branch)

### 1. New file: `src/types/config.ts`
- Define `Indentation` type: `"TwoSpaces" | "FourSpaces" | "Tabs"`
- Define `FormatterConfig` interface with `indentation` field
- Export `DEFAULT_CONFIG` constant

### 2. New file: `src/utils/indentation.ts`
- Pure function `applyIndentation(code: string, indentation: Indentation): string`
- Transforms leading 2-space indentation in each line to the target format
- Used as post-processing fallback when the WASM binary doesn't support the config parameter

### 3. New file: `src/utils/indentation.test.ts`
- Tests for TwoSpaces (no-op), FourSpaces, Tabs conversions
- Edge cases: empty lines, no indentation, inline spaces preserved

### 4. Modify: `src/types/wasm.d.ts`
- Update `format` signature to accept optional third JSON config string:
  ```typescript
  export function format(code: string, filename: string, configJson?: string): string;
  ```

### 5. Modify: `src/utils/wasm.ts`
- Update `formatCode` to accept a `FormatterConfig`, serialize to JSON, pass to WASM `format`
- Try calling with 3 args; if the old WASM binary ignores it, fall back to post-processing via `applyIndentation`

### 6. Modify: `src/hooks/useFormatter.ts`
- Add `config` state (`useState<FormatterConfig>`)
- Pass config through to `formatCode`
- Re-format when config changes (store raw WASM output, re-apply config transform)
- Expose `config` and `handleConfigChange` from the hook

### 7. Modify: `src/utils/permalink.ts`
- Extend `PermalinkState` to include optional `config?: FormatterConfig`
- Old URLs without config decode normally (backward compatible)

### 8. New file: `src/components/ConfigPanel/index.tsx`
- Compact component with a labeled `<select>` dropdown for Indentation
- Options: "2 Spaces" (default), "4 Spaces", "Tabs"
- Props: `config`, `onChange`
- Styled to match existing design (Tailwind, `text-sm`, etc.)

### 9. New file: `src/components/ConfigPanel/index.test.tsx`
- Tests: renders with correct default, calls onChange, shows all options

### 10. Modify: `src/components/SideBySide/index.tsx`
- Import and render `ConfigPanel` in the instruction bar area
- Wire `config` and `handleConfigChange` from `useFormatter`
- Layout: flex row with instruction text on left, config panel on right

### 11. Modify: `src/components/SideBySide/index.test.tsx`
- Add tests for ConfigPanel rendering in SideBySide

## Verification

1. Run `pnpm test` to ensure all tests pass (existing + new)
2. Check that the config dropdown renders in the instruction bar
3. Verify that changing indentation updates the formatted output in real-time
4. Verify permalink encodes/decodes config correctly
5. Verify backward compatibility with old permalink URLs (no config field)
