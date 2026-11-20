# Segment 5 (16:07 - 18:48) - VBORE_zH5oc

Note: the code editor (a macOS VS Code-like window titled "<file> - web-editor", sidebar WEB-EDITOR with index.html, main.js) shows NO line-number gutter and no save messages. Line numbers below for main.js are inferred (confirmed by the Firefox console error `main.js:6:16`). Webcam overlay "Human" ignored.

## Timeline
- [16:07-16:11] Firefox on ace.c9.io (Ace "Built for Code" About page: demo editor with `/** In fact, you're looking at ACE right now...`, `function add(x, y)` sample). Hover on "Embedding Guide" tab, click it; URL becomes `ace.c9.io/#nav=embedding`. Page "Embedding Ace in Your Site": "Ace can be easily embedded into a web page. Get prebuilt version of ace from ace-builds repository and use the code below:" showing a 29-line sample HTML (see Shell & outputs). Scrolls down to show lines 1-29 and "Now check out the How-To Guide...".
- [16:15] Switch to editor, main.js open (text cursor mid-word in `srcdoc`). Contents: three getElementById consts, a commented-out `//const iframedoc = render.contentDocument || render.contentWindow.documen...`, and the btnRun click listener setting `render.srcdoc = editor.value;`.
- [16:16] index.html open, scrolled to `<body>`. Textarea-based layout: `<textarea id="editor"></textarea>` inside first `.column`; iframe `#render` in second column; `<script src="main.js" type="module">`.
- [16:21-16:32] Back in Ace docs. At 16:21/16:22 he selects line 22 (`<script src="/ace-builds/src-noconflict/ace.js" type="text/javascript" charset="utf-8"></script>`); at 16:27-16:32 he selects lines 24-26 (`var editor = ace.edit("editor"); editor.setTheme("ace/theme/monokai"); editor.session.setMode("ace/mode/javascript");`) - copying them. Highlighting in the doc now shows fold arrows.
- [16:38] main.js (cursor on main.js in sidebar): still the old version (`const editor = document.getElementById("editor");` first line). Then back to Ace docs (16:56 same selection of lines 24-26 still active).
- [17:00] main.js edited: the first line `const editor = document.getElementById("editor");` is REMOVED and the pasted Ace init lines placed after the commented iframedoc line (see File states): `const editor = ace.edit("editor");`, `editor.setTheme("ace/theme/monokai");`, `editor.session.setMode("ace/mode/javascript");`, a blank current line, then the btnRun listener (unchanged, still `render.srcdoc = editor.value;`). Mouse hovering over index.html in sidebar.
- [17:07] Window-switch animation showing index.html: textarea replaced by `<div id="editor"></div>` inside first column (line highlighted).
- [17:08] Firefox `http://127.0.0.1:5500/index.html`: tan nav bar with bold "Web Editor" at left and "Run" button at right; empty white body (no editor).
- [17:14] index.html: `<div id="editor"></div>` with matching div tags highlighted.
- [17:24] Firefox devtools Console open. Warning: `Loading failed for the <script> with source "http://127.0.0.1:5500/ace-builds/src-noconflict/ace.js".` (index.html:54:90). Error: `Uncaught ReferenceError: ace is not defined` `<anonymous> http://127.0.0.1:5500/main.js:6` (main.js:6:16). (He had pasted the doc's `/ace-builds/src-noconflict/ace.js` path; no such local file.)
- [17:26-17:29] Back to Ace docs, reading: "Loading Ace from a Local URL" ("If you want to clone and host Ace locally you can use one of the pre-packaged versions. Just copy one of src* subdirectories somewhere into your project, or use RequireJS to load the contents of lib/ace folder as ace:") and "Loading Ace from a CDN" ("The packaged version can also be loaded from CDN's such as jsDelivr or cdnjs.").
- [17:33] Hovering "pre-packaged versions" link (status bar github.com/ajaxorg/ace-builds/); then clicks cdnjs link: `http://cdnjs.com/libraries/ace/` loading ("Waiting for cdnjs.com...").
- [17:34] cdnjs page: "ace @ 1.44.0", "Ace (Ajax.org Cloud9 Editor)", "BSD licensed https://ace.c9.io/", "Keywords: code, editor", URL row `https://cdnjs.cloudflare.com/ajax/libs/ace/1.44.0/ace.js`, "978 files", version dropdown 1.44.0, file list ace.js, ace.min.js, ext-beautify.js...
- [17:41] cdnjs scrolled; ace.min.js row shows a checkmark (script tag copied). In index.html (17:41) the old head line `<script src="/ace-builds/src-noconflict/ace.js" type="text/javascript" ch...` has `builds` highlighted (about to be replaced).
- [17:42] index.html scrolled right on that line: `...zu9/g==" crossorigin="anonymous" referrerpolicy="no-referrer"></script>` - the cdnjs `<script>` tag with SRI integrity hash pasted.
- [17:47-17:49] cdnjs list; then Ace docs again, hovering "cdnjs" link.
- [17:51] Firefox 127.0.0.1:5500 devtools Console: `Socket is closed. Reconnect will be attempted in 1 second. <empty string>` (fiveserver.js:1:11869); warning `Layout was forced before the page was fully loaded. If stylesheets are not yet loaded this may cause a flash of unstyled content.` (node.js:416:1). Page reloaded by live server.
- [17:56-18:03] Inspector tab: `<html>`, `<head>`, `<body>` (flex, overflow) > `<div class="nav">` (flex) > `div.title` "Web Editor", `div.action`; `<div class="row">` with two `div.column` + `::after`; `<script src="main.js" type="module">`. Tooltips: body 888 x 295 Flex Container; div.nav 888 x 42; div.action 35.9833 x 22; div.row 888 x 253. At 18:01-18:03 the first column contains `<div id="editor" class=" ace_editor ace_hidpi ace-monokai ace_dark">`; tooltip `div#editor.ace_editor.ace_hidpi.ace-monokai.ace_dark 444 x 0`: the editor div has ZERO height. Rules: `.ace_editor { position: relative; overflow: hidden; padding: 0; font: 12px/normal 'Monaco', 'Menlo', 'Ubuntu Mono', 'Consolas', 'Source Code Pro', 'source-code-pro', monospace; direction: ltr; text-align: left; -webkit-tap-highlight-color: rgba(0,0,0,0); forced-color-adjust: none; }` (inline:19), `.ace-monokai { background-color: #272822; color: #F8F8F2; }`, `* { box-sizing: border-box; }` (inline:2).
- [18:09] He adds inline style `height: 100%;` to the editor element via devtools: editor renders as dark monokai pane with gutter "1" and cursor in left half (444 x 253); right half white (iframe).
- [18:10-18:14] Devtools adding another property (empty input box in element rule); style attr reads `style="height: 100%;"`. Then switches to editor; index.html shown scrolled right at the long script line (18:13).
- [18:14] index.html CSS: selector now `.editor, iframe {` (was `textarea, iframe {` earlier; see 17:41 frame which still has `textarea, iframe`). The head ends `</style>`, `<script src="https://cdnjs.cloudflare.com/ajax/libs/ace/1.44.0/ace.min.j...`, `</head>`. (Selector `.editor` is a class selector but the element uses `id="editor"` - will not match.)
- [18:22-18:23] Page reload animation; Firefox status "Transferring data from cdnjs.cloudflare.com...", Inspector shows only `<html>`/`<head>` while loading.
- [18:30-18:39] Reload finished: editor pane blank again (no height). Inspector: head contains `<style id="ace-monokai">`, `<style id="error_marker.css">`, `<style id="ace-tm">`, `<style id="ace_editor.css">`, `<style id="ace_scrollbar.css">`, `<meta name="viewport" content="width=device-width, initial-scale=1">`, `<style>`, and `<script src="https://cdnjs.cloudflare.com/ajax/libs/ace/1.44.0/ace.min.js" integrity="sha512-nX2VMiv7R3QlrD1gIskPrBThnEgNbKxfzD1kOMm/cNaeHBt05SlNhNtxMeBHzcXV1DJ/Y1eTzE136o+1Ezu9/g==" crossorigin="anonymous" referrerpolicy="no-referrer"></script>` (hash read from small text; some chars uncertain). Body: div.nav, div.row > div.column > `div#editor.ace_editor...` 444 x 0 again.
- [18:41-18:42] Back to index.html: `.editor` token of `.editor, iframe {` selected/highlighted (about to fix; fix not visible in this segment).
- [18:45] Firefox: editor area blank, devtools has editor div selected.
- [18:46] Firefox "List all tabs" menu: `127.0.0.1:5500/index.html`; `Html code as IFRAME source rather than a URL - Stack Ov...`; `Ace - The High Performance Code Editor for the Web`; `GitHub - ajaxorg/ace-builds: Packaged version of Ace code...`; `ace @ 1.44.0 - Libraries - cdnjs`.
- [18:47-18:48] Switches to Ace docs tab (embedding page; scrolling); end of segment, editor still zero height.

## File states (as of 18:48)

### main.js (line numbers inferred; editor gutter hidden)
```
1| const btnRun = document.getElementById("btnRun");
2| 
3| const render = document.getElementById("render");
4| //const iframedoc = render.contentDocument || render.contentWindow.documen?? (cut off at window edge)
5| 
6| const editor = ace.edit("editor");
7| editor.setTheme("ace/theme/monokai");
8| editor.session.setMode("ace/mode/javascript");
9| 
10| btnRun.addEventListener("click", () => {
11|     render.srcdoc = editor.value;
12| })
```
Earlier version (16:15-16:38, before Ace paste):
```
1| const editor = document.getElementById("editor");
2| 
3| const btnRun = document.getElementById("btnRun");
4| 
5| const render = document.getElementById("render");
6| //const iframedoc = render.contentDocument || render.contentWindow.documen...
7| 
8| 
9| btnRun.addEventListener("click", () => {
10|     render.srcdoc = editor.value;
11| })
```
(`editor.value` is a leftover from the textarea; an Ace editor needs `editor.getValue()`. Not yet fixed in this segment.)

### index.html (partial; top of file / head start / .nav and .row CSS not visible)
```
?? (top of file, <head>, first CSS rules not visible; `* { box-sizing: border-box; }` is at inline:2)
   .column {
     float: left;
     width: 50%;
     height: 100%;
   }

   .row:after {
     content: "";
     display: table;
     clear: both;
   }

   .editor, iframe {          <- was `textarea, iframe {`; .editor does not match id="editor"
     width: 100%;
     height: 100%;
   }
   .action {
     margin-left: auto;
   }
 </style>
 <script src="https://cdnjs.cloudflare.com/ajax/libs/ace/1.44.0/ace.min.js" integrity="sha512-nX2VMiv7R3QlrD1gIskPrBThnEgNbKxfzD1kOMm/cNaeHBt05SlNhNtxMeBHzcXV1DJ/Y1eTzE136o+1Ezu9/g==" crossorigin="anonymous" referrerpolicy="no-referrer"></script>
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
</body>

</html>
```
Before 17:07 the editor column held `<textarea id="editor"></textarea>`. Between 17:41 and 17:42 the head line `<script src="/ace-builds/src-noconflict/ace.js" type="text/javascript" charset...>` was replaced by the cdnjs ace.min.js tag.

## Shell & outputs
No terminal. Browser outputs:
- 17:08 page http://127.0.0.1:5500/index.html: tan nav bar, "Web Editor" bold serif at left, "Run" button at right; empty white area.
- 17:24 Console: warning `Loading failed for the <script> with source "http://127.0.0.1:5500/ace-builds/src-noconflict/ace.js".` (index.html:54:90); error `Uncaught ReferenceError: ace is not defined <anonymous> http://127.0.0.1:5500/main.js:6` (main.js:6:16).
- 17:51 Console: `Socket is closed. Reconnect will be attempted in 1 second. <empty string>` (fiveserver.js:1:11869); `Layout was forced before the page was fully loaded. If stylesheets are not yet loaded this may cause a flash of unstyled content.` (node.js:416:1).
- 18:09 after devtools `height: 100%`: left column shows dark Ace editor with "1" gutter; right half white.
- Ace docs sample (read 16:11):
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
10|        bottom: 0;
11|        left: 0;
12|    }
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

## Observations
- He integrates Ace into his hand-built editor by reading the official embedding docs: copies script tag and the three init lines (24-26), adapting var to const in main.js.
- Debugging is console-first: `ace is not defined` -> doc script path was relative to Ace's own site -> "Loading Ace from a CDN" -> cdnjs.com -> ace.min.js 1.44.0 tag with SRI.
- Uses devtools Inspector to diagnose layout: found `#editor` 444 x 0 (zero height); proved the fix live with inline `height: 100%`, then tried to port it into CSS by renaming `textarea, iframe` to `.editor, iframe` (class selector on an id element; still failing at segment end).
- Latent bug: `render.srcdoc = editor.value` still uses the textarea API.
- Uses Five Server live reload (fiveserver.js) at 127.0.0.1:5500 in Firefox; a StackOverflow tab ("Html code as IFRAME source rather than a URL") is the source of the srcdoc approach, with the commented iframedoc/contentDocument alternative left in place.
- Style: 2-space HTML indentation, 4-space JS indentation; leaves commented-out alternatives.
