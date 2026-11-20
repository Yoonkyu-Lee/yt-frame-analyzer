# Segment 12 (45:48 - 53:22) - VBORE_zH5oc

Note: the editor (VS Code-like, window title "<file> - web-filemanager") shows no line-number gutter; numbers below are my own sequential counts from the top of the file, not editor-reported. No save messages are visible in this segment. Webcam overlay ignored.

## Timeline
- [45:48] Firefox on MDN page `developer.mozilla.org/en-US/docs/Web/HTML/R...` (Reference > Elements > <input> > <input type="file">), section "Using file inputs": demo "Choose file to upload [Browse...] No file selected." Text: "The accept attribute doesn't validate the types of the selected files; it provides hints for browsers..." then heading "Detecting cancellations".
- [45:49] Scrolled to the "Detecting cancellations" JS example (quoted in File states). Next "Notes": "1. You cannot set the value of a file picker from a script - doing something like the following has no effect: const input = document.querySelector("input[type=file]"); input.value = "foo";" and "2. When a file is chosen using an <input type="file">, the real path to the source file is not shown in the input's value attribute for obvious security reasons".
- [45:59]-[46:00] Cursor goes to the app switcher; switches to the editor, sidebar.js (project WEB-FILEMANAGER: detail.js, index.html, main.js, sidebar.js).
- [46:00] sidebar.js state (cursor at end of `inpUpload.click();`): `inpUpload` input with `type = "file"`; `btnUpload` button with innerHTML "Upload" and click listener calling `inpUpload.click();`; `ctn.append(btnUpload); return ctn;`.
- [46:08]-[46:11] Back to MDN, reads the cancel/change example; selects (highlights) the `change` listener block to copy it.
- [46:12] Window-switch animation to the editor.
- [46:13] sidebar.js: added line `inpUpload.multiple = true;` (cursor at end of it, after `inpUpload.type = "file";`).
- [46:42] Back on MDN (selection of change block still shown).
- [46:44]-[46:46] Firefox at http://127.0.0.1:5500/index.html; page shows "Upload" button top-left, "Detail" text on the right with a vertical divider. DevTools Console open. Console: `Socket is closed. Reconnect will be attempted in 1 second. <empty string>` (fiveserver.js:1:11869) and warning `This page is in Quirks Mode. Page layout may be impacted. For Standards Mode use "<!DOCTYPE html>". Learn More` (index.html).
- [46:48] Clicks Upload: native macOS file dialog opens in "Downloads" (see Shell & outputs).
- [46:51] Multi-selects "Untitled Design(5).png" and "Untitled Design(4).png"; cursor moves to Open button.
- [46:52] Console prints `FileList [ File, File ]`.
- [47:33]-[47:34] Editor: sidebar.js now has the MDN change handler pasted and adapted: `inpUpload.addEventListener("change", () => {` / `console.log(inpUpload.files);` / commented-out `// if (inpUpload.files.length === 1) {` ... (see File states). Cursor at end of console.log line.
- [47:42] Console expanded: `0: File { name: "Untitled Design(4).png", lastModified: 1793207740219, size: 601630, ... }` (type "image/png", webkitRelativePath ""), `1: File { name: "Untitled Design(5).png", lastModified: 1793213825944, size: 584475, ... }`, `length: 2`. Source ref `sidebar.js:10:17`.
- [47:44] Editor back on sidebar.js; cursor on `})` of click listener; no change.
- [49:48] sidebar.js refactored: new `sidebarNav` div with className "sidebar-nav"; `inpUpload`, `btnUpload`, and a `tabs` div declared together up top; `ctn.append(side|, tabs);` being typed with autocomplete popup (`sidebar function sidebar(): HTMLDivElement`, `sidebarNav`); he is typing "side" to complete `sidebarNav`.
- [50:02] Firefox reloading (X stop icon), Console cleared, page still "Upload" + "Detail".
- [50:35] Firefox Inspector: DOM `<div class="sidebar">` > `<div class="sidebar-nav" style="height: 42px;border-bottom: 1px solid black;display: flex;align-items: center;padding: 12px;">` + empty `<div></div>` (tabs), then `<div>Detail</div>` sibling. He is live-editing the inline style in the Rules panel to prototype the nav bar. Button at left with a black line under the nav. `:root { --border-color: #dadce0; }` and `* { box-sizing: border-box; }` visible.
- [50:37] sidebar.js: added `sidebarNav.append(btnUpload);` (cursor at end), `ctn.append(sidebarNav, tabs);`.
- [50:38] index.html head/style seen: `*`, `:root { --border-color: #dadce0; }`, body, `.sidebar`.
- [50:52] index.html: new rule `.sidebar-nav { height: 42px; border-bottom: 1px solid var(--border-color); display: flex; align-items: center; padding: 12px; }` (copied from devtools values, black replaced with the var).
- [50:53]-[50:56] Firefox Inspector: body flex; hovering `div.sidebar` highlights "272 x 295 Flex Item".
- [50:57]-[50:59] Back in editor; hover tooltip on `border-bottom`: "Shorthand property for setting border width, style and color. Widely available across major browsers (Baseline since 2015). Syntax: <line-width> || <line-style> || <color> MDN Reference".
- [51:00] Firefox new empty tab ("Search with DuckDuckGo or enter address").
- [51:04] Opens www.google.com with DevTools Inspector, inspecting Google's body CSS (design reference; error badge 3).
- [51:12] Back to the localhost page, Inspector.
- [51:34] Inspector: `<button style="background: #f3f5f6;border: 1px solid black;padding: 6px;">Upload</button>` inside sidebar-nav; typing a new property (autocomplete shows `border-bottom`, ... he is heading for border-radius).
- [51:49] Google page again (error badge 5).
- [51:55]-[51:57] Localhost: button style `background: #f3f5f6;border: 1px solid black;padding: 6px;border-radius: 6px;margin-left: auto;`; Upload button pushed to the right of the sidebar nav. [51:56] flipped to google.com again.
- [52:02] Added `color: #3c4043;` to the button style (devtools); the block of declarations is selected for copying.
- [52:03] Hover highlight on `html` element (1014 x 295).
- [52:04] Editor index.html: still only `.sidebar-nav` rule visible (cursor on its border-bottom line).
- [52:21] index.html now has `.btn-upload { background: #f3f5f6; border: 1px solid var(--border-color); padding: 6px; border-radius: 6px; margin-left: auto; color: #3c4043; }`; VS Code color-picker hover on `#3c4043` ("Sets the color of an element's text").
- [52:22]-[52:25] `#3c4043` selected; at [52:25] shows partial `#3c404|` while editing/selecting.
- [52:26]-[52:33] Window switching animations editor <-> Firefox. [52:27] Firefox button still default-looking (class `.btn-upload` not yet applied in JS). [52:30]-[52:33] editor scrolled with `#3c4043` selected (to cut/copy).
- [52:58]-[52:59] index.html top: `:root` now has `--border-color: #dadce0;` and `--text-color: #3c4043;`; body gains `color: var(--text-color);`.
- [53:01] Firefox Rules panel shows `body { ... color: var(--text-color) }` at inline:11; `:root` lists `--text-color: #3c4043`.
- [53:05] Editor index.html with cursor at end of `color: var(--text-color);`. (Same timestamp-ish: sidebar.js shown with hover tooltip on `document.createElement`: "(method) Document.createElement<"div">(tagName: "div", options?: ElementCreationOptions | undefined): HTMLDivElement (+2 overloads) In an HTML document, the document.createElement() method creates the HTML element specified by localName, or an HTMLUnknownElement if localName isn't recognized. MDN Reference".)
- [53:08] sidebar.js: typing `btnUpload.` on a new line after `const btnUpload = ...` (autocomplete popup open, ATTRIBUTE_NODE, CDATA_SECTION_NODE, ...); evidently about to add the class name (not yet visible).
- [53:21]-[53:22] Firefox: Upload button now styled (light gray rounded, no black border), right-aligned in nav; this indicates `btnUpload` got its class (not seen in code). Inspector shows body `color: var(--text-color)`.

## File states

### sidebar.js - end of segment (53:08)
```
 1| export function sidebar() {
 2|     const ctn = document.createElement("div");
 3|     ctn.className = "sidebar"
 4| 
 5|     const sidebarNav = document.createElement("div");
 6|     sidebarNav.className = "sidebar-nav";
 7|     const inpUpload = document.createElement("input");
 8|     const btnUpload = document.createElement("button");
 9|     btnUpload.⟨being typed, autocomplete open⟩
10|     const tabs = document.createElement("div");
11| 
12|     inpUpload.type = "file";
13|     inpUpload.multiple = true;
14|     inpUpload.addEventListener("change", () => {
15|         console.log(inpUpload.files);
16|         // if (inpUpload.files.length === 1) {
17|         //     console.log("File selected: ", elem.files[0]);
18|         // }
19|     });
20| 
21| 
22|     btnUpload.innerHTML = "Upload";
23|     btnUpload.addEventListener("click", function () {
24|         inpUpload.click();
25|     })
26| 
27|     sidebarNav.append(btnUpload);
28| 
29|     ctn.append(sidebarNav, tabs);
30|     return ctn;
31| }
```
Line numbers are mine. 4-space indent. `ctn.className = "sidebar"` has no semicolon; `})` after click listener has no semicolon.

Earlier version (46:13):
```
 1| export function sidebar() {
 2|     const ctn  = document.createElement("div");
 3|     ctn.className = "sidebar"
 4| 
 5|     const inpUpload = document.createElement("input");
 6|     inpUpload.type = "file";
 7|     inpUpload.multiple = true;
 8| 
 9|     const btnUpload = document.createElement("button");
10|     btnUpload.innerHTML = "Upload";
11|     btnUpload.addEventListener("click", function() {
12|         inpUpload.click();
13|     })
14| 
15|     ctn.append(btnUpload);
16|     return ctn;
17| }
```
(By 47:33 the editor auto-formatted: `const ctn = ...` single space, `function () {`.)

### index.html - end of segment (53:05; assembled from 50:38, 52:21, 52:58, 53:05)
```
 1| <html>
 2| 
 3| <head>
 4|   <style>
 5|     * {
 6|       box-sizing: border-box;
 7|     }
 8| 
 9|     :root {
10|       --border-color: #dadce0;
11|       --text-color: #3c4043;
12|     }
13| 
14|     body {
15|       width: 100%;
16|       padding: 0;
17|       margin: 0;
18|       display: flex;
19|       height: 100vh;
20|       color: var(--text-color);
21|     }
22| 
23|     .sidebar {
24|       height: 100%;
25|       width: 272px;
26|       border-right: 2px solid var(--border-color);
27|     }
28| 
29|     .sidebar-nav {
30|       height: 42px;
31|       border-bottom: 1px solid var(--border-color);
32|       display: flex;
33|       align-items: center;
34|       padding: 12px;
35|     }
36| 
37|     .btn-upload {
38|       background: #f3f5f6;
39|       border: 1px solid var(--border-color);
40|       padding: 6px;
41|       border-radius: 6px;
42|       margin-left: auto;
43|       color: #3c4043;
44|     }
45|   </style>
46| </head>
47| 
48| <body>
49|   <script src="main.js" type="module"></script>
50| </body>
51| 
52| </html>
```
Line numbers mine, 2-space indent nesting. Line 43 was last seen as `#3c4043` (52:33, selected); whether it was later replaced by `var(--text-color)` is not visible. Line 1 `<html>` has no doctype (Quirks Mode warning).

### MDN example seen (not his code)
```
const elem = document.createElement("input");
elem.type = "file";
elem.addEventListener("cancel", () => {
  console.log("Canceled.");
});
elem.addEventListener("change", () => {
  if (elem.files.length === 1) {
    console.log("File selected: ", elem.files[0]);
  }
});
elem.click();
```

main.js, detail.js: not shown in this segment.

## Shell & outputs
- No terminal commands.
- [46:44] Browser console: `Socket is closed. Reconnect will be attempted in 1 second. <empty string>` (fiveserver.js:1:11869); warning `This page is in Quirks Mode. Page layout may be impacted. For Standards Mode use "<!DOCTYPE html>". Learn More` (index.html). Page: left column with "Upload" button at top-left; "Detail" text at x~480 right pane; vertical divider.
- [46:48] macOS file picker: sidebar Favorites (<account home>, Desktop, Documents, Downloads, sources, lab), Media (Music, Photos, Movies). Folder Downloads, list view: Untitled Design(6).png 456 KB PNG; photo_..._y(1).jpg 119 KB; photo_..._y.jpg 173 KB; photo_..._y.jpg 119 KB; Screen Recording 2...9 at 9.43.19 PM.mp4 19.9 MB MPEG-4; Untitled Design(5).png 584 KB; Untitled Design(4).png 602 KB; Untitled Design(3).png 704 KB; Untitled Design(2).png 462 KB; Untitled Design(1).png 314 KB; Untitled Design.png 421 KB. Buttons Cancel / Open.
- [46:52] Console: `FileList [ File, File ]`.
- [47:42] Expanded: `0: File { name: "Untitled Design(4).png", lastModified: 1793207740219, size: 601630, ... }`; `1: File { name: "Untitled Design(5).png", lastModified: 1793213825944, size: 584475, ... }`; `length: 2`; `<prototype>: FileListPrototype { item: item(), length: Getter, ... }`; from `sidebar.js:10:17`.
- [53:21]-[53:22] Rendered page: Upload button right-aligned in a 42px nav bar with thin bottom border; "Detail" in right pane.

## Observations
- Workflow: reads MDN docs first (input type="file", change/cancel events, `multiple`), copies the example into the editor, adapts it, tests with console.log of `inpUpload.files` before building UI.
- Technique: hidden file input (never appended to DOM) triggered by a custom button (`btnUpload` click -> `inpUpload.click()`).
- Styling workflow: prototypes CSS live in Firefox DevTools (inline style edits in Inspector), then ports values into named classes with CSS variables (`--border-color`, `--text-color`); consults google.com's DOM/CSS as a design reference (colors like #3c4043, #dadce0 are Google-style).
- Layout: body flex, `.sidebar` 272px with right border, `.sidebar-nav` 42px bar with bottom border, `tabs` div under it, "Detail" right pane; button aligned right via `margin-left: auto`.
- Style: plain functions returning DOM elements (`export function sidebar()` loaded via `<script type="module">`), verbose createElement code, 4-space JS indent, 2-space HTML/CSS nesting, commented-out leftovers kept. Live-reload dev server FiveServer at 127.0.0.1:5500. Quirks Mode warning ignored (no doctype).
