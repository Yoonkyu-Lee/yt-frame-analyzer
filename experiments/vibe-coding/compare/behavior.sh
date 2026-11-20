#!/usr/bin/env bash
# Run human (hand-coding) and Codex (vibe-coding) programs through identical checks in WSL.
#   wsl.exe -e bash compare/behavior.sh             human = sources/hayyoth (as recorded in the videos)
#   wsl.exe -e bash compare/behavior.sh completed   human = hand-completed (deferred work finished)
#   wsl.exe -e bash compare/behavior.sh remade      human = hand-remade (easy gaps closed, same style)
# Everything is built with -fsanitize=address,undefined; leaks are reported by LeakSanitizer.
set -u
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
H="$ROOT/../../sources/hayyoth"; HDEF=""
HRM=""; HFIND=".c"; HFIND2="cat"   # human CLI: rm is recursive by default, find matches substrings
[ "${1:-}" = completed ] && { H="$ROOT/hand-completed"; HDEF="-DCOMPLETED"; }
[ "${1:-}" = remade ] && { H="$ROOT/hand-remade"; HDEF="-DCOMPLETED -DREMADE"; HRM="-r"; HFIND="*.c"; HFIND2="cat*"; }
echo "# human code: $H"
W="$(mktemp -d)"
SAN="-g -O1 -fsanitize=address,undefined -fno-omit-frame-pointer"
AUTHOR="-Wall -Wextra -pedantic -Wno-unused-parameter"
export ASAN_OPTIONS=detect_leaks=1 UBSAN_OPTIONS=print_stacktrace=0
san() { grep -hoE "ERROR: AddressSanitizer: [a-z-]+|runtime error: [^:]+|SUMMARY: AddressSanitizer: [0-9]+ byte\(s\) leaked in [0-9]+ allocation\(s\)" "$1" | sort -u | sed 's/^/    [san] /'; }
# warn <file> [include-dir]   (include dir passed separately: the repo path contains spaces)
warn() { gcc $AUTHOR ${2:+-I"$2"} -c "$1" -o /dev/null 2>&1 | grep "warning:" | tee -a "$W/warnings.txt" | wc -l; }

hdir() { ls -d "$H"/"$1"_* ; }
cp "$(hdir 07)/src/main.c" "$W/h07_src.c"; cp "$(hdir 08)/src/main.c" "$W/h08_src.c"; cp "$(hdir 09)/src/main.c" "$W/h09_src.c"
cp "$ROOT"/compare/probes/*.c "$W/"

echo "## 1. Compiler warnings (count) — author flags: $AUTHOR"
for pair in "02:02_tree" "03:03_random-moving-numbers" "05:05_linux-file-utils" "07:07_linked-list" "08:08_dynamic-array" "09:09_hash-table"; do
  n=${pair%%:*}; v=${pair#*:}; hw=0; cw=0
  for f in "$(hdir $n)"/src/*.c; do hw=$((hw + $(warn "$f"))); done
  for f in "$ROOT/$v"/output/*.c; do cw=$((cw + $(warn "$f" "$ROOT/$v/output"))); done
  echo "  $v: human $hw, codex $cw"
done
[ -s "$W/warnings.txt" ] && sed 's/^/    /' "$W/warnings.txt"
echo; echo "## 2. Data-structure probes under ASan/UBSan"
cd "$W"
gcc $SAN $HDEF h07.c -o h07 2>/dev/null; gcc $SAN -I"$ROOT/07_linked-list/output" c07.c "$ROOT/07_linked-list/output/list.c" -o c07
gcc $SAN $HDEF h08.c -o h08 2>/dev/null; gcc $SAN -I"$ROOT/08_dynamic-array/output" c08.c "$ROOT/08_dynamic-array/output/dynarray.c" -o c08
gcc $SAN $HDEF h09.c -o h09 2>/dev/null; gcc $SAN -Wno-format-truncation -I"$ROOT/09_hash-table/output" c09.c "$ROOT/09_hash-table/output/hash_table.c" -o c09
for p in h07 c07 h08 c08 h09 c09; do
  echo "  -- $p"; ./$p > $p.out 2> $p.err; rc=$?
  sed 's/^/    /' $p.out; echo "    exit=$rc"; san $p.err
done

echo; echo "## 3. tree on the same directory"
T="$W/t"; mkdir -p "$T/a/b" "$T/a/empty" "$T/locked"; touch "$T/a/b/c.txt" "$T/.hidden" "$T/z.txt"
ln -s a "$T/link_to_a"; ln -s . "$T/a/loop"; chmod 000 "$T/locked"
gcc $SAN "$(hdir 02)/src/main.c" -o htree; gcc $SAN "$ROOT/02_tree/output/main.c" -o ctree
for p in htree ctree; do echo "  -- $p t"; (cd "$W" && ./$p t > $p.out 2> $p.err; echo "    exit=$?"); sed 's/^/    | /' $p.out; grep -v "^$" $p.err | grep -v Sanitizer | head -3 | sed 's/^/    ! /'; san $p.err; done
for p in htree ctree; do printf "  -- $p /nope: "; ./$p /nope > /dev/null 2> e; echo "exit=$? $(head -1 e)"; done
chmod 755 "$T/locked"

echo; echo "## 4. File utilities: same intentions, each with its own CLI"
mkdir -p hu cu
for u in touch cat mkdir mv rm find; do gcc $SAN "$(hdir 05)/src/$u.c" -o hu/$u 2>/dev/null; done
(cd "$ROOT/05_linux-file-utils/output" && for u in touch cat mkdir mv rm find; do gcc $SAN -I. $u.c common.c -o "$W/cu/$u" -lm; done)
run() {
  local who=$1; shift
  "$@" > "$W/run.out" 2>&1; local rc=$?
  local shown="$*"; shown="${shown//$W\//}"
  printf "    %-6s %-34s exit=%d %s\n" "$who" "$shown" $rc \
    "$(grep -v -e Sanitizer -e '^=*$' -e '^ *#' -e '^ *$' "$W/run.out" | head -2 | tr '\n' ' ' | cut -c1-70)"
  san "$W/run.out"
}
for who in hu cu; do
  D="$W/s_$who"; mkdir -p "$D"; cd "$D"; B="$W/$who"
  echo "  -- $who"
  run $who $B/touch a.txt; run $who $B/cat a.txt
  run $who $B/mkdir d; run $who $B/mkdir d; run $who $B/mkdir x/y
  run $who $B/mv a.txt b.txt; run $who $B/mv b.txt d
  touch f.txt; run $who $B/rm f.txt; [ -e f.txt ] && echo "    f.txt still exists" || echo "    f.txt removed"
  mkdir -p tree/sub; touch tree/sub/q.c tree/w.c
  if [ $who = hu ]; then run $who $B/find "$HFIND" tree; else run $who $B/find tree -name '*.c'; fi
  run $who $B/rm tree; [ -e tree ] && echo "    tree still exists" || echo "    tree removed"
  [ -e tree ] && { run $who $B/rm -r tree; [ -e tree ] && echo "    tree still exists" || echo "    tree removed"; }
  # cases found while finishing the human code
  echo keep > k.txt; run $who $B/touch k.txt; echo "    k.txt after touch: '$(cat k.txt)' (expect 'keep')"
  echo new > n.txt; echo old > o.txt; run $who $B/mv n.txt o.txt; echo "    o.txt after mv onto it: '$(cat o.txt 2>/dev/null)' (expect 'new')"
  touch cat.c; if [ $who = hu ]; then run $who $B/find "$HFIND2" cat.c; else run $who $B/find cat.c -name 'cat*'; fi
  mkdir -p deep/x deep/locked; touch deep/x/f deep/locked/g; chmod 555 deep/locked
  if [ $who = hu ]; then run $who $B/rm $HRM deep; else run $who $B/rm -rf deep; fi
  echo "    after rm on dir with read-only subdir: $(find deep 2>/dev/null | wc -l) entries left (expect 3: deep, locked, g)"
  chmod 755 deep/locked 2>/dev/null; command rm -rf deep
done

echo; echo "## 5. Animation programs: run 2 s in a pseudo-terminal"
cd "$W"
gcc $SAN "$(hdir 03)/src/main.c" -o hwalk; gcc $SAN "$ROOT/03_random-moving-numbers/output/main.c" -o cwalk -lm
for p in hwalk cwalk; do timeout 2 script -qc "./$p" /dev/null > $p.out 2> $p.err; echo "  $p: exit=$? bytes_written=$(wc -c < $p.out)"; san $p.out; done
rm -rf "$W"
