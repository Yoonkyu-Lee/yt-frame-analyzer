# Segment 4 (11:31 - 16:07) - Hayyoth, pure JS web editor

Editor: a VS Code-like light-theme app, window title "main.js — web-editor" (macOS style), sidebar WEB-EDITOR with index.html and main.js. No line-number gutter is visible; no save messages visible. Browser: Firefox at http://127.0.0.1:5500/index.html with devtools docked at the bottom.

## Timeline
- [11:31] Firefox, Stack Overflow question page (stackoverflow.com/questions/6102636/htm..., "Html code as IFRAME source rather than a URL"). Scrolled to an "Edit" answer: `var iframe = document.getElementById('foo'),` / `    iframedoc = iframe.contentDocument || iframe.contentWindow.document;` / blank / `iframedoc.body.innerHTML = 'Hello world';`. Then "Edit 2 (December 2017): use the Html5's srcdoc attribute ..." with `<iframe srcdoc="<html><body>Hello, <b>world</b>.</body></html>"></iframe>`. A Google sign-in popup (Vietnamese text) overlays; ignored.
- [11:35] main.js (before the SO lookup). Full file at that time:
  ```
  const editor = document.getElementById("editor");
  const render = document.getElementById("render");
  const btnRun = document.getElementById("btnRun");

  btnRun.addEventListener("click", ()=>{
      console.log(editor.value);
  })
  ```
  Cursor after `console.log(editor.value);`.
- [12:18-12:20] Firefox: the web editor page. Textarea contains `hello world hahaha`; Run button top right of the tan nav bar; gray right pane (iframe). Devtools Console: `Socket is closed. Reconnect will be attempted in 1 second. <empty string>` (fiveserver.js:1) and a red error: `Uncaught ReferenceError: assignment to undeclared variable iframedoc` / `<anonymous> http://127.0.0.1:5500/main.js:6`, "Learn More". Layout overlay shows 888.0px x 421.0px (row) and 444.0px x 379.0px (column).
- [12:25] Back to SO (srcdoc-polyfill text).
- [12:26] main.js rewritten from the SO snippet. State:
  ```
  const editor = document.getElementById("editor");

  const btnRun = document.getElementById("btnRun");

  const render = document.getElementById("render");
  iframedoc = render.contentDocument || render.contentWindow.document;


  btnRun.addEventListener("click", () => {
      iframedoc.body.innerHTML = 'Hello world';
  })
  ```
  (render now declared after btnRun; handler sets iframedoc.body.innerHTML to 'Hello world').
- [12:30] Autocomplete popup open (console, const, ConstantSourceNode, ...). He types `cons` before `iframedoc` -> line reads `consiframedoc = render.contentDocument || render.contentWindow.document;` (mid-edit, adding `const `).
- [12:31-12:33] Window-switch animation between editor and Firefox (SO page, then the app page with console).
- [12:39] Firefox page reloaded, Console shows only the Socket line (error gone). Textarea `hello world hahaha`, empty gray pane. Layout overlay still showing.
- [12:40] Pressed Run: right pane shows "Hello world" (plain serif text). Console only socket line.
- [12:40-12:48] main.js now: `const iframedoc = render.contentDocument || render.contentWindow.document;` (iframedoc word-highlighted), `iframedoc.body.innerHTML = 'Hello world';`.
- [12:48-12:49] Animation back to Firefox; page reloaded, empty right pane, console clean.
- [13:08] Textarea: `<h1>Hello World</h1>` / `<h2>description here</h2>`; right pane renders "Hello World" (h1) and "description here" (h2) in bold serif on gray. Run works.
- [13:09] main.js click handler changed to `iframedoc.body.innerHTML = editor.value;` (cursor after `value`, before `;`). Window mid-animation.
- [13:10] Switched to index.html, scrolled to bottom; see File states. Cursor on the `<button id="btnRun">Run</button>` line.
- [13:19] Firefox: page reloaded, textarea `<h1>Hello World</h1>` / `<h2>description here</h2>`, right pane empty white. Console open.
- [14:20] Textarea now:
  ```
  <h1>Hello World</h1>
  <h2>description here</h2>
  <button onclick="run()">click me</button>

  <script>
  function run() {
    alert("Runnnnnn");
  }
  </script>
  ```
  Right pane renders heading, description, and "click me" button (blue inspector highlight over html). Inspector tab: `<div class="column">` > `<iframe id="render" src frameborder="0">` > `#document` > `<html> [event]`; red error badge "1". Breadcrumb: html > body > div.row > div.column > iframe#render > html.
- [14:21] Inspector expanded: `<html>` > `<head></head>`, `<body>...</body>`.
- [14:35] body expanded: `<h1>Hello World</h1>`, `<h2>description here</h2>`, `<button onclick="run()">click me</button> [event]`, `<script> function run() { alert("Runnnnnn"); }</script>`. Devtools pane taller (textarea now ~295px tall). Firefox "List all tabs" dropdown open: tabs `127.0.0.1:5500/index.html` and `Html code as IFRAME source rather than a UR... - Stack Ov...`.
- [14:36-14:57] SO tab; scrolled through comments and the 177-vote answer "use html5's new attribute srcdoc (srcdoc-polyfill) Docs":
  ```
  <iframe
      srcdoc="<html>
              <body>Hello, <b>world</b>.</body>
              </html>">
  </iframe>
  ```
  then "Browser support - Tested in the following browsers:" list (IE 6-11, Edge 13/14, Safari 4 ... 10, Chrome 14 ... 25.0.1364.5 (dev), Opera 11.1 ... 42, Mozilla FireFox 3.0 ... 50). Comments seen: IE8 data URI note; cross-origin header question; single-quotes question; php rawurlencode note; and (Feb 9, 2017) "Note that if you use innerHTML browser will not execute descendant script tags. For more information check Security considerations section of Element.innerHTML MDN page." (explains why script does not run via innerHTML). Commenter names omitted.
- [14:49-14:50] Switched to main.js (sidebar main.js selected); line 10 `iframedoc.body.innerHTML = editor.value;`.
- [15:05] Firefox: Inspector at 295px pane; textarea has the script content; right pane shows Hello World / description here / click me.
- [15:07] main.js: blank line 8 has cursor, file otherwise same as 15:05.
- [15:25-15:29] Firefox Inspector, right pane empty after reload. DOM tree: `<!DOCTYPE html>`, `<html> [event][scroll]`, `<head>...</head>`, `<body> [flex][overflow]`, `<div class="nav"> [flex]`, `<div class="row">`, two `<div class="column">`, iframe#render with `#document` > `<html>` > `<head></head>`, `<body></body>` (empty). Hover over div.row: "div.row 888 x 253 Flex Item".
- [15:31-15:33] SO tab again (srcdoc answer, cursor near `</body>`).
- [15:34-15:35] main.js edit (15:34 mid-animation, 15:35 stable):
  ```
  const editor = document.getElementById("editor");

  const btnRun = document.getElementById("btnRun");

  const render = document.getElementById("render");
  //const iframedoc = render.contentDocument || render.contentWindow.documen...

  btnRun.addEventListener("click", () => {
      render.srcDoc = editor.value;
  })
  ```
  Note `srcDoc` (capital D); cursor is in front of the `D`. Old iframedoc line commented out with `//` (line cut off at window edge).
- [15:39] SO tab shown briefly.
- [15:40] Firefox: empty iframe, inspector, overlay 888.0px x 295.0px / 444.0px x 253.0px.
- [15:44] After Run: an alert dialog "127.0.0.1:5500 / Runnnnnn / OK" appears (the script in the iframe ran). Inspector shows `<iframe id="render" src frameborder="0" srcdoc="<h1>Hello World</h1> <h2>description here</h2> <button oncli… <script> function run() { alert("Runnnnnn"); } </script>">` with `#document` child. So srcdoc worked (whether `srcDoc` was corrected between 15:35 and 15:44 is not visible; a DOM property `srcDoc` would not set the attribute, so likely fixed to `srcdoc`).
- [15:45] After OK: right pane renders Hello World, description here, click me. Devtools open.
- [15:48] Dialog again.
- [15:49-15:50] Dismissed; at 15:50 devtools closed; textarea full height, right pane white, vertical page scrollbar at right edge, Run button slightly clipped.
- [15:51-15:59] Clicked "click me" inside iframe: dialog with checkbox "Don't allow 127.0.0.1:5500 to prompt you again", message "Runnnnnn", OK.
- [16:01] New URL entry typing `ace` (dropdown with search suggestion and personal history entries; omitted).
- [16:02] URL bar: `ace text editor`, status "Looking up duckduckgo.com...".
- [16:04] DuckDuckGo results for "ace text editor": top "Ace - The High Performance Code Editor for the Web" (https://ace.c9.io) with sub-links Kitchen Sink, Ace API Reference, Ace Mode Creator, Ace Autocompletion Demo, Editor, Static Code Highlighter Using Ace; next "Ace API Reference" (ace.c9.io > api); next "GitHub - ajaxorg/ace: Ace (Ajax.org Cloud9 Editor)". He clicks first link; page loading "Transferring data from ace.c9.io...".
- [16:06] Ace homepage: "THE HIGH PERFORMANCE CODE EDITOR FOR THE WEB." logo, Star/Fork buttons, nav: About, Embedding Guide, How-To Guide, Syntax Highlighters, API Reference, Real World Users, T...; "Built for Code".
- [16:07] Scrolled: Star 27,132, Fork 5,263. Text: "Ace is an embeddable code editor written in JavaScript. It matches the features and performance of native editors such as Sublim[e], Vim and TextMate. It can be easily embedded in any web page and JavaScript application. Ace is maintained as the primary editor for Cloud9 IDE and is the successor of the Mozilla Skywriter (Bespin) project." Demo editor (lines 1-16):
  ```
  1  /**
  2   * In fact, you're looking at ACE right now. Go ahead and play with it!
  3   *
  4   * We are currently showing off the JavaScript mode. ACE has support for 45
  5   * language modes and 24 color themes!
  6   */
  7
  8  function add(x, y) {
  9      var resultString = "Hello, ACE! The result of your math is: ";
  10     var result = x + y;
  11     return resultString + result;
  12 }
  13
  14 var addResult = add(3, 2);
  15 console.log(addResult);
  16
  ```
  Below: "Learn how to embed this in..." and "Check out the kitchen sink."

## File states

### main.js at [15:35] (latest in segment)
```
1| const editor = document.getElementById("editor");
2|
3| const btnRun = document.getElementById("btnRun");
4|
5| const render = document.getElementById("render");
6| //const iframedoc = render.contentDocument || render.contentWindow.documen⟨cut off by window width; presumably "t;"⟩
7|
8|
9| btnRun.addEventListener("click", () => {
10|     render.srcDoc = editor.value;
11| })
```
(Line numbers inferred; no gutter shown.) The capital-D `srcDoc` is as seen; later frames show srcdoc attribute working, so a correction is likely but not visible.

### main.js intermediate [15:05-15:07] (before srcdoc)
```
1| const editor = document.getElementById("editor");
2|
3| const btnRun = document.getElementById("btnRun");
4|
5| const render = document.getElementById("render");
6| const iframedoc = render.contentDocument || render.contentWindow.document;
7|
8|
9| btnRun.addEventListener("click", () => {
10|     iframedoc.body.innerHTML = editor.value;
11| })
```

### index.html (tail only, seen at [13:10]; no line numbers shown)
```
?? (head and earlier CSS not visible)
    .action {
      margin-left: auto;
    }
  </style>
</head>

<body>

  <div class="nav">
    <div class="title">Web Editor</div>

    <div class="action">
      <button id="btnRun">Run</button>
    </div>
  </div>

  <div class="row">
    <div class="column" style="background-color: #aaa;">
      <textarea id="editor"></textarea>
    </div>
    <div class="column" style="background-color: #bbb;">
      <iframe id="render" src="" frameborder="0"></iframe>
    </div>
  </div>

  <script src="main.js" type="module"></script>
</body>

</html>
```
Indentation as seen: `<body>` col 0, `<div class="nav">` indented 2, its children 4, button 6; `.action {` at 4 inside `<style>`.

## Shell & outputs
No terminal. Browser console output:
- `Socket is closed. Reconnect will be attempted in 1 second. <empty string>` (fiveserver.js:1:11869) on each reload.
- [12:20] `Uncaught ReferenceError: assignment to undeclared variable iframedoc` / `<anonymous> http://127.0.0.1:5500/main.js:6` (module strict mode; fixed by adding `const` at 12:30).
- Alert dialogs: "127.0.0.1:5500 / Runnnnnn / OK" at [15:44], [15:48]; with checkbox "Don't allow 127.0.0.1:5500 to prompt you again" at [15:51-15:59].

Rendered page: tan nav bar with bold serif "Web Editor" at left and "Run" button at right; below two equal columns: left textarea (white, monospace), right iframe pane (gray when empty before, white once srcdoc used).

## Observations
- Approach: first `iframe.contentDocument || contentWindow.document` + `iframedoc.body.innerHTML = editor.value` copied from a Stack Overflow answer. He tested with a `<button onclick="run()">` and `<script>` in the textarea and inspected the iframe DOM in Firefox devtools; the SO comment notes innerHTML does not execute script tags.
- He switched to the `srcdoc` attribute (top-voted answer), commenting out the old `iframedoc` line instead of deleting.
- Debugging style: read console error, fix, reload via Five Server live reload; uses Inspector to confirm DOM.
- `type="module"` script caused strict-mode undeclared variable error.
- After srcdoc works and alert fires, he searches "ace text editor" and opens ace.c9.io, planning to replace the plain textarea with a real code editor.
- Style: 4-space indent in JS, 2-space in HTML; autocomplete used.
