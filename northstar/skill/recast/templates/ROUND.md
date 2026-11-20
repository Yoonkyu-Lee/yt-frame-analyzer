# ROUND <n>: <what this round did>

## Work
What was deleted, rewritten or re-derived, with the intent IDs each change serves.

## Reviewer findings (skeptic round)
Reviewer: <other model>, read-only, given the original, this version and INTENT.md.

| # | Finding | Class | Handling | Pinned by (mutation check) |
|---|---|---|---|---|
| 1 | … | regression | fixed: … | `test_name` (fails before the fix; fails again when the fix is reverted) |
| 2 | … | inherited | fixed / recorded | hidden `test_name` (original fails too) |
| 3 | … | intent gap | owner decision → INTENT revision n | — |
| 4 | … | accepted trade-off | disclosed in header `Weak:` | — |

Classes: **regression** (introduced by this work), **inherited** (also in the original),
**intent gap** (INTENT.md does not say; the owner decides), **accepted trade-off** (known cost of the
design, disclosed). No finding is dismissed by argument alone.

## Measurements
| | previous | this round |
|---|---|---|
| code lines | | |
| own tests | | |
| hidden tests (n) | | |
| benchmark scenarios | | |
| reviewer: new regressions (highest severity) | | |

## Stop?
Stop when the last review found no high-severity regression and the remaining ones are pinned by tests.
