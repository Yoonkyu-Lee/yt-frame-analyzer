# 해체 분석: "Linked List. Conjured in C"

- 원본: https://youtu.be/iz2j9_AwQdQ (Hayyoth, 2026-10-20, 14:24, 무음 화면 녹화)
- 영상 설명: *"Linked List data structure. Lisp makes it look easy, but in C, you have to build the whole world from scratch"*
- 철학 정리: [../PHILOSOPHY.md](../PHILOSOPHY.md) (이 영상의 근거는 `[list]` 태그)

| 경로 | 내용 |
|---|---|
| [src/main.c](src/main.c) | 최종 코드 (60줄): 연결 리스트, `map`, `apply` |
| [history/](history/) | 저장 시점별 5개 버전 |
| [transcripts/](transcripts/) | 구간별 프레임 판독 원문 4개 (Chez REPL 기록 포함) |

---

## 1. 형식: Lisp 한 줄 vs C 한 세계

- **화면 구성**: 왼쪽은 vim(`main.c`), 오른쪽 위는 Chez Scheme REPL입니다. **같은 연산을 Lisp에서 먼저 보여 주고, C로 직접 만듭니다.**

| Lisp (REPL) | C (직접 구현) |
|---|---|
| `'(10 20 30)` | `typedef struct Node { int val; struct Node *next; } Node;` + `create()` + `append()` |
| `(append my-list '(40 50))` | `append()`: `while (p->next) p = p->next; // go to last node` |
| 리스트 출력 | `printList()` |
| `(map (lambda (x) (* x 2)) my-list)` | `map(Node *p, int (*f)(int))` + `int x2(int x)` |
| `(apply + my-list)` → 60, `(apply - my-list)` → -40 | `apply(Node *p, int (*f)(int, int))`: 왼쪽 접기(fold) |
| `'(10 1.5 "Hello")`: 섞인 타입 | (C 리스트는 `int`만 담음) |
| `(eval my-code)`: 코드가 곧 데이터 | **`; this very hard to implement in C`** (마지막 줄) |

## 2. 복원 신뢰도

| 버전 | 시점 | 화면 체크섬 | 복원본 |
|---|---|---|---|
| [v1](history/v1_0155_stack_nodes.c) | 01:55 | `18L, 222B` | 18L, 220B (−2) |
| [v2](history/v2_0306_printlist_bug.c) | 03:06 | `25L, 333B` | 25L, 331B (−2) |
| [v3](history/v3_0318_printlist_fix.c) | 03:18 | `25L, 337B` | 25L, 335B (−2) |
| [v4](history/v4_0615_create_append.c) | 06:15 | `38L, 559B` | 38L, 557B (−2) |
| [v6](history/v6_1233_apply.c) = src | 12:33 | `60L, 1003B` | 60L, 1000B (−3) |

- 모든 버전의 줄 수가 일치합니다. v1부터 −2B가 그대로 이어지므로 **버전 사이의 편집은 바이트까지 정확히 복원되었습니다.**
- v1은 처음 판독에서 들여쓰기가 1칸으로 잘못 읽혀 −6B였습니다. 2칸으로 바로잡자 다른 버전과 같은 −2B가 되었습니다.
- 실행 결과(WSL, gcc 13):
  - v2는 영상의 버그인 `for (; p; p->next)`(증가식에 대입이 없음) 때문에 `10 10 10 …`을 끝없이 출력합니다. 저자는 이 버그를 clang 경고 두 개(`-Wfor-loop-analysis`, `-Wunused-value`)를 보고 잡았습니다.
  - 최종본은 `10 20 30` / `sub = -40`을 출력합니다. REPL의 `(apply - my-list)` 결과와 같습니다.

## 3. 타임라인

| 시각 | 내용 |
|---|---|
| 00:00 | 스티커 메모 "this video help you: - understand linked lis…"를 입력했다가 지웁니다. 셸에 `# lisp list`를 입력하고 `chez`를 실행합니다. `'(10 20 30)`, `append`를 시연합니다. |
| 00:56–01:55 | `Node` 구조체를 만들고, 스택에 노드 3개를 둡니다(`Node n1 = {.val=10};`). `n1.next = &n2;`처럼 손으로 연결합니다. |
| 02:44–03:18 | `printList`를 추가합니다. 증가식 버그에 clang 경고 2개가 나오고, `p = p->next`로 고칩니다. |
| 03:18–06:15 | `#include <stdlib.h>`, `create()`(`malloc`, 실패하면 `exit(1)`), `append()`를 추가합니다. 손으로 연결하던 노드는 지웁니다. `sizeof Node`라고 썼다가 `sizeof(Node)`로 고칩니다(컴파일 오류). |
| 06:16–10:06 | `// add free`(TODO)를 남깁니다. REPL의 `map`을 보고 C에 함수 포인터 `map`과 `x2`, `x10`을 만듭니다. 실행 결과 `20 40 60` → `200 400 600`. |
| 10:11–12:33 | `map`을 복사해서 `apply`로 고칩니다. 첫 원소에서 시작해 나머지를 접는 방식입니다. `add`, `sub`를 추가합니다. |
| 12:39–14:23 | REPL로 돌아가 섞인 타입 리스트, `cons`(일부러 낸 오류 포함), `eval`을 시연합니다. 마지막으로 **`; this very hard to implement in C`**라고 입력합니다. |

## 4. 해석

- **Lisp를 기준으로 삼고 C로 내려갑니다.**
  - 04번 Lisp 영상에서 한 줄로 끝나던 연산(`append`, `map`, `apply`)을 C에서는 struct, `malloc`, 함수 포인터로 직접 지어야 합니다.
  - 영상 설명의 "you have to build the whole world from scratch"가 이 대비를 말합니다.
  - 마지막에 C로 흉내 낼 수 없는 것(섞인 타입, `eval`)을 REPL에서 보여 주며 끝냅니다.
- **복사해서 고치는 방식이 여기서도 나옵니다.** `apply`는 `map`을 4줄 복사해서 시그니처와 본문을 고친 것입니다(PHILOSOPHY B4).
- **스택 노드에서 힙 노드로 넘어갑니다.** 처음에는 지역 변수 3개를 손으로 연결해 구조를 눈으로 보여 줍니다. 그다음 `create`와 `append`로 일반화합니다. "상수로 먼저 확인하고 나중에 일반화한다"(B3)와 같은 흐름입니다.
- **남은 결함**
  - `// add free`라고 적었지만 해제 코드를 끝내 쓰지 않습니다(누수).
  - `map`은 원본 리스트를 바꿉니다. Scheme의 `map`은 새 리스트를 돌려줍니다.
  - 빈 리스트를 `apply`에 넘기면 NULL을 역참조합니다.
- **녹화가 일직선이 아닙니다.** 01:56 시점의 셸 스크롤백에 완성된 프로그램의 출력과 오류가 이미 보입니다. 미리 한 번 만들어 본 뒤 녹화한 것으로 보입니다.
