# Common brief for every arm (prepend to the arm-specific brief)

You are one arm of a controlled comparison. Your result will be measured by tests you cannot see.

- Workspace: `<arm workspace>` — a harness copy **without** hidden tests. Edit only
  `<arm workspace>/arm/<module>`. Build/test: `<own test command>`; benchmark: `<bench command>`
  (baseline: `<numbers>`).
- Size metric: `<how lines are counted>`. Original: `<n>`.
- Read first: `INTENT.md` (the oracle), the style references `<NORTHSTAR.md, AGENTS.md, a style
  philosophy file>`.
- Forbidden: reading the hidden tests, other arms, the project repository, or anything outside
  your workspace except the files named above. Do not change the public contract. Do not add
  dependencies.
- Own tests live inside the module and must be self-contained (the real project does not have the
  harness). Pin every intent ID with at least one own test.
- Comments in the project's language; keep only intent-ID and "why" comments. Start the file with
  the five-line header (`Intent / Data / Assumes / Weak / Cost`); `Weak` must state real limits.
- Deliver: `<arm folder>/<module>` (final copy) and `<arm folder>/ROUND.md` (steps with code lines,
  own tests, benchmark after each; decisions with intent IDs; open doubts). Final message: final
  numbers you actually ran, and what you were unsure about.
