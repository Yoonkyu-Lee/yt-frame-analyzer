# Segment 2 (02:39 - 05:15) - g4SQ1LGRI00

Setup: macOS-style windows. Left: editor window titled `main.go — lab` (sidebar "LAB" with `go.mod`, `main.go`). Right: Firefox-like browser on `pkg.go.dev/net/http`. Bottom right: "Human" webcam overlay (ignored). No terminal visible in this segment. No editor save messages visible. The editor has no line-number gutter.

## Timeline

- [02:39-03:08] Browser shows the pkg.go.dev `net/http` page; a native dropdown (section jump menu, currently checked item "Overview") is open and the mouse hovers/scrolls through entries. Entries seen: Source Files, Directories; Documentation: Overview (checked), Index, Examples, Constants, Variables, Functions, Types; Examples: CrossOriginProtection, FileServer, FileServer (DotFileHiding), FileServer (StripPrefix), Get, Handle, HandleFunc, Hijacker, ListenAndServe, ListenAndServeTLS, NotFoundHandler, Protocols (Http1), Protocols (Http1or2), ResponseWriter (Trailers), ServeMux.Handle, Server.Shutdown, StripPrefix; Functions: CanonicalHeaderKey(s), DetectContentType(data), Error(w, error, code), Handle(pattern, handler), HandleFunc(pattern, handler), ListenAndServe(addr, handler), ListenAndServeTLS(addr, certFile, keyFile, handler), MaxBytesReader(w, r, n), NotFound(w, r), ParseHTTPVersion(vers), ParseTime(text), ProxyFromEnvironment(req), ProxyURL(fixedURL), Redirect(w, r, url, code), Serve(l, handler), ServeContent(w, req, name, modtime, content), ServeFile(w, r, name), ServeFileFS(w, r, fsys, name), ServeTLS(l, handler, certFile, keyFile), SetCookie(w, cookie), StatusText(code); Types: type Client, type ClientConn, type CloseNotifier, type ConnState... Hover highlight wanders (Handle, Protocols (Http1), ResponseWriter, ListenAndServeTLS, NotFound, ServeContent, DetectContentType, ListenAndServe, ServeMux.Handle, Hijacker, FileServer (DotFileHiding), Handle, ListenAndServe, FileServer (StripPrefix)). No obvious selection; menu closed by 03:17.
- [03:17] Page `pkg.go.dev/net/http` ("http" package, "standard library"), dropdown "Overview". Scrolled to end of the Get example (`// handle error`, `}`, `defer resp.Body.Close()`, `body, err := io.ReadAll(resp.Body)`, `// ...`, gray-highlighted), then heading "Clients and Transports": "For control over HTTP client headers, redirect policy, and other settings, create a Client:".
- [03:18-03:21] Scrolls to the Client sample:
  ```
  client := &http.Client{
      CheckRedirect: redirectPolicyFunc,
  }

  resp, err := client.Get("http://example.com")
  // ...

  req, err := http.NewRequest("GET", "http://example.com", nil)
  // ...
  req.Header.Add("If-None-Match", `W/"wyzzy"`)
  resp, err := client.Do(req)
  // ...
  ```
  Next paragraph: "For control over proxies, TLS configuration, keep-alives, compression, and other settings, create a Transport:" then `tr := &http.Transport{ MaxIdleConns: 10, IdleConnTimeout: 30 * t...(hidden), DisableCompression: true, } client := &http.Client{Transpo...(hidden) resp, err := client.Get("https...(hidden)`.
- [03:29] Switches to editor (cursor after the `)` closing the import block). Existing file from segment 1: package main; imports fmt, io, net/http; `func main()` with `// but without user-agent, some sites will block`, `resp, err := http.Get("https://www.nokia.com/bell-labs/about/dennis-m-ritchie/")` (full URL visible at 04:29), `if err != nil { // handle error }`, defer Close, ReadAll, Println. Browser meanwhile has the NewRequest line highlighted.
- [03:37] Adds a new function above main (cursor in empty body):
  ```
  func crawl(link string) {

  }
  ```
- [03:39-03:45] Back in browser: `req, err := http.NewRequest("GET", "http://example.com", nil)` line selected (copy target). Editor shows `crawl` squiggle (unused).
- [03:46] Pastes the Client sample into `crawl` body (verbatim, with `// ...`; unformatted). Errors appear: `redirectPolicyFunc` undefined, `resp, err :=` problems; tab badge `main.go 4`.
- [03:49] Badge 5; file scrolled a bit.
- [03:51-03:57] Prunes: `client := &http.Client{` / `CheckRedirect: redirectPolicyFunc,` / `}` becomes `client := &http.Client{` + `}` (03:51, 03:52) then `client := &http.Client{}` on one line (03:56) with a blank line after; badge 3.
- [04:06] Edits `resp, err := client.Get("http://example.com")` into `resp, err := client.Get(link)`; removes the `// ...` after it; starts error check, typing `lo` inside:
  ```
  resp, err := client.Get(link)
  if err != nil {
    lo
  }
  ```
- [04:11] `log.Fatal(err)` typed (badge 2).
- [04:12] Cursor on `req` in `req, err := http.NewRequest(...)` (occurrences highlighted).
- [04:23] Blank cursor line created after `resp, err := client.Get(link)` (badge 4; at that moment the if block was cut).
- [04:26-04:27] Selects the `if err != nil { log.Fatal(err) }` block (copy), scrolled so `func crawl` is at top.
- [04:29] Editor scrolled horizontally left (sidebar hidden); full main URL visible: `resp, err := http.Get("https://www.nokia.com/bell-labs/about/dennis-m-ritchie/")`.
- [04:36-04:37] Pastes the same block after `NewRequest`:
  ```
  req, err := http.NewRequest("GET", "http://example.com", nil)
  if err != nil {
    log.Fatal(err)
  }
  req.Header.Add("If-None-Match", `W/"wyzzy"`)
  resp, err := client.Do(req)
  ```
  (the `// ...` between NewRequest and Header.Add is gone).
- [04:47-04:49] Scrolled to top; imports now include `"log"`:
  ```
  import (
    "fmt"
    "io"
    "log"
    "net/http"
  )
  ```
- [04:54] Changes `resp, err := client.Do(req)` to `resp, err = client.Do(req)` (`:=` -> `=`).
- [04:56] Adds blank lines after the Do call inside crawl (cursor on empty line).
- [05:04] Deletes the earlier `resp, err := client.Get(link)` plus its `if err != nil {log.Fatal(err)}` block (cursor on blank line below `client := &http.Client{}`); `resp` in `resp, err = client.Do(req)` now flagged (undefined), blank lines remain after Do.
- [05:05] Cursor at end of `client := &http.Client{}`; `}` bracket highlight.
- [05:09] Switches to browser: still net/http Overview Client sample, text selected gray; editor shows `resp, err := client.Do(req)` again? At 05:09 the editor line reads `resp, err := client.Do(req)` with `:=` (partly occluded; at 05:05 it reads `resp, err = client.Do(req)`) - uncertain `⟨?⟩`.
- [05:10] New blank browser tab, address bar focused "Search or enter address" (search engine icon dropdown).
- [05:11] Navigates to `hayyoth.org` (author's public site): serif page "Hayyoth", link "- White Paper -", tagline "Understand the Game. Don't Play It Blindly. The Art of Quitting.", a Year / Everyone / I / Outcome table, "Videos" heading with "Opinions on AI" link. (Biography table text not transcribed.)
- [05:12] Opens DevTools (Network tab selected next to Inspector; "Disable Cache" checked; message "Perform a request or Reload the page to see detailed network activity." / "No requests").
- [05:15] After reload: 2 requests (`GET /` doc html; `GET favicon.ico` img), "2 requests 19.26 kB / 6.89 kB transferred". Selected request `GET https://hayyoth.org/`: Status 200, Version HTTP/3, Transferred 2.17 kB (3.85 kB size), Request Priority Highest, DNS Resolution System. Response Headers (611 B): `alt-svc: h3=":443"; ma=86400`, `cache-control: public, max-age=0, must-revalidate` (truncated), `cf-cache-status: HIT`, `cf-ray: ...-HKG`, `content-encoding: zstd`, `content-type: text/html`, `date: Fri, 06 Nov 2026 16:36:21 GMT`. Demonstrating a raw HTTP response (headers) for the crawler context.

## File states

### main.go at end of segment (~05:09); no line-number gutter, so no numeric prefixes
```
package main

import (
  "fmt"
  "io"
  "log"
  "net/http"
)

func crawl(link string) {
  client := &http.Client{}

  req, err := http.NewRequest("GET", "http://example.com", nil)
  if err != nil {
    log.Fatal(err)
  }
  req.Header.Add("If-None-Match", `W/"wyzzy"`)
  resp, err = client.Do(req)    // ⟨?⟩ `=` at 05:05, possibly `:=` at 05:09


}

func main() {
  // but without user-agent, some sites will block
  resp, err := http.Get("https://www.nokia.com/bell-labs/about/dennis-m-ritchie/")
  if err != nil {
    // handle error
  }
  defer resp.Body.Close()
  body, err := io.ReadAll(resp.Body)

  fmt.Println(string(body))
}
```
Indent is 2 spaces. `go.mod` visible in sidebar, content not shown.

### Intermediate (04:37, before the Get block was removed)
```
func crawl(link string) {
  client := &http.Client{}

  resp, err := client.Get(link)
  if err != nil {
    log.Fatal(err)
  }

  req, err := http.NewRequest("GET", "http://example.com", nil)
  if err != nil {
    log.Fatal(err)
  }
  req.Header.Add("If-None-Match", `W/"wyzzy"`)
  resp, err := client.Do(req)
}
```

## Shell & outputs
No terminal in this segment. Browser-rendered content only: pkg.go.dev net/http Overview ("Clients and Transports" section, quoted above) at 03:17-03:21 and 05:09; hayyoth.org home page at 05:11; DevTools Network panel for https://hayyoth.org/ at 05:12-05:15 (details above).

## Observations
- Builds from the official net/http docs: copies the Client / NewRequest / Header.Add / Do sample into a new `crawl(link string)` function, then prunes by hand (drops CheckRedirect, uses default `&http.Client{}`, substitutes `link`).
- Pivots from `client.Get` to the explicit NewRequest + `client.Do` path, consistent with the main-file comment "without user-agent, some sites will block" (wants to set request headers such as User-Agent).
- Error handling style in crawl: `log.Fatal(err)` after each call (main still has the placeholder `// handle error`); copy/paste of the if-block rather than retyping.
- Fixed `:=` redeclaration by switching to `=`, then removed the earlier Get, leaving `resp` undeclared (likely fixed next).
- Shows his own site in the browser DevTools Network tab to illustrate what an HTTP response/headers look like (200, HTTP/3, zstd) as motivation for the crawler/wget.
- Editor has no line numbers; tab badge shows problem count (4,5,3,2,4,3 over the segment).
