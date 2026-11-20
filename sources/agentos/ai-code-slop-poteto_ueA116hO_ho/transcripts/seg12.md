# Segment 12 (19:58 - 21:38)

## Timeline

Original-talk frames have the conference sidebar (title card "AI Engineer Code Summit 2026 / Lauren Tan (SpaceXSI) · 2026-11-11", "Google DeepMind" sponsor, speaker camera) and the lower-third "All Code Is Slop (Except Mine) / Lauren Tan / Principal Engineer / SPACEX". Caption lines: EN = English caption, KO = Korean caption (quoted only when it adds something).

- [19:58] Slide "how do i trust my agents more?" (poteto pyramid, final form). EN: "Start with a lint rule to prevent new occurrences of anti-patterns." (KO: 새 안티패턴이 생기지 않게 린트 규칙부터 만들고,)
- [20:02] Same slide. EN: "And then clean up those old patterns as well."
- [20:05] Same slide. EN: "And then make sure that your verification story is really strong."
- [20:08] Transition: the pyramid slide cross-fades into a new slide with a hand-drawn girl (glasses, brown hair) holding a small red factory; text "should i build a software factory*?". Caption unchanged.
- [20:09] Full new slide: "should i build a software factory*?" with footnote "* aka should i automate writing, shipping, deploying code?"
- [20:10] EN: "A lot of this work is a prerequisite to automation anyway."
- [20:13] EN: "And if your inner coding loop produces slop, then automating that loop just produces more slop."
- [20:20] EN: "And you need to throw more and more and more tokens at the problem, which we all know is very expensive." (KO: 그러면 토큰을 계속 더 쏟아부어야 하는데, 다들 알다시피 아주 비싸죠.)
- [20:25] Slide changes to "don't automate a bad loop" with two loops: "low trust" and "high trust" (see Texts). Same caption still showing.
- [20:26] EN: "And once you meet these investments into your codebase, static analysis, runtime and formal verification,"
- [20:33] EN: "you'll start to find that many things become possible with these foundations,"
- [20:37] Slide gets a third panel ("factory" / "trust enables scale" / "automated trigger") appearing from the right; the left two loops shift left. (Mid-animation frame at 20:33 shows the arrow moving.)
- [20:38] EN: "and that you can really easily build your own factory on top of these foundations."
- [20:41] Same, 3-panel slide fully built.
- [20:42] CHANNEL INSERT begins: full-screen dark purple card with white line illustration: a hand pump on a brick well, two bystanders on the left, a bucket under the spout overflowing with foam/suds, one person pushing the pump handle on the right; an orange electric motor is attached to the handle. No text yet, no caption.
- [20:43] KO caption only: "코드를 쓰는 안쪽 루프가"
- [20:44] KO caption: "슬롭을 내고 있으면,"
- [20:45] KO caption: "그 루프를 자동화해도"
- [20:46] KO caption: "슬롭만 더 많이 나온다는 거죠." Orange label appears right of the illustration: "안쪽 루프가 슬롭 → 자동화해도 슬롭만 더"
- [20:47] Card cross-fades to a dimmed re-show of the "don't automate a bad loop" slide (two-panel version) in a rounded frame, small top-left tag "발표 슬라이드 15:55 · AI Engineer Code Summit 2026". KO caption still "슬롭만 더 많이 나온다는 거죠."
- [20:48] Slide re-show full brightness. KO: "그래서 새로 생기는 안티패턴을"
- [20:49] Orange pill under slide: "① 새 안티패턴 → 린트로 먼저". KO: "린트로 먼저 막고,"
- [20:50] Slow zoom-in on slide (title shifts); KO: "이미 있는 건 그다음에" (20:50 frame); pill 1 stays, grey pill added: "② 옛것은 그다음 청소".
- [20:51] KO: "치우라는 순서였어요."
- [20:52] Next slide re-show, tag changes to "발표 슬라이드 16:15 · AI Engineer Code Summit 2026": "grok bot as your outer loop" (see Texts). KO: "이 기초 위에 로렌 탄 팀이 올린 것이"
- [20:54] KO: "마지막 예시예요."
- [20:55] KO: "로렌 탄은 자기 팀 제품인"
- [20:56] KO: "Grok Bot 을"
- [20:57] Grey pill appears under slide: "발표자 팀 제품 · 바깥 루프 권유". KO: "이 바깥 루프로 쓰라고 권해요."
- [20:58] Orange highlight box on bullet "feedback on X". KO: "X 에 올라온 피드백이나"
- [20:59] Highlight moves to "linear ticket created". KO: "이슈 티켓, 슬랙 메시지,"
- [21:00] Highlight on "new slack msg" (KO still "이슈 티켓, 슬랙 메시지,")
- [21:01] Highlight on "datadog/sentry alert". KO: "모니터링 알림 같은 신호가 들어오면"
- [21:02] same highlight, same KO.
- [21:03] KO: "Grok Bot 이 에이전트를 띄워서"
- [21:04] KO: "일을 시작하는 그림이에요."
- [21:05] CHANNEL INSERT card: line illustration of a smartphone surrounded by six hands tapping/swiping its UI blocks, with a small orange crack at the top-right corner of the screen. KO: "마지막 예시에 나오는 퍼징은"
- [21:06] Orange pill: "퍼징 = 마구 던져서 깨지는 곳 찾기"
- [21:07] KO: "앱에 온갖 입력과 조작을"
- [21:08] same card, no caption.
- [21:09] KO: "깨지는 곳을 찾는 테스트예요."
- [21:10] Grey pill added: "슬랙 스레드 = 결과 보고". KO: "화면의 슬랙 스레드는"
- [21:11] KO: "그 결과 보고고요."
- [21:12] Third pill (orange): "릴리스마다". KO: "새 릴리스를 낼 때마다"
- [21:13] KO: "이런 일이 벌어진다고 해요."
- [21:14] Card still, no caption.
- [21:15] Back to ORIGINAL TALK: slide "grok bot as your outer loop" now showing a dark Slack-style thread screenshot (see Texts). EN: "And as a quick example, whenever we cut a new release now,"
- [21:19] EN: "Grok Bot will automatically spin up a new Cursor project for us that will fuzz the entire application, the entire build," (KO adds a gloss: 퍼징(무작위로 두드려 보기))
- [21:26] EN: "and it will find bugs by running the app and using it like a real user."
- [21:30] EN: "And then it's going to open fix PRs for each of those bugs that it finds, and it's going to auto-merge it,"
- [21:37] EN: "as crazy as that sounds."
- [21:38] EN: "And that's because our codebase is good enough that pull requests tend to just be good by default."

## Texts on screen

### Slide "how do i trust my agents more?" (the poteto pyramid) (19:58 - 20:08, original talk)
```
how do i trust my agents more?
the poteto pyramid
[inverted triangle, top to bottom; vertical arrow on left labelled "cost" pointing down]
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

### Slide "should i build a software factory*?" (20:08 - ~20:25)
Cross-fade at 20:08 shows pyramid slide fading behind it.
```
[drawing: girl with glasses holding a small red factory with smoking chimneys]
should i build a
software factory*?

* aka should i automate writing, shipping, deploying code?
@poteto
```

### Slide "don't automate a bad loop" (20:25 - 20:37, two panels; 20:38 - 20:41 three panels)
Two-panel version (20:25 - ~20:37):
```
don't automate a bad loop

low trust                          high trust
most people get stuck here         after the poteto pyramid

[loop 1]                           [loop 2]
agent writes                       agent writes
slop code  (slop in red, underl.)  good code (good in green, underl.)
human verifies it (human red, underl.)   agent verifies it (agent green, underl.)
issues found                       issues found
(circular arrows: writes -> verifies -> issues found -> writes)

[bold arrow between loop 1 and loop 2]

improve your loop before automating!
@poteto
```
Three-panel final version (20:38 - 20:41):
```
don't automate a bad loop

low trust                  high trust                  "factory"
most people get stuck here after the poteto pyramid    trust enables scale
                                                       automated trigger  (green, underlined; arrow pointing down into loop 3)
agent writes               agent writes                agent writes
slop code                  good code                   good code
human verifies it          agent verifies it           agent verifies it
issues found               issues found                issues found
          [arrow] ->                  [arrow] ->

improve your loop before automating!
@poteto
```
(In loop 1 "slop" and "human" are red/underlined; in loop 2 "good" and "agent" are green/underlined; in loop 3 none are coloured.)

### Channel insert card, hand pump (20:42 - 20:46)
Dark purple background. Line drawing: hand pump on brick well, two standing/crouching figures at left watching a bucket overflowing with suds, a figure on the right working the handle, an orange motor bolted to the handle.
```
안쪽 루프가 슬롭 → 자동화해도 슬롭만 더
```
(translation) "Inner loop is slop -> even automated, only more slop." (orange pill, appears at 20:46)

### Channel re-show of "don't automate a bad loop" (20:47 - 20:51)
Tag top-left: `발표 슬라이드 15:55 · AI Engineer Code Summit 2026` (translation: "Talk slide 15:55 ..."). Slide is the two-panel version (text as above). Pills added below:
```
① 새 안티패턴 → 린트로 먼저        (orange)
② 옛것은 그다음 청소               (grey)
```
(translation) "1. New anti-patterns -> block with lint first" / "2. Old ones: clean up afterwards"

### Slide "grok bot as your outer loop" (channel re-show tag 16:15, 20:52 - 21:04)
```
grok bot as your outer loop

outer loop
• feedback on X
• linear ticket created
• new slack msg
• cron/schedule
• datadog/sentry alert
• etc
   -> [black round bot icon]  shared Team Bot that
                              everyone on your
                              team can use
        (arrow down)
   agent writes            agent
   good code               verifies it
        [small cube icon]
        inner loop
          issues
          found
   (circular arrows)

x.ai/bot  (blue, underlined link)
@poteto
```
Channel highlights during this re-show: orange box around "feedback on X" (20:58), "linear ticket created" (20:59), "new slack msg" (21:00), "datadog/sentry alert" (21:01-21:04). Grey pill under slide from 20:57: `발표자 팀 제품 · 바깥 루프 권유` (translation: "Speaker's team's product · recommends this outer loop").

### Channel insert card, phone + hands (21:05 - 21:14)
Line drawing: smartphone with UI blocks, six hands tapping/swiping from all sides, small orange crack at upper-right of the screen. Pills accumulate:
```
퍼징 = 마구 던져서 깨지는 곳 찾기     (orange, from 21:06)
슬랙 스레드 = 결과 보고               (grey, from 21:10)
릴리스마다                            (orange, from 21:12)
```
(translation) "Fuzzing = throw things at it at random and find where it breaks" / "Slack thread = results report" / "Every release"

### Slide "grok bot as your outer loop", Slack thread screenshot (21:15 - 21:38+, original talk)
Dark Slack-like thread inside the white slide. Some words are pixelated in the video (marked ▮ / ⟨?⟩).
```
grok bot as your outer loop

Thread 🔒 ⟨pixelated channel name⟩                               ⋮
[SpongeBob avatar] sandcastle  AGENT   11 minutes ago
Grok Bot ⟨▮▬⟩ Linux fuzz results
We fuzzed the stable Grok Bot ⟨▮▀⟩ Linux x64 .deb at commit ⟨pixelated commit hash⟩ and compared every finding with 0.79.4. Both
.debs matched their published sha256.
The backend was a local stack built from `main`, not prod. Client gates came from a saved copy of the prod gate
values.
After merging 2 duplicate pairs there are 50 distinct bugs.
NEW: 3 bugs. 0 high, 2 medium, 1 low.
PRE-EXISTING: 47 bugs. 1 high, 9 medium, 37 low.
Both NEW medium bugs come from ⟨pixelated text⟩, not from ⟨▮▮⟩ code. The one high bug also
happens on 0.79.4.
Each bug has its own reply in this thread, NEW first and then by severity. Each reply keeps its finding id from the full
report.
❤ 1   [add-reaction icon]
53 replies

x.ai/bot
@poteto
```
(The speaker camera in 21:15-21:38 shows the same Slack text on the stage screen behind her; ignored.)

## Observations

What Lauren Tan argues in this segment (from EN captions):
- Closing recommendations of the pyramid: add a lint rule so no new instances of an anti-pattern appear, then clean up old instances, and make verification strong.
- Then asks "should i build a software factory" (automate writing, shipping, deploying code). Her answer: much of the pyramid work is a prerequisite to automation anyway; if the inner coding loop produces slop, automating it only produces more slop and burns more tokens ("very expensive"). Slide "don't automate a bad loop" shows three stages: low trust (agent writes slop, human verifies) -> high trust (agent writes good code, agent verifies) -> "factory" (same loop with an automated trigger; "trust enables scale").
- Concrete example: the team's Grok Bot as the outer loop (triggers: X feedback, Linear ticket, Slack message, cron, Datadog/Sentry alert) driving an inner write/verify loop. On every release, Grok Bot spins up a Cursor project that fuzzes the whole app/build by using it like a real user, then opens fix PRs per bug and auto-merges them "as crazy as that sounds", justified by "our codebase is good enough that pull requests tend to just be good by default".

Evidence vs assertion:
- Backed by an on-screen artifact: the Slack thread screenshot (a Linux x64 .deb fuzz report: 50 distinct bugs; NEW 3 = 0 high / 2 medium / 1 low; PRE-EXISTING 47 = 1 high / 9 medium / 37 low; 53 replies). Note the screenshot shows the report and one bug per reply but not the fix PRs or auto-merge outcomes; those claims (auto-opened and auto-merged fix PRs, PRs "good by default") are spoken assertions, not shown here. Pixelated parts hide specifics.
- The claims "slop in -> more slop out", "tokens are expensive", and "a lot of this work is a prerequisite to automation" are bare assertions (the diagram is conceptual, no numbers).
- The three-stage loop diagram is conceptual; no data.

Channel (AgentOS) additions:
- Hand-pump/motor illustration (20:42-20:46) with orange label "안쪽 루프가 슬롭 → 자동화해도 슬롭만 더": a metaphor for automating a bad loop (a motor on a pump that gushes suds).
- "다시보기" style re-show of slides with tag "발표 슬라이드 15:55" and "16:15" (timestamps of the original talk), with their own summary pills: "① 새 안티패턴 → 린트로 먼저", "② 옛것은 그다음 청소" (the channel turns the spoken lint-then-clean sequence into a numbered order), and "발표자 팀 제품 · 바깥 루프 권유" (flags that this is the speaker's own team's product and she is recommending it, i.e. a self-promotion disclosure).
- Highlight boxes cycling through the outer-loop trigger bullets, timed to the KO narration.
- Phone-and-hands illustration with glossary pills: "퍼징 = 마구 던져서 깨지는 곳 찾기", "슬랙 스레드 = 결과 보고", "릴리스마다". The KO caption also adds a gloss at 21:19, "퍼징(무작위로 두드려 보기)", that is not in the English line.
- Channel's Korean narration frames the Grok Bot slide as "마지막 예시" and explicitly says the speaker "권해요" (recommends) it, which is the channel's reading rather than a quote.
- The KO captions in the channel-insert stretches (20:43-20:46, 20:48-20:51, 20:52-21:04, 21:05-21:13) are the channel's own narration; no English captions run during them.
