# R1 — module level

## 1. Summarize (one sentence per unit) → 2. compare with INTENT + tools

| Unit | Purpose | Intent | Tool evidence | Decision |
|---|---|---|---|---|
| `common.c` `error_path`, `init_program`, `base_name` | uniform error message, program name | C1 | used by 6 / 3 tools | keep |
| `common.c` `join_path`, `clean_path`, `xmalloc`, `xstrdup` | path building | I4–I6 | used by 2–4 tools | keep (shared, reduces lines) |
| `common.c` `copy_tree` (62 lines) | cross-file-system move by copying | non-goal | never called by intent tests | **delete** |
| `common.c` `confirm` | `-i` prompt | non-goal | never called | **delete** (+ `-i` in mv, rm) |
| `common.c` `parse_mode` | `mkdir -m` | non-goal | never called | **delete** (+ `-m` in mkdir) |
| `common.c` `remove_tree` | recursive delete | I5 | callers: rm, mv fallback; fallback gone → rm only | **move into rm.c** (L3: one caller) |
| `mv.c` staging block | copy-then-rename on `EXDEV` | non-goal | only reachable across devices | **delete**; `EXDEV` reported like any error |

## 3. Verify
- Intent tests 60/60 (plain and ASan/UBSan), no warnings.

## 5. Record
| | r0 | r1 |
|---|---|---|
| code lines | 812 | **678 (−16.5%)** |
| functions | 36 | 34 |
| coverage by intent tests | 53.8% | 64.4% |
| cat / find / rm -r | 1.77 / 0.04 / 0.21 s | 1.86 / 0.04 / 0.21 s |
