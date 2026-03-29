# Publish to crates.io

## Status
`ready`

## Priority
`P1-high`

## Context

Rust developers expect `cargo install alejandra` to work. Currently, the `alejandra` library
crate has proper metadata but has never been published. The CLI crate (`alejandra_cli`) uses
a path dependency on the library, which must be converted to a versioned dependency for publishing.

This is a low-effort, high-impact change that removes an adoption barrier for the entire
Rust ecosystem.

**Issues**: [#449](https://github.com/kamadorueda/alejandra/issues/449)

## Scope

**In scope:**
- Prepare both crates for crates.io publishing
- Fix any metadata issues
- Document the publishing process for maintainers

**Out of scope:**
- Automated publishing in CI (follow-up work)
- API stability guarantees beyond what semver already provides

## Implementation

### Step 1: Audit crate metadata

**File**: `src/alejandra/Cargo.toml`

Current state (mostly complete):
```toml
[package]
authors = ["Kevin Amado <kamadorueda@gmail.com>"]
description = "The Uncompromising Nix Code Formatter"
edition = "2021"
license = "Unlicense"
name = "alejandra"
readme = "../../README.md"
repository = "https://github.com/kamadorueda/alejandra"
version = "4.0.0"
```

Missing fields to add:
```toml
homepage = "https://github.com/kamadorueda/alejandra"
keywords = ["nix", "formatter", "code-formatter", "nixos"]
categories = ["command-line-utilities", "development-tools"]
```

### Step 2: Fix CLI crate dependency

**File**: `src/alejandra_cli/Cargo.toml`

Current:
```toml
alejandra = { path = "../alejandra" }
```

Must change to include version for publishing:
```toml
alejandra = { path = "../alejandra", version = "4.0.0" }
```

Also add missing metadata:
```toml
homepage = "https://github.com/kamadorueda/alejandra"
keywords = ["nix", "formatter", "code-formatter", "nixos"]
categories = ["command-line-utilities", "development-tools"]
readme = "../../README.md"
```

### Step 3: Verify wildcard dependencies

Several dependencies use `version = "*"` which crates.io rejects. These must be
pinned to actual version ranges:

In `src/alejandra/Cargo.toml`:
```toml
rnix = { version = "*" }     # → pin to "0.11" or whatever's in Cargo.lock
serde = { version = "*" }    # → pin to "1"
```

In `src/alejandra_cli/Cargo.toml`:
```toml
clap = { version = "*" }     # → pin to "4"
futures = { version = "*" }  # → pin to "0.3"
num_cpus = { version = "*" } # → pin to "1"
rand = { version = "*" }     # → pin to "0.8"
toml = { version = "*" }     # → pin to "0.8"
walkdir = { version = "*" }  # → pin to "2"
```

Check `Cargo.lock` for exact versions currently resolved and use compatible ranges.

### Step 4: Dry-run publish

```bash
cd src/alejandra && cargo publish --dry-run
cd src/alejandra_cli && cargo publish --dry-run
```

Fix any errors reported.

### Step 5: Publish

```bash
# Library first (CLI depends on it)
cd src/alejandra && cargo publish
# Wait for it to appear on crates.io, then:
cd src/alejandra_cli && cargo publish
```

Note: This requires crates.io authentication. The agent should prepare everything
and leave the actual `cargo publish` for the maintainer.

## Branch
`feat/crates-io-publishing`

## Verification

1. `cargo publish --dry-run` succeeds for both crates
2. `cargo install --path src/alejandra_cli` works locally
3. All tests still pass
4. No `version = "*"` dependencies remain

## Dependencies
None.
