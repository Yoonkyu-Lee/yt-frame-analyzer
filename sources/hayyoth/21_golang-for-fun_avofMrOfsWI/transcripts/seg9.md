# Segment 9 of 17 (43:22 - 51:11), video avofMrOfsWI "a bit of Golang for fun"

Setup: macOS, VSCodium window titled "<file> — lab" (workspace "lab"), Firefox windows (one on DuckDuckGo/localhost, one on go.dev / StackOverflow), terminal panel at bottom of editor. Webcam "Human" overlay ignored. No editor save messages appear in this segment.

## Timeline

- [43:22] Firefox window `http://localhost:8080/web/chat.html`, page "Chat pages" (h1), text input containing `hayyoth here`, button `Send`. DevTools Network tab open, Response sub-tab: 4 requests (chat.html GET, favicon.ico GET 404, two `send` POST). Selected `send` response JSON: `Status: "ok"`, `Message: "hayyoth here"`. Footer: "4 requests 887 B / 1.45 kB transfe..". Terminal: `lab $ go run .` (server running).
- [43:31] chat.html in editor (title `chat.html — lab`). Visible client code:
  `<h1>Chat pages</h1>`, `<input type="text" id="inpMess" placeholder="Enter Message"><br><br>`, `<button onclick="send()">Send</button>`, `<script>` with `const inpMess = document.getElementById("inpMess");`, `async function send() { try { const data = { id: 1, message2: inpMess.value };` (note `message2` key - deliberate/experimental, so server decodes to empty Message), `const response = await fetch("/api/send", {method: "POST", body: JSON.stringify(data),});`, `if (!response.ok) { throw new Error(\`Response status: ${response.status}\`); }`, `const result = await response.json(); console.log(result);`.
- [43:54] main.go (title `main.go — lab`). In the handler's else-branch, the `response := MessageRes{Status: "ok", Message: messageReq.Message,}` + `json.NewEncoder(w).Encode(response)` block is selected (to be dedented/moved out of the `else`).
- [44:00] `else` removed; the ok-response block now at function top level, dedented. Cursor at end of last `json.NewEncoder(w).Encode(response)`. Error branch still has Message "Not right format".
- [44:25] Error message changed to `"can't decode message"`, `return` added after the Encode in the error branch; starting to type a new `if me` (autocomplete popup: messageReq, MessageReq, MessageRes, metrics, http.MethodConnect...).
- [44:32] Browser: response `Status: "ok"`, `Message: ""` (3 requests, 837 B / 1.29 kB) - consequence of client sending `message2`, server's Message empty. Editor behind has `if messageReq` line typed.
- [44:33] New Firefox URL bar typing `google.com/` (Firefox Suggest dropdown: Google Translate entries, Google Gemini, YouTube Studio).
- [44:36] Search "golang string" results (DuckDuckGo): GeeksforGeeks "Strings in Golang", pkg.go.dev "strings package", golangdoc.github.io "strings - The Go Programming Language", go.dev blog "Strings, bytes, runes and characters in Go", Programiz.
- [44:38-44:40] Opened GeeksforGeeks "Strings in Golang" (Last Updated 10 Mar, 2026); example with `My_value_1 := "Welcome to GeeksforGeeks"`, `var My_value_2 string`, `fmt.Println("String 1: ", My_value_1)`; output shown; note "String can be empty, but they are not nil." then "String Literals". Back to search results.
- [44:44] Editor: `if messageReq.Message` typed (no `== ""` yet), `}` of the err block cursor line. `response :=` underlined red (redeclared).
- [45:05-45:11] Created/used `test.go` (`package main` / `func main() {` / `}`), file explorer (LAB): hello/hello.go, public/chat.html, public/index.html, a.out, go.mod, main.go (red, 3 errors), test.go. Right-click context menu (New File, New Folder, Open with Five Server (root), Reveal in Finder, ... Run Tests, Debug Tests). Dragged test.go into a new folder `test`; dialog "Are you sure you want to move 'test.go' into 'test'?" [Do not ask me again] [Cancel] [Move]; clicked Move. Result tree: hello, public, test/test.go, a.out, go.mod, main.go (3).
- [46:57] Terminal now `lab $ go run main.g...` (command changed from `go run .` to `go run main.go`). Editor shows if-block for empty message with Status/Message...; browser still shows previous response `Status: "ok" Message: ""`.
- [47:06] main.go: `if messageReq.Message == "" {` block complete: `response := MessageRes{Status: "error", Message: "message empty",}`, `json.NewEncoder(w).Encode(response)`, `return`, `}`. Terminal `lab $ go run main.go`.
- [47:12] Browser response now `Status: "error"`, `Message: "message empty"` (3 requests, 853 B / 1.30 kB).
- [47:13-47:14] Switching windows: StackOverflow question 15672556 (Go handler printing req.Form, example with `type test_struct struct { Test string }`, `json.Unmarshal`), then `pkg.go.dev/net/http#Request` ("type Request struct", Method, URL, Proto...).
- [47:16-47:28] go.dev/doc/articles/wiki/ read: "Introducing Function Literals and Closures" - viewHandler/editHandler/saveHandler with `title string`, `makeHandler(fn func (http.ResponseWriter, *http.Request, string)) http.HandlerFunc`, `validPath.FindStringSubmatch(r.URL.Path)`, `fn(w, r, m[2])`; saveHandler with `http.Error(w, err.Error(), http.StatusInternalServerError)` and `http.Redirect(w, r, "/view/"+title, http.StatusFound)`.
- [47:32] Alt-tab to main.go; `Message` highlighted (all occurrences).
- [47:37-47:51] Back to wiki page, highlights `http.StatusInternalServerError`, `Error` in `http.Error(w, err.Error(), http.StatusInternalServerError)`; flips to editor again.
- [48:03-48:12] Wiki page: whole line `http.Error(w, err.Error(), http.StatusInternalServerError)` selected; scrolls to "Introducing Function Literals and Closures". Editor cursor after `return` in the first error branch.
- [48:57] main.go empty-message branch now has an added line between Encode and return: `http.Error(w, err.Error(), http.StatusBadRequest)` (copied/adapted from wiki). Terminal typed `lab $ go run main.go` (cursor at end, not run yet).
- [48:58] Mission-control view (Desktop 3) showing three windows: main.go editor, localhost chat.html Firefox with Network response `Status: "error"`, `Message: "message empty"`, go.dev wiki. `err.Error()` underlined yellow in editor.
- [48:59] Browser with chat page, editor shows `http.Erro..` line.
- [49:11] Terminal output after request: Go panic stack trace (`/opt/homebrew/Cellar/go/1.27.3/libexec/src/net/http/server.go:3436 +0xb0`, `net/http.(*conn).serve(0x3fc8bf72c1b0, {0x104f08278, 0x3fc8bf7a11a0})`, `.../server.go:2157 +0x5a0`, `created by net/http.(*Server).Serve in goroutine 1`, `.../server.go:3604 +0x3e8`). Browser console shows 2 errors: GET favicon.ico 404 and chat.html:25:21 NetworkError when attempting to fetch resource.
- [49:24] Terminal now: `lab $ go run main.go` / `lab $` / `lab $ go run main.go` / `2026/11/05 11:26:17 http: panic serving 127.0.0.1:62725: runtime error: invalid memory address or nil pointer dereference` / `goroutine 20 [running]:` / `net/http.(*conn).serve.func1()`. Hover over `http.StatusBadRequest` shows `const http.StatusBadRequest untyped int = 400`, "HTTP status codes as registered with IANA..." Cause: `err` is nil in that branch so `err.Error()` panics.
- [50:38] Edited line to `http.Error(w, json.NewEncoder(w).Encode(response)⟨cursor⟩ http.StatusBadRequest)` (mid-edit); hover diagnostic: `cannot use json.NewEncoder(w).Encode(response) (value of interface type error) as string value in argument to http.Error compiler(IncompatibleAssign)`. A blank line left where the earlier Encode line was removed.
- [50:45] Browser console tab: warning "This page is in Quirks Mode. Page layout may be impacted. For Standards Mode use "<!DOCTYPE html>". chat.html"; `GET http://localhost:8080/favicon.ico [HTTP/1.1 404 Not Found 0ms]`; error "NetworkError when attempting to fetch resource. chat.html:25:21".
- [50:50] Line is now `http.Error(w, json.NewEncoder(w).Encode(response).Error(), http.StatusBadRequest)` followed by `return` (compiles-ish but still nil-pointer: Encode returns nil error). Terminal panel stacktrace again (go 1.27.3, new addresses `0x1b56e4088000`); drag-resizing the terminal panel taller.
- [50:58] Terminal enlarged: second panic at `2026/11/05 11:28:03 http: panic serving 127.0.0.1:62731: runtime error: invalid memory address or nil pointer dereference`, `goroutine 33 [running]:`, `net/http.(*conn).serve.func1()`, `server.go:1959 +0xac`, `panic({0x10092b270?, 0x1009ac150?})`, `runtime/panic.go:859 +0x120`, `main.handler({0x100967da8, 0x1b56e40be000}, 0x1b56e40aa000)`, `net/http.serverHandler.ServeHTTP({0x10?}, ...)` server.go:2923 +0x194.
- [51:03] Browser Network: chat.html 200 GET, favicon.ico 404, `POST send` blocked/failed (red, NS_E... = NS_ERROR_NET_...); "3 requests 808 B / 1.71 kB transferred Finish: 1.70 s DOMContentLoaded: 20 ms load: 29 ms".
- [51:04] New tab URL bar typing `golang ` (suggest list).
- [51:08] Search "golang return json error": results StackOverflow "go - Can you return json in golang http.Error?" (snippet: "Instead of using http.Error, you can send your own error response with json just like how you would send any other response by setting the status code to an error code."), StackOverflow "How to serve up a JSON response using Go?", go.dev "Return and handle an error", go.dev blog "Error handling and Go".
- [51:11] Opened `go.dev/doc/tutorial/handle-errors` "Return and handle an error" (greetings module, `errors.New("empty name")`).

## File states

### main.go (as of 50:50, lines above/below partially unseen; no line numbers shown in frames, so none invented)
Top of file (package, imports, types MessageReq/MessageRes, main) not visible in this segment except fragments. Known pieces: `type MessageReq`/`MessageRes` with fields Message (req) and Status, Message (res); in main (seen 43:54): `fs := http.FileServer(http.Dir("./public"))` and an `http.Handle("/web/", http.StripPrefix("/web/", fs))` style line (partial: `http.Handle("/web/", http.StripPrefix("/web/", fs))` - only lower half seen, ⟨?⟩).

```go
?? (lines above not visible)
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

		http.Error(w, json.NewEncoder(w).Encode(response).Error(), http.StatusBadRequest)
		return
	}

	response := MessageRes{
		Status:  "ok",
		Message: messageReq.Message,
	}

	json.NewEncoder(w).Encode(response)
}

func main() {
	fs := http.FileServer(http.Dir("./public"))
	?? (rest of main not visible)
```
(Indentation is tabs in the editor; field alignment `Status:  "error"` with two spaces seen.)

Intermediate (48:57): empty-message branch ended with
```go
		json.NewEncoder(w).Encode(response)
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
```
(this panics because err == nil).

### chat.html (public/chat.html, seen 43:31; no line numbers visible)
```html
<h1>Chat pages</h1>
<input type="text" id="inpMess" placeholder="Enter Message"><br><br>
<button onclick="send()">Send</button>


<script>
    const inpMess = document.getElementById("inpMess");
    async function send() {
        try {
            const data = {
                id: 1,
                message2: inpMess.value
            };
            const response = await fetch("/api/send", {
                method: "POST",
                body: JSON.stringify(data),
            });
            if (!response.ok) {
                throw new Error(`Response status: ${response.status}`);
            }

            const result = await response.json();
            console.log(result);
            ?? (remainder, catch block, not visible)
```

### test/test.go (moved from root test.go at 45:11)
```go
package main

func main() {

}
```

## Shell & outputs

- [43:22-47:06] `lab $ go run .` (server running, no output visible).
- [46:57-48:59] `lab $ go run main.go` (server restarted with the edited handler; cursor idle).
- [48:57] typed `lab $ go run main.go` again (cursor right after `go`, highlighted).
- [49:11] output:
```
lab $ go run main.go
        /opt/homebrew/Cellar/go/1.27.3/libexec/src/net/http/server.go:3436 +0xb0
net/http.(*conn).serve(0x3fc8bf72c1b0, {0x104f08278, 0x3fc8bf7a11a0})
        /opt/homebrew/Cellar/go/1.27.3/libexec/src/net/http/server.go:2157 +0x5a0
created by net/http.(*Server).Serve in goroutine 1
        /opt/homebrew/Cellar/go/1.27.3/libexec/src/net/http/server.go:3604 +0x3e8
```
- [49:24]:
```
lab $ go run main.go
lab $ 
lab $ go run main.go
2026/11/05 11:26:17 http: panic serving 127.0.0.1:62725: runtime error: invalid memory address or nil pointer dereference
goroutine 20 [running]:
net/http.(*conn).serve.func1()
```
- [50:58]:
```
        .../server.go:2923 +0x194
net/http.serverHandler.ServeHTTP({0x10?}, {0x100967da8?, 0x1b56e3f16000?}, 0x1?)
        /opt/homebrew/Cellar/go/1.27.3/libexec/src/net/http/server.go:3436 +0xb0
net/http.(*conn).serve(0x1b56e3ede240, {0x100968278, 0x1b56e3ea91a0})
        .../server.go:2157 +0x5a0
created by net/http.(*Server).Serve in goroutine 1
        .../server.go:3604 +0x3e8
2026/11/05 11:28:03 http: panic serving 127.0.0.1:62731: runtime error: invalid memory address or nil pointer dereference
goroutine 33 [running]:
net/http.(*conn).serve.func1()
        /opt/homebrew/Cellar/go/1.27.3/libexec/src/net/http/server.go:1959 +0xac
panic({0x10092b270?, 0x1009ac150?})
        /opt/homebrew/Cellar/go/1.27.3/libexec/src/runtime/panic.go:859 +0x120
main.handler({0x100967da8, 0x1b56e40be000}, 0x1b56e40aa000)
```
Browser outputs: see timeline (Network response JSON `{Status:"ok", Message:"hayyoth here"}`, then `{Status:"ok", Message:""}` after client key renamed to `message2`, then `{Status:"error", Message:"message empty"}`; after the http.Error edits POST fails with NS_ERROR (server panic), console: favicon 404 + NetworkError at chat.html:25:21).

## Observations

- Process: iterative request-validation in a JSON API handler. Client intentionally sends `message2` instead of `message` to provoke the empty-message path; server then replies with JSON error `{Status:"error", Message:"message empty"}`.
- Refactor: flattened `if err != nil {...} else {...}` into guard clauses with early `return` (error message "Not right format" -> "can't decode message").
- Wanted to also send a non-200 HTTP status code on error; consulted the Go wiki article (http.Error with StatusInternalServerError), tried `http.Error(w, err.Error(), http.StatusBadRequest)` -> nil-pointer panic because `err` is nil in that branch; then wrongly tried `json.NewEncoder(w).Encode(response).Error()` (still nil error -> panic, and compile diagnostic `IncompatibleAssign` en route). Then searched "golang return json error" to find the right idiom (StackOverflow "Can you return json in golang http.Error?" suggests setting status code via `w.WriteHeader` + JSON body).
- Looked up Go strings (GeeksforGeeks) while thinking about the empty string check; also checked pkg.go.dev net/http Request docs.
- Housekeeping: organizing scratch `test.go` into a `test/` folder via drag in explorer; switched `go run .` to `go run main.go` because test.go (also package main) is in the same dir or to avoid package conflict (rationale not stated on screen).
- Dev-style: reads docs/StackOverflow in Firefox constantly, uses DevTools Network/Console to verify server responses, uses macOS Mission Control (Desktop 3) to switch windows. Noted page in Quirks Mode (no `<!DOCTYPE html>`) and missing favicon (404) - ignored.
