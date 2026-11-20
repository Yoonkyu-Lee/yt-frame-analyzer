# Segment 3 (2:25 - 3:45) - "firefox - but better" (Hayyoth)

## Timeline
- [02:25] macOS desktop. A Firefox window on the `about:profiles` page ("About Profiles") with a "Parent process Browser Toolbox" window (Firefox chrome devtools) overlapping it. Toolbox mode radio: "Parent process only (Fast)" selected, "Multiprocess (Slower)" unselected. Inspector tab open, Rules pane visible. Webcam "Human" overlay bottom right. About Profiles page shows "Profile: default-release" (Default Profile: no; Root Directory `/Users/<account>/Library/Application Support/Firefox/Profiles/z7hv307q.default-release`; Local Directory `/Users/<account>/Library/Caches/Firefox/Profiles/z7hv307q.default-release`; buttons Rename, Remove, Set as default profile, Launch profile in new browser) and "Profile: TestProfiles" ("This is the profile in use and it cannot be deleted."; Default Profile: yes; Root Directory `.../Firefox/Profiles/qq0mrgdf.TestProfiles`; Rename button). Cursor over the HTML tree (`<arrowscrollbox`, `#shadow-root`, `<html:slot>`, `<scrollbox>`).
- [02:26] Element hover highlights the tab strip: tooltip "arrowscrollbox#tabbrowser-arrowscrollbox 637 x 36 Flex Container/Item".
- [02:27] Hovers `<toolbartabstop` in the tree: tooltip "toolbartabstop 0 x 0 Flex Container/Item". Tree shows `<toolbar id="TabsToolba...`, `fullscreentoolbar="true"`, `customizationtarget="Ta...`, `data-l10n-id="tabs-too...`, `flex="1" aria-label="Br...`, `<hbox class="titlebar...`, `<hbox class="toolbar-...`, `<hbox id="TabsToolb...`, `<tabs id="tabbrows...`.
- [02:28] Hovering `<hbox class="toolbar-...`: tooltip "hbox.toolbar-items 665 x 36 Flex Container/Item".
- [02:29] Hovering `<toolbar id="TabsToolbar"`: tooltip "toolbar#TabsToolbar.browser-toolbar.browser-titlebar 777 x 36 Flex Container/Item" (tab bar highlighted blue). Tree scrolled up: `<toolbox id="navigator-t...`, `<toolbar id="toolbar-me...` (menubar), `<toolbar id="TabsToolba...`.
- [02:30-02:32] Clicks the `<toolbar id="TabsToolbar" ...` node (selected, blue). Rules pane then shows `#TabsToolbar {` (toolbarbuttons.css), `.browser-titlebar { :root[customtitlebar] & { &:-moz-window-inactive { opacity: var(--inactive-titlebar-opacity)...`, `.browser-titlebar { :root[customtitlebar] & { will-change: opacity; transition: opacity var(--inac...`, `.browser-titlebar { -moz-window-dragging: drag; }`, `.browser-toolbar {`.
- [02:34] Clicks inside the empty `#TabsToolbar { }` rule in the Rules pane (to add a declaration).
- [02:35] New declaration `display:` typed, with autocomplete list: block, block ruby, contents, flex, flow-root, flow-root list-item, grid, grid-lanes, inherit, initial, inline, inline flow-root list-item, inline list-item, inline-block.
- [02:36] Value typed: `display: none;` - the tab strip disappears live from the Firefox window (URL bar row now at top of window; page scrolled showing "Create a New Profile").
- [02:37] Selector `#TabsToolbar` highlighted (double-click) in the Rules pane; rule reads `#TabsToolbar { display: none; }`.
- [02:39] Mission-Control-like zoomed-out view of windows: a Terminal window running vim with a new file; status line `"userChrome.css" [New]  0,-1  All`, empty buffer with `~` lines; Browser Toolbox and Firefox window behind. Dock visible.
- [02:40-02:41] Terminal window now large over Firefox; vim status `"userChrome.css" [New]  0,-1  All`, empty buffer.
- [02:49] Vim buffer now contains the rule (see Code). Status line: `<ome.css" [New] 3L, 34B written`. Browser Toolbox brought to front; Rules pane still shows `#TabsToolbar { display: none; }`.
- [02:50] Toolbox window dragged to the right (partly off-screen); Firefox About Profiles fully visible (no tab strip).
- [02:51] Toolbox back; Rules pane: `#TabsToolbar { display: none; }`, `.browser-titlebar {...}`, `.browser-toolbar { appearance: none; padding: 0; border-style: none; background-color: transparent; color: inherit; }`.
- [02:53] Element picker on the Firefox window hovering the Back button: tooltip "toolbarbutton#back-button.toolbarbutton-1.chromeclass-toolbar-additional 35 x 36 Flex Container/Item". Tree shows `<hbox id="nav-bar-cus...target" flex="1">`, `<toolbarbutton id="...` with `chromeclass-toolbar...`, `back-3`, `removable="false"`, `keepbroadcastattrib...`, `command="Browser:Ba...`, `tooltip`, `label="B...`, `disabled="true">`.
- [02:54] Toolbox moved off to the right again; only Firefox visible.
- [02:55] Hover on "hbox#nav-bar-customization-target.customization-target 742 x 36 Flex Container/Item" (nav bar highlighted). Rules pane: `#nav-bar-customization-target { & > :is(toolbarbutton, toolbaritem):first-child, & > toolbarpaletteitem:first-child > :is(toolbarbutton, toolbaritem) { padding-inline-start: var(--toolbar-padding-inline); }`; `#back-button { list-style-image: url("chrome://browser/skin/back.svg"); }`; `.toolbarbutton-1 { :root:not([customizing]) &[disabled] { opacity: var(--toolbarbutton-o...`.
- [02:56] Back button hovered again (tooltip as above).
- [02:57] Toolbox widened. Tree: `<toolbarbutton id="back-button" class="toolb... chromeclass-toolbar-additional data-l10n-id="...back-3" removable="false" overflows="false" keepbroadcastattributeswhencustomizing="true" command="Browser:BackOrBackDuplicate" tooltip... label="Back" type="menu" context=... disabled="true">`; `<toolbarbutton id="forward-button" ... command="Browser:ForwardOrForwardDuplicate" ... label="Fo...`, `<toolbaritem id="stop-reload-button" class=...`.
- [02:58] `id="back-button"` value double-click highlighted (to copy).
- [02:59-03:01] Switches to Terminal/vim: buffer `#TabsToolbar {` / `  display: none;` / `}`; status `<ome.css" [New] 3L, 34B written  3,1  All`.
- [03:09] Terminal behind now shows `#TabsToolbar, #back-button[]{` / `display: none;` / `}` with mode `-- INSERT (paste) --`. In toolbox, picker hovers the Forward button: tooltip "toolbarbutton#forward-button.toolbarbutton-1.chromeclass-toolbar-additional 30 x 36 Flex Container/Item".
- [03:10] `id="forward-button"` value selected in the tree (for copying). Rules pane: `#forward-button { list-style-image: url("chrome://browser/skin/forward.svg"); }`, `.toolbarbutton-1:not(:hover):-moz-window-inactive, .bookmark-item:not(:hover):-moz-window-inactive, :root:not([customizing]) .toolbarbutton-1:-moz-window-inactive[disabled] { opacity: 0.5; }`.
- [03:11] Zoomed-out Mission Control view: Terminal (vim `-- INSERT (paste) --  1,27  All`), Firefox About Profiles window (labelled "About Profiles"), Browser Toolbox; Dock visible.
- [03:12] Terminal forward: `#TabsToolbar, #back-button| {` (INSERT paste, 1,27).
- [03:13] Vim: `#TabsToolbar, #back-button, | {` (1,29), INSERT (paste).
- [03:17] After pasting: line reads `#TabsToolbar, #back-button, #forward-but...` (toolbox in front); status `<serChrome.css" 3L, 65B written` (file saved).
- [03:18] Firefox alone: still the About Profiles page; back/forward/reload icons still visible at left (not yet reloaded).
- [03:19] Dialog over Firefox: "Close window and quit Firefox?" with checked checkbox "Ask before quitting with ⌘Q"; buttons Cancel / Quit Firefox (cursor on Quit Firefox). He quits so userChrome.css is loaded on restart.
- [03:20] Only Terminal visible (Firefox quit). Vim shows `#TabsToolbar, #back-button, #forward-butto` + block cursor on `n`, ` {`; status `<serChrome.css" 3L, 65B written   1,43   All`.
- [03:22-03:23] Cursor goes to the Dock; tooltip "Terminal", then "Firefox" as he hovers to relaunch. Line now reads `#forward-button {`.
- [03:24] Firefox relaunched: a new window with URL bar "Search with Google or enter address" focused, magnifier/search-engine dropdown at left, reload icon at far left; NO tab strip.
- [03:25] Cursor on the reload button area at left; no back/forward buttons visible any more.
- [03:26] Right-click on toolbar: context menu "Turn on Vertical Tabs", "Bookmarks Toolbar >", "Customize Toolbar...".
- [03:28] Customize Toolbar mode: "Drag your favorite items into the toolbar or overflow menu."; items: History, Save page, Print, Find, Open file, Sidebars, Zoom Controls, Edit controls, Repair text encoding, Email link, Passwords, Share, Synced tabs, Send tab, Settings, Forget, New private window, Firefox View (+ three icon-only items cut off). "Overflow Menu" panel: "Drag and drop items here to keep them within reach but out of your toolbar..." with a dragon mascot. Toolbar shows reload, empty placeholder boxes (flexible spaces) either side of the URL bar, downloads icon, extensions icon, overflow (>>) and hamburger. Bottom bar: [ ] Title Bar, "Toolbars" dropdown, "Window density settings", "Manage themes", "Customize Touch Bar...", "Restore Defaults", "Done".
- [03:31] Back to normal mode: toolbar is reload (shifted right), URL bar "Search with Google or enter address", tab-search icon, extensions icon, hamburger. Still no tab strip or back/forward.
- [03:34] Types `hello` in URL bar; status "Waiting for www.google.com..." (X stop icon at left).
- [03:35] Google results for `hello` (Vietnamese UI: "Chế độ AI", "Tất cả", "Hình ảnh", "Video", "Video ngắn", "Mua sắm", "Tin tức", "Thêm", "Công cụ", "Đăng nhập"); yellow popup "Looking for results in English?". AI overview starts "Xin chào! "Hello" là một từ tiếng Anh thông dụng...".
- [03:36] Blank new tab (URL bar focused): he opened a new tab, which replaces the visible view since there is no tab strip.
- [03:37] Types `abc`; status "Transferring data from www.google.com...".
- [03:38] Results for `abc` loaded; cursor over the extensions icon; status "The TLS handshake finished for ogs.google.com...".
- [03:39] Clicks the "List all tabs" icon (left of the extensions icon): dropdown with "Search All Tabs  ⇧⌘A", "New Container Tab >", "Current window": "New Tab", "hello - Tìm trên Google" (highlighted), "abc - Tìm trên Google"; "View All Tabs".
- [03:40] Picks hello: URL bar and page switch to `hello` results.
- [03:41] Opens the dropdown again (hello highlighted, abc below).
- [03:42] Picks abc: page switches back to `abc`; cursor near Google apps icon.
- [03:45] macOS menu bar appears: "Firefox File Edit View History Bookmarks Profiles Tools Window Help", clock "Fri Nov 6 6:22 PM"; same `abc` page.

## Texts on screen

### Rule edited live in the Browser Toolbox Rules pane (2:34 - 2:50)
```
#TabsToolbar {
    display: none;
}
```
(source file shown as `toolbarbuttons.css`)

### userChrome.css, first version in vim (2:49 - 3:01)
```
#TabsToolbar {
  display: none;
}
```
Vim status: `<ome.css" [New] 3L, 34B written`

### userChrome.css, final version (3:17 - 3:25)
```
#TabsToolbar, #back-button, #forward-button {
  display: none;
}
```
(In the 3:20 frame the final `n` of `#forward-button` is under the block cursor, in 3:22 it reads `#forward-butto` + cursor, in 3:23 `#forward-button`.) Vim status: `<serChrome.css" 3L, 65B written   1,43   All`.

### Toolbar context menu (3:26)
```
Turn on Vertical Tabs
Bookmarks Toolbar >
Customize Toolbar...
```

### Customize Toolbar footer (3:28)
```
[ ] Title Bar   [Toolbars v]   Window density settings   Manage themes   [Customize Touch Bar...] [Restore Defaults] [Done]
```

### List all tabs dropdown (3:39 - 3:42)
```
Search All Tabs   ⇧⌘A
New Container Tab >
Current window
  New Tab
  hello - Tìm trên Google
  abc - Tìm trên Google
View All Tabs
```

### Quit dialog (3:19)
```
Close window and quit Firefox?
[x] Ask before quitting with ⌘Q
[Cancel] [Quit Firefox]
```

### Google popup (3:35 - 3:45)
```
Looking for results in English?
Change to English
Tiếp tục sử dụng Tiếng Việt
Cài đặt ngôn ngữ
```
(translation) "Continue using Vietnamese" / "Language settings"

## Code & shell
Vim (in macOS Terminal), new file `userChrome.css`; the element ids are copied from the Browser Toolbox and pasted (mode line `-- INSERT (paste) --`):
```
1  #TabsToolbar, #back-button, #forward-button {
2    display: none;
3  }
```
Selectors/ids and values seen in the Browser Toolbox: `#TabsToolbar`, `#back-button`, `#forward-button`, `#nav-bar-customization-target`, `#stop-reload-button`; `command="Browser:BackOrBackDuplicate"`, `command="Browser:ForwardOrForwardDuplicate"`; `list-style-image: url("chrome://browser/skin/back.svg");`, `list-style-image: url("chrome://browser/skin/forward.svg");`.

## Observations
- Teaches restyling Firefox's own UI with userChrome.css: open the Browser Toolbox (Parent process only mode), use the element picker to find ids (`#TabsToolbar`, `#back-button`, `#forward-button`), test `display: none;` live in the Rules pane, then copy the ids into a userChrome.css in vim and restart Firefox.
- Workflow: live-edit in devtools to see effect immediately, then persist in a file (vim, pasting ids copied from the inspector); quit via the "Close window and quit Firefox?" dialog and relaunch so the stylesheet is loaded.
- Result: no tab strip, no back/forward buttons; navigation relies on the URL bar and the "List all tabs" dropdown, demonstrated with two Google searches (`hello`, `abc`) and switching between them. Implied philosophy: strip visual clutter, minimalism. Customize Toolbar mode was opened (empty flexible spaces around the URL bar) but items were not otherwise changed in this segment.
- The webcam overlay shows the author talking throughout; no audio used.
