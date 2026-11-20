# 해체 분석: "build your own wget (Web Crawler) in Go by hand"

- 영상: https://youtu.be/g4SQ1LGRI00 (24:57), 태그 `[wget]`
- 결과물: 복원한 코드 [src/main.go](src/main.go) (약 80줄), 전사 [transcripts/](transcripts/) (9구간)
- 방법: 프레임 1,497장 → 웹캠을 가린 키프레임 448장(임계값 0.02) → 9구간 전사(코드 중심 형식). 오디오는 쓰지 않았습니다.
  - VSCodium이라 줄 번호와 저장 메시지가 없어 체크섬이 없습니다. 파일 맨 위(`package`, 첫 import 세 개)는 화면에 나오지 않아 쓰임새로 보충했습니다.
  - 이 컴퓨터에 Go가 없어 실행 검증은 하지 않았습니다. 아래 지적은 정적 읽기입니다.

## 1. 흐름

| 시각 | 한 일 |
|---|---|
| 0:00~2:39 | pkg.go.dev `net/http`를 읽고 `http.Get` → `io.ReadAll` → 출력. example.com, wingram.org는 되지만 DuckDuckGo Lite는 봇 확인, nokia.com은 "Access Denied". 주석 `// but without user-agent, some sites will block` |
| 2:39~7:19 | 문서의 Client/NewRequest/Do 예제를 붙여 넣고 줄여서 `crawl(link)`. Firefox 개발자 도구에서 자기 브라우저의 User-Agent를 복사해 헤더에 넣음 |
| 7:19~9:24 | 주석 `// get link in html` → `golang.org/x/net/html` 문서의 Parse 예제를 가져와 `<a href>`를 뽑음. wingram.org의 링크가 `/`, `#about`, `about/wingram/` 등으로 나옴 |
| 9:24~16:05 | `strings.HasPrefix`로 `#` 링크를 거름 → `crawl`을 재귀로. 출력에 `//`가 끝없이 늘어나고 gnu.org 같은 외부 링크가 붙어 `^C`로 멈춤 → `#`, `/`, `https`로 시작하는 링크를 모두 거름 |
| 16:05~21:33 | `save(link, content)`: 주석 "to create file , we need remove https from link". `path/filepath`, `net/url` 문서, **자기 옛 Go 저장소(gfm)의 `MkdirAll`/`WriteFile`을 참고** → `url.Parse`로 `Host + Path`, `/`로 끝나면 `index.html` |
| 21:33~24:57 | 루트가 `wingram.org`라는 파일로 저장되는 버그를 `if u.Path == "" { link += "/" }`로 고침. 저장된 폴더를 Five Server로 열어 Home → Philosophy → Father → Sites를 오프라인으로 클릭해 봄. 마지막 주석 **`// done - simple`** |

## 2. 최종 코드의 핵심

```go
func save(link string, content []byte) {
  u, err := url.Parse(link)
  if err != nil {
    log.Fatal(err)
  }
  link = u.Host + u.Path
  os.MkdirAll(link, 0777)

  if u.Path == "" {
    link += "/"
  }

  if strings.HasSuffix(link, "/") {
    link += "index.html"
  }

  fmt.Println("===> save ", link)

  // save file content
  os.WriteFile(link, content, 0777)
}
```
`crawl`은 요청 → 저장 → `<a href>` 가운데 상대 링크만 골라 `crawl(link + "/" + subLink)`로 재귀합니다. 전체 크롤러가 함수 셋, 약 80줄입니다.

## 3. 정적 읽기로 본 한계

영상 속 대상(작가의 옛 사이트, 페이지 몇 개에 상대 링크만 있음)에서는 잘 동작합니다. 일반 사이트에 쓰면 다음 문제가 생깁니다.
- **방문 기록이 없습니다.** 서로를 가리키는 페이지나 `../` 링크가 있으면 끝없이 돕니다. 9~16분에 실제로 `//`가 끝없이 늘어나 `^C`로 멈춘 것도 같은 원인이고, 접두사 필터로 증상만 막았습니다.
- **상대 링크를 페이지 위치 기준으로 풀지 않습니다.** `link + "/" + subLink`라서 `about/wingram/` 페이지 안의 `x`는 `about/wingram//x`가 됩니다. `net/url` 문서(`Parse`, `JoinPath`)를 읽었지만 링크를 페이지 기준으로 푸는 데는 쓰지 않았습니다(표준 방법은 `ResolveReference`).
- `http://`로 시작하는 외부 링크는 `https` 필터를 통과해 주소 뒤에 그대로 붙습니다.
- `MkdirAll(link)`가 `index.html`을 붙이기 전에 불립니다. `/`로 끝나지 않는 페이지(`a/b.html`)는 `b.html`이라는 폴더가 먼저 생겨 `WriteFile`이 실패하고, 그 오류는 무시됩니다.
- CSS, 이미지, JS는 받지 않습니다(`<a>`만 따라감). 네트워크 오류 하나에 `log.Fatal`로 전체가 끝납니다.

## 4. 해석

- **"wget"이지만 목적은 자기 사이트를 오프라인으로 읽는 것입니다.** 24번(w3m)에서 쓴 `crawl` 별칭과 "crawl all webpages to work offline" 메모가 같은 생각입니다. 일반 크롤러가 아니라 지금 필요한 만큼만 만들고 `// done - simple`로 끝냅니다(D1, D3).
- **문서 예제를 붙여 넣고 줄입니다.** Client 예제와 html.Parse 예제를 그대로 가져와 필요 없는 줄을 지우고, 주석 `// handle error`도 남깁니다. 21번에서 본 C2의 반례(Go·JS에서는 문서 예제를 붙여 넣는다)가 다시 나옵니다.
- **자기 옛 코드를 참고합니다(B4).** 파일 저장은 자기 Go 저장소 gfm의 코드를 열어 보고 씁니다.
- **주석으로 다음 할 일을 먼저 적고 채운 뒤 지웁니다(B2)**: `// get link in html`, `// create folder`, `// save file content`.
- **결과를 진짜 쓰임새로 확인합니다.** 저장된 폴더를 정적 서버로 열어 링크를 하나씩 눌러 봅니다. 테스트 코드는 없습니다(B1).
- 21번에서 "Go는 GC가 붙은 C"라고 적은 뒤 처음으로 Go로 끝까지 만든 도구입니다. 18번의 "i'm done with Go"는 Go를 버린 것이 아니라 그 프로젝트에 맞지 않았던 것으로 보입니다.
