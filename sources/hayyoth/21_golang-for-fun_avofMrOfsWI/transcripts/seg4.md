# Segment 4/17 (19:00 - 21:30) — avofMrOfsWI "a bit of Golang for fun"

## Timeline
- [19:00-19:05] Firefox window (front, URL go.dev/doc/articles/wiki/) scrolled through the Go "Writing Web Applications" tutorial; VSCodium window (`main.go — lab`) sits behind at left, partly covered. Editor content: the random demo (`fmt`, `math/rand`, `fmt.Println(rand.Int…`). Terminal shows 6 earlier `lab $ go run main.go` runs with outputs 0, 0, 1, 1, 5, 0, then an empty prompt. Page sections read: "Data Structures" (`type Page struct { Title string; Body []byte }`), `func (p *Page) save() error` (`filename := p.Title + ".txt"`, `return os.WriteFile(filename, p.Body, 0600)`), `loadPage` (first version ignoring error with `_`, then the `(*Page, error)` version), the test main (`p1 := &Page{Title: "TestPage", Body: []byte("This is a sample Page.")}`, `p1.save()`, `p2, _ := loadPage("TestPage")`, `fmt.Println(string(p2.Body))`), and `$ go build wiki.go` / `$ ./wiki` / `This is a sample Page.`
- [19:06-19:08] Scrolls to "Introducing the net/http package (an interlude)": full web-server example (`//go:build ignore`, `package main`, imports fmt/log/net/http, `handler` printing `"Hi there, I love %s!"` with `r.URL.Path[1:]`, main with `http.HandleFunc("/", handler)` and `log.Fatal(http.ListenAndServe(":8080", nil))`). Cursor hovers over the import list.
- [19:10] Browser window shifts right (left edge x~476); editor comes to front (`main.go — lab`), the word `Intn` partly highlighted by the I-beam. Editor still `fmt.Println(rand.Intn(10))`.
- [19:11] Editor behind browser again. Import block is now EMPTY (`import (` / `)`); `fmt.Println(rand.Intn(10))` has red squiggles under `fmt` and `rand`; lightbulb quick-fix icon in the gutter. The author deleted `"fmt"` and `"math/rand"` from the imports.
- [19:16-19:17] Browser scrolled over the example; editor in front at 19:17 with `Println(rand.Intn(10))` selected (highlighted blue), about to delete it.
- [19:25] Editor in front, content between 19:17 and 19:38 reduced to imports `"log"`, `"net/http"` and `func main()` with `log.Fatal(http.ListenAndServe(":80…` (confirmed at 19:38). Terminal shows `lab $ go run .` with caret on next line (server running, blocking). (No frame at 19:25 text beyond this; it is the step where rand code was replaced.)
- [19:38] Browser URL bar focused, text `lemire.me/` selected with autocomplete row "lemire.me — Visit" (blank page behind; public site). Editor: imports `"log"`, `"net/http"`, `func main() {` / `log.Fatal(http.ListenAndServe(":80…`. Terminal: `lab $ go run .`.
- [19:45] Back on go.dev/doc/articles/wiki/, web-server example; author drag-selects `func handler(w http.ResponseWriter, r *http.Request) {` … `fmt.Fprintf(w, "Hi there, I love %s!", r.URL.Path[1:])` … `}` to copy it.
- [19:46] Editor in front (title dimmed during focus change). Caret on the blank line between `)` and `func main() {` (about to paste). main body: `log.Fatal(http.ListenAndServe(":8080", nil))`.
- [19:57] Pasted: imports now `"fmt"`, `"log"`, `"net/http"` (fmt re-added); `func handler(w http.ResponseWriter, r …` with `fmt.Fprintf(w, "Hi there, I love %…`; `func main() {` / `log.Fatal(http.ListenAndServe(":80…`. `handler` has a wavy underline (unused). Browser still has the handler code selected.
- [20:00] Editor in front, full text visible (see File states); caret right after `func main() {`. Terminal: `lab $ go run .`.
- [20:08] Editor main now has `http.HandleFunc("/", handler)` as the first line (browser shows that same line highlighted in the tutorial = copied). Editor pane layout grew: TERMINAL header moved from y~598 to y~746. Terminal still `lab $ go run .` with caret on next line. Firefox "List all tabs" panel open; tabs: "crypto package - crypto - Go Packages", "fmt package - fmt - Go Packages", "Writing Web Applications - The Go Programming Langua…" (bold, current), "localhost:8080/".
- [20:16-20:25] Author reads the wiki text: explanation of HandleFunc/ListenAndServe/log.Fatal/handler/ResponseWriter/Request; "http://localhost:8080/monkeys" -> page "Hi there, I love monkeys!"; "Using net/http to serve wiki pages" (import list fmt, os, log, net/http; `viewHandler`: `title := r.URL.Path[len("/view/"):]`, `p, _ := loadPage(title)`, `fmt.Fprintf(w, "<h1>%s</h1><div>%s</div>", p.Title, p.Body)`); main with routes `/view/`, `/edit/`, `/save/` (viewHandler, editHandler, saveHandler); `editHandler` with hard-coded HTML form; start of "The html/template package". At 20:20 and 20:21 he scrolls back up to the simple server example (`http.HandleFunc("/", handler)` highlighted). Editor unchanged. (Webcam background changes — ignored.)
- [20:23-20:36] Reads html/template section: imports (html/template, os, net/http), `edit.html` (`<h1>Editing {{.Title}}</h1>`, `<form action="/save/{{.Title}}" method="POST">`, `<div><textarea name="body" rows="20" cols="80">{{printf "%s" .Body}}</textarea></div>`, `<div><input type="submit" value="Save"></div>`, `</form>`), editHandler with `t, _ := template.ParseFiles("edit.html")` / `t.Execute(w, p)`, explanation of printf and escaping, `view.html` (`<h1>{{.Title}}</h1>`, `<p>[<a href="/edit/{{.Title}}">edit</a>]</p>`, `<div>{{printf "%s" .Body}}</div>`), `renderTemplate(w http.ResponseWriter, tmpl string, p *Page)` helper. Editor unchanged.
- [20:37] End of section: `viewHandler`/`editHandler` calling `renderTemplate(w, "view", p)` / `renderTemplate(w, "edit", p)`; "If we comment out the registration of our unimplemented save handler in main, we can once again build and test our program."; heading "Handling non-existent pages".
- [20:38] Browser URL bar being typed: `gola` with Firefox Suggest dropdown (x.com/golang, DuckDuckGo "golang x", YouTube "Is Golang Dying? 2026 Outlook", several Google Translate entries).
- [20:43] DuckDuckGo results for URL-bar text `golang static sites handle`: Alex Edwards "Serving Static Sites with Go", GitHub darccio/zas, Stack Overflow "go - Serving static files in an HTTP server", "How do you serve a static html file using a go web server".
- [20:47-20:59] Reads www.alexedwards.net/blog/serving-static-sites-with-go: `$ mkdir static-site`, `$ cd static-site`, `$ touch main.go`, `$ mkdir -p static/stylesheets`, `$ touch static/example.html static/stylesheets/main.css`; static/example.html (`<!doctype html>`, `<title>A static page</title>`, `<link rel="stylesheet" href="/stylesheets/main.css">`, `<h1>Hello from a static page</h1>`); main.css `body {color: #c0392b}`; main.go (package main, imports "log", "net/http", FileServer). Explanation text (FileServer, http.FileSystem, http.Handle, port 3000, pattern "/" matches all paths). Run block: `$ go run main.go` / `Listening on :3000...`; screenshot of "Hello from a static page" at localhost:3000/example.html; then "Almost-Static Sites" (html/template layout; main.go snippet with `fs := http.FileServer(http.Dir("./static"))`, `http.Handle("/static/", http.StripPrefix("/static…`, `log.Print("Listening on :3000...")`, `err := http.ListenAndServe(":3000", nil)`, `if err != nil { log.Fatal(err) }`). Editor unchanged behind.
- [21:00] Editor in front (`main.go — lab`), same text as 20:00-20:08 (route still `"/"`); I-beam on the word "Hi" in `"Hi there…"`.
- [21:14] Browser back on go.dev wiki article (renderTemplate/handlers part). Editor behind now shows `http.HandleFunc("/hello", handler)` — route edited from "/" to "/hello" (and the server presumably restarted; not visible).
- [21:16] Browser on alexedwards.net; lines `fs := http.FileServer(http.Dir("./static"))` and `http.Handle("/", fs)` highlighted. Editor behind still "/hello".
- [21:18] A new Firefox window appears (offset), URL bar focused with `lemire.me/` and a suggestion dropdown (account-specific YouTube Studio entries — not transcribed).
- [21:26] The new small Firefox window shows URL `http://localhost:8080/hello`, content in monospace: `Hi there, I love hello!`.
- [21:27] That localhost window is partly covered; Alex Edwards page in front again.
- [21:30] Editor in front (`main.go — lab`): a blank line inserted between handler's closing `}` and `func main() {`; caret on that blank line. Terminal still `lab $ go run .` (no re-run visible).

## File states
### main.go as of 21:30 (editor shows no line numbers; numbering below is by visible line)
```
1| package main
2| 
3| import (
4|     "fmt"
5|     "log"
6|     "net/http"
7| )
8| 
9| func handler(w http.ResponseWriter, r *http.Request) {
10|     fmt.Fprintf(w, "Hi there, I love %s!", r.URL.Path[1:])
11| }
12| 
13| func main() {
14|     http.HandleFunc("/hello", handler)
15|     log.Fatal(http.ListenAndServe(":8080", nil))
16| }
```
(At 21:30 a stray extra blank line with caret sits between lines 11 and 13, i.e. there appear to be two blank lines in the editor frame at the moment: the frame shows `}` then caret-line then `func main() {` with the blank gap taller than before.) Indentation is a tab/4 columns.

Intermediate version at 20:00-21:00 (route "/" instead of "/hello"; `http.HandleFunc("/", handler)`; otherwise identical).

Intermediate at 19:38 (before paste):
```
1| package main
2| 
3| import (
4|     "log"
5|     "net/http"
6| )
7| 
8| func main() {
9|     log.Fatal(http.ListenAndServe(":8080", nil))
10| }
```
Version at 19:11-19:17 (imports deleted, body not yet):
```
1| package main
2| 
3| import (
4| )
5| 
6| func main() {
7|     fmt.Println(rand.Intn(10))
8| }
```
Version at 19:00-19:10 (rand demo):
```
1| package main
2| 
3| import (
4|     "fmt"
5|     "math/rand"
6| )
7| 
8| func main() {
9|     fmt.Println(rand.Intn(10))
10| }
```

## Shell & outputs
- [19:00-19:17] Integrated terminal (prompt `lab $`):
```
lab $ go run main.go
0
lab $ go run main.go
0
lab $ go run main.go
1
lab $ go run main.go
1
lab $ go run main.go
5
lab $ go run main.go
0
lab $ ▯
```
- [19:25-21:30] Terminal reduced to `lab $ go run .` with the caret on the next line (server running and blocking; no output printed).
- Browser-rendered output [21:26]: URL `http://localhost:8080/hello`, white page with monospace text `Hi there, I love hello!`.
- No compiler errors in the terminal in this segment; editor squiggles at 19:11 (undefined `fmt`/`rand` after removing imports) and a wavy underline on `handler` at 19:57-20:00 (unused).
- No editor save messages are visible (no status bar; window title is `main.go — lab`).

## Observations
- The author follows the official Go "Writing Web Applications" article, copying the net/http interlude example in pieces (first the `handler` func, then the `http.HandleFunc("/", handler)` line) after clearing out the previous random-number demo (first the imports, then the body).
- Rapid test loop: renames the route to "/hello" and checks http://localhost:8080/hello in a separate small Firefox window ("Hi there, I love hello!") to confirm `r.URL.Path[1:]` behaviour.
- He reads through the html/template and renderTemplate sections without coding them, then deliberately searches `golang static sites handle` and reads Alex Edwards' "Serving Static Sites with Go" (http.FileServer, http.Dir("./static"), http.StripPrefix), i.e. exploring static-file serving rather than finishing the wiki tutorial.
- Open Firefox tabs: crypto package, fmt package (Go Packages), the wiki tutorial, localhost:8080/ — signs of earlier exploration.
- Address bar repeatedly autocompletes `lemire.me/` (public personal blog) when he starts typing; no navigation to it is visible in this segment.
- Terminal use is minimal (single `lab $` prompt, cleared before `go run .`).
