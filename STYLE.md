# Alejandra's Style Guide

Alejandra's mission is to produce a **consistent style**
that is **easy to read**
and produces **clean diffs**.
This means trading aggressively compact code
for regularity and ease of modification.

## If-Then-Else

✅ Good:

```nix
if predicate
then foo
else bar
```

- The keyword at the beginning of the line
  states clearly the meaning of the content that follows.
- Produces a clean diff when you add more code.
  For example: adding content to the `else`
  only produces a diff in the `else`.

❌ Bad:

<!-- nixfmt -->

```nix
if predicate then foo else bar
```

- One-liners are hard to understand,
  specially when nested,
  or when logic gets long.
- Adding content produces a diff in the entire `if-then-else`.

✅ Good:

```nix
if something <= 2.0
then
  if somethingElse
  then foo
  else bar
else if something <= 4.0
then baz
else if something <= 6.0
then foo
else bar
```

- It's easy to follow that there are many conditionals.
- The indentation makes it easy to read
  which expression is associated to each conditional.
- Adding or modifying the branches produces a clean diff.

❌ Bad:

<!-- nixpkgs-fmt -->

```nix
  if cond
  then if
    looooooooooooooooooooooooooooooooooooooooooooooooooooong
  then foo
  else bar
  else if cond
  then foo
  else bar
```

- It's complex to distinct the parent `if-then-else`
  from the child `if-then-else`
