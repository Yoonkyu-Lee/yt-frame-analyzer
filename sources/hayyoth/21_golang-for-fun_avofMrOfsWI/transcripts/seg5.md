# Segment 5 (21:30 - 27:47) — avofMrOfsWI "a bit of Golang for fun" (Hayyoth)

Setup: macOS desktop (traffic-light window buttons, Finder dock, Mission Control with Desktop 1/2/3), VSCodium window titled "<file> — lab" with workspace folder "LAB", Firefox windows. The editor shows NO line-number gutter in these frames (line numbers disabled), so File states below use my own sequential numbering, marked `(n)`. Webcam overlay "Human" ignored. No editor save messages are visible in this segment.

## Timeline

- [21:30] VSCodium, `main.go — lab`, sidebar hidden, terminal panel open at bottom showing `lab $ go run .` with a blinking cursor box (server is running, no other output). Editor shows main.go (handler + main with `http.HandleFunc("/hello", handler)` and `log.Fatal(http.ListenAndServe(":8080", nil))`). Behind: Firefox with alexedwards.net/blog/serving-static-sites-... (code block with `"net/http"`).
- [21:46] Sidebar (explorer "LAB") now visible: `hello/` (hello.go), `a.out`, `go.mod`, `main.go` (selected). A right-click context menu on the explorer root is open (New File..., New Folder... highlighted, Open with Five Server (root), Reveal in Finder, Open in Images Preview, Open in Integrated Terminal, Add Folder to Workspace..., Open Folder Settings, Remove Folder from Workspace, Find in Folder..., Open Timeline, Paste, Copy Path, Copy Relative Path, Run Tests, Debug Tests). main.go behind menu already contains new lines (visible fragments): `...(http.Dir("./public"))`, `s)`, `lo", handler)`, `AndServe(":8080", nil))`.
- [21:50] A new folder `public` now exists (collapsed) in the explorer; context menu on `public` shows New File... highlighted (also Cut/Copy/Paste, Rename..., Delete). Code fragments behind: `leServer(http.Dir("./public"))`, `/web", fs)`, `nc("/hello", handler)`.
- [22:02] Firefox window at `http://localhost:8080/hello` displays `Hi there, I love hello!`. Explorer: `hello/hello.go`, `public/index.html` (created), `a.out`, `go.mod`, `main.go`.
- [22:22] VSCodium: hover popup over `http.Dir(` showing godoc "type Dir string — A Dir implements FileSystem using the native file system restricted to a specific directory tree. While the FileSystem.Open method takes '/'-separated paths, a Dir's string value is a directory path on the native file system, not a URL, so it is separated by filepath.Separator, which isn't necessarily '/'. Note that Dir could expose sensitive files and directories. Dir will follow symlinks pointing out of the directory tree, ..." Cursor at end of `http.Handle("/web", fs)`. Code now: `fs := http.FileServer(http.Dir("./public"))` / `http.Handle("/web", fs)` / blank / `http.HandleFunc("/hello", handler)` / `log.Fatal(http.ListenAndServe(":8080", nil))`.
- [22:25] Firefox (localhost window) URL bar focused with `http://localhost:8080/web` selected; page body `404 page not found`; dropdown of top sites (Hayyoth hayyoth.org, Telegram Web, YouTube, ChatGPT, Google Translate).
- [22:31] Firefox with the alexedwards blog: code block highlighted: `fs := http.FileServer(http.Dir("./static"))` / `http.Handle("/", fs)`, then `log.Print("Listening on :3000...")`, `err := http.ListenAndServe(":3000", nil)`, `if err != nil { log.Fatal(err) }`. Text: "Let's step through this. First we use the http.FileServer() function to create a handler which responds to all HTTP requests with the contents of a given file system. ... Next we use the http.Handle() function to register the file server as the handler for all requests, and launch the server listening on port 3000. It's worth pointing out that in Go the pattern "/" matches all request paths, rather than just the empty path."
- [22:35] Editor: main.go with `"/web"` route unchanged; `./public` highlighted in `http.Dir("./public")`; cursor near ListenAndServe.
- [22:37] Back to Firefox blog (same section).
- [22:45] Editor: `http` identifier occurrences highlighted; cursor between `http` and `.Handle` on the `http.Handle("/web", fs)` line.
- [22:56] Blog scrolled slightly; editor explorer shows `public` collapsed. Editor main() body now has two lines beginning `http.Handl...` directly under `fs := http...` and a blank line then `log.Fatal(` (intermediate edit: HandleFunc line moved up under Handle; see 23:28/23:54).
- [22:57-23:01] Scrolling the blog: "Go ahead and run the application:" with dark block `$ go run main.go` / `Listening on :3000...`; then "And open http://localhost:3000/example.html in your browser. You should see the HTML page we made with a big red heading." Screenshot of Firefox at localhost:3000/example.html showing red heading "Hello from a static page" (tab title "A static page").
- [23:03] Firefox localhost window at `http://localhost:8080/web/index.html` -> `404 page not found`.
- [23:15] Blog: screenshot "Hello from a static page"; next heading "Almost-Static Sites".
- [23:16] Blog "Almost-Static Sites": "If you're creating a lot of static HTML files by hand, it can be tedious to keep repeating boilerplate content. Let's explore using Go's html/template package to put shared markup in a layout file. At the moment all requests are being handled by our file server. Let's make a slight adjustment to our application so the file server only handles request paths that begin with the pattern /static/ instead." Code block `File: main.go`: `...` / `func main() {` / `fs := http.FileServer(http.Dir("./static"))` / `http.Handle("/static/", http.StripPrefix("/static/", fs))` / `log.Print("Listening on :3000...")` ...
- [23:22] Blog: "Notice that because our static directory is set as the root of the file system, we need to strip off the /static/ prefix from the request path before searching the file system for the given file. We do this using the http.StripPrefix() function. If you restart the application, you should find the CSS file we made earlier available at http://localhost:3000/static/stylesheets/main.css. Now let's create a templates directory, containing a layout.html file with shared markup, and an example.html file with some page-specific content." Command block `$ mkdir templates` / `$ touch templates/layout.html templates/example.html`.
- [23:23] Blog shows `File: templates/layout.html`: `{{define "layout"}}` / `<!doctype html>` / `<html>` / `<head>` / `<meta charset="utf-8">` / `<title>{{template "title"}}</title>` / `<link rel="stylesheet" href="/static/stylesheets/main.css">` / `</head>` / `<body>` / `{{template "body"}}` / `<footer>Made with Go</footer>` / `</body>` / `</html>` / `{{end}}`.
- [23:24-23:25] Blog scrolled back up to the StripPrefix main() snippet (a selection of its two lines appears in the next frames).
- [23:28] Editor main.go (sidebar: public collapsed). main() body at this moment: `fs := http.FileServer(http.Dir("./public"))` / `http.Handle("/web", fs)` / `http.HandleFunc("/hello", handler` (closing paren missing, being edited) / (empty line with cursor) / `log.Fatal(http.ListenAndServe(":8080", nil))`. Behind it the blog selection shows `/static/", fs))`.
- [23:48] Firefox URL bar typed `http://localhost:8080/public/index.html` (page body `404 page not found`).
- [23:54] Editor main.go: line now `http.Handle("/web/", http.StripPrefix("/web/", fs))` (was `"/web", fs`), blank line, `http.HandleFunc("/hello", handler)`, blank, `log.Fatal(http.ListenAndServe(":8080", nil))`. Explorer: `public/` expanded with `index.html`; `public` row focused.
- [24:01] Editor now has `chat.html — lab` open (public/ now contains `chat.html`, `index.html`); contents `<h1>Chat pages</h1>`. Firefox at `http://localhost:8080/web/` shows rendered index.html: big "Hello World" heading and paragraph "Hayyoth here" (so `/web/` + StripPrefix works).
- [24:07] Firefox at `http://localhost:8080/web/chat.html` shows heading "Chat pages".
- [24:15] Blog again (highlighted: `fs := http.FileServer(http.Dir("./static"))` and `http.Handle("/static/", http.StripPrefix("/static/", fs))`).
- [24:16-24:18] Mission Control (Desktop 3 of 3): windows chat.html editor, Firefox localhost chat.html, Firefox blog; then back to Firefox chat.html.
- [24:29] Firefox navigated to www.w3schools.com/html/html_forms.asp ("An HTML form is used to collect user input. The user input is most often sent to a server for processing." Example with First name: John / Last name: Doe / Submit; "The <form> Element").
- [24:31] w3schools tryit page (www.w3schools.com/html/tryit.asp?filename=tr...): code listed in File states below; Result Size 423 x 586.
- [24:35] Editor chat.html: pasted the w3schools form, so chat.html temporarily is: `<h1>Chat pages</h1>` / blank / `<form action="/action_page.php">` + label/input lines + `</form>` (cursor after `</form> `).
- [25:05] chat.html trimmed (visible left side): `<h1>Chat pages</h1>` / blank / `<form action="/send">` / `  <input type="text" id="messag...` / `  <input type="submit" value="S...` / `</form>`. Firefox on w3schools tryit with form section highlighted.
- [25:06] w3schools html_forms page again.
- [25:07] Firefox `http://localhost:8080/web/chat.html` shows only heading "Chat pages" (form not rendered in frame; likely page not yet reloaded).
- [25:18] Firefox with DevTools (Network tab) open at `http://localhost:8080/send?message=hello` -> `404 page not found`. Network: `GET http://localhost:8080/send?message=hello`, Status `404 Not Found`, Version HTTP/1.1, Transferred 176 B (19 B size), Referrer Policy strict-origin-when-cross-origin, Request Priority Highest; second request favicon.ico 404. Console error badge: 2. (Form submitted via default GET with query string.)
- [25:22] Editor chat.html: `<h1>Chat pages</h1>` / blank / `<form action="/send">` / `<input type="text" id="message" name="message"><br><br>` / `<input type="submit" value="Submit">` / `</form>`.
- [25:24] Network list: row1 `404 GET localhost:... send?message=hello document plain 176 B`, row2 `404 GET localhost:... favicon.ico img plain 176 B`; "2 requests, 38 B / 352 B transferred, Finish: 36 ms, DOMContentLoaded: 18 ms, load: 26 ms".
- [25:32] URL bar focused with `http://localhost:8080/send?message=hello` (`send?message=hello` selected), top-sites dropdown shown.
- [25:41] Blog window brought to front (StripPrefix section); Network panel row now shows method `P...` (POST) `send`.
- [25:58] Firefox `http://localhost:8080/send` -> `404 page not found`; Network > Request tab: Form data `message: "hayyoth"` (POST, 404).
- [26:00-26:01] Same view (window repositioned).
- [26:05] Editor chat.html: `<form action="/send" method="post">` (method="post" added); Firefox URL bar cleared ("Search with DuckDuckGo or enter address").
- [27:31] Firefox still showing `http://localhost:8080/send` 404 with Form data `message: "hayyoth"`. Editor (behind) chat.html has been rewritten (fragments): `<h1>Chat pages</h1>` / `<input type="text"...` / `<button onclick="s...` / blank / blank / `<script>` / `const inpMess = do...` / `function send() {` / `alert(inpMess....` / `}` / ... / `</script>`.
- [27:39] Firefox at `http://localhost:8080/web/chat.html`: page shows "Chat pages", a text input with `hayyoth` typed, "Send" button. Network: `200 GET localhost:8... chat.html document html 433`, `404 GET favicon.ico img plain 176`; 267 B / 609 B transferred, Finish 36 ms, DOMContentLoaded 18 ms, load 27 ms. Console error badge 1.
- [27:40] Clicked Send: browser alert "localhost:8080 — hayyoth" with OK.
- [27:41] Dialog dismissed, page shows input `hayyoth`.
- [27:44] Second alert: input `hello`, dialog "localhost:8080 — hello" with checkbox "Don't allow localhost:8080 to prompt you again", OK.
- [27:45] Dialog dismissed, input `hello`.
- [27:47] Editor chat.html final (full file below).

## File states

### main.go (end of segment, as of 23:54; no later view)
```
(1)  package main
(2)  
(3)  import (
(4)      "fmt"
(5)      "log"
(6)      "net/http"
(7)  )
(8)  
(9)  func handler(w http.ResponseWriter, r *http.Request) {
(10)     fmt.Fprintf(w, "Hi there, I love %s!", r.URL.Path[1:])
(11) }
(12) 
(13) func main() {
(14)     fs := http.FileServer(http.Dir("./public"))
(15)     http.Handle("/web/", http.StripPrefix("/web/", fs))
(16) 
(17)     http.HandleFunc("/hello", handler)
(18) 
(19)     log.Fatal(http.ListenAndServe(":8080", nil))
(20) }
```
Indent is 4 spaces. The import strings are red-underlined in the editor in all frames (squiggle).

Version at 21:30 (before edits):
```
(1)  package main
(2)  
(3)  import (
(4)      "fmt"
(5)      "log"
(6)      "net/http"
(7)  )
(8)  
(9)  func handler(w http.ResponseWriter, r *http.Request) {
(10)     fmt.Fprintf(w, "Hi there, I love %s!", r.URL.Path[1:])
(11) }
(12) 
(13) func main() {
(14)     http.HandleFunc("/hello", handler)
(15)     log.Fatal(http.ListenAndServe(":8080", nil))
(16) }
```
Version at 22:22-22:45 (intermediate) main() body:
```
    fs := http.FileServer(http.Dir("./public"))
    http.Handle("/web", fs)

    http.HandleFunc("/hello", handler)
    log.Fatal(http.ListenAndServe(":8080", nil))
```

### public/index.html
Not shown in the editor. Rendered at /web/ as an h1 "Hello World" and paragraph "Hayyoth here". Source not visible.

### public/chat.html (end of segment, 27:47)
```
(1)  <h1>Chat pages</h1>
(2)  <input type="text" id="inpMess" placeholder="Enter Message"><br><br>
(3)  <button onclick="send()">Send</button>
(4)  
(5)  
(6)  <script>
(7)  const inpMess = document.getElementById("inpMess");
(8)  function send() {
(9)      alert(inpMess.value);
(10) }
(11) 
(12) 
(13) </script>
```

Earlier versions:
- 24:35 (after pasting the w3schools sample):
```
<h1>Chat pages</h1>

<form action="/action_page.php">
  <label for="fname">First name:</label><br>
  <input type="text" id="fname" name="fname" value="John"><br>
  <label for="lname">Last name:</label><br>
  <input type="text" id="lname" name="lname" value="Doe"><br><br>
  <input type="submit" value="Submit">
</form> 
```
- 25:22:
```
<h1>Chat pages</h1>

<form action="/send">
  <input type="text" id="message" name="message"><br><br>
  <input type="submit" value="Submit">
</form>
```
- 26:05:
```
<h1>Chat pages</h1>

<form action="/send" method="post">
  <input type="text" id="message" name="message"><br><br>
  <input type="submit" value="Submit">
</form>
```
(form replaced by the JS version between 26:05 and 27:31)

### Reference: w3schools Try-it code seen at 24:31
```
<!DOCTYPE html>
<html>
<body>

<h2>HTML Forms</h2>

<form action="/action_page.php">
  <label for="fname">First name:</label><br>
  <input type="text" id="fname" name="fname" value="John"><br>
  <label for="lname">Last name:</label><br>
  <input type="text" id="lname" name="lname" value="Doe"><br><br>
  <input type="submit" value="Submit">
</form>

<p>If you click the "Submit" button, the form-data will be sent to a page called "/action_page.php".</p>

</body>
</html>
```

## Shell & outputs

- Terminal (VSCodium integrated) constant through the whole segment: `lab $ go run .` (hollow circle prompt marker, cursor box below, no output). No new shell commands are typed in this segment; the server is not visibly restarted in sampled frames (restarts may have occurred between frames).
- Browser-rendered outputs:
  - [22:02] `http://localhost:8080/hello` -> `Hi there, I love hello!` (plain text).
  - [22:25] `http://localhost:8080/web` -> `404 page not found`.
  - [23:03] `http://localhost:8080/web/index.html` -> `404 page not found`.
  - [24:01] `http://localhost:8080/web/` -> "Hello World" (h1) / "Hayyoth here".
  - [24:07] `http://localhost:8080/web/chat.html` -> "Chat pages" (h1).
  - [25:18] `http://localhost:8080/send?message=hello` -> `404 page not found` (GET, 404 Not Found, 176 B).
  - [25:58] `http://localhost:8080/send` POST, form data `message: "hayyoth"` -> `404 page not found`.
  - [27:39] `http://localhost:8080/web/chat.html` -> input + Send button; Network `200 chat.html html 433`.
  - [27:40/27:44] JS alert dialogs "hayyoth" and "hello".
- Third-party pages viewed: alexedwards.net "Serving Static Sites with Go" (FileServer, run the app, Almost-Static Sites, StripPrefix, templates/layout.html); w3schools HTML Forms page and Try-it editor.

## Observations

- Build order: static file serving first (`http.FileServer` + `http.Dir("./public")`, mounted at `/web`); `/web` and `/web/index.html` 404ed, so he adopted the blog's pattern `http.Handle("/path/", http.StripPrefix("/path/", fs))` as `/web/`. Verified by URL in the browser (`/web/` serves index.html; `/web/chat.html` works).
- Then began a chat page (`chat.html`) with an HTML form copied from w3schools; first `<form action="/send">` (GET with `?message=hello`, 404 since no /send handler), then added `method="post"` (form data seen in Network > Request). Then pivoted to a JS approach: `<input id="inpMess">` + `<button onclick="send()">` + `alert(inpMess.value)` as a stand-in for sending a message (presumably next: fetch/WebSocket to the Go server).
- Style: learns by reading tutorials (alexedwards.net blog, godoc hover tooltips, w3schools) and adapting snippets; uses Firefox DevTools Network tab to inspect requests; leaves the `/hello` handler in place; 4-space indentation in his own Go/JS/HTML (w3schools' 2-space indent kept when pasted).
- VSCodium extras visible: "Open with Five Server (root)" (Five Server extension), "Open in Images Preview", Run/Debug Tests (Go extension).
- Apparent goal: a small chat app ("Chat pages") served by Go.
