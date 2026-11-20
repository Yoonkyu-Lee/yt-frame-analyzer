# Segment 16 of 20 (67:37 - 70:20) - VBORE_zH5oc (Hayyoth, web editor & file manager)

Note: frames are sampled; webcam overlay "Human" ignored. No editor save messages or terminal commands in this segment. Mac desktop (Firefox, code editor titled "<file> - web-filemanager", Finder, macOS file-open dialog).

## Timeline
- [67:37] Firefox at `http://127.0.0.1:5500/index.html`, page "HFiles" with an "Upload" button. The sidebar is squeezed narrow (shrunk by a large image in the detail pane); 3 uploaded tabs: `photo_6320995415423563863_y.jpg`, `Untitled Design(5).png`, `Untitled Design(3).png`. Detail pane shows the "hello" char-array diagram with a Terminal showing C code (`hstrlen`/`hmemcpy`). Devtools Inspector open, `<div class="sidebar">` selected. In Rules, `.sidebar { height: 1..; width: 27..; border-right..; display: flex` being edited (autocomplete dropdown: `flex`, `flow-root`, `flow-root list-item`). DOM: `body (flex) > script src="main.js" type="module"`, `div.sidebar (flex) > div.sidebar-nav (flex), div.sidebar-tabs (scroll badge)`, `div.detail`.
- [67:42] `.sidebar` (inline:20) now `height: 100%; width: 272px; border-right: 2px solid var(--border-color); display: flex; flex-flow: column;` (typing `column`, autocomplete `column` / `column-reverse`). Sidebar back at normal width with "HFiles" and Upload.
- [67:47] Rule rows `display: flex;` / `flex-flow: column;` highlighted, checkbox being toggled; experimenting in devtools to stop the sidebar shrinking.
- [67:48] Mission-Control style switch to the editor: `index.html - web-filemanager`, file tree: detail.js, index.html, main.js, sidebar.js.
- [67:49] index.html CSS tail visible, cursor at end of `max-height: 100%;` in `.detail > *` (see File states).
- [68:11] Back in Firefox, page reloaded: empty sidebar and empty detail. Inspector: `body (flex)` selected; `body { width: 100%; padding: 0; margin: 0; display: flex; height: 100vh; color: var(--text-color); }` (inline:11). DOM: `div.sidebar` (flex), `div.detail` (empty).
- [68:12-68:13] Clicks Upload; macOS open dialog (Downloads, list view). Files: Untitled Design(6).png 456 KB (Today 2:48 PM), photo_6320995415423563863_y(1).jpg 119 KB, photo_6118596717982104515_y.jpg 173 KB, photo_6320995415423563863_y.jpg 119 KB, Screen Recording 2...9 at 9.43.19 PM.mp4 19.9 MB MPEG-4 movie, Untitled Design(5).png 584 KB, (4) 602 KB, (3) 704 KB, (2) 462 KB, (1) 314 KB, Untitled Design.png 421 KB.
- [68:15] Multi-selected photo_6320995415423563863_y.jpg, Untitled Design(5).png, Untitled Design(3).png; clicks Open.
- [68:16] Sidebar shows the 3 tabs (serif font, light-grey rounded cards).
- [68:18-68:28] Clicks tabs in turn; detail pane shows (images at natural size, sidebar squeezed to ~460px wide when image wide): Untitled Design(5).png = "hello" array diagram + Terminal C code; photo_...y.jpg = a calendar app screenshot (December 2023, month view); Untitled Design(3).png = macOS desktop with System Settings/Finder windows and a yellow sticky note. Frames alternate: 68:18 Design(5), 68:19 photo, 68:20 Design(3), 68:21 photo, 68:22 Design(5), 68:23 photo, 68:24 Design(3), 68:26/68:27 Design(5), 68:28 photo.
- [68:31-68:32] Clicks Upload again; dialog open; hovers Screen Recording ... .mp4 (68:32 Open enabled, row highlighted).
- [68:38] Sidebar now has 4th tab `Screen Recording 2026-10-29 at 9.43.19 PM.mp4`; detail still showing the calendar image.
- [68:39] Clicks the mp4 tab: detail pane EMPTY (video not rendered; code only handles `image/*`).
- [68:40] Dock shown, hover label "Finder".
- [68:42-68:54] Finder: Favorites <account>, Desktop, Documents, Downloads, sources, lab; home folders Desktop, Documents, Downloads, go, lab, Movies, Music, notes, Pictures, Public, sources. Downloads in icon view: Screen Recording 2026-1...PM.mp4 (01:59, play icon on hover/quick-look), Untitled Design.png ... (6).png (1,280x720), photo_6118596717982104515_y.jpg 1,280x626, photo_63209954...63_y.jpg and ..._y(1).jpg 1,169x771. Scrolls up/down. Purpose: look at the video file (1:59 long, 19.9 MB, too large), looking for a smaller test file.
- [69:02-69:04] Finder closed; Firefox opened to a new blank tab (address bar "Search with DuckDuckGo or enter address").
- [69:09-69:11] Search results for sample mp4 (Sample files library samplelib.com/sample-mp4.html "Sample MP4 video files"; "Sample Videos for Testing"; truefilesize.com "Sample MP4 File Download - Free Test Videos, 480p to 4K"; Pixabay "2,180+ Free Mp4 Video 4K & HD Stock Videos"). Opens samplelib.com/sample-mp4.html: rows "Test video of a road in a city" 1080p 3 Mb, 720p 3 Mb, 360p 1 Mb; "Test video of a couple of buses passing by the park" 1080p 5 Mb, 720p 5 Mb; each with inline player, download and link buttons.
- [69:28-69:29] Download dialog: Save As `sample-5s-720p`, Format MPEG-4 Video, folder Downloads (a `TgK5OsEc.mp4.part` visible in listing). Clicks Save.
- [69:31] Download spinner in toolbar; first video player shows 0:03 (played).
- [69:32] Editor: `sidebar.js - web-filemanager`; cursor on an empty indented line after `detailCtn.innerHTML = "";` (file tree: detail.js, index.html, main.js, sidebar.js).
- [69:33] Hover tooltip over `file.type`: `(property) Blob.type: string` / "The type read-only property of the Blob interface returns the MIME type of the file." / link "MDN Reference".
- [69:34] Tooltip gone; cursor on closing `}` of the `if` block (line highlighted).
- [69:37] Browser: download arrow now blue (download finished). [69:38/69:39] Editor again, cursor right after `}` closing the `if` block.
- [69:45] sidebar.js: `console.log(file);` inserted as first line of the click handler, above `detailCtn.innerHTML = "";`; cursor at end of it. The blank line that was there is gone (so the blank before `if(` remains).
- [69:46] Firefox on samplelib page (download arrow blue).
- [69:49] Firefox at 127.0.0.1:5500/index.html reloaded: empty HFiles; Upload hovered.
- [69:50-69:51] Upload dialog: now top entry `sample-5s-720p.mp4` 2.6 MB MPEG-4 movie, Today at 9:39 PM, then Untitled Design(6).png etc.; no selection (Open greyed).
- [69:52] Dialog fading out (closed/cancelled).
- [70:18] Page: sidebar wider (~330px, Upload button at x~428), empty tabs; devtools `.sidebar` rule: `height: 100%; width: 272px; border-right: 2px solid var(--border-color); display: flex; flex-flow: column;` plus newly added highlighted `min-width: 272px;`. DOM shows `div.sidebar-tabs` empty and `div.detail` empty. (min-width is prototyped in devtools.)
- [70:19] Mission-Control style switch back to editor, sidebar.js showing `console.log(file);` line with cursor.
- [70:20] sidebar.js; mouse over `index.html` in the file tree (about to open index.html, presumably to persist `min-width`).

## File states

### sidebar.js (visible part at end of segment, 70:20; no line-number gutter visible; 4-space indent)
```
?? (lines before inpUpload.type not visible)
inpUpload.type = "file";
inpUpload.multiple = true;
inpUpload.addEventListener("change", () => {
    for (const file of inpUpload.files) {
        const urlFile = URL.createObjectURL(file);

        const tab = document.createElement("div");
        tab.innerHTML = file.name;
        tab.className = "sidebar-tab";

        tab.addEventListener("click", function() {
            console.log(file);
            detailCtn.innerHTML = "";

            if(file.type.startsWith("image/")) {
                const img = document.createElement("img");
                img.src = urlFile;
                detailCtn.append(img);
            }
        })

        tabs.append(tab);
    }
});


btnUpload.innerHTML = "Upload";
btnUpload.addEventListener("click", function () {
    inpUpload.click();
})

sidebarNav.append("HFiles")   <- partially visible at bottom edge in 69:32 frame
```
Earlier version (69:32-69:39): same, but without `console.log(file);` (handler began with `detailCtn.innerHTML = "";`).

### index.html (CSS tail visible at 67:49; no line-number gutter; 2-space indent inside style)
```
?? (earlier styles not visible)
  margin-left: auto;
  cursor: pointer;
}

.sidebar-tabs {
  height: 100%;
  overflow-y: auto;
  padding: 12px;
}

.sidebar-tab {
  width: 100%;
  padding: 12px;
  cursor: pointer;
  font-size: 14px;
  background: #f8f9fa;
  margin-bottom: 6px;
  border-radius: 6px;
}

.detail {
  width: 100%;
  height: 100%;
}

.detail > * {
  max-width: 100%;
  max-height: 100%;|
}
</style>
</head>

<body>
  <script src="main.js" type="module"></script>
</body>
```
Not visible in editor: `.sidebar` rule (devtools shows it at inline:20) - may not yet contain `flex-flow: column` / `min-width: 272px`, which were only tried in devtools. detail.js and main.js not opened in this segment.

### Devtools-live rules
```
.sidebar {  /* inline:20 */
  height: 100%;
  width: 272px;
  border-right: 2px solid var(--border-color);
  display: flex;
  flex-flow: column;
  min-width: 272px;   /* added ~70:18 */
}
body {  /* inline:11 */
  width: 100%;
  padding: 0;
  margin: 0;
  display: flex;
  height: 100vh;
  color: var(--text-color);
}
```

## Shell & outputs
- No terminal commands; console output not shown.
- Page output: HFiles sidebar listing uploaded file names as clickable tabs; clicking an image tab shows the image in the detail pane; clicking the .mp4 tab shows an empty detail pane.
- Web page visited: samplelib.com/sample-mp4.html; downloaded `sample-5s-720p.mp4` (2.6 MB) into Downloads at 9:39 PM.

## Observations
- Workflow: edit in editor, test at 127.0.0.1:5500 in Firefox, prototype CSS in devtools Rules panel (flex, flex-flow, min-width) before porting it.
- Bug found: a large image in `.detail` makes the flex sidebar shrink; remedy `min-width: 272px` on `.sidebar` plus `.detail > * { max-width/max-height: 100% }`.
- Feature sequence: hidden `<input type=file multiple>`, `URL.createObjectURL(file)`, one tab per file, click renders `<img>` when `file.type.startsWith("image/")`. Next target is video: his own recording (19.9 MB, 1:59) was too large, so he fetched a 5 s 720p sample from samplelib. He checks `file.type` via MDN hover tooltip (Blob.type) and adds `console.log(file)` to inspect the File object/MIME type.
- Style: plain DOM API (createElement, innerHTML, append), ES modules, 4-space indent JS, 2-space CSS, no framework.
