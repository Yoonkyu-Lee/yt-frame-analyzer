# Stage 2 — Oracle

The recast loses intent exactly where the oracle has holes. This is the most expensive stage and
the one that decides whether later results can be trusted.

## Layout
```
recast/<unit>/
  INTENT.md  ORACLE_CHANGES.md
  harness/           # builds ONE candidate file in isolation (no full app build needed)
    arm/<module>     # copied in by run.sh; the candidate under test
    src/server.*     # scriptable fake for external parties
    tests/hidden.*   # hidden intent tests — builders never see this folder
    bench/           # intent-derived scenarios
  arms/r0/           # the original
  run.sh             # one command: size, warnings, own tests, hidden tests, bench (+ probe)
```
Builders get a copy of the harness **without** `tests/hidden.*`.

## Hidden tests
- Use only the public contract. Name each test by intent ID (`d5_...`).
- Make the fake able to misbehave: ignore the protocol feature, lie about sizes, cut connections,
  go silent with the connection open, send extra or late bytes, keep connections alive, redirect.
  Every real defect found in the pilots lived in one of these.
- Assert the intent, not an implementation detail. When the intent allows slack ("up to ~1 MiB per
  connection may be refetched"), the test must allow the same slack.

## Benchmarks
Average cases hide representation problems. Derive scenarios from what the intent worries about
(e.g. per-connection throttling **and** "every third connection is slow"). Take the median of 3;
interleave versions when comparing; re-run when other jobs share the machine.

## Validate on the original
Run everything on `arms/r0` first.
- A failure is either an **original defect** (keep the test; record it) or an **oracle defect**
  (fix the oracle; record it). Decide which before going on.
- Typical oracle defects seen: the fake counted its own sleep as an open connection; a question
  assumed a server behavior that contradicted its own wording; a test stricter than INTENT; a
  repro server that sent the "late" bytes in the same chunk and so reproduced nothing.

## ORACLE_CHANGES.md
Every change after the first version was measured: when, what, why, effect on each version.
Re-run **all** versions after each change. This is what keeps comparisons honest.

## Optional: static transparency probe
Questions whose answers are observed by execution (request counts, files left after cancel, time to
error, …) answered by a blind reader who sees only the module (`prompts/reader.md`). In the pilots it
hit a ceiling for single modules; use it for larger units or with human readers.
