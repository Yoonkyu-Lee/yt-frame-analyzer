# 해체 분석: "building my own calendar in C - UNIX cal"

- 영상: https://youtu.be/xRZHl1B8wFc (28:31), 태그 `[cal]`
- 결과물: 복원한 코드 [src/main.c](src/main.c) (98줄), 검증 [verify/](verify/), 전사 [transcripts/](transcripts/) (7구간)
- 방법: 프레임 1,711장 → 웹캠을 가린 키프레임 314장 → 7구간 전사(코드 중심 형식). 오디오는 쓰지 않았습니다.
- **신뢰도: 마지막 저장 `"main.c" 98L, 2128B written`과 줄 수·바이트까지 일치.** vim 저장 메시지 14개를 모았습니다.

## 1. 검증 ([verify/result.txt](verify/result.txt))

- WSL gcc `-std=c11 -Wall -Wextra -pedantic -fsanitize=address,undefined`: 경고 0개.
- `./a.out 월 연도`의 출력을 같은 모양으로 Python `calendar`가 만든 달력과 비교했습니다(1900, 1901, 1970~2040, 2100, 2400년). **899개월 모두 일치.** 일광 절약 시간이 자정에 시작하던 시간대(상파울루, 테헤란, 아바나)에서도 같습니다.
- 유일한 불일치는 `TZ=Pacific/Apia`의 2011년 12월입니다(사모아가 날짜 변경선을 넘으며 12월 30일을 건너뛴 달). `mktime`으로 날짜를 세는 방식의 극단적인 경계입니다.
- 입력 검사가 없습니다: `13 2026`은 2027년 1월 달력을 제목 없이 출력하고, `abc`나 인자 3개는 0년의 열두 달을 출력합니다. 작가는 14분쯤 `// note: in production use strtol and validation year` 주석을 썼다가 1분 반 뒤에 지웁니다.

## 2. 흐름

| 시각 | 한 일 | 저장 |
|---|---|---|
| 0:00~3:00 | 진짜 `cal`, `cal 10 2026`, `cal 2026`(세 열)을 먼저 보여 줌. `man 3 time` → `time()`을 `%ld`로 찍고(`<time.h>` 빠져 오류), `localtime`으로 `tm_year` = 126 | 9L 108B → 11L 177B |
| 3:00~7:32 | Lua REPL로 126+1900 계산. 작은 변환 함수: `get_year/get_month/get_day`, 반대 방향 `get_year_tm/get_month_tm`. 사전으로 `date`와 `day` 중 고름(date는 과일이라는 뜻도 있어서) | 13L 255B, 18L 453B |
| 7:32~11:42 | `man mktime`에서 "필드를 정규화하고 `tm_wday`를 채운다"는 문단을 골라 `first_weekday`. **그 달 0일 = 지난달 마지막 날** 트릭으로 `days_in_month`. 2026년 9·10·11월을 진짜 `cal`과 대조 | 25L 552B → 40L 842B |
| 11:42~17:15 | 빈 vim 버퍼에 명세(`cal`, `cal 2026`, `cal 10 2026`)를 적고 `switch (argc)`. `month == 0`이면 1년 | 65L 1331B |
| 17:15~21:24 | `print_month`를 한 줄씩: 요일 머리줄(진짜 `cal`에서 복사) → 앞쪽 빈칸 → `%2d ` → 줄바꿈 조건. 단계마다 `cal`과 비교 | 78L → 82L 1652B |
| 21:24~25:34 | 오늘 날짜를 반전 표시(`\033[7m`…`\033[0m`) | 91L 1950B |
| 25:34~28:30 | `print_year`(1~12월, `-------- %2d --------`). `cal 2026`과 비교한 뒤, 셸의 `dquote>` 프롬프트를 메모장 삼아 과제를 적고 끝 | **98L 2128B** |

마지막 메모(셸의 따옴표 연속 프롬프트에 입력, 실행되지 않음):
```
dquote> In default cal it print by three column
dquote> you do that
dquote> we can update print_month not print to terminal
dquote> print_month add output to buffer ,
dquote> then print_year get buffer and print output to terminal
```

## 3. 최종 코드의 핵심

```c
int first_weekday(int year, int month) {
  struct tm date = {0};

  date.tm_year = get_year_tm(year);
  date.tm_mon = get_month_tm(month);
  date.tm_mday = 1;

  mktime(&date);

  return date.tm_wday;
}

int days_in_month(int year, int month) {
  struct tm date = {0};

  date.tm_year = get_year_tm(year);
  date.tm_mon = get_month_tm(month) + 1;
  date.tm_mday = 0;

  mktime(&date);

  return date.tm_mday;
}
```
윤년 규칙도, 요일 공식(Zeller 등)도 직접 짜지 않습니다. 표준 라이브러리의 `mktime`이 날짜를 정규화한다는 한 문장(man 페이지)에 계산을 모두 맡깁니다. `print_month`의 줄바꿈 조건은 `(first_day + day) % 7 == 0 || day == total_days` 한 줄입니다.

진짜 `cal`과 다른 점(영상에서 작가도 확인): 달 이름 제목 줄이 없고, 1년을 세 열이 아니라 한 열로 출력하고, 각 줄 끝에 공백이 남습니다.

## 4. 해석

- **진짜 도구가 명세이자 시험지입니다(A1).** 시작할 때 `cal`을 돌려 보고, 함수 하나를 만들 때마다 `cal 10 2026`과 나란히 비교합니다. 요일 머리줄도 `cal`의 출력에서 복사합니다.
- **man 페이지의 한 문장을 설계로 바꿉니다(C1).** "mktime은 범위를 벗어난 필드를 정규화한다"를 마우스로 골라 둔 뒤, "0일 = 지난달 마지막 날"로 씁니다. 날짜 계산 코드는 사실상 0줄입니다.
- **작은 변환 함수로 표준 구조체의 이상한 규칙을 가둡니다.** `tm_year`는 1900년부터, `tm_mon`은 0부터라는 규칙을 한 줄짜리 함수 다섯 개에 넣고, 나머지 코드는 사람이 쓰는 연도와 월만 다룹니다. 이름도 사전을 찾아 고릅니다.
- **스텁을 먼저 두고 하나씩 바꿉니다(B2, B3).** `printf("year = %d\n", year)` 자리에 나중에 `print_year`가 들어갑니다.
- **알면서 남긴 약점은 지우고 넘어갑니다(D3).** `atoi` 대신 `strtol`과 검사를 써야 한다는 주석을 썼다가 지웁니다. 코드는 최소로 두고, 세 열 출력은 시청자 과제로 넘깁니다(F2).
- 화면의 `cc main.c`는 엄격한 경고 옵션을 붙인 clang 별칭입니다([24번 `.zshrc`](../24_w3m_6DQCtRSTfNw/)). 영상에서 경고가 한 번도 뜨지 않았고, 우리 검증에서도 gcc 엄격 옵션으로 경고 0개입니다(C3).
