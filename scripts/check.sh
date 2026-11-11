#!/usr/bin/env bash
# Runs the quality checks of CI locally: rustfmt, clippy, the tests,
# shfmt and shellcheck on the shell scripts
# and the formatting of the repository's own .nix files.
# Stops at the first failure. Run it with `nix run .#check`.
set -euo pipefail

usage() {
  cat << 'USAGE'
Usage: scripts/check.sh [--nixpkgs DIR]

  --nixpkgs DIR   also run scripts/check-nixpkgs.sh on this nixpkgs checkout
  -h, --help      show this help
USAGE
}

nixpkgs=
while [ $# -gt 0 ]; do
  case $1 in
    --nixpkgs) nixpkgs=$2 && shift 2 ;;
    -h | --help) usage && exit 0 ;;
    *) usage >&2 && exit 2 ;;
  esac
done

root=$(git rev-parse --show-toplevel)
cd "$root"
bin="${CARGO_TARGET_DIR:-target}/release/alejandra"
current=
trap 'echo "+++ failed: $current" >&2' ERR
step() {
  current=$1
  echo "--- $1"
}

step rustfmt
git ls-files -z '*.rs' | xargs -0 rustfmt --check

step clippy
cargo clippy --workspace --all-targets -- -D warnings
cargo clippy --all-targets \
  --manifest-path src/alejandra_wasm/Cargo.toml -- -D warnings

step tests
cargo test --workspace
cargo test --manifest-path src/alejandra_wasm/Cargo.toml

step "shell scripts"
git ls-files -z '*.sh' | xargs -0 shfmt -bn -ci -sr -i 2 -s -d
git ls-files -z '*.sh' | xargs -0 shellcheck

step "nix files formatted"
cargo build --release --package alejandra_cli
git ls-files -z '*.nix' \
  ':!:src/alejandra/tests/cases/' \
  ':!:src/alejandra_cli/tests/inputs/' \
  ':!:src/alejandra_cli/tests/gitignore/' \
  | xargs -0 "$bin" --check --quiet

if [ -n "$nixpkgs" ]; then
  step nixpkgs
  scripts/check-nixpkgs.sh --formatter "$bin" --nixpkgs "$nixpkgs"
fi

echo "+++ all checks passed"
