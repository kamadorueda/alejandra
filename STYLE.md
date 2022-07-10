# Alejandra's Style Guide

Alejandra's mission is to produce a **consistent style**
that is **easy to read**
and produces **clean diffs**.
This means trading aggressively compact code
for regularity and ease of modification.

## Function

### With Destructured Arguments

✅ Good:

```nix
{mkDerivation, ...} @ attrs:
  mkDerivation # ...
```

- Indenting the body relative to the function signature
  hints that a new scope is introduced by the
  function arguments.
- Keeping the signature in one line
  when there is only 1 argument in the destructuring (`mkDerivation`)
  helps saving vertical space.
- Spacing between elements of the destructuring,
  and between opening and closing elements
  is consistent with _List_ and _Map_.

✅ Good:

```nix
{mkDerivation, ...} @ attrs:
mkDerivation # ...
```

- When there is only 1 function in the whole file
  it's valid not to indent the body
  because it's clear when reading the file from top to bottom
  that the whole remaining of the file
  is the scope of the function,
  Therefore saving an unneeded indent.

✅ Good:

```nix
{
  mkDerivation,
  lib,
  fetchurl,
  ...
} @ attrs:
  stdenv.mkDerivation # ...
```

- Adding an argument produces a minimal diff
  (including the first and last elements):

  ```patch
    mkDerivation,
    lib,
    fetchurl,
  + google-chrome-stable,
  ```

- Removing an argument produces a minimal diff
  (including the first and last elements):

  ```patch
    mkDerivation,
  - lib,
    fetchurl,
  ```

- The comma at the end is consistent with _Let-In_, and _Map_,
  where the separator goes after the element
  instead of at the beginning.

❌ Bad:

<!-- nixpkgs-fmt -->

```nix
{ lib
, mkDerivation
, fetchurl
, ...
} @ attrs:
stdenv.mkDerivation # ...
```

- Removing the first element
  produces a diff in two elements:

  ```diff
  - { lib
  - , mkDerivation
  + { mkDerivation
    , fetchurl
    , ...
    } @ attrs:
    stdenv.mkDerivation # ...
  ```

- Documenting the first argument creates an inconsistency
  between the way argument start:

  ```nix
  {
    # Lorem Ipsum
    lib
  , mkDerivation
  , fetchurl
  , ...
  } @ attrs:
  stdenv.mkDerivation # ...
  ```

- This is not consistent with _Let-In_, and _Map_,
  where the separator goes after the element
  instead of at the beginning.

❌ Bad:

<!-- nixfmt -->

```nix
{ mkDerivation, lib, fetchurl, ... }@attrs: stdenv.mkDerivation # ...
```

- One-liners are unreadable.

❌ Bad:

<!-- nixfmt -->

```nix
{ mkDerivation, lib, fetchurl, extra-cmake-modules, kdoctools, wrapGAppsHook
, karchive, kconfig, kcrash, kguiaddons, kinit, kparts, kwind, ... }@attrs:
stdenv.mkDerivation # ...
```

- It's hard to tell this destructuring has an ellipsis (`...`) at a first glance,
  because it's mixed with the other arguments.
- Moving elements becomes harder
  than a simple whole-line movement.
  (Moving a whole line is normally a keyboard-shortcut
  or command in major code editors).
- Excessively compact:
  adding, removing, or editing an argument
  produces a diff in more than one argument.
- `}@attrs` is not intuitive
  with the rules of written english,
  where you add whitespace
  after the end of the previous phrase
  (`phrase. Other phrase`).

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
