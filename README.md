<h1 align="center">Alejandra 💅</h2>

<p align="center">The Uncompromising Nix Code Formatter</p>

<p align="center">
  <a
    href="https://buildkite.com/kamadorueda/alejandra"
  >
    <img
      alt="CI/CD"
      src="https://badge.buildkite.com/67d170860f5630bbc776a97fb0be9c88a97c92860c91f77aa0.svg?branch=main"
    >
    </img>
  </a>
  <a
    href="https://coveralls.io/github/kamadorueda/alejandra?branch=main"
  >
    <img
      alt="Coverage"
      src="https://coveralls.io/repos/github/kamadorueda/alejandra/badge.svg?branch=main"
    >
    </img>
  </a>
  <a
    href="https://github.com/kamadorueda/alejandra/blob/main/UNLICENSE"
  >
    <img
      alt="License: The Unlicense"
      src="https://img.shields.io/badge/license-The Unlicense-green.svg"
    >
  </a>
  <a
    href="https://github.com/kamadorueda/alejandra"
  >
    <img
      alt="style: Alejandra"
      src="https://img.shields.io/badge/code%20style-Alejandra-green.svg"
    >
  </a>

</p>
<p align="center">
  Try it on your browser!
  <a
    href="https://kamadorueda.github.io/alejandra/"
  >
    here
  </a>
</p>

<a href="https://asciinema.org/a/470438" target="_blank">
  <img src="https://asciinema.org/a/470438.svg" />
</a>

## Features

- ✔️ **Fast**

  It's written in [Rust](https://www.rust-lang.org/)
  and formats [Nixpkgs](https://github.com/NixOS/nixpkgs)
  in just a few seconds.
  [^benchmark-specs]

- ✔️ **Powerful**

  We define a comprehensive style
  for all possible combinations of the Nix expression language.

- ✔️ **Reliable**

  High coverage, battle tested.

  From Nix's eyes, code is _just_ the same.
  [^semantic-changes]

- ✔️ **Beautiful**

  Beauty is subjective, right?

  We started from the wisdom of the crowd,
  which comes in big part
  from the 2.3 million lines of code of [Nixpkgs](https://github.com/NixOS/nixpkgs).
  Then we applied the feedback of developers
  who have used [Nix](https://nixos.org) on a day to day basis for several years.

- ✔️ **Transparent**

  You won't notice the formatter after a while.

  Humans care about the content,
  machines about the style!

- ✔️ **Native**

  We integrate with common code editors and workflows:

  - [Alejandra extension for Visual Studio Code](https://marketplace.visualstudio.com/items?itemName=kamadorueda.alejandra)
  - [Doom-emacs](https://github.com/hlissner/doom-emacs):

    ```
    (set-formatter! 'alejandra "alejandra --quiet" :modes '(nix-mode))
    ```

  - [Pre-commit](https://pre-commit.com/):

    ```yaml
    repos:
      - repo: https://github.com/kamadorueda/alejandra
        rev: 1.4.0
        hooks:
          # Choose one of the following:
          - id: alejandra # Requires Nix to be previously installed in the system
          - id: alejandra-system # Requires Alejandra to be previously installed in the system
    ```
    
  - [pre-commit-hooks.nix](https://github.com/cachix/pre-commit-hooks.nix)
  
    ```nix
    pre-commit-check = pre-commit-hooks.lib.${system}.run {
      hooks = {
        alejandra.enable = true;
      };
    };
    ```

## Getting started

### On the web editor

Please visit:
[kamadorueda.github.io/alejandra](https://kamadorueda.github.io/alejandra/).

### Prebuilt binaries

You can download a binary for your platform:

- [aarch64-unknown-linux-musl](https://github.com/kamadorueda/alejandra/releases/download/1.4.0/alejandra-aarch64-unknown-linux-musl)
- [armv6l-unknown-linux-musleabihf](https://github.com/kamadorueda/alejandra/releases/download/1.4.0/alejandra-armv6l-unknown-linux-musleabihf)
- [armv7l-unknown-linux-musleabihf](https://github.com/kamadorueda/alejandra/releases/download/1.4.0/alejandra-armv7l-unknown-linux-musleabihf)
- [i686-unknown-linux-musl](https://github.com/kamadorueda/alejandra/releases/download/1.4.0/alejandra-i686-unknown-linux-musl)
- [x86_64-unknown-linux-musl](https://github.com/kamadorueda/alejandra/releases/download/1.4.0/alejandra-x86_64-unknown-linux-musl)

Make it executable (`$ chmod +x`)
and run Alejandra with:

```bash
$ ./alejandra --help
```

or:

```bash
$ /path/to/alejandra --help
```

### From [Nixpkgs](https://github.com/nixos/nixpkgs)

Please visit: [search.nixos.org/packages?query=alejandra](https://search.nixos.org/packages?channel=unstable&show=alejandra&from=0&size=50&sort=relevance&type=packages&query=alejandra).

### Nix installation

- Nix with [Flakes](https://nixos.wiki/wiki/Flakes):

  ```bash
  $ nix profile install github:kamadorueda/alejandra/1.4.0
  ```

- Nix stable:

  Pick one depending on your platform:

  ```bash
  $ nix-env -ivA aarch64-darwin -f https://github.com/kamadorueda/alejandra/tarball/1.4.0
  $ nix-env -ivA aarch64-linux -f https://github.com/kamadorueda/alejandra/tarball/1.4.0
  $ nix-env -ivA i686-linux -f https://github.com/kamadorueda/alejandra/tarball/1.4.0
  $ nix-env -ivA x86_64-darwin -f https://github.com/kamadorueda/alejandra/tarball/1.4.0
  $ nix-env -ivA x86_64-linux -f https://github.com/kamadorueda/alejandra/tarball/1.4.0
  ```

Then run Alejandra with:

```bash
$ alejandra --help
```

### NixOS installation

- Nix with [Flakes](https://nixos.wiki/wiki/Flakes):

  ```nix
  {
    inputs = {
      nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";

      alejandra.url = "github:kamadorueda/alejandra/1.4.0";
      alejandra.inputs.nixpkgs.follows = "nixpkgs";
    };

    outputs = {alejandra, nixpkgs, ...}: {
      nixosConfigurations = {
        example = nixpkgs.lib.nixosSystem rec {
          # We support: aarch64-darwin, aarch64-linux, i686-linux, x86_64-darwin, x86_64-linux
          system = "x86_64-linux";

          modules = [
            {
              environment.systemPackages = [alejandra.defaultPackage.${system}];
            }
            # Import your other modules here
            # ./path/to/my/module.nix
            # ...
          ];
        };
      };
    };
  }
  ```

- Nix stable:

  ```nix
  let
    alejandra =
      (import (builtins.fetchTarball {
        url = "https://github.com/kamadorueda/alejandra/tarball/1.4.0";
        sha256 = "0000000000000000000000000000000000000000000000000000";
      }))
      # Pick one from: aarch64-darwin, aarch64-linux, i686-linux, x86_64-darwin, x86_64-linux
      .x86_64-linux
      .outPath;
  in {
    environment.systemPackages = [alejandra];
  }
  ```

## Do I need to configure anything?

- No.

## Discussion

- [RFC 0101 - Nix formatting](https://github.com/NixOS/rfcs/pull/101)

## Cool libraries

- [rnix-parser](https://github.com/nix-community/rnix-parser)

## Alternatives

- [nixpkgs-fmt](https://github.com/nix-community/nixpkgs-fmt)
- [nixfmt](https://github.com/serokell/nixfmt)

See why Alejandra was created
and a comparison between alternatives
[here](https://discourse.nixos.org/t/the-uncompromising-nix-code-formatter/17385/3?u=kamadorueda).

Alternatively, checkout the code examples of the different formatters [here](https://github.com/kamadorueda/rfc-0101).

## Versioning

We use [semver](https://semver.org/) to version Alejandra.

Our public API consists of:

- The formatting rules (a.k.a. the style).
- The CLI tool (`$ alejandra`),
  command line flags,
  positional arguments,
  exit codes,
  and stdout.

## Changelog

Please read: [CHANGELOG](./CHANGELOG.md).

## Contributors

The following people have helped improving Alejandra.

Thank you ❤️

- [Kevin Amado](https://github.com/kamadorueda) ~
  [Email](mailto:kamadorueda@gmail.com),
  [Patreon](https://www.patreon.com/kamadorueda).
- [Thomas Bereknyei](https://github.com/tomberek).
- [Piegames](https://github.com/piegamesde).
- [Joachim Ernst](https://github.com/0x4A6F).
- [David Arnold](https://github.com/blaggacao).
- [David Hauer](https://github.com/DavHau).
- [Ryan Mulligan](https://github.com/ryantm).
- [Fabian Möller](https://github.com/B4dM4n).
- [Rok Garbas](https://github.com/garbas).
- [Yorick van Pelt](https://github.com/yorickvP).
- [Rehno Lindeque](https://github.com/rehno-lindeque).
- [Jörg Thalheim](https://github.com/Mic92).
- [Vincent Ambo](https://github.com/tazjin).
- [Loïc Reynier](https://github.com/loicreynier).
- [Mr Hedgehog](https://github.com/ModdedGamers).
- [Tristan Maat](https://github.com/TLATER).
- [Norbert Melzer](https://github.com/NobbZ).
- [Patrick Stevens](https://github.com/Smaug123).
- [Connor Baker](https://github.com/ConnorBaker).
- [Florian Finkernagel](https://github.com/TyberiusPrime).

## Footnotes

[^benchmark-specs]:
    Running on a [machine](https://github.com/kamadorueda/machine) with:

    - CPU: 8 physical x Intel(R) Core(TM) i7-10700K, 16 logical
    - MHz: 3800.00
    - BogoMips: 7599.80
    - Cache Size: 16384 KB

    Using:

    ```bash
    # x86_64-unknown-linux-gnu
    $ time alejandra --threads $threads /path/to/nixpkgs
    ```

    Results:

    | $threads | Seconds |
    | :------: | :-----: |
    |    1     |  13.4   |
    |    2     |   6.9   |
    |    4     |   3.6   |
    |    8     |   2.6   |
    |    16    |   3.1   |

[^semantic-changes]: The methodology to claim this is:

    1.  Checkout [Nixpkgs](https://github.com/nixos/nixpkgs) and run:

        ```bash
        $ nix-env -qaP --json --drv-path > before
        ```

    1.  Now format with Alejandra and run:

        ```bash
        $ nix-env -qaP --json --drv-path > after
        ```

    As of 2022-02-21,
    there are 46 differences in a set of 36314 derivations
    because of things like this:

    ```
    goDeps = ./deps.nix;
    ```

    Since `./deps.nix` was also formatted
    you get a semantical difference.

    This is something that should be solved on Nixpkgs
    and not a bug in Alejandra. For example:

    - https://github.com/NixOS/nixpkgs/pull/157760
