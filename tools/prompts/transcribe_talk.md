# Segment transcription prompt: talks, lectures, slide decks, screen essays

One Sonnet agent per `work/<id>/segments/segN_frames.txt`, all in parallel. Send this file as the agent's instructions, plus a launch message with: VIDEO (title, length, one-paragraph context: speaker or channel, what kind of screen, overlays to ignore, chapter titles if any), ID, N, T, OUT (`sources/<channel>/<folder>/transcripts/segN.md`).

---

You are doing forensic transcription of a video whose message is carried by TEXT ON SCREEN: slides, burned-in captions, sticky notes, documents, browser pages, terminal. The audio track is not used; work only from frames. Ignore the overlays named in the launch message (e.g. a webcam or a speaker camera) except to note when they are the only thing that changes.

Keyframes: <ROOT>\work\<ID>\keyframes\
Your frame list (time ordered; k_03m52s.jpg = 3:52): <ROOT>\work\<ID>\segments\seg<N>_frames.txt

Read the list, then view EVERY frame in it in order with the Read tool (parallel batches of ~5 are fine). Consecutive frames often differ only by a caption line or one build step of a slide; still look at each and keep the LAST complete version of every text.

Write your result to OUT with this structure:
1. `## Timeline`: `[mm:ss] <what's on screen / what changed>`: slide changes, build steps, captions, demos, scene changes. Quote new text verbatim as it appears. Burned-in captions: quote every distinct caption line in order (they are the closest thing to the speech).
2. `## Texts on screen`: the final, complete verbatim text of every slide, diagram, code sample, screenshot of a document/tweet/chart, or note seen in your segment, each in its own fenced block with a heading saying what it is and the time range it was visible. Keep original language and spelling (do not correct typos); if text is not English, transcribe it and add a translation line below marked `(translation)`. Mark unreadable parts `⟨?⟩` and hidden parts `?? (…)`. For tables and charts give the grid and every number shown, with axis labels and units.
3. `## Code & shell`: any code or terminal content, verbatim with line numbers if shown (omit the section if none).
4. `## Observations`: what the speaker is arguing or teaching in this segment, in plain words; which claims rest on a number, a screenshot, or an example shown on screen, and which are bare assertions; anything the video maker added (their own captions, highlights, commentary) as opposed to the original material.

**Privacy (mandatory):** never write e-mail addresses, phone numbers, login/verification codes, or the names of private chats/groups and their messages. Write `<mail address>`, `<phone>`, `<code>`, `<private chat>` instead. Public people speaking in their professional role (a conference speaker, the author of a public post shown on a slide) and public product/site/company names may be written. Describe other people in photos neutrally; do not identify them.

Do not guess content you can't see; mark it. Do not analyze anything outside your frame list. Your final reply should just be a 3-line summary and confirm the file was written.
