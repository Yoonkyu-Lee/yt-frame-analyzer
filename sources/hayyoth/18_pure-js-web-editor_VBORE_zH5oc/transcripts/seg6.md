# Segment 6 (18:48 - 22:17) — VBORE_zH5oc

Editor is a macOS code editor window titled "<file> — web-editor" (sidebar WEB-EDITOR, light theme, line numbers NOT shown). Browser is Firefox at 127.0.0.1:5500 with Five-server live reload ("fiveserver.js"). No editor save messages visible in this segment.

## Timeline
- [18:48] Firefox tab ace.c9.io/#nav=embedding ("Embedding Ace in Your Site"). Cursor on line 25 of the sample (`editor.setTheme("ace/theme/monokai");`). Sample shows `<script src="/ace-builds/src-noconflict/ace.js" type="text/javascript" charset="utf-8"></script>`, `var editor = ace.edit("editor");`, `editor.session.setMode("ace/mode/javascript");`.
- [18:49] Page scrolled up; full 29-line sample visible (see Reference below).
- [18:52] Author selects the `#editor { position: absolute; top: 0; right: 0; bottom: 0; left: 0; }` rule (lines 6-12) in the Ace sample.
- [18:55] Editor window (index.html — web-editor, files: index.html, main.js). In the CSS, a rule `.editor, iframe {` with `.editor` highlighted (word selected) -> about to be renamed to `#editor`. Visible CSS: `.row:after { content: ""; display: table; clear: both; }`, `.editor, iframe { width: 100%; height: 100%; }`, `.action { margin-left: auto; }`, `</style>`, `<script src="https://cdnjs.cloudflare.com/ajax/libs/ace/1.44.0/ace.min.j...` (cut off), `</head>`, `<body>`, nav/title/action/btnRun markup.
- [19:00] Reload of http://127.0.0.1:5500/index.html; blank page, status "Transferring data from cdnjs.cloudflare.com..."; devtools Inspector open (html, head).
- [19:01] Page loaded: tan nav bar "Web Editor" with "Run" button, Ace editor at left (dark monokai, line 1 only), white iframe area at right. Devtools tooltip "888.0px x 295.0px" over Run button. Inspector shows `<body>` with `flex` and `overflow` badges.
- [19:02] Inspector head contents: `<style id="ace-monokai">`, `error_marker.css`, `ace-tm`, `ace_editor.css`, `ace_scrollbar.css`, `<meta name="viewport" content="width=device-width, initial-scale=1">`, `<style>`, then `<script src="https://cdnjs.cloudflare.com/ajax/libs/ace/1.44.0/ace.min.js" integrity="sha512-nX2VMiv7R3QlrD1gIskPrBThnEgNbKxfzD1kOMm/cNaeHBt05SlNhNtxMeBHzcXV1DJ/Y1eTzE136o+1Ezu9/g==" crossorigin="anonymous" referrerpolicy="no-referrer"></script>`, then `<!--Code injected by Five-server-->` and `<script async data-id="five-server" ...>`. Editor was working with the CDN version.
- [19:09] Author opens cdnjs.cloudflare.com/ajax/libs/ace/1.44.0/ace.min.js in the browser ("Looked up cdnjs.cloudflare.com...") — intent: copy ace.min.js to serve locally.
- [19:12-19:14] Raw minified ace.min.js source displayed (starts `(function(){var e=function(){return this}();if(!e&&"undefined"!=typeof window&&(e=window),"undefined"==typeof requirejs){...`; later a `define("ace/lib/default_english_messages"...` region).
- [19:15] Whole page text selected (Select-All highlight) for copy.
- [19:16] Window-switch animation back to editor: index.html shows `#editor, iframe {` (renamed; blank lines above the rule), sidebar index.html, main.js.
- [19:17] In editor sidebar a new-file name input box appears (empty, focused) above index.html/main.js — creating a new file (to hold pasted ace.min.js).
- [19:30] Browser reloaded: nav bar "Web Editor" and Run, editor area blank/white (444.0px x 253.0px tooltip), devtools error badge "3"; mouse over Debugger tab.
- [19:32] Error badge "2"; page shows nav and white area with scrollbar.
- [19:33] Inspector hover highlight over `<html>` (888 x 295), badge "2".
- [19:34] Console tab. Lines:
  - `Socket is closed. Reconnect will be attempted in 1 second. <empty string>`  (fiveserver.js:1:11869)
  - `None of the "sha512" hashes in the integrity attribute match the content of the subresource at "http://127.0.0.1:5500/ace.min.js". The computed hash is "A69NNUCV6VNSppCWPW...V15Cf7oQdBJn1Hhjl2yNMfY2ycAqR8aidX6oswaufRSR3x/a1W+1MVveeNjTEIvZA==".` (index.html) — partly obscured by cursor.
  - `Uncaught ReferenceError: ace is not defined` `<anonymous> http://127.0.0.1:5500/main.js:6` (main.js:6:16), "Learn More".
  So local ace.min.js (served from 127.0.0.1:5500/ace.min.js) fails the SRI integrity check, so `ace` is undefined.
- [19:37] Editor: sidebar now has files `ace.min.js`, `index.html`, `main.js`. index.html script tag edited to `<script src="./ace.min.js" integrity="sha512-nX2VMiv7R3QlrD1gIskPrBThnEg...` (cursor after `./`). Visible markup below: `<div class="row"> <div class="column"> <div id="editor"></div>`.
- [19:44] Same line `<script src="./ace.min.js" integrity="sha512-nX2VMiv7R3QlrD1gIskPrBThnEg` with the `integrity="..."` attribute portion selected (about to be deleted since hash mismatch).
- [19:47] Browser: editor now appears (light default theme, line 1) but console shows:
  - `GET http://127.0.0.1:5500/theme-monokai.js [HTTP/1.1 404 Not Found 1ms]`
  - `GET http://127.0.0.1:5500/mode-javascript.js [HTTP/1.1 404 Not Found 1ms]`
  - warnings `Loading failed for the <script> with source "http://127.0.0.1:5500/theme-monokai.js".` and `...mode-javascript.js".` (index.html:1:1)
  - badge "2". So after removing integrity, ace loads locally, but themes/modes are loaded on demand as separate files that are missing.
- [20:03] Tab: github.com/ajaxorg/ace-builds/ (master, "package 21.10.26", folders .github, css, demo, src-min-noconflict, src-min, src-noconflict, src, types).
- [20:04] Firefox tab-list popup: tabs "127.0.0.1:5500/index.html", "Html code as IFRAME source rather than a URL - Stack Ov...", "Ace - The High Performance Code Editor for the Web", "GitHub - ajaxorg/ace-builds: Packaged version of Ace co...", "ace @ 1.44.0 - Libraries - cdnjs".
- [20:05-20:06] Back on ace.c9.io embedding page, scrolling; section "Loading Ace from a Local URL": "If you want to clone and host Ace locally you can use one of the pre-packaged versions. Just copy one of src* subdirectories somewhere into your project, or use RequireJS to load the contents of lib/ace folder as ace:" then "Loading Ace from a CDN". Author hand over mouth (thinking).
- [20:11-20:13] Author selects `ace/mode/javascript` text in line 26 of the sample.
- [20:16] New tab URL bar with typed `ace/mode/javascript` (reads roughly so; "Search with DuckDuckGo").
- [20:19] DuckDuckGo results: ace.c9.io snippet "Two files are created and placed in lib/ace/mode: one for the language mode, and one for the set of highlight rules...", GitHub ajaxorg/ace, "ace/lib/ace/mode at master · ajaxorg/ace · GitHub".
- [20:20] Back to ace.c9.io sample, `ace/mode/javascript` selected in line 26.
- [20:22] Tab popup again; cursor on "ace @ 1.44.0 - Libraries - cdnjs".
- [20:23] cdnjs.com/libraries/ace/ : "978 files", version 1.44.0; list: ace.js, ace.min.js, ext-beautify(.min).js, ext-code_lens, ext-command_bar, ext-diff, ext-elastic_tabstops_lite, ext-emmet ... (each with link/code/SRI icons).
- [21:14] cdnjs list scrolled far down: snippets/handlebars.min.js, haskell_cabal, haskell, haxe, hjson, html_elixir, html_ruby, html.js, html.min.js (author scanning for needed theme/mode files).
- [21:15] Editor window, ace.min.js open: a single huge minified line ending `...at(e.style.left),n=+t.left;Math.abs(i-n)>1&&(e.st` plus blue "Show more (447.2 KB)" button (editor truncates long lines). Sidebar: ace.min.js (selected), index.html, main.js.
- [21:19] Window-switch animation; index.html script src is back to `https://cdnjs.cloudflare.com/ajax/libs/ace/1.44.0/ace.min.j...` (link style) — author reverted to the CDN src (integrity not visible in this frame).
- [21:20] Browser: github.com/ajaxorg/ace-builds/ ; hover over commit link c9ea63c (github.com/ajaxorg/ace-builds/commit/c9ea63c0a0a436fbb477c918d6736644f77cab9f).
- [21:21] Back to ace.c9.io sample; `ace/mode/javascript` still selected.
- [21:22] StackOverflow tab: stackoverflow.com/questions/6102636/htm... answer "use html5's new attribute srcdoc (srcdoc-polyfill)" with `<iframe srcdoc="<html><body>Hello, <b>world</b>.</body></html>"></iframe>` (177 votes). A Google sign-in popup overlays (Vietnamese text); ignored.
- [21:23] Reload of 127.0.0.1:5500/index.html ("Waiting for 127.0.0.1..."): editor dark monokai, Run button; console cleared.
- [21:25] Loaded, dark monokai editor; console only `Socket is closed. Reconnect will be attempted in 1 second.` (fiveserver.js:1:11869). No 404s — with CDN version theme/mode load from the CDN base path.
- [21:26] index.html lines visible: `.action { margin-left: auto; }`, `</style>`, `<script src="https://cdnjs.cloudflare.com/ajax/libs/ace/1.44.0/ace.min.j` (URL up to `1.44.0` highlighted), `</head>`, `<body>`, nav/title/action/btnRun block, `<div class="row">` with `<div class="column"><div id="editor"></div></div>` and `<div class="column"><iframe id="render" src="" frameborder="0"></iframe></div>`, `<script src="main.js" type="module"></script>`, `</body>`.
- [21:36] main.js shown (see File states): ace.edit, setTheme monokai, setMode("ace/mode/html"), run handler `render.srcdoc = editor.value;`. Lightbulb icon on the blank line after setMode.
- [21:37] Browser console: `Socket is closed...` and warning `Layout was forced before the page was fully loaded. If stylesheets are not yet loaded this may cause a flash of unstyled content.` (node.js:416:1). Editor dark, text cursor in editor.
- [21:52] New tab URL bar typing `ac` with Firefox Suggest dropdown (unrelated history entries; not transcribed).
- [21:54] Query `ace editor` submitted; "Connected to duckduckgo.com...".
- [21:57] Results: Ace - The High Performance Code Editor for the Web (ace.c9.io), GitHub ajaxorg/ace, SourceForge Ace Editor.
- [22:01] ace.c9.io home: "THE HIGH PERFORMANCE CODE EDITOR FOR THE WEB."; nav tabs About / Embedding Guide / How-To Guide / Syntax Highlighters / API Reference / Real World Users / Tr...; "Built for Code" intro.
- [22:02-22:03] Scrolling About: demo editor with JS (`function add(x, y) {...}`), "Looking for a more full-featured demo? Check out the kitchen sink.", Features list (Syntax highlighting for over 110 languages; Over 20 themes; Automatic indent and outdent; An optional command line; Handles huge documents (four million lines seems to be the limit!); Fully customizable key bindings including vim and Emacs modes; Search and replace with regular expressions; Highlight matching parentheses; Toggle between soft tabs and real tabs; Displays hidden characters; Drag and drop text using the mouse; Line wrapping; Code folding).
- [22:04] Star 27,132 / Fork 5,263; cursor on "Embedding Guide".
- [22:06] "How-To Guide" (ace.c9.io/#nav=howto): "Working with Ace". "Configuring the editor": `ace.edit(element, { mode: "ace/mode/javascript", selectionStyle: "text" })`, `editor.setOptions({ autoScrollEditorIntoView: true, copyWithEmptySelection: true, });`.
- [22:07] `editor.setOption("mergeUndoDeltas", "always");`, `editor.setTheme(...)`, `editor.getOption("optionName");`; "See Configuring-Ace wiki page for a more detailed list of options."
- [22:09] "Changing the size of the editor": `editor.resize()`; maxLines option. "Setting Themes": `editor.setTheme("ace/theme/twilight");`.
- [22:10] "Setting the Programming Language Mode": `editor.session.setMode("ace/mode/javascript");`. Then "One Editor, Multiple Sessions".
- [22:11] `var js = ace.createEditSession("some js code"); var css = ace.createEditSession(["some", "css", "code here"]); editor.setSession(js);`. "Common Operations / Set and get content": `editor.setValue("the new text here"); editor.setValue("text2", -1); // set value and move cursor to the start of the text; editor.session.setValue("the new text here"); // set value and reset undo history; editor.getValue(); // or session.getValue`.
- [22:12] Firefox find-in-page bar, typed `g` (page at top).
- [22:13] Find bar `getVa` -> "1 of 2 matches", highlights `getValue`; also visible "Get selected text" `editor.getSelectedText();`, "Insert at cursor, emulating user input: `editor.insert("Something cool");`", "Replace text in range".
- [22:17] Back in editor, main.js: `ace/mode/html` string selected in setMode; mouse over `editor.value`. Not yet changed to getValue() in this frame.

Mistakes/fixes: (1) SRI integrity mismatch for the locally pasted ace.min.js -> integrity attribute removed; (2) local ace.min.js lacks separate theme/mode files (404s) -> abandoned, reverted to CDN script; (3) `editor.value` is not the Ace API, found via How-To guide (`getValue()`).

## File states

### index.html (as of ~21:26; line numbers not shown in editor)
```
?? (lines before .row:after not visible; see earlier segments)
    .row:after {
      content: "";
      display: table;
      clear: both;
    }


    #editor, iframe {
      width: 100%;
      height: 100%;
    }
    .action {
      margin-left: auto;
    }
  </style>
  <script src="https://cdnjs.cloudflare.com/ajax/libs/ace/1.44.0/ace.min.j⟨?? cut off; DOM at 19:02 showed integrity="sha512-nX2VMiv7R3QlrD1gIskPrBThnEgNbKxfzD1kOMm/cNaeHBt05SlNhNtxMeBHzcXV1DJ/Y1eTzE136o+1Ezu9/g==" crossorigin="anonymous" referrerpolicy="no-referrer"⟩
</head>

<body>

  <div class="nav">
    <div class="title">Web Editor</div>

    <div class="action">
      <button id="btnRun">Run</button>
    </div>
  </div>

  <div class="row">
    <div class="column">
      <div id="editor"></div>
    </div>
    <div class="column">
      <iframe id="render" src="" frameborder="0"></iframe>
    </div>
  </div>

  <script src="main.js" type="module"></script>
</body⟨?>
```
Change history this segment: `.editor, iframe {` -> `#editor, iframe {` (by 19:16, with extra blank line above); script src temporarily `./ace.min.js` with integrity attr being deleted (19:37-19:44), reverted to the cdnjs URL by 21:19.

### main.js (as of 21:36 / 22:17; no line numbers displayed)
```
const btnRun = document.getElementById("btnRun");

const render = document.getElementById("render");
//const iframedoc = render.contentDocument || render.contentWindow.documen⟨t... cut off⟩

const editor = ace.edit("editor");
editor.setTheme("ace/theme/monokai");
editor.session.setMode("ace/mode/html");

btnRun.addEventListener("click", () => {
    render.srcdoc = editor.value;
})
```
(Handler body indented 4 spaces; `})` has no semicolon. The earlier error main.js:6:16 "ace is not defined" matches `const editor = ace.edit("editor");` being line 6.)

### ace.min.js (new file, created ~19:17, in sidebar by 19:37)
Single-line minified Ace 1.44.0 build pasted from cdnjs (447.2 KB; editor shows "Show more (447.2 KB)"). Not reproduced.

### Reference: Ace embedding sample (ace.c9.io, seen 18:49)
```
 1| <!DOCTYPE html>
 2| <html lang="en">
 3| <head>
 4| <title>ACE in Action</title>
 5| <style type="text/css" media="screen">
 6|     #editor {
 7|         position: absolute;
 8|         top: 0;
 9|         right: 0;
10|         bottom: 0;
11|         left: 0;
12|     }
13| </style>
14| </head>
15| <body>
16| 
17| <div id="editor">function foo(items) {
18|     var x = "All this is syntax highlighted";
19|     return x;
20| }</div>
21| 
22| <script src="/ace-builds/src-noconflict/ace.js" type="text/javascript" charset="utf-8"></script>
23| <script>
24|     var editor = ace.edit("editor");
25|     editor.setTheme("ace/theme/monokai");
26|     editor.session.setMode("ace/mode/javascript");
27| </script>
28| </body>
29| </html>
```

## Shell & outputs
No terminal used. Browser outputs:
- [19:01, 21:25] http://127.0.0.1:5500/index.html: tan nav bar with bold serif "Web Editor" at left and a "Run" button at right; below, left half Ace editor (dark monokai when working; line number 1 only, empty), right half white empty iframe.
- [19:34] Console (verbatim): `None of the "sha512" hashes in the integrity attribute match the content of the subresource at "http://127.0.0.1:5500/ace.min.js". The computed hash is "A69NNUCV6VNSppCWPW...V15Cf7oQdBJn1Hhjl2yNMfY2ycAqR8aidX6oswaufRSR3x/a1W+1MVveeNjTEIvZA==".` and `Uncaught ReferenceError: ace is not defined <anonymous> http://127.0.0.1:5500/main.js:6` (main.js:6:16).
- [19:47] `GET http://127.0.0.1:5500/theme-monokai.js [HTTP/1.1 404 Not Found 1ms]`; `GET http://127.0.0.1:5500/mode-javascript.js [HTTP/1.1 404 Not Found 1ms]`; `Loading failed for the <script> with source "http://127.0.0.1:5500/theme-monokai.js".`; `Loading failed for the <script> with source "http://127.0.0.1:5500/mode-javascript.js".` (index.html:1:1). Editor rendered in light default theme.
- [21:25] `Socket is closed. Reconnect will be attempted in 1 second. <empty string>` (fiveserver.js:1:11869); [21:37] also warning `Layout was forced before the page was fully loaded. If stylesheets are not yet loaded this may cause a flash of unstyled content.` (node.js:416:1).

## Observations
- Author tries to self-host Ace (copy ace.min.js into the project) to avoid the CDN, but it fails twice: SRI integrity mismatch (pasted file differs from original, possibly due to editor line/size handling), and on-demand theme/mode files (theme-monokai.js, mode-javascript.js) 404 since they resolve relative to the script location. He researches the fix (ace-builds repo src-noconflict, Ace docs "Loading Ace from a Local URL", cdnjs 978-file list) and then reverts to the CDN URL.
- Workflow: heavy documentation hopping across tabs (ace.c9.io embedding and How-To guides, GitHub ace-builds, cdnjs, DuckDuckGo, StackOverflow "Html code as IFRAME source rather than a URL" -> `srcdoc`). Uses Firefox devtools Inspector/Console for debugging.
- Borrows Ace's `#editor` sizing idea and renames `.editor` to `#editor` to match `<div id="editor">`.
- Design intent: Run sets `iframe.srcdoc` from editor content; a commented-out `iframedoc = render.contentDocument || render.contentWindow.document` shows an earlier approach replaced by srcdoc. At segment end he reads the How-To guide for the correct getter (`editor.getValue()`), since `editor.value` is undefined on an Ace editor.
- Style: plain HTML/CSS/JS, no framework, `const` + arrow functions, ES module script tag, 2-space indent in HTML/CSS, 4 spaces in the JS handler.
