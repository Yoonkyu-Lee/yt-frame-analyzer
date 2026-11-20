# Segment 6 (27:47 - 32:33), video avofMrOfsWI

Editor: VSCodium-like window titled "<file> — lab" (workspace folder "lab"). No editor line numbers are visible (gutter hidden), so File states use my own relative numbering `n|` (NOT editor numbers). No save messages visible in this segment. Windows: editor (left), Firefox localhost window (middle, devtools open at bottom), second Firefox window (right: alexedwards.net, go.dev, MDN). Webcam overlay ignored. macOS-style window chrome; a macOS Mission Control view (Desktop 1/2/3) appears briefly at 29:51.

## Timeline
- [27:47] chat.html in editor (title "chat.html — lab"). Terminal shows `lab $ go run .` (server running, no further output). Page content is the pre-existing "alert" version (File states v1). Browser behind at http://localhost:8080/web/chat.html. Devtools badge shows 1 error.
- [27:48] Browser page: h1 "Chat pages", input containing `hello`, `Send` button. Devtools Network tab: `200 GET localhost:8... chat.html document html 433`, `404 GET localhost:8... favicon.ico img plain 176`. Footer: "2 requests  267 B / 609 B transferred  Finish: 36 ms  DOMContentLoaded: 18 ms  load: 27 ms". "Disable Cache" checked.
- [27:49] Typing in Firefox URL bar `js fe` (DuckDuckGo default engine); Firefox Suggest dropdown shown (history entries; personal items not transcribed).
- [27:52] DuckDuckGo results for `js fetch mdn`: top hit "Using the Fetch API - Web APIs | MDN - MDN Web Docs" (developer.mozilla.org/en-US/docs/Web/API/Fetch_API/Using_Fetch, Aug 20 2025), then "Fetch API - Web APIs | MDN" (Jun 8 2026), W3Schools "JavaScript Fetch API", "JavaScript Asynchronous Fetch".
- [27:53] MDN "Using the Fetch API" opened. Sidebar: Making a request / Canceling a request / Handling the response / See also. Text: "Fetch is the modern replacement for XMLHttpRequest: unlike XMLHttpRequest, which uses callbacks, Fetch is promise-based and is integrated with features of the modern web such as service workers and Cross-Origin Resource Sharing (CORS). With the Fetch API, you make a request by calling fetch(), which is available as a global function in both window and worker contexts..."
- [27:55] Scrolled to MDN example `getData` (verbatim in Shell & outputs).
- [28:07] Back to editor: chat.html still v1 (alert version), cursor at end of `alert(inpMess.value);`.
- [28:51] MDN snippet fully highlighted in grey (selected/copied). Editor behind now shows the script replaced: `<script>` / `const inpMess ...` / `async function ...` / `try {` — i.e. the MDN snippet was pasted over `function send(){alert(...)}` and indented one level (script body now indented).
- [28:53] Browser still shows old page (`hello`, Send), Network unchanged (2 requests).
- [28:59] Editor chat.html v2 (pasted + adapted): `const response = await fetch("/api/send");`, error template string on one line, then `const result = await response.json();` etc. Devtools badge now 3 errors (a send request had happened).
- [29:00] Network: 3 requests: chat.html (200), favicon.ico (404), `send` (404, selected, method column `G`). Request tab: "No payload for this request." "3 requests 627 B / 1.13 kB transferred".
- [29:06] Editor cursor over `response.json()` line.
- [29:10-29:11] Network Headers tab for `send`: `GET http://localhost:8080/api/send`, Status `404 Not Found`, Version HTTP/1.1, Transferred `176 B (19 B size)`, Referrer Policy `strict-origin-when-cross-origin`, Request Priority `Highest`.
- [29:14-29:16] New tab URL `js ` typed, then results `js fetch mdn` again (also GeeksforGeeks "Fetch API in JavaScript"), MDN article top again.
- [29:22] MDN getData snippet re-read.
- [29:26] Editor: cursor on blank line under the `if` block; mouse near `"/api/send"`; lightbulb icon in gutter.
- [29:27] MDN section "Setting the method": "By default, fetch() makes a GET request, but you can use the method option to use a different request method:" with `const response = await fetch("https://example.org/post", {` / `method: "POST",` / `// …` / `});`; `method: "POST",` highlighted. "If the mode option is set to no-cors, then method must be one of GET, POST or HEAD." Next heading "Setting a body". Editor behind already shows fetch call expanded (`me...` line then `});`).
- [29:27 (browser)] Browser reloaded: Network shows only `GET chat.html` pending: "1 request, 0 B / 0 B transferred, Finish: 0 ms, DOMContentLoaded: 19 ms, load: 28 ms". Error badge cleared.
- [29:47] Editor v3: `const response = await fetch("/api/send", {` / `method: "POST",` / `});`. Devtools error badge 3 again.
- [29:50] Cursor at end of `fetch("/api/send", {` line.
- [29:51] Mission Control (Desktop 3) overview: editor, localhost Firefox window (Network: chat.html 200, favicon.ico 404, `send` row method `P...`, Request tab "No payload for this request", "3 requests 676 B / 1.18 kB transferred"), and a Firefox window on www.alexedwards.net/blog/serving-static-sites-... showing a Go main.go snippet.
- [29:52] Localhost window: POST `send` row (status 404 still, "No payload for this request"); mouse on Firefox toolbar.
- [29:53] alexedwards.net text: "Notice that because our static directory is set as the root of the file system, we need to strip off the /static/ prefix from the request path before searching the file system for the given file. We do this using the http.StripPrefix() function. If you restart the application, you should find the CSS file we made earlier at localhost:3000/static/stylesheets/main.css. Now let's create a templates directory, containing a layout.html file with shared markup, and an example.html file with some page-specific content." Code (first two statements of main highlighted grey) in Shell & outputs.
- [29:55] Same window now at go.dev/doc/articles/wiki/ ("Writing Web Applications"): renderTemplate / viewHandler / editHandler, "Handling non-existent pages".
- [29:56] URL bar in right window: typing `js fe` (suggestions incl. previous `js fetch mdn at DuckDuckGo`).
- [29:59] Results `js fetch mdn`, hovering first hit ("Read developer.mozilla.org" status text). [30:00] MDN article: "Here's a minimal function that uses fetch() to retrieve some JSON data from a server:".
- [30:08] Editor chat.html v3. Behind it MDN "Setting a body" snippet: `await fetch("https://example.org/post", {` ... `...gify({ username: "example" }),`.
- [30:51] Localhost window (reloaded): chat.html GET 200 `950 B` / `765 B`, favicon 404; "2 requests 784 B / 1.13 kB transferred Finish: 30 ms DOMContentLoaded: 17 ms load: 26 ms". Editor behind (scrolled to top) shows the file extended with a `const` line and `body` line inside fetch options.
- [30:57] Editor chat.html v4: added `const data = { message: inpMess.value };` and `body: JSON.stringify(data),` in the fetch options. Cursor at start of the `body:` line. Error badge 3.
- [31:02] Network, `send` POST request, Request tab: `JSON` section `message: "hello"` (status still 404). "3 requests 803 B / 1.30 kB transferred".
- [31:09] Editor with Explorer sidebar open ("LAB"): `hello/` -> `hello.go`; `public/` -> `chat.html` (selected), `index.html`; `a.out`, `go.mod`, `main.go`. File v5: `id: 1,` added to `data`. Tooltip "72 x 7" (terminal resize) visible. Cursor at `async`; `await` occurrences highlighted.
- [31:47] Network payload `JSON: id: 1, message: "hello"`; "3 requests 826 B / 1.33 kB transferred". Editor behind now shows main.go (imports fmt/log/net/http, handler, main with `fs := http.Fil...`, `http.Handle("/...`, `http.HandleFu...`, `log.Fatal(http...`).
- [32:02] Network: `send` row first cell now `20..` (green), method P...; "3 requests 812 B / 1.27 kB transferred". Right window MDN "Setting a body": "To set a request body, pass it as the body option:" with `body: JSON.stringify({ username: "example" }),` highlighted; "You can supply the body as an instance of any of the following types: a string, ArrayBuffer, TypedArray, DataView, Blob, File, URLSearchParams..."
- [32:05] Editor main.go — lab (v1 below). Cursor on blank line between handler and main.
- [32:10] main.go edited: comment `// now , return json` inserted as first line in handler above `fmt.Fprintf(w, "Ok ok")`. Mouse near Firefox toolbar.
- [32:17-32:33] Right window switched to go.dev/doc/articles/wiki/ and scrolled: renderTemplate/viewHandler/editHandler, "Handling non-existent pages", http.Redirect, "Saving Pages", "Error handling", "Template caching", "Validation", "Introducing Function Literals and Closures", simplified handlers, "Try it out!", "Other tasks". At [32:26] the URL bar is focused/empty ("Search with DuckDuckGo or enter address"), at [32:27] page back on the wiki. No editor changes visible 32:10-32:33.

## File states

### chat.html — v1 at 27:47 (start of segment)
```
1| <h1>Chat pages</h1>
2| <input type="text" id="inpMess" placeholder="Enter Message"><br><br>
3| <button onclick="send()">Send</button>
4| 
5| 
6| <script>
7| const inpMess = document.getElementById("inpMess");
8| function send() {
9|     alert(inpMess.value);
10| }
11| 
12| 
13| </script>
```

### chat.html — v2 at 28:59 (after MDN paste)
```
?? (lines 1-5 as above)
6| <script>
7|     const inpMess = document.getElementById("inpMess");
8|     async function send() {
9|         try {
10|             const response = await fetch("/api/send");
11|             if (!response.ok) {
12|                 throw new Error(`Response status: ${response.status}`);
13|             }
14| 
15|             const result = await response.json();
16|             console.log(result);
17|         } catch (error) {
18|             console.error(error.message);
19|         }
20|     }
21| 
22| 
23| </script>
```

### chat.html — final state at 31:09 (v5)
```
1| <h1>Chat pages</h1>
2| <input type="text" id="inpMess" placeholder="Enter Message"><br><br>
3| <button onclick="send()">Send</button>
4| 
5| 
6| <script>
7|     const inpMess = document.getElementById("inpMess");
8|     async function send() {
9|         try {
10|             const data = {
11|                 id: 1,
12|                 message: inpMess.value
13|             };
14|             const response = await fetch("/api/send", {
15|                 method: "POST",
16|                 body: JSON.stringify(data),
17|             });
18|             if (!response.ok) {
19|                 throw new Error(`Response status: ${response.status}`);
20|             }
21| 
22|             const result = await response.json();
23|             console.log(result);
24|         } catch (error) {
25|             console.error(error.message);
26|         }
27|     }
28| 
29| 
30| </script>
```
(v3 at 29:47-30:08: no `data` const; fetch options had only `method: "POST",`. v4 at 30:57: `data` had only `message: inpMess.value` (no trailing comma, no id).)

### main.go — at 32:10 (end of segment; package line and anything above `import` not visible)
```
?? (package line not visible)
1| import (
2|     "fmt"
3|     "log"
4|     "net/http"
5| )
6| 
7| func handler(w http.ResponseWriter, r *http.Request) {
8|     // now , return json
9|     fmt.Fprintf(w, "Ok ok")
10| }
11| 
12| func main() {
13|     fs := http.FileServer(http.Dir("./public"))
14|     http.Handle("/web/", http.StripPrefix("/web/", fs))
15| 
16|     http.HandleFunc("/api/send", handler)
17| 
18|     log.Fatal(http.ListenAndServe(":8080", nil))
19| }
```
At 32:05 (before the comment) identical minus line 8, with a blank (cursor) line between `}` and `func main() {`.

Workspace tree (31:09): lab/ { hello/hello.go, public/{chat.html, index.html}, a.out, go.mod, main.go }.

## Shell & outputs
- Terminal (whole segment, unchanged): `lab $ go run .` then cursor (server running; no log lines).
- Network rows: [27:48] `200 GET localhost:8... chat.html document html 433`; `404 GET localhost:8... favicon.ico img plain 176`. [29:00-29:11] third row `send`: GET, 404 Not Found, `176 B (19 B size)`, `GET http://localhost:8080/api/send`. [29:51-29:52] same row now POST, 404. [31:02] POST body JSON `message: "hello"`. [31:47] JSON `id: 1`, `message: "hello"`. [32:02] POST `send` status 20x (green).
- MDN getData sample (27:55, verbatim):
```
async function getData() {
  const url = "https://example.org/products.json";
  try {
    const response = await fetch(url);
    if (!response.ok) {
      throw new Error(`Response status: ${response.status}`);
    }

    const result = await response.json();
    console.log(result);
  } catch (error) {
    console.error(error.message);
  }
}
```
- alexedwards.net snippet (29:53): `↳ File: main.go` / `...` / `func main() {` / `    fs := http.FileServer(http.Dir("./static"))` / `    http.Handle("/static/", http.StripPrefix("/static/", fs))` / (blank) / `    log.Print("Listening on :3000...")` / `    err := http.ListenAndServe(":3000", nil)` / `    if err != nil {` / `        log.Fatal(err)` / `    }` / `}`. The two statements `fs := ...` and `http.Handle(...)` are highlighted (they correspond to main.go lines 13-14, adapted to ./public and /web/).
- go.dev wiki (rendered page, 29:55 onward): `renderTemplate` with `t, _ := template.ParseFiles(tmpl + ".html")` / `t.Execute(w, p)`; viewHandler/editHandler with `r.URL.Path[len("/view/"):]`, `loadPage(title)`, `&Page{Title: title}`; `http.Redirect(w, r, "/edit/"+title, http.StatusFound)`; saveHandler with `r.FormValue("body")`, `[]byte(body)`; `http.Error(w, err.Error(), http.StatusInternalServerError)`; `var templates = template.Must(template.ParseFiles("edit.html", "view.html"))`; `templates.ExecuteTemplate(w, tmpl+".html", p)`; Validation `var validPath = regexp.MustCompile("^/(edit|save|view)/([a-⟨?⟩` (cut off), getTitle using `validPath.FindStringSubmatch(r.URL.Path)`, `http.NotFound(w, r)`, `errors.New("invalid Page Title")`, `return m[2], nil // The title is the second subexpression.`; makeHandler `func makeHandler(fn func (http.ResponseWriter, *http.Request, string)) http.HandlerFunc`; "Try it out!": `$ go build wiki.go` / `$ ./wiki`, visiting `http://localhost:8080/view/ANewPage`; "Other tasks: Store templates in tmpl/ and page data in data/."

## Observations
- Workflow: a tiny Go server (`/web/` serves ./public via FileServer+StripPrefix; `/api/send` handled by `handler`), with chat.html in `public/`. He evolves the alert() demo into a fetch POST: googles "js fetch mdn", copies the MDN `getData` snippet nearly verbatim (try/catch, `response.ok`, `.json()`), points it at `/api/send`, adds `method: "POST"` (MDN "Setting the method") and `body: JSON.stringify(data)` (MDN "Setting a body"), adds `id: 1` to the payload to inspect it in devtools.
- Debugging loop: Firefox devtools Network tab, checking status (404 first because `/api/send` did not yet respond to the request as expected; GET then POST), Headers/Request tabs; by 32:02 the POST returns 200 from the Go handler (which writes "Ok ok").
- Go side: copied the FileServer/StripPrefix lines from Alex Edwards' static-sites blog (adapted); reads go.dev "Writing Web Applications" wiki repeatedly for redirects, error handling, templates, closures, but only reads in this segment.
- Intent note in code: `// now , return json` (next step: make the API return JSON so `response.json()` works).
- Style: 4-space indents, semicolons in JS, relies on docs and copy-adapt rather than memorizing; uses macOS Mission Control to juggle windows.
