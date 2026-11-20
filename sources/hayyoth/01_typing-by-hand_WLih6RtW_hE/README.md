# 해체 분석: "No AI. Do you remember the days when we used to type code by hand in the terminal?"

- 원본: https://youtu.be/WLih6RtW_hE (Hayyoth, 2026-10-15, 1:49, 무음 화면 녹화). **채널의 첫 영상**입니다.
- 철학 정리: [../PHILOSOPHY.md](../PHILOSOPHY.md) (이 영상의 근거는 `[intro]` 태그)

| 경로 | 내용 |
|---|---|
| [src/main.c](src/main.c) | 최종 코드 (10줄) |
| [history/](history/) | vim 저장 시점별 2개 버전 |
| [transcripts/](transcripts/) | 구간별 프레임 판독 원문 2개 |

---

## 1. 복원 신뢰도

| 버전 | 시점 | 화면의 vim 저장 메시지 | 복원본 | 결과 |
|---|---|---|---|---|
| [v1](history/v1_0049_hello.c) | 00:49 | `"main.c" [New] 7L, 79B written` | 7줄, 79B | ✅ 완전 일치 |
| [v2](history/v2_0127_no_ai_loop.c) | 01:30 | `"main.c" 10L, 123B written` | 10줄, 123B | ✅ 완전 일치 |

- 두 버전 모두 **바이트 단위로 일치**합니다.
- 문자열 `"No AI --- "`는 1:28 프레임을 원본 해상도로 확대해 확인했습니다. 하이픈 3개 뒤에 공백 1칸이 있고 `\n`은 없습니다. 터미널 폰트의 합자(ligature) 때문에 하이픈이 한 줄로 이어져 보입니다.
- 저자와 같은 플래그(`-Wall -Wextra -pedantic -Wno-unused-parameter`)로 컴파일하면 경고가 없습니다. 실행하면 `Hello World` 한 줄 뒤에 `No AI --- `가 끝없이 출력됩니다.

---

## 2. 최종 산출물

```c
#include <stdio.h>

int main(void) {
  printf("Hello World\n");
  while (1) {
    printf("No AI --- ");
  }

  return 0;
}
```

---

## 3. 타임라인

| 시각 | 내용 |
|---|---|
| 00:00–00:03 | 빈 데스크톱에서 Dock으로 Terminal을 엽니다. |
| 00:07 | `scam start` 실행 → `starting camera... / camera started / pid: 1045 / log: /tmp/scam.log`. 웹캠을 켜는 저자의 **자작 도구**로 보입니다. |
| 00:09–00:16 | "Human" 웹캠 창이 크게 떴다가 줄어들고 오른쪽에 자리 잡습니다. 이후 영상에서 계속 쓰이는 화면 구성이 이때 만들어집니다. |
| 00:19–00:29 | `mkdir my-project`, `cd my-project`, `ls`(비어 있음), `vim main.c` |
| 00:29–00:49 | Hello World를 한 글자씩 직접 입력하고 저장합니다 (`7L, 79B`). |
| 00:50–00:59 | 두 번째 탭(zsh)을 열고 `cc main.c`, `./a.out` → `Hello World` |
| 01:02–01:30 | vim으로 돌아가 `while (1) { printf("No AI --- "); }`를 추가하고 저장합니다 (`10L, 123B`). |
| 01:32 | `cc main.c`를 실행하는 순간 탭 제목에 `...extra -pedantic -Wno-unused-parameter main.c`가 보입니다. `cc`가 엄격한 경고 플래그를 붙이는 alias라는 뜻이고, tree 영상에서 확인한 것과 같습니다. |
| 01:35–01:44 | `./a.out`이 화면을 `No AI --- `로 가득 채웁니다(한 행에 9번, 28행). Ctrl-C로 멈춥니다. |
| 01:47 | 녹화를 종료합니다. |

---

## 4. 해석: 이 영상은 무엇인가

- **채널의 선언문**입니다. 코드는 사실상 Hello World뿐입니다. 기술을 가르치는 영상이 아니라, 채널이 앞으로 무엇을 할지 화면으로 보여 주는 영상입니다.
  - 제목이 "Do you remember the days…"입니다. 터미널에서 손으로 코드를 치던 감각을 되살리자는 뜻입니다.
  - 프로그램의 출력 `No AI --- `가 끝없이 반복됩니다. 이 출력이 곧 메시지입니다.
- **이후 영상의 형식이 이미 다 갖춰져 있습니다.**
  - 웹캠 창 이름이 "Human"입니다. AI가 아니라 사람이 친다는 점을 계속 드러냅니다.
  - vim 탭과 zsh 탭을 오가며 편집하고 컴파일합니다.
  - `cc`는 엄격한 경고 alias이고, 들여쓰기는 2칸, 함수는 `int main(void)`로 씁니다.
- 컴파일할 때 경고가 0개입니다. 무한 루프 뒤의 `return 0;`은 실행될 수 없는 코드인데도 지우지 않았습니다. 처음 쓴 Hello World 줄도 그대로 둔 채 그 아래에 코드를 덧붙였습니다. tree 영상의 "동작에 영향이 없으면 다듬지 않는다"와 같은 태도입니다.
