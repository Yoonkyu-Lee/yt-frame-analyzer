# Segment 4 (07:19 - 09:24) - g4SQ1LGRI00

Windows: VSCodium-like editor titled `main.go — lab` (left, with a toggleable bottom TERMINAL panel, prompt `lab $`), Firefox (right, URL bar visible), "Human" webcam overlay ignored.

## Timeline

- [07:19-07:22] Editor top pane shows end of `crawl`: `req, err := http.NewRequest("GET", link, nil)`, `if err != nil { log.Fatal(err) }`, `req.Header.Add("User-Agent", `Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:159...` (line cut off at pane edge), `resp, err := client.Do(req)`, err check, `defer resp.Body.Close()`, `body, err := io.ReadAll(resp.Body)`. Terminal panel is scrolled in a huge dump of the DuckDuckGo-lite HTML (`<td>`, `<a rel="nofollow" href="//duckduckgo.com/l/?uddg=https%3A%2F%2Fopen.spotify.com%2Ftrack%2F0ENSn4fwAbCGeFGVUbXEU3&amp;rut=f761f5b8504f4f8824b6bb8b40adf4507382ac4f6c4be197f75959d456352a3f" class='result-link'>Hello - song and lyrics by Adele | Spotify</a>`, `<!-- Only show abstract separately if there's a click URL (not EOF) -->`, `<td class='result-snippet'> From the single <b>Hello</b> October 23, 2015 ℗ 2015 XL Recordings Ltd., under exclusive license to Columbia Records, a Division of Sony Music Entertainment`). He selects/highlights parts of this raw HTML with the mouse (the `<td><a ...>` block at 07:22) i.e. showing the links are inside `<a href>` tags. Firefox behind shows `lite.duckduckgo.com/lite/?q=hello+` results (Adele "Hello").
- [07:23] Terminal panel closed. Editor shows end of crawl, then `fmt.Println(string(body))` and `func main() { crawl("https://lite.duckduckgo.com/lite/?q=hello+") }`; cursor just after `hello+` inside the string.
- [07:24] Editor scrolled to top of `crawl`. Code now has begun refactor: `body` line replaced by
  `bodyByte, err := io.ReadAll(resp.Body)`, blank, `body := string(bodyByte)` (`body` red-squiggled: declared and not used), blank, comment `// get link in html`. `fmt.Println(string(body))` line removed.
- [07:49] Focus switches to Firefox: DuckDuckGo Lite results page (Hello (Adele song) - Wikipedia, Adele - Hello (Official Music Video) - YouTube, ... HELLO! magazine, Home | HOME hello.hanu.edu.vn, Hello - Wikipedia). URL bar focused.
- [07:50] URL bar dropdown with history suggestions (Hayyoth hayyoth.org, Google Gemini, Telegram Web, Hayyoth - YouTube, Google Translate).
- [07:51] Types `golang.org/` in URL bar; Firefox Suggest list (includes a Vietnamese-titled Gemini chat about saving files from a URL in Golang, "html package - golang.org/x/net/html - Go Packages", etc.).
- [07:52] URL bar: `golang x ` -> suggestions incl. `html package - golang.org/x/net/html`, websocket package.
- [07:53] URL bar: `golang x html` ("Search with Duck Lite").
- [07:54] DuckDuckGo Lite results for `lite.duckduckgo.com/lite/?q=golang+x+html`: #2 "html package - golang.org/x/net/html - Go Packages" (pkg.go.dev/golang.org/x/net/html), #3 "net module - golang.org/x/net - Go Packages", #4 "How to Parse HTML in Golang: 2026 Tutorial - ZenRows", #5 "Parsing HTML in Go - ZetCode". Cursor hovers #2/#3.
- [07:57] pkg.go.dev/golang.org/x/net/html: "html package", Version v0.62.0 Latest, Published: Oct 27, 2026, License BSD-3-Clause, Imports: 12, Imported by: 16,083; Details; Repository cs.opensource.google/go/x/net.
- [07:58] Scrolled to Documentation > Overview: "Package html implements an HTML5-compliant tokenizer and parser." "Tokenization is done by creating a Tokenizer for an io.Reader r. It is the caller's responsibility to ensure that r provides UTF-8 encoded HTML." code `z := html.NewTokenizer(r)`; "Given a Tokenizer z, the HTML is tokenized by repeatedly calling z.Next() ..." loop `for { tt := z.Next(); if tt == html.ErrorToken { // ... return ... } // Process the current token. }`.
- [07:59] Same, scrolled slightly (Next/Raw/Token discussion: "There are two APIs for retrieving the current token... `Next {Raw} [ Token | Text | TagN...`").
- [08:00-08:05] Opens the page "outline" dropdown (Outline: Documentation, Source Files, Directories; Documentation: Overview, Index, Examples, Constants, Variables, Functions, Types; Examples: Parse; Functions: EscapeString(s), Render(w, n), UnescapeString(s); Types: Attribute, Node, NodeType, ParseOption, Token, TokenType, Tokenizer; type Node: Parse(r), ParseFragment(r, context), ParseFragmentWithOptions(r, context, opts), ParseWithOptions(r, opts)...). Hovers Index -> Variables -> Overview -> Examples, clicks "Examples" (08:04).
- [08:04-08:05] Examples section: "Parse" example link; Constants "This section is empty."; Variables `var ErrBufferExceeded = errors.New("max buffer exceeded")`; Functions `func EscapeString(s string) string`.
- [08:07] Firefox window dragged to left/center (now overlapping the editor, x~290-1040).
- [08:08-08:09] Scrolling; "Variables ¶" hover. [08:10] clicks "Parse" example link.
- [08:11] Example expanded (URL `pkg.go.dev/golang.org/x/net/html#example-...`). Visible code:
```
package main

import (
    "fmt"
    "log"
    "strings"

    "golang.org/x/net/html"
    "golang.org/x/net/html/atom"
)

func main() {
    s := `<p>Links:</p><ul><li><a href="foo">Foo</a><li><a href="/bar/baz">BarBaz</a></ul>`   (line cut off at right: `<li><a href="`... )
    doc, err := html.Parse(strings.NewReader(s))
    if err != nil {
        log.Fatal(err)
    }
    for n := range doc.Descendants() {
        if n.Type == html.ElementNode && n.DataAtom == atom.A {
            for _, a := range n.Attr {
                if a.Key == "href" {
                    fmt.Println(a.Val)
                    break
                }
            }
        }
    }

}
```
  with `Output: foo /bar/baz` (seen later at 08:30). He reads this carefully across 08:12-08:16, scrolling the code box and the page (08:16 shows the Parse docs text: "...Parse will reject HTML that is nested deeper than 512 elements. The input is assumed to be UTF-8 encoded.").
- [08:20] He selects the two import lines `"golang.org/x/net/html"` and `"golang.org/x/net/html/atom"` in the example (copy). Firefox window moved slightly right (x~670-1420).
- [08:21] Switches to editor. `main.go` top: `package main`, `import ( "fmt" "io" "log" "net/http" )` (all imports squiggled/underlined, `fmt` grayed unused), then `func crawl(link string) {...`. Cursor at `if err != nil {` line.
- [08:22] A new blank line added inside the import block after `"net/http"` (before `)`).
- [08:23] Pasted: blank line then
  `"golang.org/x/net/html"` and `"golang.org/x/net/html/atom"` in the import block (red squiggles; module not yet installed).
- [08:24-08:25] Terminal panel reopened: still shows old HTML dump (the highlighted `<td><a ...>` block), then cleared; at 08:25 terminal shows `lab $ go m` (typing a command, likely `go mod ...`; rest not visible).
- [08:30-08:31] Back to Firefox on the Parse example (Output: `foo`, `/bar/baz`). Editor imports now show `"golang.org/x/net/html"` / `.../atom` in lighter (resolved/greyed) squiggle color - suggests `go mod tidy`/`go get` ran in between (not captured).
- [08:37] Selects (copies) the example body in Firefox from `doc, err := html.Parse(strings.NewReader(s))` through the closing `}` of the `for n := range doc.Descendants()` loop.
- [08:45] Editor: scrolled so imports at top (`"fmt" "io" "log" "net/http"`, blank, `"golang.org/x/net/html"`, `"golang.org/x/net/html/atom"` `)`), crawl func; cursor on blank line after `client := &http.Client{}`.
- [08:46] Scrolled down: end of crawl shows `body := string(bodyByte)`, blank, `// get link in html`, `}`, blank, `func main() { crawl("https://lite.duckduckgo.com/lite/?q=hello+") }`.
- [08:48] Pasted the example block replacing `// get link in html`: 
  `doc, err := html.Parse(strings.NewReader(s))` (squiggle under `strings.NewReader` and `s`), then the err check and the `for n := range doc.Descendants()` loop, indented with 2-space tabs (editor reformatted). Cursor after closing `}` of for loop. Also: `body` still greyed (unused).
- [08:49] Same, scrolled up a bit.
- [08:55] Fixed arg: `doc, err := html.Parse(strings.NewReader(body))`; `body :=` no longer squiggled/unused. (`s` -> `body`.)
- [08:59-09:00] Cursor on `break` (selection highlight on `break` and matching `for _, a := range n.Attr`); inspecting the loop. Mouse near `fmt.Println(a.Val)`.
- [09:04] Highlights line `if a.Key == "href" {`.
- [09:19] Terminal opened: `lab $ go run .` Output:
```
/
#wingramorg
#about
about/wingram/
about/father/
about/sites/
lab $
```
  (output highlighted by mouse). Note: the crawl URL in the main() visible earlier was lite.duckduckgo.com, but the output corresponds to wingram.org - he must have changed `main()`'s URL to the wingram.org site between 09:04 and 09:19 (not captured in frames).
- [09:22] Firefox in front again; terminal output still visible (same). Example highlighted in Firefox.
- [09:23] Firefox URL bar: types `wingram.org` ("Looking up wingram.org...").
- [09:24] Page loaded: dark theme "Wingram.Org" site, header with small green figure logo, title "Wingram.Org", subtitle "Wingram on the way we find the Truth.", section "About" with links: "Wingram Philosophy", "Wingram Father", "Sites - Contact"; footer text partly hidden: "Don't forget b...", "You can forget this site, bu...". (Matches the printed hrefs: `/`, `#wingramorg`, `#about`, `about/wingram/`, `about/father/`, `about/sites/`.)

## File states

main.go as of ~09:04 (end of visible editing). Editor line numbers are NOT displayed in this editor, so none shown; lines in order:

```
package main

import (
  "fmt"
  "io"
  "log"
  "net/http"

  "golang.org/x/net/html"
  "golang.org/x/net/html/atom"
)
?? (imports: "strings" not visible in import list at 08:45-08:49 although strings.NewReader is used; at 08:23 list shows fmt, io, log, net/http + the two golang.org imports; "strings" apparently missing -> compile error likely, not seen)

func crawl(link string) {
  client := &http.Client{}

  req, err := http.NewRequest("GET", link, nil)
  if err != nil {
    log.Fatal(err)
  }
  req.Header.Add("User-Agent", `Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:159⟨?⟩ ...`)   // rest of line cut off, closed by backtick + ")" presumably
  resp, err := client.Do(req)
  if err != nil {
    log.Fatal(err)
  }

  defer resp.Body.Close()
  bodyByte, err := io.ReadAll(resp.Body)

  body := string(bodyByte)

  doc, err := html.Parse(strings.NewReader(body))
  if err != nil {
    log.Fatal(err)
  }
  for n := range doc.Descendants() {
    if n.Type == html.ElementNode && n.DataAtom == atom.A {
      for _, a := range n.Attr {
        if a.Key == "href" {
          fmt.Println(a.Val)
          break
        }
      }
    }
  }
}

func main() {
  crawl("https://lite.duckduckgo.com/lite/?q=hello+")   // as of 09:04; later run printed wingram.org links => URL changed afterwards (not seen)
}
```

Earlier intermediate (07:23): end of crawl was `body, err := io.ReadAll(resp.Body)` / blank / `fmt.Println(string(body))` / blank / `}`. At 07:24 changed to `bodyByte, err := io.ReadAll(resp.Body)`, `body := string(bodyByte)`, `// get link in html`.

## Shell & outputs

- [07:19-07:22] terminal (scrollback of previous `go run`): raw lite.duckduckgo.com HTML for query `hello+` (see Timeline).
- [08:25] `lab $ go m` (incomplete; likely `go mod init`/`go mod tidy`, unconfirmed).
- [09:19] `lab $ go run .` -> output:
```
/
#wingramorg
#about
about/wingram/
about/father/
about/sites/
```
- Browser: DuckDuckGo Lite `?q=golang+x+html`; pkg.go.dev/golang.org/x/net/html (v0.62.0, published Oct 27, 2026), Overview, Examples > Parse example (Output: foo, /bar/baz); wingram.org page (described at 09:24).

## Observations

- Build order: raw GET -> dump HTML to terminal -> realize links are in `<a href>` -> search for an HTML parser -> use x/net/html. He rewrites the `body` handling (`bodyByte`/`body := string(bodyByte)`) and leaves a `// get link in html` TODO comment before researching.
- Research habit: searches the web ("golang x html" via DuckDuckGo Lite in Firefox), opens pkg.go.dev, reads Overview (tokenizer API), then switches to the Examples > Parse example and copy-pastes its imports and the Parse/Descendants loop straight into his code, then adapts it (`s` -> `body`). Uses the new `doc.Descendants()` iterator (range-over-func) form shown in the current docs.
- Test target switches from DuckDuckGo Lite (search results) to wingram.org, a tiny static site whose links are easy to read; output shows relative links and fragment links (`#about`), foreshadowing the need to resolve relative URLs and skip anchors.
- Style: 2-space indentation in the editor, Go imports grouped stdlib then third-party, minimal naming, immediate `log.Fatal(err)` error handling.
