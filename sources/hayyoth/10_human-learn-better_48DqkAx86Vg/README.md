# 해체 분석: "How we (Human) learn better than AI?"

- 영상: https://youtu.be/48DqkAx86Vg (11:37, 2026-10-23 업로드), 태그 `[learn]`
- 결과물: 논증 정리 (이 README), 전사 [transcripts/](transcripts/) (6구간)
- 방법: 프레임 697장 → 웹캠을 가리고 임계값을 높인 키프레임 295장 → 6구간 전사(글 중심 형식). 오디오는 쓰지 않았습니다. 4구간은 첫 시도가 API 콘텐츠 필터로 중단되어 다시 돌렸고, 다시 돌린 에이전트가 거의 같은 프레임을 건너뛰어 51장 중 약 36장만 봤습니다.

## 1. 형식

코딩 영상이 아니라 주장입니다. 노란 메모에 질문을, 하늘색 메모에 한 문장씩 답을 바꿔 쓰며 말하고, 그 뒤에 증거 화면을 띄웁니다.

## 2. 논증의 흐름

| 시각 | 하늘색 메모 (차례로 다시 씀) | 화면의 증거 |
|---|---|---|
| 0:00~1:42 | "We love complexity and abstraction" / "we build everything complex and big / Love abstraction - Container / Cost / - It hard to learn from them" | 자동차·항공기 공장, 컨테이너, JS 프레임워크 목록과 크기 비교표 |
| 1:42~3:48 | "Do you know how to make car ??" / "people in 19s know / when we still use hand" / "do you understand it / very hard" / "but you can understand this" / "how understand ?? / read source code ???" | GitHub의 Rust `tree` 저장소 ↔ 자기의 C `tree` 약 55줄(02번 영상), 자기 시뮬레이션 저장소의 `hrender.c` |
| 3:48~5:34 | "when everything small and simple / you understand it" / "but AI can learn the complex" | 자기 저장소 코드, 이전 영상들 |
| 5:34~7:38 | "it work / AI love complex / your brain not" | 자기 저장소 README를 Gemini에 붙이자 **C 산불 시뮬레이션을 만들어 냄**. 컴파일해 실행(배경의 초록 격자). 이어 Kernighan & Pike *The Practice of Programming*(표지 "Simplicity / Clarity / Generality")의 연결 리스트 장 |
| 7:38~8:58 | "Hard to understand" / "this is linked list / your brain love it / simple , tiny" | 책의 리스트·해시 테이블 장, 자기의 연결 리스트(07)·해시 테이블(09) 영상, Gemini의 해시 테이블 코드, 위키백과 "Hash table" |
| 8:58~11:36 | "Hash table is just key - value" / "it hard to understand / take time" / "dynamic array is just realloc" | 자기의 작은 영상들 ↔ 위키백과·Gemini의 긴 설명과 코드 |

마지막 노란 메모(영상 설명과 같음): **"What Human problems? We love complexity and abstraction / Answer - start from zero (scratch) / no abstraction / make everything simplicity / you can learn better than AI / you opinion??"**

## 3. 해석

- **AI의 능력은 인정합니다.** README 하나로 동작하는 시뮬레이션을 만들어 내는 것을 직접 보여 주고 "it work"라고 씁니다. 반대하는 대상은 AI가 아니라, 사람이 AI처럼 복잡한 것에서 배우려는 습관입니다. "AI love complex / your brain not."
- **사람은 작은 것에서 배운다는 것이 채널의 교육 이론입니다.** 해시 테이블은 "key-value", 동적 배열은 "realloc"으로 줄일 수 있을 때 이해한 것이라는 주장입니다. 02~09번 영상들이 왜 그렇게 작은지에 대한 작가 자신의 설명입니다. 증거로 고른 책이 Kernighan & Pike(단순, 명료, 일반성)라는 점도 같은 방향입니다.
- **추상화에는 비용이 있습니다.** 컨테이너, 프레임워크, 큰 저장소를 "Cost / It hard to learn from them"으로 묶습니다. 이 리포의 northstar(의도 밀도)가 바이브 코딩에서 걱정한 것과 같은 지점입니다.
