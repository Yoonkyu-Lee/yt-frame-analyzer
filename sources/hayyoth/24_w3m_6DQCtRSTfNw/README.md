# 해체 분석: "you should try w3m - terminal web browser"

- 영상: https://youtu.be/6DQCtRSTfNw (10:59), 태그 `[w3m]`
- 결과물: 시연 정리와 작가의 `.zshrc` (이 README), 전사 [transcripts/](transcripts/) (6구간)
- 방법: 프레임 659장 → 웹캠을 가린 키프레임 288장 → 6구간 전사(글 중심 형식). 오디오는 쓰지 않았습니다. 파일 경로의 계정 이름과 검색 결과의 메일 주소는 가렸습니다.

## 1. 스티커 메모 = 목차

기능 하나를 보여 줄 때마다 메모를 바꿉니다.

| 구간 | 메모 | 시연 |
|---|---|---|
| 0:00~1:04 | "w3m - a text based web browser and pager" / **"ignores JavaScript"** | `man w3m`에서 NAME 줄을 복사. hayyoth.org, wingram.org, DuckDuckGo Lite를 터미널에서 엶 |
| 1:04~1:20 | | `.zshrc`에 `export WWW_HOME="https://lite.duckduckgo.com/lite"`를 넣어 `w3m`만 치면 검색창이 뜨게 함 |
| 1:20~1:48 | "multiple pages" | 탭 여러 개 |
| 1:48~3:41 | "good help / many shortcuts" → "crazy option config" | `H` 도움말, `o` 설정 창. GitHub가 "gunzip: unknown compression format"으로 안 열리자 설정 창에서 Accept-Encoding을 고침 |
| 3:41~5:21 | "search in page" → "bookmark" → "see bookmark" | `/` 검색, Linux 저장소와 hayyoth.org를 북마크 |
| 5:21~7:13 | "easy search and navigation" | DuckDuckGo Lite로 Beej's Guide를 찾아 `/`로 `strcat`까지 이동. 외부 브라우저를 `open`으로 설정 |
| 7:13~8:59 | "many other features …" → **"i often use w3m like this"** | Beej's C Library Reference HTML ZIP을 내려받아 압축을 풀고 터미널에서 읽음 |
| 8:59~10:00 | **"yeah, offline"** → "crawl all webpages to work offline" → "crazy fast / no wifi" | 로컬 Beej 문서에서 `fflush` 찾기. `crawl wingram.org`(wget 별칭)로 사이트 전체를 받아 디스크에서 읽음 |
| 10:00~10:58 | **"everything storage in files"** → "try it" | w3m의 history, config, bookmark.html을 vim으로 열어 모두 평범한 텍스트·HTML 파일임을 보임 |

## 2. 작가의 `.zshrc` (9:17, `.zshrc` 19L 603B, 일부)

```sh
export CLICOLOR=1
export LSCOLORS=Gxfxcxdxbxegedabagacad
export WWW_HOME="https://lite.duckduckgo.com/lite"
alias e="vim"
alias l="ls -a"
alias ll="du -sh * | sort -h"
alias web="open -a Firefox"
alias hay="open -a Firefox https://www.youtube.com/@hayyoth-org"
alias cam="scam start"
alias cc="clang -Wall -Wextra -pedantic -Wno-unused-parameter -Wno-unused-varia…"   # 창 끝에서 잘림
alias crawl='wget --recursive --page-requisites --adjust-extension --convert-li…'  # 창 끝에서 잘림
```
- `cc` 별칭은 01번부터 짐작한 엄격한 경고 설정(C3)을 처음으로 설정 파일에서 직접 보여 줍니다.
- `cam="scam start"`는 20번 첫 장면의 웹캠 명령입니다.
- `crawl`은 진짜 wget을 씁니다. 23번에서 Go로 직접 짠 크롤러가 아니라 옵션을 갖춘 wget(`--page-requisites`, `--convert-links`)을 일상에서 씁니다.

## 3. 해석

- **G1(도구에서 덜어낸다)의 끝입니다.** VSCodium(14), macOS(15), Firefox(22)에서 장식을 끈 다음, 아예 그림 없는 텍스트 브라우저를 권합니다. "ignores JavaScript"를 굵게 적은 것은 17번("Pure HTML just works")과 같은 주장입니다. HTML만으로 된 사이트는 w3m에서도 그대로 읽히고, GitHub처럼 JS가 많은 페이지는 깨집니다(화면에 그대로 둠).
- **오프라인과 파일이 요점입니다.** 문서는 내려받아 디스크에서 읽고, 사이트는 통째로 받아 두고, 브라우저 설정과 기록도 텍스트 파일입니다. 18번의 `// can it storage these files forever`, 20번의 "기록은 위조할 수 없다"와 같은 방향입니다. 자기 데이터를 자기 파일로 갖는 것입니다.
- **배움의 도구로 씁니다.** 시연 대부분이 Beej's Guide와 man 페이지 읽기입니다. 16번에서 Beej의 오류를 찾던 그 문서를 터미널에서 검색합니다(C1).
- **확장 대신 설정 한 줄.** 기본 페이지(`WWW_HOME`), 외부 브라우저, Accept-Encoding 모두 설정이나 환경 변수 한 줄로 바꿉니다(22번의 `userChrome.css`와 같음).
