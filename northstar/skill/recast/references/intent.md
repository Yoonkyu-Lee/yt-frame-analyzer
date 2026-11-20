# Stage 1 — Intent

Goal: an INTENT.md the owner approves. Everything later is judged against it.

## Sources, strongest first
1. Vibe-coding prompt/conversation history (the human's words; code is the AI's interpretation).
2. Usage evidence: logs, real call paths, issues, bug reports, options people actually use.
3. Human-written artifacts: README, design docs, commit messages (causes and measurements).
4. The code itself: reachability from entry points, callers. Good for candidates, weakest as proof.
   AI-written tests are not evidence of intent.

## What to write (templates/INTENT.md)
- Purpose in two sentences.
- **Public contract**: signatures and types callers depend on (copy them exactly).
- Intents as a table: `ID | intent | test`. Behavior, not implementation. Include limits that a
  user would notice (e.g. "never more than 8 connections", "cancel within ~1 s").
- Scale and assumptions (sizes, concurrency, "same URL serves the same bytes").
- Trust boundaries (inputs, external servers, file system).
- **Non-goals**: features and options deliberately not done. This list is what makes compression
  possible; without it every feature looks required.
- Revision log (later revisions after skeptic rounds go here, with owner approval).

## Classifying existing features
| Class | Meaning | Handling |
|---|---|---|
| Keep | linked to an intent | pinned by a test |
| Merge | same intent implemented in several places | one implementation |
| Drop | no intent; owner decided | recorded in LEDGER, deleted |
| Unknown | cannot tell | characterization test; ask the owner with a yes/no question |

Ask the owner **only** about Unknowns, as forced choices with the evidence attached
("`find -mtime`: not in the prompts, never used in logs, no caller — drop?").

## Stop
⛔ The owner approves INTENT.md. Record the date and who approved in its header.
