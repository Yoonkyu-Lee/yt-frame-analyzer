# 해체 분석: "a bit of Golang for fun"

- 영상: https://youtu.be/avofMrOfsWI (1:29:57), 태그 `[go]`
- 결과물: 복원한 코드 [src/](src/) (`main.go`, `public/chat.html`), 전사 [transcripts/](transcripts/) (17구간)
- 방법: 프레임 5,397장 → 웹캠을 가린 키프레임 894장(임계값 0.05) → 17구간 전사(코드 중심 형식).
  - 편집기에 줄 번호와 저장 메시지가 없어 체크섬이 없습니다.
  - 이 컴퓨터에 Go가 없어 컴파일 검증은 하지 않았습니다. 아래 지적은 정적 읽기입니다.
  - `socket/client.go`와 `hub.go`는 gorilla/websocket의 chat 예제를 그대로 붙여 넣은 것이라 따로 저장하지 않았습니다.
- 개인정보: 계정 이름과 주소창의 개인 기록은 전사에서 뺐습니다. 오디오는 쓰지 않았습니다.

## 1. 메모

> "It's been a year since I last wrote any Go code, and I've forgotten almost everything. In this video, I'll do a quick refresher on Go and see what I can build with it."
> -- 
> **"Learn philosophy of language don't learn syntax / all you need is just go.dev websites"** (0:30)

> **"Go is just C with garbage collector / and great library builtin"** (15:18)

## 2. 흐름

| 시각 | 한 일 |
|---|---|
| 0:00~9:35 | A Tour of Go를 읽으며 편집기를 둘로 나눠 `main.go`와 `main.c`에 같은 개념을 씀. 주석 `// go [name] [type]` / `// c [type] [name]`. slice를 보고 `// check my previous video for dynamic array`(08번) |
| 9:35~19:00 | 구조체, 패키지(`hello.sayHi` → 소문자라 export 안 됨 오류), 표준 라이브러리(`fmt`, `math/rand`, `crypto`) 훑기. `go run`을 여섯 번 돌려 난수 확인 |
| 19:00~27:47 | go.dev "Writing Web Applications"로 `net/http` 서버(`/hello`). Alex Edwards 글로 정적 파일 서버(`/web/`, `StripPrefix`). W3Schools 폼으로 `chat.html` 시작 |
| 27:47~43:22 | MDN fetch 예제로 `POST /api/send`. "return json"(주석 `// now , return json`), "read message user send"(주석 `// how read message user send`)를 차례로 검색해 `MessageReq`/`MessageRes`, `json.NewDecoder`/`Encoder` |
| 43:22~63:43 | 오류 처리: guard clause로 평평하게, `http.Error`에 nil `err`를 넘겨 패닉 두 번 → `json.Marshal` + `http.Error`. struct tag 따옴표 누락(`json:message`)을 gopls 경고로 찾음. 화면에 메시지 목록 |
| 63:43~77:16 | 실시간을 찾아 감: `net`, x/net/websocket, GoRealtimeWeb, websocket.org(gorilla는 보관 상태라는 안내를 읽고도) **gorilla/websocket chat 예제**를 복사해 `socket` 패키지로 정리 |
| 77:16~89:56 | MDN WebSocket 예제를 브라우저 콘솔에서 먼저 시험(101 확인) → `chat.html`로 옮김. HTTP 처리기와 소켓을 잇다가(`ServeWs`가 `*Client`를 돌려주게, `WritePump`로 export) 방향을 바꿔 **`/api/send` 처리기와 타입을 지움**(`// don't need this any more`). 창 두 개에서 메시지가 서로 전달되는 것을 보이며 끝 |

## 3. 최종 코드 (89:55 기준)

`chat.html`(스크립트 부분):
```js
// just for demo, don't use it in production
const inpMess = document.getElementById("inpMess");
const messagesElm = document.getElementById("messages");

const socket = new WebSocket("ws://localhost:8080/ws");

async function send() {
    socket.send(inpMess.value)
}

function appendMessage(mess) {
    const messElm = document.createElement("div");
    messElm.innerText = mess;
    messagesElm.appendChild(messElm);
}

socket.addEventListener("message", (event) => {
    appendMessage(event.data);
});
```
`main.go`는 허브를 띄우고 `/web/`(정적 파일)과 `/ws`(소켓)를 연결하는 20줄입니다([src/main.go](src/main.go)).

정적 읽기로 본 것:
- **지운 다리 코드가 실제로 위험했습니다.** 지우기 전의 `handler`는 마지막 줄에서 `client.WritePump()`를 직접 불렀습니다. `ServeWs`가 이미 연결마다 `go client.writePump()`를 띄우므로, 이 호출은 같은 연결에 두 번째 쓰기 루프를 만듭니다. gorilla 예제 주석이 금지하는 것("at most one writer")이고, HTTP 처리기는 그 루프에서 돌아오지 않습니다. 전역 `var client`는 마지막에 접속한 한 사람만 가리킵니다. 결국 이 다리를 버리고 소켓만 남긴 것이 맞는 선택이었습니다.
- 마지막에 보인 `main.go`에는 지운 `handler`를 가리키는 `/api/send` 줄이 남아 편집기가 오류를 표시합니다. 그 상태로는 컴파일되지 않습니다. 고친 장면은 프레임에 없습니다.
- 받은 메시지는 `innerText`로 넣습니다. 다른 사람이 보낸 HTML이 실행되지 않습니다.
- 페이지 맨 위 주석 `// just for demo, don't use it in production`: 서버 주소가 `localhost`로 고정돼 있고 인증이 없다는 것을 스스로 표시합니다(B5와 같은 정직한 표시).

## 4. 해석

- **"문법이 아니라 언어의 철학을 배운다"는 방법이 C와 나란히 쓰기입니다.** Go 개념마다 C로 같은 것을 써 보고, 차이(이름과 타입 순서, slice = 08번의 동적 배열)를 주석으로 남깁니다. 결론은 "Go는 GC가 붙은 C에 좋은 표준 라이브러리"입니다.
- **1차 자료는 go.dev 하나로 충분하다고 선언하지만, 실제로는 여러 문서를 섞어 씁니다**: go.dev, pkg.go.dev, MDN, W3Schools, Stack Overflow, dev.to, 개인 블로그, gorilla 예제. 공통점은 모두 사람이 쓴 문서와 예제라는 것입니다. 주소창에 Gemini가 자동 완성으로 여러 번 떴지만(0:33, 53:15 등) 한 번도 열지 않습니다.
- **기능이 필요해질 때까지 미루고, 필요 없어지면 지웁니다.** JSON API(`MessageReq`/`MessageRes`, 오류 처리)를 30분 들여 만들었지만, 소켓이 같은 일을 하자 통째로 지웁니다. 18번의 "Go를 접음"과 같은 결단입니다.
- **다만 이번에는 남의 코드를 크게 들여옵니다.** 지금까지는 짧은 예제를 붙여 넣고 고쳐 썼는데, 여기서는 gorilla의 chat 예제 두 파일(약 200줄)을 거의 그대로 가져옵니다(라이선스 주석 포함). "fun"과 "refresher"라는 제목대로, 이 영상은 처음부터 만들기보다 언어를 다시 익히는 데 목적이 있습니다.
- 18번 첫 메모의 "JS와 Go를 다시 배운다"가 18번(JS)과 21번(Go)으로 이어집니다.
