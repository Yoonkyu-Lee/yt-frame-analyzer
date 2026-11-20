# Stage 6 — Integrate

1. Copy the final candidate into the branch/worktree. Run the **whole** project's test suite and
   static analysis (not just the harness). Check the project builds the way it ships.
2. **Real targets**: the fake cannot imitate TLS, HTTP/2, CDN habits, real file systems. Run original
   and candidate side by side on public, harmless targets and compare outputs byte for byte. Send
   no personal data.
3. **Observability before the smoke test**: make sure failures can be told apart afterwards
   (lifecycle and error logs). In the app pilot a window vanished during the owner's smoke test and
   nothing recorded why; the logging was added only afterwards.
4. **Owner smoke test** on the branch build, with the owner's real workflow. Read-only investigation
   of anything odd (process state, OS event logs, produced files, checksums/CRC) before guessing.
5. **Transition note** if a persisted format changed: what happens to old state, migration code
   vs. a procedure (e.g. "finish paused downloads before switching"). ⛔ Owner decides.
6. Sync the project's own docs in the same change (architecture notes, module maps).
7. Commit on the branch following the project's commit conventions (one kind of change per commit;
   causes and measurements in the message). ⛔ Owner approves merging. Never push unasked.
8. Write the round summary and lessons into the recast folder.
