# 해체 분석: "Hash Table. Conjured in C"

- 원본: https://youtu.be/61XUoywg970 (Hayyoth, 2026-10-23, 33:52, 무음 화면 녹화)
- 영상 설명: *"I try to understand Hash Table"*
- 철학 정리: [../PHILOSOPHY.md](../PHILOSOPHY.md) (이 영상의 근거는 `[hash]` 태그)
- **AI를 쓴 두 번째 영상입니다.** ChatGPT의 답을 교재로 삼아 **자기 손으로 다시 짭니다**. 제목에 "No AI"가 없습니다.

| 경로 | 내용 |
|---|---|
| [src/main.c](src/main.c) | 저자가 손으로 다시 짠 해시 테이블 (67줄) |
| [reference/chatgpt_hashtable.c](reference/chatgpt_hashtable.c) | 영상 첫머리에 ChatGPT에서 받아 `test.c`로 붙여 넣은 코드 (91줄). **화면 체크섬과 바이트까지 일치** |
| [history/](history/) | 저자 코드의 중간 버전 |
| [verify/edge_test.py](verify/edge_test.py) | 경계 조건 시험 |
| [transcripts/](transcripts/) | 구간별 프레임 판독 원문 11개 |

---

## 1. 줄거리: AI 코드를 받아 놓고, 이해될 때까지 처음부터 다시 짠다

| 시각 | 내용 |
|---|---|
| 00:01 | 스티커 메모: **`i try to understand hashtable`** |
| 00:13–00:48 | ChatGPT(**hayyoth 계정으로 로그인**, 원본 프레임으로 확인)에 `hash table in C`를 입력합니다. 답(체이닝 방식, djb2 해시, `SIZE 101`)을 `~/lab/test.c`에 붙여 넣습니다. |
| 01:01–01:20 | 붙여 넣은 91줄을 훑어봅니다. 브라우저 콘솔에서 JS 객체 `{name:"Cong", age:20}`로 "해시 테이블은 JS에서 이렇게 생겼다"를 보여 줍니다. |
| 01:20–06:11 | **새 파일 `main.c`에서 처음부터 시작합니다.** `int arr[5]` → `arr['a']`(문자를 인덱스로) → `arr[7 % 5] = 77; // can i do this`. Wikipedia "Hash table"의 "Collision resolution" 절과 배경화면 그림을 가리킵니다. |
| 06:11–09:30 | `struct Node {value, key, next}` 배열을 만들고 `arr[2].next = &(Node){...}`로 충돌을 체인으로 손수 연결합니다. 스티커: `we can replace key int to key / string // look at chatgp[t]` |
| 09:30–13:23 | `#define SIZE 5`, `hash(int key) { return key % SIZE; }`, 이어서 문자열 키 해시를 만듭니다: 글자 코드의 합 `% SIZE`. 오른쪽 터미널에 **ChatGPT 코드(`test.c`)를 띄워 놓고** VISUAL 모드로 비교합니다. |
| 13:23–18:25 | `-Wint-conversion` 오류를 보고 키와 인덱스를 구분합니다. 스티커: **`it simple than chatgpt but / "abc" same "cba" or "acb"`**. 자기 해시의 약점(애너그램끼리 충돌)을 스스로 지적합니다. ChatGPT의 `get`을 9줄 yank하지만 **붙여 넣지 않고** 직접 타이핑합니다. |
| 18:25–20:18 | 스티커: `if you not understand linked list , check my previous video`. 자기 07번 영상을 열어 `p = p->next` 순회 부분에서 멈춥니다. |
| 20:18–24:07 | `get()`을 완성합니다. `"def"`를 `"cba"`로 바꿔 일부러 같은 버킷에서 충돌을 냅니다(`// because abc and cba have same index`). 실행 → `22` 후 **segfault**. 배경화면의 체인 그림(152 → Sandra Dee)을 가리킵니다. |
| 24:07–31:19 | `set()`을 씁니다: 빈 칸이면 바로 넣고, 아니면 끝 노드까지 가서 `malloc`으로 새 노드를 붙입니다. "끝 노드로 가는" 한 줄은 **07번 영상의 `append`(`while (p->next) p = p->next;`)를 보고 가져옵니다.** |
| 31:19–33:04 | `const` 관련 컴파일 오류 4개를 고칩니다. `main`을 `set(arr, "abc", 22)` 같은 호출로 정리하고 마지막으로 저장합니다(`67L, 1184B`). 경고 없이 컴파일됩니다. |
| 33:28 | 마지막 스티커: **`all work , we now understand hashtable and chatgpt code you implement refactor of it for complex`** |

## 2. 두 코드 비교

| | ChatGPT ([reference](reference/chatgpt_hashtable.c)) | 저자 ([src](src/main.c)) |
|---|---|---|
| 크기 | `SIZE 101`, 전역 `Entry *table[SIZE]` | `SIZE 5`, `Node arr[SIZE]`를 함수 인자로 넘김 |
| 해시 | djb2: `h = 5381; h = h * 33 + c` | 글자 코드의 합 `% SIZE`. 저자 스스로 "abc = cba"라는 약점을 메모함 |
| 버킷 | 포인터 배열, 노드는 모두 `malloc` | 구조체 배열. 첫 노드는 배열 안에 두고 충돌한 노드만 `malloc` |
| 키 | `strdup`처럼 복사해서 저장 | 리터럴 포인터를 그대로 저장 |
| 갱신 | 같은 키가 있으면 값만 덮어씀 | 없음 |
| 조회 | `int *` 또는 NULL을 반환 | `int`를 반환하고, 없으면 0 |
| 해제 | `destroy()` | 없음 |
| 형식 | 4칸 들여쓰기, 설명형 주석 | 2칸 들여쓰기, `get()`만 들여쓰기가 고르지 않음(원본 프레임으로 확인, 2절 끝 참고) |

**검증한 결함** (WSL에서 실행, [verify/edge_test.py](verify/edge_test.py))
- **같은 키가 두 번 들어갑니다.** 빈 버킷에 `set` 한 번 → 배열 칸을 채운 뒤 `return` 없이 그대로 내려가 노드를 하나 더 붙입니다. `abc` 버킷의 노드 수가 3개(abc, abc, cba)로 나옵니다.
- **빈 버킷에서 없는 키를 찾으면 segfault가 납니다.** 키가 NULL인 상태로 `strcmp`를 호출합니다. 영상 23:5x에서도 실행 중 segfault가 났지만, 그때의 원인이 같은지는 화면만으로 확인할 수 없습니다.

**복원 신뢰도**
- 최종본 `67L, 1184B`에 대해 복원본은 67L, 1177B(−7)입니다.
- `get()`의 이상한 들여쓰기(3, 4, 1칸)는 판독 오류가 아니라 **실제로 그렇게 쓰여 있었습니다.** 33:15 원본 프레임에서 확인했습니다. 들여쓰기 정규화 도구를 그대로 적용하면 오히려 틀린 결과가 나왔을 경우입니다.
- 중간 버전 [v1](history/v1_0920_array_of_nodes.c)은 `27L, 342B`에 대해 341B(−1)입니다.

## 3. 해석

- **03번과 다르게 AI를 씁니다.**
  - 03번: AI 코드를 붙여 넣고 실행해 본 뒤 **undo로 버렸습니다**.
  - 09번: AI 코드를 **오른쪽 창에 교재처럼 띄워 두고**, 왼쪽에서 가장 단순한 형태(`int arr[5]`)부터 한 단계씩 다시 짭니다.
  - 최종 산출물에 AI 코드는 한 줄도 섞이지 않았습니다. `get`을 yank했지만 붙여 넣지 않았습니다.
  - 마지막 메모의 "we now understand ... chatgpt code"가 목적을 말합니다. **AI 코드를 이해하는 것이 목표**입니다.
- **AI 코드보다 단순하게 만들고, 그 대가를 스스로 적습니다.** "it simple than chatgpt but "abc" same "cba"". 더 단순한 쪽을 고르되 무엇을 잃는지는 압니다.
- **자기 영상을 레퍼런스로 씁니다.** 연결 리스트 순회는 07번 영상을 다시 틀어 그 코드를 보고 가져옵니다(B4: 자기 코드를 복사해서 시작).
- **배경화면이 수업 자료입니다.** 해시 테이블 그림(keys → buckets → entries)을 배경으로 깔고, 충돌을 설명할 때 창을 치우고 그 그림을 가리킵니다.
- **마무리는 다시 시청자에게 넘깁니다.** "you implement refactor of it for complex"(08번의 "You implement it"과 같은 방식).
