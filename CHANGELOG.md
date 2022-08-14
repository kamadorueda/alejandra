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

## [3.0.0] - 2022-08-14

### Added

- Different levels of --quiet by repeating the `-q` flag,
  which makes Vim users have a better formatting experience by using `:%!alejandra -qq` to format the current buffer,
  specially when the file has syntax errors.
- The possibility for companies to promote their business
  by placing an add at the end of Alejandra's terminal output, and thank you messages for the sponsors of the project.

  You can obtain this benefits in @kamadorueda's [sponsor page](https://github.com/sponsors/kamadorueda).

- Thank you messages for the different people
  who have helped improving Alejandra.

### Changed

- Now big files are formatted first,
  making faster the formatting process
  when using many threads and
  when formatting a big file is slower
  than formatting all the other smaller files in the repository.
  For instance in Nixpkgs.
- The CLI was simplified,
  removing the elements that people find less useful
  and polishing those that people use the most.
  The screen is not cleared anymore.
- Upgraded dependencies to its latest version.

### Fixed

- The name displayed in the CLI application used to be `alejandra_cli`,
  now it is `Alejandra`.
- Typos here and there.

## [2.0.0] - 2022-07-13

### Added

- A [Style Guide](./STYLE.md),
  explaining why certain decisions were taken
  and why they are optimal
  from a productive Nix hacker point of view.

  This guide is a work in progress,
  but the aim is that it eventually documents
  all of the aspects of the Nix language,
  so that we all have the peace of mind
  that comes from knowing that the style in Alejandra
  is the way it is for a reason.

- Integration guides for: Neovim and GNU Emacs.
- Published Alejandra as [a crate](https://crates.io/crates/alejandra),
  and added API [documentation](https://docs.rs/alejandra/)
  so other tools and integrations can be built in top of it.

### Changed

- Now running: `$ alejandra -` formats stdin,
  as [POSIX](https://pubs.opengroup.org/onlinepubs/009695399/basedefs/xbd_chap12.html#tag_12_02)
  suggests.
- Comments are now indented by a multiple of 2
  [issues/294](https://github.com/kamadorueda/alejandra/issues/294):

  Previously
  (notice how some lines are indented with 3 and 5 spaces):

  ```nix
  /*
   Bla bla bla.

   More bla:
     - Bla bla
     - Bla
   */
  123
  ```

  Now (all lines are indented to 2 and 4 spaces):

  ```nix
  /*
  Bla bla bla.

  More bla:
    - Bla bla
    - Bla
  */
  123
  ```

  This makes it easier to write comments on major code editors,
  where a `<TAB>` (or `<Shift>+<Tab>`)
  moves the cursor/content to the next multiple of 2,
  and so indenting the comment contents
  to an odd number of spaces (like 3, 5, 7)
  is uncomfortable and unproductive.

  This change also allows cooperation
  with other tools
  like [EditorConfig](https://editorconfig.org/),
  to further exercise good practices over a codebase.

- Updated dependencies to its latest version.

### Fixed

- Empty lines in comments are now effectively empty,
  avoiding git from warning about extra whitespace:
  [issues/314](https://github.com/kamadorueda/alejandra/issues/314).

## [1.5.0] - 2022-06-22

### Changed

- Updated dependencies to its latest version.

## [1.4.0] - 2022-05-19

### Added

- A pre-commit hook (requires Alejandra to be installed in the host).

## [1.3.0] - 2022-05-09

### Added

- A pre-commit hook (requires Nix to be installed in the host).

### Changed

- Updated dependencies to its latest version.

## [1.2.0] - 2022-04-05

### Added

- A new i686-linux system to the flake.
- `apps.${system}.default` to the Flake
  so that newer versions of Nix
  understand this instead of `defaultApp.${system}`.

### Fixed

- A typo in the documentation where `aarch64-linux` appeared twice
  and `aarch64-darwin` didn't.

### Security

- New CVEs were discovered in the third party dependencies of our website:
  https://kamadorueda.github.io/alejandra/
  and so we updated those front-end dependencies to their latest version.

## [1.1.0] - 2022-03-10

### Added

- Emacs integration instructions.
- A `--quiet` flag to the CLI which hide output details,
  disable the TUI
  and only print error messages.

### Changed

- Updated dependencies to its latest version.

## [1.0.0] - 2022-03-03

### Added

- NixOS installation instructions

### Changed

- The indentation for function applications was improved,
  so that indentation is now correct to the human eye in all cases:

  ```diff
  -  name2 = function arg {
  -    asdf = 1;
  -  }
  -  argument;
  +  name2 =
  +    function arg {
  +      asdf = 1;
  +    }
  +    argument;
  ```

- String interpolations (`"${something}"`)
  now follow the same logic as parentheses (`(something)`),
  since ultimately, they are the same family of elements.

- Parentheses handling logic was rewritten
  and by extension string interpolations as well.

  ```diff
  - (
  -   self: super: {
  -     # ...
  -   }
  - )
  + (self: super: {
  +   # ...
  + })
  ```

  ```diff
  -      builtins.map (
  -        pkg: {
  -          name = "alejandra-${pkg.stdenv.targetPlatform.config}";
  -          value = pkg;
  -        }
  -      )
  +      builtins.map (pkg: {
  +        name = "alejandra-${pkg.stdenv.targetPlatform.config}";
  +        value = pkg;
  +      })
  ```

  ```diff
  -      (
  -        fenix.combine [
  -          fenix.latest.rustc
  -          fenix.latest.toolchain
  -          fenix.targets."wasm32-unknown-unknown".latest.rust-std
  -        ]
  -      )
  +      (fenix.combine [
  +        fenix.latest.rustc
  +        fenix.latest.toolchain
  +        fenix.targets."wasm32-unknown-unknown".latest.rust-std
  +      ])
  ```

  ```diff
     pkgs.writeText "other-modules.json"
     (l.toJSON
  -  (l.mapAttrs
  -  (pname: subOutputs: let
  -    pkg = subOutputs.packages."${pname}".overrideAttrs (old: {
  -      buildScript = "true";
  -      installMethod = "copy";
  -    });
  -  in "${pkg}/lib/node_modules/${pname}/node_modules")
  -  outputs.subPackages))
  +    (l.mapAttrs
  +      (pname: subOutputs:
  +        let
  +          pkg = subOutputs.packages."${pname}".overrideAttrs (old: {
  +            buildScript = "true";
  +            installMethod = "copy";
  +          });
  +        in
  +          "${pkg}/lib/node_modules/${pname}/node_modules")
  +      outputs.subPackages))
  ```

  ```diff
  -  (with a;
  -  /*
  -   comment
  -   */
  -  with b;
  -  with c; {
  -    a = 1;
  -    b = 2;
  -  })
  +  (with a;
  +    /*
  +      comment
  +      */
  +    with b;
  +    with c; {
  +      a = 1;
  +      b = 2;
  +    })
  ```

  In some cases it's possible to insert a newline after the
  opening element (either `(` or `${`) to force a tall formatting.

### Removed

- A few internal position counters, nothing visible from the outside.
- The new features cost a little of runtime speed,
  but anyway we are still pretty fast. ⚡

## [0.7.0] - 2022-02-28

### Added

- Inline comments support in binary operators:
  ```diff
  -      ++
  -      # subsections go last
  +      ++ # subsections go last
  ```
- Inline comments support in `with` and `assert` expressions:
  ```diff
  -  assert (libXft != null) -> libpng != null;
  -  # probably a bug
  -  assert stdenv.isDarwin -> libXaw != null;
  -  # fails to link otherwise
  +  assert (libXft != null) -> libpng != null; # probably a bug
  +
  +  assert stdenv.isDarwin -> libXaw != null; # fails to link otherwise
  +
  ```
- Inline comments support for `if-then-else` expressions:
  ```diff
                  if y ? ${a}
  -                then v x.${a} y.${a}
  -                # both have attr, use merge func
  -                else x.${a}
  -              # only x has attr
  +                then v x.${a} y.${a} # both have attr, use merge func
  +                else x.${a} # only x has attr
  ```
- Inline comments support for `inherit` expressions:
  ```diff
      inherit
        (callPackage ../development/tools/ocaml/ocamlformat {})
  -      ocamlformat
  -      # latest version
  +      ocamlformat # latest version
         ocamlformat_0_11_0
  ```
- Inline comments support for `parenthesis` expressions:
  ```diff
  -        || (
  -          # Accept {} for tests that are unsupported
  +        || ( # Accept {} for tests that are unsupported
            isDerivation x
            && x ? meta.timeout
          );
  ```

### Changed

- Linux binaries now use [mimalloc](https://github.com/microsoft/mimalloc)
  to provide much better performance when formatting Nixpkgs:

  - x86_64-unknown-linux-gnu, 1.3x faster,
    from 0m10.639s to 0m8.381s

  - x86_64-unknown-linux-musl, 15.8x faster,
    from 2m32.686s to 0m9.642s

  - [On QEMU](https://www.qemu.org/) aarch64-unknown-linux-musl,
    4.6x faster,
    from 5m26s to 1m10s

  - [On QEMU](https://www.qemu.org/) armv6l-unknown-linux-musleabihf,
    1.05x faster,
    from 8m7s to 7m41s

  - [On QEMU](https://www.qemu.org/) armv7l-unknown-linux-musleabihf,
    1.15x faster,
    from 5m54s to 5m7s

  - [On QEMU](https://www.qemu.org/) i686-unknown-linux-musl,
    1.07x faster,
    from 2m44s to 2m33s

- After profiling the code as suggested by
  [nnethercote's perf-book](https://nnethercote.github.io/perf-book/profiling.html)
  one critical path of Alejandra was identified an optimized,
  yielding huge performance boosts:

  - x86_64-unknown-linux-gnu, 2.5x faster,
    from 0m8.381s to 0m3.410s

  - x86_64-unknown-linux-musl, 2.3x faster,
    from 0m9.642s to 0m4.134s

  - [On QEMU](https://www.qemu.org/) aarch64-unknown-linux-musl,
    2.4x faster,
    from 1m10s to 0m29s

  - [On QEMU](https://www.qemu.org/) armv6l-unknown-linux-musleabihf,
    1.85x faster,
    from 7m41s to 4m8.399s

  - [On QEMU](https://www.qemu.org/) armv7l-unknown-linux-musleabihf,
    1.88x faster,
    from 5m7s to 2m42.595s

  - [On QEMU](https://www.qemu.org/) i686-unknown-linux-musl,
    1.65x faster,
    from 2m33s to 1m32.671s

  In general this is an algorithmic improvement
  and therefore the following platforms should be faster as well
  by a similar ratio
  (not measured):

  - aarch64-apple-darwin
  - x86_64-apple-darwin

- A `--threads` flag, so you can pick how many formatting threads to spawn.
  Defaults to the number of logical CPUs in your system.

- Position counters were improved to offer an extra 1.13x speedup.

## [0.6.0] - 2022-02-25

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

[unreleased]: https://github.com/kamadorueda/alejandra/compare/3.0.0...HEAD
[3.0.0]: https://github.com/kamadorueda/alejandra/compare/2.0.0...3.0.0
[2.0.0]: https://github.com/kamadorueda/alejandra/compare/1.5.0...2.0.0
[1.5.0]: https://github.com/kamadorueda/alejandra/compare/1.4.0...1.5.0
[1.4.0]: https://github.com/kamadorueda/alejandra/compare/1.3.0...1.4.0
[1.3.0]: https://github.com/kamadorueda/alejandra/compare/1.2.0...1.3.0
[1.2.0]: https://github.com/kamadorueda/alejandra/compare/1.1.0...1.2.0
[1.1.0]: https://github.com/kamadorueda/alejandra/compare/1.0.0...1.1.0
[1.0.0]: https://github.com/kamadorueda/alejandra/compare/0.7.0...1.0.0
[0.7.0]: https://github.com/kamadorueda/alejandra/compare/0.6.0...0.7.0
[0.6.0]: https://github.com/kamadorueda/alejandra/compare/0.5.0...0.6.0
[0.5.0]: https://github.com/kamadorueda/alejandra/compare/0.4.0...0.5.0
[0.4.0]: https://github.com/kamadorueda/alejandra/compare/0.3.1...0.4.0
[0.3.1]: https://github.com/kamadorueda/alejandra/compare/0.3.0...0.3.1
[0.3.0]: https://github.com/kamadorueda/alejandra/compare/0.2.0...0.3.0
[0.2.0]: https://github.com/kamadorueda/alejandra/compare/0.1.0...0.2.0
[0.1.0]: https://github.com/kamadorueda/alejandra/compare/0.0.0...0.1.0
[0.0.0]: https://github.com/kamadorueda/alejandra/compare/6adfbe8516bf6d9e896534e01118e1bc41f65425...0.0.0
