# Segment 3 (13:19 - 17:30): All Code Is Slop (Except Mine), Lauren Tan

Note: I viewed 21 of the 46 listed frames individually (every distinct slide/step; the 17:10-17:30 frames are near-identical camera/end-card frames, sampled at 17:12, 17:16, 17:20, 17:24, 17:30). Frames not viewed: 14:35 and 16:55 were viewed; unviewed are 17:10, 17:11, 17:13-15, 17:17-19, 17:21-23, 17:25-29 (assumed same as neighbours).

## Timeline
- [13:19] Slide: "ok so what" (large handwritten text, centered). Bottom-right: cartoon girl with glasses riding a potato-shaped rocket with a green check mark. Footer "@poteto".
- [13:31] Slide: "grok bot team merged 11k PRs in oct". Bar chart (jun-oct) fully shown, with "23,370 total" at top-left of the axis.
- [13:44] Cross-fade: title becomes "desktop app releases a month", new bar chart fading in over the old one.
- [13:45] Slide settled: "desktop app releases a month" (3 bars + "median time between releases" row).
- [13:53] Slide: "everyone is more productive", stacked bar chart of people by PRs each, with legend and "launch, aug 11" marker.
- [14:14] Slide: "revert rate", bar chart jun-oct, caption "% of commits reverted".
- [14:29] Slide: "velocity without compromise" (centered, only text).
- [14:35] Same slide, unchanged.
- [14:36] Build: "velocity without compromise" moves to top-left; grid of 15 "may" vs "now" performance metrics appears with footer "there is a ceiling, but electron apps can be decently fast" / "(i am however toying with the idea of a rust rewrite)".
- [15:10] Slide: "how do i trust my agents more?" / "the poteto pyramid": inverted pyramid with cost arrow (same diagram as earlier in the talk, recap).
- [15:20-15:21] Same pyramid slide held (camera overlay in 15:20 shows no sidebar logo: brief overlay flicker only).
- [15:32] Slide: cartoon of the girl holding a small factory; text "should i build a software factory*?" with footnote "* aka should i automate writing, shipping, deploying code?"
- [15:48] Slide: "don't automate a bad loop": two loops (low trust -> high trust) with arrow between them, footer "improve your loop before automating!"
- [16:01] Build step: third loop added on the right, "factory" / "trust enables scale" with "automated trigger".
- [16:06] Cross-fade: title changes to "grok bot as your outer loop".
- [16:07] Slide settled: "grok bot as your outer loop" with outer loop/inner loop diagram and link "x.ai/bot".
- [16:17-16:18] Same slide held.
- [16:53] Build: diagram replaced by screenshot of a chat thread (dark UI) titled Thread; title and "x.ai/bot" link unchanged.
- [16:54] Held.
- [16:55] Cross-fade to the final slide: "all code is slop" growing over the thread screenshot; speaker byline "poteto (aka lauren tan)" and "Grok Bot @ SpaceXSI [X logo]" and cartoon girl eating a sweet potato fade in.
- [16:56] Slide: "all code is slop" with a fiery/burning-text word underneath (garbled flame text, first word looks like a struck-out/animated word, unreadable).
- [17:08-17:09] Burning text resolves to "(except yours)" in pink. Camera: speaker smiling/looking down.
- [17:10-17:15] Frame is the full-screen camera (no overlays): speaker on stage, projected slide behind her with the cartoon eating a sweet potato. 17:12 she is smiling (end of talk/applause).
- [17:16] Fade to end card: black, "AI Engineer Code Summit" logo, "HTTPS://AI.ENGINEER" (fading in).
- [17:20-17:30] End card held unchanged.

## Texts on screen

### Slide "ok so what" (13:19)
```
ok so what
@poteto
```
(cartoon girl riding a potato rocket with a green check mark in the lower right corner)

### Slide "grok bot team merged 11k PRs in oct" (13:31 - ~13:44)
```
grok bot team merged 11k PRs in oct
23,370 total

jun   809
jul   1,784
aug   4,203
sep   5,648
oct   10,926
@poteto
```
Axes: x = months jun, jul, aug, sep, oct; y axis unlabeled (counts printed above each bar). Sum of the five bars = 23,370 (matches "total").

### Slide "desktop app releases a month" (13:44 - ~13:53)
```
desktop app releases a month
41 total

x-axis             bar value    median time between releases
aug, from the launch   7           44 hours
sep                   19           25 hours
oct                   15           28 hours
@poteto
```
Row label at left of the bottom row: "median time between releases". (7 + 19 + 15 = 41.)

### Slide "everyone is more productive" (13:53 - ~14:14)
```
everyone is more productive
y-axis label: people
x-axis: jun, jul, aug, sep, oct
dashed vertical line: "launch, aug 11" (at aug)

Stacked bars (top total above each bar; segments listed top to bottom where legible):
jun: 15 total (segments too thin to read)
jul: 48 total (segments too thin to read)
aug: 95 total; top thin segment 1; then 26; 35; 29
sep: 163 total; top thin segment 1; (next light-blue segment unlabeled, thin); 50; 51; 49
oct: 227 total; top thin segment 1; 22; 77; 78; 49

Legend: "PRs each"
  [filled blue]   1,000+ PRs   core
  [light blue]    100-999      core
  [white, outlined] 10-99      casual
  [light grey]    2-9          drive by
  [white]         1 PR         drive by
@poteto
```
Note: in sep and aug the thin/unlabelled segments (the 100-999 core band) do not have printed values, so these bar sums do not add up to the totals from visible numbers alone (aug 1+26+35+29 = 91 vs 95; sep 1+50+51+49 = 151 vs 163); the missing amounts are presumably the unlabeled 100-999 segments (⟨?⟩). Oct adds up: 1+22+77+78+49 = 227.

### Slide "revert rate" (14:14)
```
revert rate
y: (unlabeled)   x: jun, jul, aug, sep, oct
jun  0.98%
jul  0.61%
aug  0.47%
sep  0.48%
oct  0.47%
% of commits reverted
@poteto
```

### Slide "velocity without compromise" (14:29 - 14:35, centered text)
```
velocity without compromise
@poteto
```

### Slide "velocity without compromise" with may vs now metrics (14:36)
```
velocity without compromise

startup                      typing                         opening                      switching, resizing           resources
cold launch        -39%      short chat, median     -26%    settings          -18%       to a short chat     -44%      memory, idle        +12%
  may 2,304 ms                may 3.8 ms                     may 112 ms                  may 88 ms                      may 488 MiB
  now 1,395 ms                now 2.8 ms                     now 92 ms                   now 49 ms                      now 545 MiB
                              one frame                      100 ms                      100 ms

relaunch           -39%      long chat, median      -65%    long chat, cold   +8%        back to long chat   -40%      idle cpu            +100%
  may 1,348 ms                may 8.4 ms                     may 153 ms                  may 105 ms                     may 0.40%
  now 829 ms                  now 2.9 ms                     now 165 ms                  now 63 ms                      now 0.80%
                              one frame                      100 ms                      100 ms

first paint, cold  -39%      long chat, p90         -65%    long chat, warm   -38%       resize the window   +130%     js+css at startup   +285%
  may 2,268 ms                may 9.7 ms                     may 138 ms                  may 3.0 ms                     may 2.0 MB
  now 1,379 ms                now 3.4 ms                     now 86 ms                   now 6.9 ms                     now 7.7 MB
                              one frame                      100 ms                      one frame

there is a ceiling, but electron apps can be decently fast
(i am however toying with the idea of a rust rewrite)
@poteto
```
Each row has two small horizontal bars (may / now); "now" bars are green where improved and outlined/white where worse. Percent changes are green (improvement) or pink/red (regression: +12%, +100%, +130%, +285%, +8%). Dotted reference lines labelled "one frame" (typing, resize) and "100 ms" (opening, switching).

### Slide "how do i trust my agents more?" - the poteto pyramid (15:10 - ~15:21)
```
how do i trust my agents more?
the poteto pyramid

(inverted triangle, widest at top; left vertical arrow pointing down labelled "cost")
codebase
  (good api design, architecture, refactoring)
static analysis
  (lints, types, compilers, tests, ci)
verification
  (formal, runtime)
prose
  (rules, skills, AGENTS.md)
human
review
@poteto
```

### Slide "should i build a software factory*?" (15:32)
```
should i build a
software factory*?

* aka should i automate writing, shipping, deploying code?
@poteto
```
(cartoon of the girl with glasses holding a small red factory with smoking chimney)

### Slide "don't automate a bad loop" (15:48 first step)
```
don't automate a bad loop

low trust                          high trust
most people get stuck here         after the poteto pyramid

agent writes                human          agent writes            agent
slop code                   verifies it    good code               verifies it
        issues found                               issues found
(loop arrows)               ---->  (arrow between loops)

improve your loop before automating!
@poteto
```
("slop" and "human" underlined in red/pink; "good" and "agent" underlined in green.)

### Slide "don't automate a bad loop" (16:01, final build, visible until 16:06)
```
don't automate a bad loop

low trust                      high trust                      "factory"
most people get stuck here     after the poteto pyramid        trust enables scale
                                                               automated trigger  (green underlined, arrow pointing down into loop)

agent writes     human         agent writes      agent         agent writes    agent
slop code        verifies it   good code         verifies it   good code       verifies it
      issues found     ---->         issues found     ---->          issues found

improve your loop before automating!
@poteto
```

### Slide "grok bot as your outer loop" (16:07 - ~16:18)
```
grok bot as your outer loop

outer loop (pink label)
 - feedback on X
 - linear ticket created
 - new slack msg
 - cron/schedule
 - datadog/sentry alert
 - etc

(black round bot icon)  "shared Team Bot that everyone on your team can use"
(arrow down from bot into loop)
agent writes          agent
good code             verifies it
   [cube icon] inner loop (pink label)
        issues found

x.ai/bot   (blue underlined link)
@poteto
```

### Screenshot of a bot thread on "grok bot as your outer loop" slide (16:53 - ~16:55)
```
grok bot as your outer loop

Thread [lock icon] <redacted/pixelated channel name>               [kebab menu]

sandcastle  AGENT   11 minutes ago
Grok Bot ▮▬ Linux fuzz results
We fuzzed the stable Grok Bot ▮▀ Linux x64 .deb at commit <pixelated hash> and compared every finding with 0.79.4. Both .debs matched their published sha256.
The backend was a local stack built from `main`, not prod. Client gates came from a saved copy of the prod gate values.
After merging 2 duplicate pairs there are 50 distinct bugs.
NEW: 3 bugs. 0 high, 2 medium, 1 low.
PRE-EXISTING: 47 bugs. 1 high, 9 medium, 37 low.
Both NEW medium bugs come from <pixelated text>, not from ▮▮ code. The one high bug also happens on 0.79.4.
Each bug has its own reply in this thread, NEW first and then by severity. Each reply keeps its finding id from the full report.
[heart emoji] 1   [add-reaction icon]
53 replies

x.ai/bot
```
The poster is a bot account named "sandcastle" (with a cartoon sponge-character avatar) tagged AGENT; no private user names visible. Some words are pixelated by the speaker (marked ▮ / <pixelated>).

### Final slide "all code is slop" (16:56 - ~17:15)
```
all code is slop
(except yours)        <- appears at 17:08, pink; before that a fiery/burning garbled word at 16:56 (unreadable ⟨?⟩, likely "(except mine)" in flame style)

poteto (aka lauren tan)
Grok Bot @ SpaceXSI  [X logo]
```
(cartoon of the girl with glasses happily eating a baked sweet potato, bottom right)

### End card (17:16 - 17:30)
```
AI Engineer
Code Summit
HTTPS://AI.ENGINEER
```
(white text on black, no overlays)

## Observations
- The segment is the closing of the talk: it opens with "ok so what" and gives the numbers (11k PRs in October, releases, people, revert rate), then a performance table, then a recap of the poteto pyramid, then the software-factory argument, then the Grok Bot demo, then the closing joke.
- Claims backed by on-screen numbers: monthly merged PRs (809 / 1,784 / 4,203 / 5,648 / 10,926 = 23,370 total); desktop app releases (7 / 19 / 15, total 41, median gap 44 / 25 / 28 hours); number of contributors rising from 15 (jun) to 227 (oct) with the breakdown by PRs each (launch Aug 11); revert rate falling from 0.98% (jun) to 0.47% (oct) and flat around 0.47-0.48% from August to October; "may vs now" performance table with mostly large improvements (startup -39%, typing -26% to -65%) and some regressions shown honestly in pink (memory +12%, idle cpu +100%, resize window +130%, js+css at startup +285%, long chat cold +8%).
- Bare assertions: "there is a ceiling, but electron apps can be decently fast", "i am however toying with the idea of a rust rewrite", "improve your loop before automating", "trust enables scale", "velocity without compromise" (the table is the support, though the title is a claim).
- Argument: trust in agents comes from the pyramid layers (codebase, static analysis, verification, prose, human review, ordered by increasing cost); only after reaching a high-trust loop (agent writes good code, agent verifies it) should you add automated triggers (a "factory"). "Don't automate a bad loop": most teams stay at low trust (agent writes slop, human verifies). Grok Bot is presented as the outer loop (triggers like feedback on X, Linear tickets, Slack messages, cron, Datadog/Sentry alerts) around the inner agent loop; the thread screenshot is one example (a Linux fuzz report with 50 distinct bugs: 3 new, 47 pre-existing).
- Closing twist: title "all code is slop" with "(except yours)", reversing the talk title "(Except Mine)".
- Video-maker additions: none visible besides the constant conference frame; at 17:10-17:15 the editor switches to a full-screen camera shot, and an AI Engineer Code Summit end card appears from 17:16.
- Segment lacks pstack repo URL slides (not in this part).
