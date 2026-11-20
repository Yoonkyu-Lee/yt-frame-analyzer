# R4: skeptic round (final)

Codex reviewed r3 against `INTENT.md` and r0 (read-only, static review:
[skeptic/codex_last_message.md](../../skeptic/codex_last_message.md)). Every finding that could be
reproduced became a test first (INTENT revision 1). The new tests were then run on all three versions:

| New test | r0 | control | r3 | Origin of the r3 failure |
|---|---|---|---|---|
| I2_write_error (`cat` to /dev/full, unbuffered) | pass | pass | **fail** | R2 dropped the `ferror(stdout)` checks |
| I2_write_error_stops (`cat /dev/zero > /dev/full`) | pass | pass | **fail (hangs)** | R2 |
| I6_write_error | pass | pass | **fail** | R2 |
| I6_all_tests_name / _type (repeated tests) | pass | pass | **fail** | R2: one variable per test kind |
| C2_dashdash_touch / cat / mv / find | pass | pass | **fail** | R2 dropped option parsing that handled `--` |
| I3_empty_operand (`mkdir -p '' x`) | pass | pass | pass, but **ASan: heap-buffer-overflow** | R3's `strchr(path + 1, …)` |
| I5_slash_on_file (`rm file/`) | **fail** | **fail** | **fail** | inherited from r0 |
| I4_slash_on_source / _target | **fail** | **fail** | **fail** | inherited from r0 |
| I6_slash_on_file | pass | pass | pass | |

## Fixes
| File | Change | Non-blank lines |
|---|---|---|
| common.c | `first_operand()` skips a leading `--` (4 callers); `operand_path()` keeps "file/" an error | +11 (with header) |
| cat.c | stop copying when `putchar` fails; check `ferror(stdout)` at the end | ±0 |
| find.c | tests evaluated straight from argv (`-name`/`-type` repeatable); `ferror(stdout)`; `operand_path` | +5 |
| mkdir.c | parent loop starts at `path`, guarded by `*p` | ±0 |
| mv.c | `rename` gets the operands as typed, so the kernel rejects "file/" | +3 |
| rm.c | `operand_path` | +1 |
| touch.c | `futimens` on the opened descriptor (skeptic #7: a file created between `utimensat` and `open` kept old times) | +3 |

## Findings not fixed
| # | Finding | Decision |
|---|---|---|
| 2, 10 | races: a directory swapped for a symlink between `lstat` and `opendir`; a path vanishing under `rm -f` mid-removal | inherited from r0 and present in the control. A real fix means descriptor-relative traversal (`openat`/`fdopendir`/`unlinkat`), which is a redesign, not compression. Recorded as an open intent question |
| 9 | `close`/`fclose`/`closedir` results ignored | all on descriptors that were only read, or on an empty file that was never written. Nothing the user could act on is lost (L12) |
| 11 | `find / -name /` does not match `/` | real but cannot be tested without walking the whole file system. Left recorded (no code without a test) |
| 12 | `refusing to remove '.'`, `destination must be a directory`, and getopt's own messages do not follow the C1 shape exactly | inherited; C1 is about *where* errors go and *whether* processing continues. The message shape is checked only for operand errors |

## Verify and record
| | r3 | r4 |
|---|---|---|
| code lines | 270 | **291 (+21)** |
| intent tests (plain / ASan+UBSan) | 62/74, ASan report 1 | **74/74, 0 reports** |
| coverage by intent tests | 99.4% | 98.9% |
| cat 200MB / find 50k / rm -r 50k | 0.51 / 0.04 / 0.21 s | 0.54 / 0.04 / 0.22 s |
