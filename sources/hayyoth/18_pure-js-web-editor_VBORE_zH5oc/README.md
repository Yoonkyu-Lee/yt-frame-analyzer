# 해체 분석: "No AI - Just pure JS hand coding to relax - Web Editor & Files Manager"

- 영상: https://youtu.be/VBORE_zH5oc (1:28:00), 태그 `[webjs]`
- 결과물: 복원한 코드 [src/](src/) (`web-editor/main.js`, `web-filemanager/sidebar.js`, `storage.js`), 전사 [transcripts/](transcripts/) (20구간)
- 방법: 프레임 5,280장 → 웹캠을 가린 키프레임 1,157장 → 20구간 전사(코드 중심 형식).
  - 타자 변화가 거의 매초 있어서 임계값을 0.05로 올렸습니다. 그래서 중간 타자 과정 일부는 건너뛰었습니다.
  - 편집기에 줄 번호가 없고, vim 저장 메시지 같은 체크섬도 없습니다. 복원한 코드는 줄 수와 바이트로 확인할 수 없습니다.
- 개인정보: 주소창 추천 목록의 메일 주소, 계정 이름, 비공개 대화방 이름, Wi-Fi 이름은 전사에서 뺐습니다. 오디오는 쓰지 않았습니다.

## 1. 첫 메모 (0:00, 1:05에 지움)

> "Lately, I've found myself sharing my views and engaging in debates quite a bit. It's starting to wear me out. From now on, I'm going to focus solely on programming; I'll be taking a break from expressing my personal opinions for a while.
> It's been a year since I last did any web development; I've forgotten almost everything about JS and Go. I'm going to relearn them."

10~20번의 논쟁 영상이 끝나고, 다시 손으로 만드는 영상으로 돌아오겠다는 선언입니다. "다 잊었으니 다시 배운다"는 전제도 밝힙니다.

## 2. 흐름

| 시각 | 프로젝트 | 한 일 |
|---|---|---|
| 0:00~2:06 | web-editor | `mkdir web-editor && codium .`. 확장 Live Preview를 설치했다가 Five Server로 바꿈(127.0.0.1:5500). "css two column"을 검색해 W3Schools 예제를 붙여 넣음 |
| 2:06~11:31 | | devtools에서 CSS를 먼저 고쳐 보고 소스로 옮기는 방식으로 레이아웃(상단 막대 + 두 열)을 만듦. 왼쪽은 `<textarea>`, 오른쪽은 `<iframe>`, Run 버튼. `btnRun.onclick(...)` 오류 → `addEventListener`로 고침 |
| 11:31~16:07 | | iframe에 HTML을 넣는 법: Stack Overflow의 `contentDocument.body.innerHTML`을 쓰다가 `srcdoc`으로 바꿈(옛 줄은 주석으로 남김). 이어서 Ace 편집기를 찾음 |
| 16:07~22:17 | | Ace 연결. 문서의 상대 경로가 안 돼서 cdnjs 1.44.0으로 바꿈. 높이 0 문제(devtools로 확인). 로컬 복사본은 SRI 불일치와 테마 파일 404로 실패해 CDN으로 되돌림 |
| 22:17~26:20 | | `editor.value` → `editor.getValue()`(Ace 문서에서 찾음). Vue SFC Playground를 참고로 훑어봄. 자기 편집기에 입력창과 버튼이 있는 작은 페이지를 써서 실행해 봄("Hello Cong") → **편집기 완성** |
| 26:20~34:40 | go-filemanager | Go로 파일 관리자를 시작. gopls PATH 문제, `io/fs` 문서 예제를 붙여 넣고 주석 "what the heck fsys, it look like zig", "many year ago , i remember not have this". Zig 블로그를 읽음. 결국 **"i'm done with Go / No Go anymore"** |
| 34:40~45:48 | web-filemanager | `index.html` + `main.js` + 모듈(`sidebar.js`, `detail.js`). 각 화면 조각은 `createElement`로 만든 요소를 돌려주는 함수. 너비 272px 사이드바, CSS 변수 `--border-color` |
| 45:48~63:43 | | MDN에서 `<input type="file">`을 읽음. 숨긴 input을 Upload 버튼으로 클릭시킴, `multiple`, 파일마다 탭 하나. 제목은 "Files" → "Hay Files" → "HFiles". `URL.createObjectURL` |
| 63:43~74:31 | | `file.type.startsWith("image/")` → `<img>`, `"video/"` → `<video><source>`(W3Schools 예제). 큰 이미지 때문에 사이드바가 줄어드는 문제 → `min-width: 272px` |
| 74:31~87:59 | | 주석 **`// can it storage these files forever`**. MDN에서 IndexedDB와 Web Storage를 비교하고 localForage를 고름(min.js를 직접 내려받음). `setItem(file.name, file)`, `initFiles()`(keys → getItem), 탭을 만드는 코드를 `appendFile`로 뽑아 냄. 새로 고침 뒤에도 파일이 남는 것을 확인. 콘솔에 `bye bye` |

## 3. 최종 코드

### web-editor (`main.js`, 22:17~24:02 기준)
```js
const btnRun = document.getElementById("btnRun");

const render = document.getElementById("render");
//const iframedoc = render.contentDocument || render.contentWindow.document;

const editor = ace.edit("editor");
editor.setTheme("ace/theme/monokai");
editor.session.setMode("ace/mode/html");

btnRun.addEventListener("click", () => {
    render.srcdoc = editor.getValue();
})
```
HTML은 `.nav`(제목 + Run) 아래 `.row`에 `#editor`와 `<iframe id="render">`가 있는 구조입니다. 편집기 전체가 JS 12줄, 외부 의존은 CDN의 Ace 하나입니다.

### web-filemanager (`sidebar.js`, 87:53 기준)
전문은 [src/web-filemanager/sidebar.js](src/web-filemanager/sidebar.js)에 있습니다.
- `initFiles(tabs, detailCtn)`: 저장된 키를 돌며 파일을 다시 붙입니다. 영상 마지막 장면에서 이 함수 본문이 `const file = localforage.get`까지만 보입니다. 저장본은 그가 직전에 연 localForage `getItem` 문서대로 두 줄을 채웠고 `RECONSTRUCTED`로 표시했습니다.
- `appendFile(file, tabs, detailCtn)`: 탭을 만들고, 클릭하면 이미지나 동영상을 오른쪽 칸에 보여 줍니다.
- `sidebar(detailCtn)`: 사이드바(제목 "HFiles", Upload 버튼, 탭 목록)를 만들어 돌려줍니다.

정적 읽기로 보이는 한계(영상에서 고치지 않은 것):
- 파일 이름이 저장 키입니다. 같은 이름의 파일을 올리면 덮어씁니다.
- 새로 고침 뒤 탭 순서가 바뀝니다(정렬 없음, 영상에서도 보임).
- 같은 이름을 두 번 올리면 화면에는 탭이 두 개 생깁니다(저장소에는 하나).
- `createObjectURL`로 만든 주소를 해제하지 않습니다. 이 주의 사항이 있는 MDN 문단을 보고 지나갑니다.
- `<!DOCTYPE html>`가 없어 Quirks Mode 경고가 계속 뜹니다.

## 4. 해석

- **"다 잊었다"에서 시작해 문서로 다시 배웁니다.** 레이아웃은 W3Schools, iframe은 Stack Overflow, 편집기는 Ace 문서, 업로드는 MDN, 저장은 localForage 문서에서 가져옵니다. 예제를 붙여 넣고, 콘솔 오류를 읽고, 자기 이름으로 고칩니다. AI는 한 번도 열지 않습니다.
- **devtools가 연습장이고 소스가 원본입니다.** CSS 값은 먼저 브라우저에서 바꿔 보고, 맞으면 소스로 옮기는 순서를 수십 번 반복합니다(높이 62/52/42px 시도, google.com 색을 참고).
- **프레임워크 없이 함수와 DOM으로 짭니다.** 화면 조각마다 함수 하나가 `createElement`로 요소를 만들어 돌려줍니다. 빌드 도구도 없이 라이브러리 파일을 직접 내려받아 폴더에 넣습니다.
- **필요가 생길 때만 추상화합니다(D1).** 탭을 만드는 코드는 업로드 처리기 안에 있다가, 다시 불러오기 기능이 같은 일을 하게 되자 `appendFile`로 뽑힙니다.
- **맞지 않는 도구는 버립니다.** Go의 `fsys` 추상화에 고개를 젓고("it look like zig") 30분 만에 Go를 접습니다. 같은 파일 관리자를 브라우저 JS로 다시 시작합니다. 13번 "C is flat"의 기준이 언어를 고를 때도 쓰입니다.
- 주석은 질문의 형태로 남습니다: `// can it storage these files forever`. 이 질문에 답하는 것이 마지막 20분입니다.
