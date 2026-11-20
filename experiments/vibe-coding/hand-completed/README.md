# hand-completed: 사람 코드의 남은 일을 같은 기조로 마무리한 버전

[sources/hayyoth/](../../../sources/hayyoth/)에서 복원한 코드에는 영상 시연 범위 밖으로 미룬 일과 버그가 남아 있었습니다. 이 폴더는 그것을 **저자의 접근 방식 그대로** 마무리한 버전입니다. Codex 결과물([../](../README.md))과 같은 조건에서 다시 비교하기 위해 만들었습니다.

원본은 바꾸지 않았습니다. 폴더마다 `src/`가 완결판이고, `changes.diff`가 원본과의 차이입니다.

## 지킨 원칙

**유지한 스타일**
- 2칸 들여쓰기, K&R 중괄호, 문장 하나짜리 본문은 중괄호 생략
- 오류는 `perror(path); return 1;`, 주석은 저자 말투의 짧은 소문자 영어
- 파일 하나로 완결. 새 헤더, 공통 라이브러리, 추상화 계층을 만들지 않음

**설계로 보고 유지한 것** (버그가 아님)

| 대상 | 유지한 설계 |
|---|---|
| 05 | 저자가 정한 CLI(`find PATTERN PATH`), `rm`은 기본이 재귀 삭제(설계 메모 "if dir, open child, if dir walk path") |
| 07 | `append`는 O(n) 순회(tail 포인터 없음) |
| 09 | 고정 `SIZE 5`, 글자 코드 합 해시(애너그램 충돌은 저자가 이미 메모함), 키 문자열은 복사하지 않음(호출자가 소유) |
| 02 | 정렬하지 않고 readdir 순서 그대로 즉시 출력, 숨김 파일도 출력 |

**고친 것**
- 크래시와 UB, 메모리 누수, 틀린 결과
- 저자 자신의 설계 메모와 다른 동작
- 저자가 남긴 TODO: `// add free`, `// add err handle`, `# use fix this bug`

## 과제별 수정 내역

| 과제 | 원본의 문제 | 수정 | 줄 수 |
|---|---|---|---|
| 02 tree | 하위 폴더 오류를 무시하고 종료 코드 0. `perror("opendir")`가 경로를 알려 주지 않음. 파이프로 넘겨도 색 코드가 섞임. `d_type`을 채우지 않는 파일시스템에서는 폴더를 파일로 봄 | 재귀 결과를 `err`로 모아 반환. `perror(path)`. `isatty`일 때만 색 사용. `DT_UNKNOWN`이면 `lstat`으로 확인 | 43 → 50 |
| 03 움직이는 숫자 | 프레임마다 `system("clear")`로 프로세스를 새로 띄움(glibc `-Wunused-result` 대상) | ANSI 이스케이프 `\033[H\033[2J`로 화면 지우기(02번 영상에서 저자가 쓴 방식) | 72 → 72 |
| 05 touch | **`fopen(path, "w")`라서 기존 파일의 내용을 지움**(이번에 새로 발견) | `"a"`로 열고 `utime(path, NULL)`로 시각만 갱신 | 17 → 22 |
| 05 cat | `char c`로 EOF를 비교함(0xFF 바이트에서 멈춤) | `int c` | 0 |
| 05 mv | **대상이 이미 있는 파일이면 오류.** 저자의 설계 표에는 "if target is file or not exit, rename"이라고 되어 있음 | 원본이 하나이고 대상이 파일이면 `rename`으로 덮어씀 | 47 → 54 |
| 05 rm | 파일을 지운 뒤 `opendir`로 넘어가 오류 + 종료 코드 1(`# use fix this bug`). `closedir` 누락. 하위 실패를 무시하고 `rmdir`. 디버그용 `printf("on this case")`가 남아 있음 | 파일이면 지우고 `return 0`. 루프에서는 파일이든 폴더든 `walkrm` 하나로 처리해서 중복 코드를 없앰. `closedir` 추가. 하위가 실패하면 `rmdir` 생략 | 63 → 56 (오히려 줄어듦) |
| 05 find | rm과 같은 구조라 파일 경로를 주면 `opendir` 오류. 루프 안 오류 경로에서 `DIR`이 샘 | 파일이면 `return 0`. 오류가 나도 `continue`로 계속 찾고, 마지막에 `closedir` 후 오류를 반환 | 58 → 61 |
| 05 공통 | "Useage", "patter" 오타 | 수정 | 0 |
| 07 연결 리스트 | `// add free`만 있고 해제하지 않음(21,001건 누수). 빈 리스트를 `apply`하면 NULL 역참조 | `freeList()` 추가. `apply`는 빈 리스트면 0 반환 | 44 → 53 |
| 08 동적 배열 | `cap = 0`으로 시작하면 `cap * 2 = 0` → UB. `realloc` 실패 무시(`// add err handle`). 범용 버전에 `int *tmp`가 남음. 자기 원소를 다시 push하면 `realloc` 뒤 해제된 메모리를 읽음 | 최소 용량 1. 실패하면 이전 배열을 그대로 두고 1 반환. `void *tmp`. 늘리기 전에 값을 VLA에 복사 | 55 → 62 |
| 09 해시 테이블 | 빈 칸을 채운 뒤에도 내려가서 노드를 하나 더 붙임. 같은 키를 다시 넣으면 갱신되지 않음. 빈 칸에서 `strcmp(NULL)` → segfault. 체인 해제 없음. `get()` 들여쓰기가 3, 4, 1칸 | 빈 칸이면 `return`. 같은 키면 값 갱신. `p && p->key` 확인. `destroy()` 추가(첫 노드는 배열 안에 있으므로 체인만 해제). `malloc` 실패 시 `exit(1)`(07번 `create`와 같은 방식) | 51 → 70 |

**합계**: 6개 과제 486줄 → 536줄(+50줄, +10%). 빈 줄과 주석을 뺀 코드 기준이며, 같은 기준으로 Codex는 2,524줄(4.7배)입니다.

## 검증

[../compare/behavior.sh](../compare/behavior.sh)를 `completed` 인자로 실행했습니다(ASan/UBSan). 결과는 [../compare/behavior_result_completed.txt](../compare/behavior_result_completed.txt)에 있습니다.
- 저자 경고 플래그에서 경고 0개
- 07, 08, 09 검사가 모두 기대값과 일치하고, 누수와 UB가 없음
- 02 `tree`: 읽을 수 없는 폴더는 `t/locked: Permission denied`를 출력하고 종료 코드 1. 파일 30만 개 폴더에서 **1.4MB / 0.05초**로 스트리밍이 그대로 유지됨(Codex 62MB / 0.59초)
- 05: touch가 내용을 보존하고, 파일 위로 mv가 되며, 파일에 find를 써도 정상, 읽기 전용 하위 폴더가 있으면 rm이 정확히 그것만 남김. 모두 Codex와 같은 결과
