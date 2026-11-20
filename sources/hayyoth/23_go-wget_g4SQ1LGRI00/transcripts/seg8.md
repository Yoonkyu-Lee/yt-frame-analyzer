# Segment 8 of 9 (17:22 - 21:33) - go wget crawler (Hayyoth)

Privacy note: the crawled site's domain is written `<site>.org` (its path names are kept); the GitHub user/org name is `<account>`.

## Timeline
- [17:22-17:25] Firefox window (pkg.go.dev/net/url) with a page-section dropdown open: Documentation (Overview, Index, Examples, Constants [checked], Variables, Functions, Types) and Examples (ParseQuery, PathEscape, PathUnescape, QueryEscape, QueryUnescape, URL, URL (Roundtrip), URL.EscapedFragment, URL.EscapedPath, URL.Hostname, URL.IsAbs, URL.JoinPath, URL.MarshalBinary, URL.Parse, URL.Port, URL.Query, URL.Redacted, URL.RequestURI, URL.ResolveReference, URL.String, URL.UnmarshalBinary, Values, Values.Add...). He moves the mouse up the list and selects "URL" at 17:25. Editor `main.go - lab` is behind on the left.
- [17:27] URL becomes `pkg.go.dev/net/url#example-URL`; dropdown label "Parse(rawURL)", collapsed "Example" and "Example (Roundtrip)", then `func Parse(rawURL string) (*URL, error)`: "Parse parses a raw url into a URL structure. The url may be relative (a path, without a host) or absolute (starting with a scheme). Trying to parse a hostname and path without a scheme is invalid but may not necessarily return an error, due to parsing ambiguities." Then `func ParseRequestURI(rawURL string) (*URL, error)`.
- [17:29-17:30] "Example" expanded. Example code: package main; imports "fmt", "log", "net/url"; `func main() {` `u, err := url.Parse("http://bing.com/search?q=dotnet")`, `if err != nil { log.Fatal(err) }`, `u.Scheme = "https"`, `u.Host = "google.com"`, `q := u.Query()`, `q.Set("q", "golang")`, `u.RawQuery = q.Encode()`, `fmt.Println(u)`.
- [17:38] Windows rearranged (Firefox at right, editor at left). He has highlighted the first four lines of the example (`u, err := url.Parse(...)` through the closing `}` of the err check). Editor shows save func with `fmt.Println("===> save ", link)` and a comment `// to create file , we need remove https from link`; `content` param has blue squiggle (unused).
- [17:39] Editor focused, cursor at end of the comment line.
- [17:40] Pasted example lines into `save`, between the Println and the comment: `u, err := url.Parse("http://bing.com/search?q=dotnet")` (red squiggle under `u` and `url`), `if err != nil {`, `log.Fatal(err)`, `}`.
- [18:01] Changed to `u, err := url.Parse(link)`; added `fmt.Println(u.Host)` after the err check; the comment line removed; blank line with cursor below it.
- [18:02] Cursor inside `Host` in `fmt.Println(u.Host)`.
- [18:15] Terminal panel opened (maximised, tab TERMINAL): `lab $ ` prompt only, empty.
- [18:35] Back in editor; first line `fmt.Println("===> save ", link)` in save is highlighted (selected); the Println after err check is `fmt.Println("=========", u.Host, "----", u.Path)` (the `"----"` being edited; mouse cursor over it).
- [18:38] New line above that Println: partially typed `lin` (red squiggle), beginning of `link = ...`. Println is `fmt.Println("=========", u.Host, "----", u.Path)`.
- [18:47] save now: first Println removed; `u, err := url.Parse(link)` / err check / `link = u.Host + u.Path` / `fmt.Println("=========", u.Host, "----", u.Path)`.
- [18:51] Terminal run (see Shell). 87x37 size hint. `/about/sites/` selected.
- [18:58] Last Println changed to `fmt.Println("===> save ", link)` after `link = u.Host + u.Path` (the "=========" debug print removed). `content` underlined again (unused).
- [18:59] Blank line then `// ` typed below Println.
- [19:02] Comment finished: `// create folder`. Explorer sidebar (LAB: go.mod, go.sum, main.go) now shown, editor shifted right.
- [19:05] Firefox brought to front, still on pkg.go.dev/net/url Parse example.
- [19:06-19:10] In Firefox URL bar he types `github.com/` (Firefox Suggest list shown), then `github.com/<account>` and `github.com/<account>/gfm`; status "Looking up github.com...".
- [19:12] GitHub repo page "gfm" (Public; "Golang File Manager"; tags cli, file-manager, golang-cli, golang-tool, linux-tool; MIT license; 4 stars, 1 fork). Webcam person with hand on head.
- [19:13] Repo file list: .gitignore, LICENSE, README.md, go.mod, help.go, main.go; README heading "gfm - Golang File Manager". Mouse hovering the README row.
- [19:15] Opens main.go (109 lines (99 loc), 2.16 KB). Lines 1-22 visible: `package main`, imports "flag" "fmt" "log" "os", `func main() {`, `flag.Usage = func() { fmt.Fprint(os.Stderr, help...`, `log.SetFlags(0) // hide timest...`, `// to read arg by flag, we mus...`, `flag.Parse()`, `err := execCommand()`, `if err != nil { log.Fatal(err) }`.
- [19:16-19:19] Scrolls reference `execCommand() error` (switch over `getArg(0, helpMess)`): case "info" (`os.Stat(path)`, Printf Name/Size/Mode/IsDir/Modification...), "ls" (`os.ReadDir(".")`, loop printing `file.Name()`), "rm" (`os.RemoveAll(path)`), "pwd" (`os.Getwd()`), "mv" (`os.Rename(path1, path2)`), "mkdir" (`path := getArg(1, "Enter Dir")`, `return os.MkdirAll(path, 0777)`), "cat" (`os.ReadFile(path)`, `os.Stdout.Write(data)`), "touch" (`os.Create(path)`).
- [19:23] Back in editor (blank line above `// create folder`, cursor there). Firefox shows `(path, 0777)` of the MkdirAll line highlighted.
- [19:32] Terminal run (see Shell), size hint "66 x 37"; link printed without scheme.
- [19:42] Editor: `os.MkdirAll(link, 0777)` added after the Println, with `// create folder` comment below it. Explorer now shows created directories: `<site>.org / about` with subfolders `father`, `sites`, `<site>` then go.mod, go.sum, main.go. Occurrences of `link` highlighted.
- [19:47] Old comment line replaced; new `//` being typed under a blank line below MkdirAll.
- [19:51] Comment now `// save file content`. Firefox shows gfm main.go lines 65-89 (`return os.MkdirAll(path, 0777)` highlighted).
- [19:52-19:56] Scrolls reference: lines 70-108: touch (`_, err := os.Create(path)`), cp (`os.ReadFile(path1)`, `return os.WriteFile(path2, data, 0777)`), echo (`os.WriteFile(path, []byte(content), 0777)`), default `fmt.Print(helpMess)`, `// this function must get arg` / `// if user not enter arg , it will...` / `func getArg(indexArg int, mess str...` with `val := flag.Arg(indexArg)`, `if val == "" { log.Fatal(mess) }`, `return val`.
- [20:02] Editor: cursor on blank line above `// save file content`; in Firefox the echo line `os.WriteFile(path, []byte(content), 0777)` is highlighted.
- [20:07] Added below the comment: `os.WriteFile(path, []byte(content), 0777)`. Hover error: `undefined: path compiler(UndeclaredName)`, "View Problem (Opt+F8)", "Quick Fix... (Cmd+.)". main.go in sidebar red with badge `1`.
- [20:08] Terminal shows previous run output (same as 19:32) with main.go badge 1.
- [20:26] Editor: `os.WriteFile(path, ...)` with red wavy under `path`; cursor at end of line.
- [20:30-20:31] Blank line(s) inserted above `// save file content` (cursor on new empty line, then moved further).
- [20:55] Added:
  `if strings.HasSuffix(link, "/") {` / blank / `}` (typing `"/"`).
- [21:19] Body filled: `link += "index.html"`; comment `// if link is example.com/somethin` being typed above the if; the os.WriteFile line is commented out as `//os.WriteFile(path, []byte(content), 0777)`; `content` underlined again; main.go badge gone.
- [21:30] Two comment lines (selected): `// if link is example.com/something.html -> not add` and `// if not -> add index.html`.
- [21:31] Those two comment lines deleted (cursor on blank line above `if`).
- [21:33] Cursor inside `"index.html"` (after `inde`). No editor save message visible in this segment.

## File states
Editor shows no line numbers; none are available. Top of file (package/imports) not visible. State at 21:33:

```go
?? (top of file: package, most imports not visible)
	"golang.org/x/net/html"
	"golang.org/x/net/html/atom"
)

func save(link string, content []byte) {
  u, err := url.Parse(link)
  if err != nil {
    log.Fatal(err)
  }
  link = u.Host + u.Path
  fmt.Println("===> save ", link)
  os.MkdirAll(link, 0777)

  if strings.HasSuffix(link, "/") {
    link += "index.html"
  }

  // save file content
  //os.WriteFile(path, []byte(content), 0777)
}

func crawl(link string) {
  client := &http.Client{}

  req, err := http.NewRequest("GET", link, nil)
  if err != nil {
    log.Fatal(err)
  }
  req.Header.Add("User-Agent", `Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:159 ?? (rest of line cut off)
  resp, err := client.Do(req)
  if err != nil {
    log.Fatal(err)
  }

  defer resp.Body.Close()
  bodyByte, err := io.ReadAll(resp.Body)
  ?? (lines below not visible in later frames; earlier frame at 17:38 showed:)
  body := string(bodyByte)
  save(link, bodyByte)

  doc, err := html.Parse(strings.NewReader(body))
  ?? (rest of crawl not visible)
```
Indentation 2 spaces (indent guides).

Earlier version at 17:38:
```go
func save(link string, content []byte) {
  fmt.Println("===> save ", link)
  // to create file , we need remove https from link
}
```
Version at 18:47:
```go
func save(link string, content []byte) {
  u, err := url.Parse(link)
  if err != nil {
    log.Fatal(err)
  }
  link = u.Host + u.Path
  fmt.Println("=========", u.Host, "----", u.Path)
}
```
Other files: go.mod, go.sum. Output directories created by runs: `<site>.org/about/{father,sites,<site>}`.

## Shell & outputs
- [18:15] Terminal: `lab $ ` (empty).
- [18:51]
```
lab $ go run .
===> save  https://<site>.org
========= <site>.org ----
===> save  https://<site>.org/about/<site>/
========= <site>.org ---- /about/<site>/
===> save  https://<site>.org/about/father/
========= <site>.org ---- /about/father/
===> save  https://<site>.org/about/sites/
========= <site>.org ---- /about/sites/
lab $
```
- [19:32] (and again displayed at 20:08)
```
lab $ go run .
===> save  <site>.org
===> save  <site>.org/about/<site>/
===> save  <site>.org/about/father/
===> save  <site>.org/about/sites/
lab $
```
- Web pages: pkg.go.dev/net/url (func Parse docs + Example), GitHub repo "gfm" and its main.go (described in Timeline).

## Observations
- Builds save() incrementally, comment-first: writes a comment describing the next step (`// create folder`, `// save file content`, `// if link is ... -> add index.html`), then codes it, and deletes the comment afterwards.
- Heavy use of debug Printlns and `go run .` to inspect url.Parse fields (Host, Path) before deciding the on-disk path.
- Consults official docs (pkg.go.dev net/url Parse example), copy-pastes example lines and adapts them; also consults his own earlier file-manager repo as reference for os.MkdirAll / os.WriteFile with 0777.
- Design: file path = u.Host + u.Path; directories via os.MkdirAll(link, 0777); links ending in "/" get "index.html" appended. Note: at 21:33 os.MkdirAll is still called on the full link (including a would-be file name) and os.WriteFile is commented out and still refers to undefined `path` - unresolved here.
- Webcam overlay ("Human") ignored.
