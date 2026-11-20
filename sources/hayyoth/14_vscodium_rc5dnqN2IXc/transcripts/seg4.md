# Segment 4 (3:05 - 4:06) - VSCodium: stripping the UI down

## Timeline
- [03:05] VSCodium window "cat.c - xv6-riscv" (macOS), cat.c scrolled to lines 27-44 (minimap visible top right). Command Palette open, typed `> stick`. Results: "View: Toggle Tree Sticky Scroll" (recently used), "Focus Editor Sticky Scroll", "Terminal: Toggle Sticky Scroll", "View: Toggle Editor Sticky Scroll" (tooltip: "Toggle/enable the editor sticky scroll which shows the nested scopes at the top of the viewport").
- [03:08-03:09] Palette closed; minimap still shown; cursor hovers over the minimap.
- [03:10] Right-click on minimap opens a context menu: "Minimap", "Render Characters" (checked), "Vertical size >", "Slider >", "Side >".
- [03:11] "Minimap" item highlighted with a check mark (about to be clicked).
- [03:12] Minimap is gone (editor right edge clean), cursor in editor.
- [03:14] Menu bar visible (VSCodium, File, Edit, Selection, View, Go, Run, Terminal, Window, Help; clock "Wed Oct 28 2:35 PM"). View menu open: Command Palette... (shift-cmd-P), Open View..., Appearance >, Editor Layout >, Explorer (shift-cmd-E), Search (shift-cmd-F), Source Control (ctrl-shift-G), Run (shift-cmd-D), Extensions (shift-cmd-X), Browser (opt-cmd-/), Problems (shift-cmd-M), Output (shift-cmd-U), Debug Console (shift-cmd-Y), Terminal (ctrl-`), Word Wrap (opt-Z), Enter Full Screen.
- [03:15] View > Appearance submenu: Full Screen, Zen Mode [cmd-K Z], Centered Layout, Primary Side Bar (checked, cmd-B), Secondary Side Bar (opt-cmd-B), Status Bar (unchecked), Panel (cmd-J), Move Primary Side Bar Right, Activity Bar Position >, Panel Position >, Align Panel >, Tab Bar >, Editor Actions Position >, Minimap (now unchecked), Breadcrumbs (checked), Sticky Scroll (unchecked), Render Whitespace >, Render Control Characters (checked), Zoom In (cmd-=), Zoom Out (cmd--), Reset Zoom [cmd-NumPad0].
- [03:16-03:18] Cursor sweeps the submenu (Status Bar, Align Panel/Editor Actions Position, then Breadcrumbs highlighted with check). Webcam shows him smoking; only change besides cursor.
- [03:19] Menu closed. Breadcrumb bar ("user > cat.c") has disappeared; editor shifted up slightly. Breadcrumbs now off.
- [03:20] Scrolled to top of cat.c: lines 1-34 visible, line numbers on.
- [03:21] Same view, cursor moved.
- [03:22] Settings UI (modal) opened: tabs User / Workspace, "Commonly Used": "Editor: Font Size" = 12 ("Controls the font size in pixels."), "Editor: Format On Save" (unchecked), "Files: Auto Save" = off.
- [03:23] Click in Font Size field, value 12.
- [03:25] Font Size changed to 14 (modified-setting marker shown); editor text behind enlarges.
- [03:26] Settings closed; cat.c at larger font (14), lines 1-29.
- [03:27] Scrolled down to lines 7-35.
- [03:28] Command Palette opened (empty `>`): "View: Toggle Editor Sticky Scroll" (recently used), "View: Toggle Tree Sticky Scroll", "Preferences: File Icon Theme", "Preferences: Color Theme" (cmd-K cmd-T), "Preferences: Open User Settings (JSON)", "File: Open Folder...", "Accounts: Manage Accounts", "Accounts: Manage Extension Account Preferences...", "Accounts: Manage Trusted Extensions For Account", "Add Cursor Above", "Add Cursor Below".
- [03:29] Typed `> open us`; top hit "Preferences: Open User Settings (JSON)" (recently used); also "Preferences: Open User Settings", "Tasks: Open User Tasks".
- [03:30] settings.json opened (path ~/Library/Application Support/VSCodium/U...). 18 lines, see Code & shell.
- [03:31-03:32] Menu bar shown, File menu opened (New Text File, New File..., New Window, New Window with Profile, Open..., Open Folder..., Open Workspace from File..., Open Recent, Add Folder to Workspace..., Save Workspace As..., Duplicate Workspace, Save, Save As..., Save All, Share, Auto Save, Revert File, Close Editor, Close Folder [cmd-K F], Close Window). Cursor on "Auto Save" (toggled).
- [03:33] settings.json now has 19 lines: new line 18 `"files.autoSave": "afterDelay"` (File > Auto Save wrote it).
- [03:34] Scrolled in settings.json; line 17 `"editor.fontSize": 14,`.
- [03:36-03:37] He types a new line 19: `"editor.` with autocomplete list (acceptSuggestionOnCommitCharacter ... autoClosingOvertype), then `"editor.line` -> lineHeight, lineNumbers, linkedEditing...
- [03:38] `"editor.lineNumbers": ` with value choices "interval", "off", "on" (Default value), "relative".
- [03:40] `"editor.lineNumbers": "off",` ("off" selected).
- [03:41-03:44] New line 20 typed: `"e`, `"editor.g`, `"editor.gy`, `"editor.gyl`, back to `"editor.g` (glyphMargin offered first, then gotoLocation..., guides.bracketPairs). Line numbers in the editor behind are now gone and code shifts left.
- [03:45-03:46] Code behind shifts left further (gutter shrinking); `"editor.g"` with suggestions.
- [03:48] `"editor.glyphMargin": true` (true highlighted).
- [03:50] Changed to `"editor.glyphMargin": false`.
- [03:51] Settings JSON closed. Editor has no line numbers, no glyph margin; code flush against sidebar at larger font.
- [03:52] Scrolled to top: the `#include` lines visible without numbers (cursor at start of line 1).
- [03:53] Scrolled slightly.
- [03:54] Cursor moves to Explorer sidebar; sidebar scrolled to show "kernel", "mkfs", "user" (expanded) with files cat.c (selected), dorphan.c, echo.c, forktest.c, forphan.c, grep.c, grind.c, init.c, kill.c, ln.c, logstress.c, ls.c, mkdir.c, printf.c, rm.c, sh.c, stressfs.c, sync.c, ulib.c, umalloc.c, user.h, user.ld, usertests.c, usys.pl...
- [03:55] Sidebar scrolls up a little (a folder above kernel, kernel, mkfs, user).
- [03:56] Sidebar scrolled back down; cursor in editor on closing `}` of the if block in cat() (bracket highlight on `if (n < 0) {`).
- [03:57] Bottom Panel opened (cmd-J): tabs PROBLEMS, OUTPUT, TERMINAL, "..."; terminal (zsh) with prompt "xv6-riscv $" and hint "Show suggestions [^][Space]. Start typing to dismiss or don't show this again."
- [04:00] Right-click on PROBLEMS tab: context menu: Configure Keybinding, Hide 'Problems', Hide Badge, Move To >, [checked] Problems, Output, Debug Console, Terminal, Ports; Panel Position >, Align Panel >, Show Icons, Hide Panel (cmd-J).
- [04:02] Problems tab gone; tabs now OUTPUT, DEBUG CONSOLE, TERMINAL, PORTS.
- [04:03] Right-click on OUTPUT tab; menu with "Hide 'Output'"; Problems now unchecked, Output highlighted.
- [04:04] Output hidden; tabs DEBUG CONSOLE, TERMINAL, PORTS.
- [04:05-04:06] Right-click on DEBUG CONSOLE tab: menu "Hide 'Debug Console'"; Problems and Output unchecked; Debug Console highlighted (about to hide). Webcam: he looks down at the screen.

## Texts on screen

### Command Palette, "stick" query (03:05)
```
> stick
View: Toggle Tree Sticky Scroll        recently used
Focus Editor Sticky Scroll             other commands
Terminal: Toggle Sticky Scroll
View: Toggle Editor Sticky Scroll
  Toggle/enable the editor sticky scroll which shows the nested scopes at the top of the viewport
```

### Minimap context menu (03:10-03:11)
```
Minimap
-------
v Render Characters
Vertical size >
Slider >
Side >
```

### View > Appearance submenu (03:15-03:18)
```
Full Screen                      ^ cmd F
Zen Mode [cmd K Z]
Centered Layout
[v] Primary Side Bar             cmd B
    Secondary Side Bar           opt cmd B
    Status Bar
    Panel                        cmd J
Move Primary Side Bar Right
Activity Bar Position >
Panel Position >
Align Panel >
Tab Bar >
Editor Actions Position >
    Minimap
[v] Breadcrumbs
    Sticky Scroll
Render Whitespace >
[v] Render Control Characters
Zoom In (cmd =) / Zoom Out (cmd -) / Reset Zoom [cmd NumPad0]
```
(Checks as seen at 03:15-03:18; Minimap and Status Bar already unchecked.)

### Settings UI (03:22-03:25)
```
Settings   [User | Workspace]   Search settings
Commonly Used
Editor: Font Size
Controls the font size in pixels.
[12] -> [14]
Editor: Format On Save
[ ] Format a file on save. A formatter must be available and the editor must not be shutting down. When Files: Auto Save is set to afterDelay, the file will only be formatted when saved explicitly.
Files: Auto Save
Controls auto save of editors that have unsaved changes.
[off]
```

### Command Palette, empty (03:28)
```
>
View: Toggle Editor Sticky Scroll   recently used
View: Toggle Tree Sticky Scroll
Preferences: File Icon Theme
Preferences: Color Theme            cmd K cmd T
Preferences: Open User Settings (JSON)
File: Open Folder...
Accounts: Manage Accounts                              other commands
Accounts: Manage Extension Account Preferences...
Accounts: Manage Trusted Extensions For Account
Add Cursor Above
Add Cursor Below
```

### Panel tab context menu (04:00-04:06)
```
Configure Keybinding
Hide '<tab>'
Hide Badge
Move To >
[v] Problems  [v] Output  [v] Debug Console  [v] Terminal  [v] Ports
Panel Position >
Align Panel >
Show Icons
Hide Panel   cmd J
```
Terminal tab: `xv6-riscv $` (zsh), with a first-run hint "Show suggestions [^][Space]. Start typing to dismiss or don't show this again."

## Code & shell

### User settings.json (final version seen, 03:48-03:50)
Title path: `~/Library/Application Support/VSCodium/U...` (truncated). Lines 1-5 are from the 03:30-03:40 frames; line 11 is cut at the right edge of the viewer.
```
 1  {
 2      "workbench.activityBar.location": "hidden",
 3      "workbench.statusBar.visible": false,
 4      "workbench.editor.showTabs": "none",
 5      "window.commandCenter": false,
 6      "workbench.layoutControl.enabled": false,
 7      "workbench.editor.editorActionsLocation": "hidden",
 8      "workbench.browser.showInTitleBar": false,
 9      "workbench.startupEditor": "none",
10      "security.workspace.trust.enabled": false,
11      "workbench.preferredHighContrastLightColorTheme": "Light Modern",
12      "workbench.iconTheme": null,
13      "workbench.tree.enableStickyScroll": false,
14      "editor.stickyScroll.enabled": false,
15      "editor.minimap.enabled": false,
16      "breadcrumbs.enabled": false,
17      "editor.fontSize": 14,
18      "files.autoSave": "afterDelay",
19      "editor.lineNumbers": "off",
20      "editor.glyphMargin": false
21  }
```
At 03:30 the file had 18 lines (line 17 `"editor.fontSize": 14` with no trailing comma, `}` on line 18). Lines 2-16 were already present when first seen.

### cat.c (xv6-riscv user/cat.c), assembled from 03:08 (lines 27-44), 03:20 (1-34), 03:25 (1-29), 03:51-03:56
Line numbers were shown until about 03:44, then turned off.
```
 1  #include "kernel/types.h"
 2  #include "kernel/fcntl.h"
 3  #include "user/user.h"
 4
 5  char buf[512];
 6
 7  void
 8  cat(int fd)
 9  {
10    int n;
11
12    while ((n = read(fd, buf, sizeof(buf))) > 0) {
13      if (write(1, buf, n) != n) {
14        fprintf(2, "cat: write error\n");
15        exit(1);
16      }
17    }
18    if (n < 0) {
19      fprintf(2, "cat: read error\n");
20      exit(1);
21    }
22  }
23
24  int
25  main(int argc, char *argv[])
26  {
27    int fd, i;
28
29    if (argc <= 1) {
30      cat(0);
31      exit(0);
32    }
33
34    for (i = 1; i < argc; i++) {
35      if ((fd = open(argv[i], O_RDONLY)) < 0) {
36        fprintf(2, "cat: cannot open %s\n", argv[i]);
37        exit(1);
38      }
39      cat(fd);
40      close(fd);
41    }
42    exit(0);
43  }
44
```
(Indentation is 2 spaces per level as rendered; exact tab/space not determinable.)

## Observations
- This segment is a live configuration demo, not a talk: he removes visual chrome from VSCodium one item at a time via three routes: right-click context menus (minimap, panel tabs), View > Appearance (breadcrumbs, status bar, sticky scroll), and editing user settings.json directly (typing keys with autocomplete).
- Turned off / hidden here: minimap (editor.minimap.enabled=false), breadcrumbs (breadcrumbs.enabled=false), sticky scroll in editor and tree (editor.stickyScroll.enabled=false, workbench.tree.enableStickyScroll=false; looked up in the Command Palette), status bar (workbench.statusBar.visible=false), line numbers (editor.lineNumbers="off"), glyph margin (editor.glyphMargin=false, after briefly typing true), and panel tabs Problems, Output, and (about to be) Debug Console, leaving Terminal and Ports. settings.json also already shows activity bar hidden, tabs "none", command center off, layout control off, editor actions hidden, startup editor "none", workspace trust disabled, icon theme null.
- Turned on / changed: font size 12 -> 14 (Settings UI), Auto Save "afterDelay" (File > Auto Save, which writes to settings.json). Format on Save left unchecked.
- No reasons are written on screen in this segment (no sticky notes or captions); the motive is only implicit: a minimal, text-only editor without distractions. He names each toggle only by operating it.
- He prefers the JSON file over the Settings UI for most items and relies on autocomplete to find setting names, showing that the whole configuration is a short plain-text file.
- cat.c from xv6-riscv is only a vehicle to show how the editor looks after stripping; nothing is edited in it. Explorer sidebar (user/ folder) is kept.
- Webcam overlay: he appears to smoke on camera at times (03:17-03:18, 04:03); otherwise only posture changes.
