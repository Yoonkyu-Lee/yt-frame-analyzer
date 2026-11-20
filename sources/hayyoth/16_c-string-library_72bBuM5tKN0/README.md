# 해체 분석: "Rewriting C String Library by Hand - K&R Style"

- 영상: https://youtu.be/72bBuM5tKN0 (19:57), 태그 `[string]`
- 결과물: [src/main.c](src/main.c) (55줄), 전사 [transcripts/](transcripts/) (8구간), 검증 [verify/](verify/)
- 방법: 프레임 1,197장 → 웹캠을 가린 키프레임 319장 → 8구간 Sonnet 전사. 오디오는 쓰지 않았습니다.

## 1. 줄거리: `<string.h>`를 한 함수씩 손으로 다시 짠다

| 시각 | 한 일 |
|---|---|
| 0:00~3:27 | `char *` 대 `char[]`: 문자열 리터럴에 쓰다가 bus error, `mess[2] = '\0'`이면 "He"와 길이 2. 스프레드시트에 글자 칸을 그려 설명합니다. Beej 26.15 `strlen()`을 읽고 포인터로 걷는 `vip_strlen`과 첨자로 세는 `normal_strlen`을 나란히 씁니다 |
| 3:27~8:14 | `normal_strlen`을 고쳐 보다가 지웁니다. 스프레드시트 "hello" 표와 셀 메모(`*p == 0 here`)로 `while (*p++)`가 끝에서 한 칸 더 간다는 것을 보여 주고 `- 1`의 이유를 설명합니다. 이름을 `hstrlen`으로 바꾸고 `hmemcpy`를 씁니다 |
| 8:14~13:04 | `hmemcpy`를 n=2로 시험하다 `Go?o`가 찍히자 `t[2] = '\0'`을 더합니다. `while (n--) *c1++ = *c2++;`로 줄입니다. `strcpy`는 `while ((*p++ = *src++));`. `-Wparentheses` 경고로 괄호를 겹치고, 이름이 libc 내장 함수와 부딪혀 `h` 접두사를 붙입니다 |
| 13:04~16:38 | Beej 26.4 `strcat()`을 읽다가 메모 **"spotted , beej guide wrong here"**, `man 3 strcat`으로 확인합니다. `hstrcat`: 끝까지 간 뒤 복사. Beej의 예제를 `main`에 가져와 `"Hello, World!"`를 확인합니다 |
| 16:38~19:57 | Beej 26.5 `strcmp`. 본문이 빈 `while (…);`로 무한 루프에 빠지자 `{ s1++; s2++; }`를 넣습니다. `return *s1 - *s2;`를 주석으로 남기고(`// it can have negative`) `unsigned char`로 빼는 형태로 바꿉니다. 시험 결과는 0, 32, -32. 최종 저장 `"main.c" 55L, 888B written` |

## 2. 복원과 검증

| 확인 | 결과 |
|---|---|
| vim 저장 기록 | 27회: `[New] 8L, 100B`(0:18) → 263B → 397B → 414B → 615B → 811B → **`55L, 888B`**(18:39). 복원 **55줄 일치**, 886바이트(−2B, 빈 줄의 공백으로 보임) |
| 컴파일과 실행 | `gcc -Wall -Wextra -pedantic`: 경고 0. 실행 결과 `-32`가 영상과 같음 |
| libc와 비교 ([verify/compare.c](verify/compare.c), [결과](verify/compare_result.txt)) | `hstrlen`, `hstrcpy`, `hstrcat`, `hstrcmp`(부호), `hmemcpy`: 빈 문자열, 비ASCII 바이트를 포함한 7개 입력의 모든 조합에서 **libc와 같음** |
| 알려진 차이 | `hstrncpy`는 n바이트를 무조건 복사합니다. libc는 원본의 `\0`에서 멈추고 나머지를 0으로 채웁니다. 원본보다 긴 n을 주면 **원본 끝을 넘어 읽습니다**(ASan: global-buffer-overflow). 영상에서는 `"Goats"`와 n=2로만 시험해서 드러나지 않았습니다 |

## 3. 해석

- **표준 라이브러리를 다시 짜는 것이 공부입니다.** 각 함수를 Beej 레퍼런스의 설명 → 원형 복사 → 손으로 본문 → 시험 순서로 짓습니다. 결과는 K&R 스타일의 짧은 포인터 관용구입니다(`while (*p++)`, `while ((*p++ = *src++))`, `while (n--) *p++ = *src++`).
- **두 방식을 나란히 두고 하나를 지웁니다.** 포인터로 걷는 `vip_strlen`과 첨자로 세는 `normal_strlen`을 함께 써 본 뒤 첨자 쪽을 지웁니다. 손으로 비교한 뒤 고른 것입니다.
- **메모리를 먼저 그립니다.** 문자열 칸, `\0`, `p`가 멈추는 위치를 스프레드시트 셀과 메모로 그린 뒤 코드를 설명합니다(13번 `[flat]`의 방식).
- **1차 자료도 의심합니다.** Beej 가이드의 오류를 찾아 메모로 남기고 man 페이지로 확인합니다. 자료를 읽되 믿지는 않는다는 태도입니다(C1).
- **경고가 진도를 정합니다.** `const` 누락, `-Wparentheses`, 내장 함수와 이름 충돌을 컴파일러가 알려 줄 때마다 고칩니다(C3).
- **시연 범위 밖은 남깁니다.** `hstrncpy`의 짧은 원본 문제는 손대지 않았습니다(D3).
