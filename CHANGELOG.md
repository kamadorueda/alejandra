# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

<!--
Types of changes
- Added for new features.
- Changed for changes in existing functionality.
- Deprecated for soon-to-be removed features.
- Removed for now removed features.
- Fixed for any bug fixes.
- Security in case of vulnerabilities.
-->

### Added

- A `--check` flag, which makes Alejandra emit an exit code of 2
  if any file was changed during formatting.

  This means you can now use Alejandra in your CI/CD
  to ensure your code complies the Alejandra style.

### Fixed

- Multiline strings are handled as utf-8 correctly, preventing panics
  on utf-8 whitespace like:

  ```nix
    ''
    foo
  \u{2002}bar
  ''
  ```

- All inputs and dependencies were updated to their latest version

## [0.5.0] - 2022-02-23

### Changed

- Pattern matching lambdas now always have the comma after the argument:

  ```diff
  -    depthLimit
  +    depthLimit,
      /*
        If this option is true, an error will be thrown, if a certain given depth is exceeded
        */
  -    ,
  ```

- Pattern matching lambdas now support inline comments:

  ```diff
  -  revision ? ""
  -  # Specify revision for the options
  +  revision ? "", # Specify revision for the options
  ```

- If-then-else expressions are indented only when necessary:

  ```diff
  -      then
  -        {
  -          crossDrv = overrideDerivation drv.crossDrv f;
  -          nativeDrv = overrideDerivation drv.nativeDrv f;
  -        }
  +      then {
  +        crossDrv = overrideDerivation drv.crossDrv f;
  +        nativeDrv = overrideDerivation drv.nativeDrv f;
  +      }
  ```

- All inputs and dependencies were updated to their latest version.

- A lot of code was refactored to improve maintainability.

## [0.4.0] - 2022-02-21

### Added

- A text user interface with a progress-bar
  and modern, colorful output (requires a tty).

  When no tty is available
  or in non-interactive environments like a CI/CD
  or when piping alejandra to other commands (`$ alejandra 2> file`, `$ alejandra | cat`)
  the old school program output will be used.

- A `--exclude` option to the CLI.

- Refactors to the codebase. We now comply with `clippy`, a Rust linter.

- A new structure to the codebase and link time optimizations.

  Binaries were reduced in size by 15%
  with respect to the previous release
  (even with the added features),
  performance was improved in the reference machine
  from 45 seconds to 35 while formatting Nixpkgs on a single core.

### Changed

- The old school program output is now less verbose.

  It prints only the path of files that were changed,
  and a summary of the number of errors and files changed during formatting.

### Removed

- The `--debug` flag in the CLI.

## [0.3.1] - 2022-02-20

### Added

- Prebuilt binaries for armv6l-linux, armv7l-linux, i686-linux.

### Changed

- Native aarch64-linux now use GNU libc instead of musl,
  this makes the binary run faster on multi-threaded hardware.

### Fixed

- All prebuilt binaries are now fully statically linked.
- An attempt to subtract with overflow in some scenarios of a `let-in` expression.

## [0.3.0] - 2022-02-18

### Changed

- Let-in expressions are now indented in the top-level of a file.
- Patterns avoid a new line after `@`:

  ```diff
  -        args @
  -        {
  +        args @ {
  ```

  ```diff
  -  }
  -  @ inp:
  +  } @ inp:
  ```

- Attribute sets no longer have spaces
  before the first element or after the last:

  ```diff
  - { b = 1; }
  + {b = 1;}
  ```

- Pattern matching lambdas no longer have spaces
  before the first or after the last element:

  ```diff
  - ({ ... }: _)
  + ({...}: _)
  ```

- Ellipsis is no longer count as an element when spreading pattern matching lambdas:

  ```diff
  -  {
  -    pkgs,
  -    ...
  -  }:
  +  {pkgs, ...}:
  ```

- Pattern matching lambdas now follow the equal sign:
  ```diff
  -  fnLocationRelative =
  -    {
  -      name,
  -      value,
  -    }:
  +  fnLocationRelative = {
  +    name,
  +    value,
  +  }:
  ```
- `with` expressions now indent the new scope and follow the equal sign:
  ```diff
  -    binPath =
  -      with pkgs;
  +    binPath = with pkgs;
  ```
- Nested lambdas are now not indented:

  ```diff
    # comment
    a:
  -   # comment
  -   b:
  -     _
  + # comment
  + b:
  +   _
  ```

- Brace-like elements after a pattern entry now follow the exclamation mark:

  ```diff
  -  rootPoolProperties ?
  -    {
  -      autoexpand = "on";
  -    },
  +  rootPoolProperties ? {
  +    autoexpand = "on";
  +  },
  ```

## [0.2.0] - 2022-02-17

### Added

- A `--version` flag to the CLI.
- Pre-built binaries for x86_64-linux and aarch64-linux.
- Support for inline comments on lists, attr-sets, and let-in expressions.

### Changed

- Made the logic of the `or-default` (`a or b`) node
  to be equal to the binary operator (`a $operator b`).
  This increases consistency across the same family of elements.
- Reduce 1 indentation level in `let-in` expressions,
  when the target expression is a parenthesis, attr-set, list, or string.
- String interpolations in multi-line strings
  now have a nice-looking indentation.

### Removed

- Users freedom to insert newlines
  before the `?` in pattern bindings (`a ? b`).

  Inserting a newline after the `?` is still possible.

  This increases consistency on where to break a long pattern binding.

- Space on empty containers (`[]`, `{}`).

### Fixed

- A bug in the current position counter
  caused a small percentage of multiline comments in Nixpkgs
  to be unaligned by one character.

## [0.1.0] - 2022-02-14

### Added

- Initial version
- A Changelog

## [0.0.0] - 2022-01-09

### Added

- The project :)

---

[unreleased]: https://github.com/kamadorueda/alejandra/compare/0.5.0...HEAD
[0.5.0]: https://github.com/kamadorueda/alejandra/compare/0.4.0...0.5.0
[0.4.0]: https://github.com/kamadorueda/alejandra/compare/0.3.1...0.4.0
[0.3.1]: https://github.com/kamadorueda/alejandra/compare/0.3.0...0.3.1
[0.3.0]: https://github.com/kamadorueda/alejandra/compare/0.2.0...0.3.0
[0.2.0]: https://github.com/kamadorueda/alejandra/compare/0.1.0...0.2.0
[0.1.0]: https://github.com/kamadorueda/alejandra/compare/0.0.0...0.1.0
[0.0.0]: https://github.com/kamadorueda/alejandra/compare/6adfbe8516bf6d9e896534e01118e1bc41f65425...0.0.0
