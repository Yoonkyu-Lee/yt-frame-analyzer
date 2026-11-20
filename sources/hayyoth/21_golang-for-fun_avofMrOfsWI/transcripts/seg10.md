# Segment 10 (51:11 - 53:44) - avofMrOfsWI

## Timeline
- [51:11] Firefox window (go.dev/doc/tutorial/handle-errors, "Return and handle an error", still loading: "Waiting for go.dev...") in front of the VSCodium window (main.go - lab) and a second Firefox window behind (the net/http wiki article). VSCodium behind shows the tail of a handler in main.go and a terminal with a Go panic trace (see Shell). Webcam overlay "Human" ignored.
- [51:12-51:19] Scrolls the handle-errors tutorial: greetings.go sample (`Hello(name string) (string, error)` returning `errors.New("empty name")`, `fmt.Sprintf("Hi, %v. Welcome!", name)`), "In this code, you:" bullet list.
- [51:15] Firefox shows a DuckDuckGo results page for query `golang return json error` (two Stack Overflow questions, go.dev handle-errors, go.dev blog error-handling-and-go). [51:16] opens Stack Overflow "Can you return json in golang http.Error?" (asked 6 years 9 months ago, viewed 17k times).
- [51:16-51:21] Reads the SO question: `myObj := MyObj{MyVar: myVar}`, `data, err := json.Marshal(myObj)`, `http.Error(w, err.Error(), http.StatusInternalServerError)`, `w.Write(data)`, `w.Header().Set("Content-Type", "application/json")`, `http.Error(w, "some error happened", http.StatusIn...`. Text: "I see that it returns 200 with no json but the json is embed in tex...". A Google sign-in popup (Vietnamese text) overlays. Top answer (score 22) quotes stdlib http.Error:
  ```
  func Error(w ResponseWriter, error string, code int) {
      w.Header().Set("Content-Type", "text/plain; charset=utf-8")
      w.Header().Set("X-Content-Type-Options", "nosniff")
      w.WriteHeader(code)
      fmt.Fprintln(w, error)
  }
  ```
  and a custom `func JSONError(w http.ResponseWriter, err interface{}, cod...` setting "application/json; char...", nosniff, `w.WriteHeader(code)`, `json.NewEncoder(w).Encode(err)`.
- [51:23] Another answer: "Also I think your usage of http.Er... the response is sent and response body will be closed. That is why you are getting 200 status instead of 500 from http.Error. Instead of using http.Error, you can send your own error response with json just like how you would send any other response by setting the status code to an error code." Next answer has `type Response struct { StatusCode int; Msg string }`.
- [51:30-51:36] Back to search results; scrolls (pjebs/jsonerror GitHub, LabEx "How to resolve JSON parsing errors", pkg.go.dev errors, dangillis.dev "HTTP JSON Error Responses in Go", Sling Academy "Handling JSON Requests and Responses in Go").
- [51:38-52:13] Opens the dangillis.dev blog article "HTTP JSON Error Responses in Go". Scrolls: first JSON sample `{"error": {"type": "validation_failed", ...}}`; dispatch package using gorilla/mux and `eh.ErrHandler{Env: env, H: handler.CreateUser}`; errorHandler package: `type Error interface { error; Status() int }`, `type HTTPErr struct { Code int; Type string; Err error }` with methods Error(), SetErr, ErrType, Status; the ServeHTTP type switch `case Error:` with `log.Printf("HTTP %d - %s", e.Status(), e)`, `er := errResponse{Error: svcError{Type: e.ErrType(), Message: e.Error()}}`, `errJSON, _ := json.MarshalIndent(er, "", "    ")` (MarshalIndent highlighted), `http.Error(w, string(errJSON), e.Status())`, and `default:` returning 500 with `Type: "unknown_error"`, `Message: "Unexpected error - contact support"`.
- [52:05] Switches to VSCodium main.go. Line reads:
  `http.Error(w, json.NewEncoder(w).Encode(response).Error(), http.StatusBadRequest)` followed by `return` (cursor on the return line, `return` keywords highlighted). The block above is `if messageReq.Message == "" { response := MessageRes{ Status: "error", Message: "message empty", } ...`.
- [52:06] Editing that line: now `http.Error(w, , http.StatusBadRequest)` (second argument deleted, red squiggle after `w, `). Back to the blog at 52:11-52:13 (same MarshalIndent snippet), main.go at 52:14 (still `http.Error(w, , http.StatusBadRequest)`).
- [52:23] main.go modified: the `response := MessageRes{...}` block in the `if messageReq.Message == ""` branch is commented out (each line prefixed `// `), and the call is now `http.Error(w, "Not good", http.StatusBadRequest)`. Terminal reset: `lab $ go run main.go` with a hollow-circle marker (server restarted, no output).
- [52:24-52:28] Window switching animation shows the net/http wiki article (go.dev/doc/articles/wiki/, "Introducing Function Literals and Closures"): "Catching the error condition in each handler introduces a lot of repeated code. What if we could wrap each of the handlers in a function that does this validation and error checking? Go's function literals provide a powerful means of abstracting functionality that can help us here." Code: `func viewHandler(w http.ResponseWriter, r *http.Request, title string)` (also edit/save), `func makeHandler(fn func (http.ResponseWriter, *http.Request, string)) http.HandlerFunc { return func(w http.ResponseWriter, r *http.Request) { // Here we will extract the page title from the Request, // and call the provided handler 'fn' } }`; "The returned function is called a closure because it encloses values defined outside of it. In this case, the variable fn (the single argument to makeHandler) is enclosed by the closure..."; next code `func makeHandler(fn func(http.ResponseWriter, *http.Request...` partly visible.
- [52:27] Mid-switch frame shows pkg.go.dev/net/http#Request ("type Request struct { // Method specifies the HTTP method (GET, POST, PUT, etc.). ... Method string; URL *url.URL; Proto string // "HTTP/1.0"; ProtoMajor int // 1 }") with the dock visible.
- [52:29-52:30] Back on the dangillis.dev tab; clicks the "Search All Tabs" button: current window tabs listed: `localhost:80?0/web/chat.html` and "HTTP JSON Error Responses in Go".
- [52:38] Chat tab: localhost:8080/web/chat.html: h1 "Chat pages", text input with `hayyoth here`, button "Send". DevTools open (Inspector, Console, Debugger, Network, Storage); Network shows 1 request GET chat.html (document).
- [52:41] main.go in front again (same state). DevTools badge shows 3 errors.
- [52:42] After Send: Network lists 3 requests: chat.html (2xx), favicon.ico (4xx), send (POST, 4xx). Selected `send` -> Response tab, Response Payload: line 1 `Not good`, line 2 blank. Footer "3 requests, 817 B / 1.32 kB transferre...".
- [52:46] Types `golang` in the Firefox URL bar (Firefox Suggest: X, DuckDuckGo, YouTube "Is Golang Dying? 2026 Outlook", Google Translate history items).
- [52:50] DuckDuckGo results for `golang json to string`: SO "Converting Json to string in golang", "A Complete Guide to JSON in Golang (With Examples)", pkg.go.dev encoding/json, go.dev "Tutorial: Working with JSON".
- [52:50-53:04] Opens pkg.go.dev/encoding/json (Version go1.27.3 Latest, Published Oct 20, 2026, BSD-3-Clause, Imports: 15, Imported by: 1,523,261). Scrolls Documentation overview: Security Considerations, "Migrating to v2" (v1 now formally the v1 package; v2 at encoding/json/v2; list of v1/v2 differences e.g. nil slice/map marshals as null in v1, empty array/object in v2; byte arrays; MarshalJSON on pointer receivers), "All new usages of "json" in Go should use the v2 package, but the v1 package will forever remain supported." Index lists `func Compact`, `func HTMLEscape`, `func Indent(dst *bytes.Buffer, src []byte, prefix, indent string) error`, `func Marshal(v any) ([]byte, error)`, `func MarshalIndent(v any, prefix, indent string) ([]byte, error)`, `func Unmarshal(data []byte, v any) error`, `func Valid(data []byte) bool`, `type Decoder`, `func NewDecoder(r io.Reader) *Decoder`. Expands "Example (CustomMarshalJSON)" (zoo census: `blob := ["gopher","armadillo","zebra","unknown","gopher","bee","gopher","zebra"]`, `json.Unmarshal([]byte(blob), &zoo)`, `census := make(map[Animal]int)`, prints "Zoo Census: * Gophers: 3 * Zebras: 2 * Unknown: 3").
- [53:13] Back on the `golang json to string` results.
- [53:15] URL bar shows `gemini.google.com/` autocompleted ("Google Gemini"); page blank.
- [53:21] New search `golang struct to json string`: SO "Converting Go struct to JSON", SO "Encoding struct to json Go", Delft Stack "How to Convert Struct to String in Golang", INFO Tutorials "Go Structs and JSON".
- [53:23] Opens SO "Converting Go struct to JSON" (asked 14 years 11 months ago, score 337, banner: already has an answer "json.Marshal(struct) returns "{}"", closed 2 years ago). Question code: `type User struct { name string }`, `user := &User{name:"Frank"}`, `json.Marshal(user)` gives `{}`.
- [53:37] main.go shown: cursor on the `// }` line area; code unchanged (commented block, `http.Error(w, "Not good", http.StatusBadRequest)`).
- [53:42] SO accepted answer (score 564): capitalised field `Name string`, `user := &User{Name: "Frank"}`, `b, err := json.Marshal(user)` (Marshal highlighted), `fmt.Println(string(b))`, Output `{"Name":"Frank"}`.
- [53:44] main.go: an extra blank line inserted between the commented `// }` and `http.Error(...)` (two blank lines now; the http.Error line moved down one row). No new code typed. Terminal unchanged.

No editor save messages ("written") visible in this segment.

## File states
Only a window of main.go is visible; the gutter/line numbers are hidden in the frames, so no editor line numbers are available. Visible region at the end of the segment (53:44):

```
?? (earlier lines not visible; top cropped line looks like `Status:  "error",`)
        Message: "can't decode message",
    }
    json.NewEncoder(w).Encode(response)
    return
}
if messageReq.Message == "" {
    // response := MessageRes{
    //  Status:  "error",
    //  Message: "message empty",
    // }

    <blank line>
    http.Error(w, "Not good", http.StatusBadRequest)
    return
}

response := MessageRes{
    Status:  "ok",
    ?? (remainder not visible)
```
(Indentation appears as tabs ~4 columns wide; the `if` is one level inside the handler body.)

Intermediate version at 52:05:
```
if messageReq.Message == "" {
    response := MessageRes{
        Status:  "error",
        Message: "message empty",
    }

    http.Error(w, json.NewEncoder(w).Encode(response).Error(), http.StatusBadRequest)
    return
}
```
At 52:06-52:14 the call line is `http.Error(w, , http.StatusBadRequest)` (second arg deleted, red squiggle).

chat.html (localhost:8080/web/chat.html): source not visible; renders h1 "Chat pages", a text input (value "hayyoth here"), a "Send" button.

## Shell & outputs
- [51:11-52:14] Terminal (VSCodium integrated terminal) shows a Go panic trace from a prior run:
```
lab $ go run main.go
net/http.serverHandler.ServeHTTP({0x10?}, {0x100967da8?, 0x1b56e3f16000?}, 0x1?)
        /opt/homebrew/Cellar/go/1.27.3/libexec/src/net/http/server.go:3436 +0xb0
net/http.(*conn).serve(0x1b56e3ede240, {0x100968278, 0x1b56e3ea91a0})
        /opt/homebrew/Cellar/go/1.27.3/libexec/src/net/http/server.go:2157 +0x5a0
created by net/http.(*Server).Serve in goroutine 1
        /opt/homebrew/Cellar/go/1.27.3/libexec/src/net/http/server.go:3604 +0x3e8
2026/11/05 11:28:03 http: panic serving 127.0.0.1:62731: runtime error: invalid memory address or nil pointer dereference
goroutine 33 [running]:
net/http.(*conn).serve.func1()
        /opt/homebrew/Cellar/go/1.27.3/libexec/src/net/http/server.go:1959 +0xac
panic({0x10092b270?, 0x1009ac150?})
        /opt/homebrew/Cellar/go/1.27.3/libexec/src/runtime/panic.go:859 +0x120
main.handler({0x100967da8, 0x1b56e40be000}, 0x1b56e40aa000)
```
(The trace is from the earlier nil-pointer panic; likely caused by `.Error()` on the nil error returned by Encode - inference, not visible.)
- [52:23] Terminal cleared and server restarted: `lab $ go run main.go` with a hollow-circle marker, then cursor; no output. Same through 53:44.
- Browser Network panel [52:42]: GET chat.html, GET favicon.ico (4xx), POST send (4xx), response payload `Not good`.

## Observations
- Debug flow: handler writes JSON errors with `json.NewEncoder(w).Encode(response)` then `return`; he wanted an HTTP 400 on an empty message. He tried `http.Error(w, json.NewEncoder(w).Encode(response).Error(), http.StatusBadRequest)`, which hit a nil-pointer panic. He then searched "golang return json error", learned from SO that http.Error writes plain text/sets Content-Type text/plain and that JSON errors need a custom writer, and replaced the call with `http.Error(w, "Not good", http.StatusBadRequest)` as a test (the browser shows plain text `Not good`), commenting out the MessageRes block.
- References consulted: go.dev handle-errors tutorial, Stack Overflow (twice), a blog post on JSON error responses (MarshalIndent + status from an error type in ServeHTTP), the go.dev wiki article's function literal/closure (makeHandler) wrapper for repeated error handling, pkg.go.dev net/http Request, pkg.go.dev encoding/json (v1/v2 note), and he typed gemini.google.com.
- Likely next step (inferred from searches "golang json to string" / "golang struct to json string"): marshal a MessageRes struct to a JSON string and pass it as the http.Error message so the status is 400 with a JSON body. SO answer reinforced exported (capitalised) struct fields.
- Style: tab-indented ad hoc handler in main.go in a project folder "lab"; manual testing through a minimal chat.html page on localhost:8080 with Firefox DevTools Network tab; prefers search engine + docs.
