# seg11 (41:37 - 45:48) - VBORE_zH5oc

Editor: macOS VS Code-like window titled "<file> - web-filemanager" (no line-number gutter visible; line numbers below are INFERRED from content and Firefox "inline:N" labels, where inline N = file line - 3). Explorer "WEB-FILEMANAGER" lists: detail.js, index.html, main.js, sidebar.js. Browser: Firefox. Webcam overlay "Human" ignored.

## Timeline
- [41:37] Firefox on www.w3schools.com/css/css3_variables.asp, section "CSS Declare a Variable". Text: "CSS variables can have a global or local scope." "To create a global variable, declare it inside the :root selector. The :root selector matches the document's root element." "To create a local variable, declare it inside the selector that is going to use it." "A CSS variable name must begin with two dashes (--) and is case sensitive!" Syntax example: `:root { --primary-bg-color: green; /* global scope */ }` and `.note { --note-bg: yellow; /* local scope */ }`. Left sidebar: CSS Variables > The var() Function (highlighted).
- [41:40] Scrolled; author selects the `:root { --primary-bg-color: green; /* global scope */ }` block (copying it). Below: "The CSS var() Function".
- [41:41] Switch to editor, index.html. Existing `.sidebar` has `border-right: 1px solid #dadce0;` with `#dadce0` selected (color swatch shown). In this frame `margin:` in body is hidden by the mouse cursor (`margin: 0;` in later frames).
- [41:42] Editor, cursor on new blank line 8 (after the `* {...}` rule) - about to paste.
- [42:01] Back to w3schools: "The CSS var() Function": "The var() function is used to insert the value of a declared CSS variable." Syntax `var(--name, value)`; table: name "Required. The variable name (must start with two dashes)"; value "Optional. The fallback value (used if the variable is not found)". "Advantages of using var() are:".
- [42:05] Editor: pasted+edited block now
  `:root {` / `--border-color: #dadce0;` / `/* global scope */` / `}`; `.sidebar` line reads `border-right: 1px solid |;` (value removed, cursor after "solid ").
- [42:09] `border-right: 1px solid var(--border-color);` typed (window mid-transition to Firefox).
- [42:10-42:15] Firefox at http://127.0.0.1:5500/index.html with devtools (Inspector). Page shows "Sidebar" (left, ~272px) and "Detail" with a vertical divider line. DOM: `<html> event`, `<head> ...</head>`, `<body> flex`: `<script src="main.js" type="module"></script>`, `<div class="sidebar">Sidebar</div>`, `<div>Detail</div>`. Rules: `body {width:100%; padding:0; margin:0; display:flex; height:100vh}` inline:11; `* {box-sizing:border-box}`; Inherited from html `:root { --border-color: #dadce0; }`. Hover on body shows overlay "body 1014 x 295 Flex Container".
- [42:15] Hover on div.sidebar: "div.sidebar 272 x 295 Flex Item". Rule `.sidebar {height:100%; width:272px; border-right:1px solid (swatch) var(--border-color ...);}` inline:19.
- [42:21] Author hovers the `.sidebar` rule in devtools (checkboxes shown).
- [42:22] Editor: cursor right after `--border-color` inside `var(...)`; the `--border-color` token highlighted (word-occurrence highlight also in `:root`).
- [42:37] Editor (scrolled): same, comment `/* global scope */` still present; cursor in `var(--border-color|)`.
- [42:46] Firefox: body rule now at inline:10 (comment line deleted, page reloaded). Devtools otherwise unchanged. (Frame order in list: 42:37 editor shown scrolled; 42:46 browser.)
- [43:25] Editor: comment `/* global scope */` deleted; `border-right` changed to `2px solid var(--border-color);`. `:root` now has only `--border-color: #dadce0;`.
- [43:28] Firefox page now shows an "Upload" button at top-left of sidebar (hover). DOM: `<div class="sidebar"> ... </div>` collapsed. Body rule inline:10.
- [43:30] URL bar: typing "js up" (DuckDuckGo suggestion list; suggestions omitted). [43:33] search "js upload file" loading ("Waiting for duckduckgo.com...").
- [43:33-44:02] DuckDuckGo results for "js upload file": GeeksforGeeks "How to Upload Files in JavaScript?" (Jul 23, 2025); bytescale.com "Upload.js - JavaScript File Upload Library"; W3School "HTML DOM Input FileUpload files Property - W3Schools" (w3schools.com/jsref/prop_fileupload_files.asp); FreeCodecamp "How to Upload Files with JavaScript"; TutorialsPoint "JavaScript - Upload Files"; Stack Overflow "JavaScript: Upload file" (questions/5587973); Stack Overflow "How to upload a file using javascript?" (questions/51035478).
- [43:37] Clicked the GeeksforGeeks result (loading, "Transferring data from assets.geeksforgeeks.org..."); went back [43:45].
- [43:50-43:53] Opened W3Schools "HTML DOM Input FileUpload files Property" (www.w3schools.com/jsref/prop_fileupload_files...). Example shown:
  ```
  var x = document.getElementById("myFile");
  var txt = "";
  if ('files' in x) {
    if (x.files.length == 0) {
      txt = "Select one or more files.";
    } else {
      for (var i = 0; i < x.files.length; i++) {
        txt += "<br><strong>" + (i+1) + ". file</strong><br>";
        var file = x.files[i];
        if ('name' in file) {
          txt += "name: " + file.name + "<br>";
        }
        if ('size' in file) {
          txt += "size: " + file.size + " bytes <br>";
        }
      }
    }
  }
  document.getElementById ("demo").innerHTML = txt;
  ```
  ("Try it Yourself >>" button, then "Description" heading.) Skimmed, then back to results.
- [43:58-44:03] Back on results page, scrolls through; visited links purple.
- [44:03] URL bar typed "in" (suggestions omitted); [44:08] search "input type files mdn" results: Mozilla Developer "<input type="file"> HTML attribute value - HTML | MDN" (sub-links: accept HTML attribute; Using files from web applications; <input> HTML input element), MDN "<input> HTML input element", W3School "HTML input type="file"".
- [44:09-44:13] Opened MDN page developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/input/file. "Try it": `<label for="avatar">Choose a profile picture:</label>` / `<input type="file" id="avatar" name="avatar" accept="image/png, image/jpeg" />`; output "Browse..." "No file selected."
- [44:14] Author clicks "Browse..." - macOS native file picker opens (sidebar: home folder, Desktop, Documents, Downloads, sources, lab; Media: Photos). Dismissed by [44:15].
- [44:20-44:22] MDN scrolling: "Additional attributes" > accept: "The accept attribute value is a string that defines the file types the file input should accept. This string is a comma-separated list of unique file type specifiers." Word example `<input type="file" id="docpicker" accept=".doc,.docx,.xml,application/msword,application/vnd.openxmlformats-officedocument.wordprocessingml.document" />`; "capture" attribute; "Unique file type specifiers" (.jpg/.pdf/.doc, MIME type, audio/*, video/*, image/*); example `<input type="file" accept="image/*,.pdf" />`.
- [44:31-44:33] MDN "Using file inputs > A basic example": `<div><label for="file">Choose file to upload</label><input type="file" id="file" name="file" multiple /></div>`; text on `multiple`; "Getting information on selected files": `HTMLInputElement.files` is a FileList of File objects; File has name, lastModified, lastModifiedDate (deprecated), size, type, webkitRelativePath.
- [44:38] Editor: sidebar.js (title "sidebar.js - web-filemanager"), content as File states v1.
- [44:39] Back to MDN basic example.
- [45:05] (Frame is MDN basic example page, not editor.) 
- [45:09] sidebar.js edited: inserted above the button code:
  `const inpUpload = document.createElement("input");` and `inpUpload.type = "file"` (no semicolon; cursor on that line). Button code still present below (not yet changed).
- [45:29-45:30] Firefox on MDN basic example (window re-focus animation); at 45:30 cursor on the Firefox toolbar devtools/responsive icon.
- [45:32] Switched to localhost tab (127.0.0.1:5500/index.html) with devtools: "Upload" button, DOM `<div class="sidebar"> ... </div>`.
- [45:33] Cursor over the "Upload" button (pressed look).
- [45:34] Upload clicked: macOS file picker opens (Desktop listing loading). [45:36] navigated to Downloads: list of image files (several "Untitled Design(N).png", a few photo_*.jpg, one screen-recording .mp4); nothing selected. Dialog dismissed by [45:37]. (The code change enabling this click-to-picker behavior was not seen between 45:09 and 45:34; unknown whether the picker came from the input or the button.)
- [45:37] Back on page with devtools; cursor over "Debugger" tab (not clicked).
- [45:42-45:48] MDN scrolling: "Getting information on selected files" (name, lastModified, size "The size of the file in bytes.", type "The file's MIME type.", webkitRelativePath); "Limiting accepted file types" (accept examples: "image/png" or ".png"; "image/png, image/jpeg" or ".png, .jpg, .jpeg"; "image/*"; Word docs); "more complete example" `<div><label for="profile_pic">Choose file to upload</label><input type="file" id="profile_pic" name="profile_pic" accept=".jpg, .jpeg, .png" /></div>`; note that accept doesn't validate types, "...make sure that the accept attribute is bac[ked up with] server-side validation."; heading "Detecting cancellations" ("The cancel event is fired when the user does not change their selecti[on]...").
- No editor save messages visible in this segment.

## File states

### index.html (end of segment, as of 43:25; line numbers inferred, nesting 2 spaces per level as rendered)
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
11|     }
12|
13|     body {
14|       width: 100%;
15|       padding: 0;
16|       margin: 0;
17|       display: flex;
18|       height: 100vh;
19|     }
20|
21|     .sidebar {
22|       height: 100%;
23|       width: 272px;
24|       border-right: 2px solid var(--border-color);
25|     }
26|   </style>
27| </head>
28|
29| <body>
30|   <script src="main.js" type="module"></script>
31| </body>
32|
33| </html>
```
Earlier version (~42:05-42:37): line 10 `--border-color: #dadce0;`, line 11 `/* global scope */` (copied from w3schools sample), `}` at 12, `border-right: 1px solid var(--border-color);` at line 25. Before 41:41 there was no :root block and the value was literal `1px solid #dadce0;`. The rendered DOM has `<div class="sidebar">Sidebar</div><div>Detail</div>` created by main.js (not seen in this segment).

### sidebar.js (latest seen, 45:09; line numbers inferred, 4-space indent)
```
 1| export function sidebar() {
 2|     const ctn  = document.createElement("div");
 3|     ctn.className = "sidebar"
 4|
 5|     const inpUpload = document.createElement("input");
 6|     inpUpload.type = "file"
 7|
 8|     const btnUpload = document.createElement("button");
 9|     btnUpload.innerHTML = "Upload";
10|
11|     ctn.append(btnUpload);
12|     return ctn;
13| }
```
Version at 44:38 (before inserting the inpUpload lines): `const btnUpload = ...` at line 5, `btnUpload.innerHTML = "Upload";` at 6, blank 7, `ctn.append(btnUpload);` at 8, `return ctn;` at 9, `}` at 10. Note double space in `const ctn  =` and missing semicolons after `"sidebar"` and `"file"`.

### detail.js, main.js
Not visible in this segment.

## Shell & outputs
No terminal. Browser output:
- [42:10-45:37] http://127.0.0.1:5500/index.html: two-column page; left sidebar column 272px wide with right border (light gray, 1px then 2px); text "Sidebar" at top-left until 43:25, then a gray "Upload" button; right column text "Detail". Firefox devtools docked at bottom (Inspector tab; other tabs Console, Debugger, Network, Storage).
- Inspector values seen: body 1014 x 295 flex container; div.sidebar 272 x 295 flex item.
- [45:34-45:37] Clicking Upload opens the OS file chooser (nothing chosen; no console output seen).

## Observations
- Author learns by consulting docs live: W3Schools for CSS custom properties (copy the `:root` sample, rename to `--border-color`, remove sample comment), then DuckDuckGo -> W3Schools/MDN for file upload (`input type="file"`, `.files`, FileList/File, `accept`, `multiple`, cancel event).
- Introduces CSS variable `--border-color: #dadce0` in `:root` for the sidebar border; temporarily bumped border to 2px as a visibility test after confirming in devtools Rules that the var resolves.
- Uses devtools Inspector heavily: hover to verify sizes (272 x 295) and inherited custom property.
- UI built imperatively in JS modules (`sidebar()` returns a container div class "sidebar"); `main.js` mounts it next to a "Detail" div. Naming convention: `ctn` (container), `btnUpload`, `inpUpload`.
- Plan emerging: file input plus Upload button (button triggers the file picker); reading MDN for `multiple`/`accept`/File properties.
- Loose style: missing semicolons, double space, no formatter run.
