# 해체 분석: "No AI, 52 min. Build all Linux Files utils in C by Hand"

- 원본: https://youtu.be/gQRmSlcTq9c (Hayyoth, 52:27, 무음 화면 녹화)
- 철학 정리: [../PHILOSOPHY.md](../PHILOSOPHY.md) (이 영상의 근거는 `[utils]` 태그)

| 경로 | 내용 |
|---|---|
| [src/notes.txt](src/notes.txt) | 저자가 미리 써 둔 **설계 표**(명령, 쓸 함수, 알고리즘)와 Helper 메모. 칸 간격은 근사치 |
| [src/touch.c](src/touch.c), [cat.c](src/cat.c), [mkdir.c](src/mkdir.c), [mv.c](src/mv.c), [rm.c](src/rm.c), [find.c](src/find.c) | 유틸리티 6개. 각각 단일 파일이며 `cc x.c -o x`로 빌드 |
| [history/](history/) | rm.c 중간 버전 |
| [verify/](verify/) | 판독 원문에서 파일을 추출·조립하고 체크섬과 대조하는 스크립트 |
| [transcripts/](transcripts/) | 구간별 프레임 판독 원문 14개 |

---

## 1. 복원 신뢰도

| 파일 | 화면 체크섬 | 복원본 | 결과 |
|---|---|---|---|
| touch.c | `20L, 326B` (01:56) | 20L, 326B | ✅ 완전 일치 |
| cat.c | `25L, 395B` (03:04) | 25L, 391B | 줄 수 일치, −4B |
| mkdir.c | `19L, 313B` (04:33) | 19L, 313B | ✅ 완전 일치 |
| mv.c | `67L, 1307B` (22:58) | 67L, 1304B | 줄 수 일치, −3B |
| rm.c v1 | `81L, 1562B` (41:56) | 81L, 1555B | 줄 수 일치, −7B |
| rm.c (src) | `82L, 1592B` (43:28) | 82L, 1585B | 줄 수 일치. v1→v2의 +30B는 정확히 일치 |
| find.c | 편집 후 저장 메시지가 보이지 않음 | 71L, 1347B | 16행, 44행, `main`은 원본 프레임으로 직접 확인 |
| notes.txt | `43L, 1893B` | 32L | 표와 Helper만 보임. 칸 간격은 근사치 |

**남은 몇 바이트는 저자의 vim 습관 때문으로 보입니다.**
- cat.c는 touch.c(완전 일치)를 복사해 만들었고, 추가된 줄은 모두 `for` 블록 안에 있습니다. 이 블록의 들여쓰기는 4칸이고, 차이도 정확히 −4B입니다.
- tree 영상(6칸 블록, −6B)과 03 영상에서도 같은 현상이 나왔습니다. **블록 안의 빈 줄이 들여쓰기 공백을 그대로 품고 있는 것**으로 추정합니다.
- 공백은 화면에 보이지 않으므로 어느 줄인지는 확정할 수 없습니다. 동작에는 영향이 없습니다.

**실행으로 검증한 것** (WSL, gcc 13, `-Wall -Wextra -pedantic -Wno-unused-parameter`)
- 6개 모두 경고 없이 빌드됩니다.
- 영상과 같은 시나리오를 재현했습니다:
  - `touch abc.txt def.txt xyz.txt`
  - `cat cat.c`
  - `mkdir abc`, `mkdir abc/def xyz`
  - `mv xyz.txt my-xyz.txt`(이름 변경), `mv my-xyz.txt abc`(디렉터리로 이동)
  - `find cat abc` → `abc/cat.c`
  - `rm abc`(재귀 삭제)
- 영상에 나온 **rm 버그도 재현됩니다**. 일반 파일에 `./rm`을 쓰면 파일은 지워지지만 이어서 `opendir`에 실패하고 `<name>: No such file or directory`를 출력하며 종료 코드 1로 끝납니다.

## 2. 타임라인

| 시각 | 유틸 | 내용 |
|---|---|---|
| 00:00 | (설계) | **설계 표 `notes.txt`(43줄)가 처음부터 열려 있습니다.** 행마다 명령, 쓸 시스템 함수, 알고리즘이 적혀 있습니다. 작업하는 동안 현재 행을 VISUAL 모드로 강조합니다. `tree` 행에는 "watch my previous video"라고 적혀 있습니다(02번 영상을 가리킴). |
| 00:00–01:56 | touch | 6줄짜리 템플릿에서 시작합니다(사용법 출력, argv 루프). `man fopen`을 읽고 `fopen(path, "w")` → `fclose`로 구현합니다. |
| 02:00–03:04 | cat | touch.c 전체(20줄)를 복사해 새 파일로 붙여 넣습니다. `"r"`로 바꾸고 `while ((c = fgetc(fp)) != EOF) putchar(c);`를 추가합니다. `./cat cat.c`가 자기 소스를 출력합니다. |
| 03:09–04:33 | mkdir | cat 코드에서 11줄을 지우고 `mkdir(path, 0777)`을 씁니다. 그 전에 **`man 2 mkdir` 페이지를 끝까지 읽습니다**. |
| 04:40–23:03 | mv (약 18분) | 빈 vim 버퍼에 사용 예를 먼저 적습니다("mv file1.txt file2.txt --> it is rename", "use 2 -> move to folder"). 이후 여러 번 막히고 고칩니다(아래 표 참고). |
| 23:28–43:28 | rm (약 20분) | cat.c를 뼈대로 시작합니다. 메모에 "rm abc --> if is folder , we must remove all file child then remove folder abc"라고 적은 뒤 그 문장을 **그대로 코드 주석으로 옮깁니다.** `lstat`/`unlink` → `opendir`/`readdir` → 경로 결합 → 재귀 → `rmdir` 순으로 만듭니다. 경로 결합 코드는 `:e mv.c`로 자기 mv.c에서 복사해 옵니다. |
| 44:00– | (버그) | 테스트 중에 `./rm rm`, `./rm mkdir`을 실행해 자기가 만든 바이너리를 지웁니다. 일반 파일 경로의 버그도 이때 드러납니다. 셸에 `# use fix this bug`라고 남기지만 **고치지 않고 다음 유틸로 넘어갑니다.** |
| 43:28–51:40 | find | rm.c 전체(82줄)를 복사하고 `unlink` 부분을 지운 뒤 `strstr`로 이름을 비교합니다(`man 3 strstr`). 디버그 출력 `"on check %s %s\n"`를 쓰다가 마지막에 `"%s\n"`로 줄입니다. |
| 51:40–52:20 | (실사용) | `./find main.c ~/sources`(10건), `./find READ ~/sources`(15건). 실제 소스 트리에서 확인합니다. |
| 52:20 | — | **`## ok all work , good luck`** (원본 프레임으로 확인) |

### mv에서 겪은 문제 (약 18분)

| 문제 | 해결 |
|---|---|
| `errno`, `ENOENT` 미선언 컴파일 오류 | `#include <errno.h>` |
| `rename` 인자 순서가 헷갈림 | `man rename` |
| `strchr(path, "/")`에 문자열 리터럴을 넘김 | `man strchr`를 읽고 `strrchr(path, '/')`로 변경(마지막 슬래시를 찾아야 함) |
| `snprintf` 관련 `-Wint-conversion` 오류 | `man 3 snprintf`를 읽고 `path_len` 변수를 따로 둠 |
| 경로에 슬래시가 없으면 이름의 첫 글자가 잘림(`y-xyz.txt`) | `if (!name) name = path; else name++;` |

### rm에서 겪은 문제

| 문제 | 해결 |
|---|---|
| `man 2 opendir` → "No manual entry" | `man opendir`(3장)의 EXAMPLES 루프를 자기 변수명으로 고쳐 씀 |
| `perror(dir)`에 `DIR *`를 넘겨 경고 발생 | `perror(path)` |
| `<string.h>` 누락, `strcmp` 오용 | 컴파일 오류를 보고 수정 |
| `man rmdir`이 1장 명령 문서를 띄움 | `man 3 rmdir`(없음) → `man 2 rmdir` |

## 3. 설계 해설

**모든 유틸이 같은 뼈대를 씁니다.**
```c
int main(int argc, char *argv[]) {
  if (argc == 1) { printf("Useage: X [path]...\n"); return 0; }
  for (int i = 1; i < argc; i++) {
    const char *path = argv[i];
    /* 동작 */  — 실패하면 perror(path); return 1;
  }
  return 0;
}
```
- 새 유틸은 **직전 파일을 복사해서 고치는 방식**으로 만듭니다: touch → cat → mkdir → mv, cat → rm → find.
- "Useage" 오타가 모든 파일에 그대로 퍼져 있습니다. 복사로 만들었다는 흔적입니다.

**재귀 탐색은 tree 영상의 `walk`를 그대로 다시 씁니다.** `walkrm`과 `walkfind`의 구조는 같습니다.
- `lstat` → 일반 파일이면 처리
- `opendir`/`readdir` → `.`과 `..`는 건너뜀
- VLA 버퍼에 `snprintf("%s/%s")`로 하위 경로를 만듦
- 하위 항목도 `lstat` → 디렉터리면 재귀, 아니면 처리
- 심볼릭 링크를 따라가지 않도록 `stat`이 아니라 `lstat`을 씁니다. notes.txt의 Helper에 "lstat: check path, if file is link, return link"라고 미리 적혀 있습니다.

**설계 표가 곧 작업 목록입니다.**
- "Function use" 열의 함수가 곧 그 유틸의 `#include`와 man 페이지 목록입니다.
- "Algorithm" 열의 문장(예: "walk path, if file remove / if dir, open child, if dir walk path")이 코드 구조와 일대일로 대응합니다.

## 4. 남은 결함 (직접 실행해 확인)

| 결함 | 원인 |
|---|---|
| `rm <일반 파일>`이 파일을 지운 뒤 오류를 내고 종료 코드 1 | 최상위의 `unlink` 뒤에 `return`이 없어 `opendir`로 넘어감. 영상에서 `# use fix this bug`라고 남겼지만 고치지 않음 |
| `rm`에서 `closedir` 누락 | 편집 중에 지움. 재귀 깊이만큼 디렉터리 핸들이 샘 |
| `rm`, `find`의 재귀 호출 반환값을 버림 | 하위 디렉터리에서 실패해도 계속 진행 |
| `cat`의 `char c` | `fgetc`는 `int`를 반환하므로 0xFF 바이트를 EOF로 오인할 수 있음(고전적인 버그) |
| `rm.c`에 디버그 `printf("on this case\n")`가 남아 있음 | 마지막 저장본 그대로 |
| "Useage", "patter", "not exit" 오타 | 복사로 전파됨 |

## 5. 해석

- **52분 동안 6개를 만들었는데, 시간은 대부분 mv(18분)와 rm(20분)에 들었습니다.** 둘 다 "대상이 디렉터리인가"에 따라 분기해야 하는 유틸입니다. touch, cat, mkdir은 각각 2~3분이면 끝났습니다.
- **글 → 표 → 코드 순서로 진행합니다.** 각 단계를 바로 아래 단계로 옮겨 적습니다.
  - 설계 표의 "Algorithm" 칸에 짧게 씁니다.
  - 빈 버퍼에 사용 예를 영어 문장으로 풉니다.
  - 그 문장을 코드 주석으로 옮기고, 그 아래에 코드를 씁니다.
- **아는 것은 복사하고, 모르는 것은 man 페이지에서 찾습니다.**
  - 모든 시스템 함수(`fopen`, `mkdir`, `rename`, `unlink`, `opendir`/`readdir`, `rmdir`, `strchr`, `snprintf`, `strstr`)를 man 페이지로 확인했습니다.
  - 장(section) 번호가 틀리면 다른 장을 다시 찾아봅니다.
  - 이미 자기 코드에 있는 것(경로 결합, `walk` 구조)은 자기 파일에서 복사해 옵니다.
- **버그를 알아도 범위를 지킵니다.** 자기 바이너리를 지워 버린 rm 버그를 발견하고 메모만 남긴 채 find로 넘어갔습니다. "all work"는 영상의 시연 범위 안에서 다 동작한다는 뜻입니다.
