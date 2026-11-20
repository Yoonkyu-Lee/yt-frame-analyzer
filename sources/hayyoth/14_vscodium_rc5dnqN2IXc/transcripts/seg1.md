# seg1 (0:00-1:05) - VSCodium install and first look

## Timeline
- [00:00] Empty grey desktop, only the "Human" webcam window (bottom right). Author looks down.
- [00:01] Cursor goes to the dock; tooltip "Firefox". Dock icons: Finder, Apps, (windows), Firefox, Terminal, a Q-shaped icon, "exec", a screenshot thumbnail, Trash.
- [00:02] Firefox window opens; address bar shows typing "vs". Dropdown: "vs - Search with DuckDuckGo", Firefox Suggest items ("vscode disable trust folder at DuckDuckGo", Google Translate, YouTube Studio pages, "Hayyoth - YouTube"). Search engine is DuckDuckGo.
- [00:03-00:06] Search "vscode" loads. First result: Visual Studio Code, code.visualstudio.com, "Visual Studio Code - The open source AI code editor | Your home ..." with sitelinks Download, Windows, Docs, Updates, Blog, API. Second: "Download Visual Studio Code". Third: vscode.dev.
- [00:04-00:09] Page is zoomed in heavily (browser zoom) and panned horizontally/vertically while the cursor hovers the first result title (status bar "code.visualstudio.com"). The author only points at it, does not open it.
- [00:09-00:11] He clicks the address bar, types "vscodi" then completes "vscodium" (suggestion: "GitHub - VSCodium/vscodium: binary release...", github.com/VSCodium/vscodium).
- [00:11-00:12] DuckDuckGo results for "vscodium": 1) GitHub - VSCodium/vscodium: binary releases of VS Code witho..., 2) "Install | VSCodium - Open Source Binaries of VSCode" (vscodium.com > install), 3) "VSCodium - Open Source Binaries of VSCode" (vscodium.com), 4) "VSCodium - GitHub". Cursor on result 1; he clicks it.
- [00:15-00:16] GitHub page github.com/VSCodium/vscodium loads: repo "vscodium" Public, Issues 135, Pull requests 14, Fork 1.9k, Star 33.8k, branch master; last commit "fix: replace removed variable wit..." 12 hours ago; folders .github, build, dev, docs, font-size, icons, patches, src, stores, upstream, .ecformat_ignore. About: "binary releases of VS Code without MS branding/telemetry/licensing", link vscodium.com, tags editor, floss, ide, open-source-licensing, vscode; MIT license, Code of conduct, Contributing, Activity.
- [00:17-00:26] Page zoomed way in on the About text. Cursor moves over it and he selects text in steps: first "without" (00:25), then "without MS branding" (00:26) - highlighting the reason for VSCodium: no MS branding / telemetry / licensing. Zoom then relaxes (00:28-00:29), selection "without MS branding" still highlighted.
- [00:30-00:31] Cursor to dock: tooltip "Terminal". Terminal window opens (empty prompt "~ $").
- [00:35] Terminal shows command and output (see Code & shell). Already installed, so nothing happens.
- [00:37] Focus back to Firefox/GitHub window briefly with terminal behind it, webcam shows author holding a cigarette to mouth.
- [00:38-00:39] Firefox closed/hidden: desktop with only Terminal and webcam.
- [00:40] Dock "Apps" tooltip; an Applications launcher grid (title "Applications", search field) fades in over the terminal.
- [00:41] Launcher gone, terminal only.
- [00:42] A large white window appears full screen with traffic-light buttons (VSCodium launching).
- [00:43] VSCodium started: macOS menu bar "VSCodium File Edit Selection View Go Run Terminal Window Help"; clock "Wed Oct 28 2:33 PM". Empty editor with grey VSCodium logo and shortcut hints: Show All Commands (shift-cmd-P), Open Recent (ctrl-R), Open File or Folder (cmd-O), New Untitled Text File (cmd-N). Activity bar on left (Explorer, Search, Source Control, Run and Debug, Extensions, account, gear).
- [00:44] Welcome tab (italic): "VSCodium / Editing evolved"; Start: New File..., Open..., Clone Git Repository..., Connect to...; Recent: "You have no recent folders, open a folder to start."; checkbox "Show welcome page on startup" (checked). Title bar has "Search" box and layout toggle icons. Status bar: 0 errors 0 warnings.
- [00:45-00:47] He shrinks/moves the window (resizes smaller, drags across the desktop). Welcome page layout reflows; at the smallest sizes the "VSCodium / Editing evolved" heading disappears and only Start/Recent remain.
- [00:48-00:50] Terminal window brought back in front of VSCodium and dragged to the right; author again with cigarette (00:48).
- [00:54] Terminal screen cleared (just "~ $").
- [00:59] Terminal closed/hidden; VSCodium window alone.
- [01:00-01:01] VSCodium window moved up-left and slightly resized.
- [01:02] Mouse at left edge shows a horizontal resize cursor.
- [01:03] Right-click in the activity bar area opens context menu: checked items Explorer (shift-cmd-E), Search (shift-cmd-F), Source Control (ctrl-shift-G), Run and Debug (shift-cmd-D), Extensions (shift-cmd-X); checked Accounts; submenus Activity Bar Position, Activity Bar Size; "Move Primary Side Bar Right".
- [01:04] Menu closed (no change yet).
- [01:05] Menu reopened (item at lower position); "Activity Bar Position" highlighted.
- [01:05+] (last frame) Submenu: Default (checked), Top, Bottom, Hidden; cursor on "Hidden". Outcome (selected or not) is not visible in this segment.

## Texts on screen
### GitHub About box (visible 0:15-0:37)
```
About
binary releases of VS Code
without MS branding/
telemetry/licensing
vscodium.com
editor  floss  ide
open-source-licensing
vscode
```
(Highlighted by the author: "without MS branding".)

### VSCodium welcome page (0:44-1:05)
```
VSCodium
Editing evolved

Start
  New File...
  Open...
  Clone Git Repository...
  Connect to...

Recent
  You have no recent folders, open a folder to start.

[x] Show welcome page on startup
```

### Editor empty-state hints (0:43)
```
Show All Commands        shift cmd P
Open Recent              ctrl R
Open File or Folder      cmd O
New Untitled Text File   cmd N
```

### Activity bar context menu (1:03-1:05)
```
[x] Explorer         shift cmd E
[x] Search           shift cmd F
[x] Source Control   ctrl shift G
[x] Run and Debug    shift cmd D
[x] Extensions       shift cmd X
[x] Accounts
Activity Bar Position >   (Default [x] / Top / Bottom / Hidden)
Activity Bar Size >
Move Primary Side Bar Right
```

## Code & shell
Terminal (macOS, 0:35 onward), window title "Terminal":
```
1  ~ $ brew install vscodium
2  Warning: Not upgrading vscodium, the latest version is already installed
3  ~ $
```
(Terminal output wraps mid-word at "alrea / dy installed".) No C source shown in this segment.

## Observations
- The author starts by searching "vscode", points at the Microsoft site (described on screen as "The open source AI code editor"), then deliberately switches to "vscodium" - the framing is VS Code minus Microsoft: GitHub description "binary releases of VS Code without MS branding/telemetry/licensing", which he highlights word by word.
- Install is one line via Homebrew; VSCodium was already installed, so he just launches it from the Applications launcher. The welcome tagline is "Editing evolved".
- He heavily uses browser zoom and panning to emphasise text on screen, and selects text with the mouse as a pointer.
- At the end of the segment he begins stripping the UI: the activity bar context menu is opened and the cursor goes to Activity Bar Position > Hidden (decision not yet visible). He also repeatedly resizes/moves the window. No stated reasons for removals appear as text in this segment.
- Webcam shows only the author talking/smoking; nothing else changes there.
