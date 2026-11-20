# 해체 분석: "Dynamic Array. Conjured in C"

- 원본: https://youtu.be/SOjHybBunAc (Hayyoth, 2026-10-21, 19:34, 무음 화면 녹화)
- 영상 설명: *"Building a generic dynamic array from scratch in C using malloc and realloc."*
- 철학 정리: [../PHILOSOPHY.md](../PHILOSOPHY.md) (이 영상의 근거는 `[slice]` 태그)

| 경로 | 내용 |
|---|---|
| [src/main.c](src/main.c) | 최종 코드 (76줄): 원소 타입을 가리지 않는 `Slice` |
| [history/](history/) | 단계별 5개 버전 (고정 배열 → malloc → realloc → int Slice → 범용 Slice) |
| [transcripts/](transcripts/) | 구간별 프레임 판독 원문 5개 |

---

## 1. 줄거리: "HARD" 단계를 하나씩 올라간다

| 단계 | 시각 | 코드 | 체크섬 (화면 / 복원) |
|---|---|---|---|
| [v1](history/v1_0111_fixed.c) | 01:11 | `int arr[3]` 고정 배열 | `15L, 188B` / 185B (−3) |
| [v2](history/v2_0211_malloc.c) | 02:11 | `malloc(sizeof(int) * cap)` | `21L, 309B` / 305B (−4) |
| [v3](history/v3_0307_realloc.c) | 03:07 | "how we add 44": `cap++` 후 `realloc`(실패 대비 `tmp` 사용) | `33L, 514B` / 510B (−4) |
| — | 03:10 | 주석 **`/* HARD VERSION */`**: `typedef struct { len; cap; int *data } Slice` | |
| [v4](history/v4_1046_slice_int.c) | 10:46 | `sl_init`, `sl_append`(가득 차면 `cap * 2`), `sl_free`, `sl_print` | `69L, 1194B` / 1193B (−1) |
| — | 11:32 | 주석 **`/* HARD HARD VERSION - Any data type */`**: `void *data`, `elm_size`, `sl_get`, `memcpy` | |
| [v5](history/v5_1918_generic_strings.c) | 19:18 | `int` → 복합 리터럴 `&(int){11}` → `float` → `char *` 순으로 확장 | `76L, 1474B` / 1473B (−1) |
| [src](src/main.c) | 19:33 | 주석을 **`/* HARD HARD HARD VERSION - You implement it */`**로 바꿈: 시청자에게 내는 과제 | (저장 메시지 없음) |

- 모든 단계의 줄 수가 일치하고, 바이트 차이는 −1~−4B입니다.
- 처음 판독본은 차이가 −7~−22B로 컸습니다. 원인은 들여쓰기를 1칸씩 덜 읽은 것이었습니다. 1:11 원본 프레임에서 확인하고 [tools/reindent.py](../../../tools/reindent.py)로 바로잡았습니다. 이 도구는 이미 검증된 23개 파일에서 원본을 그대로 재현하는지 먼저 시험했습니다.
- 실행 결과(WSL, gcc 13, 저자의 경고 플래그로 경고 0개)는 영상과 같습니다:
  - `11 22 33` → `11 22 33 44`
  - 최종본: `Hello World Hayyoth`

## 2. 비유: 최후의 만찬 식탁

- 03:52에 화면 전체에 식탁 그림이 나옵니다. `len`은 앉은 사람 수, `cap`은 의자 수입니다. 배경화면도 최후의 만찬 그림입니다.
- 구조체 주석에 비유가 그대로 남아 있습니다.
```c
typedef struct {
  size_t len; // such as table have 6 person
  size_t cap; // such as table can have max 12 person
  size_t elm_size;
  void *data; // array data
} Slice;
```
- `sl_append`의 주석도 같은 비유를 씁니다.
  - `// on max size table`
  - `// so, we need move to big table`
  - `// you don't want call resize every time call append`: 매번 늘리지 않고 두 배로 늘리는 이유를 한 줄로 설명합니다.

## 3. 범용화 기법

- `void *data` + `elm_size`: 원소 크기만 알면 어떤 타입이든 담을 수 있습니다. 06번 `hayyoth-lab`의 `cellSize`와 같은 기법입니다.
- `sl_get`은 `char *`로 바꾼 뒤 `index * elm_size`만큼 이동해 주소를 돌려줍니다.
- 원소 복사는 `memcpy(sl_get(sl, sl->len), val, sl->elm_size)`입니다. `strcmp`부터 쓰기 시작했다가 man 페이지(strcmp, string, strcpy, memcpy)를 차례로 읽고 `memcpy`로 바꿉니다.
- 꺼낼 때는 호출하는 쪽이 캐스트합니다: `*(char **)sl_get(sl, i)`.
- 컴파일 오류가 두 번 나고 고칩니다:
  - `len` 미선언 → `sl->len`
  - 지운 `sl_print`를 아직 호출하고 있음 → 호출 삭제

## 4. 남은 결함

| 결함 | 상태 |
|---|---|
| `realloc` 실패를 확인하지 않음 | `// add err handle` 주석만 남음 |
| 범용 버전에 `int *tmp`가 그대로 남음 | `void *` 경로에 남은 흔적. C에서는 암시적 변환이라 동작은 같음 |
| `sl_append`에서 원소 크기가 0인 경우 등은 처리하지 않음 | 범위 밖 |
| HARD HARD HARD 단계 | 시청자 몫으로 남김 |

## 5. 해석

- **"어려움"을 단계 이름으로 붙입니다.** `HARD`, `HARD HARD`, `HARD HARD HARD`. 단계마다 직전 단계를 고쳐서 만듭니다. 각 단계가 컴파일되고 실행되는 상태를 유지합니다.
- **설명 주석은 비유로 씁니다.** 처음 보는 사람을 위한 말을 코드 안에 남깁니다. 무음 영상이라 주석이 내레이션 역할을 합니다(F1).
- **마지막을 과제로 끝냅니다.** "You implement it". 05번의 "good luck", 03번의 "you write your rule"과 같은 맺음 방식입니다. 완성품을 주기보다 시청자에게 다음 단계를 넘깁니다.
- **미리 준비한 흔적이 있습니다.** 1:13 시점의 셸 스크롤백에 이 파일과 맞지 않는 `-Wsign-compare` 경고가 남아 있습니다. 리허설한 다른 파일의 출력으로 보입니다.
