# Segment transcription prompt: coding videos

One Sonnet agent per `work/<id>/segments/segN_frames.txt`, all in parallel. Send this file as the agent's instructions, plus a launch message with: VIDEO (title, length, one-paragraph context: language, what is built, editor, overlays to ignore), ID, N, T, OUT (`sources/<channel>/<folder>/transcripts/segN.md`). For the last segment add "(final segment: the finished program and its output are most likely shown here)".

---

You are doing forensic transcription of a screen-recorded coding video. The audio track is not used; work only from frames. Ignore the overlays named in the launch message (e.g. a webcam window).

Keyframes: <ROOT>\work\<ID>\keyframes\
Your frame list (time ordered; k_03m52s.jpg = 3:52): <ROOT>\work\<ID>\segments\seg<N>_frames.txt

Read the list, then view EVERY frame in it in order with the Read tool (parallel batches of ~5 are fine; do not skip frames). Exact characters matter: syntax, format strings, escapes, indentation width, editor line numbers.

Write your result to OUT with this structure:
1. `## Timeline`: `[mm:ss] <what's on screen / what changed>`: which file/tab, lines added/edited/deleted (quote them), editor mode if useful, shell commands and their output verbatim (compiler errors/warnings too), docs/man pages/web pages read (which section), on-screen notes. Note refactors, renames, deletions, mistakes and fixes. **Quote editor save messages verbatim (e.g. vim `"main.c" 58L, 1119B written`); they are used as checksums.** Note window/tab titles (they can reveal real commands behind aliases).
2. `## File states`: for every file seen, the most complete verbatim content as of the END of your segment, in fenced code blocks WITH editor line numbers prefixed (`12| ...`). Mark gaps `?? (lines 14-20 not visible)` and uncertain characters `⟨?⟩`. Include an intermediate version (with timestamp) if a file changed a lot mid-segment.
3. `## Shell & outputs`: every terminal command and output, verbatim, with timestamps (browser-rendered output of a web page counts: describe what the page looks like and quote its visible text).
4. `## Observations`: design-intent signals: notes, build order, coding style, references consulted, what the author argues on screen, anything showing their philosophy.

**Privacy (mandatory):** never write e-mail addresses, phone numbers, login/verification codes, account or user names, or the names of private chats/groups and their messages. Write `<mail address>`, `<phone>`, `<code>`, `<account>`, `<private chat>` instead. Public product/site/company names may be written. Describe people in photos neutrally; do not identify them.

Do not guess content you can't see; mark it. Do not analyze anything outside your frame list. Your final reply should just be a 3-line summary and confirm the file was written.
