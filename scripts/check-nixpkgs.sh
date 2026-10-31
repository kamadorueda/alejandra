#!/usr/bin/env bash
# Formats nixpkgs with Alejandra and checks that the formatter is
# idempotent and keeps the meaning of the code.
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage: scripts/check-nixpkgs.sh [options] [PATH...]

Formats a copy of nixpkgs twice and reports:
  - files that change on the second run (idempotence)
  - files where `nix-instantiate --parse` differs after formatting (meaning)
  - wall time of the first run

PATH is a file or directory relative to nixpkgs (default: all of nixpkgs).

Options:
  --formatter BIN   alejandra binary (default: `nix build` of this repository)
  --nixpkgs DIR     nixpkgs checkout (default: the one pinned in flake.lock)
  --compare BIN     also count the lines that differ between the output of
                    --formatter and the output of this other binary
  -h, --help        show this help

Exits with status 1 on any idempotence or meaning failure.
USAGE
}

repo=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
formatter=
nixpkgs=
compare=
paths=()

while [ $# -gt 0 ]; do
  case $1 in
  --formatter) formatter=$2 && shift 2 ;;
  --nixpkgs) nixpkgs=$2 && shift 2 ;;
  --compare) compare=$2 && shift 2 ;;
  -h | --help) usage && exit 0 ;;
  --) shift && paths+=("$@") && break ;;
  -*) echo "Unknown option: $1" >&2 && usage >&2 && exit 64 ;;
  *) paths+=("$1") && shift ;;
  esac
done

if [ -z "$formatter" ]; then
  echo "Building alejandra with nix"
  out=$(nix --extra-experimental-features 'nix-command flakes' build \
    --no-link --print-out-paths "$repo")
  formatter=$out/bin/alejandra
fi

if [ -z "$nixpkgs" ]; then
  echo "Fetching the nixpkgs pinned in flake.lock"
  nixpkgs=$(nix --extra-experimental-features 'nix-command flakes' eval \
    --raw --impure --expr "
      let lock = builtins.fromJSON (builtins.readFile $repo/flake.lock);
      in (builtins.fetchTree lock.nodes.nixpkgs.locked).outPath
    ")
fi

[ ${#paths[@]} -gt 0 ] || paths=(.)

work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT

# Copies the .nix files of the given paths of nixpkgs into $1
copy_nix_files() {
  local dest=$1
  mkdir -p "$dest"
  (
    cd "$nixpkgs"
    find "${paths[@]}" -name '*.nix' -type f -print0 |
      xargs -0 --no-run-if-empty cp --parents -t "$dest"
  )
  chmod -R u+w "$dest"
}

copy_nix_files "$work/before"
files=$(find "$work/before" -type f | wc -l)
echo "Checking $files files from $nixpkgs"

cp -a "$work/before" "$work/once"

# alejandra exits with an error on files that do not parse, that is expected
start=$(date +%s)
"$formatter" --quiet "$work/once" >/dev/null 2>&1 || true
elapsed=$(($(date +%s) - start))

cp -a "$work/once" "$work/twice"
"$formatter" --quiet "$work/twice" >/dev/null 2>&1 || true

# idempotence
(cd "$work/once" && diff -rq . "$work/twice" || true) |
  sed -n 's/^Files \.\/\(.*\) and .* differ$/\1/p' |
  sort >"$work/not-idempotent.txt"

# meaning: compare the syntax trees that Nix parses, in original and formatted
parse() {
  # relative paths are printed as absolute: hide the directory
  nix-instantiate --parse "$1" 2>/dev/null | sed "s|$2|ROOT|g"
}
check_meaning() {
  local file original formatted
  set -o pipefail
  for file in "$@"; do
    original=$(parse "$work/before/$file" "$work/before") || continue
    formatted=$(parse "$work/once/$file" "$work/once") || formatted=
    [ "$original" = "$formatted" ] || echo "$file" >>"$work/meaning.txt"
  done
}
export -f parse check_meaning
export work

: >"$work/meaning.txt"
(cd "$work/before" && find . -type f -name '*.nix' -print0 | cut -z -c3-) |
  xargs -0 -P "$(nproc)" -n 20 bash -c 'check_meaning "$@"' _
sort -o "$work/meaning.txt" "$work/meaning.txt"

report() {
  local name=$1 list=$2
  echo "$name: $(wc -l <"$list")"
  sed 's/^/  /' "$list"
}

echo
echo "First run: ${elapsed}s"
report "Files that change on the second run" "$work/not-idempotent.txt"
report "Files where the meaning changes" "$work/meaning.txt"

if [ -n "$compare" ]; then
  copy_nix_files "$work/other"
  "$compare" --quiet "$work/other" >/dev/null 2>&1 || true
  changed=$(diff -r -U0 "$work/once" "$work/other" |
    grep -E '^[-+]' | grep -cvE '^(\+\+\+|---) ' || true)
  echo "Lines that differ between the two formatters: $changed"
fi

if [ -s "$work/not-idempotent.txt" ] || [ -s "$work/meaning.txt" ]; then
  exit 1
fi
