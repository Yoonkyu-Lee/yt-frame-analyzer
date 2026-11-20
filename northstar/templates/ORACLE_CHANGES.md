# ORACLE_CHANGES

Every change to the oracle (intent tests, hidden tests, fake servers and fixtures, benchmarks,
probes) after the first version under test was measured. The oracle has bugs too: a test can be
stricter than INTENT.md, a fake server can hide a blind spot, a repro test can fail to reproduce.
Recording each change, and re-running **every** version after it, keeps comparisons honest.

| # | When | What changed | Why | Effect on each version |
|---|---|---|---|---|
| 1 | before any version | e.g. fake server stopped counting its own sleep as an open connection | the original failed a test for an artifact of the server | original passes |
| 2 | after version X was measured | e.g. test relaxed to the allowance INTENT D4 states | the test was stricter than INTENT; X was within intent and failed | original and X both pass |

Rules
- Validate a new oracle on the original first. A failure is either an original defect (record it)
  or an oracle defect (fix it here).
- Never tighten a test beyond INTENT.md. If INTENT is too loose, change INTENT (owner approval) first.
- A change made after a version was measured is listed with "after …" in the When column.
