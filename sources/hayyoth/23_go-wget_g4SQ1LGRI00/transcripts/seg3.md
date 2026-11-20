# Segment 3 (05:15 - 07:19) - g4SQ1LGRI00

Setup: the editor is a macOS-style window titled `main.go — lab` (VSCodium look, light theme, no line-number gutter, no tabs visible, 2-space indentation with indent guides). A Firefox window (hayyoth.org with DevTools Network panel, later pkg.go.dev, later DuckDuckGo Lite) sits behind it. The webcam window "Human" is ignored. Because no gutter is shown, the line numbers below are relative to the visible fragment and are NOT editor line numbers.

## Timeline

- [05:15] Firefox in front (address bar `hayyoth.org`, DevTools Network tab, "Disable Cache" checked). Two requests listed: `GET / doc htm 2.17...` (selected) and `GET favicon.ico img x-i 4.72...`. Footer "2 requests 19.26 kB / 6.89 kB transferred". Headers pane for `GET https://hayyoth.org/`: Status 200, Version HTTP/3, Transferred 2.17 kB (3.85 kB size), Request Priority Highest, DNS Resolution System. Response Headers (611 B): alt-svc `h3=":443"; ma=86400`, cache-control `public, max-age=0, must-r...`, cf-cache-status HIT, content-encoding zstd, content-type text/html, date Fri, 06 Nov 2026 16:36:21 GMT. Page on the right: "Hayyoth - White Paper - Understand the Game. Don't Play It Blindly. The Art of Quitting." plus a Year/Everyone/I table (2018..2025).
  Editor behind (main.go) at this time:
  `func crawl(link string) {` with `client := &http.Client{}`, `req, err := http.NewRequest("GET", "http://example.com", nil)`, `if err != nil { log.Fatal(err) }`, `req.Header.Add("If-None-Match", `W/"wyzzy"`)`, `resp, err := client.Do(req)`, empty line, `}`; and `func main()` containing the comment `// but without user-agent, some sites will block`, `resp, err := http.Get("https://www.nokia.com/bell-labs/...`, error check, `defer resp.Body.Close()`, `body, err := io.ReadAll(resp.Body)`, `fmt.Println(string(body))`.
- [05:16] Scrolls response headers: priority u=0,i; report-to {"group":"cf-nel","max_age":604800,"endpoints":[{"url":"https://a.nel.cloudflare.com/report/v4?s=..."}]}; server cloudflare; server-timing cfExtPri. Request Headers (492 B) begin: Accept text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8; Accept-Encoding gzip, deflate, br, zstd; Accept-Language en-US,en;q=0.9; Cache-Control no-cache; Connection keep-alive.
- [05:17-05:18] Request Headers fully visible: Host hayyoth.org, Pragma no-cache, Priority u=0, i, Sec-Fetch-Dest document, Sec-Fetch-Mode navigate, Sec-Fetch-Site none, Sec-Fetch-User ?1, TE trailers, Upgrade-Insecure-Requests 1, User-Agent `Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:159.0) Gecko/20100101 Firefox/159.0` (wraps).
- [05:24] Firefox window dragged left; User-Agent row highlighted.
- [05:28] Firefox moved right; the User-Agent value text is selected (copied).
- [05:29] Editor brought to front, caret at end of `if err != nil {` line (line after NewRequest). Full lines visible: `req, err := http.NewRequest("GET", "http://example.com", nil)`; main still has nokia `http.Get("https://www.nokia.com/bell-labs/about/dennis-m-ritchie/")`.
- [05:38] Line `req.Header.Add("If-None-Match", `W/"wyzzy"`)` edited: the editor is scrolled horizontally to the right; the line now ends with `...illa/5.0 (Macintosh; Intel Mac OS X 10.15; rv:159.0 Gecko/20100101 Firefox/159.0`)` i.e. the pasted UA string is inside backticks.
- [05:39] Scrolled partially; line reads `Header.Add("If-None-Match", `Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:159.0)...` (the key is still "If-None-Match" at this instant, value already replaced).
- [05:45] Key now changed: `req.Header.Add("User-Agent", `Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:159...` (line runs off the right edge). `resp, err := client.Do(req)` shows `resp` in grey with squiggle (unused variable).
- [05:46] Editor scrolled up 1 line (`"log"` partially visible at top). Same code.
- [05:51] Hover tooltip over `http` in `http.NewRequest`: "package http / Package http provides HTTP client and server implementations. / Get, Head, Post, and PostForm make HTTP (or HTTPS) requests: resp, err := http.Get("http://example.com/") ... resp, err := http.Post("http://example.com/upload", "image/jpeg", &buf) ... resp, err := http.PostForm("http://example.com/form", url.Values{"key": {"Value"}, "id": {"123"}}) / The caller must close the response body when finished with it:".
- [05:52-05:53] Editor scrolled down; caret at end of `body, err := io.ReadAll(resp.Body)` in main. Top of file now starts at `func crawl(link string) {`.
- [06:02] The code that was in main after the `http.Get` call has been cut and moved into crawl: now crawl body is: client, NewRequest, err check, `req.Header.Add("User-Agent", ...)`, `resp, err := client.Do(req)`, blank, `defer resp.Body.Close()`, `body, err := io.ReadAll(resp.Body)`, blank, `fmt.Println(string(body))` (caret at its end, matching-paren highlight), blank, `}`. main now: comment, `resp, err := http.Get("https://www.nokia.com/...")`, `if err != nil { // handle error }`, blank, `}`. (`resp` squiggle in main.)
- [06:03] Same; scrolled slightly. Cursor near `User-Agent` string.
- [06:04] The `if err != nil { log.Fatal(err) }` block (3 lines) after NewRequest is selected (multi-line selection, to copy).
- [06:06] Pasted after `resp, err := client.Do(req)`: now
  `if err != nil {` / `  log.Fatal(err)` / `}` follows client.Do (caret after the closing `}`).
- [06:08-06:09] Scrolled; same code. `resp` in main still squiggled.
- [06:10] Scrolled; caret at `}` after Do's err check.
- [06:11] In main: the comment line, `resp, err := http.Get(...)`, `if err != nil { // handle error }` and trailing blank line are all selected (to delete).
- [06:23] main body deleted; typing `cr` inside `func main() {` (squiggle). 
- [06:26] main now: `crawl("https://www.nokia.com/bell-labs/about/dennis-m-ritchie/")` followed by blank line and `}`. Editor scrolled to show `func crawl(link string) {` at top.
- [06:33] Scroll up shows imports: `import (` / `"fmt"` / `"io"` / `"log"` / `"net/http"` / `)`. Frame at 06:33 then shows the integrated terminal (TERMINAL panel, maximised, size indicator "87 x 37") after `go run .`.
- [06:36] Terminal panel resized (indicator "87 x 23"); editor visible above showing `req, err := http.NewRequest("GET", link, nil)` (the hardcoded "http://example.com" has been replaced by `link`). Terminal shows the `go run .` output (see Shell).
- [06:37] Terminal cleared (clear screen): only `lab $` prompt.
- [06:40-06:48] Terminal shows output of a second `go run .` (scrolling through HTML of the Nokia Bell Labs "Dennis M. Ritchie" page, as the UA header now works). Scrolled by the mouse wheel up/down: Bibliography, Plan 9 and Inferno, court papers, etc. (see Shell).
- [06:48] Back at the bottom; the shell prompt shows `lab $ #` (the author starts typing a `#` comment).
- [06:53] Terminal hidden; editor at full height with `link` highlighted at `http.NewRequest("GET", link, nil)` (cursor right after `link`; `link` in the parameter list is also highlighted as a same-symbol match). main: `crawl("https://www.nokia.com/bell-labs/about/dennis-m-ritchie/")`.
- [06:56] Firefox brought to front, still on hayyoth.org with DevTools; User-Agent row highlighted again. (He is comparing UA.)
- [06:57] Firefox new tab/page: empty page with the address bar focused, placeholder "Search or enter address" (DuckDuckGo engine icon).
- [06:58] Firefox on `pkg.go.dev/net/http`, "http package standard library", Overview section showing the example code: `client := &http.Client{ CheckRedirect: redirectPolicyFunc, }` / `resp, err := client.Get("http://example.com")` / `// ...` / `req, err := http.NewRequest("GET", "http://example.com", nil)` / `// ...` / `req.Header.Add("If-None-Match", `W/"wyzzy"`)` / `resp, err := client.Do(req)` / `// ...` ; then "For control over proxies, TLS configuration... create a Transport:" `tr := &http.Transport{ MaxIdleConns: 10, IdleConnTimeout: 30 * tim...  DisableCompression: true, }` `client := &http.Client{Transport...`. (The code from the docs that he had originally copied is the "If-None-Match" line.)
- [07:00] Firefox address bar typed `hello ` with the suggestion dropdown (Search with Duck Lite; Firefox Suggest: hello at DuckDuckGo html.duckduckgo.com/html/, lite.duckduckgo.com/lite/, ... localhost:8080/hello).
- [07:01] URL becomes `lite.duckduckgo.com/lite/?q=hello+`; status "Transferring data from lite.duckduckgo.com...".
- [07:02] DuckDuckGo Lite results page: 1. Hello (Wikipedia: "Hello" is a song by British singer-songwriter Adele, released on 23 October 2015 ... lead single from her third studio album, 25), 2. Adele - Hello (Official Music Video) - YouTube, 3. Adele - Hello - YouTube, 4. HELLO! - Daily royal, celebrity, fashion, beauty & lifestyle news (www.hellomagazine.com), 5. Home | HOME (hello.hanu.edu.vn/web/), 6. Hello - Wikipedia. Address bar dropdown shows Hayyoth hayyoth.org, Google Gemini, Telegram Web, Hayyoth - YouTube, Google Translate (history suggestions).
- [07:04] Editor in front again; caret at end of `client := &http.Client{}`; URL in Firefox remains `lite.duckduckgo.com/lite/?q=hello+` (highlighted).
- [07:05] Editor scrolled down to show main; still `crawl("https://www.nokia.com/bell-labs/about/dennis-m-ritchie/")` (mouse near end of the URL). Terminal then reappears at the bottom (07:12). The main call was presumably changed to the DDG Lite URL before running, but it is not visible in these frames.
- [07:12] Terminal shows tail of the DDG Lite HTML (see Shell): `<form action="/lite/" method="post"> <input class='query' type="text" size="40" name="q" value="hello" > <input class='submit' type="submit" value="Search"> ... </form>`, `<p class='extra'>&nbsp;</p>`, `<img src="//duckduckgo.com/t/sl_l"/>`, `<div id='end-spacer'>&nbsp;</div>`, `</body></html>`, prompt `lab $`.
- [07:13-07:19] Scrolls up in terminal through the DDG Lite raw HTML (next-page form; result rows for adele.fandom.com/wiki/Hello_(song); "Only show abstract separately if there's a click URL (not EOF)" comment; snippet text; Spotify track result). At 07:17 a block (comment + `<tr>` for the result-snippet) is selected with the mouse.

## File states

### main.go (as of 07:05, last time the editor source is visible; 07:19 editor top half unchanged)
No gutter numbers visible; the lines below are numbered relative to the file start ONLY for the visible parts and flagged. Top of file (package line, blank, `import (` start) not fully visible: `?? (lines 1-2: presumably "package main")`.

```
 ?? package main                       (not visible in this segment)
 ?? import (
 ?? 	"fmt"
 ?? 	"io"
 ?? 	"log"
 ?? 	"net/http"
 ?? )
 ??
 ?? func crawl(link string) {
 ??   client := &http.Client{}
 ??
 ??   req, err := http.NewRequest("GET", link, nil)
 ??   if err != nil {
 ??     log.Fatal(err)
 ??   }
 ??   req.Header.Add("User-Agent", `Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:159.0) Gecko/20100101 Firefox/159.0`)
 ??   resp, err := client.Do(req)
 ??   if err != nil {
 ??     log.Fatal(err)
 ??   }
 ??
 ??   defer resp.Body.Close()
 ??   body, err := io.ReadAll(resp.Body)
 ??
 ??   fmt.Println(string(body))
 ??
 ?? }
 ??
 ?? func main() {
 ??   crawl("https://www.nokia.com/bell-labs/about/dennis-m-ritchie/")
 ?? }
```
Notes: the exact tail of the User-Agent line is only seen at 05:38 as `...rv:159.0) Gecko/20100101 Firefox/159.0`)` (ends with a backtick before the closing paren). The body after `io.ReadAll` ignores the `err` (no check). Indentation: 2 spaces (rendered as indent guides). The `main` argument after 07:05 may have been changed to the DuckDuckGo Lite URL (`lite.duckduckgo.com/lite/?q=hello+`) for the 07:12 run; that is not confirmed by a frame.

Intermediate state at 05:15 (before edit):
```
 func crawl(link string) {
   client := &http.Client{}

   req, err := http.NewRequest("GET", "http://example.com", nil)
   if err != nil {
     log.Fatal(err)
   }
   req.Header.Add("If-None-Match", `W/"wyzzy"`)
   resp, err := client.Do(req)

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
(This is the docs-example text pasted in as a skeleton, from the pkg.go.dev example.)

## Shell & outputs

Integrated terminal prompt: `lab $ `.

- [06:33] 
```
lab $
lab $ go run .
<HTML><HEAD>
<TITLE>Access Denied</TITLE>
</HEAD><BODY>
<H1>Access Denied</H1>

You don't have permission to access "http&#58;&#47;&#47;www&#46;nokia&#46;com&#47;bell&#45;labs&#47;about&#47;dennis&#45;m&#45;ritchie&#47;" on this server.<P>
Reference&#32;&#35;18&#46;15d62c17&#46;1793982811&#46;3e42dde7
<P>https&#58;&#47;&#47;errors&#46;edgesuite&#46;net&#47;18&#46;15d62c17&#46;1793982811&#46;3e42dde7</P>
</BODY>
</HTML>

lab $ 
```
(Akamai "Access Denied" page; this is the first run, evidently made with the code at 06:26 where crawl was still using the then-current code. Note the 06:26 crawl code used "http://example.com" per frames at 06:26; but the output references nokia, so the run was with the changed `link` version; the file at 06:26 may simply have been unsaved/ordering of events. At 06:36 the editor shows `link`.)
- [06:37] Screen cleared; `lab $ ` only.
- [06:40-06:48] Second run output (command line not visible; scrolled). The HTML of Dennis Ritchie's page is printed, fragments visible:
  - `<h2>Bibliography</h2>` / `<a href="bib/dmr.bib"> bibtex format </a> or` / `<a href="bib/dmr.html"> html format </a>.` / `<p>` / `<hr>` / `<em>` / `Fiddled: May 2006 to add organization changes;` / `March 2002, to add the HOPL talk link,` / `July 2002 to add the C tutorial paper, October 2002` / `to add the ICGA paper,` / `January 2003 to add the Portability paper,` / `April 2003 to add the Kleine material, October 2003` / `for additional portability papers.` / `<br>` / `December 2006 to add new Alcatel-Lucent logo.` / `</em></body>` / `</HTML>` / prompt `lab $`.
  - `<li>Not only I, but also <A href="otherunix.html">UNIX&reg;</A>` / `has lives far away, and I don't mean Linux or *BSD. See some examples.` / `<li> <A href="complinks.html">Links to sites</A> with Unix, C, and other (generally older) historical material.` / `<h2>Biography</h2>` / `A brief <a href="bigbio1st.html">biography</a>, in first person instead of obituary style.`
  - `Plan 9 and Inferno` list: `<li>The new, open-source edition of the <A HREF="https://s3.amazonaws.com/plan9-bell-labs/plan9/index.html">Plan 9</A> system is available. I contributed only a few bits and pieces to it, but did, in effect, sign some paychecks to keep it going.` / `<li> The system-structuring ideas of Plan 9 were adopted also by the Inferno system, now distributed by <A HREF="http://www.vitanuova.com">Vita Nuova</A>. Again, this was more a matter of signing paychecks than doing the work, though I did write about it.` / `<h2> Links I've gathered </h2>` / `<A href="bsdi/bsdisuit.html"> Some court papers</A> from the lawsuit brought by USL against BSDI, then the University of California, in the early 1990s about Unix intellectual property. These may be relevant today in view of SCO's recent actions.` / "synergy between Ken Thompson's activities in chess, other games, and systems. It includes a funny faked memo by Mike Lesk."
- [06:48] After scrolling to bottom, prompt: `lab $ #`
- [07:12] Third run output (command not visible) tail:
```
        <form action="/lite/" method="post">
            <input class='query' type="text" size="40" name="q" value="hello" >
            <input class='submit' type="submit" value="Search">




                <input name="kl" value="wt-wt" type="hidden">

        </form>

        <p class='extra'>&nbsp;</p>


          <img src="//duckduckgo.com/t/sl_l"/>

        <div id='end-spacer'>&nbsp;</div>

</body>
</html>
lab $
```
- [07:13] Scrolled up: `<td colspan=2>` / `<!-- Next Page Button Sub-template -->` / `<form class="next_form" action="/lite/" method="post">` / `<input type="submit" class='navbutton' value="Next Page &gt;">` / `<input type="hidden" name="q" value="hello">` / `<input type="hidden" name="s" value="10">` / `<input type="hidden" name="nextParams" value="">` / `<input type="hidden" name="v" value="l">` / `<input type="hidden" name="o" value="json">` / `<input type="hidden" name="dc" value="11">` / `<input type="hidden" name="api" value="d.js">` / `<input type="hidden" name="vqd" value="4-196480630044694883759350582541651531959">` / `&nbsp;&nbsp;&nbsp;&nbsp;` / `<input name="kl" value="wt-wt" type="hidden">`.
- [07:14-07:19] Further scrolling: `<tr><td>&nbsp;&nbsp;&nbsp;</td><td><span class='link-text'>adele.fandom.com/wiki/Hello_(song)</span></td></tr>`; result-link anchors `...fandom.com%2Fwiki%2FHello_(song)&amp;rut=07f0dce305d64f24caddd04182c03170c8b215fbe8912f591464da140ef3e5d1" class='result-link'>Hello (song) | Adele Wiki | Fandom</a>`; `<!-- Only show abstract separately if there's a click URL (not EOF) -->` then `<td class='result-snippet'> &quot;<b>Hello</b>&quot; is a song by British recording artist Adele. It was released as the lead single from her third studio album, 25, on October 23, 2015. [3] It was written by Adele and Greg Kurstin, and produced by the latter. &quot;<b>Hello</b>&quot; was first teased in a 30-second snippet that aired during The X Factor on...`; Spotify result: `<span class='link-text'>open.spotify.com/track/0ENSn4fwAbCGeFGVUbXEU3</span>`, `<a rel="nofollow" href="//duckduckgo.com/l/?uddg=https%3A%2F%2Fopen.spotify.com%2Ftrack%2F0ENSn4fwAbCGeFGVUbXEU3&amp;rut=f761f5b8504f4f8824b6bb8b40adf4507382ac4f6c4be197f75959d456352a3f" class='result-link'>Hello - song and lyrics by Adele | Spotify</a>`, snippet `From the single <b>Hello</b> October 23, 2015 ℗ 2015 XL Recordings Ltd., under exclusive license to Columbia Records, a Division of Sony Music Entertainment`.

Web pages read: hayyoth.org (DevTools Network, request/response headers, esp. User-Agent), pkg.go.dev/net/http Overview (client/request examples; `req.Header.Add`), lite.duckduckgo.com/lite/?q=hello+ (results page, as a target to crawl). No editor save messages (no vim; VSCodium shows none).

## Observations

- Workflow: observe what a real browser sends (copy its User-Agent from Firefox DevTools), then reproduce it in Go with `http.NewRequest` + `req.Header.Add("User-Agent", ...)`. Comment in earlier main said "but without user-agent, some sites will block"; the 06:33 run (Nokia Bell Labs page returning Akamai "Access Denied") demonstrates the problem, and the follow-up run with the Firefox UA prints the real HTML (Dennis Ritchie page).
- Starts from the pkg.go.dev `net/http` doc example (NewRequest/Header.Add("If-None-Match", ...)/Do) and edits it in place; he treats the docs as copy-paste scaffolding, then moves the plain `http.Get` code from `main` into `crawl(link string)` and replaces the hardcoded URL by `link`.
- Refactor: cut/paste of `if err != nil { log.Fatal(err) }` blocks rather than typing; `main` reduced to a single `crawl("...")` call. Errors are handled with `log.Fatal`; `io.ReadAll` error is unchecked; `resp.Body.Close()` via `defer`.
- Next target: a search engine results page (DuckDuckGo Lite, `lite.duckduckgo.com/lite/?q=hello+`) - he picks the Lite (no-JS HTML) variant because it is simple HTML to crawl/parse; the raw HTML contains result-link anchors with `class='result-link'`, `result-snippet`, `link-text` spans, and a "Next Page" form with hidden `q`, `s`, `vqd` fields (probably pointing toward link extraction).
- Style: terminal and editor share one window; he resizes the terminal panel by dragging; uses `clear`; no tests; minimal comments; macOS (Firefox UA says Macintosh Intel Mac OS X 10.15).
