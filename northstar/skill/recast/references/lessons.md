# Lessons behind the rules (condensed)

1. Slop has kinds; tools differ by kind (triage.md). Feature slop: non-goals give the gain.
   Verbosity: any minimizer works. Representation: only re-derivation works.
2. Compression only deletes; it never changes a data representation. Hybrid won.
3. The oracle is the most expensive part and has bugs too; log and re-run every change.
4. Passing tests ≠ intent kept. Reviews found regressions in every round; fixes create new ones.
   Converge on severity, not on zero findings.
5. Never dismiss a finding by argument.
6. A repro test must fail before the fix; a mutation check proves it guards the fix.
7. A recast is also an audit: the real-use-validated original had 7 defects; only the arms that
   had an intent found them; the naive control and the first oracle missed them.
8. Minimal ≠ monotonically shrinking: 556 → 371 → 452 lines; the 81 added lines all serve intents.
   371 was not minimal, it was missing things.
9. INTENT grows: reviewer findings exposed gaps (stall timeouts, retries, durability) that only the
   owner could decide.
10. The honest header's `Weak:` line surfaced a real hidden weakness; dynamic designs trade
    predictability for speed.
11. Some things show only in real use (real CDNs, multi-GB resume, missing observability).

Cost profile from the app pilot: rewriting took minutes per arm; building and fixing the oracle and
four review rounds took most of the time. Scope units accordingly.

Hypotheses from an external talk (poteto, "All Code Is Slop (Except Mine)", 2026-11), not yet
confirmed by a pilot of ours; enforced only as gate WARNs:
12. Code is the next author's textbook: an excused workaround gets copied as the house pattern
    (one timer patch → 183 timers in six weeks). Lock lessons into machine checks, not prose.
13. A recast that is never re-triggered decays. Keep a standing doubt loop: doubt → measured duel
    against a baseline way → decision → ratchet (references/doubt.md).
