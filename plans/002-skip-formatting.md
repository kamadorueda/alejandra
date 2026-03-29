# Skip-Formatting Directives

## Status
`ready`

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
- Preserve original whitespace/formatting verbatim within skipped regions
- Works in all contexts where Nix line comments (`#`) are valid

**Out of scope:**
- Single-line ignore (e.g., `# alejandra: ignore-next-line`) — future follow-up
- File-level ignore via config — already possible via CLI excludes
- Block comments (`/* alejandra: off */`) — line comments only for v1

## Implementation

### Architecture Overview

The formatting pipeline works as follows:
1. `format::in_memory()` parses with `rnix::Root::parse()`, creates a `BuildCtx`, calls `builder::build()`
2. `builder::build()` calls `build_step()` with `Step::Format(element)`
3. `build_step()` dispatches to `format()` which matches on `SyntaxKind` and selects a rule function
4. Each rule function returns a `LinkedList<Step>` which `build_step` processes recursively
5. Comments are `TOKEN_COMMENT` nodes in the rnix syntax tree

The key interception point is in `builder::format()` (builder.rs:134-268). When processing
a `SyntaxElement::Token`, we can detect skip directives. When processing a `SyntaxElement::Node`,
if skip is active, we bypass the formatting rules and emit the original tree verbatim.

### Step 1: Add skip state to BuildCtx

**File**: `src/alejandra/src/builder.rs`

Add a `skip_formatting: bool` field to `BuildCtx`:

```rust
pub(crate) struct BuildCtx {
    pub config:                       Config,
    pub fitting_in_single_line_depth: usize,
    pub force_wide:                   bool,
    pub force_wide_success:           bool,
    pub indentation:                  usize,
    pub path:                         String,
    pub pos_old:                      crate::position::Position,
    pub skip_formatting:              bool,  // NEW
    pub vertical:                     bool,
}
```

Initialize to `false` in `format::in_memory()`.

### Step 2: Add directive detection helpers

**File**: `src/alejandra/src/builder.rs` (or a new `src/alejandra/src/skip.rs`)

```rust
fn is_skip_off(text: &str) -> bool {
    let trimmed = text.trim_start_matches('#').trim();
    trimmed == "alejandra: off" || trimmed == "fmt: off"
}

fn is_skip_on(text: &str) -> bool {
    let trimmed = text.trim_start_matches('#').trim();
    trimmed == "alejandra: on" || trimmed == "fmt: on"
}
```

### Step 3: Intercept in the format function

**File**: `src/alejandra/src/builder.rs`, in the `format()` function

**For tokens** (line 262-267): Check comments for directives and toggle skip state:

```rust
rnix::SyntaxElement::Token(token) => {
    let text = token.text();

    // Check for skip directives in comments
    if kind == rnix::SyntaxKind::TOKEN_COMMENT {
        if is_skip_on(text) {
            build_ctx.skip_formatting = false;
        } else if is_skip_off(text) {
            build_ctx.skip_formatting = true;
        }
    }

    add_token(builder, kind, text);
    build_ctx.pos_old.update(text);
}
```

**For nodes** (line 142-259): Add early return when skip is active. Instead of
dispatching to a formatting rule, walk the original syntax tree children verbatim:

```rust
rnix::SyntaxElement::Node(node) => {
    builder.start_node(rowan::SyntaxKind(kind as u16));

    if build_ctx.skip_formatting {
        // Emit all children verbatim, but still check for "on" directives
        for child in node.children_with_tokens() {
            format(builder, build_ctx, &child);
        }
    } else {
        let rule = match kind { /* existing match */ };
        for step in rule(build_ctx, node) {
            build_step(builder, build_ctx, &step);
        }
    }

    builder.finish_node();
}
```

This recursive approach ensures that `TOKEN_COMMENT` tokens inside the skipped region
are still checked for `# alejandra: on` directives, allowing re-enabling mid-node.

### Step 4: Handle the Comment step in skip mode

In `build_step()`, when processing `Step::Comment(text)`, skip the re-indentation logic
if `build_ctx.skip_formatting` is true — emit the comment text as-is.

### Step 5: Handle whitespace preservation

In `Children::new()` (children.rs:103-107), whitespace tokens without newlines are
currently discarded. This is fine because when skip is active (Step 3), we bypass
`Children` entirely and iterate `node.children_with_tokens()` directly, which includes
all original whitespace.

### Step 6: Add test cases

**Directory**: `src/alejandra/tests/cases/default/skip_formatting/`

`in.nix`:
```nix
{
  formatted = true;

  # alejandra: off
  hand_formatted = {
    a   = 1;
    bb  = 2;
    ccc = 3;
  };
  # alejandra: on

  also_formatted = true;

  # fmt: off
  also_preserved = [
    1   2   3
    4   5   6
  ];
  # fmt: on

  back_to_normal = true;
}
```

`out.nix`: Same content — regions between off/on are preserved verbatim,
rest is formatted normally.

Additional test: unclosed `# alejandra: off` preserves to end of file.

### Step 7: Edge case — directives inside strings

A `# alejandra: off` inside a string literal must NOT be treated as a directive.
This is handled naturally: string content is `TOKEN_STRING_CONTENT`, not `TOKEN_COMMENT`.
Only actual `TOKEN_COMMENT` nodes (which cannot appear inside strings in valid Nix) trigger
the directive check.

## Branch
`feat/skip-formatting`

## Verification

1. `cargo test` — all existing tests pass
2. New test: code between `# alejandra: off` / `# alejandra: on` is preserved verbatim
3. New test: `# fmt: off` / `# fmt: on` aliases work identically
4. New test: unclosed `off` preserves rest of file
5. New test: nested off/on (second `off` inside `off` is a no-op, first `on` resumes)
6. Idempotency: format the test file twice, assert identical output
7. Edge case: `# alejandra: off` inside a string is NOT treated as a directive

## Dependencies
None.
