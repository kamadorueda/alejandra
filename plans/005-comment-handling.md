# Comment Handling Fixes

## Status
`ready`

## Priority
`P1-high`

## Context

Comment formatting bugs are the most *visible* kind of formatting issue. Users see their
carefully written comments mangled — extra newlines inserted, indentation lost, inline comments
misplaced. These bugs affect every Nix file with non-trivial comments.

**Issues**:
- [#407](https://github.com/kamadorueda/alejandra/issues/407) — Comments in inherits have extra newlines inserted
- [#383](https://github.com/kamadorueda/alejandra/issues/383) — Comment at top of function body missing indentation
- [#375](https://github.com/kamadorueda/alejandra/issues/375) — Multiple arguments with comments
- [#429](https://github.com/kamadorueda/alejandra/issues/429) — Short C-style comments not broken
- [#405](https://github.com/kamadorueda/alejandra/issues/405) — Short multi-line comments kept on one line

## Scope

**In scope:**
- Fix the 5 issues listed above
- Add regression tests for each fix

**Out of scope:**
- Comment content reformatting (e.g., reflowing long comments)
- Doc comment (`/** */`) improvements beyond bug fixes

## Implementation

### Understanding the Comment Pipeline

Comments flow through several stages:

1. **children.rs:91-101**: `TOKEN_COMMENT` tokens are dedented via `dedent_comment()`.
   Line comments (`# ...`) pass through unchanged. Block comments (`/* ... */`) are
   complex-processed: lines are realigned relative to the comment's column position.

2. **annotated_children.rs:33-42**: Comments are classified as either:
   - **Inline comments**: The first `#`-style comment after a non-trivia element (same line)
   - **Trivialities**: All other comments (own line, block comments, subsequent comments)

3. **Rule files**: Each rule handles comments via `drain_trivia()` or the annotated children
   system. The rule decides when to emit `Step::Comment(text)`, `Step::NewLine`, `Step::Pad`.

4. **builder.rs:59-84**: `Step::Comment` re-indents multi-line comments using `build_ctx.indentation`.

### Issue #407: Extra newlines in inherit comments

**File**: `src/alejandra/src/rules/inherit.rs`

**Problem**: In vertical mode, after processing a child's `trivialities`, the logic at
lines 76-98 can emit extra `NewLine + Pad` combinations. When a comment is followed by a
`Trivia::Newlines` entry, the comment handler skips emitting a newline (line 89: `continue`),
but the outer loop at line 94-97 then adds `NewLine + Pad` for `not_last_child`, resulting
in a blank line.

**Fix**: After the `continue` on line 89, the `Newlines` trivia is consumed on the next
iteration and falls through to lines 94-97 which unconditionally adds `NewLine + Pad`.
The fix is to not add `NewLine + Pad` after a `Trivia::Newlines` entry unless there's
actually more content following (not just the semicolon).

### Issue #383: Comment at top of function body missing indentation

**File**: `src/alejandra/src/rules/lambda.rs`

**Problem**: The lambda rule (lines 44-53) drains trivia after the `:` token. Comments
found here are emitted with `Step::Comment(text)` preceded by `NewLine + Pad`. However,
the indentation level at this point may not account for the function body indent.

**Fix**: Ensure `Step::Indent` is pushed before emitting the comment after `:`,
and `Step::Dedent` after, so the comment aligns with the function body.

### Issue #375: Multiple arguments with comments

**File**: `src/alejandra/src/rules/pattern.rs` and `src/alejandra/src/parsers/pattern.rs`

**Problem**: When function pattern arguments (`{ a, b, ... }:`) have comments between them,
the comments can be misplaced or cause formatting issues.

**Investigation needed**: Read `src/alejandra/src/rules/pattern.rs` and
`src/alejandra/src/parsers/pattern.rs` to understand how pattern entries with
interspersed comments are handled. The fix likely involves the trivia handling
in the pattern rule.

### Issue #429: Short C-style comments not broken

**Problem**: Short block comments like `/* foo */` get expanded to multi-line format
unnecessarily by `dedent_comment()` in `children.rs`.

**Fix**: In `dedent_comment()` (children.rs:197-305), add an early return for single-line
block comments:

```rust
fn dedent_comment(pos: &crate::position::Position, text: &str) -> String {
    if text.starts_with('#') {
        text.to_string()
    } else {
        // NEW: preserve short single-line block comments
        if !text.contains('\n') {
            return text.to_string();
        }
        // ... existing multi-line logic
    }
}
```

### Issue #405: Short multi-line comments kept on one line

This is the inverse of #429 — some multi-line comments that are short enough could be
kept on one line. This is a style preference rather than a bug. If addressed, it should
be in `dedent_comment()` with a check on total trimmed length.

### Step-by-step plan

1. Add test cases for each issue in `src/alejandra/tests/cases/default/`:
   - `comment_inherit/` for #407
   - `comment_lambda/` for #383
   - `comment_pattern/` for #375
   - `comment_block_short/` for #429

2. Create `in.nix` for each with the problematic input from the issues

3. Fix each issue in the relevant rule file

4. Run `UPDATE=1 cargo test` to generate `out.nix` files

5. Manually verify `out.nix` files look correct

6. Run `cargo test` to ensure all tests pass

## Branch
`fix/comment-handling`

## Verification

1. `cargo test` — all existing + new tests pass
2. Each issue's specific example produces correct output
3. No regressions in existing comment test cases (`tests/cases/default/comment/`)
4. Idempotency: formatting each new test case twice produces identical output

## Dependencies
None. (Independent of other plans, but benefits from 003-idempotency's test harness.)
