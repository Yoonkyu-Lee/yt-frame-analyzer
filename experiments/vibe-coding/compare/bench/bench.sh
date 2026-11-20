#!/usr/bin/env bash
# Performance: human (hand-completed, or hand-remade with "remade") vs Codex (vibe-coding), both -O2, no sanitizers.
#   wsl.exe -e bash compare/bench/bench.sh [remade]     (run from vibe-coding/)
set -u
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"; HC="$ROOT/hand-completed"; B="$ROOT/compare/bench"; HD=""
HRM=""
[ "${1:-}" = remade ] && { HC="$ROOT/hand-remade"; HD="-DREMADE"; HRM="-r"; }
echo "# human code: $HC"
W="$(mktemp -d)"; cd "$W"
t() { /usr/bin/time -f "%e s, maxRSS %M KB" "$@" 2>&1 >/dev/null | tail -1; }
src() { ls "$HC"/"$1"_*/src/"$2"; }
X=(-Wno-unused-function -Wno-unused-variable)

echo "## data structures"
for k in 07 08 09; do
  case $k in 07) lib="$ROOT/07_linked-list/output"; c=list.c;; 08) lib="$ROOT/08_dynamic-array/output"; c=dynarray.c;; 09) lib="$ROOT/09_hash-table/output"; c=hash_table.c;; esac
  sed 's/^int main(void)/int human_main(void)/' "$(src $k main.c)" > h$k.c
  gcc -O2 $HD -DT$k -include h$k.c "$B/hb.c" -o hb$k 2>/dev/null
  gcc -O2 -DT$k -I"$lib" "$B/cb.c" "$lib/$c" -o cb$k
done
for n in 20000 100000 1000000; do timeout 60 ./hb07 $n || echo "07 human  n=$n  >60s"; ./cb07 $n; done
for n in 10000000; do ./hb08 $n; ./cb08 $n; done
ulimit -s unlimited
for n in 20000 100000 1000000; do timeout 60 ./hb09 $n || echo "09 human  n=$n  >60s"; ./cb09 $n; done

echo; echo "## file utilities"
mkdir hu cu
for u in cat find rm; do gcc -O2 "$(src 05 $u.c)" -o hu/$u; (cd "$ROOT/05_linux-file-utils/output" && gcc -O2 -I. $u.c common.c -o "$W/cu/$u" -lm); done
head -c 200000000 /dev/urandom > big.bin
echo "  cat 200MB > /dev/null:  human $(t hu/cat big.bin)   codex $(t cu/cat big.bin)"
mk() { mkdir -p "$1"; for d in $(seq 1 100); do mkdir -p "$1/d$d"; (cd "$1/d$d" && seq 1 500 | sed 's/$/.c/' | xargs touch); done; }
mk ft; echo "  find in 50k files:      human $(t hu/find 77.c ft)   codex $(t cu/find ft -name '*77.c')"
mk r1; mk r2
echo "  rm 50k files:           human $(t hu/rm $HRM r1)   codex $(t cu/rm -r r2)"
echo; echo "## tree, 300k files in one dir"
gcc -O2 "$(src 02 main.c)" -o htree; gcc -O2 "$ROOT/02_tree/output/main.c" -o ctree
mkdir big && (cd big && seq 1 300000 | xargs touch)
echo "  human stream: $(t ./htree big)"
[ -n "$HD" ] && echo "  human -s    : $(t ./htree -s big)"
echo "  codex       : $(t ./ctree big)"
rm -rf "$W"
