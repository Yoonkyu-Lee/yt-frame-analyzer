# R3 — chunk level

gcov on r2 left two lines unexecuted: `find`'s "invalid expression" error and `rm`'s usage on a
bad option. Both validate arguments at the trust boundary, so they stay. This round read each
remaining chunk against the header of the function it belongs to.

| Chunk | Before | After | Why |
|---|---|---|---|
| `error_path` | `int saved = errno;` then `strerror(saved)` | `strerror(errno)` | arguments are evaluated before `fprintf` runs, so nothing can clobber errno first |
| `xmalloc` | `malloc(size ? size : 1)` | `malloc(size)` | no caller asks for 0 bytes (`join_path` ≥ 2, `xstrdup` ≥ 1) |
| `rm`/`find` readdir loop | `for(;;)`, errno reset, `if (!entry) { if (errno) …; break; }` | `while ((errno = 0, entry = readdir(dir)))`, check errno once after the loop | same contract (read errors are still reported, since the file system is a trust boundary), half the control flow |
| `rm` closedir | error report on `closedir` failure | plain `closedir` | `closedir` on a valid `DIR*` cannot fail in a way the user can act on (L12, impossible state) |
| `mkdir -p` parents | per-character scan with three skip conditions | `strchr` from slash to slash | rewritten from the header "create a, then a/b, …". With repeated slashes, `create` meets the same directory twice, which `-p` already accepts |

Kept even though they cost lines: the readdir errno checks, raising umask for parent directories,
refusing `.`, `..` and `/`, and checking `fflush` on stdout for `cat` and `find` (they detect a full disk or a closed pipe).

## Verify and record
| | r2 | r3 |
|---|---|---|
| code lines | 283 | **270 (−4.6%)** |
| functions | 18 | 18 |
| intent tests (plain / ASan+UBSan) | 60/60 | 60/60, 0 reports |
| coverage by intent tests | 98.9% | 98.9% |
| cat 200MB / find 50k / rm -r 50k | 0.54 / 0.05 / 0.21 s | 0.51 / 0.04 / 0.21 s |

## Stop rule
Gain fell below 5% (−4.6%), so the compression loop stops. One more pass would only trade readability for
characters. R4 then fixes what the skeptic review found.

Comments: every remaining comment either names the intent the code serves (`I1`…`I6`, `C1`,
`C2`), which makes it a trace link, or explains a *why* (umask, EXDEV, `a//b`). The round
removed none of them. No comment only restates *what* the code does, so there was no
scaffolding left to strip.
