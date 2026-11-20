# seg16 (77:16 - 83:51) - Go chat server with gorilla/websocket; MDN WebSocket docs; console WS experiments

Editor has no line-number gutter visible (VSCodium-like). Line prefixes below are my own sequence numbers, NOT editor numbers. No save messages visible in this segment.

## Timeline
- [77:16] Firefox, DuckDuckGo results for `js websocket`. Results: MDN "WebSocket - Web APIs | MDN" (sub-links bufferedAmount, readyState, Message, Writing WebSocket Client Applic..., The WebSocket API, EventTarget); "WebSocket API (WebSockets) - Web APIs | MDN - MDN Web Docs" (Oct 22, 2026); websocket.org "JavaScript WebSocket: Browser API & Node.js Server". Mouse over readyState.
- [77:17] Opened developer.mozilla.org/en-US/docs/Web/API/WebSocket. Baseline "WIDELY AVAILABLE"; note "This feature is available in Web Workers"; backpressure note mentioning WebSocketStream; sidebar: Constructor WebSocket(); Instance properties binaryType, bufferedAmount, extensions, protocol, readyState, url; Instance methods close(), send().
- [77:18-77:20] Scrolls: Constructor "WebSocket() Returns a newly created WebSocket object."; Instance properties (binaryType "The binary data type used by the connection.", bufferedAmount "The number of bytes of queued data.", extensions "The extensions selected by the server.", protocol "The sub-protocol selected by the server.", readyState "The current state of the connection.", url "The absolute URL of the WebSocket."); Events: close, error ("Fired when a connection with a WebSocket has been closed because of an error, such as when some data couldn't be sent. Also available via the onerror property."), message, open.
- [77:22] Examples, JS snippet (verbatim):
  ```
  // Create WebSocket connection.
  const socket = new WebSocket("ws://localhost:8080");

  // Connection opened
  socket.addEventListener("open", (event) => {
    socket.send("Hello Server!");
  });

  // Listen for messages
  socket.addEventListener("message", (event) => {
    console.log("Message from server ", event.data);
  });
  ```
- [77:23-77:24] Mission-control / window switch (Desktop 3): VSCodium window "index.html - lab" (contents `<h1>Hello World</h1>` / `<p>Hayyoth here</p>`), terminal shows `lab $ go run .` (server running, no output). Explorer LAB: hello/hello.go; public/chat.html, public/index.html; socket/client.go, socket/hub.go; test/test.go; a.out; go.mod (1 problem); go.sum; main.go.
- [77:28] Back to MDN, mouse near Copy button.
- [77:37] Firefox "List all tabs" menu (current window): localhost:8080/web/chat.html; Working with Sockets in Go; net package - net - Go Packages; GitHub - SimonWaldherr/GoRealtimeWeb: Examples how to...; GoRealtimeWeb/realtimeweb.go at master - SimonWaldherr...; websocket package - golang.org/x/net/websocket - Go Pac...; GitHub - gorilla/mux: Package gorilla/mux is a powerful HT...; Gorilla, the golang web toolkit; GitHub - gorilla/websocket: Package gorilla/websocket is a ...; websocket/examples/chat/client.go at main - gorilla/websoc...; WebSocket - Web APIs | MDN (bold/current).
- [77:39] http://localhost:8080/web/chat.html, heading "Chat pages"; messages listed: youtube / hello / what the he; input containing "what the he"; Send button. DevTools Console: warning `This page is in Quirks Mode. Page layout may be impacted. For Standards Mode use "<!DOCTYPE html>". Learn More`; red `GET http://localhost:8080/favicon.ico`.
- [77:49] Back to MDN; line `const socket = new WebSocket("ws://localhost:8080");` highlighted (selected).
- [77:54] In chat.html console he entered: `const socket = new WebSocket("ws://localhost:8080/ws");` -> `undefined` (path /ws added vs MDN example).
- [78:01-78:02] DevTools Network: 200 GET chat.html document html 1.30 kB / 1.11... ; 404 GET favicon.ico img plain 176 B / 19 B; 200 POST send (chat.html:1...) json 144 B / 36 B; 200 POST send json 142 B / 34 B; 200 POST send json 148 B / 40 B; **101 GET ws /:1 (websoc... plain 129 B / 0 B** (selected). Right pane tabs Headers/Cookies/Request/Response(selected)/Timings/Stack Trace; "No messa(ges)". Footer: `6 requests  1.24 kB / 2.04 kB transferred  Finish: 15.72 min  DOMContentLoaded: 19 ms`. The three POST `send` rows are earlier fetch calls to /api/send.
- [78:04] MDN: the `socket.addEventListener("open", ...)` block selected (copying).
- [78:07-78:15] Toggles Network (ws row, still "No messages") / Console. Console now has three entered statements, each `undefined`:
  ```
  const socket = new WebSocket("ws://localhost:8080/ws");
  socket.addEventListener("open", (event) => {
    socket.send("Hello Server!");
  });
  socket.addEventListener("message", (event) => {
    console.log("Message from server ", event.data);
  });
  ```
  Favicon line shows `[HTTP/1.1 404 Not Found 0ms]` source chat.html.
- [78:21] Console history selected (all three statements highlighted) to copy.
- [78:22] Mission control: main.go window shows handler `socket.ServeWs(hub, w, r)`; browser DevTools behind.
- [78:23-78:26] VSCodium main.go (explorer main.go selected) over browser. Terminal: `lab $ go run .` still running.
- [78:27] Browser window resized (viewport 964px x 189.5px), URL bar clicked.
- [78:29] Back to main.go.
- [78:59] Browser in front; editor behind shows chat.html JS tail.
- [79:05] Network after reload: 200 GET localhost:8080 chat.html document html 1.67 kB / 1.48 kB 5 ms; **101 GET localhost:8080 ws chat.html:44 (we... plain 129 B / 0 B 1 ms**; 404 GET favicon.ico img plain 176 B / 19 B 0 ms. Footer `3 requests  1.50 kB / 1.97 kB transferred  Finish: 46 ms  DOMContentLoaded: 29 ms  load: 42 ms`. Message list empty now.
- [79:16] chat.html in editor (title "chat.html - lab") scrolled to bottom: he has ALREADY added initSocket() with open/message listeners (File states). Console link `chat.html:51:21` is the console.log site.
- [79:25-79:29] client.go opened (gorilla chat example): readPump, writePump, ServeWs.
- [79:33] Browser console: `Message from server  Hello Server!` at chat.html:51:21 (server broadcast his hello back). Quirks warning + favicon 404 still present.
- [79:43] client.go ServeWs end visible, cursor near `// new goroutines.`
- [80:24-80:51] Reading writePump (cursor on `writePump`, word-highlight of `message`); editor scrolled horizontally by 1 char at 80:24.
- [81:00-81:01] Editor window resized wider.
- [81:12-81:18] Reading readPump/writePump.
- [81:20, 81:30] hub.go opened (Hub struct, NewHub, Run).
- [82:33] client.go being edited: ServeWs now returns `*Client`; typing `return cli` with autocomplete popup (client *Client; client.hub.register chan *Client; client.hub.unregister chan *Client; &Client{}; Client struct{...}; hub.clients map[*Client]bool; clientcredentials golang.org/x/oauth2/clientcrede...). Explorer: client.go red "2" (errors), socket folder red dot.
- [83:23] main.go: added global `var client *socket.Client` (selected) above `func main()`; /ws handler now `client = socket.ServeWs(hub, w, r)`. Explorer shows no error badges at that instant (except go.mod 1).
- [83:29] main.go scrolled up: MessageRes struct, handler().
- [83:47-83:49] client.go readPump/writePump; Explorer: client.go selected, go.mod 1, main.go red "1".
- [83:51] **writePump renamed to `WritePump`** (`func (c *Client) WritePump() {`; comment above still says `// writePump pumps ...`). Explorer: client.go red "1", main.go red "1", socket folder red dot. Cursor after `defer func() {`.

## File states

### main.go (pieces from 83:23/83:29; top of file not visible)
```
?? (earlier lines not visible)
 1| 	Status  string `json:"status"`
 2| 	Message string `json:"message"`
 3| }
 4|
 5| func handler(w http.ResponseWriter, r *http.Request) {
 6| 	w.Header().Set("Content-Type", "application/json")
 7|
 8| 	decoder := json.NewDecoder(r.Body)
 9| 	var messageReq MessageReq
10| 	err := decoder.Decode(&messageReq)
11| 	if err != nil {
12| 		response := MessageRes{
13| 			Status:  "error",
14| 			Message: "can't decode message",
15| 		}
16| 		json.NewEncoder(w).Encode(response)
17| 		return
18| 	}
19| 	if messageReq.Message == "" {
20| 		response := MessageRes{
21| 			Status:  "error",
22| 			Message: "message empty",
23| 		}
24| 		responseByte, err := json.Marshal(response)
25| 		if err != nil {
26| 			// do something in here
27| 		}
28|
29| 		http.Error(w, string(responseByte), http.StatusBadRequest)
30| 		return
31| 	}
32|
33| 	response := MessageRes{
34| 		Status:  "ok",
35| 		Message: messageReq.Message,
36| 	}
37|
38| 	json.NewEncoder(w).Encode(response)
39| }
40|
41| var client *socket.Client
42|
43| func main() {
44| 	flag.Parse()
45| 	hub := socket.NewHub()
46| 	go hub.Run()
47|
48| 	fs := http.FileServer(http.Dir("./public"))
49| 	http.Handle("/web/", http.StripPrefix("/web/", fs))
50|
51| 	http.HandleFunc("/api/send", handler)
52| 	http.HandleFunc("/ws", func(w http.ResponseWriter, r *http.Request) {
53| 		client = socket.ServeWs(hub, w, r)
54| 	})
55|
56| 	log.Fatal(http.ListenAndServe(":8080", nil))
57| }
```
Earlier (78:22): handler body was just `socket.ServeWs(hub, w, r)`; no `var client`.

### public/chat.html (tail, ~79:16; head not visible)
```
?? (head not visible)
 1| 	}
 2|
 3| 	function appendMessage(mess) {
 4| 		const messElm = document.createElement("div");
 5| 		messElm.innerText = mess;
 6| 		messagesElm.appendChild(messElm);
 7| 	}
 8|
 9| 	function initSocket() {
10| 		const socket = new WebSocket("ws://localhost:8080/ws");
11|
12| 		socket.addEventListener("open", (event) => {
13| 			socket.send("Hello Server!");
14| 		});
15|
16| 		socket.addEventListener("message", (event) => {
17| 			console.log("Message from server ", event.data);
18| 		});
19| 	}
20|
21| 	initSocket()
22|
23|
24| </script>
```
(Indent is 4 columns per level; tab vs spaces unclear. Page is in quirks mode: no DOCTYPE.)

### public/index.html
```
1| <h1>Hello World</h1>
2| <p>Hayyoth here</p>
```

### socket/client.go (gorilla chat example; seen 79:25-83:51)
```
?? (head: package, imports, constants, upgrader, Client struct - only tail visible)
   	// Buffered channel of outbound messages.
   	send chan []byte
   }

   // readPump pumps messages from the websocket connection to the hub.
   //
   // The application runs readPump in a per-connection goroutine. The application
   // ensures that there is at most one reader on a connection by executing all
   // reads from this goroutine.
   func (c *Client) readPump() {
   	defer func() {
   		c.hub.unregister <- c
   		c.conn.Close()
   	}()
   	c.conn.SetReadLimit(maxMessageSize)
   	c.conn.SetReadDeadline(time.Now().Add(pongWait))
   	c.conn.SetPongHandler(func(string) error { c.conn.SetReadDeadline(time.Now().Ad⟨cut off⟩
   	for {
   		_, message, err := c.conn.ReadMessage()
   		if err != nil {
   			if websocket.IsUnexpectedCloseError(err, websocket.CloseGoingAway, webs⟨cut off⟩
   				log.Printf("error: %v", err)
   			}
   			break
   		}
   		message = bytes.TrimSpace(bytes.Replace(message, newline, space, -1))
   		c.hub.broadcast <- message
   	}
   }

   // writePump pumps messages from the hub to the websocket connection.
   //
   // A goroutine running writePump is started for each connection. The
   // application ensures that there is at most one writer to a connection by
   // executing all writes from this goroutine.
   func (c *Client) WritePump() {          // was writePump until 83:51
   	ticker := time.NewTicker(pingPeriod)
   	defer func() {
   		ticker.Stop()
   		c.conn.Close()
   	}()
   	for {
   		select {
   		case message, ok := <-c.send:
   			c.conn.SetWriteDeadline(time.Now().Add(writeWait))
   			if !ok {
   				// The hub closed the channel.
   				c.conn.WriteMessage(websocket.CloseMessage, []byte{})
   				return
   			}

   			w, err := c.conn.NextWriter(websocket.TextMessage)
   			if err != nil {
   				return
   			}
   			w.Write(message)

   			// Add queued chat messages to the current websocket message.
   			n := len(c.send)
   			for i := 0; i < n; i++ {
   				w.Write(newline)
   				w.Write(<-c.send)
   			}

   			if err := w.Close(); err != nil {
   				return
   			}
   		case <-ticker.C:
   			c.conn.SetWriteDeadline(time.Now().Add(writeWait))
   			if err := c.conn.WriteMessage(websocket.PingMessage, nil); err != nil {
   				return
   			}
   		}
   	}
   }
```
ServeWs original (79:29 / 79:43):
```
   // serveWs handles websocket requests from the peer.
   func ServeWs(hub *Hub, w http.ResponseWriter, r *http.Request) {
   	conn, err := upgrader.Upgrade(w, r, nil)
   	if err != nil {
   		log.Println(err)
   		return
   	}
   	client := &Client{hub: hub, conn: conn, send: make(chan []byte, 25⟨cut: 256⟩)}
   	client.hub.register <- client

   	// Allow collection of memory referenced by the caller by doing all work in
   	// new goroutines.
   	go client.writePump()
   	go client.readPump()
   }
```
ServeWs at 82:33 (edit in progress):
```
   // serveWs handles websocket requests from the peer.
   func ServeWs(hub *Hub, w http.ResponseWriter, r *http.Request) *Client {
   	conn, err := upgrader.Upgrade(w, r, nil)
   	if err != nil {
   		log.Println(err)
   		return              // squiggle: missing return value
   	}
   	client := &Client{hub: hub, conn: conn, send: make(chan []byte, 256)}
   	client.hub.register <- client

   	// Allow collection of memory referenced by the caller by doing all work in
   	// new goroutines.
   	go client.writePump()
   	go client.readPump()

   	return cli⟨ent⟩        // typing; autocomplete open
   }
```
Final seen state (83:51): `WritePump` renamed/exported; call site `go client.writePump()` not re-viewed after the rename (likely the remaining client.go error).

### socket/hub.go (81:20-81:30)
```
 1| // Copyright 2013 The Gorilla WebSocket Authors. All rights reserved.
 2| // Use of this source code is governed by a BSD-style
 3| // license that can be found in the LICENSE file.
 4|
 5| package socket
 6|
 7| // Hub maintains the set of active clients and broadcasts messages to the
 8| // clients.
 9| type Hub struct {
10| 	// Registered clients.
11| 	clients map[*Client]bool
12|
13| 	// Inbound messages from the clients.
14| 	broadcast chan []byte
15|
16| 	// Register requests from the clients.
17| 	register chan *Client
18|
19| 	// Unregister requests from clients.
20| 	unregister chan *Client
21| }
22|
23| func NewHub() *Hub {
24| 	return &Hub{
25| 		broadcast:  make(chan []byte),
26| 		register:   make(chan *Client),
27| 		unregister: make(chan *Client),
28| 		clients:    make(map[*Client]bool),
29| 	}
30| }
31|
32| func (h *Hub) Run() {
33| 	for {
34| 		select {
35| 		case client := <-h.register:
36| 			h.clients[client] = true
37| 		case client := <-h.unregister:
38| 			if _, ok := h.clients[client]; ok {
39| 				delete(h.clients, client)
40| 				close(client.send)
41| 			}
42| 		case message := <-h.broadcast:
43| 			for client := range h.clients {
44| 			?? (rest not visible)
```

## Shell & outputs
- VSCodium terminal (77:24 - 79:00+): `lab $ go run .` with no further output (server running in foreground on :8080).
- Browser DevTools console (chat.html):
  - `This page is in Quirks Mode. Page layout may be impacted. For Standards Mode use "<!DOCTYPE html>". Learn More`
  - `GET http://localhost:8080/favicon.ico` `[HTTP/1.1 404 Not Found 0ms]`
  - `const socket = new WebSocket("ws://localhost:8080/ws");` -> `undefined`; the two addEventListener statements -> `undefined` each
  - [79:33] `Message from server  Hello Server!` (chat.html:51:21)
- Network panel tables: see Timeline [78:01] and [79:05].
- Web page read: MDN WebSocket (Constructor, Instance properties, Events, Examples, Specifications "WebSockets # the-websocket-interface"); page is light-themed, text as quoted above.

## Observations
- Workflow: reads MDN WebSocket client example, pastes the snippets into the browser console first to test the Go /ws endpoint (changed URL to `ws://localhost:8080/ws`), checks the 101 row in Network, then ports the same snippets into chat.html as `initSocket()`.
- Server side is gorilla/websocket's chat example almost verbatim (client.go, hub.go in package `socket`, license header kept), wired in main.go via `/ws` route plus a `/web/` static file server with StripPrefix; the earlier `/api/send` JSON handler (MessageReq/MessageRes) is retained.
- End-to-end round trip confirmed at 79:33: client sends "Hello Server!" on open, readPump broadcasts, console logs it.
- Next step in progress: bridge the HTTP handler to the WS client by making ServeWs return `*Client`, storing it in global `var client *socket.Client`, and exporting `WritePump`. Compile errors (red badges) pending at segment end.
- Many reference tabs: net package, GoRealtimeWeb (SimonWaldherr), x/net/websocket, gorilla/mux, gorilla/websocket examples/chat/client.go, "Working with Sockets in Go".
