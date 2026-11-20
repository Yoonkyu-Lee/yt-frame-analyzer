---
name: recast
description: Recast an existing module or subsystem (often AI-written "vibe-coded" code) into small, intent-faithful, transparent code. Behavior is not preserved as in refactoring; it is re-decided against a written INTENT.md (non-goals removed, defects fixed, intent gaps decided by the human owner), guarded by an oracle built first (hidden intent tests, benchmarks) and by repeated read-only skeptic reviews. Use when the user asks to recast, remake, deeply clean up, "de-slop", or rebuild a piece of existing code in a minimal, intent-driven style, or to continue a recast that already has INTENT.md / ORACLE_CHANGES.md / ROUND.md files. Not for small refactors, renames or feature work.
---

# Recast

Re-pour existing code into the mold of its intent. The mold (INTENT.md + an oracle) is built first
and checked against the original; the core is re-derived where measurements show a structural
problem; the periphery is compressed; a different model reviews the result until no high-severity
regression remains; then it is integrated on a branch and merged only with the owner's approval.

Evidence behind every rule here: two pilots (a synthetic 812-line utility set; a real app module
that was merged after real use). Lessons are condensed in `references/lessons.md`.

## Ground rules (always)

1. **Work on a branch or worktree.** Never touch the main working tree, user data, or remote
   (no push) unless the owner says so. Copy user data fixtures; never use the originals.
2. **The owner decides intent.** Stop and ask at every ⛔ below. Never invent requirements.
3. **No dismissal by argument.** A reviewer finding is fixed, pinned by a test that shows it cannot
   happen, or escalated to the owner (intent gap / accepted trade-off disclosed in `Weak:`).
4. **A fix needs a test that failed before it**, and a mutation check after it
   (`scripts/mutate.py`): revert the fixed lines once, the test must fail again.
5. **Size is a result, not the goal.** Every line must point to an intent ID, a measured need, or a
   trust boundary. Lines may grow during skeptic rounds; that is expected.
6. **Lock lessons into checks, not prose** (hypothesis). A defect class found, or a correction
   given twice → the cheapest machine check that keeps the class out (`Ratchet:` in the change note).
   Never write a comment that excuses a workaround; the next agent copies it.
7. **Answer doubts by measurement.** "Why this way?" is answered by a duel against a baseline way
   (existing code, stdlib, platform, known method, real tool), never by an explanation.
8. **Record everything in files**, so the work can stop and resume in another session:
   `INTENT.md`, `TRACE.md`, `LEDGER.md`, `ORACLE_CHANGES.md`, one `ROUND.md` per round.

## Where am I? (resume)

Read the recast folder (default `recast/<unit>/` next to the code, or wherever the owner keeps it):

| Present | Next stage |
|---|---|
| nothing | 0 Baseline |
| baseline, no approved `INTENT.md` | 1 Intent |
| approved `INTENT.md`, no oracle | 2 Oracle |
| oracle green on the original, no slop type in `TRACE.md` | 3 Triage |
| triage done, no candidate | 4 Recast |
| candidate, last `ROUND.md` not at stop rule | 5 Skeptic round |
| stop rule met | 6 Integrate |
| merged | 7 Standing doubt loop (`LEDGER.md` → Doubts) |

## Stages

### 0. Baseline
Branch/worktree, measure the unit (lines, tests, coverage, benchmarks, static analysis), copy the
original into `arms/r0/`. Pick a **unit** small enough to finish: one module or one tightly coupled
subsystem. Estimate cost before starting; most of it is stages 2 and 5.

**Fork: remake or recast?** (hypothesis) Same procedure; only the unit size differs. A remake makes
the whole project one unit and still runs stages 1–6 (never skip intent and oracle). Remake when the
skeleton itself carries a wrong premise (the same patch shape spread over many units), the intent
changed so the public contract changes and Drop exceeds half the features, the original holds little
real-use knowledge, or no seam lets a unit be cut out. Recast when the project is large, in real use,
and user data or other code depends on its contract, or development cannot stop. Under ~1k lines the
two are the same. Even a remake keeps the original beside it and runs the oracle on it first.
Evidence is module-scale only; a project-scale remake is untested. ⛔ Owner approves the choice
(one line in `LEDGER.md` Decisions).

### 1. Intent — `references/intent.md`
Draft INTENT.md from docs, commits, issues, callers, usage, and vibe-coding prompt history.
Every item gets an ID and a source; write the **public contract** callers depend on and the
**non-goals**. ⛔ Owner approves INTENT.md (especially non-goals and scale assumptions).

### 2. Oracle — `references/oracle.md`
Build what builders will not see: hidden intent tests through the public contract only, a
scriptable fake for external parties **including misbehaving ones**, intent-derived benchmark
scenarios (not just the average case). Run it on the original first. A failure is either an
original defect (record it) or an oracle defect (fix it, log it in `ORACLE_CHANGES.md`). Never make
a test stricter than INTENT.md.

### 3. Triage — `references/triage.md`
Classify the slop of each unit: **feature** (code no intent test executes → compress against
non-goals), **verbosity** (covered but long → any minimizer works), **representation** (an
intent-derived benchmark is bad → re-derive the core). Write it into `TRACE.md`.

### 4. Recast — `references/recast.md`
Default strategy is **hybrid**: re-derive the core data representation from INTENT, compress the
periphery top-down. Optionally run competing arms (compress / re-derive / hybrid / naive control)
with `prompts/arm_*.md`, each in its own workspace without the hidden tests. Every file gets the
five-line honest header (`Intent / Data / Assumes / Weak / Cost`).

### 5. Skeptic rounds — `references/skeptic.md`
Another model, read-only, compares original, candidate and INTENT.md (`prompts/skeptic.md`).
Classify each finding (regression / inherited / intent gap / accepted trade-off); turn reproducible
ones into failing tests first; fix; mutation-check; review again. ⛔ Intent gaps go to the owner;
approved answers become an INTENT revision. **Stop rule:** the last round found no high-severity
regression and the remaining ones are pinned by tests.

### 6. Integrate — `references/integrate.md`
Put the candidate on the branch; run the **whole** project's tests and static analysis; compare
against the original on real external targets the fake cannot imitate; make sure diagnostics exist
**before** the owner's smoke test; sync project docs; write a transition note if a data format
changed. ⛔ Owner approves merge (and migration vs. transition procedure). Never push unasked.

### 7. Standing doubt loop — `references/doubt.md` (hypothesis)
After the merge, keep the loop running: every change passes the gate (`Ratchet:`, `Alternatives:`,
excuse-comment WARNs); every release a different model runs the root-doubt prompt and adds Doubts;
periodically close at least one doubt by a measured duel and lock the answer into a check.

## Files in this skill

| Path | Use |
|---|---|
| `references/*.md` | Detailed procedure per stage; read the one for the current stage |
| `templates/` | INTENT, TRACE, LEDGER, CHANGE, ORACLE_CHANGES, ROUND |
| `prompts/skeptic.md` | Reviewer prompts: regression review (stage 5) and root doubt (stage 7) |
| `prompts/arm_compress.md`, `arm_rederive.md`, `arm_hybrid.md`, `arm_control.md` | Briefs for competing arms |
| `prompts/reader.md` | Blind static-reader probe (optional transparency measurement) |
| `scripts/gate.py` | Change gate: trace, intent, budget, deps, fix cause, mutation note, ratchet, alternatives, excuse comments |
| `scripts/mutate.py` | Revert-a-fix mutation check: apply a text change, run tests, always restore |
