# Segment 3 (02:02 - 03:05) - VSCodium, rc5dnqN2IXc

## Timeline
- [02:02] VSCodium window "xv6-riscv" (macOS), light theme, Restricted Mode banner on top: "Restricted Mode is intended for safe code browsing. Trust this folder to enable all features." with links "Manage", "Learn More". Explorer (XV6-RISCV) lists: .github, kernel, mkfs, user, .clang-format, .dir-locals.el, .editorconfig, .gdbinit.tmpl-riscv, .gitignore, LICENSE, Makefile, README, test-xv6.py. Empty editor shows the VSCodium logo and hints "Show All Commands (shift cmd P)", "Go to File (cmd P)", "Toggle Terminal (ctrl `)". Mouse points at the Firefox icon in the Dock.
- [02:03-02:05] Firefox opens. Address bar typed: "vsc" (suggestions include a GitHub VSCodium/vscodium result), then "vscode disab", then "vscode disable trust"; suggestion "vscode disable trust folder at DuckDuckGo" is selected (URL duckduckgo.com/?t=ffab&q=vscode+disable+trust+folder).
- [02:06] Search tab "vscode disable trust folder" on DuckDuckGo loading.
- [02:07-02:08] Results: Stack Overflow "How can I disable workspace trust in VS Code 1.57?"; "Workspace Trust - Visual Studio Code" (code.visualstudio.com/docs/editing/workspaces/workspace-trust); GitHub vscode-docs workspace-trust.md; Stack Overflow "How to untrust workspace by default in VSCode?". He clicks the first Stack Overflow result.
- [02:09] Stack Overflow question page: "I just updated to VS Code 1.57 and am now getting prompted about trusting my workspaces. How can I disable this and go back to the old behavior?" (tag vs-code, 61 votes, 2 answers, viewed 29k times). Top answer (77 votes) begins "Before disabling trusted workspaces entirely, make sure you understand what trusted workspaces are and why they can be helpful."
- [02:10-02:11] Scrolled to answer: heading "Disabling workspace trust entirely" with the setting and code block; second heading about disabling just the prompt (startupPrompt "never"). (Text under Texts.) The webcam covers the right edge.
- [02:16] Back in VSCodium; Restricted Mode banner still shown.
- [02:17-02:18] A macOS file-open dialog (folder xv6-riscv; entries user, test-xv6.py, mkfs, kernel, README, Makefile, LICENSE) opens and fades out; nothing is picked.
- [02:19] Cmd+P quick open: "Search files by name (append : to go to line or @ to go to symbol), or paste a link".
- [02:20-02:21] Command palette ">" typed "open u" then "open user"; results: "Preferences: Open User Settings", "Preferences: Open User Settings (JSON)" (highlighted), "Tasks: Open User Tasks".
- [02:22] settings.json opens (path ~/Library/Application Support/VSCodium/User/settings.json; the home folder name is an account name, omitted) as a floating panel, 10 lines (see Code).
- [02:23] Cursor at end of line 9; a comma is added after "none".
- [02:24] New line 10 typed: "security.workspace.trust.enabled": false
- [02:25] Dialog: "A setting has changed that requires a restart to take effect. Press the restart button to restart VSCodium and enable the setting." Buttons Cancel / Restart; he clicks Restart.
- [02:26] Desktop is empty grey (VSCodium quit); only the webcam window remains.
- [02:27] VSCodium relaunched: no Restricted Mode banner anymore; empty-editor hints changed to "Show All Commands (shift cmd P)", "Open Settings (cmd ,)", "Start Debugging (F5)".
- [02:30] Command palette ">" shows "Preferences: Open User Settings (JSON)" as recently used, "File: Open Folder...", then a long command list.
- [02:31] Typed "> theme": "Preferences: Browse Color Themes in Marketplace", "Preferences: Color Theme" (cmd K cmd T), "Preferences: File Icon Theme", "Preferences: Product Icon Theme", "Preferences: Toggle between Light/Dark Themes", "Preferences: Try New Default Themes", "Developer: Generate Color Theme From Current Settings".
- [02:32] Color Theme picker titled "Select Color Theme for High Contrast Light Mode": Dark High Contrast, Light High Contrast (Default High Contrast Light), then light themes: Light 2026 (Default Light), Light (VSCodium) (Visual Studio Light), Light Modern, Light+, Quiet Light, Solarized Light; dark themes: Dark 2026 (Default Dark), Abyss, Dark (VSCodium) (Visual Studio Dark) ...
- [02:33-02:34] He arrows through light themes (Light (VSCodium), then Light Modern) with live preview; window chrome becomes greyer/lighter.
- [02:36-02:38] Picker closed; Light Modern look remains (pale grey sidebar, near-white editor, faint logo). At 02:38 the window has been moved left on the desktop.
- [02:39] Explorer: "user" folder expanded: cat.c, dorphan.c, echo.c, forktest.c, forphan.c, grep.c, grind.c, init.c, kill.c, ln.c, logstress.c, ls.c, mkdir.c, printf.c, rm.c, sh.c, stressfs.c, sync.c, ulib.c, umalloc.c, user.h, user.ld, usertests.c, usys.pl, wc.c, zombie.c (with file-type icons). Mouse on cat.c.
- [02:40] cat.c opened (code below). Breadcrumb "user > cat.c"; title "cat.c - xv6-riscv".
- [02:41-02:43] Explorer scrolled down to the bottom (.clang-format ... test-xv6.py).
- [02:44-02:45] Command palette: "> t" (top: "Preferences: Color Theme" recently used), then "> file i", then "> file icon": "Preferences: File Icon Theme", "Preferences: Product Icon Theme", "Terminal: Change Icon...".
- [02:46] File Icon Theme picker "Select File Icon Theme (Up/Down Keys to Preview)": None (Disable File Icons), Minimal (VSCodium) vs-minimal, Seti (VSCodium) vs-seti (current), VSCodium Modern Icons vscode-modern-icons, "Install Additional File Icon Themes...".
- [02:48] He picks None: all file icons vanish from the explorer and from the breadcrumb (plain names only). Frames 02:48-02:51 show the icon-less explorer being scrolled with the mouse.
- [02:53] Palette typed "> st" (unrelated commands). At this moment the explorer shows the folder "user" pinned at the top of the scrolled tree (tree sticky scroll active).
- [02:54] Typed "> sticky": "Terminal: Toggle Sticky Scroll", "View: Toggle Editor Sticky Scroll", "View: Toggle Notebook Sticky Scroll", "View: Toggle Tree Sticky Scroll".
- [02:56-02:57] Palette closed. At 02:56 the explorer top is reset and cat.c is selected; at 02:57 explorer scrolled to bottom with no pinned "user" header (appears tree sticky scroll was toggled off - inferred from "recently used" marking at 03:00, not directly shown).
- [02:59] Editor scrolled to end of cat.c (lines 26-44); line 26 "{" stays pinned at the top (editor sticky scroll showing).
- [03:00] Hover on that pinned "26 {" row.
- [03:01] Cmd+P quick open shows "cat.c user" (recently opened) and "settings.json ~/Library/Application Support/VSCodium/User".
- [03:02] Palette "> stick": "View: Toggle Tree Sticky Scroll" (recently used), "Focus Editor Sticky Scroll", "Terminal: Toggle Sticky Scroll", "View: Toggle Editor Sticky Scroll", "View: Toggle Notebook Sticky Scroll".
- [03:03-03:05] Hovering "View: Toggle Editor Sticky Scroll" shows tooltip (see Texts). Segment ends here; whether he toggles it is not visible.
- Webcam window ("Human"): author looks at screen; briefly holding a cigarette around 02:08-02:09. Nothing else notable.

## Texts on screen

### Stack Overflow answer (visible 02:09-02:11)
```
Before disabling trusted workspaces entirely, make sure you understand what trusted workspaces are and why they can be helpful.

Disabling workspace trust entirely
If you still want to disable trusted workspaces, use the Security > Workspace > Trust: Enabled setting:

"security.workspace.trust.enabled": false

This will go back to the behavior in VS Code 1.56.

Disabling the workspace trust p⟨?⟩
If you just want to disable the prompt that shows when op⟨?⟩
set:

"security.workspace.trust.startupPrompt": "never"⟨?⟩

This will disable the prompt but keep trusted workspaces ⟨?⟩
that this means that all new workspace will start in untrust⟨?⟩
```

### Restricted Mode banner (visible 02:02-02:25, gone after restart at 02:27)
```
Restricted Mode is intended for safe code browsing. Trust this folder to enable all features.   Manage   Learn More
```

### Restart dialog (02:25)
```
A setting has changed that requires a restart to take effect.
Press the restart button to restart VSCodium and enable the setting.
[Cancel] [Restart]
```

### Tooltip (03:03-03:05)
```
Toggle/enable the editor sticky scroll which shows the nested scopes at the top of the viewport
```

## Code & shell

### settings.json (VSCodium user settings), final state 02:24-02:25
```
 1 {
 2     "workbench.activityBar.location": "hidden",
 3     "workbench.statusBar.visible": false,
 4     "workbench.editor.showTabs": "none",
 5     "window.commandCenter": false,
 6     "workbench.layoutControl.enabled": false,
 7     "workbench.editor.editorActionsLocation": "hidden",
 8     "workbench.browser.showInTitleBar": false,
 9     "workbench.startupEditor": "none",
10     "security.workspace.trust.enabled": false
11 }
```
(Before 02:23 the file had 10 lines: line 9 was `"workbench.startupEditor": "none"` without comma and line 10 the closing brace. He added the comma and line 10.)

### user/cat.c (seen 02:40-03:05; lines 27-28 seen at 02:40, 33-44 at 02:59)
```
 1 #include "kernel/types.h"
 2 #include "kernel/fcntl.h"
 3 #include "user/user.h"
 4
 5 char buf[512];
 6
 7 void
 8 cat(int fd)
 9 {
10   int n;
11
12   while ((n = read(fd, buf, sizeof(buf))) > 0) {
13     if (write(1, buf, n) != n) {
14       fprintf(2, "cat: write error\n");
15       exit(1);
16     }
17   }
18   if (n < 0) {
19     fprintf(2, "cat: read error\n");
20     exit(1);
21   }
22 }
23
24 int
25 main(int argc, char *argv[])
26 {
27   int fd, i;
28
29   if (argc <= 1) {
30     cat(0);
31     exit(0);
32   }
33
34   for (i = 1; i < argc; i++) {
35     if ((fd = open(argv[i], O_RDONLY)) < 0) {
36       fprintf(2, "cat: cannot open %s\n", argv[i]);
37       exit(1);
38     }
39     cat(fd);
40     close(fd);
41   }
42   exit(0);
43 }
44
```

## Observations
- This segment is a live settings walkthrough, not a talk: he strips VSCodium down so little but the code text remains. Already in settings.json: activity bar hidden, status bar hidden, tabs hidden, command center off, layout control off, editor actions hidden, browser title-bar button off, no startup editor.
- Turned off / removed here (no on-screen explanation is given; reasons are only implied by his actions):
  1. Workspace Trust: the "Restricted Mode" banner is dropped after googling "vscode disable trust folder", reading Stack Overflow, and setting security.workspace.trust.enabled to false (restart required; banner gone afterwards).
  2. File icons: File Icon Theme set to "None (Disable File Icons)".
  3. Color theme: browsed light themes and settled on a muted Light Modern look.
  4. Sticky scroll: looked at tree and editor sticky scroll toggles; tree sticky scroll appears to be off afterwards, editor sticky scroll was still shown at 03:00 and he was inspecting its toggle at the segment end.
- No sticky notes, notes app or spreadsheet in this segment; text comes from VSCodium UI, a browser search and C code (xv6-riscv user/cat.c).
- Method: keyboard-driven (cmd+shift+P, cmd+P), edits settings JSON directly rather than the GUI, and uses a web search for unfamiliar settings (not an AI assistant). Shows a minimalist preference for reading plain C (a tiny utility like cat) with minimal UI chrome.
