# LEDGER

Why the code looks the way it does. Every Keep/Drop/Add decision and every measurement that
justified growth lives here, so nobody has to reverse-engineer intent from code.

## Baseline (phase 0)
| Metric | Value | Date |
|---|---|---|
| Source lines | | |
| Files | | |
| External dependencies | | |
| Tests / intent coverage | | |
| Intent density | | |
| Benchmarks (path: time, memory) | | |

## Decisions
| Date | Item | Decision | Reason / evidence | Decided by |
|---|---|---|---|---|
| | `cat -v` option | Drop | no intent, no usage in logs | <owner> |
| | sorting in tree | Keep as `-s` only | I4; default streaming keeps memory flat (bench/tree.txt) | <owner> |
| | hash table size 65536 | Add | 100k keys: 18.1s → 0.008s (bench/hash.txt) | measurement |

## Doubts
Every "why is it like this?" goes here and stays open until a measurement answers it. A doubt is
never closed by argument. Kinds: **premise** (would we build it this way if we had known this
requirement from the start?), **alternative** (a baseline way exists: existing code, stdlib,
platform, known method, the real tool), **patch** (a guard, retry, timer or comment covering a cause),
**cost** (slow, large, wasteful in an intent scenario).

| ID | Date | Kind | Doubt (where, what) | Duel: baseline vs ours (measured) | Decision | Ratchet (check that fixes it in place) |
|---|---|---|---|---|---|---|
| D1 | | alternative | `find` expression parser, 252 lines | fnmatch(3): 1 call; <same results on tests/intent/i5_*?> <time on bench/find?> | <recast piece I5 / keep> | <lint: no new pattern parsers in src/> |
| D2 | | patch | `setTimeout(…, 0)` in the render path with "// release later, in case it comes back" | <root cause found? does removing the timer pass i3?> | <fix / owner> | <lint: no timers in render code> |
| D3 | | cost | <listing a large tree is slow> | <real `find` on the same fixture: time, memory> | <open: owner decides the scale assumption> | <bench in CI with a threshold> |

## Dependencies
| Dependency | Why the platform was not enough | Added |
|---|---|---|
