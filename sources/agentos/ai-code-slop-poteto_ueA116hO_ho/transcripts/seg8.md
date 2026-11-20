# Segment 8 (12:59 - 14:52), ueA116hO_ho

## Timeline
Original talk (slides; speaker cam and conference sidebar ignored). Caption lines: E = English burned-in caption, K = Korean caption (quoted only when it adds something, or when there is no English line, i.e. during channel inserts).

- [12:59] Slide "runtime verification" with a "feature map" diagram. Footer: "also see /create-verification-skill in pstack: github.com/cursor/plugins", "@poteto".
  E: "And the most basic example of this is runtime verification,"
- [13:03] E: "which is just a fancy way of saying giving your agent a way to run the code for itself"
- [13:07] E: "and use debugging tools that human developers use as well,"
- [13:11] E: "like the Chrome DevTools Protocol."
- [13:14] E: "In our runtime verification skills, we have this thing that we call a feature map."
- [13:18] E: "And that's basically kind of like a site map of all of the features that are available in your application from the user's point of view."
- [13:26] E: "And so with this, agents don't need to guess when you say the thingy in the left sidebar."
- [13:31] E: "It gives you some shared vocabulary."
- [13:33] Build step (fade-in): feature map shifts left; "+" and a "CLI" box appear, plus a purple bar "agent checks its own work". Caption unchanged.
- [13:34] Build complete (see slide text below).
  E: "And we also make use of custom CLIs inside these skills so that agents can reuse the CLI and the driver every time,"
- [13:42] E: "which cuts down on token usage and is also testable." (through 13:45)
- [13:46] Cut to blank dark purple screen (channel insert starts; original captions stop).
- [13:47] CHANNEL INSERT card 1: headline "사람이 보는 것 = 검증의 증거". K: "사람이 보는 건 코드가 아니라"
- [13:48] K: "검증의 증거였어요."
- [13:49] Orange pills appear: "영상", "스크린샷". K: "영상, 스크린샷, 다이어그램"
- [13:50] Pills now: "영상", "스크린샷", "다이어그램", "형식 검증". K: "같은 것들이요."
- [13:51] White pill appears: "에이전트가 직접 실행 + 증거 = 런타임 검증". K: "에이전트가 실제 앱을 직접 돌려 보고" (through 13:53)
- [13:54] K: "그 증거를 남기는 게"
- [13:55] K: "런타임 검증이고요."
- [13:56] CHANNEL INSERT card 2: maze illustration with flags and a hand placing an orange flag. K: "다음에는 낯선 이름이"
- [13:57] Box fades in: "TLA+  모든 상태를 펼쳐서 검사". K: "TLA+ 는 시스템이" (through 13:58)
- [13:59] K: "가질 수 있는 상태를 전부 펼쳐 놓고,"
- [14:00] No caption (maze + TLA+ box).
- [14:01] K: "기계가 따져 보게 하는 도구예요."
- [14:02] Stepping-stones illustration (checkmarks, orange top stone); TLA+ box dims; new orange box "Lean  한 단계씩 증명". K: "Lean 은 알고리즘이 맞다는 걸"
- [14:03] K: "수학 증명처럼"
- [14:04] K: "한 단계씩 확인하는 도구고요."
- [14:05] Funnel-and-gauge illustration (random shapes poured into a box with a dial); third box highlighted: "속성 기반 테스트  조건 + 무작위 입력". K: "속성 기반 테스트는"
- [14:06] K: "예시를 몇 개 손으로 적는 대신," (through 14:07)
- [14:08] K: "어떤 입력이 와도"
- [14:09] K: "지켜져야 하는 조건을 적어 두고"
- [14:10] K: "입력을 무작위로 쏟아붓는 방식이에요." (through 14:11)
- [14:12] Pill appears under diagram: "어디에 썼고, 어디까지인가?". K: "로렌 탄 팀은 이걸"
- [14:14] K: "스스로 어디까지라고"
- [14:15] K: "선을 그어요."
- [14:16] Back to original talk: slide "formal verification" (cartoon, two speech bubbles, TLA+ model / Lean proof boxes). No caption.
- [14:17] E: "And then recently, we've also started looking at formal verification."
- [14:21] E: "So we now use TLA+ to do some model checking of major user flows,"
- [14:25] E: "such as notifications across multiple devices and platforms."
- [14:30] E: "We use Lean as well for proving certain algorithms."
- [14:33] E: "For example, we wrote our own custom virtualization library"
- [14:37] E: "and we use Lean there to check that we don't have flickering or jittering problems."
- [14:42] E: "And we're still experimenting with this approach overall."
- [14:45] E: "It seems very promising."
- [14:47] E: "We've already found quite a few bugs with formal verification."
- [14:52] E: "But it's certainly not a silver bullet."

Korean caption differences worth noting: at 14:30 K adds a gloss: "특정 알고리즘을 증명할 땐 Lean(정리 증명기)도 써요." (Lean = theorem prover). At 13:14 K adds a gloss: "저희 런타임 검증 스킬에는 '피처 맵(기능 지도)'이라는 게 있어요." Other K lines at 12:59-14:52 are straightforward translations (e.g. 13:34 "스킬 안에선 전용 CLI도 써서, 에이전트가 매번 같은 CLI와 드라이버를 재사용하게 해요."; 13:42 "그러면 토큰 사용량도 줄고, 테스트도 할 수 있죠."; 14:42 "전반적으로는 아직 실험 중이에요."; 14:45 "꽤 유망해 보여요."; 14:47 "형식 검증으로 이미 버그를 꽤 찾았어요."; 14:52 "하지만 은탄환은 절대 아니에요.").

## Texts on screen

### Slide: runtime verification (12:59 - 13:32)
```
runtime verification

[box] feature map
  (graph: circle -- orange circle -- two branches, each ending: circle -- green circle)
  what features exist      how users reach it      what proves it works

also see /create-verification-skill in pstack: github.com/cursor/plugins
@poteto
```

### Slide: runtime verification, built out (13:33 - 13:45)
```
runtime verification

[blue box] feature map
  (same graph)  what features exist | how users reach it | what proves it works
        +
[orange box] CLI
  $ app aria-click --accessible-name "Settings"
  $ app snapshot
  $ app screenshot proof.png
  drive the real app

[purple bar] agent checks its own work

also see /create-verification-skill in pstack: github.com/cursor/plugins
@poteto
```

### Channel insert card 1 (13:46 - 13:55), dark purple
```
사람이 보는 것 = 검증의 증거
(translation) What a human looks at = the evidence of verification

[orange pills] 영상   스크린샷   다이어그램   형식 검증
(translation) video / screenshot / diagram / formal verification

[white pill] 에이전트가 직접 실행 + 증거 = 런타임 검증
(translation) agent runs it itself + evidence = runtime verification
```
Burned Korean captions (no English line): "사람이 보는 건 코드가 아니라 / 검증의 증거였어요. / 영상, 스크린샷, 다이어그램 / 같은 것들이요. / 에이전트가 실제 앱을 직접 돌려 보고 / 그 증거를 남기는 게 / 런타임 검증이고요."
(translation) What humans look at isn't the code but the evidence of verification: things like video, screenshots, diagrams. The agent actually runs the real app and leaving that evidence is runtime verification.

### Channel insert card 2 (13:56 - 14:16), dark purple, boxes added one by one
```
[maze illustration with flags, hand placing orange flag]   TLA+  모든 상태를 펼쳐서 검사
(translation) TLA+: expand every state and check

[stepping stones with checkmarks]                          Lean  한 단계씩 증명
(translation) Lean: prove one step at a time

[funnel pouring shapes into a box with a gauge]            속성 기반 테스트  조건 + 무작위 입력
(translation) Property-based testing: condition + random input

[pill, from 14:12]  어디에 썼고, 어디까지인가?
(translation) Where did they use it, and how far?
```
Burned Korean captions: "다음에는 낯선 이름이 / TLA+ 는 시스템이 / 가질 수 있는 상태를 전부 펼쳐 놓고, / 기계가 따져 보게 하는 도구예요. / Lean 은 알고리즘이 맞다는 걸 / 수학 증명처럼 / 한 단계씩 확인하는 도구고요. / 속성 기반 테스트는 / 예시를 몇 개 손으로 적는 대신, / 어떤 입력이 와도 / 지켜져야 하는 조건을 적어 두고 / 입력을 무작위로 쏟아붓는 방식이에요. / 로렌 탄 팀은 이걸 / 스스로 어디까지라고 / 선을 그어요."
(translation) Next, unfamiliar names. TLA+ is a tool that lays out every state a system can have and has a machine check it. Lean checks that an algorithm is correct one step at a time, like a math proof. Property-based testing, instead of hand-writing a few examples, writes down the condition that must hold for any input and pours random inputs in. Lauren Tan's team draws the line itself on how far (this goes).

### Slide: formal verification (14:16 - 14:52)
```
formal verification

[cartoon: girl with glasses holding a paper with a green check, two small robots]
  speech bubble (blue "TLA+"):  Model a state machine. Check that its rules hold.
  speech bubble (red "Lean"):   Write proofs about code. Check every logical step.

[blue box]  TLA+ model   model state machines and check that all states are correctly handled
[red box]   Lean proof   check implementation for bugs

also see /create-formal-verification-skill in pstack: github.com/cursor/plugins
@poteto
```

## Observations
- Speaker teaches: runtime verification means giving the agent a way to run the code itself and use the same debugging tools humans use (Chrome DevTools Protocol). Two parts: a "feature map" (a site map of user-visible features: what exists, how users reach it, what proves it works) that gives agents and humans shared vocabulary so the agent does not guess what "the thingy in the left sidebar" means; and custom CLIs inside the skills (shown example: `app aria-click --accessible-name "Settings"`, `app snapshot`, `app screenshot proof.png`) so the agent reuses the same CLI and driver every time. The claim that this "cuts down on token usage and is also testable" is a bare assertion (no number on screen).
- Formal verification: TLA+ for model checking major user flows (example: notifications across multiple devices and platforms); Lean for proving certain algorithms (example: their own custom virtualization library, checked for flickering or jittering). Speaker hedges: "still experimenting", "seems very promising", "already found quite a few bugs", "certainly not a silver bullet". No bug counts, screenshots or data on screen: all bare assertions, only the slide's one-line definitions are shown.
- Channel added (13:46 - 14:16, about 30 s of full-screen cards; the original talk is paused and only Korean captions run): (1) a reframing card "사람이 보는 것 = 검증의 증거" listing video / screenshot / diagram / formal verification as evidence types and the equation "agent runs it itself + evidence = runtime verification" (the channel's synthesis, not stated in the English captions of this segment); (2) a plain-language explainer of TLA+ (maze/flags), Lean (stepping stones), and property-based testing (funnel + gauge). Property-based testing is not mentioned in the English captions of this segment; the channel inserts it (chapter "형식 검증과 속성 기반 테스트" starts 13:46). The pill "어디에 썼고, 어디까지인가?" and the claim that Lauren Tan's team draws the line itself are channel framing. Korean captions on the main talk also add glosses "피처 맵(기능 지도)" and "Lean(정리 증명기)".
