# Segment 1 (0:00 - 1:17) - macOS - but better - No Apple

## Timeline
- [00:00] Desktop with blue macOS wallpaper (clock Wed Oct 28 3:16 PM). Finder window "Movies" in column view (sidebar Favorites: <account> home, Desktop, Documents, Downloads, sources, lab; column 1: Desktop, Documents, Downloads, lab, Movies, Music, notes, Pictures, Public, sources; column 2: TV). Behind it: System Settings (General pane) and a Notes window. An empty yellow Stickies note at top right. Webcam "Human" window bottom right (ignored). Finder sidebar is the floating rounded panel (before state).
- [00:01] Stickies app active (menu: Stickies File Edit Font Color Window Help); the yellow sticky has a text cursor but is empty.
- [00:14] Finder frontmost again; sticky no longer shown.
- [00:16] System Settings brought to front, General pane. Sidebar: Search, "Sign in with your Apple Account", "Software Update Available (1)", Wi-Fi, Bluetooth, Network, Battery, General (selected), Accessibility, Appearance, Apple Intelligence & Siri, Desktop & Dock, Displays, Menu Bar, Spotlight, Wallpaper... Right pane: About, Software Update, Storage, AppleCare & Warranty, AirDrop & Continuity, AutoFill & Passwords, Date & Time, Language & Region. No setting is changed in this segment.
- [00:20] Terminal opened at top right, empty prompt "~ $". Windows shift slightly at 00:20-00:21 (animation).
- [00:23] Notes app brought forward: note titled "Notes" dated October 28, 2026 at 3:10 PM, list snippet "Remove float sidebar". Two code snippets visible (see Texts on screen).
- [00:27] Terminal: first command pasted and selected: `defaults write -g NSSplitViewItemSidebarDefaultsToFloatingAppearance -bool false` (wraps over 2 lines).
- [00:28] Command executed; new prompt "~ $" below it.
- [00:29] Back in Notes; first snippet highlighted pink (selected/copied).
- [00:34] Terminal: second command pasted and selected: `defaults write -g NSConvolutionOverride1 -float 4`.
- [00:35] Second command executed; fresh prompt.
- [00:36-00:37] Finder brought to front, windows reshuffled; Finder sidebar still floating rounded.
- [00:39] "C Camera" app active (webcam app); not relevant.
- [00:42-00:45] Terminal shows a cleared prompt "~ $"; he drags the Terminal window to different positions.
- [00:48] Apple menu opened: About This Mac, System Settings... (1 update), App Store, Recent Items, Force Quit..., Sleep, Restart..., Shut Down..., Lock Screen, Log Out <account>...
- [00:49] Hovers "Force Quit...".
- [00:50] Force Quit Applications window: Firefox, Notes, QuickTime Player, Stickies, System Settings, Terminal, Finder (selected). Button "Relaunch".
- [00:52] Dialog: "Do you want to force Finder to relaunch? You will lose any unsaved changes." Cancel / Relaunch (cursor on Relaunch).
- [00:53] Dialog fading, Finder selected in list.
- [00:54] Finder is relaunching: list now shows "camera, Firefox, Notes, QuickTime Player, Stickies, System Settings, Terminal" and the button reads "Force Quit" (greyed).
- [00:55] Finder window reappears with NEW look: the sidebar is now a flat full-height column (no longer a floating rounded panel), title area "Movies" above the columns.
- [00:56] Finder window dragged lower-left; flat sidebar confirmed.
- [01:00] Notes frontmost (clock now 3:17 PM); second snippet highlighted pink (selected). Finder window at left now shows the flat style.
- [01:01] Dock briefly shows a shorter layout (Notes icon and trash/separator missing) while Notes is being relaunched/reloaded; cursor on desktop.
- [01:02] Spotlight search: typed "news" with "Open"; results News (top), Notes, Maps, Search in Finder, Search the Web; tabs Applications, TextEdit...
- [01:03] Notes reopens (list placeholders loading); more notes are listed beneath (private details not transcribed).
- [01:04] Notes fully loaded, note "Notes / Remove float sidebar" selected.
- [01:06] Notes window dragged to the upper left (position change only).
- [01:07] Back to System Settings/Finder arrangement; Notes hidden. Finder flat sidebar.
- [01:08] Terminal frontmost, empty prompt.
- [01:11] Terminal: `defaults write -g NSConvolutionOverride1 -float 4` selected/pasted (cursor right after "4").
- [01:12] Terminal still shows only the typed command, no output.
- [01:13] Cursor over the Dock Terminal icon (tooltip "Terminal"); Finder frontmost.
- [01:14] Terminal shows an empty prompt "~ $"; window being moved.
- [01:16] Terminal at top right, empty prompt; Finder frontmost.
- [01:17] Terminal shows `defaults write -g NSConvolutionOverride1 -float 4` highlighted again; menu bar "Edit" highlighted (paste/copy action).

## Texts on screen

### Apple Notes note titled "Notes" (visible 00:23-00:29 and 01:00-01:08; date line "October 28, 2026 at 3:10 PM")
```
Notes

Remove float sidebar
 defaults write -g NSSplitViewItemSidebarDefaultsToFloatingAppearance
-bool false

Change corner radius

defaults write -g NSConvolutionOverride1 -float 4
```
(Notes list entry: "Notes - 3:10 PM - Remove float sidebar". Other notes in the list are personal and not transcribed.)

### Stickies note (00:00-00:01)
```
(empty yellow sticky, cursor only)
```

### Force Quit Applications dialog (00:50-01:17)
```
If an app doesn't respond for a while, select its name and click Relaunch.
Firefox / Notes / QuickTime Player / Stickies / System Settings / Terminal / Finder
You can open this window by pressing Command-Option-Escape.
```
Confirmation: "Do you want to force Finder to relaunch? You will lose any unsaved changes." [Cancel] [Relaunch]

### System Settings (General) sidebar and list (00:16)
```
Sign in with your Apple Account
Software Update Available (1)
Wi-Fi, Bluetooth, Network, Battery, General, Accessibility, Appearance,
Apple Intelligence & Siri, Desktop & Dock, Displays, Menu Bar, Spotlight, Wallpaper...
General: About, Software Update, Storage, AppleCare & Warranty, AirDrop & Continuity, AutoFill & Passwords, Date & Time, Language & Region
```
Shown only; no value changed in this segment.

## Code & shell
Terminal ("~ $" prompt), commands run in order:
```
~ $ defaults write -g NSSplitViewItemSidebarDefaultsToFloatingAppearance -bool false
~ $ defaults write -g NSConvolutionOverride1 -float 4
```
No output shown. Finder was then force-relaunched (Apple menu > Force Quit... > Finder > Relaunch, about 00:48-00:54). The second command is pasted again at 01:11 and 01:17 (selected in terminal; whether it was re-executed is not visible).

## Observations
- Opening move is a "make macOS less flashy" tweak: he turns off the floating (inset, rounded) sidebar style of the new macOS UI via the hidden defaults key NSSplitViewItemSidebarDefaultsToFloatingAppearance = false, and changes the window corner radius via NSConvolutionOverride1 = 4. Before -> after is visible: Finder's sidebar goes from a floating rounded panel to a flat full-height sidebar after the Finder relaunch; corners look sharper.
- No LuLu firewall, Apple Intelligence, Login Items or Desktop & Dock setting is changed in this segment; System Settings is only displayed. No reason for the tweaks is stated on screen.
- Method: commands were prepared beforehand in an Apple Note (created 3:10 PM, with headings "Remove float sidebar" and "Change corner radius"); he copies each into Terminal, runs it, then force-relaunches Finder to apply. Spotlight is used to launch an app ("news" typed, Notes second result). The empty Stickies note at the start is not filled in this segment.
- Style: terse, practical, shell-first; prefers config via defaults write over GUI toggles.
