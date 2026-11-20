#!/bin/sh
# Regression test: build every program, run it, check what we see is what we expect.
#   sh tests.sh          (needs cc and a POSIX shell)
D=$(cd "$(dirname "$0")" && pwd)
T=$(mktemp -d); cd "$T" || exit 1
pass=0; fail=0
check() { # name, expected, actual
  if [ "$2" = "$3" ]; then pass=$((pass + 1)); else fail=$((fail + 1)); echo "FAIL $1: expect [$2] got [$3]"; fi
}

cc -o tree "$D"/02_*/src/main.c
for u in touch cat mkdir mv rm find; do cc -o "$u" "$D"/05_*/src/"$u".c; done
cc -o list "$D"/07_*/src/main.c
cc -o slice "$D"/08_*/src/main.c
cc -o hash "$D"/09_*/src/main.c

# tree
mkdir -p t/b t/a; touch t/a/x t/c
check "tree -s sorted" "a
  x
b
c" "$(./tree -s t)"
check "tree missing path" 1 "$(./tree /nope 2>/dev/null; echo $?)"

# touch, cat
echo keep > k; ./touch k new
check "touch keeps content" keep "$(./cat k)"
check "touch makes file" yes "$([ -f new ] && echo yes)"

# mkdir
./mkdir -p p/q/r
check "mkdir -p" yes "$([ -d p/q/r ] && echo yes)"
check "mkdir -p on existing" 0 "$(./mkdir -p p/q; echo $?)"
check "mkdir on existing" 1 "$(./mkdir p 2>/dev/null; echo $?)"

# mv
echo new > n; echo old > o; ./mv n o
check "mv over file" new "$(./cat o)"
./mv o p
check "mv into dir" new "$(./cat p/o)"

# find
touch p/q/main.c p/q/note.txt
check "find glob" "p/q/main.c" "$(./find '*.c' p)"
check "find on file" "p/q/main.c" "$(./find 'main.*' p/q/main.c)"

# rm
check "rm dir without -r" 1 "$(./rm p 2>/dev/null; echo $?)"
./rm -r p
check "rm -r" no "$([ -e p ] && echo yes || echo no)"
./rm k
check "rm file" no "$([ -e k ] && echo yes || echo no)"

# data structure demos
check "list" "$(printf '10 20 30 \nsub = -40')" "$(./list)"
check "slice" "Hello World Hayyoth " "$(./slice)"
check "hash" "22
77
33" "$(./hash)"

echo "$pass passed, $fail failed"
cd / && rm -rf "$T"
[ "$fail" -eq 0 ]
