# Stage 3 — Triage (what kind of slop is this?)

The right tool depends on the kind of slop. Measure before choosing.

| Kind | Signal | Tool | Pilot evidence |
|---|---|---|---|
| **Feature slop** — features nobody asked for | a large share of code is not executed by intent tests (coverage under intent tests) | compress against the non-goals list | synthetic utils: 46% unexecuted; non-goal judgment produced 97% of the reduction; a naive "minimize" control got 7% |
| **Verbosity slop** — the same work said at length | coverage is high but the code is long (extra sidecars, sentinel strings, wrappers, 9-argument functions) | compression; any reasonable method works | app module: naive control −33%, structured arms −33…−40% |
| **Representation slop** — wrong data representation or algorithm | an intent-derived benchmark is bad (e.g. one slow connection holds the tail) | re-derive the core | app module: only re-derivation fixed it (6.1 s → 2.3 s); compression and control stayed at 6.1 s |

Write per unit into `TRACE.md`: intents, slop kind, chosen tool, and the measurement behind it.

Also note the **cost** of the unit (lines, external dependencies to fake, protected zones) and pick
the order: smallest fan-in first, highest accidental share first, protected zones last and only with
the owner.
