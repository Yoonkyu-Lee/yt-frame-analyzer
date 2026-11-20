# Stage 5 — Skeptic rounds

Passing tests do not prove nothing intended was lost, and every fix can create a new regression.
In the app pilot the reviewer found new regressions 7 → 2 → 2 → 1 times across four rounds, all in
versions that passed every hidden test.

## Run a review
- Reviewer = a different model, **read-only**, in an isolated folder containing only
  `before/<module>`, `after/<module>`, `INTENT.md`. Prompt: `prompts/skeptic.md`.
- Codex: `codex exec --skip-git-repo-check --ephemeral -s read-only -C <dir> --json -o <out.md> "<prompt>" < /dev/null`
  (close stdin; without it the CLI can wait forever for input).
- No Codex: spawn a fresh Claude agent that has not seen this conversation, give it only that folder.

## Handle each finding (record in ROUND.md)
| Class | Handling |
|---|---|
| regression (introduced now) | fix |
| inherited (original has it too) | fix when cheap and data-safety related; otherwise record |
| intent gap (INTENT does not say) | ⛔ ask the owner; approved answer → INTENT revision, then tests |
| accepted trade-off | disclose in the header `Weak:` and in INTENT |

Never dismiss by argument. A finding dismissed with "the wrong bytes can't get in" came back the
next round with a counterexample.

## Fix procedure
1. Write the repro test. Implementation-independent → hidden test (run it on the original and all
   arms; an original failure confirms "inherited"). Format-specific → own test in the module.
2. **Confirm it fails before the fix.** If it passes, the test does not reproduce the problem
   (a repro server once sent the "late" bytes in the same chunk). Fix the test first.
3. Fix with the smallest change that serves the intent.
4. **Mutation check**: `python scripts/mutate.py --file <f> --find '<fixed text>' --replace '<old text>' -- <test cmd>`.
   The test must fail on the mutant. Record as `Mutation:` in the change note.
5. Re-run timing-sensitive tests several times; re-run the whole run.sh.

## Stop rule
Stop when the last round found no high-severity regression and the remaining regressions are pinned
by tests. Expect the code to grow here; every added line should cite an intent ID.
