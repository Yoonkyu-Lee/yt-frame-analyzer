# INTENT

Owner: <human who approves changes to this file>
Approved: <date>

## Purpose
<one paragraph: what this program is for and who uses it>

## Intents
Each line is one thing the program must do. Code must trace to these IDs (TRACE.md).
Source = where this intent came from (doc, issue, user, log). Mark guesses as `assumed`.

Proof = what a person who does not read the code runs, and what they should see (the evidence an
agent attaches to a change for this intent).

| ID | Intent | Source | Test | Proof (run this → see that) |
|---|---|---|---|---|
| I1 | <e.g. list a directory tree, deepest entries indented 2 spaces per level> | <README / issue #12> | tests/intent/i1_* | `./tree fixtures/deep` → same lines as `tree fixtures/deep` |
| I2 | | | | |

## Scale and resource assumptions
Named limits the code may rely on. Exceeding them is a new intent, not a bug.

| Assumption | Value | Where in code |
|---|---|---|
| <max keys in table> | <1,000,000> | <`SIZE` in hash.c> |

## Trust boundaries
Where input comes from outside and must be validated (everything else trusts its callers).
- <CLI arguments>
- <network requests>

## Non-goals
Things this program deliberately does not do. Agents must not add them.
- <e.g. GNU option compatibility>
- <e.g. sorting output (costs memory on huge directories)>
