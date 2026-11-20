# Segment 9/9 (21:33 - 24:56) - go-wget by Hayyoth

Editor: VSCodium (macOS), project folder "lab", window title "main.go — lab". Editor has NO line-number gutter (line numbers not visible), so File states use no numbers (only relative order). Firefox window behind shows github.com/codegram01/gfm/blob/main/ma... (a Go file with os.WriteFile example) until 24:29, then localhost preview.

## Timeline

- [21:33] main.go, `save` function visible. Cursor inside the string `"index.html"` (word "index" highlighted). Lines in save at this point (order): `u, err := url.Parse(link)`, err check, `link = u.Host + u.Path`, `fmt.Println("===> save ", link)`, `os.MkdirAll(link, 0777)`, blank, `if strings.HasSuffix(link, "/") { link += "index.html" }`, blank, `// save file content`, `//os.WriteFile(path, []byte(content), 0777)` (still commented out). Explorer tree: wingram.org/about (father, sites, wingram expanded - folders already created by earlier run), go.mod, go.sum, main.go.
- [21:37] The `fmt.Println("===> save ", link)` line was moved: now order is `link = u.Host + u.Path`, `os.MkdirAll(link, 0777)`, blank, `if strings.HasSuffix...`, ..., `// save file content`. main.go shows "1" problem badge (red) at that instant.
- [21:40] `fmt.Println("===> save ", link)` re-placed AFTER the HasSuffix block (before `// save file content`). Explorer: wingram folder row selected (blue), mouse on wingram.org/about.
- [21:43] Terminal opened (panel maximized). `lab $ go run .` typed (not yet executed). Explorer shows only go.mod, go.sum, main.go (folders deleted).
- [21:52] Terminal output after run (see Shell). Explorer now: `> wingram.org`, go.mod, go.sum, main.go. Selection highlights last three lines. Note the first line printed `wingram.org` with no path (no trailing "/" appended, so MkdirAll made wingram.org as dir; file not named index.html).
- [22:01] Terminal: first output line `wingram.org` highlighted with mouse - he points out that the root URL saves as "wingram.org" (a directory), not wingram.org/index.html.
- [22:18] Back in main.go. A blank line inserted after `os.MkdirAll(link, 0777)`; `content` param has blue squiggle (unused variable warning). Cursor at blank line.
- [22:43] More blank lines added (editing cursor on blank line before `if strings.HasSuffix`).
- [22:51] Terminal (resized, hint box "66 x 37"): garbage text typed at prompt `lab $ wingram.org must wingram.org/index.html` (he's typing/reading notes into terminal, not executed).
- [23:00] main.go: added debug line `fmt.Println("path is", u.Path)` after MkdirAll (u.Path highlighted).
- [23:02] Debug line still there, cursor on blank line below it; scrolled.
- [23:04] Same, scrolled further.
- [23:15] Added `if u.Path == "" {` with empty body and closing `}` (between debug println and HasSuffix if). The debug println line is still present just above.
- [23:37] Completed: `if u.Path == "" { link += "/" }`. The debug `fmt.Println("path is", u.Path)` line is gone. Fold arrows (chevrons) now showing in the gutter.
- [23:39] Terminal cleared: `lab $ ` (hint "66 x 37").
- [23:47] main.go: same code, mouse near `"/"` in HasSuffix; save still ends with commented `//os.WriteFile(path, []byte(content), 0777)`.
- [23:48] scrolled: same code.
- [23:58] Edited last line: `//os.WriteFile(path, []byte(content), 0777)` replaced by `os.WriteFile(link, content, 0777)` (cursor right after `content`). Squiggle on `content` gone (now used).
- [23:59] Scrolled up to show imports: `"strings"`, blank, `"golang.org/x/net/html"`, `"golang.org/x/net/html/atom"`, `)`.
- [24:01] Terminal: `lab $ go run .` output (see Shell). Explorer: `> wingram.org`, go.mod, go.sum, main.go.
- [24:07] Context menu on wingram.org folder in Explorer (New File..., New Folder..., Open with Five Server (root), Reveal in Finder, Open in Images Preview, Open in Integrated Terminal, Find in Folder..., Open Timeline, Cut, Copy, Paste, Copy Path (highlighted), Copy Relative Path, Run Tests, Debug Tests, Rename..., Delete).
- [24:08] Terminal shows `lab $ go run .` typed again (cursor box, not yet run); wingram.org folder selected.
- [24:15] Re-ran; terminal pane reduced (hint "66 x 22"). Editor opened `wingram.org/index.html` (title "index.html — lab"). Content: HTML head with dark theme (see File states). Terminal output same four `===> save` lines.
- [24:16] Hover tooltip on `<meta` shows MDN text: "The meta element represents various kinds of metadata that cannot be expressed using the title, base, link, style, and script elements. / Widely available across major browsers (Baseline since 2015) / MDN Reference".
- [24:19] Opened wingram.org/about/father/index.html (tree: wingram.org > about > father > index.html; sites; wingram; index.html). Head shows `../../` relative paths and title "Wingram Father - Wingram.Org".
- [24:24] Opened wingram.org/about/wingram/index.html scrolled to body (navbar, `<script src="../../js/darkmode.js"></script>` etc.).
- [24:25] Mouse on root wingram.org/index.html in tree.
- [24:26] Right-click context menu on wingram.org/index.html: Open to the Side, Open with Five Server, Open With..., Reveal in Finder, Open in Integrated Browser, Open in Integrated Terminal, Select for Compare, Open Timeline, Cut, Copy, Copy Path, Copy Relative Path, Run Tests, Debug Tests, Rename..., Delete.
- [24:28] Firefox in front at `http://127.0.0.1:5500/wingram.org/index.html` (served via Five Server / live server): unstyled page of the crawled copy: "Wingram.Org" link, bullet list "Wingram.Org > About", H1 "Wingram.Org", "Wingram on the way we find the Truth.", H2 "About" with links "Wingram Philosophy", "Wingram Father", "Sites - Contact"; "Don't forget bookmark this site."; "You can forget this site, but you can't forget an Ideology"; "Keyboard Shortcuts" table (Keys/Action: ? Open this help; n Next page; p Previous page; s Search). (CSS not downloaded, so unstyled.)
- [24:29-24:30] Scrolled the same page.
- [24:32] Clicked "Wingram Philosophy": URL `http://127.0.0.1:5500/wingram.org/about/wingra...`; page: "Wingram Philosophy", "Before we can answer, we need the question", H2 "Why, how and what we need to find?" (What's Life and Life Purpose? / How and Why to Live? / What's Universe, what created it? / What question need to find?), H2 "How do we find the Truth?" (Knowledge is all we have / Philosophy help we continue / Science is good method / Computer is great tool), "Faith gives strength".
- [24:33] Back to index page.
- [24:34-24:35] "Wingram Father" page at `.../wingram.org/about/father/`: H1 "Wingram Father", "I'm just one person who wants to make something good for the world", sections "I had a strange dream", "How I help you?", "I quit anything", "What i do today?" (page text is public site content).
- [24:36] Back to index again.
- [24:37] "Sites" page `.../wingram.org/about/sites/`: "You can follow Wingram on these platform" with list Web, Telegram, Github (github.com/codegram01), Youtube, X, Mail (<mail address>); "Home" link; keyboard shortcuts table.
- [24:40] Back to editor, wingram.org/index.html, cursor on blank line within head; browser dimmed behind.
- [24:41] main.go opened again: final save() visible (see File states).
- [24:42-24:46] Scrolling through main.go (crawl function visible).
- [24:48] Editor window maximized/resized (title bar moved), mouse at top.
- [24:49] Editor fullscreen; scrolled to end showing end of crawl and `func main()`.
- [24:50-24:51] Scrolled; User-Agent line fully visible.
- [24:52] Cursor placed on a blank line inserted before `func crawl`; main.go tab dimmed (unsaved/selected).
- [24:56] Typed comment `// done - simple` above `func crawl`. macOS menu bar visible (VSCodium menus: File Edit Selection View Go Run Terminal Window Help; clock "Fri Nov 6 11:56 PM"; screen-recording stop button hovered) - he is about to stop recording. End of video. No save message visible.

## File states

### main.go (final, ~24:56). Note: no gutter numbers visible; indentation is 2 spaces (tabs rendered). Package/import head partly hidden.

```
?? (package main, `import (` and early imports not visible; between visible imports "net/http","net/url","os","strings"; earlier ones likely "fmt","io","log" - seen in earlier segments)
	"net/http"
	"net/url"
	"os"
	"strings"

	"golang.org/x/net/html"
	"golang.org/x/net/html/atom"
)

func save(link string, content []byte) {
  u, err := url.Parse(link)
  if err != nil {
    log.Fatal(err)
  }
  link = u.Host + u.Path
  os.MkdirAll(link, 0777)

  if u.Path == "" {
    link += "/"
  }

  if strings.HasSuffix(link, "/") {
    link += "index.html"
  }

  fmt.Println("===> save ", link)

  // save file content
  os.WriteFile(link, content, 0777)
}

// done - simple
func crawl(link string) {
  client := &http.Client{}

  req, err := http.NewRequest("GET", link, nil)
  if err != nil {
    log.Fatal(err)
  }
  req.Header.Add("User-Agent", `Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:159.0) Gecko/20100101 Firefox/159.0`)
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
}
```

Notes: `// done - simple` comment added at 24:56 (before it, only a blank line there at 24:52). Between `if err != nil` after Do and `defer` there is one blank line. The `MkdirAll(link, 0777)` is called BEFORE the "/" + index.html suffix is added, so it creates a directory named after the full path e.g. `wingram.org/about/father/` (and for root `wingram.org`); the `index.html` suffix logic runs after. (Mkdir of the link when link ends with "/" is the directory; when it doesn't, it creates directory named same as the page - observable in tree.)

Intermediate versions of save():
- 21:33: `link = u.Host + u.Path` / `fmt.Println("===> save ", link)` / `os.MkdirAll(link, 0777)` / blank / HasSuffix block / `// save file content` / `//os.WriteFile(path, []byte(content), 0777)`.
- 21:40: Println moved below HasSuffix block.
- 23:00: debug `fmt.Println("path is", u.Path)` inserted after MkdirAll.
- 23:15: `if u.Path == "" {` `}` started; 23:37 filled with `link += "/"`, debug line removed.
- 23:58: WriteFile uncommented and changed to `os.WriteFile(link, content, 0777)`.

### wingram.org/index.html (crawled output, viewed 24:15/24:40; partial)

```
<!DOCTYPE html>
<html lang="en" data-bs-theme="dark">
    <head>
        <meta charset="utf-8">
        <meta http-equiv="X-UA-Compatible" content="IE=edge">
        <meta name="viewport" content="width=device-width, initi...
        <meta name="description" content="Wingram on the way we ...
        <meta name="author" content="Wingram.Org">
        <link rel="canonical" href="https://wingram.org/">
        <link rel="shortcut icon" href="img/favicon.ico">
    <title>Home - Wingram.Org</title>
    <meta name="title" property="og:title" content="Home - Wingr...
    <meta name="url" property="og:url" content="https://wingram....
    ?? (blank lines)
        <meta name="image" property="og:image" content="/img/log...
    ?? 
        <link href="css/bootstrap.min.css" rel="stylesheet">
        <link href="css/fontawesome.min.css" rel="stylesheet">
        <link href="css/brands.min.css" rel="stylesheet">
        <link href="css/solid.min.css" rel="stylesheet">
        <link href="css/v4-font-face.min.css" rel="stylesheet">
        <link href="css/base.css" rel="stylesheet">
        <link id="hljs-light" rel="stylesheet" href="https://cdn...
        <link id="hljs-dark" rel="stylesheet" href="https://cdnj...
        <link href="assets/css/style.css" rel="stylesheet">
        <script src="https://cdnjs.cloudflare.com/ajax/libs/high...
        <script>hljs.highlightAll();</script>
```
(Site-generated static HTML; not authored by the video.) The father page uses `../../css/...`, `../../img/favicon.ico`, title "Wingram Father - Wingram.Org".

### Explorer tree (24:56)
```
wingram.org
  about
    father
      index.html
    sites (collapsed)
    wingram
      index.html
  index.html
go.mod
go.sum
main.go
```

## Shell & outputs

- [21:43] `lab $ go run .` (typed in terminal, empty before).
- [21:52] output:
```
lab $ go run .
===> save  wingram.org
===> save  wingram.org/about/wingram/index.html
===> save  wingram.org/about/father/index.html
===> save  wingram.org/about/sites/index.html
lab $
```
(note double space after "save" since Println adds space after the string "===> save ").
- [22:51] garbage typed (not run): `lab $ wingram.org must wingram.org/index.html`
- [23:39] cleared terminal `lab $ `.
- [24:01] after fix (`go run .`):
```
lab $ go run .
===> save  wingram.org/index.html
===> save  wingram.org/about/wingram/index.html
===> save  wingram.org/about/father/index.html
===> save  wingram.org/about/sites/index.html
lab $
```
- [24:08] `lab $ go run .` typed again after deleting/cleaning wingram.org; [24:15] same output as above (terminal "66 x 22").
- Browser (Five Server at http://127.0.0.1:5500/wingram.org/...) at 24:28-24:37 shows the crawled pages unstyled as described in Timeline.

## Observations

- Key bug found: for the root URL, `u.Path == ""` so link had no trailing slash and file saved as a file/dir named `wingram.org`; fix is `if u.Path == "" { link += "/" }` before the HasSuffix check. Debugged with a temporary `fmt.Println("path is", u.Path)`.
- Final design: `save(link, content)` computes `host+path`, mkdirs, appends `index.html` for directory-like URLs, logs `===> save`, and writes with `os.WriteFile(link, content, 0777)` (adapted from the codegram01/gfm GitHub reference file visible in Firefox: `WriteFile(path, []byte(content), 0777)`).
- crawl is minimal recursive: fetch with custom Firefox User-Agent, save body, parse with golang.org/x/net/html (`doc.Descendants()` iterator, Go 1.23+), follow only relative links not starting with "#", "/" or "https" (`crawl(link + "/" + subLink)`), `break` after the href attribute. No visited-set, no assets (CSS/JS/images not downloaded, so the offline copy is unstyled).
- He verifies the result by serving the saved folder with the Five Server extension and clicking through the offline copy (Home -> Philosophy / Father / Sites), then closes with the comment `// done - simple` and stops screen recording (~24:56). Tone: "simple is done" philosophy; web content shown is his own project site.
