# Segment 15 (63:43 - 67:37) - VBORE_zH5oc

## Timeline
- [63:43-63:47] Firefox on stackoverflow.com/questions/67164994/dif... ("Differentiate a video or image on file upload JS"). A Google sign-in popup overlays the page. Question code scrolls past (FileReader onload, Toast.fire "Uploading your File...", socket.emit('message', `<video controls autoplay="1" alt="Video Uploaded"...`, `<img alt="Image Uploaded" style="cursor: zoom-in;...`, `reader.readAsDataURL(input.files[0]);`). Comment: "Test the file extension?" "mp4, avi, mkv etc --> video; jpg, jpeg, png etc --> image". Accepted answer: "the reader.result contains MIME type of the file, from it you can detect if it's image or video or something else", with code `var match = reader.result.match(/^data:([^/]+)\/([...`, `var type = match[1];`, `var format = match[2];`, `if (type == "video") {...} else if (type == "image") {...}`.
- [63:48-63:54] Search results page, query "js check file is image video" (DuckDuckGo). Results: SO 67164994 (visited), SO 56587627 "Detect media (image, video) type by URL", GeeksForGeeks "File Type Validation while Uploading it using JavaScript", Cloudinary blog, xjavascript.com x2, npm is-image / is-video, GitHub file-type-checker. [63:51] he opens SO 56587627 (briefly), then returns to results.
- [63:58] Browser http://127.0.0.1:5500/index.html: app "HFiles" with "Upload" button, empty sidebar, empty detail area; devtools Console: `Socket is closed. Reconnect will be attempted in 1 second. <empty string>` and Quirks Mode warning `This page is in Quirks Mode. Page layout may be impacted. For Standards Mode use "<!DOCTYPE html>". Learn More`.
- [63:59-64:02] Clicks Upload; macOS file dialog (Downloads folder). Selects Untitled Design(5).png and Untitled Design(4).png, Open. Sidebar lists "Untitled Design(5).png", "Untitled Design(4).png"; console prints two blob URLs: `blob:http://127.0.0.1:5500/c2efd7d1-ed0e-4272-bcde-728cdcc878ae` and `blob:http://127.0.0.1:5500/7fab9d05-b936-4684-bcd1-1d3751335b7e`.
- [64:06] Editor (title "sidebar.js — web-filemanager"; files: detail.js, index.html, main.js, sidebar.js). Mid-edit: inside the tab click listener, empty body (cursor line) `tab.addEventListener("click", function() {` ... `})`.
- [64:16-64:18] Window-switch animation back to Firefox, Console cleared.
- [64:22-64:23] Upload dialog again; selects multiple files (photo_6320995415423563863_y.jpg, Screen Recording ...mp4, Untitled Design(4).png).
- [64:32] Sidebar shows "Untitled Design(4).png", "Screen Recording 2026-10-29 at 9.43.19 PM.mp4", "photo_6320995415423563863_y.jpg". Console (from sidebar.js:28:25, clicking tabs) logs File objects: `File { name: "Untitled Design(4).png", lastModified: 1793207740219, webkitRelativePath: "", size: 601630, type: "image/png" }` and `File { name: "photo_6320995415423563863_y.jpg", lastModified: 1793339061472, webkitRelativePath: "", size: 118914, type: "image/jpeg" }` (expanded: lastModified, name, size, type, webkitRelativePath, prototype FilePrototype getters).
- [64:45] Console typing: `const typeTest = "image/png"` -> `undefined`; then typing `t`.
- [64:57-64:58] Editor sidebar.js: click handler now has `console.log(file);`; `const urlFile = URL.createObjectURL(file);` line (the earlier `console.log(urlFile)` removed; urlFile greyed = unused).
- [65:32] Editor typing `if(file.type.startsWith("image/")) {` then `const img = document.createElement("img")` with IntelliSense tooltip for createElement(tagName: "img", ...) HTMLImageElement.
- [66:00] Top of sidebar.js visible: `export function sidebar(detailCtn) {` (new parameter detailCtn).
- [66:01] Typing `det` with autocomplete (detailCtn parameter; also `detail` from ./detail).
- [66:07] Block completed: `img.src = urlFile;` `detailCtn.append(img);`.
- [66:08-66:13] Switch to Firefox, reload, Upload dialog; selects Untitled Design(3).png, then extends to (5),(4),(3), Open.
- [66:14] Sidebar: Untitled Design(3).png, (4).png, (5).png.
- [66:15] Clicking tab: image (a macOS System Settings/Finder screenshot) appears in detail area at full size; console `File { name: "Untitled Design(3).png", lastModified: 1793176215892, webkitRelativePath: "", size: 703610, type: "image/png" }`. Sidebar squeezed (HFiles and Upload overlap, tab names wrap) because the large image shrinks the flex sidebar.
- [66:19-66:23] Inspector: `<body>` flex scroll overflow; `<script src="main.js" type="module">`, `<div class="sidebar">` (children `div.sidebar-nav` flex, `div.sidebar-tabs`), then an unclassed `<div>` containing `<img src="blob:http://127.0.0.1:5500/f95a952e-7b9f-40b9-a6a4-5af139732d31">` (hover shows `div 1280 x 295 Flex Item`). CSS: `* { box-sizing: border-box; }`, `body { color: var(--text-color) }`, `:root { --border-color: #dadce0; --text-color: #3c4043; }`.
- [66:24-66:25] Back to editor (window-switch animation); sidebar.js unchanged.
- [66:59] Editor index.html (problem badge "2" on file): editing CSS at the bottom of the `<style>`; shows `.sidebar-tab {...}`, two `.detai...` rules (first: width, heig...; second: `max-...` / `max-` being typed, autocomplete list max-width, max-height...). Body: `<body>` `<script src="main.js" type="module"></script>` `</body>`.
- [67:06] Firefox: after reload, detail is `<div class="detail"></div>` (class now exists); body rules `width: 100%; padding: 0; margin: 0; display: flex; height: 100vh; color: var(--text-color)`. Sidebar narrower, fixed.
- [67:07-67:09] Clicks Upload, dialog (Downloads), selects photo_6118596717982104515_y.jpg; sidebar shows only that file.
- [67:10] Clicking the tab shows a large screenshot (task-manager board UI) in detail. [67:14] Devtools panel resized; image shown at `1014.0px x 376.0px`; sidebar stays fixed width.
- [67:17-67:19] Reload, empty; Upload dialog again.
- [67:21-67:22] Selects photo_6320995415423563863_y.jpg, Untitled Design(5).png, Untitled Design(3).png; sidebar lists those three in that order.
- [67:23] Clicks "Untitled Design(5).png" tab: image shows a terminal screenshot with C code (hstrlen/hmemcpy, a "hello" char table) at natural size; page/detail scrolls (scrollbar). [67:24-67:27] scrolls the page (the HFiles nav goes off-screen, sidebar scrolls with the page) then back.
- [67:30] Inspector hover on `div.sidebar-tabs` 254.367 x 416; rule `.sidebar-tabs { height: 100%; overflow-y: auto; padding: 12px; }` (inline:45).
- [67:33] `.sidebar { height: 100%; width: 272px; border-right: 2px solid (swatch) var(--border-color); }` (inline:20).
- [67:37] In devtools Rules he adds `display: flex` to `.sidebar` (autocomplete flex/flow-root); layout then changes (sidebar content squeezed horizontally; body loses scroll badge, sidebar-tabs gets scroll badge).

## File states
### sidebar.js (state at ~66:07, end of edits shown)
```
 1| export function sidebar(detailCtn) {
 2|     const ctn = document.createElement("div");
 3|     ctn.className = "sidebar"
 4|
 5|     const sidebarNav = document.createElement("div");
 6|     sidebarNav.className = "sidebar-nav";
 7|
 8|     const inpUpload = document.createElement("input");
 9|
10|     const btnUpload = document.createElement("button");
11|     btnUpload.className = "btn-upload";
12|
13|     const tabs = document.createElement("div");
14|     tabs.className = "sidebar-tabs";
15|
16|     inpUpload.type = "file";
17|     inpUpload.multiple = true;
18|     inpUpload.addEventListener("change", () => {
19|         for (const file of inpUpload.files) {
20|             const urlFile = URL.createObjectURL(file);
21|
22|             const tab = document.createElement("div");
23|             tab.innerHTML = file.name;
24|             tab.className = "sidebar-tab";
25|
26|             tab.addEventListener("click", function() {
27|                 console.log(file);
28|                 if(file.type.startsWith("image/")) {
29|                     const img = document.createElement("img");
30|                     img.src = urlFile;
31|                     detailCtn.append(img);
32|                 }
33|             })
34|
35|             tabs.append(tab);
36|         }
37|     });
38|
39|
40|     btnUpload.innerHTML = "Upload";
41|     btnUpload.addEventListener("click", function () {
42|         inpUpload.click();
43|     })
44|
45|     sidebarNav.append("HFiles")
46|     sidebarNav.append(btnUpload);
47|
48|     ctn.append(sidebarNav, tabs);
49|     return ctn;
?? (closing `}` presumably line 50, not visible)
```
(Line numbers are my own count; no editor gutter numbers appear in these frames.) At 64:06 the listener had `console.log(urlFile);` after createObjectURL and an empty click body; at 64:57 the click body was `console.log(file);` only.

### index.html style fragment (state at 66:59, partial)
```
?? (earlier rules not visible)
    height: 100%;
    overflow-y: auto;
    padding: 12px;
  }

  .sidebar-tab {
    width: 100%;
    padding: 12px;
    cursor: pointer;
    font-size: 14px;
    back⟨?⟩ ...   (hidden by autocomplete popup; presumably background)
    marg⟨?⟩ ...
    bord⟨?⟩ ...
  }

  .detai⟨?⟩ {
    widt⟨?⟩
    heig⟨?⟩
  }

  .detai⟨?⟩ {
    max-⟨?⟩
    max-
  }
</style>
</head>

<body>
  <script src="main.js" type="module"></script>
</body>
```
Values after 67:06 not visible. Computed in devtools: `.sidebar {height:100%; width:272px; border-right:2px solid var(--border-color)}`, `.sidebar-tabs {height:100%; overflow-y:auto; padding:12px}`, `body {width:100%; padding:0; margin:0; display:flex; height:100vh; color:var(--text-color)}`, `* {box-sizing:border-box}`, `:root {--border-color:#dadce0; --text-color:#3c4043}`.

## Shell & outputs
No terminal commands. Browser console outputs listed in the timeline (blob URLs at 64:02; File objects at 64:32 and 66:15; `const typeTest = "image/png"` -> `undefined` at 64:45). Page look: left sidebar with bold serif "HFiles" heading + "Upload" button, light grey tab cards with serif file names, right detail pane showing the selected image. Standing console messages: "Socket is closed. Reconnect will be attempted in 1 second." (fiveserver.js:1:11869, dev server) and the Quirks Mode warning (index.html).

## Observations
- He researches how to tell image from video (Stack Overflow: parse data-URL MIME or file extension) and settles on the File object's `type` (`"image/png"`, `"image/jpeg"`), testing in the console with `const typeTest = "image/png"` before writing `file.type.startsWith("image/")`.
- Incremental test-in-browser loop: edit, switch to Firefox, reload, re-upload via file dialog each time (state not persisted), inspect DOM/CSS in devtools, then fix CSS (fixed sidebar width 272px, detail sizing/max-width/max-height) in index.html.
- Component-style plain JS: `sidebar(detailCtn)` receives the detail container and appends the preview directly (no state yet; images would accumulate in detail on each click rather than replace).
- Quirks Mode warning (missing doctype) is ignored.
- Webcam overlay ignored per instructions. Other windows appearing (terminal with C code hstrlen/hmemcpy, "hello" table) are just uploaded test images.
