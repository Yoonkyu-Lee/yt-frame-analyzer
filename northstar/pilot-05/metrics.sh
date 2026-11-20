#!/usr/bin/env bash
# Measure one round of the compression loop.
#   wsl.exe -e bash metrics.sh rounds/r1        (run from northstar/pilot-05/)
# Builds every tool from <round>/<tool>.c (+ common.c if present), then reports:
#   size, warnings, intent tests (plain and ASan/UBSan), line coverage under intent tests, hot-path timing.
set -u
P="$(cd "$(dirname "$0")" && pwd)"; NAME="$(basename "$1")"
W="$(mktemp -d)"; TOOLS="touch cat mkdir mv rm find"
# work on a copy: the repo path contains spaces
cp -r "$1" "$W/src"; R="$W/src"; cp "$P/tests/intent.sh" "$W/intent.sh"; P2="$W"
common() { [ -f "$R/common.c" ] && echo "$R/common.c"; }
lines() { cat "$@" | grep -vE '^\s*$|^\s*(//|/\*|\*)' | wc -l; }

src=$(ls "$R"/*.c "$R"/*.h 2>/dev/null)
echo "round      $NAME"
echo "files      $(echo "$src" | wc -l)"
echo "code lines $(lines $src)"
echo "functions  $(cat $src | grep -cE '^[a-zA-Z_][a-zA-Z0-9_ \*]*\([^;]*\)\s*\{?\s*$')"

warn=0
for t in $TOOLS; do
  warn=$((warn + $(gcc -Wall -Wextra -Wpedantic -I"$R" -c "$R/$t.c" -o /dev/null 2>&1 | grep -c 'warning:')))
done
[ -f "$R/common.c" ] && warn=$((warn + $(gcc -Wall -Wextra -Wpedantic -c "$R/common.c" -o /dev/null 2>&1 | grep -c 'warning:')))
echo "warnings   $warn  (gcc default std, -Wall -Wextra -Wpedantic)"

build() { # dir, flags...
  local d=$1; shift; mkdir -p "$d"
  for t in $TOOLS; do gcc "$@" -I"$R" "$R/$t.c" $(common) -o "$d/$t" -lm || { echo "BUILD FAILED: $t"; return 1; }; done
}
build "$W/plain" -O2 && echo "intent     $(bash "$P2/intent.sh" "$W/plain" | tail -1)"
bash "$P2/intent.sh" "$W/plain" | grep '^FAIL' | sed 's/^/           /'

build "$W/san" -g -O1 -fsanitize=address,undefined -fno-omit-frame-pointer
mkdir -p "$W/sanlog"
ASAN_OPTIONS="detect_leaks=1:log_path=$W/sanlog/asan" UBSAN_OPTIONS="log_path=$W/sanlog/ubsan:print_stacktrace=0" \
  bash "$P2/intent.sh" "$W/san" > "$W/san.out"
echo "sanitizers $(tail -1 "$W/san.out"); reports: $(ls "$W/sanlog" | wc -l)"
cat "$W"/sanlog/* 2>/dev/null | grep -oE "ERROR: AddressSanitizer: [a-z-]+|runtime error: [^:]+|SUMMARY: AddressSanitizer: .*" | sort | uniq -c | sed 's/^/           /'

# coverage: objects built separately so common.c counters accumulate across tools
C="$W/cov"; mkdir -p "$C"; cp "$R"/*.c "$R"/*.h "$C/" 2>/dev/null
( cd "$C" && for f in *.c; do gcc -O0 --coverage -I. -c "$f" -o "${f%.c}.o"; done
  for t in $TOOLS; do gcc --coverage "$t.o" $( [ -f common.o ] && echo common.o ) -o "$t" -lm; done )
bash "$P2/intent.sh" "$C" > /dev/null
( cd "$C" && gcov -n *.c 2>/dev/null ) | awk '/^File/{f=$2} /^Lines executed/{split($2,a,":"); sub("%","",a[2]); n=$4; e+=a[2]*n/100; t+=n} END{printf "coverage   %.1f%% of %d executable lines run by intent tests\n", 100*e/t, t}'

# hot paths (P1)
cd "$W"; head -c 200000000 /dev/urandom > big.bin
mk() { mkdir -p "$1"; for d in $(seq 1 100); do mkdir -p "$1/d$d"; (cd "$1/d$d" && seq 1 500 | sed 's/$/.c/' | xargs touch); done; }
mk ft; mk rt
tm() { /usr/bin/time -f "%e" "$@" 2>&1 >/dev/null | tail -1; }
echo "perf       cat 200MB $(tm "$W/plain/cat" big.bin)s | find 50k -name $(tm "$W/plain/find" ft -name '*77.c')s | rm -r 50k $(tm "$W/plain/rm" -r rt)s"
rm -rf "$W"
