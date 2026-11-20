# Segment 6 (08:58 - 11:31), video ueA116hO_ho

Note: 08:58-09:17 are CHANNEL INSERTS (full-screen purple card, a re-show of an earlier slide, a setTimeout drawing card) with Korean-only narration captions (no English line). From 09:18 the original talk with English + Korean captions resumes.

## Timeline

[08:58] Channel card, dark purple. Two panels. Panel 1 "① 쓰기 까다로운 기능" with chip `useEffect`, red/orange box "제품 코드에서 금지". Panel 2 "② 섞이면 안 되는 코드" with chips "화면을 그리는 쪽" ~ "뒤에서 일하는 쪽", orange text "→ 앱이 버벅임", box "폴더 둘 + import 금지". Orange pill below: "공통 · 리뷰에서 빠짐". Korean caption: "두 경우 모두 리뷰에서"
[08:59] Same card. Caption: "다시 볼 일이 없어졌다고 해요."
[09:00] Same card, dimming (fade out). Same caption.
[09:01] Cut to re-show of the talk slide "quick fixes considered harmful" (white slide in a frame; small label top-left "발표 슬라이드 8:10 · AI Engineer Code Summit 2026"). Slide shows "Sep 9" code box, "fixes a flicker", agent quote, and two dashed placeholders "Oct 3 · ?" and "Oct 17 · ?". Caption: "다음 대목에는 화면에"
[09:02] Caption: "날짜 세 개가 차례로 나와요."
[09:03] Orange outline highlights "Sep 9"; orange pill under slide "9월 9일". Caption: "9월 9일, 10월 3일, 10월 17일."
[09:04] Second pill "10월 3일" (grey), third "10월 17일" (dimmer) added under slide. Same caption.
[09:05] Fourth pill "Cursor 에서 있었던 일". Caption: "Cursor 에서 있었던 일이라고 해요."
[09:06] Same.
[09:07] Cut to channel card: drawing of a stopwatch (orange) taped over with an X of tape on a cracked wall, tape strips below; orange pill "setTimeout = 나중에 실행". Caption: "여기 나오는 setTimeout 은"
[09:08] Caption: "이 코드를 조금 있다가 실행하라고"
[09:09] Same card, no caption.
[09:10] Card fades out. Caption: "미뤄 두는 타이머예요."
[09:11] Back to the slide re-show, zoomed in (title cut off at the top), "Sep 9" and "Oct 3 · ?" box visible; orange outline around "fixes a flicker". Caption: "9월 9일에는 화면 깜빡임을"
[09:12] Zoomed further. Caption: "고치려고 넣은 한 줄이었어요."
[09:14] Orange highlight band over the comment line "// release later, in case it comes back". Caption: "빨간 글씨가 붙어 있었는데,"
[09:15] Same, orange pill "10월 17일엔 · ?" under slide. Caption: "10월 17일에는"
[09:17] Same, pill remains, no caption.
[09:18] Cut to ORIGINAL TALK layout (sidebar "AI Engineer Code Summit", "PRESENTED BY Google DeepMind", speaker camera bottom-left, title bar "All Code Is Slop (Except Mine)", "Lauren Tan / Principal Engineer", SpaceX logo; top-left tag "AI Engineer Code Summit 2026 / Lauren Tan (SpaceXSI) · 2026-11-11"). Slide: "stop agents from writing unsolicited markdown" (cartoon of a robot saying "I made it up" and a person asking "Source?"). No caption yet.
[09:20] EN: "And then one more surprising thing that I've seen is agents using Markdown." KO: "그리고 또 하나 의외였던 건, 에이전트가 마크다운을 쓰는 방식이에요."
[09:24] EN: "So things like documentation, code comments, rules, or skills." KO: "문서, 코드 주석, 규칙, 스킬 같은 거요."
[09:28] EN: "And they use this as a place to write down their justifications for not doing the right thing." KO: "에이전트는 이걸 옳은 일을 안 한 데 대한 변명을 적는 곳으로 써요."
[09:34] EN: "Or I don't know if you've ever seen this, but they'll over-rotate on some small correction that you made," KO: "아니면, 보셨을지 모르겠지만, 여러분이 한 작은 교정에 과하게 반응해서"
[09:39] EN: "and they treat it like a hard rule that can never be broken ever again." KO: "절대 깨면 안 되는 철칙처럼 다루기도 하죠."
[09:43] Slide cross-fades to "quick fixes considered harmful" full timeline (same EN/KO caption still showing).
[09:44] Full slide visible. EN: "And here's another example from Cursor." KO: "Cursor에서 있었던 예를 하나 더 볼게요."
[09:47] EN: "So agents really loved using imperative timers," KO: "에이전트는 명령형 타이머를 정말 좋아했어요."
[09:50] EN: "so things like setTimeout, requestAnimationFrame, requestIdleCallback," KO: "setTimeout, requestAnimationFrame, requestIdleCallback 등이요."
[09:54] EN: "and they were using these as quick fixes because they were really patching over an architecture clash" KO: "이걸 땜질용으로 썼는데, 사실은 아키텍처 충돌을 덮고 있었던 거예요."
[10:02] EN: "between VS Code's imperative style and React's declarative style." KO: "VS Code의 명령형 스타일과 React의 선언형 스타일 사이의 충돌이요."
[10:06] EN: "So our choice of architecture actually made both the client and the agent performance much worse." KO: "결국 저희의 아키텍처 선택이 클라이언트 성능도, 에이전트 성능도 훨씬 나쁘게 만든 거죠."
[10:13] EN: "And if you've ever done any performance work in JavaScript, you'll know" KO: "JavaScript로 성능 작업을 해 보셨다면 아실 텐데요,"
[10:16] EN: "that when you have tons of timers being rapidly created and destroyed, it causes a lot of major garbage collection," KO: "타이머가 엄청 많이 빠르게 생겼다 없어졌다 하면 큰 가비지 컬렉션이 자주 일어나고,"
[10:22] EN: "which causes a lot of lag and jank." KO: "그게 렉과 버벅임을 많이 일으켜요."
[10:26] EN: "But the worst part is that agents wrote a lot of code comments, like you can see in red here," KO: "그런데 최악은, 에이전트가 여기 빨간 글씨처럼 코드 주석을 잔뜩 달았다는 거예요."
[10:30] EN: "explaining why this quick fix was justified." KO: "이 땜질이 왜 정당한지 설명하는 주석을요."
[10:34] EN: "And before you know it, we had timers spreading everywhere," KO: "그러다 어느새 타이머가 사방에 퍼졌고,"
[10:38] EN: "and the performance of the app really suffered." KO: "앱 성능이 정말 나빠졌어요."
[10:40] Same caption; slide starts to cross-fade to the next slide.
[10:41] Mid-transition: new title "comments are liabilities" overlapping the old slide; bars 65,824 / 1,227 fading in.
[10:42] Slide "comments are liabilities" (bar chart) fully shown. Same caption.
[10:49] EN: "Now, in Grok Bot, we learned our lesson there, and we decided to just ban 99% of comments" KO: "그래서 Grok Bot에선 교훈을 얻어, 주석의 99%를 그냥 금지하기로 했어요."
[10:51] EN: "because they go stale easily." KO: "금방 낡거든요."
[10:55] EN: "Humans just really don't bother reading the comment slop anyway." KO: "사람은 어차피 주석 슬롭을 굳이 읽지도 않고요."
[10:59] EN: "And again, it gives an agent an escape hatch" KO: "또, 주석은 에이전트에게 탈출구가 돼요."
[11:03] EN: "to justify not doing the right thing because it's hard." KO: "어렵다는 이유로 옳은 일을 안 하는 걸 정당화하는 탈출구요."
[11:07] EN: "And the worst part about comments, though, is that they're not enforceable." KO: "그리고 주석의 가장 나쁜 점은 강제할 수 없다는 거예요."
[11:13] EN: "So what seems well-intentioned, like, I'm going to tell the next agent not to do something," KO: "다음 에이전트한테 '이건 하지 마' 하고 알려 주는 게 좋은 의도 같아 보여도,"
[11:16] EN: "never ends up actually being anything that you can trust." KO: "결국 믿을 수 있는 건 되지 못해요."
[11:23] EN: "So in Grok Bot and in Dune, we now have lint rules that give" KO: "그래서 Grok Bot과 Dune에는 이제 린트 규칙이 있는데요,"
[11:29] EN: "good and very helpful error messages telling the agent to fix the root cause, the real problem," KO: "근본 원인, 진짜 문제를 고치라고 에이전트에게 알려 주는 아주 친절한 에러 메시지를 내요."
[11:31] EN: "rather than just patch around it." KO: "땜질만 하지 말고요." (Frame list ends at 11:31; the caption shown at 11:31 is this one, the slide is still "comments are liabilities". Note: a frame labelled 11:31 in my list shows slide "whenever you correct your agent" with the same caption; see Texts on screen.)

## Texts on screen

### Channel card, two things not to allow (08:58-09:00)
```
① 쓰기 까다로운 기능
   [useEffect]
   [제품 코드에서 금지]

② 섞이면 안 되는 코드
   [화면을 그리는 쪽] ~ [뒤에서 일하는 쪽]
   → 앱이 버벅임
   [폴더 둘 + import 금지]

[공통 · 리뷰에서 빠짐]
```
(translation) 1 Hard-to-use feature: useEffect, banned in product code. 2 Code that must not be mixed: the side that draws the screen ~ the side that works in the background -> app gets janky; two folders + ban on imports. Common: dropped out of review.

### Talk slide "quick fixes considered harmful" re-show with channel pills (09:01-09:17)
```
quick fixes considered harmful
Sep 9
  // release later, in case it comes back
  setTimeout(() => release(id), 0)
  fixes a flicker
  agent: "it would be a larger change"
[Oct 3 · ?]   [Oct 17 · ?]          @poteto
```
Channel label top-left: "발표 슬라이드 8:10 · AI Engineer Code Summit 2026" (translation: Talk slide 8:10).
Channel pills below (added in order): "9월 9일" / "10월 3일" / "10월 17일" / "Cursor 에서 있었던 일"; later "10월 17일엔 · ?" (translation: Sep 9 / Oct 3 / Oct 17 / Something that happened at Cursor; On Oct 17 · ?).
Highlights added by the channel: orange outline on "Sep 9" (09:03), orange outline on "fixes a flicker" (09:11), orange band on the red comment line (09:14).

### Channel card, setTimeout (09:07-09:10)
Drawing: orange stopwatch crossed with two strips of tape over a cracked line, three tape strips below.
```
[setTimeout = 나중에 실행]
```
(translation) setTimeout = run later. Narration: "여기 나오는 setTimeout 은 이 코드를 조금 있다가 실행하라고 미뤄 두는 타이머예요." (translation: The setTimeout here is a timer that postpones this code to run a little later.)

### Talk slide: stop agents from writing unsolicited markdown (09:18-09:43)
```
stop agents from writing unsolicited markdown
```
Cartoon: robot with speech bubble "I made it up", floating ".md" documents, woman with glasses holding a ".md" doc, speech bubble "Source?". Credit "@poteto".

### Talk slide: quick fixes considered harmful, full (09:43-10:40)
```
quick fixes considered harmful

Sep 9                               Oct 3                      Oct 17
// release later, in case it        [stack of three boxes,     [ 183 timers ]
comes back                           top one reads:
setTimeout(() => release(id), 0)     setTimeout(…, 0) ]
fixes a flicker                     copied into a
agent: "it would be a larger         shared module
change"
(arrows: Sep 9 box -> Oct 3 stack -> Oct 17 box)                 @poteto
```

### Talk slide: comments are liabilities (10:41-11:29)
```
comments are liabilities

bar chart (no y-axis labels, no unit stated)
  jul 31 : 65,824   (tall red outlined bar)
  today  : 1,227    (thin green bar)

also see /no-comments in pstack: github.com/cursor/plugins
```
@poteto

### Talk slide: whenever you correct your agent (seen in the last frames, about 11:31)
```
whenever you correct your agent

step line chart, x ticks: jul 1, aug 1, sep 1, oct 1, nov 1; no y-axis label
step values labelled (left to right): 7, 17, 38, 90, 139, 149, 157, 167

lint against it or categorically eliminate the problem
```
(Axis meaning is not stated on screen; it is a step line rising from 0 at jul 1 to 167 around nov 1.)

## Code & shell

Only the slide snippet (09:01 onward):
```
// release later, in case it comes back
setTimeout(() => release(id), 0)
```
Also `setTimeout(…, 0)` in the Oct 3 stack, and API names in the captions: setTimeout, requestAnimationFrame, requestIdleCallback.

## Observations

- The segment opens with the tail of a channel card summarizing two "hard-to-use / must-not-mix" bans (useEffect banned in product code; draw-side and background-side code in two folders with imports banned), with an orange pill saying the common point is that these drop out of review (i.e. enforced by machine rather than by reviewers). This is the channel's summary, not talk text.
- The channel then re-shows the Sep 9 / Oct 3 / Oct 17 slide (labelled as talk slide 8:10), adding date pills, highlights and a setTimeout explainer card. These are the channel's own emphasis and narration; the Korean-only captions in 08:58-09:17 are channel narration, not translation of the speaker.
- The talk proper (from 09:18): Lauren Tan argues agents (a) write unsolicited markdown (docs, code comments, rules, skills) as a place to justify not doing the right thing, or over-apply a small correction as an eternal hard rule; (b) at Cursor, used imperative timers (setTimeout, requestAnimationFrame, requestIdleCallback) as quick fixes that papered over a clash between VS Code's imperative style and React's declarative style, hurting client and agent performance (garbage collection, lag, jank). This rests on a shown timeline: Sep 9 one-line setTimeout fix with a justifying comment, Oct 3 copied into a shared module, Oct 17 "183 timers". The agent quote "it would be a larger change" is the shown rationale for the shortcut.
- Comments claim: justifying comments are liabilities because they go stale, humans do not read them, they give the agent an escape hatch, and they are not enforceable. Backed on screen by a bar chart 65,824 (jul 31) vs 1,227 (today); the slide does not say what is counted (presumably comments) and has no axis scale. The caption says Grok Bot banned 99% of comments; 1,227 / 65,824 is about 1.9%, roughly consistent with removing ~98%, but the slide itself shows no percentage and the speech says "ban 99%" (a policy), not "reduced by".
- Remedy: lint rules in Grok Bot and Dune with helpful error messages telling the agent to fix the root cause instead of patching around it; the next slide "whenever you correct your agent ... lint against it or categorically eliminate the problem" with a rising step chart (7 to 167 between jul 1 and nov 1) begins at the very end of this segment.
- Bare assertions: agents "love" timers; the architecture choice made agent performance worse (no measurement shown); humans do not read comments.
- Channel additions in this segment: the purple recap card, orange pills/outlines/highlights, the "발표 슬라이드 8:10" label, the setTimeout drawing card. The 183-timer and comment numbers come from the original slides.
