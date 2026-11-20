# Segment 18 (74:31 - 78:16) - research on browser storage (MDN, localForage), start of storage.js

## Timeline
- [74:31-74:32] Firefox window, MDN "Web APIs" index (developer.mozilla.org/en-US/docs/Web/API), scrolling the alphabetical list (W: Web Storage API, Web Workers API, WebGL...; then S: Screen Capture API ... Storage API, Storage Access API, Streams API; T, U, V). Webcam overlay ignored.
- [74:41-74:49] MDN "IndexedDB API" page (breadcrumb Web > Web APIs > IndexedDB API). Reads "Key concepts and usage": "IndexedDB is a transactional database system, like an SQL-based RDBMS ... a JavaScript-based object-oriented database ... store and retrieve objects that are indexed with a key; any objects supported by the structured clone algorithm can be stored." Note box on same-origin policy. "Synchronous and asynchronous: Operations performed using IndexedDB are done asynchronously, so as not to block applications." "Storage limits and eviction criteria". Interfaces: IDBFactory, IDBOpenDBRequest, IDBDatabase, IDBTransaction, IDBCursor, IDBCursorWithValue, IDBKeyRange, IDBVersionChangeEvent. Scrolls through Examples (To-do Notifications), Specifications (Indexed Database API 3.0), See also (Web Storage API, Window: localStorage property, Window: sessionStorage property, StorageEvent), and the footer ("last modified on Apr 3, 2025").
- [74:52] Clicks "Web Storage API" (See also) -> MDN "Web Storage API" page, Baseline "Widely available".
- [74:53-75:04] Reads "Concepts and usage": sessionStorage (partitioned by tab and origin; closing tab destroys data) vs localStorage (partitioned by origin only; persists when browser closed and reopened). "Both sessionStorage and localStorage in Web Storage are synchronous in nature ... can potentially affect the performance". "Asynchronous alternatives, such as IndexedDB, may be more suitable for scenarios where performance is a concern or when dealing with larger datasets." Note on third-party iframes. Scrolls through "Determining storage access by a third party" (diagram Page publisher.com / iframe or script content.com) and "Web Storage interfaces" (Storage, Window, StorageEvent).
- [75:12] Examples section ("Web Storage Demo", landing page, event output page), Specifications (HTML dom-localstorage-dev, dom-sessionstorage-dev), start of Browser compatibility.
- [75:13] Types "local" in the URL bar (Firefox Suggest shows history entries; not transcribed).
- [75:15] Search "localforage" via DuckDuckGo (loading; "Transferring data from links.duckduckgo.com...").
- [75:20] localForage site (localforage.github.io/localForage/): "LOCALFORAGE - Offline storage, improved." Example shown:
  ```
  // Set a value with localStorage:
  localStorage.setItem('key', JSON.stringify('value'));
  doSomethingElse();

  // The same code with localForage:
  localforage.setItem('key', 'value').then(doSomethingElse);

  // localForage also support callbacks:
  localforage.setItem('key', 'value', doSomethingElse);
  ```
  Text: "localForage is a JavaScript library that improves the offline experience of your web app by using an asynchronous data store with a simple, localStorage-like API." localStorage-backed fallback for browsers with no IndexedDB/WebSQL. "localForage offers a callback API as well as support for the ES6 Promises API". "Download localforage.min.js".
- [75:22-75:30] INSTALLATION section: `npm install localforage`, `yarn add localforage`, `<script src="localforage.js"></script>` / `<script>console.log('localforage is: ', localforage);</script>`; "Then simply include the JS file and start using localForage ... You don't need to run any init method or wait for any onready events."
- [75:44-75:47] getItem docs: callback version `localforage.getItem('somekey', function(err, value) {...})`, async/await try/catch version; "getItem(key, successCallback): Gets an item from the storage library and supplies the result to a callback. If the key does not exist, getItem() will return null." Note: "Even if undefined is saved, null will be returned by getItem() ...". Scrolls back to top.
- [75:52-76:00] Scrolling top/installation of localForage page.
- [76:01] Opens Firefox "search all tabs" menu. Current window tabs: `127.0.0.1:5500/index.html`; CSS Variables - The var() function; Google; "javascript - how to convert a file object to a url - Stack Ove..."; "URL: createObjectURL() static method - Web APIs | MDN"; "js check file is image video at DuckDuckGo"; "Sample MP4 video files"; "HTML Video"; "Web Storage API - Web APIs | MDN"; localForage; "raw.githubusercontent.com/mozilla/localForage/master/dist...".
- [76:03] Opens raw.githubusercontent.com/mozilla/localForage/ (dist/localforage.min.js): header comment "localForage -- Offline Storage, Improved / Version 1.10.0 / https://localforage.github.io/localForage / (c) 2013-2017 Mozilla, Apache License 2.0", then minified code.
- [76:07] macOS Save dialog: "Save As: localforage.min", Tags empty, Format "Text File", location Downloads (listing includes photo_*.jpg, sample-5s-720p.mp4, Untitled Design*.png).
- [76:16] Navigates dialog to lab > web-filemanager (siblings go-filemanager, web-editor, web-filemanager; folder contents detail.js, index.html, main.js, sidebar.js). Clicks Save. Webcam overlay is moved up for a moment.
- [76:17-76:22] Back on minified page, then localForage docs (webcam overlay now at right, mid-height).
- [76:28] Editor (light theme VS Code-like), title "sidebar.js — web-filemanager". Explorer: detail.js, index.html, localforage.min.js, main.js, sidebar.js (localforage.min.js now present). A new comment line `// can it storage these files forever` is being typed in sidebar.js between `inpUpload.multiple = true;` and `inpUpload.addEventListener("change", () => {`, cursor at end of it.
- [76:30] Editor tab index.html (title "index.html — web-filemanager"), CSS region (body, .sidebar, .sidebar-nav, .btn-upload) visible; current line `min-width: 272px;`.
- [76:59-77:10] Back to localForage docs, scrolling at top (mouse over setItem example); window is scrolled to show top nav bar (localForage / View on GitHub / Issues / 中文版).
- [77:08] Editor, title "storage.js — web-filemanager": new file storage.js in explorer (detail.js, index.html, localforage.min.js, main.js, sidebar.js, storage.js). Content so far: `function storageAdd(key, value) {` / `lo` being typed / `}`.
- [77:10] storage.js: `localforage.setItem('key', 'value')` typed in the function body (literal strings, not yet using the parameters).
- [78:15] Browser: on localForage page the text `localforage.setItem('key', 'value')` is selected in the "same code with localForage" example (copy source).
- [78:16] "Search all tabs" menu open again (tab list as before without the raw.githubusercontent entry; last entry localForage); hovering `127.0.0.1:5500/index.html`, about to switch to the running app.
(No editor save messages are visible in this segment.)

## File states
### storage.js (new; state at 77:10 / end of segment)
```
1| function storageAdd(key, value) {
2|     localforage.setItem('key', 'value')
3| }
```
(Line 2 has no trailing semicolon as visible; the text cursor sits after `)`. At 77:08 line 2 was just `lo`.)

### sidebar.js (partial, 76:28 snapshot; editor shows no line-number gutter)
```
?? (earlier lines not visible)
   const btnUpload = document.createElement("button");
   btnUpload.className = "btn-upload";

   const tabs = document.createElement("div");
   tabs.className = "sidebar-tabs";

   inpUpload.type = "file";
   inpUpload.multiple = true;
   // can it storage these files forever
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
               else if (file.type.startsWith("video/")) {
                   const video = document.createElement("video");
                   video.controls = true;
                   ?? (continues, not visible)
```
The `// can it storage these files forever` comment is new at this point.

### index.html (CSS portion visible at 76:30; no gutter; 2-space indentation)
```
body {
  width: 100%;
  padding: 0;
  margin: 0;
  display: flex;
  height: 100vh;
  color: var(--text-color);
}

.sidebar {
  height: 100%;
  width: 272px;
  min-width: 272px;
  border-right: 2px solid var(--border-color);
  display: flex;
  flex-flow: column;
}

.sidebar-nav {
  height: 42px;
  border-bottom: 1px solid var(--border-color);
  display: flex;
  align-items: center;
  padding: 12px;
  font-weight: bold;
  font-size: 18px;
}

.btn-upload {
  background: #f3f5f6;
  border: 1px solid var(--border-color);
  ?? (rest not visible)
```

### localforage.min.js
Downloaded from raw GitHub (Version 1.10.0), saved into lab/web-filemanager as "localforage.min" (Format: Text File); the explorer then lists it as `localforage.min.js`.

## Shell & outputs
No terminal in this segment. Browser/OS items:
- [75:15] DuckDuckGo search "localforage".
- [76:07-76:16] macOS Save dialog for localforage.min (Downloads, then lab/web-filemanager); buttons Cancel / Save.
- [76:01, 78:16] Firefox "search all tabs" lists (see Timeline).

## Observations
- Motive for storage: the in-code comment `// can it storage these files forever` in sidebar.js (next to the upload input that uses URL.createObjectURL) shows he wants uploaded files to persist beyond object URLs / page reload.
- Research path: MDN Web APIs index -> IndexedDB (async, objects, structured clone) -> Web Storage API (synchronous string key/value; MDN itself suggests IndexedDB for larger data) -> localForage (async, localStorage-like API over IndexedDB). He picks localForage and downloads localforage.min.js manually (raw GitHub, Save As) into the project folder, consistent with the no-build plain-HTML style (to be loaded with a script tag).
- He starts storage.js as a thin wrapper `storageAdd(key, value)` and first pastes the docs' `localforage.setItem('key', 'value')` literally, to be parameterised next.
- Project layout: lab/web-filemanager (detail.js, index.html, main.js, sidebar.js, localforage.min.js, storage.js); sibling projects web-editor and go-filemanager.
- Uses browser tabs as a working notebook (earlier tabs: CSS var(), createObjectURL, file-type check, HTML video, sample MP4).
