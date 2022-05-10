#! /usr/bin/env sh

set -eux

if ! command -v nix-build; then
  echo 'ERROR: this pre-commit hook requires "nix-build" to be installed first'
  exit 1
fi

if !command -v nix-instantiate; then
  echo 'ERROR: this pre-commit hook requires "nix-instantiate" to be installed first'
  exit 1
fi

echo INFO: computing current system
system="$(nix-instantiate --eval --expr builtins.currentSystem)"

echo INFO: building Alejandra

nix-build \
  --attr ${system} \
  --out-link result-alejandra \
  https://github.com/kamadorueda/alejandra/tarball/1.3.0

echo INFO: running Alejandra:
result-alejandra/bin/alejandra -- -q "${@}"
