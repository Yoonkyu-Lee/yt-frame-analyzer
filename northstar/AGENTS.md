# AGENTS.md — Intent Density rules (Recast)

Copy this file to the root of a project you are recasting (also as `CLAUDE.md` for Claude).
Recast = re-pour existing code into the mold of its intent: behavior is not preserved but
re-decided against `INTENT.md` (non-goals removed, defects fixed, intent gaps decided by the owner).
Background and evidence: NORTHSTAR.md, WORKFLOW.md (in the repo this file came from).

## The goal
Smallest code that (1) passes every intent test, (2) stays within the measured resource budget,
(3) defends trust boundaries. Code may grow only to satisfy one of those three. Everything else is
accidental and is a deletion candidate.

## Before you change anything
1. Read `INTENT.md`. If the change does not serve an intent ID listed there, stop and ask the
   intent owner (the human). Never invent requirements, options or features.
2. Read `TRACE.md` to find the files that implement the intent you are touching. Read those files,
   not the whole repo. If a file you need is missing from `TRACE.md`, add it with its intent ID.
3. Check `INTENT.md` non-goals. Do not implement anything listed there, even if it looks useful.

## While changing code
4. **Smallest verified step.** Make a small diff; build, run intent tests and sanitizers after
   each step. Grow from the simplest working version.
5. **Delete before you add.** For a bug, find and write the root cause first. Prefer a fix that
   removes code or merges duplicated paths. If you must add a special case, write why no deletion
   fix exists.
6. **Platform first.** Use the language standard library and the OS before writing your own.
   A new third-party dependency needs a `LEDGER.md` entry with the reason.
7. **No abstraction before the second real use.** No interface with one implementation, no wrapper
   used once, no `utils`/`common` module unless it has two or more callers *and* reduces total lines.
8. **Assumptions as named constants.** State limits (sizes, scale) as constants with a one-line
   comment instead of building general machinery nobody asked for.
9. **Defend at trust boundaries only.** Validate input where it enters (user, network, public API).
   Inside, trust your own functions; do not repeat checks for states that cannot happen.
10. **Measure before growing for speed.** Performance-motivated additions need a before/after
    benchmark recorded in `LEDGER.md`. Robustness additions need a failing test or sanitizer report first.
11. **One error strategy, one style.** Follow the existing project conventions exactly; do not
    introduce a second way of doing the same thing.
12. **Units fit one screen.** Functions about 40 lines, one concept per file. Explain exceptions.

## Tests
13. Tests describe intended behavior (inputs → observable outputs), named by intent ID. Do not
    write tests that pin implementation details. A behavior with no intent is not protected by a
    test; it is recorded as a non-goal or dropped.
14. When remaking a unit, freeze its intended behavior with characterization tests *before*
    deleting anything.
15. Passing tests do not prove nothing intended was lost: compression removes exactly what the
    tests do not pin. After a deletion pass, have a different reviewer compare old vs. new against
    `INTENT.md` (read-only). Turn each reproducible finding into an `INTENT.md` line and a test
    *before* fixing it.
16. Never dismiss a review finding by argument. Dismiss it only with a test showing the failure
    cannot happen, or escalate it to the intent owner (an `INTENT.md` gap or an accepted trade-off
    written into the file header's `Weak:` line). Reasoned dismissals came back with counterexamples.
17. Every fix ships with a test that **failed before the fix** (if it passes before the fix, it does
    not reproduce the problem; fix the test first). After the fix, revert the fixed lines once and
    confirm that test fails again (mutation check). Record it as `Mutation:` in the change note.
    Run timing-sensitive tests several times.
18. Every source file starts with a five-line honest header:
    ```
    Intent:  the intent IDs this file serves
    Data:    the data representation it is built around
    Assumes: named assumptions (sizes, ordering, trust)
    Weak:    real limitations a reader must know (what breaks, when, and what it costs)
    Cost:    time/space per operation, requests, writes
    ```
    `Weak:` lists accepted trade-offs and unhandled cases. It is not modesty: hiding a weakness
    there is a defect. Each `Weak:` item names the test or the LEDGER doubt (`D<n>`) that tracks it.

## Doubt, then duel, then lock in
19. **Lessons go into machine checks, not prose.** When a reviewer finds a class of defect, or the
    owner has to correct you twice about the same thing, add the cheapest check that makes the
    whole class impossible or loud: a lint rule with an error message that names the real fix, a
    type, a test, an import ban between folders, or removing the API that invites the mistake.
    Do not add a line to AGENTS.md / CLAUDE.md for something a check can enforce. Record it as
    `Ratchet:`. Block new occurrences first, then clean up the old ones in separate changes.
20. **No excuse comments.** Never write a comment that explains why a workaround, guard, retry,
    timer or "temporary" path is acceptable (`// in case it comes back`, `// for now`, `// TODO`).
    The next agent copies the workaround as the house pattern. Fix the cause, or stop and add a
    LEDGER doubt for the owner. The gate warns on these comments.
21. **Duel the baseline before you add.** Before a new source file or growth over budget, try the
    baseline ways in order: existing code in this repo, the standard library, a platform feature,
    a well-known method or tool. Measure size and the intent-derived benchmark for the baseline
    and for yours. Write the result as `Alternatives:`. "Ours is more flexible" is not a result.
22. **Answer doubts by measurement.** A doubt about a premise, an alternative, a patch or a cost
    (LEDGER Doubts) is closed only by a measured duel or by the owner. Never by explaining why the
    current code is fine.
23. **Evidence is something you ran.** `Evidence:` is the output of the real program path the intent
    names (INTENT `Proof` column): command output, a screenshot, a benchmark. The owner may not
    read code; the evidence is what they review.

## Every change ships with a change note (see templates/CHANGE.md)
```
Intent: I3
Root cause: (bugs only) ...
Deletion considered: (bugs only) ...
Budget: +N lines because <intent | measurement | trust boundary>   (omit if net lines <= 0)
Alternatives: (new file / over budget) baseline ways, measured, and why this one won
Evidence: what you ran and what it showed
Mutation: (bugs only) the test that failed before the fix and fails again when it is reverted
Ratchet: (bugs, repeated corrections) the check that keeps the class out, or `none: <why>`
```
Run `python gate.py --base <ref>` before proposing the change; it must pass (FAIL lines) and every
WARN line must be answered in the note.

## Never
- Add options, flags, config, formats, caching, sorting or buffering that no intent asks for.
- Keep code "just in case". Git remembers it.
- Silence a failing test or a warning instead of fixing the cause.
- Patch over a cause with a timer, retry, sleep, catch-all or extra guard and explain it in a comment.
- Mark a change done without running the build, the intent tests and the gate.
