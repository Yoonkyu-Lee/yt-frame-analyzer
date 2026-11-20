# Segment 5 (09:24 - 13:07) - g4SQ1LGRI00

Setup: editor window titled "main.go — lab" (light theme, NO line numbers shown, embedded TERMINAL panel toggles open/closed at bottom), Firefox window behind it (right). Webcam overlay ignored. Terminal prompt is `lab $`.

## Timeline
- [09:24] Terminal open under main.go. `lab $ go run .` output: `/`, `#wingramorg`, `#about`, `about/wingram/`, `about/father/`, `about/sites/`. Firefox shows wingram.org: title "Wingram.Org", tagline "Wingram on the way we find the Truth.", "About" list with links "Wingram Philosophy", "Wingram Father", "Sites - Contact"; footer "Don't forget b..." / "You can forget this site, bu...".
- [09:29-09:34] Same output; author selects/points at the first lines (`/`, `#wingramorg`, `#about`) with the mouse - the `#` fragment links are unwanted.
- [09:48] Terminal closed. Editor shows a half-typed edit inside the link loop: after `link = a.Val`, blank line, then `if ` (cursor after `if `) and the old `break` underlined red.
- [09:49] Firefox new tab: empty address bar "Search or enter address".
- [09:51-09:52] Types `pkg.go.dev/` (Firefox Suggest: net/http, std, golang.org/x/net/html, html, net/url, io/fs, path/filepath, websocket...), then `pkg.go.dev/std`; suggestion "strings package - strings - Go Packages pkg.go.dev/strings#HasSuffix".
- [09:57] Page pkg.go.dev/strings: "strings package, standard library, Version: go1.27.3 Latest, Published: Oct 20, 2026, License: BSD-3-Clause, Imports: 11, Imported by: 2,683,284".
- [09:58] Scrolled to Documentation / Overview: "Package strings implements simple functions to manipulate UTF-8 encoded strings." Index starts: func Clone, Compare, Contains, ContainsAny...
- [09:59-10:08] Opens the outline dropdown (Outline: Documentation, Source Files; Documentation: Overview, Index, Examples, Constants, Variables, Functions, Types; Examples: Builder, Clone, Compare, Contains, ContainsAny, ContainsFunc, ContainsRune, Count, Cut, CutPrefix, CutSuffix, EqualFold, Fields, FieldsFunc, FieldsFuncSeq, FieldsSeq, HasPrefix, HasSuffix, Index, IndexAny, IndexByte, IndexFunc, IndexRune, Join, LastIndex, LastIndexAny, LastIndexByte, LastIndexFunc, Lines, Map, NewReplacer, Repeat, Replace, ReplaceAll, Split, SplitAfter, SplitAfterN, SplitAfterSeq, SplitN, SplitSeq, Title, ToLower, ToLowerSpecial, ToTitle, ToTitleSpecial, ToUpper, ToUpperSpecial, ToValidUTF8, Trim, TrimFunc, TrimLeft, TrimLeftFunc, TrimPrefix, TrimRight, TrimRightFunc, TrimSpace, TrimSuffix; then "Functions"). Scrolls the list.
- [10:09-10:10] Index section: `func Cut(s, sep string) (before, after string, found bool)`, `func CutLast(...)`, `func CutPrefix(s, prefix string) (after string, found bool)`, `func CutSuffix(s, suffix string) (before string, found bool)`, `func EqualFold(s, t string) bool`, `func Fields(s string) []string`, `func HasPrefix(s, prefix string) bool`, `func HasSuffix(s, suffix string) bool`, `func Index(s, substr string) int`, ..., `func Join(elems []string, sep string) string`, `func Lines(s string) iter.Seq[string]`.
- [10:12] Clicks HasPrefix: URL `pkg.go.dev/strings#HasPrefix`. "func HasPrefix(s, prefix string) bool - HasPrefix reports whether the string s begins with prefix."
- [10:15-10:16] Scrolls to `func HasSuffix(s, suffix string) bool` - "HasSuffix reports whether the string s ends with suffix."; also `func Index(s, substr string) int` ("Index returns the index of the first ins[tance of substr in s, or -1 if substr is not] present in s.").
- [10:17] Back to editor, still the `if ` / `break` stub.
- [10:59] Terminal open; `go run .` output now: `/`, `about/wingram/`, `about/father/`, `about/sites/` (the `#...` lines gone). Author selects lines.
- [11:00] Same, unselected.
- [11:06] Editor (scrolled) shows the completed edit: `if !strings.HasPrefix(link, "#") {` / `fmt.Println(link)` / `}` / `break`.
- [11:07] Scrolled further; cursor at a closing `}`.
- [11:15-11:19] Scrolling through the whole file; `fmt.Println(link)` selected (highlighted). Top of file at 11:16: imports `"net/http"`, `"strings"`, blank, `"golang.org/x/net/html"`, `"golang.org/x/net/html/atom"`, `)`, then `func crawl(link string) {`.
- [11:25] Edit in progress: `link = a.Val` changed to `subLink = a.Val` (red squiggle; undeclared), later lines still use `link`.
- [11:57] Now `subLink := a.Val` and `if !strings.HasPrefix(subLink, "#") {` containing `crawl(link + "/" + subLink)` (recursion replaces fmt.Println).
- [11:58-12:01] Scrolling; at 12:01 a blank line is inserted after `body := string(bodyByte)`.
- [12:14] Added `fmt.Println("===============> ", link)` after `body := string(bodyByte)`.
- [12:15] Cursor at end of that Println line.
- [12:20] Terminal: `lab $ go run .` output, still running:
  ```
  ===============>  https://wingram.org
  ===============>  https://wingram.org//
  ===============>  https://wingram.org////
  ===============>  https://wingram.org//////
  ===============>  https://wingram.org////////
  ```
  The root `/` link is appended repeatedly (infinite recursion); author gestures.
- [12:24-12:25] Terminal closed; editor with cursor at end of the Println line.
- [12:42] Condition being edited (view scrolled a few px left): `if !strings.HasPrefix(subLink, "#") && !strings.HasPrefix(subLink, "#") {` (second clause copy-pasted, still "#").
- [12:49] Terminal cleared: only `lab $ `.
- [12:51] Condition now `if !strings.HasPrefix(subLink, "#") && !strings.HasPrefix(subLink, "/") {`.
- [12:52] `lab $ go run .` running.
- [12:56-12:57] Output shows base URL, `.../about/wingram/`, `.../about/father/`, then `https://wingram.org/about/father//https://www.gnu.org/philosophy/who-does-that-server-really-serve`, then `.../who-does-that-server-really-serve/https://www.mkdocs.org/`, then `//https://www.mkdocs.org/` stacking more each line.
- [12:58] Ends with `^Csignal: interrupt` then `lab $ `.
- [12:59] Terminal scrolled up: `lab $` / `lab $ go run .` (red dot marker) and the output.
- [13:05-13:07] Terminal closed; editor viewed with mouse at the `"/"`; view scrolled horizontally right (left edge clipped). No further edits. Absolute (http...) links are the next problem.
- No editor save messages visible in this segment.

## File states
main.go (no editor line numbers visible, so none prefixed). State at ~12:51-13:07:

```
?? (top of file: package main and earlier imports not visible; "fmt","io","log" presumably above)
import (
  ?? (earlier imports not visible)
  "net/http"
  "strings"

  "golang.org/x/net/html"
  "golang.org/x/net/html/atom"
)

func crawl(link string) {
  client := &http.Client{}

  req, err := http.NewRequest("GET", link, nil)
  if err != nil {
    log.Fatal(err)
  }
  req.Header.Add("User-Agent", `Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:159 ?? (line truncated off-screen)
  resp, err := client.Do(req)
  if err != nil {
    log.Fatal(err)
  }

  defer resp.Body.Close()
  bodyByte, err := io.ReadAll(resp.Body)

  body := string(bodyByte)
  fmt.Println("===============> ", link)

  doc, err := html.Parse(strings.NewReader(body))
  if err != nil {
    log.Fatal(err)
  }
  for n := range doc.Descendants() {
    if n.Type == html.ElementNode && n.DataAtom == atom.A {
      for _, a := range n.Attr {
        if a.Key == "href" {
          subLink := a.Val

          if !strings.HasPrefix(subLink, "#") && !strings.HasPrefix(subLink, "/") {
            crawl(link + "/" + subLink)
          }
          break
        }
      }
    }
  }
}

func main() {
  crawl("https://wingram.org")
}
```
Indent is 2 spaces.

Intermediate (11:06) inner block:
```
link = a.Val

if !strings.HasPrefix(link, "#") {
  fmt.Println(link)
}
break
```
(11:57) became `subLink := a.Val` / `if !strings.HasPrefix(subLink, "#") {` / `crawl(link + "/" + subLink)`.

## Shell & outputs
- [09:24] `lab $ go run .` -> `/`, `#wingramorg`, `#about`, `about/wingram/`, `about/father/`, `about/sites/`
- [10:59] `lab $ go run .` -> `/`, `about/wingram/`, `about/father/`, `about/sites/`
- [12:20] `lab $ go run .` -> `===============>  https://wingram.org`, then `https://wingram.org//`, `////`, `//////`, `////////` (growing; running).
- [12:52-12:58] `lab $ go run .` -> `===============>  https://wingram.org`, `.../about/wingram/`, `.../about/father/`, `.../about/father//https://www.gnu.org/philosophy/who-does-that-server-really-serve`, then repeated `/https://www.mkdocs.org/` (`//https://www.mkdocs.org/` stacking); ends `^Csignal: interrupt`.
- Browser pages: wingram.org (dark theme, green links); pkg.go.dev/strings (Go site, teal header, documentation index).

## Observations
- Workflow: run, compare output with the live site, fix. Looks things up on pkg.go.dev (strings.HasPrefix) instead of from memory.
- Filters `#fragment` links via `!strings.HasPrefix(link, "#")`; converts the print into recursive `crawl(link + "/" + subLink)` (naive URL joining, depth-first, no visited set).
- Debug marker `fmt.Println("===============> ", link)` shows the URL being fetched.
- Bugs found live: root `/` link causes infinite `//` growth; absolute links (www.gnu.org, www.mkdocs.org) get concatenated to the base URL, unbounded recursion; stopped with Ctrl-C. Fix attempt: also skip `/`-prefixed links; absolute URLs not yet handled (net/url appeared in suggestions, likely next, plus a visited set).
- Style: no comments, single crawl function, errors via log.Fatal, hard-coded User-Agent string in backticks.
