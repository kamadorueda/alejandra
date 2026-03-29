# Work Items Backlog

Ordered by priority, then by impact within each tier.
Agents: pick the first `ready` item you can handle.

---

## P0 — Critical (ship or lose users)

| # | Item | Plan | Status | Issues | Notes |
|---|---|---|---|---|---|
| 1 | Skip-formatting directives (`# alejandra: off/on`) | `plans/002-skip-formatting.md` | ready | [#292](https://github.com/kamadorueda/alejandra/issues/292), [#418](https://github.com/kamadorueda/alejandra/issues/418), [#463](https://github.com/kamadorueda/alejandra/issues/463) | #1 most-requested feature. Every mature formatter has this. |
| 2 | Idempotency guarantee | `plans/003-idempotency.md` | ready | [#250](https://github.com/kamadorueda/alejandra/issues/250), [#408](https://github.com/kamadorueda/alejandra/issues/408) | Format twice → same output. Add CI check. |
| 3 | Multiline string correctness | `plans/004-multiline-string-correctness.md` | ready | [#409](https://github.com/kamadorueda/alejandra/issues/409), [#442](https://github.com/kamadorueda/alejandra/issues/442) | Formatter must never change code semantics. |

## P1 — High (key differentiators)

| # | Item | Plan | Status | Issues | Notes |
|---|---|---|---|---|---|
| 4 | Comment handling fixes | `plans/005-comment-handling.md` | ready | [#407](https://github.com/kamadorueda/alejandra/issues/407), [#383](https://github.com/kamadorueda/alejandra/issues/383), [#375](https://github.com/kamadorueda/alejandra/issues/375), [#429](https://github.com/kamadorueda/alejandra/issues/429) | Most visible formatting bugs. |
| 5 | Container spacing config (`[ x ]` vs `[x]`) | `plans/007-container-spacing.md` | ready | [#360](https://github.com/kamadorueda/alejandra/issues/360), [#181](https://github.com/kamadorueda/alejandra/issues/181), [#108](https://github.com/kamadorueda/alejandra/issues/108) | Configurable spacing in containers. |
| 6 | Frontend config editor (WASM + UI) | `plans/001-add-config-editor.md` | ready | — | Config panel for web playground. |
| 7 | Publish to crates.io | `plans/006-crates-io-publishing.md` | ready | [#449](https://github.com/kamadorueda/alejandra/issues/449) | `cargo install alejandra` should work. |
| 8 | Line length / expression breaking improvements | — | needs-plan | [#349](https://github.com/kamadorueda/alejandra/issues/349), [#331](https://github.com/kamadorueda/alejandra/issues/331), [#462](https://github.com/kamadorueda/alejandra/issues/462) | Line 80 limit not enforced in several contexts. |

## P2 — Medium (grow the market)

| # | Item | Plan | Status | Issues | Notes |
|---|---|---|---|---|---|
| 9 | RFC 166 compatibility mode | — | needs-plan | [#404](https://github.com/kamadorueda/alejandra/issues/404) | Optional preset: `style = "rfc166"` in alejandra.toml. |
| 10 | Windows support | — | needs-plan | [#430](https://github.com/kamadorueda/alejandra/issues/430) | Expand platform reach. |
| 11 | .gitignore support | — | needs-plan | [#428](https://github.com/kamadorueda/alejandra/issues/428) | Expected behavior for any file-processing CLI tool. |
| 12 | Attribute sorting (opt-in) | — | needs-plan | [#276](https://github.com/kamadorueda/alejandra/issues/276) | Power feature nixfmt doesn't have. |
| 13 | macOS binary/package fixes | — | needs-plan | [#470](https://github.com/kamadorueda/alejandra/issues/470) | Broken defaultPackage reference. |
| 14 | Let-in expression formatting | — | needs-plan | [#257](https://github.com/kamadorueda/alejandra/issues/257) | 9 comments, long-standing style issue. |

## P3 — Low (long-term vision)

| # | Item | Plan | Status | Issues | Notes |
|---|---|---|---|---|---|
| 15 | Diff-aware formatting (`--diff HEAD`) | — | needs-plan | — | Format only changed lines for incremental adoption. |
| 16 | Migration tooling (`--from nixfmt`) | — | needs-plan | — | Zero-cost switching from other formatters. |
| 17 | Community style presets | — | needs-plan | — | Named presets: default, compact, rfc166, nixpkgs-classic. |
| 18 | GitHub Action (`alejandra-action`) | — | needs-plan | — | One-line CI integration. |
| 19 | Stable Rust API / library documentation | — | needs-plan | [#306](https://github.com/kamadorueda/alejandra/issues/306) | Embeddability story for third-party tools. |

---

## Status Legend

| Status | Meaning |
|---|---|
| `needs-plan` | Work item identified but no implementation plan written yet |
| `draft` | Plan exists but incomplete or under review |
| `ready` | Plan is complete, agent can pick it up |
| `in-progress` | An agent is actively working on this |
| `blocked` | Waiting on a dependency or external input |
| `done` | Shipped and verified |
