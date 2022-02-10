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

  We optimize for the wisdom of the crowd,
  which comes in big part
  from the 2.3 million lines of code of [Nixpkgs](https://github.com/NixOS/nixpkgs).

- ✔️ **Transparent**

  You won't notice the formatter after a while.

  Humans care about the content,
  machines about the style!

## Getting started

Let's get Alejandra on our systems:

- Nix with Flakes:

  ```bash
  $ nix run github:kamadorueda/alejandra -- --help
  ```

- Nix stable:

  Pick one depending on your platform:

  ```bash
  $ nix-env -ivA x86_64-darwin -f https://github.com/kamadorueda/alejandra/tarball/main
  $ nix-env -ivA x86_64-linux -f https://github.com/kamadorueda/alejandra/tarball/main
  ```

  Then run with:

  ```bash
  $ alejandra --help
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

## Footnotes

[^benchmark-specs]:
    Running on a [machine](https://github.com/kamadorueda/machine) with:

    - CPU: 8 physical x Intel(R) Core(TM) i7-10700K, 16 logical
    - MHz: 3800.00
    - BogoMips: 7599.80
    - Cache Size: 16384 KB

    Results:

    | Logical Cores | Seconds |
    | :-----------: | :-----: |
    |       1       |   45    |
    |       2       |   22    |
    |       4       |   14    |
    |       8       |   10    |
    |      16       |   11    |

[^semantic-changes]: The methodology to claim this is:

    1.  Checkout [Nixpkgs](https://github.com/nixos/nixpkgs) and run:

        ```bash
        $ nix-env -qaP --json --drv-path > before
        ```

    1.  Now format with Alejandra and run:

        ```bash
        $ nix-env -qaP --json --drv-path > after
        ```

    As of 2022-02-08,
    there are 47 differences in a set of 36278 derivations
    because of things like this:

    ```
    goDeps = ./deps.nix;
    ```

    Since `./deps.nix` was also formatted
    you get a semantical difference.

    This is something that should be solved on Nixpkgs
    and not a bug in Alejandra. For example:

    - https://github.com/NixOS/nixpkgs/pull/157760
