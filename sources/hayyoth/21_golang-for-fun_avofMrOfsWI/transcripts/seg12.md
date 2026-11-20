# Segment 12 (60:33 - 63:43) — avofMrOfsWI "a bit of Golang for fun"

Note: the editor (macOS-style window titled `chat.html — lab`, light theme) shows NO line-number gutter in these frames, so line numbers below are relative (`~`). Webcam "Human" overlay ignored. No editor save messages visible.

## Timeline
- [60:33] Editor `chat.html — lab` in front; Firefox behind at `http://localhost:8080/web/chat.html`. Cursor at end of line `<div class="message">How about</div>`. Top of file visible (h1, messages div, input, button, script start). Firefox devtools behind shows 1 error and a `[HTTP/1.1 404 Not Found 0ms]` (favicon).
- [60:38] Firefox page: "Chat pages" h1, "Hello", "How about", an input containing `youtube`, a Send button. Console: warning `This page is in Quirks Mode. Page layout may be impacted. For Standards Mode use "<!DOCTYPE html>".` (chat.html) and `GET http://localhost:8080/favicon.ico [HTTP/1.1 404 Not Found 0ms]`. Editor behind: a bare `<br>` line now sits between `</div>` and the `<input ...>` line.
- [60:40] Editor: line after `</div>` is `<br>` (caret after it); then `<input type="text" id="inpMess" placeholder="Enter Message"><br><br>`, then `<button onclick="send()">Send</button>`.
- [60:42] Editor now `<br><br>`; page reloaded ("Waiting for localhost...", console cleared); a bit more space under the messages.
- [60:45] Editor: `<br><br>` with the second `br` selected/being edited (about to become `hr`).
- [60:48] Line now `<br><hr>`. Browser shows a bigger gap between "How about" and the input; console again shows the Quirks Mode warning + favicon 404.
- [60:54] Editor: `<br><hr>` line; the browser behind shows a horizontal rule (dark line). Cursor near `</div>`.
- [61:15] Editor scrolled down: author is writing a new function at the end of the script: `function appendMessage(mess) {` / `const messElm = document.c` with autocomplete popup (caretPositionFromPoint, characterSet, childElementCount, childNodes, children, cloneNode, close, compareDocumentPosition, compatMode, contains, contentType, cookie). Above it in send(): `const result = await r...`, `const message = result...`, `alert("Send " + messag...`, `} catch (error) {`, `console.error(error.me...`.
- [62:04] Firefox (front): page with `<hr>` between messages and input; input `youtube`. Editor behind (scrolled so `<script>` is at top): in send() the `alert("Send "...)` line has been replaced by a call starting `append...` (highlighted line); below it `function appendMessage(mess) {` / `const messElm = document.createElement("div");`.
- [62:17] Page now shows messages "youtube", "hello", "what the he" (appended), input contains `what the he`. A second Firefox window in front at `pkg.go.dev/net/http#Request`: `type Request struct {` with comments for Method, URL, Proto, ProtoMajor (reading Request struct docs).
- [62:19] Same window now at `go.dev/doc/articles/wiki/`, section "Introducing Function Literals and Closures" (viewHandler/editHandler/saveHandler with `title string`, `makeHandler`).
- [62:20] Back to chat.html Firefox window (devtools Console, favicon 404 + quirks warning).
- [62:22] A new Firefox window/tab opens; URL bar "Search with DuckDuckGo or enter address" focused.
- [62:23] Typing `go` in URL bar; suggestions (Google Gemini, Google Translate ...).
- [62:24] URL bar: `golang real`.
- [62:27] Search `golang socket` loading (DuckDuckGo; "Performing a TLS handshake to links.duckduckgo.com...").
- [62:35] Results: ZetCode "Working with Sockets in Go - ZetCode", "net package - net - Go Packages" (pkg.go.dev/net, Oct 20, 2026), kelche.co "A Complete Guide to Socket Programming in Go - Kelche", "socket package - golang.org/x/net/internal/socket - Go Packages". Cursor on net package link.
- [62:39] `pkg.go.dev/net`: Version go1.27.3 Latest, Published Oct 20, 2026, License BSD-3-Clause, Imports 24, Imported by 652,843; Overview: "Package net provides a portable interface for network I/O, including TCP/IP, UDP, domain name resolution, and Unix domain sockets."
- [62:40] Overview text: "most clients will need only the basic interface provided by the Dial, Listen, and Accept functions and the associated Conn and Listener interfaces." Code shown (see Shell & outputs).
- [62:43] Listen example with `ln.Accept()` loop and `go handleConnection(conn)`. Then "Name Resolution" section.
- [62:46-62:47] Scrolling Name Resolution text (pure Go resolver vs cgo, GODEBUG netdns).
- [62:48-62:50] Scrolling the package Index (LookupPort, LookupTXT, ParseCIDR, Pipe, SplitHostPort, type Addr, AddrError, Buffers, Conn: Dial, DialTimeout, FileConn; DNSConfigError, DNSError, Dialer: Dial, DialContext, DialIP, DialTCP, DialUDP, DialUnix, MultipathTCP, SetMultipathTCP; Error, Flags, HardwareAddr, IP ...).
- [62:52-62:54] Back to DuckDuckGo `golang socket` results; hover on net link (status `pkg.go.dev/net`), then to ZetCode link.
- [62:55] `zetcode.com/golang/socket/` "Go socket" (last modified April 11, 2024), dark theme. Intro: "In this article we show how to work with sockets in Golang. Socket programming is low-level. ..." Note: "In networking, the term socket has a different meaning. It is used for the combination of an IP address and a port number."
- [62:56] "Network protocols" (TCP/IP vs UDP) and "Go net package": `func Dial(network, address string) (Conn, error)`.
- [62:57] "The Dial function connects to the address on the named network -- it opens a socket. We use the Write function to write to the socket and Read function to read from the socket." Known networks: tcp, tcp4, tcp6, udp, udp4, udp6, ip, ip4, ip6, unix, ...
- [62:58-63:00] "Go UDP socket example": Echo protocol (RFC 862, port 7); `$ cat /etc/services | grep echo | head -4` sample output; `# apt install xinetd`; xinetd echo-dgram config; `# systemctl start xinetd`; `echo_client.go` begins (`package main`, imports "fmt" "log" "net" "os").
- [63:06] ZetCode echo_client.go snippets (checkErr, net.Dial("udp", "debian:7"), Write, make([]byte, 1024), Read, Println).
- [63:07] New tab, URL typed `google.com/` (autocomplete).
- [63:12] Search `golang realtime` results: LibHunt "Top 23 Go Realtime Projects | LibHunt" (Oct 22, 2026), MESL PDF "Go-RealTime: A Lightweight Framework for Multiprocessor R...", GitHub topics realtime, GitHub "realtime: A realtime service written in Gol..." (WebSockets / Server Sent Events).
- [63:13-63:35] Scrolls `golang realtime` results: Centrifugo (centrifugal.dev), FreeCodecamp "How to Build a Real-Time Notification System with Go and Kafka", Codez Up "Go WebSocket Tutorial: Build a Real-Time Chat Server" (Jun 2, 2025), Viblo "Build a Realtime Chat Server With Go and WebSockets - Viblo" (Vietnamese snippet), Ably "Scaling realtime apps with Golang and WebSockets", malnossi.github.io "Real Time Chat System With Golang, Redis and Server Side Events" (Oct 10, 2025), page "2" marker, GitHub "<account> › GoRealtimeWeb" "Examples how to write realtime web applications in Golang" (mentions Server-sent events, Long Polling, Websocket), Medium "Building a Real-Time Notification Service with Golang". Hovers several links (status-bar URLs) without opening.
- [63:34] Clicks the GoRealtimeWeb repo: `github.com/<account>/GoRealtimeWeb` loading.
- [63:35] Repo page: files .gitignore, LICENSE, README.md, event.html, longpoll.html, realtimeweb.go ("switch websocket from cod..."), script.js, socket.html. 221 stars, 29 forks; About: "Examples how to write realtime web applications in Golang".
- [63:43] Repo page scrolled: Contributors 2, Go 61.7% / JavaScript; README "GoRealtimeWeb ... This repository contains examples for the following real-time implementations:". Cursor hovering the `realtimeweb.go` link (tooltip `.../blob/master/realtimeweb.go`).

## File states

### chat.html (end of segment, ~63:43; latest visible states; no gutter, `~` = relative; ⟨?⟩ = not fully visible)
```
~1| <h1>Chat pages</h1>
~2| <div id="messages">
~3|     <div class="message">Hello</div>
~4|     <div class="message">How about</div>
~5| </div>
~6| <br><hr>
~7| <input type="text" id="inpMess" placeholder="Enter Message"><br><br>
~8| <button onclick="send()">Send</button>
~9|
~10|
~11| <script>
~12|     const inpMess = document.getElementById("inpMess");
~13|     const messagesElm = document.getElementById("messages");
~14|
~15|     async function send() {
~16|         try {
~17|             const data = {
~18|                 id: 1,
~19|                 message: inpMess.value
~20|             };
~21|             const response = await fetch("/api/send", {
~22|                 method: "POST",
~23|                 body: JSON.stringify(data),
~24|             });
~25|             if (!response.ok) {
~26|                 const result = await response.json();
~27|                 alert(result.message)
~28|                 return;
~29|             }
~30|
~31|             const result = await r⟨?⟩      (likely `response.json();`)
~32|             const message = result⟨?⟩      (not fully visible)
~33|
~34|             append⟨?⟩(...)                (call to appendMessage; args not visible; replaced `alert("Send " + messag...`)
~35|         } catch (error) {
~36|             console.error(error.me⟨?⟩      (likely error.message)
~37|         }
~38|     }
~39|
~40|     function appendMessage(mess) {
~41|         const messElm = document.createElement("div");
~42|         ?? (rest of function body not visible)
~43|     }
~44|
~45|
~46| </script>
```
Blank-line count between lines ~29-34 is uncertain (frames at 61:15 and 62:04 differ in scroll).
Intermediate [61:15] (mid-typing): `alert("Send " + messag...` still present; `function appendMessage(mess) {` / `    const messElm = document.c` (autocomplete open) / `}` then blank lines then `</script>`.

HTML changes this segment: line between `</div>` and `<input>`: (none) -> `<br>` -> `<br><br>` -> `<br><hr>` (final).

## Shell & outputs
No terminal visible in this segment. Browser outputs:
- [60:38] Firefox Console at `http://localhost:8080/web/chat.html`: warning `This page is in Quirks Mode. Page layout may be impacted. For Standards Mode use "<!DOCTYPE html>".` (chat.html); error `GET http://localhost:8080/favicon.ico [HTTP/1.1 404 Not Found 0ms]`. Page: "Chat pages" heading, messages (later `youtube` / `hello` / `what the he`), `<hr>`, text input, Send button.
- [62:17] pkg.go.dev/net/http#Request (header "GO http", package, standard library; dropdown "type Request"): `type Request struct {` / `// Method specifies the HTTP method (GET, POST, PUT, etc.).` / `// For client requests, an empty string means GET.` / `Method string` / `// URL specifies either the URI being requested (for server requests) or the URL to access (for client requests).` ... `URL *url.URL` / `Proto string // "HTTP/1.0"` / `ProtoMajor int // 1`.
- [62:19] go.dev/doc/articles/wiki/ "Introducing Function Literals and Closures":
  `func viewHandler(w http.ResponseWriter, r *http.Request, title string)` (same for editHandler, saveHandler);
  `func makeHandler(fn func (http.ResponseWriter, *http.Request, string)) http.HandlerFunc {` / `    return func(w http.ResponseWriter, r *http.Request) {` / `        // Here we will extract the page title from the Request,` / `        // and call the provided handler 'fn'` / `    }` / `}`.
- [62:40] pkg.go.dev/net:
  `conn, err := net.Dial("tcp", "golang.org:80")` / `if err != nil {` / `    // handle error` / `}` / `fmt.Fprintf(conn, "GET / HTTP/1.0\r\n\r\n")` / `status, err := bufio.NewReader(conn).ReadString('\n')` / `// ...`
- [62:43] `ln, err := net.Listen("tcp", ":8080")` / `if err != nil {` / `    // handle error` / `}` / `for {` / `    conn, err := ln.Accept()` / `    if err != nil {` / `        // handle error` / `    }` / `    go handleConnection(conn)` / `}`
- [62:58-63:06] zetcode.com/golang/socket/: `$ cat /etc/services | grep echo | head -4` -> `echo 7/tcp`, `echo 7/udp`, `echo 4/ddp # AppleTalk Echo Protocol`; `# apt install xinetd`; xinetd: `service echo { disable = no; type = INTERNAL; id = echo-dgram; socket_type = dgram; protocol = udp; user = root; wait = yes }`; `# systemctl start xinetd`; `func checkErr(err error) { if err != nil { log.Fatal(err) } }`; `con, err := net.Dial("udp", "debian:7")`; `_, err = con.Write([]byte(msg))`; `reply := make([]byte, 1024)`; `_, err = con.Read(reply)`; `fmt.Println("reply:", string(reply))`.
- [63:12-63:43] DuckDuckGo `golang realtime` results and GitHub GoRealtimeWeb repo page, as in Timeline.

## Observations
- Author is building a small chat UI (chat.html served by his Go server at /web/chat.html, posting to /api/send). He tweaks layout (br -> br br -> br hr) and adds a client-side `appendMessage` that creates a `div` per message; live test shows messages appending (youtube, hello, what the he).
- He then pivots to real-time delivery: reads Go docs for `http.Request`, the wiki tutorial's closure/makeHandler pattern, then searches `golang socket` (pkg.go.dev/net Dial/Listen/Accept, ZetCode UDP echo) and `golang realtime` (WebSockets, Server-Sent Events, long polling, Kafka/Redis/Centrifugo). Ends opening a public Go real-time examples repo (event.html = SSE, longpoll.html, socket.html, realtimeweb.go).
- Style: test-in-browser immediately, skim docs/search results rather than copy code, vanilla JS + standard library preference, researching the push-to-clients mechanism before implementing.
- URL-bar suggestions contained account-specific history (YouTube Studio pages); not recorded.
