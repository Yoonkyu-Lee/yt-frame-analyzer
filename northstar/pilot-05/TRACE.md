# TRACE (final = rounds/r4)

| File | Intents | Lines | Notes |
|---|---|---|---|
| common.c / common.h | C1, C2, I4–I6 | 49 + 18 | `error_path` is the only error message format; `first_operand` (`--`); `operand_path` keeps "file/" an error; path helpers |
| touch.c | I1, C1, C2 | 29 | `utimensat(NOW)`; create with `O_CREAT` (no `O_TRUNC`), then `futimens` |
| cat.c | I2, C1 | 26 | byte copy that stops on a write error; `-`/no operands = stdin |
| mkdir.c | I3, C1, C2 | 48 | `-p` walks slash to slash; parents stay `u+wx` under any umask |
| mv.c | I4, C1, C2 | 28 | `rename` with the operands as typed; EXDEV is reported, not handled (non-goal) |
| rm.c | I5, C1, C2 | 65 | `lstat` (no following links); refuses `.`, `..`, `/`; `-f` with no operands exits 0 |
| find.c | I6, C1 | 64 | `-name`/`-type` pairs evaluated from argv (all must match); `lstat` walk; write errors → exit 1 |
| Makefile | — | — | build plus `make test` → `tests/intent.sh` |

Intent → tests (`tests/intent.sh`, 74 checks, named by intent ID: `C1_*`, `C2_*`, `I1_*` … `I6_*`).

No file is traced to `none`.
