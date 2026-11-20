# Segment 17/17 (83:51 - 89:56) - Hayyoth "a bit of Golang for fun"

Editor: VSCodium, workspace folder "lab" (window title e.g. `client.go — lab`). Editor line numbers are HIDDEN in this theme, so file listings below use sequential numbers of the visible/reconstructed text only (not editor line numbers). No terminal visible in this segment; no save messages shown.

Explorer tree (stable all segment): hello/hello.go; public/chat.html, public/index.html; socket/client.go, socket/hub.go; test/test.go; a.out; go.mod (yellow, "1" problem); go.sum; main.go. At 83:51-84:01 client.go/main.go shown red with error badge "1" (main.go: calling unexported `writePump`); fixed at ~84:07.

## Timeline

- [83:51] client.go open, cursor in `func (c *Client) WritePump()` (already renamed from lowercase `writePump`; the doc comment above still says `// writePump pumps messages from the hub to the websocket connection.`). client.go and main.go red with "1" error each. Firefox devtools behind (console: 404 favicon, `chat.html:51:21`).
- [83:52] client.go scrolled up: `var (newline, space)`, `var upgrader = websocket.Upgrader{...}`, `type Client struct`, readPump comment+start.
- [83:55] client.go top: Gorilla copyright header, package socket, imports, const block (writeWait/pongWait/pingPeriod/maxMessageSize). Mouse over main.go in explorer.
- [84:01] main.go open: at end of handler `client.writePump()` (lowercase, red squiggle under `w`itePump) - error since method is unexported. Cursor on it.
- [84:05] Firefox at http://localhost:8080/web/chat.html, page "Chat pages", input "what the he", Send button; Console: Quirks Mode warning, `GET http://localhost:8080/favicon.ico` 404, `Message from server  Hello Server!` at `chat.html:51:21`.
- [84:07] main.go now `client.WritePump()` (capitalized) - error cleared, main.go no longer red, explorer shows only go.mod "1".
- [84:11] Firefox console again (404 favicon `1ms`, "Message from server  Hello Server!").
- [84:21] main.go; Firefox devtools now Network tab partially visible behind.
- [84:25]-[84:26] client.go WritePump body reviewed (select / case message / ticker). [84:26] cursor after `ticker.Stop()`.
- [84:50]-[84:54] Firefox Network tab: rows: 200 GET localhost:8080 chat.html document html 1.67 kB/1.48 kB 6 ms; 101 GET ws (initiator chat.html:44 (we...)) plain 129 B 0 B; 404 GET favicon.ico img plain 176 B 19 B 1 ms; POST localhost:8080 send (initiator chat.html:20 (fet...)) pending/no status. "4 requests, 1.50 kB / 1.97 kB transferred, Finish: 2.72 s, DOMContentLoaded 26 ms, load 34 ms". Disable Cache checked.
- [85:04]-[85:40] Back in client.go, scrolling through WritePump (cursor on `ticker.Stop()`), reading code; no edits.
- [85:40] Switch to chat.html (tab public/chat.html): bottom of script: `function initSocket() {` containing `const socket = new WebSocket("ws://localhost:8080/ws");`, open listener sending "Hello Server!", message listener `console.log("Message from server ", event.data);`, then `initSocket()` call and `</script>`.
- [85:43] In initSocket body, the two addEventListener blocks selected (about to cut/move them out).
- [86:07]-[86:15] chat.html scrolled up to show `send()` fetch-based async function (try/catch, POST /api/send) and top of the file. Then [86:15] top of script shows `const socket = new WebSocket("ws://localhost:8080/ws");` now at script top level (initSocket wrapper removed, socket const moved up beside inpMess/messagesElm).
- [86:25] Body of send() (try { ... } catch ...) selected.
- [86:27] Selected try/catch body commented out line-by-line with `//` (Cmd+/): `// try {` ... `// }`.
- [86:29] Scrolled below: appendMessage, then the two socket.addEventListener blocks now at top level (indent 4 inside script), `initSocket()` still there, `</script>`.
- [86:35] Cursor/selection on `"message"` string in `socket.addEventListener("message"...` (word highlight).
- [86:49] Typing new first line in send(): `socket.send(inpMess.)` with JS completion popup (ATTRIBUTE_NODE ...) - finishing as `inpMess.value`.
- [86:52] Scrolled: commented block, appendMessage, listeners, `initSocket()` call still present at this point.
- [86:57] chat.html (sidebar back): send() = `socket.send(inpMess.value)` followed by commented-out try/catch block; `const socket = new WebSocket(...)` above.
- [87:02] Switch to main.go: block from `if messageReq.Message == "" {` through end of handler selected (old handler body).
- [87:03] main.go top: whole old structure selected from `// message from user send` through handler (type MessageReq, MessageRes, func handler...).
- [87:04] Selection of `func handler(...)` onward (from just under `type MessageRes`).
- [87:07] main.go: `func handler` block commented out with `//` (main.go now shows error badge "2" red). Cursor on empty line after MessageRes struct.
- [87:18] Scrolled further: inserted comment line `// don't need this any more` above `// func handler(...` ; whole commented handler selected. One un-commented stray whitespace line (`→ →`) between commented lines (after `// do something in here }`).
- [87:20] main.go bottom: `func main()` shown, with `http.HandleFunc("/api/send", handler)` still referencing now-commented `handler` -> red squiggle under `handler`.
- [87:23] main.go top: `"example.com/socket"` import, `var client *socket.Client`, MessageReq / MessageRes structs selected (about to delete).
- [87:24] After deletion: file is `package main`... imports, `var client *socket.Client`, then `func main()` directly. `/api/send` line still red (handler undefined). Error badge "2".
- [88:09] Firefox Network tab after reload: 200 chat.html 1.68 kB/1.50 kB 1 ms; 101 GET ws initiator chat.html:14; 404 favicon.ico. "3 requests ... Finish: 40 ms"; error count 3. Input still "what the he".
- [88:16] chat.html: script top with send() = `socket.send(inpMess.value)` + commented try/catch. Console shows error count 2 and entries at `chat.html:54:5` plus 404.
- [88:20] Firefox console cleared: only Quirks Mode warning and 404 favicon (1 error).
- [88:28] chat.html bottom: open listener now `//socket.send("Hello Server!");` (commented out); message listener still console.log; `initSocket()` call GONE (script ends after message listener, blank line, `</script>`). Console shows two entries `chat.html:51:17`.
- [88:39] send(): commented try/catch block selected (to delete).
- [88:40] Commented block deleted: send() is now `async function send() {` / `socket.send(inpMess.value)` / blank line (cursor) / `}`.
- [89:03] Firefox console: input "hayyoth here"; console lines `Message from server  what the he` and `Message from server  hayyoth here` (both `chat.html:51:17`) - server now echoes/broadcasts raw text over websocket.
- [89:12] A second Firefox window opened (empty, URL bar "Search with DuckDuckGo or enter address").
- [89:14] Window one now shows page rendered messages in the DOM: `hayyoth here` / `what hello` in div#messages, input "what hello"; URL bar dropdown shows suggestions (Hayyoth - hayyoth.org, Telegram Web, "(4) Hayyoth - YouTube", ChatGPT, Google Translate). Console: Quirks Mode + 404 favicon.
- [89:15] Window repositioned/resized; URL bar highlighted.
- [89:16] Second window URL typed `http://localhost:8080/web/chat.html` (Visit).
- [89:24] Two browser windows side by side: left window shows `hayyoth here`, `what hello`, `helloooooooooo`; right (new) window shows only `helloooooooooo` and input `helloooooooooo`, mouse on Send. => broadcast hub works across clients (new client only gets messages after connect).
- [89:39] chat.html in editor: message listener now `appendMessage(event.data);` (was console.log) - changed between 89:03 and 89:39.
- [89:40]-[89:46] Scrolling through chat.html; no edits.
- [89:55] First line inside `<script>` added: `// just for demo, don't use it in production` (cursor on that line). macOS menu bar visible: "VSCodium File Edit Selection View Go Run Terminal Window Help", clock "Thu Nov 5 12:07 PM".
- [89:56] Same; mouse near menu bar (about to stop screen recording). END of video.

## File states (end of segment)

### socket/client.go (only change in this segment's view: method is `WritePump` capitalized; doc comment not updated)
Line numbers hidden in editor; numbers below are sequential in what was visible.
```
1| // Copyright 2013 The Gorilla WebSocket Authors. All rights reserved.
2| // Use of this source code is governed by a BSD-style
3| // license that can be found in the LICENSE file.
4|
5| package socket
6|
7| import (
8| 	"bytes"
9| 	"log"
10| 	"net/http"
11| 	"time"
12|
13| 	"github.com/gorilla/websocket"
14| )
15|
16| const (
17| 	// Time allowed to write a message to the peer.
18| 	writeWait = 10 * time.Second
19|
20| 	// Time allowed to read the next pong message from the peer.
21| 	pongWait = 60 * time.Second
22|
23| 	// Send pings to peer with this period. Must be less than pongWait.
24| 	pingPeriod = (pongWait * 9) / 10
25|
26| 	// Maximum message size allowed from peer.
27| 	maxMessageSize = 512
28| )
29|
30| var (
31| 	newline = []byte{'\n'}
32| 	space   = []byte{' '}
33| )
34|
35| var upgrader = websocket.Upgrader{
36| 	ReadBufferSize:  1024,
37| 	WriteBufferSize: 1024,
38| }
39|
40| // Client is a middleman between the websocket connection and the hub.
41| type Client struct {
42| 	hub *Hub
43|
44| 	// The websocket connection.
45| 	conn *websocket.Conn
46|
47| 	// Buffered channel of outbound messages.
48| 	send chan []byte
49| }
50|
51| // readPump pumps messages from the websocket connection to the hub.
52| //
53| // The application runs readPump in a per-connection goroutine. The application
54| // ensures that there is at most one reader on a connection by executing all
55| // reads from this goroutine.
56| func (c *Client) readPump() {
57| 	defer func() {
58| 		c.hub.unregister <- c
59| 		c.conn.Close()
60| 	}()
61| 	c.conn.SetReadLimit(maxMessageSize)
62| 	?? (SetReadDeadline line not visible)
63| 	c.conn.SetPongHandler(func(string) error { c.conn.SetReadDeadline(time.Now().Ad⟨?⟩ ?? (cut off at right edge)
64| 	for {
65| 		_, message, err := c.conn.ReadMessage()
66| 		if err != nil {
67| 			if websocket.IsUnexpectedCloseError(err, websocket.CloseGoingAway, webs⟨?⟩ ?? (cut off)
68| 				log.Printf("error: %v", err)
69| 			}
70| 			break
71| 		}
72| 		message = bytes.TrimSpace(bytes.Replace(message, newline, space, -1))
73| 		c.hub.broadcast <- message
74| 	}
75| }
76|
77| // writePump pumps messages from the hub to the websocket connection.
78| //
79| // A goroutine running writePump is started for each connection. The
80| // application ensures that there is at most one writer to a connection by
81| // executing all writes from this goroutine.
82| func (c *Client) WritePump() {
83| 	ticker := time.NewTicker(pingPeriod)
84| 	defer func() {
85| 		ticker.Stop()
86| 		c.conn.Close()
87| 	}()
88| 	for {
89| 		select {
90| 		case message, ok := <-c.send:
91| 			c.conn.SetWriteDeadline(time.Now().Add(writeWait))
92| 			if !ok {
93| 				// The hub closed the channel.
94| 				c.conn.WriteMessage(websocket.CloseMessage, []byte{})
95| 				return
96| 			}
97|
98| 			w, err := c.conn.NextWriter(websocket.TextMessage)
99| 			if err != nil {
100| 				return
101| 			}
102| 			w.Write(message)
103|
104| 			// Add queued chat messages to the current websocket message.
105| 			n := len(c.send)
106| 			for i := 0; i < n; i++ {
107| 				w.Write(newline)
108| 				w.Write(<-c.send)
109| 			}
110|
111| 			if err := w.Close(); err != nil {
112| 				return
113| 			}
114| 		case <-ticker.C:
115| 			c.conn.SetWriteDeadline(time.Now().Add(writeWait))
116| 			if err := c.conn.WriteMessage(websocket.PingMessage, nil); err != nil ⟨{ cut off⟩
117| 				return
118| 			}
119| 		}
120| 	}
121| }
122|
123| // serveWs handles websocket requests from the peer.
124| func ServeWs(hub *Hub, w http.ResponseWriter, r *http.Request) *Client {
125| 	conn, err := upgrader.Upgrade(w, r, nil)
126| 	if err != nil {
127| 		log.Println(err)
128| 		return nil
129| 	?? (rest of ServeWs not visible in this segment)
```
(Doc comment says `// serveWs ...` while function is `ServeWs` and returns `*Client`; `WritePump` comment still says writePump.)

### main.go (end of segment, as last seen at 87:24; handler + request/response types deleted)
Top of file (package/imports) only partly visible.
```
?? (package main + earlier imports not visible; last import line before `)` is:)
 1| 	"example.com/socket"
 2| )
 3|
 4| var client *socket.Client
 5|
 6| func main() {
 7| 	flag.Parse()
 8| 	hub := socket.NewHub()
 9| 	go hub.Run()
10|
11| 	fs := http.FileServer(http.Dir("./public"))
12| 	http.Handle("/web/", http.StripPrefix("/web/", fs))
13|
14| 	http.HandleFunc("/api/send", handler)
15| 	http.HandleFunc("/ws", func(w http.ResponseWriter, r *http.Request) {
16| 		client = socket.ServeWs(hub, w, r)
17| 	})
18|
19| 	log.Fatal(http.ListenAndServe(":8080", nil))
20| }
```
(`handler` is undefined at this point -> red squiggle; main.go badge "2". Any later fix, e.g. removing the `/api/send` line, was not seen.)

Intermediate main.go (84:01-87:03) with the old handler; commented out at 87:07 (with `// don't need this any more`), deleted ~87:24:
```
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
			Message: "can't decode message",
		}
		json.NewEncoder(w).Encode(response)
		return
	}
	if messageReq.Message == "" {
		response := MessageRes{
			Status:  "error",
			Message: "message empty",
		}
		responseByte, err := json.Marshal(response)
		if err != nil {
			// do something in here
		}

		http.Error(w, string(responseByte), http.StatusBadRequest)
		return
	}

	response := MessageRes{
		Status:  "ok",
		Message: messageReq.Message,
	}

	client.WritePump()

	json.NewEncoder(w).Encode(response)
}
```
(At 84:01 the call was `client.writePump()` with a red error; fixed to `client.WritePump()` at 84:07.)

### public/chat.html (end of segment, 89:55)
Anything above `<h1>` not visible. No `<!DOCTYPE html>` (Firefox reports Quirks Mode).
```
1| <h1>Chat pages</h1>
2| <div id="messages">
3| </div>
4| <br>
5| <hr>
6| <input type="text" id="inpMess" placeholder="Enter Message"><br><br>
7| <button onclick="send()">Send</button>
8|
9|
10| <script>
11| 	// just for demo, don't use it in production
12| 	const inpMess = document.getElementById("inpMess");
13| 	const messagesElm = document.getElementById("messages");
14|
15| 	const socket = new WebSocket("ws://localhost:8080/ws");
16|
17| 	async function send() {
18| 		socket.send(inpMess.value)
19| 	}
20|
21| 	function appendMessage(mess) {
22| 		const messElm = document.createElement("div");
23| 		messElm.innerText = mess;
24| 		messagesElm.appendChild(messElm);
25| 	}
26|
27| 	socket.addEventListener("open", (event) => {
28| 		//socket.send("Hello Server!");
29| 	});
30|
31| 	socket.addEventListener("message", (event) => {
32| 		appendMessage(event.data);
33| 	});
34|
35| </script>
```
(Indent is 4 columns inside script; blank lines 8-9 and 34 as seen at 89:43/89:46.)

Intermediate chat.html (85:40, before refactor): script contained inpMess, messagesElm, `async function send()` with `try { const data = { id: 1, message: inpMess.value }; const response = await fetch("/api/send", { method: "POST", body: JSON.stringify(data), }); if (!response.ok) { const result = await response.json(); alert(result.message) return; } const result = await response.json(); appendMessage(result.message) } catch (error) { console.error(error.message); }`, appendMessage, then
```
function initSocket() {
    const socket = new WebSocket("ws://localhost:8080/ws");

    socket.addEventListener("open", (event) => {
        socket.send("Hello Server!");
    });

    socket.addEventListener("message", (event) => {
        console.log("Message from server ", event.data);
    });
}

initSocket()
```

## Shell & outputs
No terminal visible in this segment. Browser outputs only (see timeline): Firefox console `Message from server  Hello Server!` (chat.html:51:21, 84:05); later `Message from server  what the he` and `Message from server  hayyoth here` (chat.html:51:17, 89:03); warning `This page is in Quirks Mode. Page layout may be impacted. For Standards Mode use "<!DOCTYPE html>". Learn More`; `GET http://localhost:8080/favicon.ico [HTTP/1.1 404 Not Found 0ms]`. Network tab rows as in 84:54 and 88:09. Final UI: page "Chat pages" at http://localhost:8080/web/chat.html listing messages (`hayyoth here`, `what hello`, `helloooooooooo`) in a div, text input, Send button; two Firefox windows both receiving broadcast.

## Observations
- Goal of segment: wire the Gorilla chat-example hub/client into a custom main.go HTTP + WebSocket server, replacing the earlier REST `/api/send` JSON handler with a pure websocket flow: `socket.send(inpMess.value)` -> server readPump -> hub broadcast -> each client's WritePump -> `appendMessage(event.data)`.
- Exported names by necessity: `writePump` -> `WritePump` after the editor error when called from package main; `ServeWs` exported and returns `*Client`; main keeps a package-level `var client *socket.Client`.
- Style: comments out old code before deleting it (whole `handler`, then the try/catch, with the note `// don't need this any more`), deletes after verifying. Uses Firefox devtools console/network as the verification loop; no terminal in this segment. Removes the `initSocket()` wrapper to simplify.
- Last edit is a disclaimer comment `// just for demo, don't use it in production`; video ends as he moves to the menu bar to stop recording.
- Uses Gorilla WebSocket chat example code largely verbatim (license header, pumps, ping/pong consts).
