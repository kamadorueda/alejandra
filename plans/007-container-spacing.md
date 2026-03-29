# Expanded Configuration: Spacing in Containers

## Status
`ready`

## Priority
`P1-high`

## Context

The spacing debate is one of the longest-running discussions in Alejandra's history.
Currently, Alejandra formats inline lists and sets without spaces: `[x]`, `{a = 1;}`.
Many users (and the official nixfmt) prefer spaces: `[ x ]`, `{ a = 1; }`.

With v4.0.0 introducing the config system (`alejandra.toml`), this is now solvable
as a configuration option. Recent comments in #360 (March 2026) confirm ongoing demand:
*"I still prefer alejandra when it comes to other formatting decisions so also think
it would be awesome if there were an option to include spaces."*

**Issues**:
- [#360](https://github.com/kamadorueda/alejandra/issues/360) — Spacing in containers
- [#181](https://github.com/kamadorueda/alejandra/issues/181) — Add spaces around inline list brackets
- [#108](https://github.com/kamadorueda/alejandra/issues/108) — Enforcing spaces in containers

## Scope

**In scope:**
- New config option: `container_spacing` with values `"compact"` (default, current behavior)
  and `"spaced"` (`[ x ]`, `{ a = 1; }`)
- Apply to inline (single-line) lists, attribute sets, and function patterns
- Does NOT affect empty containers (`[]`, `{}` stay as-is regardless)
- Does NOT affect multi-line containers (they already have proper spacing)

**Out of scope:**
- Changing the default style (stays compact for backward compatibility)
- Spacing around function application, operators, etc.

## Implementation

### Step 1: Extend Config

**File**: `src/alejandra/src/config.rs`

```rust
#[derive(Clone, Copy, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    #[serde(default)]
    pub indentation: Indentation,

    #[serde(default)]
    pub container_spacing: ContainerSpacing,  // NEW
}

#[derive(Clone, Copy, Default, Deserialize)]
pub enum ContainerSpacing {
    #[default]
    Compact,
    Spaced,
}
```

### Step 2: Update list rule

**File**: `src/alejandra/src/rules/list.rs`

When rendering a single-line (non-vertical) list with at least one element,
add `Step::Whitespace` after `[` and before `]` if config is `Spaced`.

### Step 3: Update attr_set rule

**File**: `src/alejandra/src/rules/attr_set.rs`

Same logic for `{` and `}` in single-line attribute sets.

### Step 4: Update pattern rule

**File**: `src/alejandra/src/rules/pattern.rs`

Same logic for `{` and `}` in single-line function patterns.

### Step 5: Update alejandra.toml

**File**: `alejandra.toml`

```toml
# Indentation style: "TwoSpaces" (default), "FourSpaces", or "Tabs"
indentation = "TwoSpaces"

# Container spacing: "Compact" (default) or "Spaced"
# Compact: [x], {a = 1;}
# Spaced:  [ x ], { a = 1; }
container_spacing = "Compact"
```

### Step 6: Add test cases

**Directory**: `src/alejandra/tests/cases/container-spacing-spaced/`

Create test cases for lists, attr_sets, and patterns with `Spaced` config.
Mirror the structure of existing `indentation-*` test directories.

Add a new config entry in `tests/fmt.rs`:
```rust
("container-spacing-spaced", Config {
    container_spacing: ContainerSpacing::Spaced,
    ..Config::default()
}),
```

## Branch
`feat/container-spacing`

## Verification

1. `cargo test` — all tests pass
2. Default config produces identical output to current behavior (no regressions)
3. `Spaced` config adds spaces in inline containers
4. Empty containers `[]` and `{}` remain unchanged in both modes
5. Multi-line containers are unaffected by the setting

## Dependencies
None.
