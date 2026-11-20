# Hayyoth: 손코딩 채널 해체 분석

AI 없이 손으로 코딩하는 Hayyoth 채널(https://www.youtube.com/@hayyoth-org)의 영상을 해체 분석한 결과물입니다. 2026-11-17 기준 채널의 영상 26개를 모두 분석했습니다. 다른 출처와의 관계는 [../README.md](../README.md)에 있습니다.

- **[PHILOSOPHY.md](PHILOSOPHY.md)**: 여러 영상에 걸쳐 쌓아 가는 작업 원칙입니다. 원칙마다 관찰과 해석을 나누고, 영상 태그와 타임스탬프를 근거로 붙입니다.
- 영상별 폴더: `NN_<slug>_<videoId>/`. NN은 채널 업로드 순서입니다.

## 분석한 영상

| # | 폴더 | 태그 | 영상 | 길이 | 결과물 | 복원 정확도 |
|---|---|---|---|---|---|---|
| 01 | [01_typing-by-hand_WLih6RtW_hE](01_typing-by-hand_WLih6RtW_hE/) | `[intro]` | [No AI. Do you remember the days when we used to type code by hand…](https://youtu.be/WLih6RtW_hE) | 1:49 | C 10줄 (채널 선언문) | 체크섬 2/2 **바이트까지 일치** |
| 02 | [02_c-tree-viewer_tiMHtc6l7xM](02_c-tree-viewer_tiMHtc6l7xM/) | `[tree]` | [No AI. Build your own tree directory viewer in C by hand](https://youtu.be/tiMHtc6l7xM) | 19:55 | C 58줄 `tree` 클론 | 체크섬 9/9 줄 수 일치(2개는 바이트까지), 공백 7B 차이 |
| 03 | [03_c-random-moving-numbers_IJe5WtpXx5c](03_c-random-moving-numbers_IJe5WtpXx5c/) | `[walk]` | [Programming magical random moving numbers in C](https://youtu.be/IJe5WtpXx5c) | 24:41 | C 87줄 입자 시뮬레이션. **ChatGPT 사용 후 되돌림** | 체크섬 3/3 줄 수 일치(1개는 바이트까지), 공백 3B 차이 |
| 04 | [04_lisp-intro_5ulMKWo2wJI](04_lisp-intro_5ulMKWo2wJI/) | `[lisp]` | [Intro Lisp, we conjure the spirits of the computer with our spells](https://youtu.be/5ulMKWo2wJI) | 10:07 | 강의 목차 + REPL 세션 스크립트 | 큰 수 결과를 Python으로 교차 검증 |
| 05 | [05_c-linux-file-utils_gQRmSlcTq9c](05_c-linux-file-utils_gQRmSlcTq9c/) | `[utils]` | [No AI, 52 min. Build all Linux Files utils in C by Hand](https://youtu.be/gQRmSlcTq9c) | 52:27 | C 유틸 6개 + 설계 표 | 체크섬 6/6 줄 수 일치(2개는 바이트까지) |
| 06 | [06_c-simulation-lab_Kb0YFATY3ug](06_c-simulation-lab_Kb0YFATY3ug/) | `[lab]` | [Simulation in C. We build the Machine, the Machine builds Us](https://youtu.be/Kb0YFATY3ug) | 6:26 | 프레임워크 시연 + 주석 추가 (코드는 공개 저장소) | **공개 저장소와 대조: 화면에 보인 274줄 100% 일치** |
| 07 | [07_c-linked-list_iz2j9_AwQdQ](07_c-linked-list_iz2j9_AwQdQ/) | `[list]` | [Linked List. Conjured in C](https://youtu.be/iz2j9_AwQdQ) | 14:24 | C 60줄 + Lisp 대비 | 체크섬 5/5 줄 수 일치, 공백 2~3B 차이 |
| 08 | [08_c-dynamic-array_SOjHybBunAc](08_c-dynamic-array_SOjHybBunAc/) | `[slice]` | [Dynamic Array. Conjured in C](https://youtu.be/SOjHybBunAc) | 19:34 | C 76줄 범용 동적 배열 | 체크섬 5/5 줄 수 일치, 공백 1~4B 차이 |
| 09 | [09_c-hash-table_61XUoywg970](09_c-hash-table_61XUoywg970/) | `[hash]` | [Hash Table. Conjured in C](https://youtu.be/61XUoywg970) | 33:52 | C 67줄 + ChatGPT 원본 코드. **AI 코드를 교재로 다시 짬** | ChatGPT 코드 **바이트까지 일치**, 저자 코드 −7B |
| 10 | [10_human-learn-better_48DqkAx86Vg](10_human-learn-better_48DqkAx86Vg/) | `[learn]` | [How we (Human) learn better than AI?](https://youtu.be/48DqkAx86Vg) | 11:37 | 논증 정리 (코드 없음) | 메모 문구 전사. 1개 구간은 프레임 일부만 판독 |
| 11 | [11_c-ed-editor_MKmPyByntzA](11_c-ed-editor_MKmPyByntzA/) | `[ed]` | [No AI. Build ed (the standard text editor) in 100 lines of C](https://youtu.be/MKmPyByntzA) | 1:10:04 | C 113줄 `ed` 클론. **ChatGPT의 549줄을 보여 주고 거절** | 체크섬 29회 수집, 최종 **줄 수 일치**(113L), 공백 −12B. 영상 속 세션 재현 일치 |
| 12 | [12_my-story_gLn7F22HngI](12_my-story_gLn7F22HngI/) | `[story]` | [I am Human, not AI. This is my story.](https://youtu.be/gLn7F22HngI) | 15:43 | 작가의 이야기 요약 (코드 없음) | 메모·번역문 전사, 개인 식별 정보는 가림 |
| 13 | [13_c-is-flat_wj9os_e8Hs4](13_c-is-flat_wj9os_e8Hs4/) | `[flat]` | [C is Flat. If everything is Flat, you can't go wrong.](https://youtu.be/wj9os_e8Hs4) | 18:10 | 강의 정리. **AI(Gemini)가 쓴 시연 코드를 처음 그대로 실행** | 메모·시트·코드 전사 |
| 14 | [14_vscodium_rc5dnqN2IXc](14_vscodium_rc5dnqN2IXc/) | `[codium]` | [vscode - but better](https://youtu.be/rc5dnqN2IXc) | 5:39 | VSCodium 설정 과정 (장식을 모두 끔) | 설정·`settings.json` 전사 |
| 15 | [15_macos-no-apple_abLEkywTtD4](15_macos-no-apple_abLEkywTtD4/) | `[noapple]` | [macOS - but better - No Apple](https://youtu.be/abLEkywTtD4) | 6:06 | macOS 설정 + LuLu 방화벽으로 Apple 연결 차단 | 설정 전후·명령 전사 |
| 16 | [16_c-string-library_72bBuM5tKN0](16_c-string-library_72bBuM5tKN0/) | `[string]` | ["Rewriting C String Library by Hand - K&R Style"](https://youtu.be/72bBuM5tKN0) | 19:57 | C 55줄: `<string.h>` 함수 6개 다시 짜기 | 체크섬 27회, 최종 **줄 수 일치**(55L), 공백 −2B. **libc와 비교 일치**(`hstrncpy` 제외) |
| 17 | [17_pure-html_fG7mM6Yymaw](17_pure-html_fG7mM6Yymaw/) | `[html]` | [Pure HTML just works and beats AI Slop](https://youtu.be/fG7mM6Yymaw) | 10:01 | 논증 + 손으로 쓴 `index.html`(26줄, 일부 판독) | 메모·시트·HTML 전사 |
| 18 | [18_pure-js-web-editor_VBORE_zH5oc](18_pure-js-web-editor_VBORE_zH5oc/) | `[webjs]` | [No AI - Just pure JS hand coding to relax - Web Editor & Files Manager](https://youtu.be/VBORE_zH5oc) | 1:28:00 | JS 웹 편집기(12줄 + Ace) + 파일 관리자 "HFiles"(localForage). 중간에 Go로 시작했다가 접음 | 체크섬 없음(줄 번호·저장 메시지 없음). 마지막 `initFiles` 본문 2줄은 문서대로 보충, 표시함 |
| 19 | [19_human-beats-slop_uRLMMYFoGB4](19_human-beats-slop_uRLMMYFoGB4/) | `[proof]` | [pure Human beats AI slop](https://youtu.be/uRLMMYFoGB4) | 4:06 | 논증 + 채널 통계(28일 61만 조회, 구독자 +9.4K) | 메모·통계 전사 |
| 20 | [20_ai-cannot-fake_TAEBfnNUYAc](20_ai-cannot-fake_TAEBfnNUYAc/) | `[history]` | [What AI cannot fake](https://youtu.be/TAEBfnNUYAc) | 3:11 | 논증: Wayback Machine의 옛 사이트, 2024년 GitHub | 메모·보관 페이지 전사 |
| 21 | [21_golang-for-fun_avofMrOfsWI](21_golang-for-fun_avofMrOfsWI/) | `[go]` | [a bit of Golang for fun](https://youtu.be/avofMrOfsWI) | 1:29:57 | Go 다시 배우기 → `net/http` + WebSocket 채팅(gorilla 예제 사용) | 체크섬 없음. Go가 없어 컴파일 검증 안 함(정적 읽기) |
| 22 | [22_firefox_TJRWmmVLIyo](22_firefox_TJRWmmVLIyo/) | `[firefox]` | [firefox - but better](https://youtu.be/TJRWmmVLIyo) | 3:46 | Firefox 설정: AI 기능 차단, 데이터 수집 끔, `userChrome.css`로 탭 막대 제거 | 설정·CSS 전사 |
| 23 | [23_go-wget_g4SQ1LGRI00](23_go-wget_g4SQ1LGRI00/) | `[wget]` | [build your own wget (Web Crawler) in Go by hand](https://youtu.be/g4SQ1LGRI00) | 24:57 | Go 약 80줄 크롤러(옛 사이트를 통째로 저장) | 체크섬 없음. Go가 없어 정적 읽기만 |
| 24 | [24_w3m_6DQCtRSTfNw](24_w3m_6DQCtRSTfNw/) | `[w3m]` | [you should try w3m - terminal web browser](https://youtu.be/6DQCtRSTfNw) | 10:59 | 텍스트 브라우저 시연 + 작가의 `.zshrc` | 메모·명령·설정 전사 |
| 25 | [25_c-cat_ie0PfrXZsc](25_c-cat_ie0PfrXZsc/) | `[cat]` | [No AI. Build 'cat' command in C - but better for AI](https://youtu.be/_ie0PfrXZsc) | 27:32 | C 92줄: 폴더를 재귀로 도는 "AI에 넘기기 좋은" `cat`. **Gemini는 결과를 읽는 쪽** | 체크섬 9회, 최종 **줄 수 일치**(92L), 공백 −7B. 동작 시험 8개 |
| 26 | [26_c-cal_xRZHl1B8wFc](26_c-cal_xRZHl1B8wFc/) | `[cal]` | [building my own calendar in C - UNIX cal](https://youtu.be/xRZHl1B8wFc) | 28:31 | C 98줄 `cal` 클론 | 체크섬 14회, 최종 **바이트까지 일치**(98L 2128B). Python 달력과 899개월 일치 |

**분석 현황**: 2026-11-17 기준 채널의 영상 26개를 모두 분석했습니다. 새 영상이 올라오면 같은 방식으로 이어서 분석합니다.

10번부터는 형식이 바뀝니다. 웹캠에 얼굴이 크게 나오고, 대화형·강의형 영상이 섞입니다. 오디오 트랙은 있지만 자막은 없으며, 원칙대로 프레임만으로 분석했습니다. 대화형 영상은 코드 대신 화면의 글(메모, 노트, 번역문)을 원문 그대로 받아 적는 형식으로 전사했습니다.

## 영상별 요약

영상마다 프로그래머의 임무, 만든 것, 핵심 개념, 접근 방식만 추렸습니다. 복원 근거와 자세한 타임라인은 각 폴더의 README에 있습니다.

### 01. 손으로 치던 시절을 기억하나 (1:49)
채널의 첫 영상이자 선언문입니다. 기술을 가르치는 영상이 아닙니다. 터미널에서 vim으로 Hello World를 직접 치고, 그 아래에 `No AI --- `를 끝없이 출력하는 루프를 붙입니다. 화면을 가득 채운 출력 자체가 메시지입니다. 웹캠 창 이름 "Human", vim 탭과 셸 탭을 오가는 작업 방식, 엄격한 경고를 켠 `cc` 등 이후 영상의 형식이 이 짧은 영상에서 이미 다 갖춰집니다.

### 02. `tree` 직접 만들기 (19:55)
- **임무**: 리눅스 `tree` 명령처럼 폴더 내용을 계층으로 보여 주는 프로그램을 만드는 것입니다.
- **핵심 개념**: 파일시스템은 트리이므로 재귀로 다루는 것이 자연스럽습니다.
  - `opendir`/`readdir`로 항목을 하나씩 읽습니다.
  - 항목이 폴더(`DT_DIR`)면 그 경로로 같은 함수를 다시 부릅니다.
  - 재귀 깊이가 그대로 들여쓰기 깊이가 됩니다.
- **접근**
  - 진짜 `tree`를 먼저 실행해 목표 모양을 확인합니다.
  - README에 설계 4줄을 적습니다.
  - POSIX 명세와 man 페이지를 보며 이름 출력 → `.`/`..` 건너뛰기 → 폴더 판별 → 색 → 재귀 → 들여쓰기 순으로 하나씩 붙입니다.
  - 경고가 날 때마다 고칩니다.

### 03. 혼자 움직이는 숫자 (24:41)
- **임무**: 터미널 화면 위에서 숫자들이 스스로 움직이는 시뮬레이션을 만드는 것입니다.
- **핵심 개념**: 시뮬레이션은 격자 상태, 짧은 규칙, 매 프레임 다시 그리기로 이루어집니다. 한 번 계산하는 동안 이미 옮긴 값이 다시 옮겨지지 않도록 현재(`state`)와 다음(`next`) 두 버퍼를 씁니다.
- **접근**
  - 규칙을 한 줄씩 바꿔 가며 바로 실행해 봅니다. 예: 양을 보존하면 퍼지기만 하고, 감소를 빼면 숫자가 불어납니다.
  - 막히자 ChatGPT에서 Game of Life 코드를 받아 돌려 봅니다. 잘 돌았지만 undo로 전부 걷어 냅니다.
  - Wikipedia의 확산 그림을 보고 입자와 먹이 규칙을 직접 만듭니다.
  - 마지막 말은 "you write your rule"입니다.

### 04. Lisp 입문 (10:07)
- **임무**: Lisp가 왜 좋은지 보여 주는 것입니다. 코드베이스는 없고 REPL 시연 강의입니다.
- **핵심 개념**
  - 전위 표기
  - 정확한 분수(`2/3`)와 넘치지 않는 큰 수
  - 루프 대신 재귀
  - 리스트(`car`/`cdr`, `map`, `apply`)
  - 코드가 곧 데이터라는 점(`eval`)
- **접근**
  - 미리 써 둔 목차를 옆에 띄우고 한 줄짜리 예제로 하나씩 시연합니다.
  - 입력을 10배씩 키워 결과로 개념을 보여 줍니다.
  - 마지막에 참고서로 SICP와 TSPL을 추천합니다.

### 05. 리눅스 파일 명령 6종 (52:27)
- **임무**: `touch`, `cat`, `mkdir`, `mv`, `rm`, `find`를 C로 직접 구현하는 것입니다.
- **핵심 개념**: 각 명령은 결국 시스템 호출 하나를 감싼 것입니다(`fopen`, `mkdir`, `rename`, `unlink`). 어려운 것은 대상이 폴더인지에 따라 갈리는 명령들입니다.
  - `mv`: 대상이 폴더면 그 안으로 옮깁니다.
  - `rm`: 폴더면 안을 모두 지운 뒤 폴더를 지웁니다.
  - `find`: 재귀로 돌며 이름을 비교합니다.
- **접근**
  - "명령 / 쓸 함수 / 알고리즘" 설계 표를 먼저 써 둡니다.
  - 새 명령은 직전 파일을 통째로 복사해서 고칩니다.
  - 모르는 함수는 전부 man 페이지로 확인합니다.
  - 시간은 대부분 `mv`와 `rm`에 듭니다.
  - `rm` 버그를 발견하고도 메모만 남기고 다음으로 넘어갑니다.

### 06. 시뮬레이션 프레임워크 (6:26)
- **임무**: 저자가 미리 만들어 둔 C 시뮬레이션 프레임워크 `hayyoth-lab`을 소개하는 것입니다.
- **핵심 개념**: 사용자는 함수 세 개만 씁니다. 그러면 단순한 규칙에서 복잡한 패턴이 생겨나는 것(창발)을 볼 수 있습니다.
  - 초기 상태
  - 규칙: 주변 3×3 칸을 보고 다음 값을 정함
  - 보기: 칸을 색으로 바꿈
- **접근**
  - Conway, 산불, 개미, 와류 같은 시뮬레이션을 실행해 보여 줍니다.
  - 코드 위에 주석을 달아 가며 구조를 설명합니다.
  - 마지막에 "03번 영상이 이 라이브러리의 단순한 버전"이라고 직접 밝힙니다.

### 07. 연결 리스트 (14:24)
- **임무**: 연결 리스트를 이해시키는 것입니다.
- **핵심 개념**
  - 노드는 값과 다음 노드의 주소로 이루어집니다.
  - Lisp에서는 `'(10 20 30)` 한 줄이면 되는 것을, C에서는 struct, `malloc`, 포인터로 직접 지어야 합니다.
  - `map`과 `apply`는 함수 포인터로 구현합니다.
- **접근**
  - 오른쪽에 Lisp REPL을 띄워 같은 연산을 먼저 보여 주고, 왼쪽에서 C로 옮깁니다.
  - 처음엔 노드 세 개를 손으로 연결해 구조를 눈으로 보여 줍니다.
  - 그다음 `create`/`append`로 일반화합니다.
  - 마지막 말은 "this very hard to implement in C"입니다(`eval`을 두고 한 말).

### 08. 동적 배열 (19:34)
- **임무**: 원소를 계속 추가할 수 있는 배열을 만드는 것입니다.
- **핵심 개념**
  - 길이(`len`, 식탁에 앉은 사람 수)와 용량(`cap`, 의자 수)은 다릅니다.
  - 꽉 차면 `realloc`으로 용량을 두 배로 늘립니다.
  - `void *` 데이터와 원소 크기만 알면 int든 문자열이든 담을 수 있습니다.
- **접근**
  - 고정 배열 → `malloc` → `realloc` → struct → 범용 버전 순으로 키웁니다. 단계마다 "HARD", "HARD HARD"라는 이름을 붙입니다.
  - 마지막 단계("HARD HARD HARD - You implement it")는 시청자 과제로 남깁니다.

### 09. 해시 테이블 (33:52)
- **임무**: 해시 테이블을 이해하는 것입니다.
- **핵심 개념**: 키를 숫자로 바꿔(해시) 배열의 칸 번호로 씁니다. 같은 칸에 키가 겹치면(충돌) 연결 리스트로 이어 붙입니다.
- **접근**
  - ChatGPT에서 받은 코드를 옆 창에 교재처럼 띄워 둡니다.
  - 왼쪽에서 `int arr[5]`부터 다시 짭니다. 순서는 숫자 키 → 나머지 연산 → 노드 체인 → 문자열 키 → `set`/`get`입니다.
  - AI 코드는 한 줄도 붙여 넣지 않습니다. 리스트 순회는 자기 07번 영상을 다시 보고 가져옵니다.
  - 자기 해시가 "abc"와 "cba"를 같은 칸에 넣는다는 약점도 스스로 메모합니다.

### 10. 사람은 AI보다 어떻게 더 잘 배우나 (11:37)
코딩이 아니라 주장입니다. 메모를 한 줄씩 고쳐 쓰며 말합니다.
- **주장**: AI는 복잡한 것에서 배우고, 사람은 단순한 것에서 배웁니다. 사람의 문제는 복잡함과 추상화를 좋아한다는 것입니다.
- **증거**: Rust `tree` 저장소 대 자기의 C 55줄, 위키백과의 해시 테이블 대 "Hash table is just key - value". 자기 README로 Gemini가 동작하는 시뮬레이션을 만드는 것도 보여 주며 "it work / AI love complex / your brain not"이라고 씁니다.
- **답**: 처음부터 시작하고, 추상화하지 말고, 모든 것을 단순하게 만들면 AI보다 잘 배울 수 있다. 02~09번 영상이 왜 그렇게 작은지에 대한 작가 자신의 설명입니다.

### 11. `ed` 100줄로 만들기 (1:10:04)
- **임무**: 유닉스 줄 편집기 `ed`의 핵심(`a`, `.`, `p`, 줄 번호, `d`, `w`, `q`, 파일 열기)을 C로 만드는 것입니다.
- **핵심 개념**: 문서는 줄 포인터 배열(`char *lines[]`)이고, 편집기는 "명령 모드 / 입력 모드" 두 상태를 오가는 `fgets` 루프입니다.
- **접근**
  - ChatGPT가 만든 549줄을 열어 보이고 "too complicated"로 치웁니다.
  - 진짜 `ed`를 시연하고, 짜는 동안에도 옆 터미널에서 같은 입력을 넣어 비교합니다. 오류 문구까지 진짜 `ed`에서 가져옵니다.
  - man 페이지와 Beej의 레퍼런스를 읽으며 명령을 하나씩 붙이고, 컴파일러 경고를 하나씩 고칩니다.
  - `d`를 짜기 전에 빈 vim 버퍼에 줄 옮기는 그림을 먼저 그립니다.
  - 시연 범위 밖의 결함(마지막 줄 삭제 뒤의 위치, 끼워 넣기 대신 덮어쓰기)은 그대로 남깁니다.

### 12. 나는 AI가 아니다, 내 이야기 (15:43)
작가의 개인사입니다. "사람들이 내가 AI가 만든 사람이라고 한다"는 메모로 시작해, 노트 앱의 연도별 기록으로 지금까지를 보여 줍니다. 대학, 회사, 비자유 소프트웨어를 차례로 떠났고, 지식과 나눔을 돈보다 앞에 두었으며, 농장을 하며 밖에서 코딩합니다. AI가 소프트웨어를 바꾸는 것을 보고 채널을 만들었습니다. 결론은 "AI가 싫은 것도, AI가 일을 빼앗는 것도 아니라, 사람을 묶는 체계가 늘 싫었다"입니다. "No AI"가 기술 혐오가 아니라 일관된 삶의 태도에서 나온다는 것을 알려 주는 영상입니다.

### 13. C는 평평하다 (18:10)
- **임무**: 메모리와 C가 왜 "평평한" 모델인지 보여 주는 것입니다.
- **핵심 개념**: 메모리는 주소가 붙은 칸의 나열이고, 비트를 숫자로 읽는 것, 칸 묶음을 "사람"으로 부르는 것, 구조체, `#include`가 모두 그 위에 사람이 정한 약속이라는 주장입니다.
- **접근**
  - 스프레드시트에 메모리를 그리고, 같은 그림을 C 배열로 옮깁니다.
  - 시연 코드는 Gemini에게 쓰게 하고 그대로 실행합니다. 다만 뼈대는 자기가 먼저 쓰고, Gemini가 struct로 답하자 "don't use struct"라며 평평한 배열로 되돌리게 합니다.
  - 중첩 함수, 중첩 구조체, `import`를 C로 시험해 "평평하지 않은 것은 C에 없거나 평평하게 바뀐다"를 보여 줍니다.

### 14. VSCodium, 장식 없이 (5:39)
VS Code에서 마이크로소프트 원격 측정과 라이선스를 뺀 공개 빌드 VSCodium을 설치하고, 활동 표시줄·상태 표시줄·탭·미니맵·줄 번호·아이콘·패널 탭을 차례로 끕니다. 남는 것은 글자와 터미널뿐이고, 그 창에서 xv6 커널 소스를 읽고 Hello World를 짭니다. 마지막 주석은 "Look good now, beauty, clean". "but better"는 기능을 더한 것이 아니라 뺀 것입니다.

### 15. macOS, Apple 없이 (6:06)
14번의 운영체제판입니다. `defaults write`로 둥근 모서리와 떠 있는 사이드바를 없애고, 색과 투명도를 줄이고, Apple 계정에 로그인하지 않습니다. 외부 연결을 막는 공개 방화벽 LuLu를 설치해 `*.apple.com`과 `*.icloud.com`을 차단합니다. 메모는 "apple can't track you anymore", 그리고 "this is clean mac". 기기를 버리는 것이 아니라 기기의 주인이 되려는 것입니다.

### 16. C 문자열 라이브러리 다시 짜기 (19:57)
- **임무**: `<string.h>`의 `strlen`, `memcpy`, `strcpy`, `strncpy`, `strcat`, `strcmp`를 손으로 다시 짜는 것입니다.
- **핵심 개념**: 문자열은 `\0`으로 끝나는 바이트 칸이고, 포인터 한 개로 걸으면 대부분의 함수가 한두 줄입니다(`while (*p++)`, `while ((*p++ = *src++))`).
- **접근**
  - Beej 레퍼런스에서 원형을 읽고, 본문을 손으로 쓰고, 시험합니다.
  - 포인터 방식과 첨자 방식을 나란히 써 본 뒤 하나를 지웁니다.
  - 스프레드시트에 글자 칸과 `\0`을 그려 `- 1`의 이유를 설명합니다.
  - Beej 가이드의 오류를 찾아 메모로 남기고 man 페이지로 확인합니다.
  - 검증해 보면 함수 5개는 libc와 같고, `hstrncpy`만 짧은 원본에서 끝을 넘어 읽습니다(시연 범위 밖).

### 17. 순수 HTML이 AI 슬롭을 이긴다 (10:01)
요즘 사이트들이 서로 닮은 것을 "slop by AI"로 부르고, 스프레드시트에 AI가 만든 사이트 다섯 개와 손으로 만든 "Legend" 개인 사이트 스무여 개를 나란히 적습니다(bellard.org, paulgraham.com, templeos.org 등). 논제는 "Your website is your soul, it must reflect your personality". 자기 사이트는 CSS 한 줄짜리 HTML 26줄이고, 도메인을 사서 정적 파일로 올리는 과정까지 보여 줍니다. "Boom / all done / that it, simple, clean, work".

### 18. 순수 JS로 웹 편집기와 파일 관리자 (1:28:00)
- **첫 메모**: 논쟁에 지쳐 당분간 의견 대신 프로그래밍만 하겠다, 웹 개발을 1년 쉬어 JS와 Go를 다 잊었으니 다시 배우겠다.
- **web-editor (0~26분)**: 왼쪽 편집기, 오른쪽 `<iframe>`, Run 버튼. 편집기는 textarea에서 Ace(CDN)로 바꾸고, 실행은 `iframe.srcdoc = editor.getValue()` 한 줄. 자기 JS는 12줄입니다.
- **Go 우회 (26~34분)**: 파일 관리자를 Go로 시작했다가 `io/fs`의 `fsys`에 "it look like zig"라고 적고 "i'm done with Go"로 접습니다.
- **web-filemanager "HFiles" (34~88분)**: 업로드, 파일마다 탭, 이미지·동영상 미리보기, localForage(IndexedDB)로 새로 고침 뒤에도 유지. 주석 `// can it storage these files forever`가 마지막 20분의 질문입니다.
- **접근**: W3Schools·MDN·Stack Overflow·라이브러리 문서에서 예제를 가져와 고쳐 씁니다. CSS는 devtools에서 먼저 맞춘 뒤 소스로 옮깁니다. 프레임워크와 빌드 도구는 없습니다.

### 19. 사람이 AI 슬롭을 이긴다 (4:06)
"I am not anti-tech. I've used AI from day one"으로 시작해 Zero AI를 "요점을 증명하려는 극단적 실험"이라고 부릅니다. 분홍 메모에 반대편 주장("The myth: AI faster…")을 적고, YouTube Studio를 열어 답합니다: 28일 조회 611,056, 구독자 +9.4K, 실시간 10,845명. 2주 동안의 일과(매일 배우고, 처음부터 만들고, 댓글을 다 읽는다)를 적은 뒤 결론은 "When content is cheap, attention fades and trust dies. … just pure Human - pure Video beat AI".

### 20. AI가 위조할 수 없는 것 (3:11)
"채널 전체가 AI로 만든 것 같다"는 시청자 메일로 시작합니다. 영상도 사이트도 AI로 만들 수 있다고 인정한 뒤 "what AI cannot fake / that is history"를 적고, Wayback Machine에서 옛 사이트의 2024년 보관본(27회 저장)과 2024년 GitHub 저장소를 엽니다. 메모 "AI can't turn back time", 결론 "So pure HTML with history beats AI again". 첫 장면의 웹캠 명령 이름이 `scam start`인 것도 같은 농담입니다.

### 21. Go 조금, 재미로 (1:29:57)
- **메모**: "Learn philosophy of language don't learn syntax / all you need is just go.dev websites", 그리고 "Go is just C with garbage collector and great library builtin".
- **접근**: Tour of Go를 읽으며 같은 개념을 `main.go`와 `main.c`에 나란히 씁니다(slice는 08번의 동적 배열).
- **만든 것**: `net/http`로 정적 파일 + JSON API(`/api/send`), 오류 처리를 고치며 패닉 두 번. 실시간이 필요해지자 gorilla/websocket chat 예제를 들여와 WebSocket 채팅으로 바꾸고, 30분 들인 JSON API는 지웁니다.
- 정적 읽기: 지우기 직전의 다리 코드(`client.WritePump()`를 처리기에서 직접 호출)는 한 연결에 쓰기 루프를 둘 만드는 버그였습니다. 마지막 화면의 `main.go`는 지운 처리기를 아직 가리킵니다.

### 22. Firefox, 장식 없이 (3:46)
14번(편집기), 15번(macOS)의 브라우저판입니다. 첫 동작이 "Block AI enhancements"이고, 데이터 수집을 모두 끄고, 새 탭을 빈 페이지로 둡니다. 마지막으로 `about:config`에서 사용자 스타일시트를 켜고, Browser Toolbox로 Firefox 화면의 id를 찾아 `userChrome.css` 세 줄로 탭 막대와 뒤로·앞으로 버튼을 없앱니다.

### 23. Go로 wget (24:57)
`http.Get`으로 시작해 User-Agent 헤더, `x/net/html`로 링크 뽑기, 재귀, `url.Parse`로 저장 경로를 만듭니다. 링크가 끝없이 늘어나는 문제를 접두사 필터로 막고, 저장된 옛 사이트를 오프라인으로 열어 확인한 뒤 `// done - simple`. 방문 기록이 없고 상대 링크를 페이지 기준으로 풀지 않아, 일반 사이트에서는 끝없이 돌 수 있습니다(정적 읽기).

### 24. w3m을 써 보라 (10:59)
스티커 메모를 목차로 쓰며 텍스트 브라우저의 기능을 하나씩 보여 줍니다: "ignores JavaScript", 탭, 도움말, 설정, 페이지 안 검색, 북마크. 결론은 "i often use w3m like this": Beej's Guide를 내려받아 터미널에서 읽고, `crawl` 별칭(wget)으로 사이트를 통째로 받아 오프라인으로 읽고, 설정과 기록도 텍스트 파일("everything storage in files"). 작가의 `.zshrc`가 나와 `cc`가 엄격한 경고 옵션을 붙인 clang 별칭임을 확인합니다.

### 25. AI에 넘기기 좋은 `cat` (27:32)
- **동기**: 코드를 AI에 넘길 때 파일을 하나씩 복사하고 이름을 적는 수고. 진짜 `cat`은 파일 이름이 없고, 개행 없는 파일 끝이 다음 파일과 붙고, 폴더에 들어가지 못합니다.
- **만든 것**: 파일마다 머리줄을 붙이고, 숨김 파일을 건너뛰며 폴더를 재귀로 도는 92줄. `fgetc` → `fread` 버퍼, V1(이름) → V2(재귀). 재귀는 자기 옛 영상(05, 02)을 다시 틀어 보고 짭니다.
- **AI**: 첫 메모 "The AI mentioned here is merely a meme and one possible use case… The main focus is learning about File I/O in C". 결과를 Gemini가 읽고, 마지막 버그는 Gemini의 답을 보기 전에 고친 뒤 `// i know it before gemini check`.
- **검증**: `./a.out .`은 아무것도 출력하지 않습니다(인자로 받은 `.`도 숨김 파일로 처리).

### 26. C로 `cal` (28:31)
진짜 `cal`을 명세이자 시험지로 씁니다. `man mktime`의 "필드를 정규화한다"는 한 문장으로 요일(`tm_wday`)과 달의 날 수("다음 달 0일")를 얻어, 윤년이나 요일 공식을 직접 짜지 않습니다. `tm_year`의 1900과 `tm_mon`의 0을 한 줄 함수 다섯 개에 가둡니다. 오늘 날짜는 반전 표시. 세 열짜리 연간 달력은 셸 메모로 과제를 남깁니다("you do that"). 검증: 마지막 저장과 바이트까지 같고, 899개월이 Python 달력과 일치합니다.

### 공통된 패턴 (01–26 기준)
1. 완성된 것(진짜 `tree`·`ed`·`cat`·`cal`, Lisp, Wikipedia, 표준 라이브러리, AI 답, Vue Playground)을 먼저 보고 목표를 정합니다. 진짜 도구는 끝까지 비교 기준으로 씁니다(26). 모르는 것은 1차 문서의 예제를 붙여 넣고 고쳐 씁니다(18, 21).
2. 가장 단순한 버전부터 손으로 짜서 한 단계씩 키웁니다. 단계마다 컴파일하고 실행합니다.
3. 자료의 모양을 먼저 그립니다(스프레드시트의 메모리, vim 버퍼의 줄 이동 그림).
4. AI는 비교 대상이거나(09, 11), 타자수이거나(13), 결과물을 읽는 쪽입니다(25). 어느 쪽이든 이해와 설계는 작가가 쥐고 있습니다. 10번과 12번이 그 이유를 직접 말하고, 19번과 20번은 결과(통계)와 기록(보관본)으로 증명하려 합니다.
5. 도구에서도 같은 원칙을 지킵니다: 장식과 AI 기능을 끄고(14, 15, 22), 원격 측정과 제조사 연결을 끊고(14, 15, 22), 웹은 HTML만으로 충분하다고 보고(17), 텍스트 브라우저로 오프라인에서 읽습니다(24).
6. 맞지 않으면 버립니다: Go(18), 30분 들인 JSON API(21), 로컬 Ace(18).
7. 마지막 한 단계는 시청자에게 과제로 넘깁니다.

## 영상별 폴더 구조

```
NN_<slug>_<videoId>/
  README.md        분석 문서: 신뢰도, 최종 코드, 타임라인, 코드 해설, 한계, 해석
  src/             복원한 최종 산출물 (코드, 노트, 세션 스크립트)
  history/         저장 시점별 버전
  transcripts/     구간별 프레임 판독 원문 (근거 자료)
  verify/          체크섬 대조와 조립 스크립트 (필요한 경우에만)
```

코딩 산출물이 없는 잡담이나 시연 영상은 `src/` 없이 `README.md`에 내용을 글로 정리합니다. 새 영상을 분석하면 위 표에 한 줄, "영상별 요약"에 한 항목을 추가하고 "공통된 패턴"도 함께 고칩니다.
