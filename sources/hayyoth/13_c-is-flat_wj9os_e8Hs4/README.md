# 해체 분석: "C is Flat. If everything is Flat, you can't go wrong."

- 영상: https://youtu.be/wj9os_e8Hs4 (18:10, 2026-10-27 업로드), 태그 `[flat]`
- 결과물: 강의 흐름과 논증 정리 (이 README), 전사 [transcripts/](transcripts/) (8구간). 화면 속 C 코드는 대부분 Gemini가 쓴 시연용이라 소스로 복원하지 않았고, 원문은 전사에 있습니다.
- 방법: 프레임 1,090장 → 웹캠을 가린 키프레임 305장 → 8구간 전사(글 중심 형식). 오디오는 쓰지 않았습니다.

## 1. 형식

코딩 영상이 아니라 강의입니다. 화면에 미리 붙여 둔 두 장의 메모가 논제입니다.

- 하늘색: **"If everything is flat, you can't go wrong." — Professor Gilbert Strang**
- 초록색: **"Memory hardware is flat / C near hardware / C is flat / ---- / just convention"**

노란 메모 한 장을 계속 다시 써서 자막처럼 씁니다. 도구는 이미지 검색, Google Sheets(메모리 그림), Gemini(코드 생성), vim과 터미널입니다.

## 2. 흐름

| 시각 | 한 일 | 노란 메모 |
|---|---|---|
| 0:00~2:59 | "flat"이 무엇인지: 평평한 지구 그림(평평함)과 지구본(평평하지 않음), RAM 모듈 사진(평평함). 시트에 Address 열과 Memory 칸을 만들고 "한 칸에 0이나 1만 담으면 bit"라는 메모를 붙임 | "what is flat?" → "this is flat" → "this is not flat" |
| 2:59~6:25 | 한 행의 비트를 숫자로 읽는 Decimal 열 (0001→1, 0010→2). vim에 `int bytes[4] = {0};`와 주석 `// RULE: each cell can storage 0 or 1` | "this is computer memory", "just convention / we set the rule", "simulation this in C" |
| 6:25~10:52 | 자기 뼈대를 Gemini에 붙여 `get_bit`/`set_bit`/`get_decimal`을 받아 실행(1010→10, 1000→8, 0001→1). 이어 `get_char`(1→'a')와 Character 열 | "get_decimal -> conventation -> rule of us", "or we can conventation like this", "what about struct" |
| 10:52~13:25 | 시트의 칸을 id / age 묶음으로 다시 이름 붙여 "사람" 레코드를 만듦. Gemini가 struct로 답하자 **"No, don't use struct, create state int[] like bytes[]"**라고 되돌려 `int memory[12]`, 한 사람 = `slot * 3`부터 세 칸으로 받음 | "so, memory alway flat / we set the rule / this is how we get and set value" |
| 13:25~15:19 | Gemini 코드를 `main2.c`로 실행("Raw value at address 1 … 30") | "memory is just array / still flat / just how we get and set", "C is flat / C love flat" |
| 15:19~18:10 | `test.c`로 다른 언어의 기능을 시험: 함수 안의 함수(컴파일 안 됨 → 바깥으로), 구조체 안의 구조체(손으로 `int x, y, r`로 펼침), JS식 `import`("not flat")와 `#include`("just copy all def.h … still flat") | "why C not have this / because it not flat", "under the hood C will do this / always flat", "understand this will help you not hate C" |

마지막에 Strang 인용 메모를 맨 앞으로 끌어와 굵게 만들고 녹화를 끝냅니다.

## 3. 해석

- **논제: C는 하드웨어처럼 평평하고, 그 위의 의미는 약속이다.** 비트를 숫자로 읽는 것, 칸 묶음을 "사람"으로 부르는 것, 구조체, `#include`가 모두 평평한 칸 위에 사람이 정한 규칙(convention)이라는 주장입니다. 그래서 중첩 함수나 모듈 같은 "평평하지 않은" 기능이 C에 없는 것은 결함이 아니라 하드웨어에 가까운 선택이라고 봅니다. 마지막 메모 "understand this will help you not hate C"가 영상의 동기입니다.
- **AI를 타자수로 씁니다. 처음으로 AI가 쓴 코드를 그대로 실행합니다.** 09에서는 AI 코드를 교재로 다시 짰고, 11에서는 거절했습니다. 여기서는 Gemini가 쓴 시연 코드를 받아서 돌립니다. 다만 뼈대(`bytes[4]`, RULE 주석)는 자기가 먼저 쓰고, AI가 struct로 답하자 **자기 모델(평평한 배열)로 되돌리게 합니다.** 지운 메모 "i know gemini will write what before it write / this is good way understa…"는 "AI가 무엇을 쓸지 미리 알면 이해한 것"이라는 시험으로 읽힙니다. 이해의 소유권은 넘기지 않고, 타이핑만 맡깁니다(PHILOSOPHY E1의 변형).
- **그림으로 먼저 생각합니다.** 메모리를 스프레드시트 칸으로 그리고, 그 그림을 코드로 옮기고, 실행 결과를 다시 그림의 행에 맞춰 봅니다. 11에서 `d`를 짜기 전에 vim 버퍼에 그림을 그린 것과 같은 습관입니다.
