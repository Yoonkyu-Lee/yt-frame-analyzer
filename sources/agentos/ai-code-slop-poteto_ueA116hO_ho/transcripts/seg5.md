# seg5 (06:41 - 08:58)

Layers: channel insert cards (dark purple, Korean only, one Korean caption line, no English line) at 06:41-06:54 and 08:53-08:58; original talk 06:55-08:52 (English caption line in italic + Korean caption line below; speaker camera bottom-left, conference sidebar "AI Engineer Code Summit 2026 / Lauren Tan (SpaceXSI) · 2026-11-11 / PRESENTED BY Google DeepMind", lower-third "All Code Is Slop (Except Mine) / Lauren Tan / Principal Engineer / SPACEX" are ignored).

## Timeline

### Channel insert card A (06:41 - 06:54)
- [06:41] Two-panel card. Left "① 쓰기 까다로운 기능" with pill "useEffect" and dashed box "결과 ?". Right "② 섞이면 안 되는 코드", empty, dashed box "결과 ?". KO caption: "제대로 쓰기가 까다로운 기능"
- [06:42] Same. KO caption: "정도로 들으시면 돼요." (this and the 06:41 line are shown in this order in the frames; the card seems to be a short loop/replay at the start of the segment, the caption order is as seen)
- [06:43] Right panel gets pill "화면을 그리는 쪽". KO caption: "두 번째는 화면을 그리는 쪽 코드와"
- [06:44] Right panel: "화면을 그리는 쪽" ~(orange squiggle)~ "뒤에서 일하는 쪽". No caption.
- [06:45] KO caption: "뒤에서 일하는 쪽 코드가 섞이면"
- [06:46] Orange text under squiggle: "→ 앱이 버벅임". KO caption: "앱이 버벅인다는 얘기예요."
- [06:47] KO caption: "백엔드로 치면,"
- [06:48] KO caption: "서로 다른 프로세스에서 도는"
- [06:49] KO caption: "모듈끼리 서로 가져다 쓰지 못하게"
- [06:50] KO caption: "모듈끼리 서로 가져다 쓰지 못하게" (same, frame shows it again)
- [06:51] KO caption: "경계를 막아 두는 것과 비슷해요."
- [06:52] Orange pill appears at bottom center: "두 예시의 공통점 ?". KO caption: "겉보기엔 다른 두 예시가"
- [06:53] same, KO caption: "겉보기엔 다른 두 예시가"
- [06:54] same, KO caption: "한 지점에서 만나요."

### Original talk (06:55 - 08:52)
- [06:55] Slide "dune: a framework for agents" (see Texts). EN: "All right, let's look at some real examples." KO: "자, 실제 예시를 볼게요."
- [06:58] EN: "So Dune is a client framework that we created for Grok Bot" KO: "Dune은 저희가 Grok Bot용으로 만든 클라이언트 프레임워크예요."
- [07:02] EN: "to implement this idea for organic architecture." KO: "이 유기적 아키텍처라는 아이디어를 구현하려고요."
- [07:06] EN: "It's not really that interesting technically." KO: "기술적으로는 그렇게 흥미롭진 않아요."
- [07:08] EN: "You can think of Dune as kind of like Next.js for Electron apps." KO: "Electron 앱용 Next.js 정도로 생각하시면 돼요."
- [07:13] EN: "But the interesting part of Dune is that it has a lot of constraints and lint rules" KO: "그런데 Dune의 흥미로운 점은 제약과 린트 규칙이 아주 많다는 거예요."
- [07:17] EN: "that we've encoded a lot of the lessons that we've learned building desktop applications." KO: "데스크톱 앱을 만들며 배운 교훈을 거기에 많이 담아 뒀죠."
- [07:23] Slide changes to "nuance tends to lead to slop / just design better APIs" (cartoon bottom right). Caption still the 07:17 line.
- [07:24] EN: "And one such lesson is that anything that requires nuance to use well often leads to slop." KO: "그 교훈 중 하나는, 잘 쓰려면 미묘한 판단이 필요한 건 슬롭으로 이어지기 쉽다는 거예요." (KO wraps to two lines)
- [07:31] EN: "Your classic example is useEffect in React." KO: "대표적인 예가 React의 useEffect예요."
- [07:35] EN: "It has its use cases, but it's often misused." KO: "쓸 데가 있긴 한데, 잘못 쓰이는 경우가 많죠."
- [07:39] EN: "Some people joke that you need a PhD to use useEffect." KO: "useEffect 쓰려면 박사 학위가 필요하다는 농담도 있어요."
- [07:43] EN: "And in other words, it really requires a lot of nuance to use well." KO: "다시 말해, 잘 쓰려면 미묘한 판단이 정말 많이 필요하다는 거죠."
- [07:48] EN: "And agents are not good at using it." KO: "그리고 에이전트는 그걸 잘 못 써요."
- [07:51] EN: "So our solution has been to just ban it in product code" KO: "그래서 저희 해결책은 제품 코드에선 아예 금지하는 거였어요."
- [07:55] EN: "and it's really only allowed in some places inside of Dune APIs." KO: "Dune API 안쪽 몇 군데에서만 허용돼요."
- [08:00] EN: "And effectively, this problem has now been categorically eliminated," KO: "사실상 이 문제는 이제 범주째 사라졌고,"
- [08:04] EN: "and we never need to catch that during code review again." KO: "코드 리뷰에서 다시 잡아낼 일도 없어요."
- [08:07] Slide changes to "directory structure is a form of context" (first state: Dune diagram + tibo.dog). Caption still the 08:04 line.
- [08:08] EN: "Another interesting pattern that we use is context through directory structure." KO: "저희가 쓰는 또 하나의 흥미로운 패턴은 디렉터리 구조로 맥락을 주는 거예요."
- [08:13] EN: "So one anti-pattern that we saw with Cursor, which is an Electron app," KO: "Electron 앱인 Cursor에서 봤던 안티패턴 하나는,"
- [08:17 / 08:18 / 08:19] EN: "is that agents would often import main thread code into the render thread," KO: "에이전트가 메인 스레드 코드를 렌더 스레드로 자주 import 하는 거였어요." (sidebar logo flickers at 08:17)
- [08:22] EN: "which would cause long tasks and jank." KO: "그러면 긴 작업과 버벅임이 생기죠."
- [08:24] same caption; diagram starts animating (boxes shift apart).
- [08:25] Diagram state 2 (boxes spread apart, still without shared/ and the X arrow). Caption as 08:24... (frame 08:25 shows the fully built diagram with caption below)
- [08:25] EN: "So in Grok Bot and in Dune apps, we've made this very, very conventional." KO: "그래서 Grok Bot과 Dune 앱에선 이걸 아주 철저하게 관례로 만들었어요." Diagram now fully built: X'd dashed arrow "not even types" between features/ and electron-main/, new box shared/ with label "shared types" and arrows from both sides.
- [08:31] EN: "We have these two top-level directories, literally called electron-renderer and electron-main." KO: "최상위 디렉터리가 두 개 있는데, 이름이 말 그대로 electron-renderer와 electron-main이에요." (two lines)
- [08:36] EN: "And imports between them are forbidden or banned by our dependency graph analyzer and some lint rules." KO: "그 둘 사이의 import는 의존성 그래프 분석기와 린트 규칙 몇 개로 막아 뒀어요."
- [08:44] EN: "And again, these are simple lint rules to set up," KO: "역시 설정하기 쉬운 린트 규칙이지만,"
- [08:46] EN: "but it means one less thing that we need to review and think about during code review." KO: "덕분에 코드 리뷰 때 살피고 고민할 게 하나 줄어요."
- [08:52] same caption.

### Channel insert card B (08:53 - 08:58)
- [08:53] Card with only the left panel "① 쓰기 까다로운 기능", pill "useEffect", dashed "결과 ?". No caption.
- [08:54] Both panels. Left result box is now filled, orange border: "제품 코드에서 금지". Right panel (화면을 그리는 쪽 ~ 뒤에서 일하는 쪽, → 앱이 버벅임) with dashed "결과 ?". KO caption: "제품 코드에서 아예 못 쓰게 했고,"
- [08:55] same, same caption.
- [08:56] KO caption: "섞이면 안 되는 건"
- [08:57] Right result box filled: "폴더 둘 + import 금지". KO caption: "폴더와 린트로 갈라 놨어요."
- [08:58] Orange filled pill at bottom: "공통 · 리뷰에서 빠짐". KO caption: "두 경우 모두 리뷰에서"

## Texts on screen

### Channel card A/B, final state (06:41-06:54 building; 08:53-08:58 building)
```
① 쓰기 까다로운 기능                 ② 섞이면 안 되는 코드
   [useEffect]                          [화면을 그리는 쪽] ~ [뒤에서 일하는 쪽]
                                         → 앱이 버벅임
 [ 결과 ? ]  → later: [제품 코드에서 금지]   [ 결과 ? ] → later: [폴더 둘 + import 금지]

 (06:52-06:54) orange pill: 두 예시의 공통점 ?
 (08:58)       orange pill: 공통 · 리뷰에서 빠짐
```
(translation) ① Feature that is hard to use properly: useEffect; result? -> banned in product code. ② Code that must not be mixed: the side that draws the screen / the side that works in the back -> app stutters; result? -> two folders + imports banned. Pills: "What do the two examples have in common?" / "Common: dropped out of review".

### Slide "dune: a framework for agents" (06:55 - 07:23)
```
dune: a framework for agents

[ app framework                 ]  +  [ one folder per feature ]  (blue box)
[ navigation, rendering, IPC    ]     [ strict import bans     ]  (green box)
                                      [ no comments in code    ]  (orange box)
@poteto
```
("navigation, rendering, IPC" is smaller grey subtext)

### Slide "nuance tends to lead to slop" (07:23 - 08:07)
```
nuance tends to lead to slop
just design better APIs        (pink text)
@poteto
```
Cartoon, bottom right: a frowning long-haired girl with glasses next to a small robot sitting on a machine with knobs, tangled cables, "??" marks.

### Slide "directory structure is a form of context" (08:07 - 08:52)
State 1 (08:07 - 08:22):
```
directory structure is a form of context

Dune
 +- electron-renderer/ --------------+   +- electron-main/ -+
 |  [features/] (blue)               |   |  [startup/]       |
 |  [client/] (green) [primitives/] (purple) | [notifications/] |
 +-----------------------------------+   |  [update/]  (red) |
                                          +------------------+
tibo.dog (side project)
 [src/
    sim/
      tick.hot.ts]  -> [runs every tick] (blue)
                    -> [no allocations] (green)
@poteto
```
State 2 (08:25 - 08:52), final: the two dashed groups are pushed apart; a dashed horizontal double arrow from features/ to the electron-main/ group with a black X in the middle and label "not even types"; between the groups a yellow box "shared/" with label "shared types", with solid arrows from electron-renderer/ group and from electron-main/ group pointing into it. Dune group title "Dune", subgroup labels "electron-renderer/" and "electron-main/"; renderer contains features/, client/, primitives/; main contains startup/, notifications/, update/. Bottom block unchanged: "tibo.dog (side project)", code box "src/ / sim/ / tick.hot.ts", boxes "runs every tick", "no allocations".

## Code & shell
None (only the file path label "src/ sim/ tick.hot.ts" in the tibo.dog diagram, shown above).

## Observations
- Argument: constraints and lint rules, not careful judgment in review, keep agents from producing slop. Principle stated on slide: "nuance tends to lead to slop / just design better APIs". Two examples: (1) useEffect is banned in product code and allowed only inside a few Dune APIs, so that class of bug is "categorically eliminated" and never has to be caught in code review; (2) directory structure carries context: two top-level directories literally named electron-renderer and electron-main, imports between them banned by a dependency graph analyzer plus some lint rules, with a shared/ folder for shared types only ("not even types" crossing directly).
- Evidence shown: diagrams and the slide texts only. No numbers in this segment. Claims such as "categorically eliminated", "agents are not good at using it", and the Cursor anti-pattern (agents importing main-thread code into the render thread causing long tasks and jank) are spoken assertions with no data on screen. "PhD to use useEffect" is a joke. The tibo.dog side project (hot file tick.hot.ts: runs every tick, no allocations) appears on the slide but is not discussed in the captions of this segment.
- Channel additions: the dark purple two-panel card (06:41-06:54) is a preview of the two examples (hard-to-use feature: useEffect; code that must not mix: UI side vs background side, "→ 앱이 버벅임") with "결과 ?" placeholders and the question pill "두 예시의 공통점 ?" placed BEFORE the talk shows the examples. The Korean captions there are the channel's own narration (not translations of an English line): it adds an analogy not on the slides: "백엔드로 치면, 서로 다른 프로세스에서 도는 모듈끼리 서로 가져다 쓰지 못하게 경계를 막아 두는 것과 비슷해요" (like preventing modules in different backend processes from importing each other). The same card returns at 08:53-08:58 with the answers filled in ("제품 코드에서 금지", "폴더 둘 + import 금지") and the punchline pill "공통 · 리뷰에서 빠짐" (both rules take the item out of code review), i.e. the channel frames the section as "enforce by tooling so review no longer has to catch it". The Korean captions on talk frames are the channel's translation; one notable rendering: "철저하게 관례로" for "very, very conventional", "범주째 사라졌고" for "categorically eliminated".
