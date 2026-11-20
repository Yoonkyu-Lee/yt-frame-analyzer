# Hayyoth의 손코딩 철학: 영상에서 관찰한 원칙

Hayyoth 채널의 손코딩 영상을 해체 분석하며 쌓아 가는 원칙 모음입니다. 이 채널 하나에서 관찰한 것만 적습니다. 여러 출처에 걸친 종합은 [northstar](../../northstar/)에서 합니다.

**작성 규칙**
- 원칙마다 **관찰**(화면에서 확인한 사실, 영상 태그와 타임스탬프)과 **해석**(왜 그렇게 했는지에 대한 추론)을 나눠 적습니다.
- 새 영상에서 같은 행동이 나오면 관찰을 추가하고 `근거 영상 수`를 올립니다. 반대되는 행동이 나오면 **반례**에 적습니다.
- 근거 영상이 1개인 원칙은 "가설"입니다. 2개 이상이 되면 "패턴"으로 봅니다.

**영상 태그** (Hayyoth 채널, 업로드 순)

| 태그 | 영상 | 언어 | 만든 것 |
|---|---|---|---|
| `[intro]` | [01](01_typing-by-hand_WLih6RtW_hE/) | C | 채널 선언문. `No AI --- ` 무한 출력 (10줄) |
| `[tree]` | [02](02_c-tree-viewer_tiMHtc6l7xM/) | C | `tree` 같은 디렉터리 뷰어 (58줄) |
| `[walk]` | [03](03_c-random-moving-numbers_IJe5WtpXx5c/) | C | 입자 무작위 이동 + 먹이 규칙 (87줄). **AI 사용 ①: 써 보고 버림** |
| `[lisp]` | [04](04_lisp-intro_5ulMKWo2wJI/) | Scheme | REPL 시연 강의 + 강의 목차 |
| `[utils]` | [05](05_c-linux-file-utils_gQRmSlcTq9c/) | C | touch, cat, mkdir, mv, rm, find (단일 파일 6개) |
| `[lab]` | [06](06_c-simulation-lab_Kb0YFATY3ug/) | C | 저자의 시뮬레이션 프레임워크 `hayyoth-lab` 시연 (공개 저장소와 바이트까지 일치) |
| `[list]` | [07](07_c-linked-list_iz2j9_AwQdQ/) | C + Scheme | 연결 리스트, `map`/`apply` (60줄). Lisp와 나란히 비교 |
| `[slice]` | [08](08_c-dynamic-array_SOjHybBunAc/) | C | 타입을 가리지 않는 동적 배열 (76줄). HARD 단계로 진행 |
| `[hash]` | [09](09_c-hash-table_61XUoywg970/) | C | 해시 테이블 (67줄). **AI 사용 ②: ChatGPT 코드를 교재로 다시 짬** |
| `[learn]` | [10](10_human-learn-better_48DqkAx86Vg/) | — | 주장: 사람은 단순한 것에서 배운다 (메모로 말하는 영상) |
| `[ed]` | [11](11_c-ed-editor_MKmPyByntzA/) | C | 줄 편집기 `ed` 클론 (113줄). **AI 사용 ③: ChatGPT의 549줄을 보여 주고 거절** |
| `[story]` | [12](12_my-story_gLn7F22HngI/) | — | 작가의 이야기: 왜 손으로 코딩하는가 |
| `[flat]` | [13](13_c-is-flat_wj9os_e8Hs4/) | C | 강의: 메모리와 C는 평평하다. **AI 사용 ④: Gemini가 쓴 시연 코드를 실행, 설계는 되돌림** |
| `[codium]` | [14](14_vscodium_rc5dnqN2IXc/) | — | VSCodium 설치, 장식을 모두 끈 편집기 |
| `[noapple]` | [15](15_macos-no-apple_abLEkywTtD4/) | — | macOS 장식 줄이기 + LuLu 방화벽으로 Apple 연결 차단 |
| `[string]` | [16](16_c-string-library_72bBuM5tKN0/) | C | `<string.h>` 함수 6개 다시 짜기 (55줄) |
| `[html]` | [17](17_pure-html_fG7mM6Yymaw/) | HTML | "AI slop" 사이트 비판 + 손으로 쓴 개인 사이트 (26줄) |
| `[webjs]` | [18](18_pure-js-web-editor_VBORE_zH5oc/) | JS | 웹 편집기(Ace + iframe) + 파일 관리자 "HFiles"(localForage). Go로 시작했다 접음 |
| `[proof]` | [19](19_human-beats-slop_uRLMMYFoGB4/) | — | 논증: "Zero AI" 실험의 결과를 채널 통계로 |
| `[history]` | [20](20_ai-cannot-fake_TAEBfnNUYAc/) | — | 논증: AI가 위조할 수 없는 것은 보관된 기록 |
| `[go]` | [21](21_golang-for-fun_avofMrOfsWI/) | Go | Go 다시 배우기 → `net/http` + WebSocket 채팅 |
| `[firefox]` | [22](22_firefox_TJRWmmVLIyo/) | — | Firefox 설정: AI 기능 차단 + `userChrome.css` |
| `[wget]` | [23](23_go-wget_g4SQ1LGRI00/) | Go | 사이트를 통째로 저장하는 크롤러 (약 80줄) |
| `[w3m]` | [24](24_w3m_6DQCtRSTfNw/) | — | 텍스트 브라우저 시연 + `.zshrc` |
| `[cat]` | [25](25_c-cat_ie0PfrXZsc/) | C | 폴더를 재귀로 도는 "AI에 넘기기 좋은" `cat` (92줄) |
| `[cal]` | [26](26_c-cal_xRZHl1B8wFc/) | C | UNIX `cal` 클론 (98줄) |

---

## A. 시작하기 전에

### A1. 만들 대상의 동작을 먼저 눈으로 확인한다
근거 영상 수: 7 (패턴)

**관찰**
- `[tree 00:00–00:40]` 코드를 쓰기 전에 진짜 `tree`를 실행해 출력 형태를 봅니다. `[tree 16:37]` 재귀를 구현한 직후 다시 실행해 나란히 비교합니다.
- `[walk 16:23–17:03]` Wikipedia "Diffusion" 문서의 입자 확산 GIF를 새 목표 동작으로 삼습니다.
- `[list 00:00]` C로 짜기 전에 Chez REPL에서 `'(10 20 30)`, `append`를 먼저 실행합니다. 이후에도 기능마다 REPL 쪽을 먼저 보여 줍니다.
- `[hash 01:01–01:20]` 브라우저 콘솔에서 JS 객체 `{name:"Cong", age:20}`로 "해시 테이블은 JS에서 이렇게 생겼다"를 먼저 보여 줍니다.
- `[ed 1:29]` 짜기 전에 진짜 `ed`로 `a`, `.`, `w hello.txt`, `q`, 다시 열어 `1`/`2`/`p`/`d`를 시연하고, 명령마다 스티커로 이름을 붙입니다. 짜는 중에도 옆 터미널에 진짜 `ed`를 띄워 같은 입력을 넣어 봅니다. 파일이 없을 때의 오류 문구도 진짜 `ed`의 것을 그대로 가져옵니다.
- `[cat 1:17–1:40]` 진짜 `cat run.js style.css`를 돌려 한계를 먼저 보입니다: 파일 이름이 없고, 개행 없는 파일 끝이 다음 파일과 붙습니다(`copyUrl)#editor {`).
- `[cal]` 시작할 때 `cal`, `cal 10 2026`, `cal 2026`을 돌려 보고, 함수 하나를 만들 때마다 같은 달을 `cal`과 나란히 비교합니다. 요일 머리줄도 `cal`의 출력에서 복사합니다. 진짜 도구가 끝까지 시험지입니다.

**해석**: 명세서 대신 동작하는 기존 것(프로그램, 그림, 다른 언어)을 정답(oracle)으로 씁니다. 요구사항을 문서로 쓰는 대신 "저것처럼"으로 정의하면 범위가 저절로 작아집니다.

### A2. 코드보다 설계 노트를 먼저 쓴다. 단, 아주 짧게
근거 영상 수: 4 (패턴)

**관찰**
- `[tree 00:40–01:11]` `README.md`에 의사코드 4줄을 씁니다. 최종 코드 구조가 이 4줄과 일대일로 대응합니다.
- `[utils 00:00]` 설계 표 `notes.txt`(Command | Function use | Algorithm)가 처음부터 열려 있습니다. Helper 메모(`stat`과 `lstat`의 차이, `ENOENT`, `S_ISDIR`)도 함께 있습니다.
- `[utils 04:40, 26:00]` 어려운 유틸(mv, rm)은 빈 버퍼에 영어 문장으로 사용 예를 먼저 씁니다.
- `[lisp 00:00–]` 강의 목차를 미리 써 두고 그 순서대로 시연합니다.
- `[slice 00:00]` 메모 버퍼에 "This video will help you understand - dynamic array in C - dynamic array in any language"라고 쓰고 시작합니다.

**해석**: 설계 문서는 생각을 정리하는 도구일 뿐 유지보수하는 산출물이 아닙니다. 표의 "Function use" 열이 곧 읽을 man 페이지 목록이고, "Algorithm" 열이 곧 코드 구조입니다.

### A3. 미리 리허설하고 녹화한다
근거 영상 수: 3 (패턴)

**관찰**
- `[list 01:56]` 녹화 초반인데 셸 스크롤백에 완성된 프로그램의 출력과 오류(`sum 600`, 38행·55행 오류)가 이미 보입니다.
- `[slice 01:13]` 스크롤백에 이 파일과 맞지 않는 `-Wsign-compare` 경고가 남아 있습니다.
- `[utils]` 설계 표와 Helper 메모가 녹화 전에 이미 완성되어 있습니다.
- `[lab]` 03번 영상(10-17)보다 먼저(10-03~06) 저자는 Conway/확산 시뮬레이션이 들어 있는 프레임워크를 이미 공개해 두었습니다.

**해석**: 영상 속 시행착오는 꾸민 것이 아니라 실제로 일어난 것입니다(체크섬이 증명합니다). 하지만 무엇을 만들지와 어디서 막히는지는 미리 알고 있습니다. 영상은 "모르는 것을 탐구하는 기록"이라기보다 **"원리를 손으로 다시 재현하는 시연"**입니다.

---

## B. 진행 방식

### B1. 아주 작은 단계로 나누고, 매 단계 컴파일하고 실행한다
근거 영상 수: 7 (패턴)

**관찰**
- `[tree]`, `[intro]`, `[walk]`, `[utils]`: 기능 하나를 추가할 때마다 `cc && ./a.out`을 실행합니다.
- `[list]` 노드 3개를 연결 → 출력 → `create`/`append` → `map` → `apply` 순으로 진행하고, 단계마다 실행합니다.
- `[slice]` 고정 배열 → malloc → realloc → struct → 범용, 단계마다 실행합니다(`11 22 33` → `11 22 33 44` → `Hello World Hayyoth`).
- `[hash]` `int arr[5]` → `arr['a']` → `arr[7 % 5]` → `Node` 배열 → `hash()` → 문자열 키 → `get` → `set`. 각 단계를 컴파일해 확인합니다.

**해석**: 한 번에 바꾸는 범위가 작으니 문제가 생기면 원인이 방금 바꾼 몇 줄 안에 있습니다. 디버거 없이도 버그를 찾을 수 있는 이유입니다.

### B2. 주석(또는 영어 문장)으로 먼저 자리를 잡고, 그 아래를 채운다
근거 영상 수: 5 (패턴)

**관찰**
- `[tree]` `// is dir`, `// is file`, `// walk to sub path`를 먼저 씁니다.
- `[utils]` 메모 버퍼의 영어 문장을 rm.c에 주석으로 그대로 옮기고 그 아래를 구현합니다.
- `[slice]` `// on max size table` / `// so, we need move to big table` / `// you don't want call resize every time call append`를 먼저 쓰고 `if` 블록을 채웁니다.
- `[hash 25:00]` `// this index is empty, just put value to it`, `// if it not empty -> go to last node and add next`를 먼저 쓰고 분기를 채웁니다.
- `[wget]` `// get link in html`, `// create folder`, `// save file content`를 먼저 적고 그 아래를 채운 뒤 주석을 지웁니다. `[cat]`은 스티커에 `- get path / - check path is file or dir / if file -> cat / if dir -> open dir …`를 적고 그대로 `walkcat`을 짭니다.

**해석**: 주석이 할 일 목록 역할을 합니다. 글 → 주석 → 코드 순으로 한 단계씩만 구체화합니다.

### B3. 가장 단순한 고정형으로 먼저 확인하고, 나중에 일반화한다
근거 영상 수: 8 (패턴)

**관찰**
- `[tree]` 들여쓰기를 `3`으로 고정해 확인한 뒤 매개변수로 바꿉니다.
- `[walk]`, `[lisp]` 숫자를 크게 바꿔 가며 실행합니다(9→90, `expt 2 n`).
- `[list]` 스택에 노드 3개를 두고 `n1.next = &n2`로 손수 연결 → `create`/`append`로 일반화합니다.
- `[slice]` `int arr[3]` → `HARD VERSION`(int Slice) → `HARD HARD VERSION - Any data type` → `HARD HARD HARD VERSION - You implement it`
- `[hash]` `int arr[5]` + 숫자 키 → `% SIZE` → `Node` 체인을 손으로 연결 → 문자열 키 → `set`/`get`으로 일반화합니다.
- `[cat 21:30–23:20]` `strrchr`로 파일 이름을 뽑는 방법을 본 파일이 아니라 따로 `test.c`(12줄)에서 `"abcdef.secret.txt"`로 시험한 뒤 세 줄을 옮깁니다. `[cal]`은 `printf("year = %d
", year)` 스텁을 먼저 두고 나중에 `print_year`로 바꿉니다.

**해석**: "구조가 맞는가"와 "일반적으로 동작하는가"를 따로 검증합니다. 단계 이름(HARD, HARD HARD)을 붙여 시청자가 어디까지 따라왔는지 알게 합니다.

### B4. 새 것은 이미 있는 자기 코드를 복사해서 시작한다
근거 영상 수: 6 (패턴), 반례 1

**관찰**
- `[utils]` touch → cat → mkdir → mv, cat → rm → find. 매번 직전 파일 전체를 복사합니다. "Useage" 오타도 함께 퍼집니다.
- `[walk]` `init`의 이중 루프 머리를 `render`에서 복사합니다.
- `[list 10:11]` `map`을 4줄 복사해서 `apply`로 고칩니다.
- `[hash 18:25–28:12]` 자기 07번 영상을 다시 틀어 `append`의 `while (p->next) p = p->next;`를 보고 `set`에 옮깁니다.

**반례**
- `[lab]` 저자의 실제 프로젝트 `hayyoth-lab`은 모듈로 나뉘어 있습니다: `src/hcell.{c,h}`(계산), `src/hrender.{c,h}`(그리기), `lab/*.c`(시뮬레이션). Makefile에는 `run-<name>` 타깃이 있습니다. 복사 대신 **라이브러리로 재사용**합니다.
- `[cat 12:06–19:04]` 재귀 순회를 짜기 전에 자기 옛 영상(`[utils]`의 `walkrm`, `[tree]`)을 유튜브에서 다시 틀어 놓고 따라 씁니다. 스티커: `check my previous video`.
- `[wget]` 파일 저장 코드는 자기 옛 Go 저장소(gfm)의 `MkdirAll`/`WriteFile`을 열어 보고 씁니다.

**해석**: 영상 속 예제는 파일 하나로 완결되도록 일부러 복사를 택합니다. 시청자가 파일 하나만 보면 되기 때문입니다. 실제로 오래 쓸 코드는 모듈로 나눕니다. 복사냐 라이브러리냐는 **"누가, 얼마나 오래 읽는가"**에 따라 정하는 것으로 보입니다.

### B5. 쉬운 기준 모델을 먼저 보여 주고, C로 내려간다
근거 영상 수: 6 (패턴)

**관찰**
- `[list]` Lisp에서는 한 줄인 `append`, `map`, `apply`를 C에서는 struct, `malloc`, 함수 포인터로 짓습니다. 끝은 **`; this very hard to implement in C`**(`eval`)입니다. 영상 설명: "Lisp makes it look easy, but in C, you have to build the whole world from scratch"
- `[hash]` JS 객체 → Wikipedia "Collision resolution" → 배경화면의 버킷 그림 → C 순서로 내려갑니다.
- `[slice 03:52]` 최후의 만찬 식탁 그림(사람 수 = `len`, 의자 수 = `cap`)을 보여 준 뒤 struct를 씁니다. 구조체 주석에도 비유가 남습니다("such as table have 6 person").
- `[flat]` 스프레드시트에 주소와 비트 칸을 그려 "메모리"를 보여 준 뒤 `int bytes[4]`로 옮기고, 실행 결과를 다시 시트의 행에 맞춰 봅니다.
- `[ed 59:20]` `d`(줄 삭제)를 짜기 전에 빈 vim 버퍼에 `1 this is line 1 / 2 xxxx // current line - delete it / … move from 2 to 3 / lines[i] = lines[i + 1]`을 먼저 적어 옮기는 순서를 그립니다.
- `[string 3:27]` 스프레드시트 "hello" 표에 글자 칸과 `\0`, 셀 메모(`*p == 0 here`)를 달아 `while (*p++)`가 한 칸 더 가는 이유(`- 1`)를 그림으로 설명합니다.

**해석**: 고수준 언어나 그림으로 "무엇"을 먼저 보여 주고, C로 "어떻게"를 짓습니다. C는 추상을 공짜로 주지 않는다는 점을 대비로 드러냅니다.

---

## C. 지식을 얻는 방법

### C1. 1차 자료를 직접 읽는다
근거 영상 수: 12 (패턴), 반례 2

**관찰**
- `[tree]` POSIX 명세 Issue 8, `man 3 strcmp`, `man snprintf`
- `[utils]` 시스템 함수마다 man 페이지를 엽니다. 장(section) 번호가 틀리면 다시 찾습니다(`man 2 opendir` → `man opendir`).
- `[lisp]` 정전 교재 두 권(SICP, TSPL4)을 추천합니다.
- `[slice]` `strcmp` → `man string` → `strcpy` → `man 3 memcpy`(겹치는 영역은 정의되지 않음, `memmove` 언급까지 읽음)를 거쳐 `memcpy`를 고릅니다.
- `[hash 03:30–06:11]` Wikipedia "Hash table"의 "Collision resolution" 절

**반례**: **E1** 참고(`[walk]`, `[hash]`에서 ChatGPT 사용).

- `[ed]` `man 3 fgets / strcspn / strcmp / strdup / fopen / strncmp / strcpy`와 Beej의 C 라이브러리 레퍼런스(`strtol`의 `endptr` 예제, `strncpy`)를 읽은 직후에 그 함수를 씁니다.
- `[string 13:04]` Beej의 `strcat()` 설명을 읽다가 메모 **`spotted , beej guide wrong here`**, `man 3 strcat`으로 확인합니다. 1차 자료도 그대로 믿지 않고 다른 1차 자료로 대조합니다.
- `[webjs]` 막힐 때마다 문서를 엽니다: W3Schools(두 열, `<video>`), MDN(`<input type="file">`, IndexedDB, Web Storage), Ace 문서(`getValue`), localForage 문서(`keys`, `getItem`). 첫 메모가 "JS와 Go를 다 잊었다"입니다.
- `[go]` 메모 **`Learn philosophy of language don't learn syntax / all you need is just go.dev websites`**. Tour of Go → pkg.go.dev(std, `net/http`, `encoding/json`) → go.dev wiki 순서로 읽습니다. 다만 dev.to, Stack Overflow, 개인 블로그도 섞어 씁니다.
- `[cat]` `man 3 fgetc` → `man 3 fread` → `man strrchr`. `[cal]` `man 3 time` → `man 3 localtime`/`mktime`에서 "필드를 정규화한다" 문단을 마우스로 골라 둔 뒤, 그 한 문장으로 요일과 달의 날 수를 계산합니다.
- `[w3m]` 도구 소개도 `man w3m`의 NAME 줄을 복사하는 것으로 시작합니다. 시연 대부분이 Beej's Guide를 내려받아 터미널에서 검색해 읽는 것입니다.

**해석**: 명세와 man 페이지가 먼저입니다. 언어를 배울 때는 정전 교재를 권합니다.

### C2. 예제를 복사하지 않고 자기 방식으로 다시 쓴다
근거 영상 수: 3 (패턴), 반례 3 (`[webjs]`, `[go]`, `[wget]`)

**관찰**
- `[tree]`, `[utils]` POSIX 예제와 man 예제를 자기 변수명과 오류 처리(`perror(path); return 1;`)로 바꿔 씁니다.
- `[hash 14:00–33:04]` ChatGPT 코드를 옆 창에 띄워 놓고도 **한 줄도 붙여 넣지 않습니다.** `get`을 9줄 yank하지만 붙여 넣지 않고, 자기 이름(`size_t index`, `const Node *p`)으로 직접 타이핑합니다.
- **반례** `[webjs]` W3Schools 두 열 예제를 데모용 주석("Should be removed. Only for demonstration")째 붙여 넣고 고쳐 나갑니다. Stack Overflow의 iframe 답, MDN의 파일 입력 예제(변수명 `elem`을 그대로 둔 주석), localForage 문서 예제도 붙여 넣은 뒤 고칩니다.
- **반례** `[go]` MDN fetch 예제를 거의 그대로, gorilla/websocket의 chat 예제 두 파일(약 200줄)은 라이선스 주석까지 그대로 들여옵니다. 붙여 넣은 뒤 바꾼 것은 패키지 이름, export(`ServeWs`, `WritePump`), 반환값 정도입니다.
- **반례** `[wget]` pkg.go.dev의 Client 예제와 `html.Parse` 예제를 붙여 넣고(주석 `// handle error`까지) 필요 없는 줄을 지웁니다.
- `[cat]`, `[cal]` (C) 문서 예제를 붙여 넣지 않고 man 페이지를 읽은 뒤 직접 씁니다. 붙여 넣는 것은 **자기 코드**(옛 영상, `test.c`의 세 줄)뿐입니다.

**해석**: 남의 코드(명세 예제든 AI든)는 읽는 대상이지 붙여 넣는 대상이 아닙니다. 자기 손으로 다시 쓴 코드만 남깁니다.

단, 이 원칙은 **C로 개념을 가르치는 영상**에 해당합니다. "다시 배우는" JS·Go 영상(`[webjs]`, `[go]`)에서는 사람이 쓴 문서 예제를 붙여 넣고 고쳐 씁니다. 두 경우 모두 AI 코드는 0줄입니다. 그러니 경계는 "복사하느냐"보다 "누가 쓴 것이냐"(사람의 문서냐, AI냐)와 "무엇을 배우는 영상이냐"(개념이냐, 도구 사용법이냐)에 있어 보입니다.

### C3. 컴파일러 경고를 최대로 켜고, 경고를 공부 계기로 삼는다
근거 영상 수: 10 (패턴)

**관찰**
- `[intro]`, `[tree]` `cc`가 실제로는 `clang -Wall -Wextra -pedantic -Wno-unused-parameter` alias입니다(탭 제목으로 확인).
- `[tree]` `\e`→`\033`, `%.*c`→`%*c` / `[walk]` `-Wstrict-prototypes` / `[utils]` `<errno.h>`, `strrchr`, `snprintf`
- `[list 03:12]` `for (; p; p->next)` → 경고 2개(`-Wfor-loop-analysis`, `-Wunused-value`) → `p = p->next`. 05:59 `sizeof Node` → 오류 → `sizeof(Node)`
- `[slice 16:59, 17:17]` `len` 미선언, 지운 `sl_print`를 아직 호출함 → 둘 다 수정
- `[hash]` `-Wint-conversion`을 보고 "키는 문자열, `hash()`는 인덱스일 뿐"이라는 구분을 배웁니다. 31:19에는 `const` 관련 오류 4개를 고칩니다.
- `[ed]` 영상 후반(40~68분)의 대부분이 저장 → `cc main.c` → 경고 → 수정의 반복입니다: `sizeof(path)` → `sizeof(filename)`, `%zu`, `fprintf` 인자 순서, `strncmp`에 빠진 `== 0`.
- `[string]` `const` 누락 경고, `-Wparentheses`(대입을 조건에 쓸 때 괄호를 겹침), libc 내장 함수와 이름 충돌(`h` 접두사를 붙인 이유)을 하나씩 고칩니다.
- `[w3m 9:17]` `.zshrc`가 화면에 나와 `alias cc="clang -Wall -Wextra -pedantic -Wno-unused-parameter -Wno-unused-varia…"`를 직접 확인합니다. `[cat]`, `[cal]`의 `cc main.c`도 이 별칭입니다.

**해석**: 경고 하나하나가 "내가 모르는 규칙이 있다"는 신호입니다. 경고를 보고 개념을 고칩니다.

### C4. 새 언어는 아는 언어(C)에 한 줄씩 대응시켜 배운다
근거 영상 수: 1 (가설)

**관찰**
- `[go 0:00–19:00]` 편집기를 둘로 나눠 `main.go`와 `main.c`에 같은 것을 씁니다. 주석 `// go [name] [type]` / `// c [type] [name]`, 배열 `[3]int{…}` / `int arr[3]`. slice에는 `// check my previous video for dynamic array`(`[slice]`)를 답니다.
- `[go 15:18]` 결론 메모: **`Go is just C with garbage collector / and great library builtin`**.

**해석**: 문법을 외우지 않고, 이미 바닥까지 아는 언어에 새 언어를 대응시킵니다. "언어의 철학을 배운다"는 메모의 방법입니다. C를 기준점으로 쓰는 B5와 같은 방향입니다.

---

## D. 코드 스타일

### D1. 추상화는 필요해질 때까지 만들지 않는다
근거 영상 수: 9 (패턴), 반례 1 (B4와 같은 `[lab]`)

**관찰**
- `[tree]` `#define BLUE`를 썼다가 되돌립니다. 헬퍼 함수는 하나뿐입니다.
- `[walk]` `int` 하나로 상태 세 가지(0은 빈칸, 양수는 입자 수, 음수는 먹이)를 표현합니다.
- `[utils]` 유틸 6개가 각각 독립된 단일 파일입니다.
- `[list]`, `[slice]`, `[hash]` 각각 파일 하나, `main` 하나로 끝납니다. `[hash]`는 ChatGPT의 `SIZE 101` 전역 테이블 대신 `SIZE 5` 지역 배열을 씁니다. 해시도 djb2 대신 글자 코드의 합을 씁니다.

**반례**: `[lab]` `void *` + `cellSize`, 함수 포인터 3개(`init`/`rule`/`view`), 계산과 그리기 모듈 분리. 영상 밖의 실제 코드는 추상화를 충분히 씁니다.
- `[webjs]` 탭을 만드는 코드는 업로드 처리기 안에 있다가, 새로 고침 뒤 다시 그리는 `initFiles`가 같은 일을 하게 되자 그때 `appendFile(file, tabs, detailCtn)`로 뽑힙니다(83~85분).
- `[go]` 30분 들여 만든 JSON API(`MessageReq`/`MessageRes`, 오류 처리)를 WebSocket이 같은 일을 하게 되자 통째로 지웁니다(주석 `// don't need this any more`). `[webjs]`에서도 Go 파일 관리자를 "i'm done with Go"로 접습니다.
- `[cal]` 날짜 계산을 직접 짜지 않고 `mktime`의 정규화에 맡깁니다(그 달 1일 → `tm_wday`, 다음 달 0일 → 지난달 마지막 날). `tm_year`의 1900, `tm_mon`의 0은 한 줄짜리 함수 다섯 개에 가둡니다.
- `[wget]` 방문 기록, 상대 경로 풀기, CSS·이미지 내려받기 없이, 자기 옛 사이트를 저장하는 데 필요한 만큼만 만들고 `// done - simple`.

**해석**: 영상 속 코드는 "이해할 수 있는 최소"를 목표로 합니다. 추상화가 정말 필요한 범용성(여러 시뮬레이션, 여러 출력 방식)이 생기면 저자도 추상화를 씁니다.

### D2. 최소한의 언어 기능과 일관된 형식을 지킨다
근거 영상 수: 7 (패턴)

**관찰**: `[intro]`, `[tree]`, `[walk]`, `[utils]`, `[list]`, `[slice]`, `[hash]` 전부에서 같은 형식이 나타납니다.
- 2칸 들여쓰기, K&R 중괄호, 문장이 하나인 본문은 중괄호 생략
- 수정하지 않는 포인터는 `const`, 반복 인덱스는 `size_t`, 오류는 `perror` + `return 1`
- 한 줄 함수 `int x2(int x) { return x * 2;}` `[list]`
- **복합 리터럴을 즐겨 씁니다**: `&(Node){...}` `[hash]`, `&(int){11}`, `&(char *){"Hello"}` `[slice]`

**예외**: `[hash 45–52행]` `get()`의 들여쓰기가 3, 4, 1칸으로 고르지 않습니다(원본 프레임으로 확인). 형식이 무너진 드문 경우입니다.

**포렌식 부산물**: 블록 안의 빈 줄에 들여쓰기 공백을 남기는 습관이 있습니다. 모든 영상의 체크섬 차이(−1 ~ −7B)가 이것으로 설명됩니다. `[lab]` 공개 저장소와 대조한 결과, **화면에 보인 코드는 판독 단계에서 들여쓰기까지 100% 정확**했습니다. 따라서 남는 차이는 판독 오류가 아닙니다.

**해석**: 형식이 일정하니 그 형식에서 벗어난 곳(남의 코드, 실수)이 눈에 띕니다.

### D3. 동작에 영향이 없으면 다듬지 않는다 (알려진 결함은 메모만 남긴다)
근거 영상 수: 10 (패턴)

**관찰**
- `[tree]` `DT_DIR )` 앞 공백, 동작과 맞지 않는 주석 / `[intro]` 실행될 수 없는 `return 0;`
- `[utils]` `# use fix this bug`라고만 남기고 다음 유틸로 넘어갑니다.
- `[list]` `// add free`라고 적었지만 해제 코드는 끝내 쓰지 않습니다.
- `[slice]` `// add err handle`라고 적었지만 `realloc` 실패는 확인하지 않습니다. 범용 버전에도 `int *tmp`가 남아 있습니다.
- `[hash]` 같은 키를 두 번 넣는 버그와 빈 버킷 segfault가 남아 있습니다(직접 실행해 확인). 마지막 말은 "all work"입니다.
- `[ed]` 시연에서 마주친 버그(`<` → `<=`, 개행, 지운 뒤 중복 저장)만 고칩니다. 마지막 줄을 지운 뒤의 위치(해제된 메모리를 가리킴), 끼워 넣기 대신 덮어쓰기, 종료 때 마지막 줄 미해제는 시연 범위 밖이라 그대로 남습니다(검증: [11/verify](11_c-ed-editor_MKmPyByntzA/verify/)).
- `[string]` `hstrncpy`는 n바이트를 무조건 복사해 짧은 원본에서 끝을 넘어 읽습니다(libc와 달리 `\0`에서 멈추지 않음). 시연(`"Goats"`, n=2)에서 드러나지 않아 그대로 남습니다(검증: [16/verify](16_c-string-library_72bBuM5tKN0/verify/)).
- `[cal 14:00]` `// note: in production use strtol and validation year`를 썼다가 1분 반 뒤에 지웁니다. 약점을 알지만 코드에는 남기지 않습니다(검증: `13 2026`은 2027년 1월을 출력).
- `[cat]` 확인하지 않은 경계: `./a.out .`은 인자로 받은 `.`도 숨김 파일로 처리해 아무것도 출력하지 않습니다(검증: [25/verify](25_c-cat_ie0PfrXZsc/verify/)).

**해석**: "영상에서 보여 주는 시나리오가 동작한다"를 완료 기준으로 삼습니다. 알려진 결함은 `// add …`나 `# fix this bug` 같은 TODO 메모로 남깁니다.

### D4. 반복은 재귀로 표현한다
근거 영상 수: 5 (패턴)

**관찰**
- `[tree]` `walk(sub_path, indent + 1)` / `[utils]` `walkrm`, `walkfind` / `[lisp]` `loop`, `print-list`

**반례 후보**: `[list]`, `[hash]`의 연결 리스트 순회는 재귀가 아니라 `for (; p; p = p->next)`, `while (p->next)` 반복문입니다. 트리 구조에는 재귀를, 선형 구조에는 반복문을 쓰는 것으로 보입니다.
- `[cat]` 폴더 순회는 `walkcat`이 자기 자신을 부르는 재귀입니다(스티커: `if dir -> walk cat again`). `[wget]` 링크를 따라가는 것도 `crawl`의 재귀입니다(방문 기록 없이).

**해석**: 트리 구조 데이터(파일시스템)는 재귀가 가장 짧은 표현입니다. 연결 리스트처럼 선형 구조는 C에서는 반복문으로 씁니다.

---

## E. AI에 대한 태도

### E1. AI의 답은 그대로 쓰지 않는다. 버리거나, 손으로 다시 짜서 이해한다
근거 영상 수: 5 (패턴) + 이유를 직접 말한 영상 3 (`[learn]`, `[story]`, `[proof]`)

**관찰**: AI를 쓴 영상은 두 편이고, 둘 다 제목에 "No AI"가 없습니다.

| | `[walk]` (10-17) | `[hash]` (10-23) |
|---|---|---|
| 계기 | 막혀서 자기 코드 23줄을 붙여 넣음 | 처음부터 `hash table in C`를 물음(**hayyoth 계정으로 로그인**) |
| 셸/메모 | `# Let's use some AI magic` | `i try to understand hashtable` |
| AI 코드를 어떻게 썼나 | 붙여 넣고 실행 → 동작함 | 별도 파일(`test.c`)에 두고 **옆 창의 교재로만** 씀 |
| 결말 | **undo로 전부 걷어 냄** → 자기 규칙을 씀 | **한 줄도 붙여 넣지 않고** 가장 단순한 형태부터 다시 짬 |
| 마지막 말 | `# ok , like this, you write your rule` | `all work , we now understand hashtable and chatgpt code you implement refactor of it for complex` |
| 결과물에 AI 코드 | 0줄 | 0줄 |

- `[hash 14:08]` 자기 해시와 AI 해시를 비교하는 메모: `it simple than chatgpt but / "abc" same "cba" or "acb"`. 더 단순한 쪽을 고르되, 무엇을 잃는지 스스로 적습니다.
- "No AI"가 붙은 영상(`[intro]`, `[tree]`, `[utils]`)에서는 AI를 쓴 흔적이 나오지 않았습니다.

10번부터 AI가 두 편 더 나옵니다.

| | `[ed]` (10-26, 제목에 "No AI") | `[flat]` (10-27) |
|---|---|---|
| 계기 | ChatGPT로 만든 `ed.c`(549줄, 10,452B)를 `ai.c`로 열어 보임 | 자기 뼈대(`int bytes[4]`, RULE 주석)를 Gemini에 붙여 시연 코드를 받음 |
| 셸/메모 | `this is chatgpt code / 550 line / too complicated` | `No , don't use struct , create state int[] like bytes[]` (AI가 struct로 답하자 되돌림) |
| AI 코드를 어떻게 썼나 | 훑어보기만 하고 치움 | **받은 코드를 그대로 컴파일해 실행** (시연용) |
| 결과물에 AI 코드 | 0줄. 손으로 113줄 | 시연 코드는 AI가 씀. 논증과 설계(평평한 배열)는 작가 |

- `[flat 13:20]` 썼다가 지운 메모: `i know gemini will write what before it write / this is good way understa…`
- `[learn]` AI 능력은 인정합니다: 자기 README로 Gemini가 동작하는 시뮬레이션을 만드는 것을 보여 주고 `it work / AI love complex / your brain not`. 결론은 "start from zero, no abstraction, make everything simplicity, you can learn better than AI".
- `[story]` 결론: `not that I hate AI, not that AI takes my job, I have always loathed systems that constrain human beings`.
- `[proof]` 초록 메모: "I am **not anti-tech**. I've used AI from day one. **But as people lose their skills and minds overusing it**, I took an extreme path to prove a point: **Zero AI.**" Zero AI를 원칙이 아니라 "요점을 증명하려는 극단적 실험"이라고 스스로 부릅니다.
- `[webjs]`, `[go]` 3시간 동안 AI를 열지 않습니다. `[go]`에서는 주소창에 Gemini가 자동 완성으로 여러 번 뜨지만 열지 않고 검색을 계속합니다.
- `[cat]` (제목에 "No AI") 첫 메모: **"The AI mentioned here is merely a meme and one possible use case. How you choose to use this tool is entirely up to you. The main focus is learning about File I/O in C."** AI는 코드를 쓰지 않고 **도구의 결과물을 읽는 쪽**입니다(`./a.out test-data` 출력을 Gemini에 붙여 넣어 요약시킴). 마지막 버그(`return;` 누락)는 Gemini에 묻지만 답을 보기 전에 스스로 고치고 주석을 답니다: **`// i know it before gemini check - yeah`**.
- `[firefox]` 브라우저 설정의 첫 동작이 "Block AI enhancements"입니다.

**해석**
- 저자는 AI를 피하지 않습니다. 다만 **AI의 결과물이 자기 코드가 되는 것을 거부합니다**.
- AI는 "읽을거리"이고, 이해의 증거는 자기 손으로 다시 쓴 코드입니다.
- 03번에서 10-23 사이에 쓰는 방식이 바뀌었습니다. "써 보고 버린다"에서 "교재로 놓고 다시 짠다"로요.
- 13번에서 처음으로 AI 코드가 그대로 실행됩니다. 하지만 거기서도 **설계는 사람이 되돌립니다**(struct → 평평한 배열). 지운 메모가 시험 방법을 말해 줍니다: AI가 무엇을 쓸지 미리 안다면 이해한 것이다. 규칙을 다시 쓰면 "AI의 결과물이 이해를 대신하게 두지 않는다"에 가깝습니다. AI가 쓴 코드가 결과물로 남는지는 부차적입니다.
- 이유는 작가가 직접 말합니다(`[learn]`, `[story]`). 사람은 단순한 것에서 배우고, 사람을 묶는 체계(AI 의존 포함)는 싫다는 것입니다. 그래서 "No AI"는 기술 거부가 아니라 학습 방법이자 삶의 태도입니다.

### E2. "AI 아니냐"는 의심에는 주장 대신 증거로 답한다
근거 영상 수: 2 (패턴)

**관찰**
- `[proof]` YouTube Studio를 열어 숫자를 보여 줍니다: 28일 조회 611,056, 구독자 +9.4K, 실시간 구독자 10,831 → 10,845. 메모 "So, who outperforming ?", "My channel is living proof."
- `[history]` "채널 전체가 AI로 만든 것 같다"는 메일에 "this video might be fake / this website might be fake"를 먼저 인정하고, "what AI cannot fake / that is **history**"를 적은 뒤 Wayback Machine의 2024년 보관본(27회 저장)과 2024년 GitHub 저장소를 엽니다. 메모 "AI can't turn back time".

**해석**
- 영상도 사이트도 위조할 수 있는 시대에, 위조할 수 없는 것은 **제3자가 남긴 시간 기록**과 **쌓인 결과**라고 봅니다.
- 결론 메모 "pure HTML with history beats AI again"이 `[html]`과 이어집니다. 단순한 것은 오래 남고, 오래 남은 것이 신뢰가 됩니다.
- 논점은 생산성이 아니라 **신뢰**입니다(`[proof]` "When content is cheap, attention fades and trust dies").

---

## F. 전달 방식 (무음 영상)

### F1. 말 대신 셸 주석, 스티커 메모, 코드 주석으로 말한다
근거 영상 수: 16 (패턴)

**관찰**: 모든 영상이 무음입니다. 메시지는 화면의 텍스트로 전달합니다.
- **셸 주석**: `[lisp]` `# In this video , i will make you love lisp` / `[tree]` `# Bravo` / `[walk]` `# Let's use some AI magic` / `[utils]` `## ok all work , good luck` / `[lab]` `# Source code`, `# How it work`, `# You should watch my previous video to understand`
- **REPL 주석**: `[list]` `; this very hard to implement in C`
- **스티커 메모**(macOS Stickies, 07번부터): `[list]` `this video help you:` / `[hash]` 메모 10여 장(`is this hashtable ?`, `now refactor key to string`, `if you not understand linked list , check my previous video` …)
- **설명용 코드 주석**: `[lab]` 시연 중에 `// Init data`, `// Main Logic simulation`, `// Map state to color to render to view`를 답니다. `[slice]` 식탁 비유 주석.
- **배경화면**: `[slice]` 최후의 만찬, `[hash]` 해시 테이블 그림(설명할 때 창을 치우고 가리킴)
- 웹캠 창의 이름은 항상 "Human"입니다.
- `[webjs]` 첫 메모로 방향 전환을 알립니다: 논쟁을 쉬고 프로그래밍에 집중하겠다, JS와 Go를 다시 배우겠다. 코드 주석은 질문 형태로 남습니다: `// can it storage these files forever`, `[go]` `// how read message user send`, `// now , return json`.
- `[proof]`, `[history]` 메모 여러 장을 말하면서 쓰고, 굵게 하고, 지우고, 다시 씁니다. 지운 판("2 week / every days …", "my old website can be fake…")이 생각의 경로를 보여 줍니다.
- `[w3m]` 스티커 메모가 목차입니다. 기능 하나를 보일 때마다 바꿉니다: "ignores JavaScript" → "multiple pages" → "crazy option config" → "bookmark" → "i often use w3m like this" → "yeah, offline" → "everything storage in files" → "try it".
- **셸 메모장**: `[cat]`, `[cal]` 셸에 따옴표 하나를 치고 `dquote>` 프롬프트를 메모장으로 씁니다(실행되지 않음). `[cat]` 사용 요령("open new terminal -> it will not have history / do cat / ctrl + A"), `[cal]` 마지막 과제.

**해석**: 터미널을 떠나지 않고 시청자에게 말을 거는 방법입니다. 07번부터는 스티커 메모가 자막 역할을 맡습니다.

### F2. 마지막은 시청자에게 과제로 넘긴다
근거 영상 수: 7 (패턴)

**관찰**
- `[walk]` `# ok , like this, you write your rule`
- `[utils]` `## ok all work , good luck`
- `[lab]` `# in this previous video , i made very simple version of this lib , check it , and read source code`
- `[slice]` `/* HARD HARD HARD VERSION - You implement it */`
- `[hash]` `… you implement refactor of it for complex`
- `[ed 70:00]` 마지막 스티커 "you implement your own…"(잘림).
- `[cal 27:40]` `dquote> In default cal it print by three column / you do that / … print_month add output to buffer , then print_year get buffer and print output to terminal`. 과제와 함께 푸는 방법까지 적어 줍니다.

**해석**: 완성품을 건네지 않고 다음 한 단계를 시청자의 손에 남깁니다. "AI 없이 손으로"라는 채널 주제와 맞는 결말입니다. 마지막 한 걸음은 시청자가 직접 쳐야 합니다.

---

## G. 도구와 환경

### G1. 도구와 화면에서도 덜어낸다
근거 영상 수: 7 (패턴)

**관찰**
- `[codium]` VS Code 대신 원격 측정과 라이선스를 뺀 VSCodium을 고릅니다(소개문 "without MS branding/telemetry/licensing"을 강조). 활동 표시줄, 상태 표시줄, 탭, 미니맵, 이동 경로, 줄 번호, 파일 아이콘, 패널 탭을 차례로 끄고 확장 기능은 하나도 설치하지 않습니다. 마지막 주석: `// Look good now, beauty, clean`.
- `[noapple]` `defaults write`로 둥근 모서리와 떠 있는 사이드바를 없애고, 색·투명도·Dock·메뉴 막대를 줄입니다. Apple 계정에는 로그인하지 않고, 공개 방화벽 LuLu로 `*.apple.com`, `*.icloud.com`을 차단합니다. 메모: `apple can't track you anymore` → `this is clean mac`.
- `[html]` 개인 사이트는 HTML 26줄, CSS는 `<h1 style="margin: 0;">` 한 줄, 정적 호스팅. 메모: `very simple. just two column, that all`, `that it , simple , clean, work`.
- `[webjs]`, `[go]` 편집기는 `[codium]`에서 장식을 다 끈 VSCodium 그대로입니다(줄 번호 없음). 다만 확장 기능 "0개"는 아닙니다. 필요해지자 Five Server(라이브 서버)와 Go 확장을 설치합니다. 라이브러리도 npm 대신 파일 하나(`localforage.min.js`)를 직접 내려받아 폴더에 넣습니다.
- `[firefox]` AI 기능 차단, 데이터 수집 끔, 새 탭은 빈 페이지, 도구 모음 버튼 대부분 제거, `#TabsToolbar, #back-button, #forward-button { display: none; }`.
- `[w3m]` "w3m - a text based web browser and pager" / **"ignores JavaScript"**.

**해석**
- 코드에서 추상화를 미루는 것(D1)과 같은 원칙을 도구와 환경에 적용합니다. 화면에 남는 것이 적을수록 코드와 생각이 앞에 섭니다.
- 장식과 함께 **제3자로 나가는 연결**도 끊습니다(원격 측정, 제조사 서버). 12번의 결론("사람을 묶는 체계가 싫다")이 도구 선택에서도 그대로 나타납니다.
- 결과 화면은 01번부터 써 온 vim과 터미널의 모습과 같아집니다. GUI 편집기를 쓰더라도 결국 같은 화면으로 수렴합니다.
- `[firefox]`, `[w3m]`에서 끝까지 갑니다. 브라우저의 탭 막대까지 지우고(`userChrome.css` 세 줄), 마지막에는 그림 없는 텍스트 브라우저를 권합니다. 바꾸는 방법도 확장 기능이 아니라 설정 파일 한 줄입니다(`WWW_HOME`, `userChrome.css`, `.zshrc` 별칭).

### G2. 데이터는 자기 디스크의 평범한 파일로, 오프라인에서도 쓸 수 있게 둔다
근거 영상 수: 3 (패턴)

**관찰**
- `[w3m]` Beej's Guide를 ZIP으로 내려받아 터미널에서 읽고("yeah, offline"), `crawl` 별칭(`wget --recursive --page-requisites --convert-links …`)으로 사이트를 통째로 받아 둡니다("crawl all webpages to work offline", "crazy fast / no wifi"). 브라우저의 기록·설정·북마크를 vim으로 열어 텍스트 파일임을 보입니다("everything storage in files").
- `[wget]` 같은 일을 직접 짭니다: 사이트를 `호스트/경로/index.html` 파일로 저장하고 정적 서버로 열어 오프라인에서 클릭해 봅니다.
- `[webjs]` 주석 `// can it storage these files forever` → 브라우저 저장소(IndexedDB)에 파일을 남깁니다.

**해석**
- 네트워크와 서비스에 기대지 않고 자기 디스크에 두면, 빠르고, 끊겨도 되고, 위조되지 않는 기록이 됩니다(`[history]`의 "AI can't turn back time").
- 평범한 텍스트와 HTML이면 어떤 도구로도 열 수 있습니다. `[html]`의 "Pure HTML just works"와 같은 이유입니다.
- 다만 일상에서는 직접 짠 크롤러(`[wget]`)가 아니라 옵션을 갖춘 진짜 wget을 별칭으로 씁니다. 직접 짜는 것은 배우기 위해서이고, 쓰는 것은 검증된 도구입니다.

---

## 열린 질문

다음 영상에서 확인할 것들입니다.
- ~~E1: "No AI" 영상에서 AI가 나오는가? 10번에서 저자는 무엇을 말하는가?~~ → `[ed]`는 제목에 "No AI"가 있지만 AI 코드를 **보여 주고 거절**합니다. 10번의 답은 "사람은 단순한 것에서 배운다". (E1에 반영)
- ~~B4/D1: 큰 프로그램에서도 단일 파일인가?~~ → `[ed]` 113줄도 `main.c` 하나, 도우미 함수는 `mode()` 하나뿐입니다.
- E1: AI 코드를 그대로 실행하는 `[flat]` 방식이 앞으로 늘어나는가, 시연용에 그치는가? → 26번까지 다시 나오지 않았습니다. `[cat]`에서 AI는 코드를 쓰지 않고 결과를 읽고 버그를 확인하는 데만 쓰입니다.
- A3: 리허설한 흔적이 갈수록 늘어나는가?
- 테스트 코드를 한 번이라도 쓰는가, 아니면 계속 실행 결과를 눈으로만 확인하는가(B1)? → `[ed]`까지 여전히 눈으로만 확인합니다. 대신 진짜 프로그램(`ed`)을 옆에 띄워 같은 입력의 결과를 비교합니다.
- ~~언어가 C나 Lisp가 아닐 때(Go, JS, HTML)도 C3처럼 엄격한 도구 설정을 쓰는가?~~ → 아닙니다. `[html]`, `[webjs]`에는 검사 도구가 없고, Quirks Mode 경고(`<!DOCTYPE>` 없음)를 1시간 넘게 그대로 둡니다. `[go]`는 gopls 경고(struct tag 따옴표)를 보고 고치지만 따로 켠 설정은 없습니다. 엄격함은 C에서만 보입니다.
- ~~C2의 반례: 붙여 넣고 고치는 방식은 "다시 배우는" 영상에만 나오는가?~~ → 언어에 따라 갈립니다. Go(`[wget]`)는 제목이 "by hand"여도 문서 예제를 붙여 넣고, C(`[cat]`, `[cal]`)는 man 페이지를 읽고 직접 씁니다. C에서 붙여 넣는 것은 자기 옛 코드뿐입니다.
