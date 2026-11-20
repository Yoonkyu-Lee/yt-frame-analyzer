# Stage 4 — Recast

## Default: hybrid
- **Core** (data representation, scheduling, state that persists): re-derive from INTENT and the
  benchmark that showed the problem. Compression only deletes; it cannot change a representation.
- **Periphery** (I/O, request building, configuration, error wording): compress.

Write the target shape before code — the five-line header:
```
Intent:  intent IDs
Data:    the representation the code is built around
Assumes: named assumptions (sizes, ordering, trust)
Weak:    real limitations (what breaks, when, what it costs) — hiding one here is a defect
Cost:    time/space per operation, requests, writes
```
Grow from the simplest working version; pin each intent with an own test inside the module (tests
must not depend on the harness, since they ship with the project).

## Compression loop (periphery, feature and verbosity slop)
Rounds top-down: module → function → chunk. Each round:
1. one-line header per unit naming its intent ID (cannot name one → deletion candidate);
2. compare with INTENT and non-goals; 3. delete, or rewrite from the header without looking at the
body; 4. verify (own tests, hidden tests via run.sh, bench); revert any proposal that breaks them;
5. record in ROUND.md. Lines not executed by intent tests are the next round's map: non-intent
(delete), missing intent (add to INTENT + test, owner), trust-boundary defense (keep, say why).
Stop a compression pass when a round gains < 5%. Keep only intent-ID and "why" comments.

## Competing arms (optional, recommended for the first unit of a project)
Run in parallel, each in its own workspace without hidden tests (`prompts/arm_*.md`):
compression, re-derivation (INTENT + contract only, never the original), hybrid, and a naive
control ("make it minimal", no INTENT). Measure all with run.sh. The control tells you whether the
gain came from the procedure or from the model.

## Measure
`run.sh` per candidate: code lines (exclude tests, blanks, comments), warnings, own tests, hidden
tests, benchmark, optional probe. Keep each candidate in `arms/<name>/` with its ROUND.md.
