# Segment 2 (1:12 - 2:25) - "firefox - but better" (Hayyoth)

Note: the webcam overlay "Human" (bottom right) is a constant talking-head and is ignored. The Firefox window sits at upper left of a grey macOS desktop. Home-directory/account names in file paths are written as `<account>`.

## Timeline
- [01:12] Firefox window, two tabs: "Settings" (about:preferences#home, "Home and startup": Default browser "Firefox is your default browser. Good choice."; Startup checkboxes "Open previous windows and tabs", "Open Firefox automatically when your computer starts up", "Always check if Firefox is your default browser" (checked, greyed); "Homepage / New windows") and "About Profiles". URL bar is being edited; the text reads `about:praferences#home` (typo, cursor after "pra") with dropdown "about:praferences#home - Visit".
- [01:13] URL bar cleared, typing `about`; suggestion list shows Google searches and Firefox Suggest `about:profiles [Switch to Tab]`, "About Us mozilla.org/about/".
- [01:14] URL bar: `about:config` (autocompleted, "fig" highlighted), top item "about:config - Visit"; Firefox Suggest list: about:profiles, about:config, about:checkerboard, about:compat, about:config, about:crashes, about:credits, about:compat, about:config.
- [01:15] about:config warning page (tab "Advanced Preferences"): "Proceed with Caution" / "Changing advanced configuration preferences can impact Firefox performance or security." / checkbox "Warn me when I attempt to access these preferences" (checked) / button "Accept the Risk and Continue".
- [01:16] about:config main page: search box "Search preference name", checkbox "Show only modified preferences", "Show All" button; text "Changing these preferences can impact Firefox performance or security."
- [01:18] Typed `styles` in the search box; results listed (see Texts on screen).
- [01:19] Search refined to `stylesheet`; 5 results plus an "add new pref" row `stylesheet` with radio Boolean (selected) / Number / String and a + button.
- [01:20] Mouse hovers over (highlights) the last result `toolkit.legacyUserProfileCustomizations.stylesheets` (false).
- [01:23] Value of `toolkit.legacyUserProfileCustomizations.stylesheets` toggled false -> true (cursor on value, row now bold with a reset arrow).
- [01:24] Row now bold `true`. Mouse moves up to the "About Profiles" tab (hover preview shows about:profiles).
- [01:25] The about:config tab is closed; About Profiles page: heading "About Profiles", text "This page helps you to manage your profiles. Each profile is a separate world which contains separate history, bookmarks, settings and add-ons.", box "Restart" with buttons "Restart with Add-ons Disabled..." and "Restart normally...", button "Create a New Profile", "Profile: default-release" table. Only one tab remains.
- [01:26] Scrolled down: profiles "default-release" and "TestProfiles" with Root Directory / Local Directory paths and "Show in Finder" buttons (see Texts on screen).
- [01:27] - [01:28] Small scroll; cursor moves to the "Root Directory" label of TestProfiles.
- [01:30] A Finder window (column view, title qq0mrgdf.TestProfiles) opens in front (see Texts on screen).
- [01:32] Right-click context menu on the folder qq0mrgdf.TestProfiles: Open in New Tab, Move to Trash, Get Info, Rename, Compress "qq0mrgdf.TestProfiles", Duplicate, Make Alias, Quick Look, Copy, Share..., colour tags, Customize Folder..., Quick Actions >, Folder Actions Setup..., New Terminal at Folder, New Terminal Tab at Folder.
- [01:33] Cursor on "New Terminal at Folder".
- [01:34] A Terminal window opens with prompt `qq0mrgdf.TestProfiles $` (empty).
- [01:38] Terminal shows `mkdir chrome` was run and `cd chr` is being typed; Finder now shows a new "chrome" folder in the profile directory listing.
- [01:43] Terminal is now a full-window editor (vim) with an empty new file; status line `"userChrome.css" [New]   0,0-1   All`. (The command that launched it, presumably `vim userChrome.css`, was not captured in a frame.)
- [01:46] Window-switching blur/transition animation (Terminal + Finder fading out) back to the Firefox window.
- [01:47] Firefox About Profiles page again, Terminal/Finder gone.
- [01:49] New tab opened: "New Tab", URL bar focused, "Search with Google or enter address".
- [01:50] New tab became "Settings" at about:preferences, "Account and sync" (Account: "You're not signed in"; Sync: "Sign in to your account to start syncing"; "Import browser data"); "Find in Settings" focused.
- [01:51] Back on the single About Profiles tab; cursor on the hamburger menu button (top right).
- [01:52] Hamburger (application) menu open (see Texts on screen); cursor on Passwords.
- [01:53] Cursor on "Find in Page...".
- [01:54] "More Tools" submenu open (no "Browser Toolbox" entry yet; see Texts on screen).
- [01:55] Cursor on "Web Developer Tools".
- [01:57] DevTools panel opens docked at the bottom (Inspector tab: html/body, Layout panel).
- [01:58] Cursor moves to the toolbox responsive/"..." icons; Inspector shows `body.wide-container` rules from info-pages.css.
- [01:59] DevTools options menu ("..." button) open (see Texts on screen).
- [02:00] DevTools Settings page begins: "Default Developer Tools" checkboxes, "Themes" Auto/Dark/Light, "Inspector" options.
- [02:02] DevTools panel enlarged (dragged up): full Settings page.
- [02:03] Scrolled to bottom: "Editor Preferences" and "Advanced settings" (partial).
- [02:04] Scrolled slightly more: full "Advanced settings" list visible, including "Use the F12 key to open or close DevTools" and "Enable custom formatters".
- [02:06] Mouse hovers "Enable browser chrome and add-on debugging toolboxes"; tooltip appears (see Texts on screen).
- [02:08] The checkbox "Enable browser chrome and add-on debugging toolboxes" is now CHECKED.
- [02:11] DevTools closed; About Profiles page full again; cursor at the hamburger button.
- [02:12] Hamburger menu open again, cursor on "Settings".
- [02:13] "More Tools" submenu now lists a new item "Browser Toolbox" (Shift-Option-Cmd-I) between Remote Debugging and Browser Console.
- [02:14] Cursor on "Browser Console" (just below Browser Toolbox).
- [02:15] Menu closed; page again.
- [02:16] A blank window appears and a modal "Incoming Connection" dialog (see Texts on screen) - the Browser Toolbox connecting to the browser's remote-debugging server.
- [02:18] The "Parent process Browser Toolbox" window is open (mode "Parent process only (Fast)" selected, Inspector tab showing the browser's own `html:body` of `html#main-window`); error badge "10".
- [02:19] Toolbox window dragged to the right; Firefox window visible behind it.
- [02:20] Toolbox dragged a bit further; cursor on the element-picker icon (top left of toolbox).
- [02:21] Element picker active: hovering the URL bar highlights `::-moz-text-control-editing-root  381.75 x 28`.
- [02:22] Picker hovering the tab strip: tooltip `html:slot  637 x 36  Flex Container/Item`.
- [02:23] Picker clicked: Inspector selects `<html:slot>` inside `<scrollbox ...>` under `<arrowscrollbox>`; Rules panel shows `slot { display: flex; flex: 1 0 0; flex-direction: inherit; }` and "Inherited from tabs#tabbrowser-tabs ... --tab-min-width-pref: 76px;".
- [02:24] Picker again on the tab strip: tooltip `scrollbox  637 x 36  Flex Container/Item`.
- [02:25] Inspector tree scrolled up; `<arrowscrollbox ... orient="horizont...">`, `#shadow-root`, `<scrollbox pa...>`, `<html:slot>`. Same Rules content.

## Texts on screen

### URL bar text (01:12)
```
about:praferences#home
```
(typo for "preferences"; dropdown shows "about:praferences#home - Visit")

### about:config warning page (01:15)
```
Proceed with Caution
Changing advanced configuration preferences can impact Firefox performance or security.
[x] Warn me when I attempt to access these preferences
[ Accept the Risk and Continue ]
```

### about:config empty search page (01:16)
```
Search preference name        [ ] Show only modified preferences
[ Show All ]
Changing these preferences can impact Firefox performance or security.
```

### about:config search "styles" (01:18), first 13 visible rows
| Preference | Value |
|---|---|
| bidi.edit.caret_movement_style | 2 |
| browser.drag_out_of_frame_style | 1 |
| devtools.debugger.features.stylesheets-in-debugger | false |
| devtools.inspector.showUserAgentStyles | false |
| devtools.styleeditor.atRulesSidebarWidth | 238 |
| devtools.styleeditor.autocompletion-enabled | true |
| devtools.styleeditor.enabled | true |
| devtools.styleeditor.navSidebarWidth | 245 |
| devtools.styleeditor.showAtRulesSidebar | true |
| devtools.styleeditor.transitions | true |
| dom.animations.commit-styles-endpoint-inclusive | true |
| dom.webcomponents.lift-adoptedstylesheets-restriction.enabled | false |
| extensions.browser_style_mv3.same_as_mv2 | false |

?? (list continues below the fold)

### about:config search "stylesheet" (01:19 - 01:24)
| Preference | Value |
|---|---|
| devtools.debugger.features.stylesheets-in-debugger | false |
| dom.webcomponents.lift-adoptedstylesheets-restriction.enabled | false |
| layout.css.stylesheet-cache.timeout_ms | 15000 |
| security.integrity_policy.stylesheet.enabled | false |
| toolkit.legacyUserProfileCustomizations.stylesheets | false, toggled to **true** at 01:23 (bold, with reset button) |

Add-pref row at bottom: `stylesheet`  (o) Boolean  ( ) Number  ( ) String  [+]

### About Profiles page (01:25 - 02:25)
```
About Profiles
This page helps you to manage your profiles. Each profile is a separate world which contains separate history, bookmarks, settings and add-ons.

Restart
[Restart with Add-ons Disabled...]
[Restart normally...]

[Create a New Profile]

Profile: default-release
Default Profile   no
Root Directory    /Users/<account>/Library/Application Support/Firefox/Profiles/z7hv307q.default-release   [Show in Finder]
Local Directory   /Users/<account>/Library/Caches/Firefox/Profiles/z7hv307q.default-release   [Show in Finder]
[Rename] [Remove] [Set as default profile]
[Launch profile in new browser]

Profile: TestProfiles
This is the profile in use and it cannot be deleted.
Default Profile   yes
Root Directory    /Users/<account>/Library/Application Support/Firefox/Profiles/qq0mrgdf.TestProfiles   [Show in Finder]
Local Directory   /Users/<account>/Library/Caches/Firefox/Profiles/qq0mrgdf.TestProfiles   [Show in Finder]
[Rename]
```

### Finder column view (01:30 - 01:43)
```
Favorites: <home folder>, Desktop, Documents, Downloads, sources, lab
Column 2: Crash Reports, Pending Pings, Profile Groups, Profiles, installs.ini, profiles.ini
Column 3 (in Profiles): qq0mrgdf.TestProfiles, z7hv307q.default-release
Column 4 (in qq0mrgdf.TestProfiles), at 01:30: bookmarkbackups, crashes, datareporting, extension-store, extensions, gmp-gmpopenh264, gmp-widevinecdm, minidumps, saved-telemetry-pings, security_state, sessionstore-backups, settings, storage, (files, cut off)
Column 4 after 01:38: bookmarkbackups, chrome, crashes, datareporting, extension-store, extensions, gmp-gmpopenh264, gmp-widevinecdm, minidumps, saved-telemetry-pings, security_state, sessionstore-backups, settings, st...
```

### Hamburger menu (01:52 - 02:12)
```
Sync your data everywhere  / Sign in
Create a New Profile >
New Tab (Cmd-T)
New Window (Cmd-N)
New Private Window (Shift-Cmd-P)
History >
Bookmarks >
Downloads (Cmd-J)
Passwords
Extensions and Themes (Shift-Cmd-E)
Print... (Cmd-P)
Save Page As... (Cmd-S)
Find in Page... (Cmd-F)
Zoom  -  100%  +  [fullscreen]
Settings (Cmd-,)
More Tools >
Help and Report >
```

### More Tools submenu - before (01:54 - 01:55) and after (02:13 - 02:14) enabling browser-chrome debugging
Before:
```
Customize toolbar...
Edit PDF...
Browser tools
Web Developer Tools      Option-Cmd-I
Task Manager             Shift-Esc
Remote Debugging
Browser Console          Shift-Cmd-J
Responsive Design Mode   Option-Cmd-M
Eyedropper
Page Source              Cmd-U
Extensions for developers
```
After (new line added):
```
Web Developer Tools      Option-Cmd-I
Task Manager             Shift-Esc
Remote Debugging
Browser Toolbox          Shift-Option-Cmd-I
Browser Console          Shift-Cmd-J
Responsive Design Mode   Option-Cmd-M
Eyedropper
Page Source              Cmd-U
Extensions for developers
```

### DevTools toolbox "..." menu (01:59)
```
(check) Dock to Bottom
Dock to Right
Dock to Left
Separate Window
Show Split Console   Esc
Settings             F1
Documentation...
Community...
```

### DevTools Settings page (02:00 - 02:08)
```
Default Developer Tools
 [x] Network  [x] Style Editor  [x] Performance  [x] Memory  [x] Storage  [x] Accessibility  [x] Application  [ ] DOM

Available Toolbox Buttons
 [x] Show the number of errors on the page
 [x] Pick an element from the page (Cmd+Shift+C or Cmd+Opt+C)
 [x] Select an iframe as the currently targeted document
 [x] Responsive Design Mode (Cmd+Opt+M)
 [ ] Take a screenshot of the entire page
 [ ] Toggle rulers for the page
 [ ] Measure a portion of the page

Themes:  (o) Auto  ( ) Dark  ( ) Light

Inspector
 [ ] Show Browser Styles
 [ ] Show Browser Shadow DOM (reloads the page)
 [x] Show comments (reloads the page)
 [x] Truncate DOM attributes
 [x] Click and drag to edit size values
 [ ] Use simpler highlighters with prefers-reduced-motion
 [x] Focus next input on Enter
 Default color unit: [As Authored v]

Web Console
 [x] Enable Split Console

Network Monitor
 Maximum request and response body size (set to 0 for unlimited): 1.05 MB

(partially seen) Local Mode allows you to load files via https URL without any external dependency. The URLs can only be loaded from tabs with DevTools opened.  [Add a new local mapping]

Editor Preferences
 [x] Detect indentation
 [x] Autoclose brackets
 [x] Indent using spaces
 Tab size [2 v]
 Keybindings [Default v]

Advanced settings
 [x] Enable Source Maps
 [ ] Disable HTTP Cache (when toolbox is open)
 [ ] Disable JavaScript (current session only, reloads the page)
 [ ] Enable Service Workers over HTTP (when toolbox is open)
 [ ] Enable browser chrome and add-on debugging toolboxes     (becomes [x] at 02:08)
 [ ] Enable remote debugging
 [x] Use the F12 key to open or close DevTools
 [ ] Enable custom formatters
```

### Tooltip on "Enable browser chrome and add-on debugging toolboxes" (02:06)
```
Turning this option on will allow you to use various developer tools in browser context (via Tools > Web Developer > Browser Toolbox) and debug add-ons from the Add-ons Manager
```

### Incoming Connection dialog (02:16)
```
Incoming Connection
An incoming request ⟨?⟩ permit remote debugging connection was detected. A remote client can take complete control over your browser!

Client Endpoint: 127.0.0.1:64789
Server Endpoint: 127.0.0.1:64787

Allow connection?
[Disable]                [Cancel] [OK]
```
(the cursor hides "to" in "request to permit")

### Parent process Browser Toolbox (02:18 - 02:25)
```
Parent process Browser Toolbox
Browser Toolbox Mode   (o) Parent process only (Fast)   ( ) Multiprocess (Slower)
Tabs: Inspector | Console | Debugger | Network | Style Editor | Performance | Memory | Storage | >>   [10 errors]
Rules | Layout | Computed | Changes | Compatibility | Fonts | Animations
```
Inspector tree (02:18): `... height="666" theme-in-app theme-effective-id="default-theme@mozilla.org" fxastatus="not_configured" fxatoolbarmenu="visible" uidensity="compact" sessionrestored="true" devtoolstheme="light"`, `<head> ... </head>`, `<html:body xmlns="http://www.mozilla.org/keymaster/gatekeeper/there.is.only.xul"> [flex]` with children `<stringbundleset id="stringbundleset">`, `<commandset id="mainCommandSet">`, `<commandset id="placesCommands" commandupdater="true" ...>`. Breadcrumb: `html#main-window > html:body`.

Rules (02:18):
```
:root:not([theme-image-in-toolbox]) body, :root[theme-image-in-toolbox] #navigator-toolbox {   browser-shared.css:366
    background-image: var(--toolbox-background-image);
    background-repeat: var(--toolbox-background-repeat);
    background-position: var(--toolbox-background-position);
    background-size: var(--toolbox-background-size);
}
body {   browser-shared.css:318
    contain: paint;
    position: relative;
    background-color: var(--toolbox-background-color);
    color: var(--toolbox-text-color);
    will-change: background-color;
    transition: background-color var(--inactive-window-transition);
}
body {   browser-shared.css:208
    display: flex;
    flex-direction: column;
    flex: 1;
```

Rules (02:23 - 02:25), on the selected tab-strip slot:
```
element {
}
slot {   arrowscrollbox...
    display: flex;
    flex: 1 0 0;
    flex-direction: inherit;
}
slot {   (user agent) html.c...
    display: contents;   (struck through)
}
Inherited from tabs#tabbrowser-tabs
element {
    --tab-min-width-pref: 76px;
}
#tabbrowser-tabs {
    &[orient="horizontal"] {
        --tab-group-label-highlight-ra...
    }
}
#tabbrowser-tabs {
    &[orient="horizontal"] { ...
```
Element tree near the end: `<splitter id="ve... resizebefore="si...`, `<hbox class="tab...`, `<arrowscrollbox orient="horizont... scrolledtoend sm...`, `#shadow-root`, `<html:link re... toolbarbutton...>`, `<html:link re... arrowscrollbo...>`, `<toolbarbutton keyNav="false" ... backwards ...>`, `<spacer part=...>`, `<scrollbox pa... smoothscroll...>`, `<html:slot>` (selected), `</scrollbox>`, `<spacer part=...>`, `<toolbarbutton ... down" keyNav=... forwards" dis...>`, `<tab class="tab...`.

## Code & shell

Terminal (01:34 - 01:43), shell prompt shows the folder name as `qq0mrgdf.TestProfiles $`:
```
qq0mrgdf.TestProfiles $ mkdir chrome
qq0mrgdf.TestProfiles $ cd chr          <- being typed (completion of "chrome" in progress)
```
Then (01:43) the terminal shows a vim buffer, empty, with tildes `~` on every line and status line:
```
"userChrome.css" [New]                      0,0-1          All
```
The command that produced this (presumably vim userChrome.css) was not visible in any sampled frame. The vim window was replaced by the Firefox window at 01:46 and not shown again in this segment, so no CSS content is visible.

about:config change in this segment:
```
toolkit.legacyUserProfileCustomizations.stylesheets  false -> true
```
DevTools setting toggled: "Enable browser chrome and add-on debugging toolboxes" unchecked -> checked.

## Observations
- The author is setting up Firefox UI customisation via userChrome.css. Step 1: in about:config flip `toolkit.legacyUserProfileCustomizations.stylesheets` to `true` (searches "styles", then "stylesheet", picks the last hit). Step 2: find the active profile folder via about:profiles ("Root Directory" then "Show in Finder"); the profile in use is "TestProfiles", a separate profile from "default-release", so the real browser is untouched. Step 3: from Finder, "New Terminal at Folder", `mkdir chrome`, `cd chrome`, and open a new `userChrome.css` in vim (empty at that moment).
- Second half teaches how to discover the names of the browser's own UI elements to style: open DevTools settings, tick "Enable browser chrome and add-on debugging toolboxes" (tooltip explains it enables the Browser Toolbox), which adds "Browser Toolbox" under More Tools; opening it triggers the "Incoming Connection" remote-debugging prompt, and then he uses the Browser Toolbox element picker on the URL bar and tab strip to find selectors (e.g. `tabs#tabbrowser-tabs`, `arrowscrollbox`, `scrollbox`, `html:slot`, `--tab-min-width-pref: 76px`), i.e. he inspects the browser with the browser's own devtools.
- Style: live mouse-driven demonstration, with terminal and vim for the file work; Finder's context menu instead of typing long paths; a disposable test profile for safety. Philosophy is Unix-flavoured: plain text files (CSS) and a directory convention reshape the tool, no extension needed; minimal, direct steps with no explanatory text beyond what the apps display.
- The typo `about:praferences#home` at 01:12 is left uncorrected on screen; he retypes the URL.
