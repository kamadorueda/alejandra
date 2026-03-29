# Multiline String Correctness

## Status
`ready`

## Priority
`P0-critical`

## Context

Alejandra's multiline string handling (`'' ... ''`) can modify the *semantic content* of strings,
not just their formatting. This is the most severe class of bug a formatter can have — the formatted
code behaves differently from the original. Users who discover this lose trust immediately.

**Issues**:
- [#409](https://github.com/kamadorueda/alejandra/issues/409) — alejandra modifies logical contents of multiline strings
- [#442](https://github.com/kamadorueda/alejandra/issues/442) — string escapes like `''${1+x}` should not be touched

## Scope

**In scope:**
- Audit `src/alejandra/src/rules/string.rs` for content-modifying behavior
- Fix any case where string semantics change after formatting
- Add regression tests for known-broken cases from issues

**Out of scope:**
- Indentation style within strings (cosmetic, not semantic) — separate concern
- String interpolation formatting improvements

## Implementation

### Architecture Understanding

The string rule (`src/alejandra/src/rules/string.rs`) handles two cases:
1. **Double-quoted strings** (`"..."`): Children are iterated and formatted. Straightforward.
2. **Indented strings** (`''...''`): Complex processing:
   - Interpolations (`${...}`) are replaced with a placeholder hash
   - Content is split by newlines
   - Lines are trimmed of trailing whitespace (when last line is whitespace-only)
   - Minimum indentation is computed and stripped ("dedent")
   - New indentation is applied based on `build_ctx.indentation` and config
   - Placeholders are replaced back with formatted interpolations

**Root causes of semantic changes:**

1. **Trailing whitespace trimming** (lines 50-62): The `should_trim_end` logic trims trailing
   whitespace from ALL lines if the last line is whitespace-only. This changes string content
   if trailing spaces are semantically significant.

2. **Dedent/re-indent** (lines 64-113): The algorithm strips the minimum common indentation
   then applies new indentation. For strings where indentation is meaningful (e.g., Makefiles,
   Python snippets, heredocs), this corrupts content.

3. **String escape handling**: Escape sequences like `''${...}` and `'''` inside indented strings
   may be mishandled during the split/rejoin process.

### Step 1: Add semantic preservation tests

**Directory**: `src/alejandra/tests/cases/default/string_semantic/`

Create `in.nix` and `out.nix` with test cases from the issues:

```nix
{
  # Trailing whitespace in multiline string should be preserved
  # when it's semantically significant
  makefileContent = ''
    target:
    \techo "hello"
  '';

  # String escapes must not be modified
  escaped = ''
    ''${1+x}
    '''
    ''''
  '';

  # Interpolation positions must be preserved exactly
  withInterp = ''
    before ${expr} after
    ${"complex"}
  '';
}
```

### Step 2: Evaluate current behavior

Run the test and capture actual output. Compare the Nix evaluation of `in.nix` and `out.nix`
to detect any semantic differences:

```bash
nix eval --file tests/cases/default/string_semantic/in.nix
nix eval --file tests/cases/default/string_semantic/out.nix
# Outputs must be identical
```

### Step 3: Fix the string rule

**File**: `src/alejandra/src/rules/string.rs`

Key fixes needed:

**3a. Trailing whitespace**: The `should_trim_end` heuristic (line 50-51) should ONLY trim
trailing whitespace that Nix's `''` string semantics would already strip. Nix's indented strings
strip the common leading indentation, but trailing whitespace on lines is preserved.
The formatter should not strip trailing whitespace from string content at all — only
surrounding formatting whitespace.

**3b. Dedent safety**: The dedent algorithm must match Nix's own indentation stripping exactly.
Nix strips the minimum indentation from all non-empty lines (excluding the first).
Alejandra must do the same — no more, no less.

**3c. Re-indent correctness**: When re-applying indentation (lines 92-113), the indentation
added must exactly correspond to `build_ctx.indentation` levels so that the Nix evaluator
sees the same final string value.

### Step 4: Add evaluation-based test

Add a test that evaluates both `in.nix` and `out.nix` with Nix and asserts identical results.
This could be a separate integration test or a shell script in CI:

```bash
#!/usr/bin/env bash
set -e
for case in tests/cases/default/string_semantic; do
  before=$(nix eval --raw --file "$case/in.nix" 2>/dev/null || echo "SKIP")
  after=$(nix eval --raw --file "$case/out.nix" 2>/dev/null || echo "SKIP")
  if [ "$before" != "$after" ] && [ "$before" != "SKIP" ]; then
    echo "SEMANTIC CHANGE in $case"
    diff <(echo "$before") <(echo "$after")
    exit 1
  fi
done
```

## Branch
`fix/multiline-string-correctness`

## Verification

1. `cargo test` — all tests pass
2. New `string_semantic` test case passes
3. Nix evaluation of `in.nix` and `out.nix` produces identical values
4. Idempotency: formatting the output file again produces no changes
5. Existing string test cases still pass (no regressions)

## Dependencies
None. (But plan 003-idempotency's test harness improvement will help catch regressions.)
