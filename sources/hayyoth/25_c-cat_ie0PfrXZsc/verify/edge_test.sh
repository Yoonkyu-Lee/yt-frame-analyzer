#!/bin/bash
# Behaviour checks for the reconstructed better-cat (src/main.c). Run from this folder in WSL.
# gnu11, not c11: lstat/DT_DIR are POSIX/BSD extensions (he built with plain `cc` on macOS).
set -u
B=/tmp/hcat; gcc -std=gnu11 -Wall -Wextra -pedantic -fsanitize=address,undefined -o $B src/main.c || exit 1
export ASAN_OPTIONS=detect_leaks=1
W=$(mktemp -d); cd $W
mkdir -p proj/src proj/.git proj/empty; printf 'a\nb' > proj/run.js; printf 'x{}\n' > proj/src/style.css
printf 'secret\n' > proj/.env; printf 'obj\n' > proj/.git/HEAD; head -c 5000 /dev/urandom > proj/src/blob.bin
ln -s src proj/link_to_src
echo "## 1. two files (last line without newline is no longer glued)"; $B proj/run.js proj/src/style.css
echo "## 2. directory walk: hidden .env/.git skipped?"; $B proj | grep -a '====='
echo "## 3. '.' as argument"; (cd proj && $B . | wc -c)
echo "## 4. './' as argument";  (cd proj && $B ./ | grep -c '=====')
echo "## 5. missing path"; $B nope 2>&1
echo "## 6. symlink to dir"; $B proj/link_to_src 2>&1 | head -3
echo "## 7. 5000-byte binary copied exactly?"; $B proj/src/blob.bin | tail -n +3 | head -c 5000 | cmp - proj/src/blob.bin && echo same
echo "## 8. no args"; $B; echo "exit=$?"
rm -rf $W
