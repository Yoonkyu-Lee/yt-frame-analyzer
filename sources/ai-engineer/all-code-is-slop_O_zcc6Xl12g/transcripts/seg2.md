# Segment 2 (4:57 to 13:19) - "All Code Is Slop (Except Mine)", Lauren Tan

Frames only; no audio. Constant overlays ignored (Code Summit sidebar, Google DeepMind, title bar "All Code Is Slop (Except Mine) / Lauren Tan / Principal Engineer / SpaceX", speaker camera). Every slide carries a "@poteto" watermark bottom-right (bottom-centre on slides with a corner mascot).

## Timeline
- [04:57] Cross-fade from the previous slide into "dune: a framework for agents" (previous slide text "yes, you should read the code sometimes" and a drawing of the girl and robot reading a code book with speech bubble "Walk me through this code." still visible).
- [04:58] Slide "dune: a framework for agents" complete: box "app framework / navigation, rendering, IPC" + three boxes "one folder per feature", "strict import bans", "no comments in code".
- [05:25] Fading in new slide: big text "nuance tends to lead to slop", pink subline "just design better APIs", mascot drawing bottom-right (girl looking unimpressed at a tangled control-panel machine with a robot). The previous Dune slide is still ghosted behind.
- [06:09] Slide "directory structure is a form of context": Dune diagram (electron-renderer/ and electron-main/) plus tibo.dog side project diagram. (Ghost of "nuance tends to lead to slop" still fading.)
- [06:10] Same slide, clean.
- [06:20], [06:21] Same slide (06:20 has a transient dark flicker of the sidebar).
- [06:27] Build step: the Dune diagram widens, a "shared/" box ("shared types") appears between the two folders with arrows from both sides, and a dotted double-arrow from features/ to electron-main/ is crossed out with an X labelled "not even types".
- [06:55] Transition: slide "stop agents from writing unsolicited markdown" (cartoon: robot says "I made it up", girl says "Source?", .md file icons); previous slide ghosted behind.
- [07:22] Slide "quick fixes considered harmful": timeline Sep 9, Oct 3, Oct 17.
- [08:19] Slide "comments are liabilities": bar chart, 65,824 (jul 31) vs 1,227 (today); footer "also see /no-comments in pstack: github.com/cursor/plugins".
- [09:09] Cross-fade to slide "whenever you correct your agent" (step chart overlaying the bar chart ghost).
- [09:10] Step chart clean: 7, 17, 38, 90, 139, 149, 157, 167 over jul 1 to nov 1; footer "lint against it or categorically eliminate the problem".
- [09:20], [09:21] Same slide (09:20 sidebar flicker).
- [09:39] Transition into slide "verification is the most important step" (ghost of the poteto pyramid slide "how do i trust my agents more?" behind it).
- [10:01] Transition: "runtime verification" slide appearing, with feature map diagram; ghost of "verification is the most important step" still visible.
- [10:37] Build step: "runtime verification" now has feature map + CLI box + purple box "agent checks its own work".
- [10:49] Transition to "formal verification" (TLA+ / Lean cartoon), runtime slide ghosted.
- [10:50] "formal verification" slide complete.
- [11:34] Transition to slide "ui property based testing" with link github.com/antithesishq/bombadil; ghost of formal verification slide. An embedded screen recording of a chat app ("dev smoke" bot) begins.
- [11:35 to 11:51] Screen recording plays inside the slide: Connect Apps / "Manage plugins and skills" dialog (11:35), chat with garbled random-text messages and a Settings panel (11:36, 11:37), "To: Start a chat with..." new-chat view with a draft (11:38), long chat with red error toast "Bot failed to respond" (11:39), Settings/routines side panel (11:40, 11:41), "Thinking it through" (11:42), more garbled chat (11:43), "Create New Bot" dialog (11:44), "Attach files / Teach a task" menu (11:45), new-chat view (11:46), "Connect Apps" dialog with 1Password listed (11:47), blank white frame (11:48, recording loop restart), chat again (11:49), command-palette search popup (11:50, 11:51). See Texts on screen.
- [12:04] New slide: screenshot of a tweet by Cindy Sridharan (@copyconstruct), dated May 15, 2026, with highlight and red oval.
- [12:20] Build: second tweet (Nov 5, 2026) from the same author quoting the first, highlighted text and red oval.
- [12:21] Same.
- [12:30] New slide: diagram of a testing taxonomy "PRE-PRODUCTION / TESTING IN PRODUCTION" (columns Deploy, Release, Post release) overlaid on the faded Cindy Sridharan tweet.
- [12:53] Same slide, columns fully filled.
- [12:54] New slide: dark statechart diagram (TLA+ bugs caught in the tibo.dog story mode) with overlay text "/statechart" and "github.com/cursor/plugins/tree/main/pstack".
- [13:18] Same slide held.
- [13:19] New slide: "ok so what" with mascot (girl riding a rocket shaped like a potato with a checkmark) bottom-right.

## Texts on screen

### Slide: dune: a framework for agents (04:57 to ~05:25)
```
dune: a framework for agents

[box, black outline]
app framework
navigation, rendering, IPC

+

[blue box]   one folder per feature
[green box]  strict import bans
[orange box] no comments in code

@poteto
```
Ghost text seen during the 04:57 cross-fade from the previous slide: "Walk me through this code." (speech bubble, "this code." in pink) and "yes, you should read the code sometimes".

### Slide: nuance tends to lead to slop (05:25 to ~06:00)
```
nuance tends to lead to slop
just design better APIs        (pink)
```
Mascot drawing bottom-right: girl with glasses, bored expression, robot on a control panel with tangled cables; "??" above the robot.

### Slide: directory structure is a form of context (06:09 to ~06:55)
Step 1 (06:09 to 06:21):
```
directory structure is a form of context

Dune
 [dashed box] electron-renderer/
     [blue]   features/
     [green]  client/
     [purple] primitives/
 [dashed box] electron-main/
     [pink]   startup/
     [pink]   notifications/
     [pink]   update/

tibo.dog (side project)
 [black box]  src/
                sim/
                  tick.hot.ts
      -> [blue box]  runs every tick
      -> [green box] no allocations
```
Step 2 (06:27, final):
```
directory structure is a form of context

Dune
 electron-renderer/                                  electron-main/
   features/  <.......... X .........>  startup/
              label under the X: "not even types"    notifications/
   client/   primitives/  --> [yellow box] shared/ <--  update/
                          label: "shared types"
 (solid arrows from electron-renderer/ to shared/ and from electron-main/ to shared/)

tibo.dog (side project)
 src/
   sim/
     tick.hot.ts   -> runs every tick
                   -> no allocations
```

### Slide: stop agents from writing unsolicited markdown (06:55)
```
stop agents from writing unsolicited markdown
```
Drawing: robot with speech bubble "I made it up"; girl holding a .md page with bubble "Source?"; floating ".md" file icons.

### Slide: quick fixes considered harmful (07:22)
```
quick fixes considered harmful

Sep 9                                   Oct 3                      Oct 17
[orange box]                            [stack of orange boxes]    [blue box]
// release later, in case it comes back   setTimeout(..., 0)       183 timers
setTimeout(() => release(id), 0)
fixes a flicker                         copied into a
agent: "it would be a larger change"    shared module
(arrows: Sep 9 -> Oct 3 -> Oct 17)
```
The code font renders "=>" as a ligature arrow in `() => release(id)`.

### Chart: comments are liabilities (08:19)
```
comments are liabilities

Bar chart (unlabelled y axis, arrowed axes)
  x: "jul 31"  -> red outlined bar, tall, label 65,824
  x: "today"   -> green flat bar, label 1,227

also see /no-comments in pstack: github.com/cursor/plugins
```

### Chart: whenever you correct your agent (09:09 to ~09:39)
```
whenever you correct your agent

Step line chart (green), y axis unlabelled, x axis ticks:
jul 1 | aug 1 | sep 1 | oct 1 | nov 1
Values on the steps, left to right:
7, 17, 38, 90, 139, 149, 157, 167
(7 between jul 1 and aug 1; 17 just after aug 1; 38 mid-aug; 90 around sep 1; 139 mid-sep; 149 around oct 1; 157 mid-oct; 167 around nov 1; line continues flat to the right. Positions are read from layout, not labelled.)

lint against it or categorically eliminate the problem
```

### Slide: how do i trust my agents more? (the poteto pyramid) (ghost only, 09:39)
Ghosted text visible while cross-fading out:
```
how do i trust my agents more?
the poteto pyramid
(inverted triangle, "cost" axis arrow on left pointing down)
  code base        (good api design, architecture, refactoring)   [partly legible]
  static analysis  (lints, types, compilers, tests, ci)
  verification     (formal, runtime)
  prose            (⟨?⟩)
  human review
```

### Slide: verification is the most important step (09:39)
```
verification
is the most
important step
```

### Slide: runtime verification (10:01 to 10:49)
Step 1 (10:01):
```
runtime verification

[blue box] feature map
   node graph: grey circle -- yellow circle -- two branches, each ending in a green circle
   labels: "what features exist"   "how users reach it"   "what proves it works"

also see /create-verification-skill in pstack: github.com/cursor/plugins
```
Step 2 (10:37, final):
```
runtime verification

[blue box] feature map  (same graph and three labels)
+
[orange box] CLI
   [black box]
   $ app aria-click --accessible-name "Settings"
   $ app snapshot
   $ app screenshot proof.png
   drive the real app

[purple box, wide] agent checks its own work

also see /create-verification-skill in pstack: github.com/cursor/plugins
```

### Slide: formal verification (10:49 to ~11:34)
```
formal verification

[speech bubble, blue "TLA+"]   Model a state machine. Check that its rules hold.
[speech bubble, red "Lean"]    Write proofs about code. Check every logical step.
(drawing: smiling girl with glasses holding a checked document, two small robots)

[blue box]  TLA+ model   | model state machines and check that all states are correctly handled
[red box]   Lean proof   | check implementation for bugs

also see /create-formal-verification-skill in pstack: github.com/cursor/plugins
```

### Slide: ui property based testing (11:34 to 12:04)
```
ui property based testing
github.com/antithesishq/bombadil
```
Embedded screen recording of a desktop chat app (Grok Bot dev build). Visible text (garbled random-string messages elided as `⟨garbled⟩`):
- Left sidebar: search and "+" icons, bot "dev smoke  DEV", bottom "TU" avatar and "Connect apps" button; user menu (11:36): "Weekly usage 0% used", "Get Grok Bot for mobile", "Support", "Settings", "Add account", "Log out".
- Chat messages (dark bubbles): "Create a public template of yourself that I can share with somebody else" (repeated), "Send me a message that says SMOKE-a74c50a4 and nothing else.", plus many `⟨garbled⟩` random-character messages (fuzzed input).
- Input placeholders: "Message dev smoke", "Message Bot"; new-chat header "To: Start a chat with..."; sidebar entry "Draft: ⟨garbled⟩".
- Error toast (11:39): "Bot failed to respond / Something went wrong while generating the reply. Try again."
- Status lines: "Thinking it through" (11:42), "Putting it together" ⟨partly legible⟩ (11:50).
- Settings side panel (11:36 to 11:46): "Settings", "Name" = "dev smoke", "Notifications / Get notified when this Bot finishes or is waiting for you" (toggle on); another panel (11:40): "dev smoke's screen", "Routines", "Ask in chat to set a routine".
- "Connect Apps" dialogs: (11:35) "Connect Apps / Manage plugins and skills / Installed / Nothing installed yet. Find plugins in the marketplace. / Private skills / No private skills yet. Ask your Bot to create one for you" (small, partly legible); (11:47) "Connect Apps  0 connected  >", search "Search across apps and skills", "Bot templates", tabs "All", "Password Managers", item "1Password - Let Grok Bot sign in to sites on its co... [Connect]".
- "Create New Bot" dialog (11:44): "Create New Bot", "Name your Bot", grid of avatar shapes, colour swatches, buttons "Cancel", "Create".
- Attach menu (11:45): "Attach files", "Teach a task".
- Command palette (11:50, 11:51): "Search", "dev smoke", "New Bot", "New group chat", "Keyboard shortcuts / Help", "DEV Open Component Gallery / Views", "DEV Open Widget Gallery / Views", "DEV Open Grok Bot Playground / Views", "DEV buy-credits:saved-card / Developer".

### Screenshot: tweet 1 (12:04)
```
Cindy Sridharan
@copyconstruct

Might be anecdotal, but just this week multiple AI generated PRs with subtle bugs got merged that required several additional days and a lot of manual verification to fix.

"Velocity" isn't going up when you consider how much effort was spent fixing...

May 15, 2026
```
Yellow highlight with red oval: "multiple AI generated PRs with subtle bugs got merged that required several additional days and a lot of manual verification to fix."

### Screenshot: tweet 2 quoting tweet 1 (12:20 to 12:30)
```
Cindy Sridharan
@copyconstruct

Revisiting my tweets from 6m ago is funny, because I don't run into any of these problems now at all.

What was missing in my setup 6 months ago was the verification process I now use.

70% of my daily token spend goes into verifying patches, and only ~20-30% into creating them.

[quoted tweet box]
Cindy Sridharan @copyconstruct · May 15
Might be anecdotal, but just this week multiple AI generated PRs with subtle bugs got merged that required several additional days and a lot of manual verification to fix.

"Velocity" isn't going up when you consider how much effort was spent fixing...

Nov 5, 2026
```
Yellow highlights on "What was missing in my setup 6 months ago was the verification process I now use." and "70% of my daily token spend goes into verifying patches, and only ~20-30% into creating them."; red oval around both.

### Diagram: testing taxonomy (12:30 to 12:53)
Behind it, faded: the Cindy Sridharan post (partial text "A mix ... in sho... This h... decad... finally ... come t... But tu... necess... genera... anymo...", "Nov 5,").
```
PRE-PRODUCTION  (arrow spanning the first column)
TESTING IN PRODUCTION  (arrow spanning Deploy, Release, Post release)
   DEPLOY | RELEASE | POST RELEASE

[purple column: pre-production]
- UNIT TESTS
- FUNCTIONAL TESTS
- COMPONENT TESTS
- FUZZ TESTS
- PROPERTY BASED TESTS
- FORMAL VERIFICATION
- STATIC ANALYSIS
- MUTATION TESTS
- COVERAGE TESTS
- BENCHMARK TESTS
- CONTRACT TESTS
- LINT TESTS
- ACCEPTANCE TESTS
- REGRESSION TESTS
- SMOKE TESTS
- USABILITY TESTS
- UI/UX TESTS
- PENETRATION TESTS
- THREAT MODELING
- PERFORMANCE TESTS

[grey column: deploy]
- INTEGRATION TESTS
- TAP COMPARE
- LOAD TESTS
- SHADOWING
- CONFIG TESTS
- SOAK TESTS

[orange column: release]
- CANARYING
- MONITORING
- TRAFFIC SHAPING
- FEATURE FLAGGING
- EXCEPTION TRACKING

[blue column: post release]
- TEEING
- PROFILING
- LOGS/EVENTS
- CHAOS TESTING
- MONITORING
- A/B TESTS
- TRACING
- DYNAMIC EXPLORATION
- REAL USER MONITORING
- AUDITING
- ONCALL EXPERIENCE
```
(At 12:30 the middle columns are still fading in with a lighter fill; 12:53 shows final fills.)

### Slide: /statechart (12:54 to 13:19)
```
/statechart
github.com/cursor/plugins/tree/main/pstack
```
Background: dark statechart diagram, title small and hard to read ("BUGS TLA+ CAUGHT IN TIBO.DOG STORY MODE" ⟨?⟩), red numbered markers 1 to 11 and a legend at the bottom (low resolution; issue numbers approximate):
```
1  A slow load froze the landing with no line on screen (#943)
2  Tibo got up behind the loading screen while WebGL was lost (#1080)
3  The gear chest gave Tibo a second Peeler, via a Grok Bot level-up before ⟨?⟩ (#1092 / #933)
4  The 8 s auto-pick cut poteto's level-up lesson (#1191)
5  The auto-pick picked while you looked away, and the run played on (#1191)
6  After a reroll, the auto-pick pointed at a card from the old hand (#1191)
7  amazing's entrance ran on behind the loading screen (#1013)
8  After her lines failed to load, the next run froze at her book (#988)
9  The pause menu opened over poteto's finale line (#988)
10 Her evil laugh kept shaking the screen under the focus pause (#1094)
11 After Reset run?, her half-typed line stayed on the splash (#929)
```

### Slide: ok so what (13:19)
```
ok so what
```
Drawing: girl on a potato-shaped rocket with a green checkmark, bottom-right.

## Code & shell
Only fragments inside slides, all verbatim above:
```
// release later, in case it comes back
setTimeout(() => release(id), 0)

setTimeout(..., 0)

$ app aria-click --accessible-name "Settings"
$ app snapshot
$ app screenshot proof.png

src/
  sim/
    tick.hot.ts
```

## Observations
- 5:00 to 6:55 (Dune): the speaker argues for hard structural rules for agents. Dune is an app framework (navigation, rendering, IPC) plus three rules: one folder per feature, strict import bans, no comments in code. "Nuance tends to lead to slop; just design better APIs." Directory structure is treated as context for the agent: electron-renderer/ (features/, client/, primitives/) and electron-main/ (startup/, notifications/, update/) may not import each other, "not even types"; only a shared/ folder of shared types is allowed. A second example (tibo.dog side project): a file name `tick.hot.ts` signals "runs every tick" and "no allocations", so naming conventions carry constraints. These are diagrams and bare assertions about design, not measured results.
- 6:55: "stop agents from writing unsolicited markdown" is a bare assertion with only a cartoon (agent invents a source).
- 7:22 "quick fixes considered harmful": a concrete dated example (Sep 9 a setTimeout fix with a comment, agent says "it would be a larger change"; Oct 3 copied into a shared module; Oct 17 "183 timers"). One anecdote.
- 8:19 "comments are liabilities": numbers 65,824 on jul 31 vs 1,227 today (units not stated on the slide; presumably comment counts).
- 9:09 "whenever you correct your agent": step chart 7, 17, 38, 90, 139, 149, 157, 167 from jul 1 to nov 1, takeaway "lint against it or categorically eliminate the problem". The y axis is unlabelled; it is presumably the number of lint rules, but the slide does not say so.
- 9:39 to 10:37: back to the pyramid; claim "verification is the most important step". Runtime verification: a feature map (what features exist / how users reach it / what proves it works) plus an `app` CLI (aria-click, snapshot, screenshot) so the agent drives the real app and checks its own work. Pointer to /create-verification-skill in pstack.
- 10:49: formal verification: TLA+ models (all states handled) and Lean proofs (check implementation for bugs). Cartoon and short definitions, no results shown. Pointer to /create-formal-verification-skill.
- 11:34 to 11:51: "ui property based testing" with a link to bombadil (antithesishq). A screen recording of the Grok Bot desktop app driven by random actions and garbled inputs (fuzzing; error toast "Bot failed to respond"). A demonstration, not a number.
- 12:04 to 12:53: social proof from a Cindy Sridharan thread. May 15, 2026: AI-generated PRs with subtle bugs cost days of manual verification. Nov 5, 2026: after adding verification, "70% of my daily token spend goes into verifying patches, and only ~20-30% into creating them." (as written the figures do not sum to 100%). Then a taxonomy of test types (pre-production, deploy, release, post release) laid over the faded tweet, shown to argue for the breadth of verification techniques; its authorship is not stated on screen beyond the faded tweet behind it.
- 12:54: "/statechart" pstack skill with a large generated statechart for tibo.dog story mode and an 11-item list of bugs that TLA+ caught, each tagged with an issue/PR number. Concrete example, but illegible at this resolution.
- 13:19: transition to "ok so what" (start of the next section).
- Video-maker additions: none observed (no added captions). The red ovals and yellow highlights on the tweets are part of the speaker's slides. Brief dark flickers of the sidebar are stream artifacts.
