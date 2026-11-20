# 해체 분석: "Intro Lisp, we conjure the spirits of the computer with our spells"

- 원본: https://youtu.be/5ulMKWo2wJI (Hayyoth, 2026-10-17, 10:07, 무음 화면 녹화)
- 철학 정리: [../PHILOSOPHY.md](../PHILOSOPHY.md) (이 영상의 근거는 `[lisp]` 태그)

| 경로 | 내용 |
|---|---|
| [src/notes.md](src/notes.md) | 저자가 미리 써 둔 강의 목차. 강의는 이 목차 순서대로 진행됩니다. |
| [src/session.ss](src/session.ss) | REPL에 입력한 식을 순서대로 모은 스크립트. 결과는 주석으로 달았습니다. |
| [transcripts/](transcripts/) | 구간별 프레임 판독 원문 3개 |

---

## 1. 형식

코드베이스를 만드는 영상이 아니라 **REPL 시연 강의**입니다.

- **화면 구성**
  - 왼쪽: Chez Scheme 10.4.1 REPL
  - 오른쪽 위: vim으로 연 목차 노트(`notes.md`)
  - 오른쪽 아래: 웹캠
- **진행 방식**: 목차의 현재 주제를 vim VISUAL 모드로 선택해 강조하고, 왼쪽 REPL에서 한 줄짜리 예제로 시연합니다. 주제가 바뀔 때마다 화면을 지웁니다.
- **시작과 끝**
  - 첫 셸 입력은 주석 `# In this video , i will make you love lisp`입니다.
  - 마지막에는 REPL 프롬프트에 `book to read`라고 써 두고 브라우저로 넘어가 참고서 두 권을 보여 줍니다.

## 2. 복원 신뢰도

- **목차**: 세 구간에서 본 내용을 합쳐 전문을 복원했습니다. 영상 내내 내용이 바뀌지 않았습니다.
- **REPL 입력**: 화면에 남은 순서대로 복원했습니다.
- **큰 수 결과**: Python으로 교차 검증했습니다.
  - `(expt 2 100)` = 1267650600228229401496703205376 ✅
  - `(expt 2 1000)`은 끝자리 `…5668069376` ✅, `(expt 2 10000)`은 끝자리 `…96709376` ✅
  - `(factorial 100)`은 `933262154439441…`로 시작 ✅, `(factorial 1000)`은 2,568자리 ✅
  - 식이 화면에 남지 않은 "…883109376"으로 끝나는 결과가 하나 있었습니다. 후보 지수를 계산해 끝자리를 맞춰 보니 **`(expt 2 100000)`**만 일치했습니다.
- **확인하지 못한 것**
  - `(* 10 1000…0)`에 넣은 0의 개수
  - `(sub ...)` 호출의 인자
  - Chez Scheme이 설치되어 있지 않아 [src/session.ss](src/session.ss)는 실행해 보지 못했습니다. `chez --script src/session.ss`로 확인할 수 있습니다.

## 3. 타임라인 (목차 순서 = 강의 순서)

| 시각 | 목차 항목 | 시연 |
|---|---|---|
| 00:00 | — | 셸에 `# In this video , i will make you love lisp` 입력, `chez` 실행 |
| 00:10– | some basic operator | `(+ 1 2)`, `(+ 1 2 3)`: 전위 표기, 인자를 여러 개 받음 |
| 00:40– | real numbers are always precise. | `(/ 2 3)` → `2/3`. `(* (/ 2 3) (/ 3 2))` → `1`: 분수가 정확한 값으로 남음 |
| 01:00– | large number never overflow | `(expt 2 n)`을 n = 1, 2, 3, 10, 100, 1000, 10000, 100000으로 키워 가며 스크롤로 자릿수를 보여 줌 |
| 02:15– | define – variable / function | `(define x 10)`, `(define (add x y) (+ x y))` |
| 03:00 | (즉흥) | `(define (+ x y) (- x y))` 후 `(+ 3 4)` → `-1`. 연산자도 평범한 이름이라 다시 정의할 수 있음을 보여 주고 REPL을 재시작 |
| 03:30– | recursion made easy – factorial | 처음에 중위 표기 `(n * ...)`로 써서 `attempt to apply non-procedure 2` 오류. 이후 `fatorial` 오타로 `variable fatorial is not bound` 오류. 둘 다 그 자리에서 고침. `(factorial 1000)`까지 실행 |
| 05:00– | – loop | `display`와 재귀로 반복을 만듦: `(loop 2 10)` |
| 05:40– | list data struct | `'(1 2 3)`, `car`, `cdr`, 재귀 `print-list`, `apply`, `map` + `lambda` |
| 07:30– | code is data – eval | `(define my-code '(+ 1 2 3))` → `(eval my-code)` → `6`. 다시 `'(apply + '(1 2 3 4))`로 바꿔 → `10` |
| 08:20– | another feature | `(system "ls")`, `(system "mkdir abc")`: Scheme에서 셸을 직접 부름. 나머지 항목(C 호출, 속도, JIT 등)은 목차만 짚고 넘어감 |
| 09:11– | (book to read) | SICP PDF(web.mit.edu)를 잠깐 보여 줌 → **TSPL4**(Dybvig, scheme.com/tspl4) 목차를 오래 봄. 6.13 Hashtables, 4.5 Multiple Values, 4.6 Variable Definitions를 열어 봄 |
| 10:06 | — | 목차 마지막 줄 "all feature in high level language have in lisp"를 선택한 상태로 녹화 종료 |

## 4. 해석: 무엇을 가르치는가

목차 문구에 저자의 관점이 직접 드러납니다.

- **"very minimal and fast (f\*\* simple)"**: 문법이 거의 없다는 점을 장점으로 내세웁니다.
- **"repl - code playground very easy"**: 문서를 읽기 전에 직접 쳐서 결과를 보라는 뜻입니다. 강의 전체가 이 방식으로 진행됩니다.
- **"all feature in high level language have in lisp"**: 다른 고수준 언어의 기능이 이미 Lisp 안에 다 있다는 주장입니다.

**시연 방식**
- 예제는 전부 한 줄짜리입니다.
- 입력값을 10배씩 키웁니다(10 → 100 → 1000 …). 숫자가 넘치지 않는다는 개념을 말로 설명하는 대신 화면을 가득 채우는 숫자로 보여 줍니다.
- 오류를 숨기지 않습니다. 중위 표기 실수는 오히려 "목록의 첫 자리가 함수"라는 규칙을 보여 주는 예가 됩니다.
- 연산자 `+`를 뺄셈으로 재정의하는 장난도 "모든 것이 이름"이라는 점을 몸으로 보여 줍니다.

**C 영상과의 연결**
- 반복을 루프 구문 대신 재귀(`loop`, `print-list`)로 만듭니다. tree 영상의 `walk` 재귀와 같은 생각입니다.
- `(system "mkdir abc")`로 언어 안에서 운영체제를 직접 다룹니다. 저수준을 감추지 않는 태도는 C 영상들과 같습니다.

**참고 자료 추천**
- 블로그나 튜토리얼이 아니라 **정전(正典) 교재 두 권**을 추천합니다: SICP와 TSPL4.
- C 영상에서 POSIX 명세와 man 페이지를 본 것처럼, 1차 자료를 우선하는 태도가 이어집니다.
