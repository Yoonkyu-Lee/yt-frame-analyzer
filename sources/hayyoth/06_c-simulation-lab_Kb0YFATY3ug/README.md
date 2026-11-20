# 해체 분석: "Simulation in C. We build the Machine, the Machine builds Us"

- 원본: https://youtu.be/Kb0YFATY3ug (Hayyoth, 2026-10-19, 6:26, 무음 화면 녹화)
- 철학 정리: [../PHILOSOPHY.md](../PHILOSOPHY.md) (이 영상의 근거는 `[lab]` 태그)
- 소스: **codeberg.org/hayyoth/hayyoth-lab** (영상 설명에 링크된 저자의 공개 저장소, 커밋 `712a658`, 2026-10-06)

| 경로 | 내용 |
|---|---|
| [verify/accuracy.py](verify/accuracy.py) | 프레임 판독과 공개 저장소 원본을 줄 단위로 대조하는 스크립트 |
| [transcripts/](transcripts/) | 구간별 프레임 판독 원문 5개 |

이 영상은 새 코드를 짜는 영상이 아닙니다. **저자가 미리 만들어 둔 시뮬레이션 프레임워크 `hayyoth-lab`을 시연하고 읽어 주는 영상**입니다. 코드는 공개 저장소에 그대로 있으므로 `src/`에 다시 복원하지 않았습니다. 영상에서 새로 생긴 것은 아래 4절의 주석 추가뿐입니다.

---

## 1. 판독 정확도 검증 (이 분석 파이프라인 전체에 대한 검증)

영상에서 vim이 파일을 열 때 띄운 크기가 공개 저장소의 파일과 **정확히 일치합니다**. git이 Windows에서 내려받으며 줄 끝을 CRLF로 바꾼 1바이트씩을 빼고 계산한 값입니다.

| 파일 | 영상 (vim 열기 메시지) | 저장소 (CRLF 제거) |
|---|---|---|
| lab/conway.c | `[noeol] 42L, 897B` | 42줄, 897B ✅ |
| src/hrender.c | `143L, 3471B` | 143줄, 3471B ✅ |
| src/hcell.c | `80L, 1841B` | 80줄, 1841B ✅ |
| src/hcell.h | `32L, 444B` | 32줄, 444B ✅ |

즉 영상 속 파일은 저장소 파일과 같습니다. 이 사실 덕분에 **Sonnet 에이전트의 프레임 판독 결과를 진짜 원본과 대조할 수 있었습니다.** ([verify/accuracy.py](verify/accuracy.py))

| 파일 | 원본 줄 수 | 글자까지 일치 (들여쓰기 포함) |
|---|---|---|
| conway.c | 42 | **42 (100%)** |
| hrender.c | 143 | **143 (100%)** |
| hcell.h | 32 | **32 (100%)** |
| hcell.c | 80 | 57 (71%). 59~80행은 화면에 나오지 않았고, 판독에서도 `??`로 표시됨 |
| 합계 | 297 | **화면에 나온 274줄 중 274줄 일치 (100%)** |

**결론**: 화면에 보인 코드에 대해 프레임 판독은 들여쓰기까지 정확합니다. 다른 영상에서 남는 몇 바이트 차이는 판독 오류가 아니라, 화면에 보이지 않는 공백 때문이라는 해석이 뒷받침됩니다.

## 2. 타임라인

| 시각 | 내용 |
|---|---|
| 00:00–00:39 | `ls lab`으로 시뮬레이션 8개를 보여 줍니다. `make run-conway`(흰색 Conway), `make run-diffusion`(터미널에 숫자 격자로 표시, 35에서 시작해 1~2로 평평해짐), `make run-forestfire`(초록 숲, 빨간 불, 검은 재)를 실행합니다. |
| 00:39–01:16 | `make run-grayscott`(파란 바탕에 청록 반점이 1→2→4→8→16개로 분열), `make run-karma`(Kármán 와류열, `Re = 100 tau = 0.548`)를 실행합니다. |
| 01:16–02:01 | `make run-ants_findfood`(개미가 먹이를 찾아 페로몬 길을 만듦, `step 500 delivered 323 / 1000`). 시뮬레이션 5개를 동시에 띄웁니다. 브라우저로 codeberg 저장소와 README를 보여 주고, 셸에 `# Source code`, `# How it work`라고 입력합니다. |
| 02:01–04:48 | **코드 읽기.** `lab/conway.c`를 열고 init, rule, view 위에 주석을 직접 답니다. `src/hrender.c`(terminal/raylib 두 가지 출력), `src/hcell.c`, `src/hcell.h`를 차례로 엽니다. |
| 04:48–05:10 | `hcellRun` 위에 이웃 배치도 주석을 답니다(`c[5] c[1] c[6]` / `c[4] c[0] c[2]` / `c[8] c[3] c[7]`). |
| 05:10–06:01 | README의 "3. Neighborhoods" 절을 보여 주고, 자기 채널과 **03번 영상**("Programming magical random moving numbers in C")을 엽니다. |
| 06:02–06:25 | 마지막 셸 주석: **`# You should watch my previous video to understand`**, **`# in this previous video , i made very simple version of this lib , check it , and read source code`** |

## 3. 프레임워크 설계 (`hayyoth-lab`)

README의 첫 문장이 설계를 요약합니다.
```
simple state
     +
simple rules
     ↓
  emergence
```
사용자는 C 함수 3개만 씁니다(conway.c 기준).

| 역할 | 함수 | 프레임워크 호출 |
|---|---|---|
| 상태 초기화 | `void init(void *state, size_t height, size_t width)` | `hDataInit(init)` |
| 규칙 | `void rule(void *c[9], void *n[9])`: `c`는 현재 3×3 이웃, `n`은 다음 상태. 가운데가 `[0]` | `hCell(rule)` |
| 보기 | `HrdView view(const void *cell)`: 칸 하나를 색(r, g, b)으로 바꿈 | `hRender(view)` |

`main`은 `hSetup(...)` → `hDataInit(init)` → `while (hRun()) { hCell(rule); hRender(view); }`로 끝납니다.

- **이중 버퍼**: `state`와 `next`를 둡니다. 규칙은 현재 세계를 바꾸지 않고 `next`에만 씁니다. README에는 "This prevents one cell from seeing a mixture of old and already-updated neighbors"라고 적혀 있습니다.
- **칸 타입을 가리지 않음**: `void *`와 `cellSize`를 씁니다. 어떤 struct든 칸이 될 수 있습니다. 08번 영상의 `elm_size` 기법과 같습니다.
- **격자 밖 이웃은 `NULL`**: 규칙 함수에서 `if (c[i])`로 확인합니다.
- **출력 방식 두 가지**: 같은 규칙을 터미널 문자(diffusion)나 raylib 창으로 보여 줍니다.

## 4. 영상에서 새로 쓴 것: 주석 8줄

저장소 원본과 비교해 영상에서 추가된 줄만 뽑았습니다.

| 파일 | 위치 | 추가한 주석 |
|---|---|---|
| conway.c | `init` 위 | `// Init data` |
| conway.c | `rule` 위 | `// Main Logic simulation` |
| conway.c | `view` 위 | `// Map state to color to render to view` |
| hrender.c | 설정부 | `// Init render`, `// Using raylib`, `// Using raylib image to render` |
| hrender.c | 그리기 함수 | `// Just get state and use view function to map color to image of raylib and render` |
| hcell.h/c | `hcellRun` 위 | 이웃 배치도 `c[5] c[1] c[6] / c[4] c[0] c[2] / c[8] c[3] c[7]` |

## 5. 해석

- **03번 영상은 이 라이브러리의 축소판이었습니다.** 저자가 직접 그렇게 말합니다("i made very simple version of this lib"). 두 영상의 구조가 같습니다.
  - 03번의 `state`/`next` + `memcpy`가 이 라이브러리의 이중 버퍼입니다.
  - 03번의 `moves[8][2]`가 이 라이브러리의 `c[9]` 이웃입니다.
  - 03번의 `render`가 이 라이브러리의 `view`입니다.
- **시간 순서가 흥미롭습니다.** 저장소의 커밋 날짜는 2026-10-03~06이고, 03번 영상은 10-17에 올라왔습니다.
  - 즉 03번 영상을 찍을 때 저자는 이미 `conway.c`를 가진 프레임워크를 갖고 있었습니다.
  - 그런데도 03번에서는 바닥부터 다시 손으로 짰고, Game of Life를 ChatGPT에 물었다가 버렸습니다.
  - 03번은 지식을 처음 얻는 과정이 아니라, **원리를 시청자 앞에서 다시 손으로 재현하는 시연**이었다고 봐야 합니다.
- **영상 속 로컬 사본이 공개 저장소보다 새 버전입니다.** 저장소에서는 `karma.c`와 `ants_findfood.c`가 `learn/`에 있는데, 영상에서는 `lab/`에 있습니다.
- **코드 설명도 주석으로 합니다.** 영상이 무음이라 함수 위에 한 줄 주석을 달면서 설명합니다(PHILOSOPHY F1). 설명하려고 단 주석이지만 코드에도 남습니다.
