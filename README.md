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

  We integrate with common code editors:

  - [Alejandra extension for Visual Studio Code](https://marketplace.visualstudio.com/items?itemName=kamadorueda.alejandra)

## Getting started

### On the web editor

Please visit:
[kamadorueda.github.io/alejandra](https://kamadorueda.github.io/alejandra/)

### Prebuilt binaries

You can download a binary for your platform,
make it executable (`$ chmod +x`)
and have fun!

- [aarch64-unknown-linux-musl](https://github.com/kamadorueda/alejandra/releases/download/0.3.1/alejandra-aarch64-unknown-linux-musl)
- [armv6l-unknown-linux-musleabihf](https://github.com/kamadorueda/alejandra/releases/download/0.3.1/alejandra-armv6l-unknown-linux-musleabihf)
- [armv7l-unknown-linux-musleabihf](https://github.com/kamadorueda/alejandra/releases/download/0.3.1/alejandra-armv7l-unknown-linux-musleabihf)
- [i686-unknown-linux-musl](https://github.com/kamadorueda/alejandra/releases/download/0.3.1/alejandra-i686-unknown-linux-musl)
- [x86_64-unknown-linux-musl](https://github.com/kamadorueda/alejandra/releases/download/0.3.1/alejandra-x86_64-unknown-linux-musl)

Alternatively there is an automated method for some platforms.
It needs:
[curl](https://curl.se/),
[sh](https://www.gnu.org/software/bash/) and
[chmod](https://www.gnu.org/software/coreutils/):

- aarch64-linux:
  `$ curl https://raw.githubusercontent.com/kamadorueda/alejandra/main/installers/aarch64-linux.sh | sh`

- x86_64-linux:
  `$ curl https://raw.githubusercontent.com/kamadorueda/alejandra/main/installers/x86_64-linux.sh | sh`

Then run Alejandra with:

```bash
$ ./alejandra --help
```

or:

```bash
$ /path/to/alejandra --help
```

### Nix installation

- Nix with [Flakes](https://nixos.wiki/wiki/Flakes):

  ```bash
  $ nix profile install github:kamadorueda/alejandra/0.3.1
  ```

- Nix stable:

  Pick one depending on your platform:

  ```bash
  $ nix-env -ivA aarch64-darwin -f https://github.com/kamadorueda/alejandra/tarball/0.3.1
  $ nix-env -ivA aarch64-linux -f https://github.com/kamadorueda/alejandra/tarball/0.3.1
  $ nix-env -ivA x86_64-darwin -f https://github.com/kamadorueda/alejandra/tarball/0.3.1
  $ nix-env -ivA x86_64-linux -f https://github.com/kamadorueda/alejandra/tarball/0.3.1
  ```

Then run Alejandra with:

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

## Versioning

We use [semver](https://semver.org/) to version Alejandra.

Our public API consists of:

- The formatting rules (a.k.a. the style).
- The CLI tool (`$ alejandra`),
  command line flags,
  positional arguments,
  and stdout.

## Changelog

Please see: [CHANGELOG.md](./CHANGELOG.md).

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
    |       1       |   35    |
    |       2       |   18    |
    |       4       |   10    |
    |       8       |   10    |
    |      16       |   10    |

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
