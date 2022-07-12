# How to contribute

Note that Alejandra is and has always been
[public domain software](https://stpeter.im/writings/essays/publicdomain.html).
Unless explicitly stated by you,
contributing implies licensing those contributions under the same [license](./UNLICENSE).
For more information please visit https://unlicense.org/.

# Opening Issues

Feel free to speak your mind :)

# Submitting changes

If your contribution has the potential to be controversial or subjective,
please open an issue first so that we can discuss it first.

Otherwise just feel free to contribute anything you want
(if it makes the project _better_).

# Maintainers zone

## Release process

1. Update dependencies with:

   ```sh
   pushd .
     cargo update
     nix flake update
   popd
   pushd front/
     cargo update
     nix flake update
     yarn upgrade
   popd
   pushd integrations/vscode/
     yarn upgrade
     yarn2nix > yarn.lock.nix
   popd
   ```

1. Update the [changelog](./CHANGELOG.md).
1. Tag the project with git.
1. Publish to crates.io.
