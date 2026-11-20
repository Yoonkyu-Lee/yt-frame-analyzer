#!/usr/bin/env bash
# Measure one arm:  bash run.sh arms/<name>      (from northstar/pilot-vnovel/)
# Copies <arm>/segmented_download.rs into the harness, then reports size, warnings,
# the arm's own tests, the hidden intent tests and the benchmark.
set -u
P="$(cd "$(dirname "$0")" && pwd)"; ARM="$1"; NAME="$(basename "$ARM")"
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$P/../../.seg-target}"
cp "$P/$ARM/segmented_download.rs" "$P/harness/arm/segmented_download.rs" || exit 1
cd "$P/harness"

# code lines: before #[cfg(test)], excluding blank lines and // comments
code=$(awk '/^#\[cfg\(test\)\]/{exit} {print}' arm/segmented_download.rs | grep -vE '^\s*$|^\s*//' | wc -l)
tests=$(awk 'f{print} /^#\[cfg\(test\)\]/{f=1}' arm/segmented_download.rs | grep -vE '^\s*$|^\s*//' | wc -l)
fns=$(awk '/^#\[cfg\(test\)\]/{exit} {print}' arm/segmented_download.rs | grep -cE '^\s*(pub(\([a-z]+\))? )?(async )?fn ')
echo "arm        $NAME"
echo "code lines $code  (own tests $tests)"
echo "functions  $fns"
warn=$(cargo clippy --offline --lib 2>&1 | grep -cE '^(warning|error)' )
echo "clippy     $warn warnings (incl. summary line)"
own=$(cargo test --offline --lib 2>&1 | grep -E '^test result' | tail -1)
echo "own tests  ${own:-BUILD FAILED}"
out=$(cargo test --offline --test hidden 2>&1)
echo "hidden     $(echo "$out" | grep -E '^test result' | tail -1)"
echo "$out" | grep -E '^test .* FAILED$' | sed 's/^/           /'
cargo run --offline --release --quiet --bin bench 2>&1 | tail -1
# transparency probe ground truth for this arm
cargo test --offline --test probe -- --nocapture --test-threads=1 2>&1 | grep -o "PROBE.*" > "$P/$ARM/PROBE.txt"
echo "probe      $(wc -l < "$P/$ARM/PROBE.txt") facts -> $ARM/PROBE.txt"
