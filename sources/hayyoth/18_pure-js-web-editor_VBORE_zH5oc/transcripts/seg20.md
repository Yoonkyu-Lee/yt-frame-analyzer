# Segment 20 of 20 (83:25 - 87:59) - VBORE_zH5oc

Editor shows NO line numbers in this video, so File states have no `N|` prefixes; only the sidebar.js region visible is given. Editor window title: `sidebar.js — web-filemanager`. No save messages visible (macOS editor, no status bar). No file other than sidebar.js was seen as source in this segment.

## Timeline
- [83:25] sidebar.js scrolled to the end of `appendFile` and the start of `sidebar(detailCtn)`. `appendFile` already contains the `tab.addEventListener("click", ...)` with image/video branches; ends with `tabs.append(tab);`.
- [83:27] Top of `appendFile`: `function appendFile() {` (no params yet), `const urlFile = URL.createObjectURL(file);`, then `localforage.setItem(file.name, file)` (no semicolon) inside appendFile, then tab creation.
- [83:40] Caret is inside the empty parens: `function appendFile(|) {` - about to add parameters. Lightbulb in gutter.
- [84:03] Hover tooltip over `document` (`var document: Document`, `window.document returns a reference to the document contained in the window.`, MDN Reference). `appendFile` now has NO `localforage.setItem` line (blank line left) - the setItem line was cut from appendFile.
- [84:06] Signature being typed: `function appendFile(file, ) {` (cursor after the comma). Blank line highlighted where setItem was.
- [84:30-84:31] Scrolled down to `sidebar(detailCtn)`. Seen: `inpUpload.multiple = true;`, comment `// can it storage these files forever`, change handler `for (const file of inpUpload.files) { localforage.setItem(file.name, file); }`, btnUpload click -> `inpUpload.click()`, `sidebarNav.append("HFiles")`.
- [84:45] Added inside the for loop after setItem: `appendFile(file, tabs, detailCtn);` (current line highlighted). Below: `sidebarNav.append(btnUpload);` and `initFiles();`.
- [84:47] Same, scrolled slightly.
- [84:50] Signature now `function appendFile(file, tabs, detailCtn) {` with `tabs, detailCtn` highlighted. Above it the end of initFiles (console.log(keys); keys.forEach(key => { console.log(key) });).
- [84:51] `function initFiles() {` ... `localforage.keys().then(function (keys) { // An array of all the key names. console.log(keys); keys.forEach(key => { console.log(key) }); })`; `tabs, detailCtn` still highlighted in appendFile signature (copy).
- [84:59] Call site changed to `initFiles(tabs, detailCtn);` (caret inside parens). Followed by `ctn.append(sidebarNav, tabs);` and `return ctn;`.
- [85:03] Selected the `appendFile(file, tabs, detailCtn);` line in the change handler.
- [85:04-85:05] Scrolling appendFile showing final signature `appendFile(file, tabs, detailCtn)`.
- [85:07] Top of `initFiles(tabs, detailCtn)` now with the new params (greyed as unused).
- [85:17] Firefox, http://127.0.0.1:5500/index.html, console open. Page: "HFiles" header + Upload button, empty left list, empty right pane. Console: `Untitled Design(4).png   sidebar.js:6:21` then error `Uncaught (in promise) TypeError: URL.createObjectURL: Argument 1 could not be converted to any of: Blob, MediaSource.` at `sidebar.js:14:25`, stack: `appendFile http://127.0.0.1:5500/sidebar.js:14`, `initFiles ...sidebar.js:8`, `initFiles ...sidebar.js:5`, `promise callback*initFiles ...sidebar.js:2`, `sidebar ...sidebar.js:78`, `<anonymous> ...main.js:5`. (Cause: initFiles passes the key string, not the stored Blob.)
- [85:18] Reads localForage docs (localforage.github.io/localForage/#data-api-keys): KEYS section with snippet `localforage.keys().then(function(keys) { // An array of all the key names. console.log(keys); }).catch(function(err) { // This code runs if there were any errors console.log(err); });`, "keys(successCallback)", "Get the list of all keys in the datastore." ITERATE section below. Part of the snippet is highlighted (copied).
- [85:20] Docs scrolled up to `key` section ("Get the name of a key based on its ID.", note "This method is inherited from the localStorage API, but is acknowledged to be kinda weird."); mouse goes to `getItem` in side nav.
- [85:38] Back in editor, inside initFiles typing: `const file = localforage.get` (incomplete) between `console.log(key)` and `appendFile(file, tabs, detailCtn);`.
- [85:46] localForage getItem docs: `localforage.getItem('somekey').then(function(value) { // This code runs once the value has been loaded // from the offline store. console.log(value); }).catch(function(err) {...});`, "// Callback version:" `localforage.getItem('somekey', function(err, value) {...})`, "// Or, use `async`/`await`:" try { const value = await localforage.getItem('somekey'); ... } catch (err) {...}. `getItem(key, successCallback)` heading below.
- [85:54] Editor unchanged: still `const file = localforage.get`.
- [86:34-86:35] Scrolling appendFile (viewing only).
- [86:52] Docs GETITEM again, first snippet lines (`localforage.getItem('somekey').then(function(value) { ... console.log(value); }`) highlighted - copying.
- [86:55-86:59] After the edit, page reloaded: sidebar lists `Untitled Design(4).png` and `sample-5s-720p.mp4`; console: `Socket is closed. Reconnect will be attempted in 1 second. <empty string>` (fiveserver.js:1:11869) and warning `This page is in Quirks Mode. Page layout may be impacted. For Standards Mode use "<!DOCTYPE html>". Learn More` (index.html). Files persisted and tabs were restored.
- [87:00] Click on `sample-5s-720p.mp4`: video player appears (park scene, play control, 0:00). Console: `File { name: "sample-5s-720p.mp4", lastModified: 1793371148938, webkitRelativePath: "", size: 2572910, type: "video/mp4" }` sidebar.js:19:17.
- [87:01] Click on `Untitled Design(4).png`: image of "THE C PROGRAMMING LANGUAGE" (Kernighan/Ritchie) cover with C code overlay (`size_t hstrlen(const char *s) {... } void *hmemcpy(...)`). Console adds `File { name: "Untitled Design(4).png", lastModified: 1793207740219, webkitRelativePath: "", size: 601630, type: "image/png" }`.
- [87:08] Mouse on Upload; webcam overlay moved lower right and enlarged.
- [87:09-87:11] macOS file dialog (Downloads): sample-5s-720p.mp4 2.6 MB, Untitled Design(6).png 456 KB, photo_..._y(1).jpg 119 KB, photo_..._y.jpg 173 KB / 119 KB, Untitled Design(5).png 584 KB, (4) 602 KB, (3) 704 KB, (2) 462 KB, (1) 314 KB, Untitled Design.png 421 KB. He selects `Untitled Design.png`, then also `Untitled Design(1).png` (multi-select), mouse over Cancel/Open.
- [87:12] After upload the list: Untitled Design(4).png, sample-5s-720p.mp4, Untitled Design.png, Untitled Design(1).png.
- [87:14] Clicking `Untitled Design(1).png`: right pane shows screenshot of an editor (`main.c — lab`, `#include <stdio.h>`, `int main(void) { printf("Hello World\n"); return 0; }`, terminal `lab $`). Console logs `File { name: "Untitled Design.png", lastModified: 1793122964623, ..., size: 421297, type: "image/png" }` and `File { name: "Untitled Design(1).png", lastModified: 1793173308804, ..., size: 314347, type: "image/png" }`.
- [87:15] Clicked mp4 again (video 0:00).
- [87:16-87:17] Clicks Firefox reload. After reload tab order CHANGES: `Untitled Design(1).png`, `Untitled Design(4).png`, `Untitled Design.png`, `sample-5s-720p.mp4` (localforage key order). All four persisted.
- [87:20] Clicked a tab; console shows File Untitled Design(4).png log.
- [87:22] Same window navigates to www.google.com with Inspector tab open (Google page DOM, Rules panel).
- [87:23] w3schools "The HTML <video> Element" (html5_video.asp): `<video width="320" height="240" controls> <source src="movie.mp4" type="video/mp4"> <source src="movie.ogg" type="video/ogg"> Your browser does not support the video tag. </video>`.
- [87:24-87:26] Closes windows to empty desktop; mouse over Dock showing "Terminal" tooltip; a new Firefox window with blank "Search with DuckDuckGo or enter address".
- [87:29-87:35] Back at 127.0.0.1:5500/index.html, window repositioned/resized. Tabs: Untitled Design(1).png, Untitled Design(4).png, Untitled Design.png, sample-5s-720p.mp4. Clicks Design(4) (C book) then mp4 (video 0:00 / 0:06). Console opened with Quirks warning + 4 File logs; dev tools panel resized by dragging its top edge.
- [87:43-87:45] Console cleared (`>>` empty). Clicks tab: C book image; then Untitled Design.png shows a spreadsheet screenshot with a green sticky note "Flat / ---- / Understand this, and you won't hate C." (Address 1-8, Memory bits, "Person Struct" id/age, person 1-4).
- [87:50-87:55] Switches to sidebar.js editor, scrolling appendFile -> sidebar; no new edits. [87:53] Back to browser; [87:55] macOS menu bar visible (Firefox, `Fri Oct 30 9:57 PM`), console input typed `bye bye ` (author signing off).
- [87:59] End: console prompt `bye bye |`, page unchanged.

## File states

### sidebar.js (end of segment; reconstructed from the 83:25-85:07 and 87:50-87:53 frames)
```js
?? (lines before initFiles not visible; imports etc.)
function initFiles(tabs, detailCtn) {
    localforage.keys().then(function (keys) {
        // An array of all the key names.
        console.log(keys);
        keys.forEach(key => {
            console.log(key)
            const file = localforage.get⟨?⟩   // last seen at 85:38/85:54: incomplete line
            appendFile(file, tabs, detailCtn);
        });
    })
}
```
The final initFiles body after the 86:52 docs visit is NOT visible. Behavior at 86:55+ (blobs restored, tabs clickable) implies it fetched each Blob via `localforage.getItem(key)` (promise form from docs) before calling appendFile.

```js
function appendFile(file, tabs, detailCtn) {
    const urlFile = URL.createObjectURL(file);

    const tab = document.createElement("div");
    tab.innerHTML = file.name;
    tab.className = "sidebar-tab";

    tab.addEventListener("click", function () {
        console.log(file);
        detailCtn.innerHTML = "";

        if (file.type.startsWith("image/")) {
            const img = document.createElement("img");
            img.src = urlFile;
            detailCtn.append(img);
        }
        else if (file.type.startsWith("video/")) {
            const video = document.createElement("video");
            video.controls = true;
            const source = document.createElement("source");
            source.src = urlFile;
            video.append(source);

            detailCtn.append(video);
        }

    })

    tabs.append(tab);
}

export function sidebar(detailCtn) {
    const ctn = document.createElement("div");
    ctn.className = "sidebar"

    const sidebarNav = document.createElement("div");
    sidebarNav.className = "sidebar-nav";

    const inpUpload = document.createElement("input");

    const btnUpload = document.createElement("button");
    btnUpload.className = "btn-upload";

    const tabs = document.createElement("div");
    tabs.className = "sidebar-tabs";

    inpUpload.type = "file";
    inpUpload.multiple = true;
    // can it storage these files forever
    inpUpload.addEventListener("change", () => {
        for (const file of inpUpload.files) {
            localforage.setItem(file.name, file);
            appendFile(file, tabs, detailCtn);
        }
    });


    btnUpload.innerHTML = "Upload";
    btnUpload.addEventListener("click", function () {
        inpUpload.click();
    })

    sidebarNav.append("HFiles")
    sidebarNav.append(btnUpload);

    initFiles(tabs, detailCtn);

    ctn.append(sidebarNav, tabs);
    return ctn;
}
```
(Blank-line layout as seen in frames; no semicolon after `sidebarNav.append("HFiles")` or `ctn.className = "sidebar"`; text after `return ctn;` not visible.)

Intermediate versions:
- 83:27: `function appendFile() {` with `localforage.setItem(file.name, file)` (no semicolon) between `const urlFile = ...` and `const tab = ...`.
- 84:06: `function appendFile(file, ) {` while typing; setItem line already removed.
- 84:50: `function appendFile(file, tabs, detailCtn) {`.
- 84:59: call changed from `initFiles();` to `initFiles(tabs, detailCtn);`.
- 85:07: `function initFiles(tabs, detailCtn) {` (was `function initFiles() {` at 84:51).

### Other files
Not shown as source. Browser-visible only: index.html at 127.0.0.1:5500 has no `<!DOCTYPE html>` (Quirks Mode warning); main.js line 5 calls `sidebar(...)` (stack trace `main.js:5`).

## Shell & outputs
No terminal commands run (Terminal Dock icon only hovered at 87:25).
Browser console outputs (verbatim) in Timeline: createObjectURL TypeError (85:17), Quirks Mode warning, "Socket is closed. Reconnect will be attempted in 1 second. <empty string>" (fiveserver.js:1:11869), and `File { ... }` logs at sidebar.js:19:17.
Final page: Firefox at http://127.0.0.1:5500/index.html. Left column "HFiles" + "Upload" button and 4 light-grey tab rows (Untitled Design(1).png, Untitled Design(4).png, Untitled Design.png, sample-5s-720p.mp4); right pane renders the clicked file (image or video with controls).

## Observations
- Persistence design: files stored in localForage by `file.name` key; on load `initFiles` lists keys and rehydrates Blobs, then `appendFile` builds the sidebar tab. A bug (key string passed to `URL.createObjectURL`) appeared first; he fixed it by reading localForage docs (keys, key, getItem) instead of guessing.
- Refactor: extracted `appendFile` taking `(file, tabs, detailCtn)`; `localforage.setItem` moved out of it into the upload handler so restoring does not rewrite storage.
- Test flow: reload to confirm persistence, multi-file upload via native dialog, click each tab for image/video preview, noticed tab order shuffles after reload (no sorting implemented).
- References consulted: localForage docs, w3schools video element, a stray google.com inspector peek.
- Code comment `// can it storage these files forever` answered by the reload test.
- Video ends casually: types "bye bye" in the dev console. The green "Flat / Understand this, and you won't hate C." sticky note is inside an uploaded image shown in the preview pane, not authored in this segment.
