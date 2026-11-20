# R2 — function level

Each function got a one-line purpose header (scaffolding) naming its intent. Functions and
options whose header could not name an intent were removed; the rest kept the project's own
style (4-space indent, `error_path`, `clean_path`).

| File | Removed (non-goal per INTENT.md) | Kept |
|---|---|---|
| touch.c | `-a -m -c -h -r -t`, `--help`, `timestamp()` date parser (22 lines), option parsing | I1: `utimensat(NOW)`, create with `O_CREAT` (no truncation) |
| cat.c | `-n -b -s -v -E -T -A -e -t -u`, `--help`, the per-byte display state machine | I2: byte copy, `-`/no operands = stdin |
| mkdir.c | `-v`, `--help`, `getopt_long` → `getopt` | I3: `-p`, parent traversability under restrictive umask |
| mv.c | `-f -n -v -T`, same-file detection, option parsing | I4: rename, into-directory, many-sources check |
| rm.c | `-d -v -R`, `--help` | I5: `-r -f`, refuse `.`/`..`/`/`, `lstat` |
| find.c | expression language: `! -o ( )`, `-iname -path -size -mtime -mmin -empty -newer -print0 -delete -prune -mindepth -maxdepth -depth -P`, `number/compare/empty/syntax/evaluate` tree (≈190 lines) | I6: `-name`, `-type f|d`, default `.`, `lstat` walk |
| Makefile | `-lm` (only `floor()` in `-mtime` needed it), `tests.py` | intent tests |

## Verify and record
| | r1 | r2 |
|---|---|---|
| code lines | 678 | **283 (−58%)** |
| functions | 34 | 18 |
| intent tests (plain / ASan+UBSan) | 60/60 | 60/60 |
| coverage by intent tests | 64.4% | **98.9%** |
| cat 200MB | 1.86 s | **0.54 s** — the per-byte option branches were the cost |
| find / rm -r 50k | 0.04 / 0.21 s | 0.05 / 0.21 s |
