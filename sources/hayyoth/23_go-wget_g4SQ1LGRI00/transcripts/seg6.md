# Segment 6 (13:07 - 16:05) - g4SQ1LGRI00

Editor: VSCodium-like window titled "main.go — lab" (no line-number gutter visible in any frame, so file states below carry no editor line numbers). Firefox window behind it (pkg.go.dev). Webcam overlay ignored.

## Timeline
- [13:07] main.go in editor, browser on `pkg.go.dev/strings#HasPrefix` (HasPrefix: "reports whether the string s begins with prefix"; HasSuffix below). Editor scrolled horizontally. Inside crawl's link loop he is editing the filter condition: `if !strings.HasPrefix(subLink, "#") && !strings.HasPrefix(subLink, "/") {` (cursor at the second `"/"`).
- [13:12] Condition now wrapped onto two lines (an extra duplicate clause being typed):
  `if !strings.HasPrefix(subLink, "#") && !strings.HasPrefix(subLink, "/")` newline `&& !strings.HasPrefix(subLink, "/"){` (then `crawl(link + "/" + subLink)`, `}`, `break`).
- [13:16-13:17] Same state; cursor at start of the 2nd line (`&& ...`).
- [13:31] Reformatted into three lines:
  `if !strings.HasPrefix(subLink, "#") &&` / `!strings.HasPrefix(subLink, "/") &&` / `!strings.HasPrefix(subLink, "/"){` (third still a duplicate "/" being edited).
- [13:39] Terminal panel opened at bottom of editor: `lab $ ` prompt (empty).
- [13:47] Third clause changed to `"https"`: `!strings.HasPrefix(subLink, "https") {` (so skips #, /, and https links).
- [13:49] New line typed after the "===> " println: `fmt.` (incomplete, red squiggles).
- [13:54] Completed to `fmt.Println(body)`. Terminal ran `go run .`; output shown (see Shell). Prompt line then shows `lab $ ok` (text "ok" at prompt, not a command result).
- [13:57] Terminal scrolled: the full HTML body of crawled page printed (cloudflare beacon script, closing </body></html>).
- [14:00-14:15] Terminal maximized (panel fills editor area); he scrolls through the dumped HTML of wingram.org pages (about/sites, about/father). Frames show page source: head meta tags, bootstrap/fontawesome css links, navbar, sidebar TOC, "Wingram Father" article text, keyboard-shortcuts modal, footer. At 14:13 a line `===============>  https://wingram.org/about/father/` is text-selected (highlighted). Purpose: showing that bodies are printed per URL with separator lines.
- [14:19] Terminal closed/editor restored, scrolled to top: import block and start of crawl visible (see File states).
- [14:21] Typing a comment line above `func crawl`: `// s` (incomplete).
- [14:45] A new function skeleton inserted above crawl:
  `func save(link string, content []byte) {` / empty body / `}`.
- [14:58] save body now `fmt.Println("===> save ", link)` (partially visible at top, cursor at end).
- [14:59] Scrolled; fmt.Println(body) still present in crawl.
- [15:02] In crawl: `fmt.Println("===============> ", link)` and `fmt.Println(body)` lines deleted; blank line with cursor after `body := string(bodyByte)`.
- [15:12] Line added: `save(link, bodyByte)` after `body := string(bodyByte)`.
- [15:13] Same state (cursor/pointer moved).
- [15:16] Terminal maximized with `lab $ go run .` typed (no output visible).
- [15:23] Back to editor; `content` parameter in save underlined with blue squiggle (unused hint); `bodyByte` highlighted (occurrences).
- [15:24] Inside save, new comment line being typed: `// ` (after the println).
- [15:37] Comment completed: `// to create file , we need remove https from link`. Firefox brought to front showing pkg.go.dev/strings HasPrefix/HasSuffix/Index docs ("Index returns the index of the first instance of substr..."), dropdown "HasSuffix(s, suffix)".
- [15:40] Firefox new tab: empty "Search or enter address" (DuckDuckGo engine icon).
- [15:45] Typed `golang ` into address bar; Firefox Suggest list (history entries).
- [15:46] Typed `golang url `.
- [15:49] Query changed to `golang urlfile`; page "Waiting for lite.duckduckgo.com...".
- [15:50] Results at `lite.duckduckgo.com/lite/?q=golang+urlfile`: 1. "url package - net/url - Go Packages" (pkg.go.dev/net/url, JoinPath text); 2. Stack Overflow "Go routine to download from a URL to a file" (snippet: `tokens := strings.Split(url, "/") fileName := tokens[len(tokens)-1]`); 3. Stack Overflow "go - http.Request: get file name from url"; 4. "[Golang] Download File From U..." (siongui.github.io); 5. "GitHub - hashicorp/go-getter: ... from a ...".
- [15:53] Address bar focused with URL selected; dropdown shows top sites (public: Hayyoth site, Google Gemini, Telegram Web, YouTube channel, Google Translate).
- [15:54-15:56] Typed `golang ` then `golang f` (suggestions list).
- [15:58] Query `golang filepath` typed.
- [16:00] Results for `lite.duckduckgo.com/lite/?q=golang+filepat...`: 1. "filepath package - path/filepath - Go Packages" (pkg.go.dev/path/filepath); 2. golangdoc.github.io filepath 1.12; 3. "Handling File Paths in Go - ZetCode"; 4. "Go by Example: File Paths"; 5. "filepath Package in Golang - G...".
- [16:02] Opened `pkg.go.dev/path/filepath` (loading): filepath package, Version go1.27.3 Latest, Published Oct 20, 2026, BSD-3-Clause, Imports 10, Imported by 813,423.
- [16:03-16:05] Scrolling doc: Overview "Package filepath implements utility routines for manipulating filename paths in a way compatible with the target operating system-defined file paths. The filepath package uses either forward slashes or backslashes, depending on the operating system. To process paths such as URLs that always use forward slashes regardless of the operating system, see the path package." Index list: Constants, Variables, func Abs, Base, Clean, Dir, EvalSymlinks, Ext, FromSlash, Glob, `HasPrefix(p, prefix string) bool` (DEPRECATED), IsAbs, IsLocal, Join(elem ...string) string, Localize, Match, Rel, Split(path string) (dir, file string), SplitList, ToSlash, VolumeName.
- No editor save message visible in this segment.

## File states
Window title: main.go — lab. Only partial views; state as of ~15:37 (end). Line numbers not shown by editor.

```
?? (package main line and anything above import not visible)
import (
  "fmt"
  "io"
  "log"
  "net/http"
  "strings"

  "golang.org/x/net/html"
  "golang.org/x/net/html/atom"
)

func save(link string, content []byte) {
  fmt.Println("===> save ", link)
  // to create file , we need remove https from link
}

func crawl(link string) {
  client := &http.Client{}

  req, err := http.NewRequest("GET", link, nil)
  if err != nil {
    log.Fatal(err)
  }
  req.Header.Add("User-Agent", `Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:159 ?? (rest of UA string cut off)
  resp, err := client.Do(req)
  if err != nil {
    log.Fatal(err)
  }

  defer resp.Body.Close()
  bodyByte, err := io.ReadAll(resp.Body)

  body := string(bodyByte)
  save(link, bodyByte)

  doc, err := html.Parse(strings.NewReader(body))
  if err != nil {
    log.Fatal(err)
  }
  for n := range doc.Descendants() {
    if n.Type == html.ElementNode && n.DataAtom == atom.A {
      for _, a := range n.Attr {
        if a.Key == "href" {
          subLink := a.Val

          if !strings.HasPrefix(subLink, "#") &&
            !strings.HasPrefix(subLink, "/") &&
            !strings.HasPrefix(subLink, "https") {
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
  ?? (closing brace and anything below not visible)
```
(Indentation 2 spaces with nested indent guides; the multi-line condition's continuation lines are indented one level deeper than `if`.)

Intermediate version at ~13:12 (before change to https): condition `if !strings.HasPrefix(subLink, "#") && !strings.HasPrefix(subLink, "/")` / `&& !strings.HasPrefix(subLink, "/"){`, and a debug line `fmt.Println("===============> ", link)` after `body := string(bodyByte)` (no fmt.Println(body) yet).

Intermediate at 14:19: crawl contained both `fmt.Println("===============> ", link)` and `fmt.Println(body)` after `body := string(bodyByte)`; no save() func yet.

## Shell & outputs
- [13:39] terminal opened: `lab $ `
- [13:54] `lab $ go run .` output:
```
===============>  https://wingram.org
===============>  https://wingram.org/about/wingram/
===============>  https://wingram.org/about/father/
===============>  https://wingram.org/about/sites/
lab $ ok
```
  (4 URLs crawled; "ok" is text at the prompt.)
- [13:57-14:15] Same `go run .` output continued with full HTML bodies printed (with `fmt.Println(body)`): wingram.org pages are an MkDocs/bootstrap site; each page ends with a cloudflareinsights beacon `<script type="module" ...></script></body></html>`. Visible content: Sites page (h1 "Sites", "You can follow Wingram on these platform", list of Web/Telegram/Github/Youtube/X/Mail links); Father page (h1 "Wingram Father", "I'm just one person who wants to make something good for the world", sections "I had a strange dream", "How I help you", "I quit anything", "What i do today?"). Page titles: "Sites - Wingram.Org", "Wingram Father - Wingram.Org". (Tokens and mail hashes in the dump not transcribed.)
- [15:16] `lab $ go run .` typed in maximized terminal; no output visible before switching back to editor.
- Note: `crawl(link + "/" + subLink)` builds URLs like https://wingram.org/about/...; relative hrefs are followed; "#...", "/..." and "https..." links are skipped.

## Observations
- Build order: crawl -> filter links (skip `#`, `/`, `https`) -> debug print URL and body -> refactor into `save(link, content)` stub that currently just prints "===> save " + link; next goal is writing files named from the URL.
- Comment in save states intent: "to create file , we need remove https from link". He searches how to turn a URL into a filename (queries: golang url, urlfile, filepath) consulting pkg.go.dev (strings HasPrefix/HasSuffix/Index, net/url, path/filepath), Stack Overflow snippets (strings.Split(url,"/") last token).
- Style: iterative, print-debug driven (print then run); duplicate condition typed by mistake and fixed in place (repeated `"/"` became `"https"`); relies on docs via DuckDuckGo Lite and pkg.go.dev.
- Uses a fake browser User-Agent on requests; uses the `doc.Descendants()` iterator from x/net/html (newer API).
- Known weaknesses visible: `break` after first href attr, no visited-set, relative-link join via `link + "/" + subLink`.
