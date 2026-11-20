# 해체 분석: "No AI. Build ed (the standard text editor) in 100 lines of C"

- 영상: https://youtu.be/MKmPyByntzA (1:10:04, 2026-10-26 업로드), 태그 `[ed]`
- 결과물: [src/main.c](src/main.c) (113줄), 전사 [transcripts/](transcripts/) (16구간), 검증 [verify/](verify/)
- 방법: 1fps 프레임 4,204장 → 웹캠을 가린 변화 감지로 키프레임 643장 → 16구간 Sonnet 전사. **오디오는 쓰지 않았습니다**(오디오 트랙은 있으나 자막 없음).

## 1. 줄거리: AI가 쓴 549줄을 보여 주고, 113줄로 다시 짠다

| 시각 | 한 일 |
|---|---|
| 0:00~1:29 | 메모 파일(`notes.txt`, ed 명령 목록과 "Evolution tree: ed -> sed -> ex -> vi -> vim")에 `- AI love ed` 한 줄을 더합니다. 스티커 메모 "why", "i did it". ChatGPT 대화로 만든 `ed.c`를 `ai.c`로 엽니다: `"ai.c" 549L, 10452B` |
| 1:29~5:09 | `ai.c`를 훑어 내리고 스티커 **"this is chatgpt code / 550 line / too complicated"**. 진짜 `ed`로 `a`, `.`, `w hello.txt`(→ `30`), `q`, 다시 열어 `1`/`2`/`p`/`d`를 시연하고, 명령마다 스티커로 이름을 붙입니다. 빈 `main.c`를 엽니다 |
| 5:09~12:27 | 뼈대: `while (1)` 명령 루프 → `fgets`(man 3 fgets, Beej 23.15) → `strcspn`으로 줄바꿈 제거 → `strcmp`로 `q` → `mode(cmd, "q")` 도우미 → `else if` 명령 분기 |
| 12:27~24:32 | `a`(입력 모드 `on_edit`, `.`로 끝), `lines[]` 버퍼와 `strdup`(man 3 strdup), `p`. 컴파일 오류 두 개(`MAX_LINEs` 오타, `char lines[]` → `char *lines[]`)를 고치고, 진짜 `ed`를 옆 터미널에 띄워 비교합니다 |
| 24:32~34:14 | 줄 번호 이동: Beej의 `strtol`/`strtod`와 `endptr` 예제를 읽고 `strtol(cmd, &endptr, 10)`. `3`을 쳤는데 `?`가 나오자 `<`를 `<=`로 고칩니다 |
| 34:14~40:27 | `./a.out 파일`로 열기: `fopen` 실패 메시지는 진짜 `ed`의 문구(`No such file or directory`)를 그대로 가져옵니다. 줄 끝 개행 버그를 `strcspn`으로 고칩니다 |
| 40:27~67:22 | `w` 저장: `strncmp(cmd, "w", 1)`, 공백 건너뛰기, 파일 이름 기억(`char filename[255] = {0}; // C add trash value to it`). 컴파일러 경고를 하나씩 고칩니다(`sizeof(path)` → `sizeof(filename)`, `%zu`, `fprintf` 인자 순서, `strncmp`에 빠진 `== 0`). 그다음 `d` 삭제: 옆 vim 버퍼에 "move from 2 to 3 / lines[i] = lines[i + 1]"을 먼저 그려 보고 짭니다 |
| 67:22~70:03 | 지운 뒤 마지막 줄이 두 번 저장되는 버그를 보고 `count_line--;`를 더합니다. 최종 저장 `"main.c" 113L, 2609B written`. 마지막 스티커는 "you implement your own…"(잘림) |

## 2. 복원과 검증

**최종 코드**: 6~113줄은 마지막 구간에서 그대로 읽었고, 화면 위로 스크롤된 1~5줄(include 3개와 `#define MAX_LINES 1024`)은 34분대 구간에서 가져왔습니다.

| 확인 | 결과 |
|---|---|
| vim 저장 기록(체크섬) | 최종 `113L, 2609B` → 복원 **113줄 일치**, 2597바이트(−12B). 지금까지 영상들과 같은 패턴인 빈 줄의 들여쓰기 공백으로 보입니다(화면에 보이지 않음) |
| 저장 기록 전체 | 29회 기록을 시간순으로 모았습니다: 212B(7:50) → 700B → 1331B → 1721B → 2055B → 2165B → 2387B → 2585B → **2609B**(68:49). 코드가 자라는 모양이 전사와 맞습니다 |
| 컴파일 | `gcc -Wall -Wextra -pedantic`: 경고 0 |
| 영상 속 마지막 세션 재현 | `a`, 4줄, `.`, `2`, `d`, `w line.txt`, `q` → `line 2` 출력, 파일은 `line 1 / line 3 / line 4`로 **영상과 같음** |

**경계 상황** ([verify/edge_test.sh](verify/edge_test.sh), 결과 [edge_test_result.txt](verify/edge_test_result.txt), ASan/UBSan):

| 상황 | 진짜 `ed` | 이 코드 |
|---|---|---|
| 마지막 줄을 지운 뒤 `p` | 새 마지막 줄 출력 | **해제된 메모리 읽기** (heap-use-after-free). `current_line`이 줄 수보다 커진 채 남음 |
| 1번 줄로 간 뒤 `a` | 1번 뒤에 끼워 넣음 | **2번 줄을 덮어씀**: 줄 하나가 사라지고, 저장 파일 끝에 `(null)` |
| 종료 | — | 마지막 줄을 해제하지 않음 (`for (i = 1; i < count_line; i++)`) |
| 1023자보다 긴 줄 | 그대로 | 여러 줄로 쪼개짐 (`MAX_LINE_LEN` 가정) |
| 이름 없이 `w` | `?` | `?` (맞음) |

모두 영상의 시연 범위(위에서 아래로 쓰고, 하나 지우고, 저장) 밖에서만 드러납니다. 작가는 시연에서 마주친 버그(`<=`, 개행, 중복 저장)만 고쳤습니다.

## 3. 해석

- **AI 코드를 출발점으로 보여 주고, 그 크기를 이유로 거절합니다.** 09(해시 테이블)에서는 ChatGPT 코드를 교재로 읽고 다시 짰습니다. 여기서는 549줄을 열어 보이기만 하고 "too complicated"라는 판단 하나로 치웁니다. 그리고 제목대로 "100 lines"(실제로는 113줄)를 손으로 씁니다. AI 코드는 **비교 대상이자 반면교사**로만 쓰입니다(PHILOSOPHY E1).
- **진짜 프로그램이 명세입니다.** 짜기 전에 진짜 `ed`를 시연하고, 짜는 중에도 옆 터미널에서 같은 입력을 넣어 비교합니다. 오류 문구까지 진짜 `ed`에서 가져옵니다(A1).
- **1차 자료를 읽고 씁니다.** `man 3 fgets / strcspn / strcmp / strdup / fopen / strncmp / strcpy`, Beej의 C 라이브러리 레퍼런스(strtol, strncpy). 문서에서 읽은 함수만 씁니다(C1).
- **경고가 진도를 정합니다.** 저장 → `cc main.c` → 경고 → 고침의 반복이 영상 후반의 대부분입니다(C3).
- **삭제는 그림으로 먼저.** `d`를 짜기 전에 빈 vim 버퍼에 줄 번호와 화살표를 적어 옮기는 순서를 그립니다. 코드 대신 자료의 모양을 먼저 봅니다.
- **알려진 한계는 남깁니다.** 끼워 넣기 대신 덮어쓰기, 마지막 줄 삭제 뒤의 위치는 시연 범위 밖이라 그대로입니다(D3). 마지막 스티커는 시청자에게 "직접 구현해 보라"고 넘깁니다(F2).
