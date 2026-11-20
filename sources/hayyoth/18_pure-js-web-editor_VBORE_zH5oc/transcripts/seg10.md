# Segment 10 (34:40 - 41:37) - VBORE_zH5oc

Setup: macOS-style desktop, editor window titled "<file> - web-filemanager" (VS Code-like, no line numbers visible, gutter hidden), Firefox with devtools, served from http://127.0.0.1:5500/index.html (Five Server live-reload; Firefox console shows "Socket is closed. Reconnect will be attempted in 1 second."). Webcam overlay ignored. Editor save messages: none visible (no status bar).

## Timeline

[34:40] Empty gray desktop, only a tiny editor-window thumbnail at left (Mission Control / window-switch animation).
[34:41] Editor window "main.js - web-filemanager". Explorer (WEB-FILEMANAGER): `index.html`, `main.js` (selected). Editor content (single line): `console.log("main work")` ("main work" string highlighted/selected).
[34:44-34:46] Switch to Firefox: http://127.0.0.1:5500/index.html, blank white page.
[34:46] Back to editor; `index.html` highlighted in explorer, still showing main.js `console.log("main work")`.
[34:58] Firefox with devtools Inspector open. DOM: `<html> event`, `<head>...</head>`, `<body>` selected, `<script id="main.js" type="module"></script>`, `<script async data-id="five-server" data-file="/Users/<user>/lab/web-filemanager/index.html" type="application/javascript" src="/fiveserver.js"></script>`, `</body>`, `<!--Code injected by Five-server-->`, `</html>`.
[35:03] Editor still main.js; index.html selected in explorer (window animating).
[35:06] index.html opened. Content:
```
<html>
    <body>
        <script id="main.js" type="module"></script>
    </body>
</html>
```
(bug: attribute is `id=` not `src=`).
[35:10] index.html edited: `<script src="main.js" type="module"></script>` (`id` changed to `src`; "main.js" shown as link).
[35:11] Firefox Inspector now shows `<head></head>` (empty), `<script src="main.js" type="module"></script>`.
[35:16] Editor index.html with src fix; I-beam cursor near `</body>`. Firefox thumbnail shows console with yellow warning.
[37:07] Editor main.js; explorer now: `index.html`, `main.js` (selected), `render.js`, `sidebar.js`. main.js:
```
console.log("main work")

const body = document.body;
```
[37:08] render.js open (content `function render() {` / blank / `}`) with confirm dialog: "Are you sure you want to delete 'render.js'? You can restore this file from the Trash." Checkbox "Do not ask me again", buttons "Move to Trash" / "Cancel".
[37:09] Checkbox "Do not ask me again" ticked; cursor on Move to Trash.
[37:10] render.js deleted; explorer: index.html, main.js, sidebar.js. main.js unchanged.
[37:31] main.js rewritten: line 1 `import { sidebar } from "./sidebar.js";`, blank, `const body = document.body;`, caret on empty next line. (console.log line removed.)
[37:32-37:33] Switch to Firefox. Console (after reload): 
 - `Socket is closed. Reconnect will be attempted in 1 second. <empty string>` (fiveserver.js:1:11869)
 - Warning: `This page is in Quirks Mode. Page layout may be impacted. For Standards Mode use "<!DOCTYPE html>". Learn More` (index.html)
 - Error: `Uncaught SyntaxError: The requested module 'http://127.0.0.1:5500/sidebar.js' doesn't provide an export named: 'sidebar'` (main.js:1:10). Error badge count 1.
[37:38] sidebar.js:
```
function sidebar() {
    const ctn  = document.createElement("div");
    ctn.innerHTML = "Sidebar";

    return ctn;
}
```
(note double space in `const ctn  =`).
[37:42] Fix: `export function sidebar() {`; sidebar.js shows "2" problems badge (red squiggles) while typing.
[37:43-37:44] Window-switch animation to Firefox; console cleared of error: only Socket + Quirks warning.
[37:46] sidebar.js: `export function sidebar() {` with `sidebar` identifier highlighted; lightbulb icon at gutter.
[37:54] Firefox shows page text (rendered as a string): `function sidebar() { const ctn = document.createElement("div"); ctn.innerHTML = "Sidebar"; return ctn; }` - because main.js appended the function itself (`append(sidebar)`), not its result.
[37:55] main.js:
```
import { sidebar } from "./sidebar.js";

const body = document.body.append(sidebar);
```
(cursor on `append`).
[37:58] Fix: `const body = document.body.append(sidebar());` (lightbulb shown).
[37:59] Firefox renders "Sidebar" (plain serif text, top-left).
[38:07-38:08] Firefox "Sidebar" text selected; animation back to editor.
[38:09] main.js: `const body = document.body.append(sidebar());`, new empty line below, caret at start of line 4.
[38:45] Firefox (page loading, reload X icon) shows "Sidebar" and below it "Detail" (two lines) - a detail module added.
[39:11] Explorer now: `detail.js`, `index.html`, `main.js` (selected), `sidebar.js`. main.js:
```
import { detail } from "./detail.js";
import { sidebar } from "./sidebar.js";

const body = document.body.append(
    sidebar(), detail()
);
```
[39:28] index.html (with styling added):
```
<html>

<head>
  <style>
    * {
      box-sizing: border-box;
    }
    body {
      width: 100%;
      padding: 0;
      margin: 0;
      display: flex;
    }
  </style>
</head>

<body>
  <script src="main.js" type="module"></script>
</body>

</html>
```
[39:29] Firefox transition: page shows "SidebarDetail" on one line; Inspector shows `<body flex>`, `<script src="main.js" type="module"></script>`, `<div>Sidebar</div>`, `<div>Detail</div>`; Rules: `body { width: 100%; padding: 0; margin: 0; display: flex; }` (inline:5), `* { box-sizing: border-box; }` (inline:2).
[39:30] Same Inspector view, page "SidebarDetail" (flex row, divs adjacent, no widths).
[39:48] Experimenting live in devtools: Sidebar div edited to `<div style="width: 272px;border-right: 1px solid black;height: 100%;">Sidebar</div>`; Rules element pane: `width: 272px; border-right: 1px solid black; height: 100%;`. Body highlighted as Flex Container 1014 x 19.2 ("Sidebar" | "Detail" with the divider at x~480).
[39:50] Page: "Sidebar" | "Detail" with caret; hovering "Click to toggle the Flexbox highlighter" on `display: flex;`.
[40:00] In devtools body rule a new property typed: `height: 100vh;` (green added-marker); page now shows full-height vertical divider line between Sidebar and Detail.
[40:01] Window switch to editor; index.html unchanged yet (cursor after `display: flex;`).
[40:02] Editor index.html with `height: 100vh;` copied into body rule after `display: flex;` (see states). Window animating.
[40:07] Firefox after reload (devtools changes lost): "SidebarDetail" jammed together again; Inspector body rule includes `height: 100vh;`.
[40:08] Hovering `<html>` in Inspector: overlay "html 1014 x 295".
[40:09] Hovering Sidebar div: "div 48.8833 x 295 Flex Item" (sidebar shrink-to-fit width, full height).
[40:10-40:15] Back in editor, index.html (body rule has `height: 100vh;`), cursor at end of `padding: 0;` line.
[40:48] Firefox: Sidebar | Detail with vertical divider; DOM now `<div class="sidebar">Sidebar</div>`, `<div>Detail</div>` (class `sidebar` added in sidebar.js, not visible; the CSS `.sidebar` rule was applied).
[41:01] URL bar dropdown: typed `google.com/` (autocomplete suggestions: Google, Google Translate, Google Gemini, ...). Search engine shown as DuckDuckGo.
[41:02] URL `www.google.com` loading ("Looking up www.google.com...").
[41:06] google.com rendered with devtools Inspector inspecting the Google page (body jsmodel="hspDDf"); error badge 1. He is looking at how Google uses CSS (color-picker eyedropper activated at 41:07, shows #ffffff, badge 3).
[41:07] Back to http://127.0.0.1:5500/index.html; Inspector: `<div class="sidebar">Sidebar</div>`, `<div>Detail</div>`; body rule intact (`height: 100vh`).
[41:12] Editor index.html now has a `.sidebar` rule (see states); text `black` is highlighted/selected in `border-right: 1px solid black;` (about to replace color with a variable).
[41:24] Window switch animation to Firefox.
[41:29] Firefox index.html unchanged view.
[41:30] URL bar: typed `cc` (suggestions list; Firefox Suggest items - private page titles omitted).
[41:32] Search query `css variable` ("Performing a TLS handshake to duckduckgo.com...").
[41:34] DuckDuckGo results: W3Schools "CSS Variables - The var () function - W3Schools" (www.w3schools.com/css/css3_variables.asp); Mozilla Developer "Using CSS custom properties (variables) - MDN"; W3Schools "HTML & CSS Custom Properties"; GeeksforGeeks "CSS Variables".
[41:36] Opens W3Schools page "CSS Variables - var() Function" (www.w3schools.com/css/css3_variables.asp). Text: "The var() function is used to insert the value of a CSS variable." "CSS variables have access to the DOM, which means that you can create variables with local or global scope, change the variables with JavaScript, and change the variables based on media queries." "A good way to use CSS variables is when it comes to the colors of your design. Instead of copy and paste the same colors over and over again, you can place them in variables."
[41:37] Scrolled to "CSS Declare a Variable": "CSS variables can have a global or local scope." "Global variables can be accessed through the entire document, while local variables can be used only inside the selector where it is declared." "To create a global variable, declare it inside the :root selector. The :root selector matches the document's root element." "To create a local variable, declare it inside the selector that is going to use it." "A CSS variable name must begin with two dashes (--) and is case sensitive!" Syntax example: `:root { --primary-bg-color: green; /* global scope */ }` / `.note { --note-bg: yellow; /* local scope */ }`.

## File states (end of segment, 41:12 for editor)

No line numbers are shown in the editor; numbers below are inferred sequential line counts.

index.html (as of 41:12)
```
 1| <html>
 2| 
 3| <head>
 4|   <style>
 5|     * {
 6|       box-sizing: border-box;
 7|     }
 8|     body {
 9|       width: 100%;
10|       padding: 0;
11|       margin: 0;
12|       display: flex;
13|       height: 100vh;
14|     }
15|     .sidebar {
16|       height: 100%;
17|       width: 272px;
18|       border-right: 1px solid black;
19|     }
20|   </style>
21| </head>
22| 
23| <body>
24|   <script src="main.js" type="module"></script>
25| </body>
26| 
27| </html>
```
(2-space indentation from 39:28 onward; the 35:06 version used 4-space indentation and no head. At 35:10 the script line was `<script src="main.js" type="module"></script>`.) No `<!DOCTYPE html>`, hence Quirks Mode warning.

main.js (as of 39:11)
```
1| import { detail } from "./detail.js";
2| import { sidebar } from "./sidebar.js";
3| 
4| const body = document.body.append(
5|     sidebar(), detail()
6| );
```
(Intermediate 38:09: `import { sidebar } from "./sidebar.js";` / blank / `const body = document.body.append(sidebar());`.)

sidebar.js (as of 37:46; probably later changed to add `ctn.className`/`classList` "sidebar" by 40:48 - not seen)
```
1| export function sidebar() {
2|     const ctn  = document.createElement("div");
3|     ctn.innerHTML = "Sidebar";
4| 
5|     return ctn;
6| }
```
Note the rendered DOM at 40:48 shows `<div class="sidebar">`, so a class assignment line was added; its exact code is ?? (not visible).

detail.js: exists in explorer from 39:11; content ?? (not visible). Renders `<div>Detail</div>`.

render.js: created earlier (`function render() {` / `}`), deleted at 37:10.

## Shell & outputs

No terminal used. Browser-rendered outputs:
- 34:44 http://127.0.0.1:5500/index.html: blank white page.
- 37:33 Console errors listed above (SyntaxError missing export `sidebar`).
- 37:54 page text: `function sidebar() { const ctn = document.createElement("div"); ctn.innerHTML = "Sidebar"; return ctn; }`.
- 37:59 page text: `Sidebar`.
- 38:45 page text: `Sidebar` newline `Detail` (block divs, before flex).
- 39:29 page text: `SidebarDetail` (flex row, no gaps).
- 40:00 page with live-edited style: Sidebar left column 272px with 1px black right border, full viewport height, Detail on right.
- 40:07-40:09 devtools hover: `html 1014 x 295`, `div 48.8833 x 295 Flex Item`.
- Console messages persistent: `Socket is closed. Reconnect will be attempted in 1 second. <empty string>` fiveserver.js:1:11869; `This page is in Quirks Mode...`.
- 41:34 DuckDuckGo results for "css variable"; 41:36-41:37 W3Schools page described above.

## Observations

- Build order: static HTML shell -> main.js entry importing component modules (sidebar.js, detail.js) -> each component is a function returning a DOM element created with `document.createElement("div")` -> main appends them -> layout via CSS (body flex, 100vh, sidebar fixed width 272px with right border).
- Components are plain ES modules (`export function`, `<script type="module">`); no framework. Deleted a scratch `render.js` (ticked "Do not ask me again" on the trash dialog).
- Mistakes and fixes made live: `id="main.js"` instead of `src`; missing `export`; appending the function instead of calling it (`append(sidebar)` -> `append(sidebar())`).
- Uses Firefox devtools to prototype CSS (inline style on the Sidebar div, `height: 100vh` on body) and then copies the result into the source; edits lost on reload, so he re-types into index.html and moves the inline style into a `.sidebar` class rule.
- Ignores the Quirks Mode warning (no DOCTYPE).
- Inspected google.com in devtools for reference (eyedropper), then researched "css variable" on W3Schools to plan using `:root` CSS variables for colors (next step: replace `black` with a variable).
- Style: 4-space indent in JS, 2-space in HTML/CSS, semicolons used, `const body = ...append(...)` naming quirk, double-space typo `ctn  =` left in place.
