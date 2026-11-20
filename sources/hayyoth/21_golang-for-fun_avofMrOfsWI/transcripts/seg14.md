# Segment 14 of 17 (69:37 - 72:18), video avofMrOfsWI "a bit of Golang for fun"

Setup on screen: macOS; left window = code editor, title bar `main.go — lab` (light theme, NO line numbers shown, no terminal visible in this segment); right = Firefox window with several tabs; bottom right = webcam overlay (ignored). Editor sidebar (visible at 72:18) is project `LAB`.

## Timeline

- [69:37] Firefox on `pkg.go.dev/golang.org/x/net/websocket#exampl...` (header "GO websocket package"). Scrolled to API list: `func (*Conn) Config`, `IsClientConn`, `IsServerConn`, `LocalAddr`. Editor (behind, partially covered) shows the end of `handler`, `func initSocket()` with the Dial/Write/Read code, and `main`.
- [69:43] Same page, scrolled to `func (Server) ServeHTTP(w http.ResponseWriter, req *http.Request)` ("ServeHTTP implements the http.Handler interface for a WebSocket"), then "Source Files": client.go, dial.go, hybi.go, server.go, websocket.go.
- [69:44] Scrolled up to the package index: `type DialError`, `type Handler` (`func (h Handler) ServeHTTP(w http.ResponseWriter, req *http.Request)`), `type ProtocolError`, `type Server`; "Examples": Dial, Handler; "Constants" block (`ProtocolVersionHybi13 = 13`, `ProtocolVersionHybi = ProtocolVersionHybi13`, `SupportedProtocolVersion = "13"`, frame constants, `DefaultMaxPayloadBytes = 32 << 20 // 32MB`).
- [69:47] Tab list dropdown opened ("Current window"): `localhost:8080/web/chat.html`, `Working with Sockets in Go`, `net package - net - Go Packages`, `GitHub - SimonWaldherr/GoRealtimeWeb: Examples how to...`, `GoRealtimeWeb/realtimeweb.go at master · SimonWaldherr...`, `websocket package - golang.org/x/net/websocket - Go P...` (active).
- [69:49] Switched to GitHub `SimonWaldherr/GoRealtimeWeb / realtimeweb.go` (217 lines, 196 loc, 5.27 KB), lines 17-42: `type Connections struct { clients map[chan string]bool; addClient chan chan string; removeClient chan chan string; messages chan string }`, `var hub = &Connections{...make(...)}`, `func (hub *Connections) Init()` with `go func() { for { select { case s := <-hub.addClient: hub.clients[s] = true; log.Println("Added new client") case s := <-hub.removeClient: delete(hub.clients, s) ...`.
- [70:03] Editor brought to front (title `main.go — lab`), showing the full current `initSocket` and `main` (see File states). Cursor on `ws` in `ws.Write`; `ws` occurrences highlighted. Behind: GitHub page.
- [70:13] Firefox: realtimeweb.go lines 54 and 104-128: `func websocketHandler(ws *websocket.Conn)` with `var in string`, `messageChannel := make(chan string)`, `hub.addClient <- messageChannel`, `for i := 0; i < 1440; {`, `select { case msg := <-messageChannel: jsonData, _ := json.Marshal(msg) ...`, `in = fmt.Sprintf("{\"str\": %s, \"time\": \"%v\"}\n\n", str, time.Now())`, `websocket.Message.Send(ws, in)`, `case <-time.After(time.Second * 60):` ... Editor is scrolled back to the top: `package main`, imports, `MessageReq`, `MessageRes`, `handler`.
- [70:14] New tab: URL bar typing `golang` (Firefox Suggest list shows websocket package, etc.).
- [70:18-70:19] DuckDuckGo search `golang x/net/websocket`; results: pkg.go.dev websocket package, net module, github golang/net websocket.go, deepwiki "WebSocket Implementation | golang/net".
- [70:21-70:23] Scrolling results: deepwiki, websocket.org "Go WebSocket Server Guide: coder/websocket vs Gorilla", gowalker.org, GitHub coder/websocket, Stack Overflow.
- [70:31] Opened `websocket.org/guides/languages/go/`: "Go WebSocket Server Guide: coder/websocket vs Gorilla" (by Matthew O'Riordan, published Sep 2, 2024, updated Mar 14, 2026). Quick Answer box: "Use coder/websocket (github.com/coder/websocket) for new Go WebSocket projects. It supports context.Context for cancellation and timeouts, handles concurrent writes safely, and is actively maintained. gorilla/websocket still works but is archived."
- [70:32] URL bar typing `google.com/` (autocomplete; abandoned). [70:34] typed `golang mo`, then [70:35] `golang mux`; the page text shows "Go's goroutine-per-connection model is a natural fit for WebSocket servers. Each connection gets its own goroutine, blocking on reads, no callback chains. The code reads like synchronous logic but handles thousands of connections concurrently." and heading "Which library to use".
- [70:35] DuckDuckGo results for `golang mux`: GitHub gorilla/mux ("HTTP request multiplexer"), pkg.go.dev mux package, Better Stack gorilla mux article.
- [70:37-70:40] `github.com/gorilla/mux`: 21.8k stars, 1.9k forks, Release v1.8.1 (Latest), 15 releases, "Used by 210K", files listing (bench_test.go, doc.go, middleware.go, mux.go, regexp.go ...). README: "It implements the http.Handler interface so it is compatible with the standard http.ServeMux", match on host/path/prefix/schemes/header/query/methods, variables with regex, reversed URLs, subrouters. TOC: Install, Examples, Matching Routes, Static Files, Serving Single Page Applications, Registered URLs, Walking Routes, Graceful Shutdown, Middleware, Handling CORS Requests, Testing Handlers, Full Example.
- [70:45] Back at top of gorilla/mux repo page.
- [70:48-70:53] URL bar `golang `; search `golang mux socket`: results pkg.go.dev mux, github gorilla/mux, github onionj/websocket-mux ("Muxr: Efficient WebSocket Multi..."), Stack Overflow "How to use gorilla websocket and mux in the same application".
- [70:55-71:00] Back to gorilla/mux page; clicked link `gorilla.github.io`; "Gorilla web toolkit" page ("A helpful toolkit for the Go programming language that provides useful, composable packages for writing HTTP-based applications."). Then back to the repo page and again to gorilla.github.io.
- [71:02-71:05] Scrolled Gorilla toolkit package cards: gorilla/mux, reverse, rpc, schema, securecookie, sessions, websocket ("provides a complete and tested implementation of the WebSocket protocol."), csrf, handlers, pat; cursor on websocket card; "Installation" heading below.
- [71:05-71:09] Navigated to `github.com/gorilla/websocket` (24.9k stars, 3.6k forks, "Package gorilla/websocket is a fast, well-tested and widely used WebSocket implementation for Go."). README: Documentation links (API Reference, Chat example, Command example, Client and server example, File watch example), Status ("complete and tested implementation of the WebSocket protocol. The package API is stable."), Installation `go get github.com/gorilla/websocket`, Protocol Compliance (Autobahn Test Suite).
- [71:17-71:25] Clicked "Chat example": `github.com/gorilla/websocket/tree/main/examples/chat` containing README.md, client.go, home.html, hub.go, main.go. README "Chat Example": run with `$ go get github.com/gorilla/websocket`, `$ cd `go list -f '{{.Dir}}' github.com/gorilla/websocket/examples/chat``, `$ go run *.go`; "To use the chat example, open http://localhost:8080/ in your browser." Server section: types `Client` and `Hub`; one goroutine for the Hub, two per Client; channels `register`, `unregister`, `broadcast`; clients map with value always true; unregister closes the client's `send` channel; full `send` buffer means client is dead/stuck. Client section: `serveWs`, `writePump`; Frontend section: `home.html`, `appendLog`. Cursor hovers `home.html`/`hub.go`.
- [71:51] Editor brought to front (cursor at end of `MessageReq` struct closing brace). Firefox behind is on `.../blob/main/exa...` with a "Symbols" panel (Hub, newHub, run) = hub.go.
- [71:57] Editor scrolled down: **`initSocket` body has been emptied** (`func initSocket() {` blank line `}`) and in `main` a new line `init` (red squiggle, unfinished) replaced `initSocket()`; the old initSocket body (Dial/Write/Read code) is gone. Firefox: gorilla `hub.go` (53 lines, 46 loc, 1.12 KB): `type Hub struct { clients map[*Client]bool; broadcast chan []byte; register chan *Client; unregister chan *Client }`, `func newHub() *Hub { return &Hub{ broadcast: make(chan []byte), ...`.
- [72:00-72:01] Firefox: gorilla chat `main.go` (40 lines, 35 loc, 893 Bytes), imports `flag`, `log`, `net/http`; lines 15-40: `serveHome` (method check, `http.ServeFile(w, r, "home.html")`) and `main` (`flag.Parse()`, `hub := newHub()`, `go hub.run()`, `http.HandleFunc("/", serveHome)`, `http.HandleFunc("/ws", func(w http.ResponseWriter, r *http.Request) { serveWs(hub, w, r) })`, `err := http.ListenAndServe(*addr, nil)`, `log.Fatal("ListenAndServe: ", err)`).
- [72:06] Editor front: cursor in `http.HandleFunc("/api/send", handler)` selecting `api`. `init` still on the line below (red squiggle). Editor right edge shows red markers (errors).
- [72:18] Editor sidebar now open (explorer `LAB`): `hello/hello.go`, `public/chat.html`, `public/index.html`, `test/test.go`, `a.out`, `go.mod`, `go.sum`, `hub.go` (highlighted, selected), `main.go` (red, badge `3` = 3 problems). So a `hub.go` file exists in the project. In Firefox, lines 33-35 of gorilla main.go (`http.HandleFunc("/ws", func(...) { serveWs(hub, w, r) })`) are being selected (copy).

No save messages visible in this segment.

## File states (end of segment, ~72:06; this editor shows no line numbers, so lines are unnumbered; ?? = not visible)

### main.go (project `lab`)

```
package main

import (
    "encoding/json"
    "golang.org/x/net/websocket"
    "log"
    "net/http"
)

// message from user send
type MessageReq struct {
    Id      int    `json:"id"`
    Message string `json:"message"`
}

// message return for user
type MessageRes struct {
    Status  string `json:"status"`
    Message string `json:"message"`
}

func handler(w http.ResponseWriter, r *http.Request) {
    w.Header().Set("Content-Type", "application/json")

    decoder := json.NewDecoder(r.Body)
    var messageReq MessageReq
    err := decoder.Decode(&messageReq)
    if err != nil {
        response := MessageRes{
            Status:  "error",
            ?? (lines not visible)
        }
        ?? (lines not visible)
    }

    response := MessageRes{
        Status:  "ok",
        Message: messageReq.Message,
    }

    json.NewEncoder(w).Encode(response)
}

func initSocket() {

}

func main() {
    fs := http.FileServer(http.Dir("./public"))
    http.Handle("/web/", http.StripPrefix("/web/", fs))

    http.HandleFunc("/api/send", handler)

    init
    log.Fatal(http.ListenAndServe(":8080", nil))
}
```

All import strings are red-underlined in the frames (likely unresolved/unused state).

### main.go - initSocket intermediate version (70:03, before being emptied)

```
func initSocket() {
    origin := "http://localhost/"
    url := "ws://localhost:3000/ws"
    ws, err := websocket.Dial(url, "", origin)
    if err != nil {
        log.Fatal(err)
    }
    if _, err := ws.Write([]byte("hello, world!\n")); err != nil {
        log.Fatal(err)
    }
    var msg = make([]byte, 512)
    var n int
    if n, err = ws.Read(msg); err != nil {
        log.Fatal(err)
    }
    fmt.Printf("Received: %s.\n", msg[:n])
}

func main() {
    fs := http.FileServer(http.Dir("./public"))
    http.Handle("/web/", http.StripPrefix("/web/", fs))

    http.HandleFunc("/api/send", handler)

    initSocket()
    log.Fatal(http.ListenAndServe(":8080", nil))
}
```
(`fmt` is red-squiggled: not imported. This body is the pkg.go.dev websocket Dial example. By 71:57 it is gone and `main` has an incomplete `init`.)

### hub.go: exists per sidebar at 72:18; content not visible.
### Other files seen only in sidebar: hello/hello.go, public/chat.html, public/index.html, test/test.go, a.out, go.mod, go.sum.

## Shell & outputs

No terminal visible in this segment; no shell commands. Browser-rendered pages are described in the Timeline (pkg.go.dev websocket package, DuckDuckGo results, websocket.org Go guide, gorilla/mux repo + README, gorilla.github.io toolkit, gorilla/websocket repo + chat example README/hub.go/main.go). Commands quoted from the gorilla chat README (rendered page):
```
$ go get github.com/gorilla/websocket
$ cd `go list -f '{{.Dir}}' github.com/gorilla/websocket/examples/chat`
$ go run *.go
```

## Observations

- Pivot at 70:03-70:19: he had a client-side `websocket.Dial` experiment (`initSocket`, targeting `ws://localhost:3000/ws`) based on the x/net/websocket docs, then moved toward a server-side broadcast hub; he consulted SimonWaldherr/GoRealtimeWeb (channel-based `Connections` hub), then searched for alternatives (coder/websocket vs gorilla, mux).
- He read the websocket.org guide saying gorilla is archived and coder/websocket is preferred, but went to gorilla/mux and gorilla/websocket anyway, settling on the gorilla/websocket chat example (hub.go, client.go, main.go) as the model. He stripped `initSocket`'s body and began a new call (`init...`) in `main`, and a `hub.go` file exists by 72:18. At the end he copies the gorilla `main.go` `/ws` handler lines (33-35).
- Style: copies idioms from docs/examples, searches via the URL bar (DuckDuckGo), reads READMEs for architecture (Hub plus per-client goroutines and channels). Existing handler keeps `/api/send` and the static `/web/` file server on :8080.
