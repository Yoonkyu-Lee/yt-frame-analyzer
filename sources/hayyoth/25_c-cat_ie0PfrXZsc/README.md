# 해체 분석: "No AI. Build 'cat' command in C - but better for AI"

- 영상: https://youtu.be/_ie0PfrXZsc (27:32), 태그 `[cat]`
- 결과물: 복원한 코드 [src/main.c](src/main.c) (92줄), 검증 [verify/](verify/), 전사 [transcripts/](transcripts/) (11구간)
- 방법: 프레임 1,651장 → 웹캠을 가린 키프레임 530장 → 11구간 전사(코드 중심 형식). 오디오는 쓰지 않았습니다.
- **신뢰도: 마지막 저장 `"main.c" 92L, 1602B written`과 줄 수 일치, 바이트는 −7B.**
  - `main.c` 저장 메시지 9개를 모았습니다.
  - 33L 594B 시점을 같은 방식으로 복원하면 −6B입니다. 코드는 같고 빈 줄의 공백만 화면에 안 보이는 것으로 보입니다(01~16번과 같은 유형).
  - `is_ignore` 본문 5줄은 마지막 구간에서는 가려졌지만 21~24분 구간에서 그대로 보여 그것을 썼습니다.
  - 배너의 `=` 개수(왼쪽 11, 오른쪽 12)는 프레임을 확대해 글자 폭으로 셌습니다.

## 1. 동기 (스티커 메모, 0:00~2:06)

> **The AI mentioned here is merely a meme and one possible use case. How you choose to use this tool is entirely up to you. The main focus is learning about File I/O in C.**

> "sometime / if you have these code and want to send to AI , you do like this / flow / copy code -> enter file name -> paste"

자기 웹 편집기 프로젝트(`test-data`: `run.js`, `style.css`, `README.md` …)의 파일을 하나씩 Gemini에 붙여 넣는 과정을 직접 보여 줍니다(파일 이름 입력 → 붙여 넣기 반복). 그리고 진짜 `cat run.js style.css`의 한계를 보여 줍니다. 파일 이름이 없고, 마지막 줄에 개행이 없는 파일은 다음 파일의 첫 줄과 붙고(`copyUrl)#editor {`), 하위 폴더에 들어가지 못합니다.

## 2. 흐름

| 시각 | 한 일 | 저장 |
|---|---|---|
| 2:06~5:38 | 사용법 검사, `argv` 반복, `fopen`/`perror`/`continue`, `fgetc`/`putchar` 루프(`man 3 fgetc`). 주석 "read 1 byte at time is not good" | 25L 407B |
| 5:38~9:26 | `man 3 fread`를 읽고 1024바이트 버퍼로 `fread`/`fwrite`. 파일마다 머리줄 `=========== 경로 ============` | 31L 540B, 33L 594B |
| 9:26~12:06 | 셸 주석 `# Now it have file name , but not walk into sub folder` / `# build better cat V2`. 스티커에 설계(get path → 파일이면 cat → 폴더면 열어서 각 항목에 다시 walk cat). 반복문 본문을 잘라 `cat(path)`로 옮기고 `walkcat` 스텁 | |
| 12:06~19:04 | **자기 옛 영상(05번 Linux 유틸, 02번 tree)을 다시 틀어** `walkrm`의 `opendir`/`readdir`/`snprintf` 재귀를 보고 따라 씀. 스티커에 "check my previous video". `<sys/stat.h>` 빠진 오류 3개 | 49L 785B → 50L 825B |
| 19:04~24:08 | `ls -a`로 `.secret`, `.vscode`를 보고 숨김 파일을 건너뛰기로 함. `man strrchr` → 따로 `test.c`로 basename 추출을 시험(저장 4회) → `is_ignore`로 붙임 | 73L 1260B |
| 24:08~26:06 | 새 터미널에서 `./a.out test-data` → 전체 선택 → Gemini에 붙여 넣음. Gemini가 프로젝트를 정확히 요약. 셸 메모장에 사용 요령("open new terminal -> it will not have history / do cat / ctrl + A") | 91L 1590B |
| 26:06~27:30 | 출력에 `main.c: Not a directory`가 섞인 것을 보고 Gemini에 물어봄. **답을 읽기 전에** `cat(path);` 뒤에 `return;`을 넣어 고침. Gemini도 같은 원인을 짚음. 마지막 주석(저장 안 함): **`return; // i know it before gemini check - yeah // bye`** | **92L 1602B** |

## 3. 최종 코드의 구조

```c
void cat(const char *path);        // 머리줄 + fread/fwrite (1024B 버퍼)
int  is_ignore(const char *path);  // 마지막 '/' 뒤 이름이 '.'으로 시작하면 1
void walkcat(const char *path);    // lstat → 파일이면 cat, 폴더면 readdir로 재귀
int  main(int argc, char *argv[]); // 인자마다 walkcat
```

## 4. 검증 ([verify/edge_test.sh](verify/edge_test.sh), [결과](verify/edge_result.txt))

WSL gcc `-std=gnu11 -Wall -Wextra -pedantic -fsanitize=address,undefined`(누수 검사 포함)에서 경고, 메모리 오류, 누수 모두 0입니다. `lstat`, `DT_DIR`은 POSIX 확장이라 `-std=c11`로는 컴파일되지 않습니다. 작가의 `cc`는 macOS clang에 엄격한 경고 옵션을 붙인 별칭이고([24번 `.zshrc`](../24_w3m_6DQCtRSTfNw/)), 언어 표준을 지정하지 않아 이 확장들이 열려 있습니다.

| 시험 | 결과 |
|---|---|
| 개행 없이 끝나는 파일 + 다른 파일 | 머리줄과 빈 줄로 나뉘어 **더 이상 붙지 않음**(진짜 `cat`의 문제를 해결) |
| 폴더 재귀 | 하위 폴더까지 출력, `.env`와 `.git/`은 건너뜀 |
| 5000바이트 바이너리 | 바이트까지 같게 복사 |
| 없는 경로 | `nope: No such file or directory` |
| **`./a.out .`** | **아무것도 출력하지 않음.** `is_ignore`가 인자로 받은 경로에도 적용되어 `.`(현재 폴더)을 숨김 파일로 봅니다. `./`나 폴더 이름으로 주면 동작합니다. |
| 폴더를 가리키는 심볼릭 링크 | 머리줄만 출력하고 내용 없음(`lstat`으로 링크를 따라가지 않고, `fopen`은 폴더를 열지만 `fread`가 조용히 실패) |

출력 순서는 `readdir` 순서(정렬 안 함)입니다.

## 5. 해석

- **AI는 "도구의 사용자"로 나옵니다.** 이 `cat`은 AI에 코드를 넘기기 쉽게 만든 도구이고, 결과물을 Gemini가 읽습니다. 하지만 코드는 손으로 쓰고, AI에게 묻는 것은 마지막 버그 하나입니다. 그마저 답을 보기 전에 스스로 고친 뒤 `// i know it before gemini check`를 남깁니다. 첫 메모가 미리 말합니다: AI는 밈이자 쓰임새 하나일 뿐, 핵심은 C의 파일 입출력이다.
- **자기 옛 영상이 교재입니다(B4).** 재귀 디렉터리 순회는 05번(와 02번) 영상을 다시 틀어 보고 짭니다. 스티커에도 "check my previous video"를 적습니다.
- **모르는 함수는 따로 작은 파일에서 시험합니다(B3).** `strrchr`로 이름을 뽑는 방법을 `test.c` 12줄로 먼저 확인하고, 세 줄을 그대로 옮깁니다.
- **도구 이름처럼 단계도 V1 → V2입니다.** "파일 이름은 생겼지만 하위 폴더는 못 들어간다"를 셸 주석으로 적고 다음 버전을 시작합니다. 한계를 먼저 글로 적고 고치는 방식(D3, F1)입니다.
- 진짜 `cat`을 고치려는 것이 아니라 **자기 쓰임새에 맞는 다른 도구**를 만듭니다. 머리줄을 넣으면 진짜 `cat`처럼 파일을 이어 붙이는 용도로는 쓸 수 없습니다. 이름이 "better for AI"인 이유입니다.
