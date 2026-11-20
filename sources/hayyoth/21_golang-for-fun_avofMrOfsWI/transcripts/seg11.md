# Segment 11 (53:44 - 60:33) — avofMrOfsWI

## Timeline
- [53:44] VSCodium window "main.go — lab", editor on top, TERMINAL below with `lab $ go run main.go` (server running, blue cursor block). Behind: Firefox on a Stack Overflow question (stackoverflow.com/questions/8270816/con...). In main.go, inside `if messageReq.Message == "" {`, the block is commented out:
  ```
  // response := MessageRes{
  //  Status:  "error",
  //  Message: "message empty",
  // }

  http.Error(w, "Not good", http.StatusBadRequest)
  return
  }
  ```
  Above it: `Message: "can't decode message",` / `}` / `json.NewEncoder(w).Encode(response)` / `return` / `}`.
- [53:49] Firefox / Stack Overflow answer (564 votes, green check): json.Marshal example (`package main`, imports "fmt","encoding/json", `type User struct { Name string }`, `user := &User{Name: "Frank"}`, `b, err := json.Marshal(user)`, `fmt.Println(string(b))`; Output `{"Name":"Frank"}`; "edited Apr 23, 2014 at 15:49"). `Marshal` highlighted. A Google sign-in popup (Vietnamese text) overlays the page.
- [53:55] Back in editor: comment slashes removed; `response := MessageRes{ Status:  "error", Message: "message empty", }` (red squiggle under `response`, unused). Cursor on blank line below `}`.
- [54:50-54:51] Editor now has `responseBy...` / `if err !=` lines (see 54:59); Firefox shows search "golang json to string" (results: Stack Overflow "http - Converting Json to string in golang", Soham Kamani "A Complete Guide to JSON in Golang (With Examples)", pkg.go.dev "json package - encoding/json - Go Packages", go.dev "Tutorial: Working with JSON - The Go Programming Language").
- [54:52] Firefox on Dan Gillis blog post (dangillis.dev/posts/archive/http-json-error-resp...): code with `errJSON, _ := json.MarshalIndent(er, "", "    ")` (MarshalIndent highlighted), `http.Error(w, string(errJSON), e.Status())`, `default:` case with `cd := http.StatusInternalServerError`, `er := errResponse{Error: svcError{Type: "unknown_error", Message: "Unexpected error - contact support"}}`.
- [54:53] Firefox http://localhost:8080/web/chat.html: page "Chat pages", input "hayyoth here", Send button; DevTools Network: chat.html (200 GET), favicon.ico (404), `send` (POST, 400) Response payload `Not good`. 3 errors.
- [54:59] Editor code now:
  ```
  responseByte, err := json.Marshal(response)
  if err != nil {

  }

  http.Error(w, string(responseByte), http.StatusBadRequest)
  return
  ```
  Terminal still `lab $ go run main.go`.
- [55:01-55:11] Firefox still shows `Not good`; editor cursor moves to `if err != nil {`. (Title greyed = window unfocused.)
- [55:25] Firefox Network now shows JSON response for `send`: `Status: "error"`, `Message: "message empty"` (capitalised keys). Editor behind shows top of file with imports `encoding/json`, `log`, `net/http`, type definitions.
- [55:26-55:50] Top of main.go: gopls hover on MessageRes `Message` field: "struct field tag `json:message` not compatible with reflect.StructTag.Get: bad syntax for struct tag value structtag(default)"; also "field Message string `json:message` // size=16 (0x10), offset=16 (0x10)", "(main.MessageRes).Message on pkg.go.dev", "View Problem (⌥F8) No quick fixes available". Shows MessageReq tags `json:id` / `json:message` (no inner quotes; yellow squiggles under Id and Message), MessageRes tags `json:status` (partly visible).
- [55:50] Firefox Network cleared: 2 requests (chat.html 200, favicon.ico 404), 1 error.
- [55:56] Editor: MessageReq tags still `json:id`, `json:message`; MessageRes tags `json:"status"`, `json:"message"` (correct).
- [56:03] Firefox after reload: Network 2 requests again.
- [56:09] After editing: MessageReq now `Id int \`json:"id"\``, `Message string \`json:"message"\`` (fixed). Cursor on `handler`.
- [56:18] Firefox Network: `send` response JSON now lowercase keys `status: "error"`, `message: "message empty"`.
- [56:19] Editor with explorer sidebar open (LAB: hello/hello.go, public/chat.html, public/index.html, test/test.go, a.out, go.mod, main.go selected). Handler shows the error block with `// do something in here` inside `if err != nil {`.
- [56:27] Firefox Console: Quirks Mode warning, `GET http://localhost:8080/favicon.ico [HTTP/1.1 404 Not Found 0ms]`, `XHR POST http://localhost:8080/api/send [HTTP/1.1 400 Bad Request 0ms]`, `Response status: 400`. Editor behind shows chat.html JS.
- [56:29] Editor tab "chat.html — lab": `message2: inpMess.value` in data (typo/deliberate to trigger empty-message error), `throw new Error(\`Response status: ${response.status}\`);` inside `if (!response.ok) {`.
- [56:41-56:44] Firefox: Send click shows alert "localhost:8080 / undefined" (from `alert(response.message)`; response is a Response object). Editor then: `//throw new Error(...)`, `alert(response.message);`, `return;`.
- [57:13] Console shows `Object { status: "error", message: "message empty" }` at chat.html:21:25.
- [57:18] Editor: inside the if: `//throw new Error(...)`, `const result = await response.json();`, `console.log(result)`, `//alert(response.message);`, `return;`.
- [57:34] Console expanded Object `{ status: "error", message: "message empty" }` (message, status, `<prototype>: Object { … }`).
- [57:36] alert dialog "localhost:8080 / message empty".
- [57:39] Editor: if block `const result = await response.json(); alert(result.message) return;`.
- [57:43] Console after reload: only favicon 404 (1 error).
- [57:49] Editor: `message: inpMess.value` (renamed from message2); `alert(result.message)`.
- [58:09] Console `Object { status: "ok", message: "hayyoth here" }` chat.html:25:21 (success path).
- [58:11] alert "localhost:8080 / Send" (from `alert("Send ", message)`, alert only takes first arg).
- [58:14] Editor: after the ok-block: `const result = await response.json(); const message = result.message;` blank, `alert("Send ", message)`.
- [58:31] Console cleared (no errors); editor shows `//cons...` commented line.
- [58:37] Editor: `//const message = result.message;` (commented), `console.log(result)`.
- [58:44] Console prints `hayyoth here` (chat.html:27:21).
- [58:46] Editor: `const message = result.message;` restored; `console.log(message)`.
- [58:54-59:04] Alert "Send hayyoth here", then with input "youtube": "Send youtube".
- [60:16] Editor: `alert("Send " + message)` (fixed to concatenation; earlier comma form).
- [60:20] Editor top of chat.html shows added `<div id="messages">` with two hardcoded `.message` divs ("Hello", "How about") and `const messagesElm = document.getElementById("messages");`.
- [60:23-60:33] Firefox renders "Chat pages", "Hello", "How about", input "youtube", Send. Author (cigarette on webcam, ignore) discusses; cursor hovers over the message divs. No save messages visible.

## File states

### main.go (as of ~56:19; editor line numbers not shown, gutter hidden)
```
?? package main
??
?? import (
??     "encoding/json"
??     "log"
??     "net/http"
?? )
??
?? // message from user send
?? type MessageReq struct {
??     Id      int    `json:"id"`
??     Message string `json:"message"`
?? }
??
?? // message return for user
?? type MessageRes struct {
??     Status  string `json:"status"`
??     Message string `json:"message"`
?? }
??
?? func handler(w http.ResponseWriter, r *http.Request) {
??     w.Header().Set("Content-Type", "application/json")
??
??     decoder := json.NewDecoder(r.Body)
??     var messageReq MessageReq
??     ?? (decode lines not visible)
??         response := MessageRes{
??             Status:  "error",
??             Message: "can't decode message",
??         }
??         json.NewEncoder(w).Encode(response)
??         return
??     }
??     if messageReq.Message == "" {
??         response := MessageRes{
??             Status:  "error",
??             Message: "message empty",
??         }
??         responseByte, err := json.Marshal(response)
??         if err != nil {
??             // do something in here
??         }
??
??         http.Error(w, string(responseByte), http.StatusBadRequest)
??         return
??     }
??
??     response := MessageRes{
??         Status:  "ok",
??         Message: messageReq.Message,
??     }
??
??     json.NewEncoder(w).Encode(response)
?? }
??
?? func main() {
??     fs := http.FileServer(http.Dir("./public"))
??     ?? (rest not visible)
```
Project tree (explorer "LAB"): hello/hello.go; public/chat.html, public/index.html; test/test.go; a.out; go.mod; main.go.

### chat.html (as of 60:33; line numbers not shown)
```
?| <h1>Chat pages</h1>
?| <div id="messages">
?|     <div class="message">Hello</div>
?|     <div class="message">How about</div>
?| </div>
?| <input type="text" id="inpMess" placeholder="Enter Message"><br><br>
?| <button onclick="send()">Send</button>
?|
?|
?| <script>
?|     const inpMess = document.getElementById("inpMess");
?|     const messagesElm = document.getElementById("messages");
?|
?|     async function send() {
?|         try {
?|             const data = {
?|                 id: 1,
?|                 message: inpMess.value
?|             };
?|             const response = await fetch("/api/send", {
?|                 method: "POST",
?|                 body: JSON.stringify(data),
?|             });
?|             if (!response.ok) {
?|                 const result = await response.json();
?|                 alert(result.message)
?|                 return;
?|             }
?|
?|             const result = await response.json();
?|             ?? (lines after not visible at 60:33; as of 60:16:)
?|             const message = result.message;
?|
?|             alert("Send " + message)
?|         } catch (error) {
?|             console.error(error.message);
?|         }
?|     }
?|
?|
?| </script>
```

## Shell & outputs
- [53:44 onward] Terminal: `lab $ go run main.go` — no output; server running throughout (no restart visible).
- Browser (Firefox DevTools): `GET http://localhost:8080/favicon.ico [HTTP/1.1 404 Not Found 0ms]`; `XHR POST http://localhost:8080/api/send [HTTP/1.1 400 Bad Request 0ms]`; "Response status: 400"; "This page is in Quirks Mode. Page layout may be impacted. For Standards Mode use “<!DOCTYPE html>”."; `Object { status: "error", message: "message empty" }`; `Object { status: "ok", message: "hayyoth here" }`; alerts "undefined", "message empty", "Send", "Send hayyoth here", "Send youtube".

## Observations
- Lock-step iteration of Go backend and chat.html frontend, verifying each change in Firefox Network/Console.
- Moves from plain-text `http.Error(w, "Not good", 400)` to JSON-encoded error body via json.Marshal + http.Error; researched via Stack Overflow, a "golang json to string" search, and Dan Gillis' HTTP JSON error response post.
- Debugged struct-tag bug (`json:message` without inner quotes) flagged by gopls; response keys flipped from Status/Message to status/message after fix.
- Client bug: alerted `response.message` (undefined) because the body was not read; fixed with `const result = await response.json(); alert(result.message)`.
- Temporarily used `message2` in the request body to trigger the empty-message error path, then restored `message`.
- Began building a message-list UI: hardcoded `#messages` div with two `.message` children and `messagesElm` reference, presumably for appending messages next.
