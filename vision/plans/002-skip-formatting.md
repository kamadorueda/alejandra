# Skip-Formatting Directives

## Status
`draft`

## Priority
`P0-critical`

## Context

The most-requested feature across Alejandra's issue tracker. Users need the ability
to disable formatting for specific code blocks — for hand-formatted tables, ASCII art,
workarounds for formatting bugs, or any code where the author's layout is intentional.

Every mature formatter supports this: Black (`# fmt: off/on`), Prettier (`// prettier-ignore`),
rustfmt (`#[rustfmt::skip]`), clang-format (`// clang-format off`).

Without this, users are forced to choose between Alejandra and their manual layout.
Many choose to leave.

**Issues**: [#292](https://github.com/kamadorueda/alejandra/issues/292),
[#418](https://github.com/kamadorueda/alejandra/issues/418),
[#463](https://github.com/kamadorueda/alejandra/issues/463)

## Scope

**In scope:**
- `# alejandra: off` / `# alejandra: on` block directives
- `# fmt: off` / `# fmt: on` as aliases (familiar from Black/Prettier)
- Preserve original whitespace verbatim within skipped regions
- Works in all contexts where Nix comments are valid

**Out of scope (for now):**
- Single-line ignore (e.g., `# alejandra: ignore-next-line`) — future follow-up
- File-level ignore via config — already possible via CLI excludes

## Implementation

*TODO: Needs investigation of the formatting pipeline. Key questions:*

1. Where in `src/alejandra/src/format.rs` does the tree walk happen?
2. Can we detect comments at the `rowan` syntax tree level before formatting a node?
3. Should we track off/on state as a stack (nested) or a simple boolean toggle?
4. How do we handle unclosed `# alejandra: off` (format to end of file? warn?)
5. What happens if `off` and `on` are at different nesting levels?

*Agent picking this up: start by reading `src/alejandra/src/format.rs`,
`src/alejandra/src/builder.rs`, and `src/alejandra/src/children.rs`
to understand the formatting pipeline before writing the implementation plan.*

## Branch
`feat/skip-formatting`

## Verification

1. Unit tests: code between `# alejandra: off` / `# alejandra: on` is preserved verbatim
2. Unit tests: `# fmt: off` / `# fmt: on` aliases work identically
3. Unit tests: unclosed `off` preserves rest of file
4. Unit tests: nested off/on (off inside off is a no-op)
5. Idempotency: formatting twice produces same output
6. Integration: format a file with mixed skip/format regions
7. Existing tests still pass

## Dependencies
None — this is independent of all other work items.
