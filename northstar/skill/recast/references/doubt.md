# Stage 7 — Standing doubt loop (hypothesis: NORTHSTAR L13–L16)

A recast is an event; slop grows back between events. After integration keep a loop that raises
doubts without anyone deciding to, answers them only by measurement, and locks each answer into a
machine check.

## One cycle: doubt → duel → decide → ratchet
1. **Doubt**: write one line in `LEDGER.md` → Doubts with a concrete location and a kind:
   - premise ("built along the way"): would we build it this way had we known this requirement
     from the start? Two failed fixes on one premise → suspect the premise.
   - alternative ("why this way"): existing code, stdlib, platform feature, well-known method,
     the real tool. How many lines and how fast would that be?
   - patch: what cause does this guard / retry / timer / sleep / excuse comment cover? How many
     copies of the same shape exist?
   - cost: in an intent scenario, is time / memory / requests / size worse than the baseline or
     the alternative?
2. **Duel**: build or find the baseline version and run it and ours through the same oracle
   (intent tests, intent-derived benchmark). Never answer a doubt with an explanation.
3. **Decide**: keep (record the numbers), recast that piece (stages 3–6 on a small unit), or
   ⛔ escalate to the owner (INTENT revision).
4. **Ratchet**: add the cheapest check that keeps the class out: lint rule whose message names the
   real fix, type, test, import ban, API removal. Block new occurrences first; clean old ones in
   separate changes. Fill the Ratchet column. If no check is possible, say why and keep the doubt open.

## Rhythm
| When | What |
|---|---|
| every change | `scripts/gate.py` (ratchet / alternatives / excuse WARNs); CI compares intent benchmarks and size with the baseline |
| every release | a different model, read-only, runs `prompts/skeptic.md` → "Root-doubt prompt" on the most-changed areas and adds Doubts; an agent drives the real app like a user (fuzzing) and collects evidence |
| fixed period (e.g. monthly) | close at least one open doubt by a duel; delete at least one thing; check whether any prose rule (AGENTS.md, `Weak:` lines) can become a check |
| new feature | INTENT line with `Proof` → test → duel the baseline → code |
