#!/usr/bin/env bash
# Replays the video's final session and probes edge cases of the reconstructed ed (src/main.c).
#   bash verify/edge_test.sh            (Linux/WSL, gcc)
# Leak detection is off for the replay because LeakSanitizer exits without flushing stdout.
set -u
D="$(mktemp -d)"; cp "$(dirname "$0")/../src/main.c" "$D/"; cd "$D" || exit 1
gcc -Wall -Wextra -pedantic -Wno-unused-parameter -fsanitize=address,undefined -g main.c -o ed11 || exit 1
export ASAN_OPTIONS=detect_leaks=0

echo "=== video session (68:39): a, 4 lines, ., 2, d, w line.txt, q"
printf 'a\nline 1\nline 2\nline 3\nline 4\n.\n2\nd\nw line.txt\nq\n' | ./ed11
echo "--- line.txt (video: line 1 / line 3 / ...)"; cat line.txt

echo "=== reopen: 1, 3, p, 4 (expect line 1, line 4, line 4, ?)"
printf '1\n3\np\n4\nq\n' | ./ed11 line.txt

echo "=== E1 delete the LAST line, then p (real ed: prints the new last line)"
printf 'a\nx\ny\n.\nd\np\nq\n' | ./ed11 2>&1 | grep -E '^(x|y|\?)$|ERROR: AddressSanitizer: [a-z-]+'

echo "=== E2 go to line 1, then a (real ed inserts after line 1)"
printf 'a\none\ntwo\nthree\n.\n1\na\nNEW\n.\nw e2.txt\nq\n' | ./ed11 > /dev/null; cat e2.txt

echo "=== E3 w with no file name and no file loaded (expect ?)"
printf 'a\nz\n.\nw\nq\n' | ./ed11

echo "=== E4 a 1500-char line (MAX_LINE_LEN 1024: split into pieces)"
head -c 1500 /dev/zero | tr '\0' x > long.txt; echo >> long.txt
printf '1\n2\nq\n' | ./ed11 long.txt | awk '{print length($0)}'

echo "=== leaks on exit (last line is never freed: loop uses i < count_line)"
ASAN_OPTIONS=detect_leaks=1 sh -c "printf 'a\nlast\n.\nq\n' | ./ed11" 2>&1 | grep -E 'SUMMARY' || echo "no leak"
rm -rf "$D"
