# 해체 분석: "No AI. Build your own tree directory viewer in C by hand"

- 원본: https://youtu.be/tiMHtc6l7xM (Hayyoth, 2026-10-16, 19:55, 무음 화면 녹화)
- 분석 방법: 대본·오디오 없이 **프레임 판독만으로** 복원
- 철학 정리: [../PHILOSOPHY.md](../PHILOSOPHY.md) (영상 공통 문서. 이 영상의 근거는 `[tree]` 태그)

| 경로 | 내용 |
|---|---|
| [src/main.c](src/main.c) | 최종 코드 (빌드: `cc main.c`) |
| [src/README.md](src/README.md) | 저자가 코드보다 먼저 쓴 설계 노트 |
| [history/](history/) | vim 저장 시점별 9개 버전 (v1~v9) |
| [transcripts/](transcripts/) | 구간별 프레임 판독 원문 8개 (근거 자료) |
| [verify/build_history.py](verify/build_history.py) | 체크섬 대조 스크립트: `python verify/build_history.py .` |

---

## 1. 복원 신뢰도

vim은 저장할 때마다 `"main.c" 58L, 1119B written`처럼 줄 수와 바이트 수를 출력합니다. 화면에 찍힌 9개 저장 메시지를 **체크섬**으로 사용해 복원본을 검증했습니다.

| 버전 | 시점 | 내용 | 줄 수 (복원/화면) | 바이트 (복원/화면) |
|---|---|---|---|---|
| [v1](history/v1_0253_args.c) | 02:53 | argv 처리 | 14/14 ✅ | 200/200 ✅ |
| [v2](history/v2_0629_readdir.c) | 06:29 | opendir/readdir 루프 | 31/31 ✅ | 488/488 ✅ |
| [v3](history/v3_1359_ansi.c) | 13:59 | DT_DIR 분기 + 색상 | 47/47 ✅ | 806/807 (−1) |
| [v4](history/v4_1415_string_h.c) | 14:15 | `<string.h>` 추가 | 48/48 ✅ | 826/827 (−1) |
| [v5](history/v5_1640_recursion.c) | 16:40 | 재귀 | 55/55 ✅ | 1006/1013 (−7) |
| [v6](history/v6_1803_indent_bug.c) | 18:03 | `%.*c` (경고 발생) | 56/56 ✅ | 1059/1066 (−7) |
| [v7](history/v7_1813_indent_fix.c) | 18:13 | `%*c` 수정 | 56/56 ✅ | 1058/1065 (−7) |
| [v8](history/v8_1909_indent_param.c) | 19:09 | indent 매개변수 | 56/56 ✅ | 1095/1102 (−7) |
| [v9](history/v9_1929_final.c) | 19:29 | 최종 | 58/58 ✅ | 1112/1119 (−7) |

**해석**
- 9개 버전 모두 줄 수가 일치합니다. v1과 v2는 바이트까지 일치합니다.
- 버전 사이의 바이트 증감은 v4 이후 모든 구간에서 화면과 정확히 같습니다. 예를 들어 v7→v8에서 +37B는 `, int indent`, `3`→`indent` 두 번, `, indent + 1`, `, 0`을 합한 값입니다. 따라서 **보이는 코드는 바이트 단위로 정확합니다.**
- 남은 7B는 화면에 나타나지 않는 공백 문자입니다.
  - 1B는 06:29~13:59 사이에 생겼습니다. 어딘가의 후행 공백으로 추정합니다. 9:12에 `{`를 지운 곳은 프레임으로 확인해 후보에서 제외했습니다.
  - 6B는 14:15~16:40의 재귀 블록 작성 중에 생겼습니다. 이 블록의 들여쓰기가 정확히 6칸이므로, 빈 줄 하나에 autoindent 공백 6칸이 남은 것으로 추정합니다. 14:31에 vim이 주석 기호 `// `를 자동 삽입했고, 저자가 그것을 지운 흔적이 있습니다.
  - 공백 문자는 프레임에 보이지 않으므로 위치를 확정할 수 없습니다. **동작과 컴파일 결과에는 영향이 없습니다.**

**실행으로 검증한 것** (WSL Ubuntu 24.04, gcc 13)
- 저자의 컴파일 플래그 `-Wall -Wextra -pedantic -Wno-unused-parameter`로 경고 0개입니다. 이 플래그는 17:39 탭 제목에서 확인했습니다.
- v6은 영상과 같은 `%.*c` 형식 경고를 재현합니다.
- v8은 19:16 실행에서 보인 "최상위 항목 앞 공백 1칸" 버그를 똑같이 재현합니다.
- v3에서 `<string.h>` 없이 `strcmp`를 쓴 진단도 재현됩니다.

---

## 2. 최종 산출물

```c
#include <stdio.h>
#include <dirent.h>
#include <string.h>

void print_file(const char *name, int is_dir, int indent) {
  if (indent)
    printf("%*c", indent * 2, ' ');

  if (is_dir)
    printf("\033[0;34m%s\033[0m\n", name);
  else
    printf("%s\n", name);
}

int walk(const char *path, int indent) {
  DIR *dir = opendir(path);
  if (!dir) {
    perror("opendir");
    return 1;
  }

  struct dirent *dp;
  while ((dp = readdir(dir)) != NULL) {
    const char *name = dp->d_name;

    // ignore files
    if (strcmp(name, ".") == 0 || strcmp(name, "..") == 0)
      continue;

    if (dp->d_type == DT_DIR ) {
      print_file(name, 1, indent);

      // walk to sub path
      size_t len = strlen(path) + strlen(name) + 2;
      char sub_path[len];
      snprintf(sub_path, len, "%s/%s", path, name);

      walk(sub_path, indent + 1);
    } else {
      print_file(name, 0, indent);
    }
  }

  closedir(dir);
  return 0;
}

int main(int argc, char **argv) {
  const char *path = "."; // default path is current dir

  if (argc > 1) {
    path = argv[1];
  }

  int err = walk(path, 0);

  return err;
}
```

- 58줄, 헤더 3개, 함수 3개입니다. `malloc`, `struct` 정의, `#define`, Makefile은 하나도 없습니다.
- 빌드는 `cc main.c`, 실행은 `./a.out [path]`입니다.
- 출력 형식: 한 단계마다 공백 2칸씩 들여쓰고, 디렉터리는 파란색(`\033[0;34m`)으로 표시합니다. 정렬하지 않고 readdir 순서 그대로 출력합니다. 가지 기호(`├──`)는 쓰지 않습니다.

---

## 3. 타임라인: 20분 동안 무엇을 어떤 순서로 했나

| 시각 | 단계 | 내용 |
|---|---|---|
| 00:00–00:40 | **목표 확인** | 진짜 `tree`를 `~/sources`에 실행합니다(83 directories, 323 files). 만들 대상의 동작을 먼저 눈으로 확인합니다. |
| 00:40–01:11 | **설계** | 코드보다 `README.md`를 먼저 씁니다. `## Design` 아래 의사코드 4줄을 적습니다. |
| 01:11–02:53 | 뼈대 | `main`, 기본 경로 `"."`, `argc > 1`이면 argv를 받습니다. 디버그용 `printf("path %s\n", path)`를 둡니다. |
| 02:42–02:50 | **첫 버그** | 처음에 `argv[0]`을 썼습니다. `./a.out abc`가 `path ./a.out`을 출력하는 것을 보고 실행으로 버그를 잡고 `argv[1]`로 고칩니다. |
| 02:53–03:35 | 인터페이스 | `int walk(const char *path)` 스텁을 만들고, `int err = walk(path); return err;`로 오류 코드를 종료 코드까지 전달합니다. |
| 03:35–06:35 | 핵심 I/O | POSIX 명세의 opendir/readdir 예제를 읽으며 직접 타이핑합니다. 예제의 `exit(1)` 대신 `perror` + `return 1`을 씁니다. 실행해 보면 `.`, `..`, `.main.c.swp`까지 출력됩니다. |
| 06:35–07:40 | 조사 | POSIX `<dirent.h>`에서 `d_type` 필드와 `DT_DIR` 상수를 찾습니다. |
| 07:40–09:19 | 분기 | `if (DT_DIR) { // is dir } else { // is file }`처럼 **주석으로 스텁**을 먼저 만듭니다. `man 3 strcmp`를 확인한 뒤 `.`/`..`를 건너뛰게 합니다. 처음엔 중괄호로 썼다가 중괄호 없는 `continue;`로 바꿉니다. |
| 09:40–12:55 | 색상 | `print_file` 스텁을 만들고 ANSI 표를 조사합니다(gist). `#define BLUE`를 시도했다가 **undo로 폐기**합니다. `"%s%s%s"` 조합도 시도한 뒤, 결국 포맷 문자열에 이스케이프를 직접 넣습니다. |
| 12:43 | **컴파일 실패** | `\e`가 `-pedantic` 경고를 2개 내고, `strcmp` 암시적 선언 오류가 납니다. |
| 12:55–14:15 | 경고 대응 | Stack Overflow와 Medium을 보고 비표준 `\e`를 표준 `\033`으로 바꿉니다. 굵은 파랑 `1;34`도 일반 파랑 `0;34`로 바꿉니다. `<string.h>`를 추가합니다. |
| 14:25–16:40 | **재귀** | 주석 `// walk to sub path`를 먼저 씁니다. 이어서 `size_t len`, VLA `char sub_path[len]`을 만들고, `man snprintf`를 확인한 뒤 `snprintf`로 경로를 조립합니다. `abc/def.txt`로 테스트하니 출력이 평평합니다. 진짜 `tree`와 비교합니다. |
| 16:46–18:13 | 들여쓰기 | `print_file`에 `indent`를 추가합니다. `%.*c`가 `-Wformat` 경고를 내자 "c printf indent"를 검색하고, SO 67089739를 읽은 뒤 `%*c`로 고칩니다. **먼저 `3`을 하드코딩해 검증**합니다. |
| 18:28–19:09 | 매개변수화 | `walk(path, indent)`, 재귀 호출은 `indent + 1`, `main`에서는 `walk(path, 0)`으로 바꿉니다. |
| 19:16 | **둘째 버그** | 최상위 항목 앞에 공백 1칸이 붙습니다. `%*c`는 폭이 0이어도 문자 1개를 출력하기 때문입니다. |
| 19:20–19:29 | 수정 | `if (indent)` 가드를 추가합니다. 최종 저장(58L, 1119B). |
| 19:41–19:54 | 실사용 | 실제 Go 저장소(`.git` 포함)에 실행해 봅니다. 셸에 `# Bravo`라고 입력하고 끝납니다. |

---

## 4. 철학

저자의 작업 원칙은 여러 영상을 비교하며 쌓을 수 있도록 [../PHILOSOPHY.md](../PHILOSOPHY.md)에 따로 정리했습니다. 이 영상에서 나온 근거에는 `[tree]` 태그와 타임스탬프를 붙였습니다.

---

## 5. 코드 레벨 심층 해설

- **`size_t len = strlen(path) + strlen(name) + 2;`**: +2는 `/` 1바이트와 NUL 1바이트입니다. 남는 바이트 없이 정확한 크기입니다.
- **`char sub_path[len];` (VLA)**: `malloc`/`free`가 필요 없습니다. 재귀가 깊어질수록 스택을 쓰지만, 300단계 깊이로 시험해도 문제없었습니다. VLA는 C11부터 선택 기능이고, `-pedantic`에서도 경고가 나지 않습니다.
- **`printf("%*c", indent * 2, ' ')`**: `*`는 필드 폭을 인자로 받습니다. 그래서 공백 문자 하나를 폭 `2*indent`로 오른쪽 정렬하면 공백이 `2*indent`개 출력됩니다. 다만 폭이 0이어도 문자 1개는 반드시 출력되므로 `if (indent)` 가드가 필요했습니다. SO 답변의 원래 관용구인 `printf("%*s", indent * 2, "")`를 썼다면 가드가 필요 없었습니다.
- **`%.*c`가 틀린 이유**: `.*`는 정밀도(precision)인데, `%c`에 정밀도를 주는 것은 정의되지 않은 동작입니다. clang이 이를 정확히 지적했습니다.
- **`\e` → `\033`**: `\e`는 GNU 확장이라 ISO C 표준이 아닙니다. `\033`(8진수 27, ESC)이 표준 표기입니다.
- **`d_type`**: readdir 결과만으로 파일 종류를 알 수 있어 `stat` 호출이 필요 없습니다. 이전에는 BSD/Linux 확장이었고, POSIX.1-2024(Issue 8)에서 표준이 됐습니다. 저자도 Issue 8 명세를 참조했습니다. 단, glibc에서 `-std=c99`처럼 엄격한 모드로 컴파일하면 `DT_DIR`이 선언되지 않습니다. 직접 확인했으며, 기본 `gnu17` 모드에서는 문제없습니다.

---

## 6. 한계와 개선 여지 (직접 실행해 확인)

| 현상 | 확인 결과 | 원인 |
|---|---|---|
| 하위 디렉터리를 읽지 못해도 종료 코드가 0 | ✅ 확인 | `walk(sub_path, ...)`의 반환값을 버림 |
| 오류 메시지에 경로가 없음 | `opendir: Permission denied`만 출력 | `perror("opendir")` |
| 파이프로 넘겨도 색상 코드가 출력됨 | `^[[0;34m`이 그대로 섞임 | `isatty()` 검사 없음 |
| 디렉터리를 가리키는 심볼릭 링크는 따라가지 않음 | `link`가 파일처럼 출력됨 | `DT_LNK` 처리 없음. 대신 순환 링크에 안전함 |
| `DT_UNKNOWN`을 반환하는 파일시스템 | 디렉터리를 파일로 출력함 | `lstat` 대체 경로 없음 (WSL의 /mnt/c는 정상) |
| 정렬, 숨김 파일 필터, 가지 기호, 개수 요약 없음 | — | 의도적인 최소 구현 |

---

## 7. 이 영상으로 연습하는 방법

1. [history/](history/)의 v1~v9를 순서대로 직접 타이핑해 보세요. 체크섬 표의 줄 수가 맞는지 확인하면 저자의 진행 순서를 그대로 따라갈 수 있습니다.
2. 6장의 표를 하나씩 고쳐 보세요. 재귀 오류 전파, `perror`에 경로 넣기, `isatty`, `%*s`로 바꿔 가드 제거하기 등입니다. 저자의 원칙대로 한 번에 하나씩 고치고, 고칠 때마다 컴파일하고 실행하세요.
3. 가지 기호(`├──`, `└──`)를 넣으려면 "형제 중 마지막인지" 알아야 합니다. 그 정보가 필요해지는 순간, 미뤄 뒀던 추상화(배열에 모으기, 정렬)가 언제 정당해지는지 체감할 수 있습니다.

---

## 부록: 재현 방법

```sh
# 1~6단계: 메타데이터, 1440p 다운로드, 1fps 프레임, 변화 감지, 키프레임 315장, 8구간 분할
python tools/prepare.py https://youtu.be/tiMHtc6l7xM --mask 0.69,0.26,0.985,0.86
```

- `--mask`는 웹캠 창 영역입니다(정규화 좌표). 이 영역을 가려야 키프레임이 의미 있는 변화만 담습니다.
- 판독: [tools/prompts/transcribe_code.md](../../../tools/prompts/transcribe_code.md) 프롬프트(당시 이름 transcribe_segment.md)로 구간마다 Sonnet 에이전트를 1개씩 띄웠습니다. 결과물이 [transcripts/](transcripts/)입니다.
- 통합과 검증:
  - 핵심 프레임은 원본 해상도로 직접 재확인했습니다(19:29 최종 화면, 9:12 중괄호 삭제, 17:39 탭 제목).
  - vim 저장 메시지를 체크섬으로 썼습니다: `python verify/build_history.py .`
  - WSL(gcc 13)에서 컴파일하고 실행해 경고와 버그가 재현되는지 확인했습니다.
