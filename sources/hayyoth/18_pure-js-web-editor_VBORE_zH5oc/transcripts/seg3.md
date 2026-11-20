# Segment 3 (05:33 - 11:31) - Hayyoth, web editor / files manager

Setup: macOS desktop; editor window titled "<file> — web-editor" (VS Code-like, sidebar "WEB-EDITOR" with index.html, main.js); Firefox at http://127.0.0.1:5500/index.html (Live Server style, console shows "fiveserver.js" socket message) with devtools docked at the bottom. Webcam overlay "Human" ignored.

## Timeline

[05:33] Firefox Inspector, hovering `<textarea style="width: 269px; height: 130px;"></textarea>` inside `div.column style="background-color:#aaa;"` in `div.row`. Tooltip "textarea 269 x 130". Page: beige nav bar "Web Editor", left column shows small light-blue highlighted textarea, right column "Column 2" / "Some text..". Rules panel: `element { width: 269px; height: 130px; }`, `* { box-sizing: border-box; }`.
[05:37] Edited live in devtools: `width: 100%; height: 130px;` - textarea now fills the left column width (to x~667). Navigation: `<textarea style="width: 100%; height: 130px;"></textarea>`.
[05:39] Devtools edit: `width: 100%; height: 100%;` -> textarea fills full left column height; page now shows a vertical scrollbar (body gets "overflow" badge) because the row height overflows.
[05:43] Rules: `width: 100%; height: 100%;` selected (about to copy it).
[05:44] Editor, index.html (no sidebar visible, window is narrow view). Bottom of file: `</style></head><body>` ... `<div class="row">`, column #aaa containing `<textarea></textarea>` (cursor line highlighted), column #bbb containing `<h2>Column 2</h2>` and `<p>Some text..</p>`, `</body>`, `</html>`.
[06:02] index.html edited: CSS rule added in `<style>`:
```
textarea {
  width: 100%;
  height: 100%;
}
```
placed after `.row:after { content: ""; display: table; clear: both; }`. Column 2 content replaced by `<iframe src="" frameborder="0"></iframe>` (cursor right after `<iframe `). Textarea tag is still `<textarea></textarea>`.
[06:03] Browser: textarea now fills left column to bottom; right column grey and empty (iframe, empty). Inspector shows `<textarea></textarea>`, rule `textarea { width: 100%; height: 100%; }`.
[06:13] Typed `<h1>Hello World</h1>` into the textarea in the browser (monospace text).
[06:14-06:15] Back to editor; index.html with iframe line, cursor in the `<iframe ` line. (Window is animating/zooming between apps.)
[06:45] Editor now on main.js (new file, sidebar shows index.html, main.js; main.js selected). Content: `console.log("main work")` with a hover tooltip "log(...data: any[]): void  The console.log() static method outputs a message to the console. MDN Reference". Note no semicolon, the cursor is inside the string after "work".
[06:46] Browser Inspector: element now `<textarea id="editor"></textarea>`; rule `textarea, iframe { width: 100%; height: 100%; }` (iframe part struck through since selector doesn't match textarea element... shown struck on "iframe"). Mouse on Console tab.
[06:49] Console tab: `Socket is closed. Reconnect will be attempted in 1 second. <empty string>` (fiveserver.js:1) and `main work` (main.js). So main.js is loaded via script tag.
[06:50] Editor main.js, `"main work"` text selected (highlight).
[06:51] Browser Console, empty prompt.
[06:52] Inspector hover on `textarea#editor` tooltip 444 x 379.
[06:53] Inspector selected `<div class="title">Web Editor</div>` inside `<div class="nav">` (nav has "flex" badge).
[06:58] macOS menu bar visible with Control Center open (Firefox frontmost; clock "Fri Oct 30 8:36 PM"); Do Not Disturb On, Bluetooth off, AirDrop off. Not relevant to code.
[07:00] Same Inspector state, mouse on page.
[07:47] Devtools live-edit on title: element style `font-weight: bold;/*! color: black; */ /*! text-align: center; */` (color and text-align struck out/disabled). The title renders bold "Web Editor".
[07:48] Editor main.js, hand cursor over sidebar (index.html / main.js), `"main work"` still selected.
[08:01] Editor index.html: CSS now includes `.title { font-weight: bold; }` (cursor after `bold`). Visible CSS (see File states): `* {box-sizing}`, `body {margin 0; height 100vh; display flex; flex-flow column}`, `.nav {height 42px; display flex; align-items center; padding 12px; background wheat}`, `.title`, `.row {height:100%}`, `.column {`.
[08:02] Browser: title bold; `.title { font-weight: bold; }` rule in Rules panel; textarea contains `<h1>Hello World</h1>|`; Inspector shows `<script src="main.js" type="module"></script>` after `div.row` in body. Tooltip "888.0px x 421.0px".
[08:03] Editor index.html again, "bold" highlighted.
[08:04] Editor main.js selected (blue outline on sidebar item), `console.log("main work")`.
[08:26] main.js rewritten:
```
const editor = document.getElementById("editor");
const render = document.getElementById("render");
```
and a blank line with cursor on line 3.
[08:27] Browser: textarea still has `<h1>Hello World</h1>`.
[08:29] Browser: new tab/URL bar typing `js o` -> DuckDuckGo suggestions (Firefox Suggest list shows some personal bookmarks/mail items - not transcribed).
[08:31] Search query "js on change" loading ("Waiting for duckduckgo.com...").
[08:33] DuckDuckGo results for "js on change": top W3Schools "onchange Event - W3Schools" (https://www.w3schools.com/jsref/event_onchange.asp; snippet "Call a function when a user changes the selected option of a <select> element... The onchange event occurs when the value of an HTML element is changed. Tip: This event is similar to the oninput event."), then MDN "HTMLElement: change event - Web APIs | MDN" (developer.mozilla.org/en-US/docs/Web/API/HTMLElement/change_event; "The change event is fired for <input>, <select>, and <textarea> elements when the user modifies the element's value. Unlike the input event, the change event is not necessarily fired for each alteration to an element's value."), then W3Schools "jQuery change () Method".
[08:36] Back to editor main.js with the two const lines (cursor on line 3 area).
[09:02] Browser still on the search results (window animating).
[09:03] Browser back on app: nav bar now has a "Run" button at right (`div.action` element inserted after `div.title` in nav); textarea empty (page was reloaded). Inspector: `<div class="title">Web Editor</div>`, `<div class="action">...</div>`.
[09:19] Inspector: `<iframe id="render" src frameborder="0">` with `#document` -> `<html>` child. Rule shown `textarea, iframe { width: 100%; height: 100%; }`.
[09:20] Editor index.html: `.action { margin-left: auto; }` rule added; `textarea, iframe` combined rule; nav contains `<div class="action"><button>Run</button></div>`; textarea gets `id="editor"`, iframe `id="render"`.
[09:53] main.js now:
```
const editor = document.getElementById("editor");
const render = document.getElementById("render");
const btnRun = document.getElementById("btnRun");

btnRun.onclick(()=>{
    console.log(editor.value);
})
```
(first draft, uses `onclick(...)` as a call; line 5 has the call). Cursor after `;` on line 6.
[09:54] Browser: Inspector iframe `#document`; error badge "1" appears in devtools toolbar (page reloaded; Run button visible).
[10:00] Typed `hello world` in textarea. Console: `Socket is closed. Reconnect will be attempted in 1 second. <empty string>` and error:
`Uncaught TypeError: btnRun.onclick is not a function` / `<anonymous> http://127.0.0.1:5500/main.js:5` / `Learn More`.
[10:01] Editor main.js unchanged (same code) - looking at the error.
[10:06] Fix in progress: line 5 is now `btnRun.addEventListener()` with signature tooltip "addEventListener(type: keyof HTMLElementEventMap, listener: (this: HTMLElement, ev: Event | UIEvent | AnimationEvent | PointerEvent | MouseEvent | InputEvent | ToggleEvent | ... 12 more ... | WheelEvent) => any, options?: boolean | AddEventListenerOptions): void  The addEventListener() method of the EventTarget interface sets up a function that will be called whenever the specified event is delivered to the target. MDN Reference" (1/2).
[10:24] main.js: `btnRun.addEventListener("click", ()=>{` / `    console.log(editor.value);` / `})`.
[10:25] Browser: console shows only the socket message (cleared) and textarea "hello world".
[10:36] Browser: textarea "hello world hahaha"; Console: `hello world` (badge 3, main.js) and `hello world hahaha` (main.js). So clicking Run now logs editor value; no error.
[10:37-10:38] Editor main.js (same code), cursor idle.
[10:39] Browser, window slightly resized/animating, same console output.
[10:40] URL bar typing `ifr` (Firefox suggest, unrelated YouTube items).
[10:53] Search query "iframe render html from source" results: Stack Overflow "Html code as IFRAME source rather than a URL - Stack Overflow" (stackoverflow.com/questions/6102636/html-code-as-iframe-source-rather-than-a-url; snippet "Using srcdoc brings the advantage you can style the background via the parent frame for continuity without using javascript or css..."), Stack Overflow "javascript - How do I create an iframe element and set the HTML ..." (questions/10418644), W3Schools "HTML Iframes - W3Schools", MDN "<iframe> HTML inline frame element - HTML | MDN" ("This example directly renders source code in an iframe. This can be used as a technique to prevent script injection when displaying user-generated content, when combined with the sandbox...").
[10:59] Opening stackoverflow.com/questions/6102636/htm... (blank, loading).
[11:02] Stack Overflow page: "Html code as IFRAME source rather than a URL", question asks how to replace iframe src URL with HTML code; example `<iframe src="http://example.com" name="test" height="120" width="600">You need...`. 5 answers.
[11:05-11:08] Scrolling the top answer (241 votes, accepted): "You can do this with a data URL..." `<html><body>foo</body></html>` encoded as `data:text/html;charset=utf-8,%3Chtml%3E%3Cbody%3Efoo%3C/body%3E%3C/html%3E` set as the src of the iframe; "Edit: The other alternative is to do this with Javascript..." code:
```
var iframe = document.getElementById('foo'),
    iframedoc = iframe.contentDocument || iframe.contentWindow.document;

iframedoc.body.innerHTML = 'Hello world';
```
[11:10] Same, "Edit 2 (December 2017): use the Html5's srcdoc attribute ..." `<iframe srcdoc="<html><body>Hello, <b>world</b>.</body></html>"></iframe>`.
[11:24-11:31] Scrolling to comments ("13 Comments"); a Google sign-in popup (Vietnamese text "Dang nhap vao Stack Exchange bang Google" / "Tiep tuc") overlays the page. Comments read include one about IE8 data URI support, one about cross-origin frame, one about rawurlencode for PHP. Reader scrolls back up to the srcdoc answer at 11:31. (No code edits in this span.)

## File states

### index.html (state at ~09:20-09:53, end of segment; partial, scrolled views only)
Only parts visible; line numbers are not shown by the editor (no gutter).
```
?? (top of file: <!DOCTYPE html>, <html>, <head>, <style> opening not fully visible except below)
<style>
  * {
    box-sizing: border-box;
  }

  body {
    margin: 0;
    height: 100vh;
    display: flex;
    flex-flow: column;
  }

  .nav {
    height: 42px;
    display: flex;
    align-items: center;
    padding: 12px;
    background: wheat;
  }

  .title {
    font-weight: bold;
  }

  .row {
    height: 100%;
  }

  .column {
  ?? (contents of .column not visible; columns render ~50/50)
  }

  ?? (possibly .column floats / .row:after)
  .row:after {
    content: "";
    display: table;
    clear: both;
  }

  textarea, iframe {
    width: 100%;
    height: 100%;
  }
  .action {
    margin-left: auto;
  }
</style>
</head>

<body>

  <div class="nav">
    <div class="title">Web Editor</div>

    <div class="action">
      <button>Run</button>
    </div>
  </div>

  <div class="row">
    <div class="column" style="background-color:#aaa;">
      <textarea id="editor"></textarea>
    </div>
    <div class="column" style="background-color:#bbb;">
      <iframe id="render" src="" frameborder="0"></iframe>
    </div>
  </div>

  ?? (the <script src="main.js" type="module"></script> seen in Inspector at 08:02, placed after div.row)
</body>

</html>
```
Notes: the Run button's id (needed for getElementById("btnRun")) is not visible in the 09:20 frame (`<button>Run</button>`); it worked by 10:36 so id="btnRun" was added off-frame. Two-space nesting indentation in HTML/CSS. Earlier (06:02) the rule was `textarea { width: 100%; height: 100%; }` and iframe was `<iframe src="" frameborder="0"></iframe>`.

### main.js (intermediate 09:53)
```
1| const editor = document.getElementById("editor");
2| const render = document.getElementById("render");
3| const btnRun = document.getElementById("btnRun");
4|
5| btnRun.onclick(()=>{
6|     console.log(editor.value);
7| })
```
### main.js (END of segment, 10:24-10:38)
```
1| const editor = document.getElementById("editor");
2| const render = document.getElementById("render");
3| const btnRun = document.getElementById("btnRun");
4|
5| btnRun.addEventListener("click", ()=>{
6|     console.log(editor.value);
7| })
```
(No semicolon after the closing `)`; 4-space indent inside callback.)

## Shell & outputs
No terminal. Browser console outputs:
- [06:49] `Socket is closed. Reconnect will be attempted in 1 second. <empty string>` fiveserver.js:1 ; `main work` main.js
- [10:00] `Uncaught TypeError: btnRun.onclick is not a function` `<anonymous> http://127.0.0.1:5500/main.js:5`
- [10:36] `hello world` (x3) main.js ; `hello world hahaha` main.js
Page look at end: beige nav "Web Editor" (bold) left, "Run" button right; left column white textarea containing typed text; right column grey (#bbb) empty iframe.

## Observations
- Workflow: prototype layout in devtools (live-editing inline widths/heights), then copy the final values back into the source (textarea width/height 100% moved from inline to a `textarea` CSS rule, then merged with iframe).
- Builds incrementally: layout (nav + two columns) -> textarea + iframe -> main.js with element lookups by id -> Run button -> click handler logging editor value -> researching how to put HTML into an iframe (srcdoc / data URL / contentDocument.body.innerHTML).
- Uses web searches (DuckDuckGo) for "js on change", "iframe render html from source"; reads W3Schools/MDN snippets and a Stack Overflow answer (srcdoc, data URL, iframe.contentDocument) rather than recalling API from memory.
- Mistake/fix: used `btnRun.onclick(()=>{...})` (calling instead of assigning) -> TypeError "btnRun.onclick is not a function"; fixed by switching to `addEventListener("click", ...)`, with editor autocomplete tooltip as reference.
- Style: no semicolons in some places (console.log("main work")), semicolons later; ES module script (`type="module"`), `const` element handles named editor/render/btnRun; no framework.
- The chosen editor-to-preview mechanism is still undecided at 11:31 (reading srcdoc vs innerHTML options).
