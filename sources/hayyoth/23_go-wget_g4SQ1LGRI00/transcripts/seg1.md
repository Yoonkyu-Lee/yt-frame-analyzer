# Segment 1 (0:00 - 2:39) - g4SQ1LGRI00 - Hayyoth "build your own wget (Web Crawler) in Go by hand"

Setup: editor window titled `main.go — lab` (VSCodium-like, light theme, sidebar "LAB" with `go.mod`, `main.go`), integrated TERMINAL panel (prompt `lab $ `), Firefox window on the right (search engine icon = DuckDuckGo), webcam "Human" overlay ignored. Editor has no visible line numbers or status bar in these frames (no save messages visible).

## Timeline
- [00:00] Empty editor (no file open, coral logo watermark) + terminal panel at bottom with prompt `lab $ `. Firefox behind, blank, address bar "Search or enter address".
- [00:15] `main.go` is open: `package main` / `func main() {` / empty line / `}`. Sidebar: go.mod, main.go (selected). In Firefox he types `pkg.go.dev/` in the address bar; autocomplete shows "Go Packages - pkg.go.dev", Firefox Suggest items (net/http, std, html, net/url, io/fs, path/filepath, ...).
- [00:17] Address bar `pkg.go.dev/net/` (suggestions: net/http, net/url).
- [00:18] Address `pkg.go.dev/net/http`, page loading ("Waiting for pkg.go.dev...").
- [00:19] pkg.go.dev page for `http` (package, standard library). Version go1.27.3 (Latest), Published Oct 20, 2026, License BSD-3-Clause, Imports 44, Imported by 1,776,917. Details checkmarks; Repository cs.opensource.google/go/go; Documentation heading begins.
- [00:21] Scrolled: "Documentation", Rendered for linux/amd64. "Overview: Clients and Transports, Servers, HTTP/2". "Package http provides HTTP client and server implementations."
- [00:22] Overview text: "Get, Head, Post, and PostForm make HTTP (or HTTPS) requests:" with code
  ```
  resp, err := http.Get("http://example.com/")
  ...
  resp, err := http.Post("http://example.com/upload", "image/jpeg", ...
  ...
  resp, err := http.PostForm("http://example.com/form",
      url.Values{"key": {"Value"}, "id": {"123"}})
  ```
  then "The caller must close the response body when finished with it:" + start of next snippet.
- [00:24] Snippet visible in full (docs):
  ```
  resp, err := http.Get("http://example.com/")
  if err != nil {
      // handle error
  }
  defer resp.Body.Close()
  body, err := io.ReadAll(resp.Body)
  // ...
  ```
  and "Clients and Transports: For control over HTTP client headers, redirect policy, and other settings, create a Client:" `client := &http.Client{ CheckRedirect: redirectPolicyFunc, }`; `resp, err := client.Get("http:...`.
- [00:29] Editor in front, main.go still the empty skeleton, cursor on the blank line inside main() (indented one tab level). Docs snippet text is highlighted in Firefox behind (he copied it).
- [01:04] main.go now has the pasted docs snippet adapted, with imports `fmt`, `io`, `net/http` (red underlines = unused-import / unresolved). Terminal shows `lab $ go run .` output: HTML of example.com ("Example Domain ... This domain is for use in documentation examples without needing permission. ...").
- [01:11] URL changed to `https://wingram.org`; terminal shows tail of its HTML (cloudflareinsights beacon script, `<!-- MkDocs version : 1.6.1 Build Date UTC : 2025-02-08 ... -->`). New line `body, err := io.ReadAll(resp.Body)` and `fmt.Println(string(body))` present.
- [01:19] Terminal showing more HTML output of wingram.org (keyboard-shortcut table: "Open this help", "Next page", "Previous page", "Search").
- [01:21] Same state (the screenshot is of the cached/scrolling terminal) - URL still wingram.org.
- [01:24] Typing a comment as first line of main(): `// ` (cursor after it).
- [01:33] Comment line now reads `// but without user-agent, some sit...` (partially hidden by Firefox). Firefox in front with empty address bar focused ("Search or enter address").
- [01:35] Firefox shows DuckDuckGo Lite search results page, URL `lite.duckduckgo.com/lite/?q=hello` (results: Hello (Adele song) Wikipedia, Adele - Hello YouTube, HELLO! magazine, hello.hanu.edu.vn, Hello - Wikipedia ...).
- [01:36] URL bar selected; dropdown of top sites (Hayyoth hayyoth.org, Google Gemini, Telegram Web, Hayyoth YouTube, Google Translate).
- [01:38] Editor front. Full comment: `// but without user-agent, some sites will block`. URL on the Get line is still `https://wingram.org`.
- [01:46] URL in code changed to `https://lite.duckduckgo.com/lite/?q=hello`; terminal shows end of output (`...&amp;iadb=0"/>` `</body>` `</html>`), editor horizontally scrolled right.
- [01:48 - 02:00] Terminal panel maximized (fullscreen toggle); he scrolls up through the DuckDuckGo Lite response which is a bot challenge page: `<div class="anomaly-modal__title">Unfortunately, bots use DuckDuckGo too.</div>`, "Please complete the following challenge to confirm this search was made by a human.", "Select all squares containing a duck:", grid of 9 image checkboxes (data-index 0..8, `anomaly-modal__box`, `anomaly-modal__image`, src `../assets/anomaly/images/challenge/<hash>.jpg`), `<button name="challenge-submit" class="btn btn--primary anomaly-modal__submit js-anomaly-modal-submit" form="challenge-form" ...>Submit</button>`, "Images not loading?" feedback block (an e-mail address and code - redacted). Page head contains `<form id="img-form" action="//duckduckgo.com/anomaly.js?sv=lite&cc=sre&ti=...` and `<form id="challenge-form" ... method="POST">`, `<iframe name="ifr" width="0" height="0" border="0" class="hidden">`, `<a class="header-url" href="/lite/"><span class="header">DuckDuckGo</span>`. He selects the form blocks (02:03).
- [02:08] Window focus switches to Firefox; address-bar dropdown of top sites again.
- [02:09] Address bar typed `devonzuegel.com/` (suggestions are YouTube Studio entries).
- [02:10] Address bar typed `deni` (suggestions: "denis ritchie nokia at DuckDuckGo", "denis nokia at DuckDuckGo").
- [02:12] Firefox shows page `www.nokia.com/bell-labs/about/dennis-m-r...` : "Dennis M. Ritchie / Location / [photo] / Dennis Ritchie, Bell Labs, Rm 2C-517, 600 Mountain Ave., Murray Hill, New Jersey 07974-0636, USA / <mail address> / <phone> (office), <phone> (fax) / In Memoriam: Dennis died in early October, 2011. This is a note from his sister and brothers: ..."
- [02:13] Full URL selected in the address bar: `www.nokia.com/bell-labs/about/dennis-m-ri...` (he copies it).
- [02:14] Editor: Get line URL being replaced; shows `resp, err := http.Get("https://lite.duckduckgo.com/lite/?q=hello")` with cursor right after `hello`, editor scrolled horizontally.
- [02:21] Terminal (non-maximized panel scrolled to the old DDG output).
- [02:22] Terminal maximized, cleared; `lab $ go run .` just executed, waiting.
- [02:24] Output: Akamai-style error page:
  ```
  <HTML><HEAD>
  <TITLE>Access Denied</TITLE>
  </HEAD><BODY>
  <H1>Access Denied</H1>

  You don't have permission to access "http&#58;&#47;&#47;www&#46;nokia&#46;com&#47;bell&#45;labs&#47;about&#47;dennis&#45;m&#45;ritchie&#47;" on this server.<P>
  Reference&#32;&#35;18&#46;15d62c17&#46;1793982811&#46;3e42dde7
  <P>https&#58;&#47;&#47;errors&#46;edgesuite&#46;net&#47;18&#46;15d62c17&#46;1793982811&#46;3e42dde7</P>
  </BODY>
  </HTML>
  ```
  then prompt `lab $ `. (Demonstrates a Go default request being blocked while a browser works.)
- [02:28] Editor: first line comment `// but without user-agent, some sites will block`; Get line now `resp, err := http.Get("https://www.nokia.com/bell-labs/about/d...` (rest off-screen).
- [02:29] Firefox in front on the Dennis Ritchie page, address bar focused.
- [02:31] Firefox back on `pkg.go.dev/net/http`, scrolled to the "The caller must close the response body" snippet, snippet highlighted.
- [02:32] Scrolled to Clients and Transports: `client := &http.Client{ CheckRedirect: redirectPolicyFunc, }`, `resp, err := client.Get("http:...`, `req, err := http.NewRequest("G...`, `req.Header.Add("If-None-Match"...`, `resp, err := client.Do(req)`.
- [02:35] The pkg.go.dev "Overview" outline dropdown opened: Outline (Documentation, Source Files, Directories); Documentation (Overview checked, Index, Examples, Constants, Variables, Functions, Types); Examples (CrossOriginProtection, FileServer, FileServer (DotFileHiding), FileServer (StripPrefix), Get, Handle, HandleFunc, Hijacker, ListenAndServe, ListenAndServeTLS, NotFoundHandler, Protocols (Http1), Protocols (Http1or2), ResponseWriter (Trailers), ServeMux.Handle, Server.Shutdown, StripPrefix); Functions (CanonicalHeaderKey(s), ...). "Types" hovered.
- [02:39] Dropdown scrolled; hovering "Handle"; Functions list: CanonicalHeaderKey(s), DetectContentType(data), Error(w, error, code), Handle(pattern, handler), HandleFunc(pattern, handler), ListenAndServe(addr, handler), ListenAndServeTLS(addr, certFile, keyFile, handler), MaxBytesReader(w, r, n)...

## File states
### main.go at end of segment (as of ~02:28; line numbers not visible in editor, assigned by order)
```
1| package main
2|
3| import (
4|   "fmt"
5|   "io"
6|   "net/http"
7| )
8|
9| func main() {
10|   // but without user-agent, some sites will block
11|   resp, err := http.Get("https://www.nokia.com/bell-labs/about/d?? (rest of URL off-screen; per browser: .../dennis-m-ritchie/))
12|   if err != nil {
13|     // handle error
14|   }
15|   defer resp.Body.Close()
16|   body, err := io.ReadAll(resp.Body)
17|
18|   fmt.Println(string(body))
19| }
```
Indentation is 2 visual columns per level in editor (displayed; tabs likely). Imports shown with red underlines (the editor flags them even though used; likely gopls not configured).

Intermediate versions:
- [00:29] body of main() empty.
- [01:04] URL `http://example.com/`; 
- [01:11-01:38] URL `https://wingram.org`, no comment, then comment added at line 10.
- [01:46-02:14] URL `https://lite.duckduckgo.com/lite/?q=hello`.
- [02:28] URL `https://www.nokia.com/bell-labs/about/d...`.

go.mod: seen only in sidebar, content not visible.

## Shell & outputs
- [00:00] `lab $ ` empty terminal.
- [01:04] `lab $ go run .` -> raw HTML of http://example.com/: `<!doctype html><html lang=en><head><meta charset=utf-8><link rel=icon href=data:,><meta name=viewport content="width=device-width,initial-scale=1"><title>Example Domain</title><style>html{color-scheme:light dark;background:light-dark(#eee,#222)}body{font:16px/1.6 system-ui,sans-serif;max-width:26em;margin:auto;padding:25vh 2em 2em;text-align:center}</style></head><body><p>This domain is for use in documentation examples without needing permission. This domain is not a service; avoid relying on it for testing and monitoring purposes.</p><script src=/s.js></script></body></html>` then `lab $ `.
- [01:11-01:19] `go run .` against https://wingram.org: long HTML ending in cloudflareinsights beacon script (`<script type="module" src="https://static.cloudflareinsights.com/beacon.min.js/vce...` with integrity/token attributes), `</body></html>`, `<!-- MkDocs version : 1.6.1 Build Date UTC : 2025-02-08 08:01:53.494741+00:00 -->`. Also a keyboard shortcuts table (`<td class="help shortcut"><kbd>?</kbd></td><td>Open this help</td>`, next `n` "Next page", prev `p` "Previous page", search `s` "Search").
- [01:46-02:00] `go run .` against https://lite.duckduckgo.com/lite/?q=hello -> bot-challenge HTML (see Timeline), ends with `<img src="//duckduckgo.com/t/tqadb?2957930&amp;s=lite&amp;e=0&amp;p=...&amp;iadb=0"/>` `</body>` `</html>`.
- [02:22-02:24] `lab $ go run .` against nokia.com page -> Access Denied page quoted above.

## Observations
- Opening move: look up `net/http` on pkg.go.dev, copy the doc's `http.Get` + `defer resp.Body.Close()` + `io.ReadAll` example verbatim into main() as the first building block of the "wget".
- Demonstrates empirically (not by explanation) why a plain Go client is insufficient: example.com and wingram.org work, DuckDuckGo Lite serves a "bots use DuckDuckGo too" challenge, nokia.com returns Akamai "Access Denied". He records this in a code comment: `// but without user-agent, some sites will block`, pointing toward setting a User-Agent header next (he then scans the docs for Clients and Transports / `http.NewRequest` / `req.Header.Add` / `client.Do(req)`).
- He compares Firefox (works, shows page) vs Go output (blocked) side by side for the same URL.
- Style: minimal setup (go.mod + single main.go), scratch directory named `lab`, iterate with `go run .` in the integrated terminal, error handling left as the docs' `// handle error` placeholder.
