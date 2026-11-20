# Segment 4 (3:58 - 4:55) - macOS - but better - No Apple

## Timeline
- [03:58-03:59] Desktop: System Settings (left, partly hidden; sidebar: Sign in with your Apple Account, Software Update Available, Wi-Fi, Bluetooth, Network, Battery, General, Accessibility, Appearance, Apple Intelligence, Desktop & Dock, Displays, Menu Bar, Spotlight, Wallpaper), Firefox on objective-see.org/products/lulu.html (cookie banner "This website uses cookies to improve your experience." / "Okay!"), and a Terminal window in front showing the finished `brew install lulu` ("lulu was successfully installed!"). At 3:59 a macOS "Display" brightness HUD appears briefly.
- [04:01] Mouse over Dock; tooltip "Apps". Dock icons: Finder, Apps, (files), Firefox, Terminal, (QuickTime), "exec", (Notes/list), System Settings (badge 1), (folder), Trash.
- [04:02] App launcher opened, search text "Applications"; results: System Settings, System Information.
- [04:03] Search "LuLu" - top result "LuLu - Open"; full app grid below (LuLu, System Settings, VSCodium, Firefox, Terminal, Notes, Stickies, App Store, Books, Calculator, Calendar, Chess, Clock, Contacts, FaceTime, Find My, Firefox, Freeform, Games, Home, iPhone Mirroring, Journal, LuLu, Magnifier, Mail, Maps, Messages, Music). Mouse on LuLu.
- [04:04] Back to terminal and Firefox; mouse near terminal.
- [04:05] Gatekeeper dialog "LuLu is an app downloaded from the Internet..." Cancel / Open (mouse on Open).
- [04:06-04:08] LuLu v4.5.4 welcome window. Notification top right: "Login Item Added" (LuLu will open automatically at login). Window dragged left at 4:08; "Next" visible.
- [04:09] Next page: "First, allow the LuLu Network Extension" with four-step illustration.
- [04:10-04:11] System dialog: "LuLu" would like to use a new network extension (OK / Open System Settings). Mouse moves to OK.
- [04:12] Dialog dismissed (OK); LuLu window shows again.
- [04:13] System Settings comes to front on the "Sign in" pane ("One account for everything Apple."). Nothing typed.
- [04:14] System Settings > General > Login Items & Extensions. "Open at Login": LuLu / Application. "App Background Activity": Podcasts, 1 item (toggle on). Extensions: By App / By Category.
- [04:15-04:16] Scrolls Extensions list By App: Books, Freeform, Journal, LuLu (Network Extension), Messages, News, Notes, Reminders, Shortcuts (all "Sharing" except LuLu). Mouse hovers near LuLu's (i) button.
- [04:17] Clicks LuLu's info: sheet "LuLu Extensions - 1 extension installed", "Network Extension" toggle OFF. Done button.
- [04:18-04:19] Toggle clicked; macOS password prompt "Login Items is trying to modify a System Extension. Enter your password to allow this." Username prefilled (<account>), Password empty/focused (nothing shown).
- [04:21] "LuLu" Would Like to Filter Network Content - Allow / Don't Allow (Don't Allow is the highlighted default; mouse parked below). Toggle behind is now on.
- [04:22] LuLu window "Now, let's configure LuLu:" with three checkboxes, all checked.
- [04:24] LuLu window moved left; "Next" visible.
- [04:28] "Support LuLu?" screen with sponsor logos; "No" button at left.
- [04:29-04:30] Notification top-left: "LuLu up & running! Access it from here anytime." Window moved left; "Yes!" button at right. Mouse hovers Huntress logo.
- [04:31] Mouse on "Yes!".
- [04:32] Firefox opens www.patreon.com/join/objective_see ("Connecting to www.patreon.com..."), blank page.
- [04:33] Firefox back on objective-see.org/products/lulu.html (LuLu product page).
- [04:34] Real LuLu Alert: "Firefox is connecting to www.patreon.com", Block / Allow (Allow cut off by webcam).
- [04:35] Alert dragged; Block / Allow both visible.
- [04:36] Mouse on Allow (Firefox -> patreon.com allowed).
- [04:38] Alert gone; extension sheet with toggle on; mouse on Done.
- [04:39-04:41] Sheet closed; Login Items & Extensions list again; cursor on window title.
- [04:45] Desktop cleared: only an empty Terminal `~ $` and webcam.
- [04:46] Terminal: `curl example.com` run; a LuLu Alert starts appearing behind the terminal; LuLu shield icon at bottom of screen.
- [04:47-04:50] LuLu Alert: "curl is connecting to example.com" (icon "exec"), Block / Allow (Allow default-highlighted); alert dragged left.
- [04:51] Alert gone; curl output printed (see Code & shell).
- [04:52] Menu bar briefly visible: Terminal menus, LuLu shield icon in status bar, recording indicators, clock "Wed Oct 28 3:20 PM".
- [04:54] Same terminal, menu bar hidden.
- [04:55] `ping google.com` typed: "PING google.com (142.250.197.238): 56 data bytes". LuLu Alert: "ping is connecting to 142.250.197.238", Block / Allow (last frame).

## Texts on screen

### Terminal history (3:58 - 4:04)
```
~ $ # Look good now
~ $ # How remove appple , macos without apple
~ $ # Apple track you
~ $ # remove it
~ $ brew install lulu
==> Downloading Homebrew API data
✔ JSON API packages.arm64_tahoe Downloaded   15.9MB/ 15.9MB
==> Fetching downloads for: lulu
✔ Cask lulu (4.5.4)                Downloaded    7.5MB/  7.5MB
==> Installing Cask lulu
==> Moving App 'LuLu.app' to '/Applications/LuLu.app'
🍺  lulu was successfully installed!
~ $
```

### Gatekeeper dialog (4:05)
```
"LuLu" is an app downloaded from the Internet. Are you sure you want to open it?
Homebrew Cask downloaded this file today at 3:19 PM. Apple checked it for malicious software and none was detected.
[Cancel] [Open]
```

### LuLu welcome (4:06-4:08)
```
LuLu v4.5.4
Welcome to LuLu
LuLu is a free open-source firewall for macOS.
It monitors network activity and alerts you when a program first tries to initiate an outgoing connection, allowing you to permit or deny it.
[Next]
```

### Notification (4:06-4:12)
```
Login Item Added
"LuLu" will open automatically when you log in. You can manage extensions in Login Items & Extensions.
```

### LuLu network extension page (4:09-4:12)
```
LuLu v4.5.4
First, allow the LuLu Network Extension:
Note: steps may vary by macOS version

1. In the "System Extension Blocked" alert, click: "Open System Settings"
   (image: System Extension Blocked - The application "LuLu" tried to load new system extension(s). If you want to enable these extensions, open Privacy & Security in System Settings. [OK] [Open System Settings])
2. In System Settings, toggle on th⟨?⟩ (cut off by webcam)
   (image: Network Extensions - Network extensions extend core networking features on your Mac. These extensions run in the background and can monitor the network traffic on your Mac. / LuLu Extension, toggle on)
3. Authenticate
   (image: Privacy & Security - Privacy & Security is trying to modify your system settings. Touch ID or enter your password to allow this. [Use Password...] [Cancel])
4. In the "Filter Network Content" alert, click: "Allow"
   (image: "LuLu" Would Like to F⟨?⟩ Network Content - All network activity on this Mac may be filtered or monitored. [Allow] [Don't Allow])
?? (bottom green bar has a spinner and text, partly visible, starting "Waiting for System Extension Approval" - not fully legible)
```

### macOS dialog (4:10-4:11)
```
"LuLu" would like to use a new network extension
You can enable this extension in Login Items & Extensions. Network extensions run in the background and can monitor network traffic on your Mac.
[OK] [Open System Settings]
```

### System Settings > General > Login Items & Extensions (4:14-4:16, 4:39-4:41)
```
Open at Login
These items will open automatically when you log in.
Item: LuLu    Kind: Application      [+] [-]

App Background Activity
Apps run in the background after you close them to perform tasks such as checking for updates. Personal data and device sensors can be accessed by apps while they run in the background, if you grant permission.
Podcasts - 1 item   [toggle on]

Extensions
Extensions add extra functionality to your Mac and apps, and some may run in the background.
[By App | By Category]
Books - Sharing
Freeform - Sharing
Journal - Sharing
LuLu - Network Extension
Messages - Sharing
News - Sharing
Notes - Sharing
Reminders - Sharing
Shortcuts - Sharing
```
Before -> after: LuLu was added automatically to Open at Login (not removed on screen in this segment). LuLu Network Extension toggle: OFF (4:17) -> ON (4:21 onward). No Apple app background-activity setting or extension was changed in this segment; the author only displays the lists.

### LuLu extension sheet (4:17-4:38)
```
LuLu Extensions
1 extension installed
Network Extension
Provides additional functionality for network applications   [toggle: off at 4:17, on from 4:34]
[Done]
```

### Auth prompt (4:18-4:19)
```
Login Items
Login Items is trying to modify a System Extension.
Enter your password to allow this.
<account>
Password (empty in frame)
[Cancel] [OK]
```

### Filter network content dialog (4:21)
```
"LuLu" Would Like to Filter Network Content
All network activity on this Mac may be filtered or monitored.
[Allow] [Don't Allow]
```

### LuLu configuration page (4:22-4:24)
```
LuLu v4.5.4
Now, let's configure LuLu:
These can be changed later, via LuLu's Settings.
[x] Allow Apple Programs
    Outgoing connections from Apple-signed programs will be allowed.
[x] Allow Already Installed Programs
    Outgoing connections from currently installed 3rd-party programs will be allowed.
[x] Allow DNS Traffic
    All outgoing UDP traffic on port 53 will be allowed.
[Next]
```
All three remain at the default (checked); no change seen.

### Support LuLu page (4:28-4:32)
```
Support LuLu?
It's free, open-source, and written by a single (Mac-loving) coder! Show some love?
Mahalo to the "Friends Objective-See"
iru, paloalto NETWORKS, jamf, Malwarebytes, THREATLOCKER, iVerify., moonlock, NORTH POLE SECURITY, RIPPLING IT, HUNTRESS
[No]  [Yes!]
```

### Notification (4:29-4:30)
```
LuLu up & running!
Access it from here anytime.
```

### Objective-See LuLu web page (4:33)
```
objective-see.org/products/lulu.html
Objective-See - a non-profit 501(c)(3) foundation.   (About / Tools / Blog)
LuLu
Translate
Nearly every application makes network connections. So does malware. LuLu is a free, open-source firewall that blocks unknown outgoing connections, protecting your privacy and your Mac.
[Download]
v4.5.3 · macOS 10.15+ · Changelog · Source · SHA-⟨?⟩
(vendor screenshot below: LuLu Alert - Safari is connecting to configuration.apple.co⟨?⟩)
```

### LuLu Alerts on the author's machine
```
4:34-4:36   Firefox is connecting to www.patreon.com          [Block] [Allow]   -> mouse on Allow
            (icons: certificate, network tree, sigma; "> Details & Options")
4:47-4:50   curl (exec icon) is connecting to example.com     [Block] [Allow]
4:55        ping (exec icon) is connecting to 142.250.197.238 [Block] [Allow]
```
The curl output appeared afterwards (so Allow was evidently chosen; the click is not in a keyframe). The ping alert's outcome is not visible in this segment.

## Code & shell
```
~ $ curl example.com
<!doctype html><html lang=en><head><meta charset=utf-8><link rel=icon href=data:,><meta name="viewport" content="width=device-width,initial-scale=1"><title>Example Domain</title><style>html{color-scheme:light dark;background:light-dark(#eee,#222)}body{font:16px/1.6 system-ui,sans-serif;max-width:26em;margin:auto;padding:25vh 2em 2em;text-align:center}</style></head><body><p>This domain is for use in documentation examples without needing permission. This is not a service; avoid relying on it for testing and monitoring purposes.</p><script src=/s.js></script></body></html>
~ $ ping google.com
PING google.com (142.250.197.238): 56 data bytes
```
Carried over from the previous segment (visible 3:58-4:04): the four `#` comment lines and `brew install lulu` (Homebrew cask 4.5.4, arm64_tahoe), shown above.

## Observations
- The author installs LuLu via Homebrew, opens it from the app grid and walks through its first-run wizard: approve the network extension (System Settings > General > Login Items & Extensions > LuLu > Network Extension toggle, with password), allow "Filter Network Content", keep the three default options (Allow Apple Programs, Allow Already Installed Programs, Allow DNS Traffic) checked, and click Yes! on the support prompt (which opens the Patreon page in Firefox).
- He then demonstrates the firewall working: Firefox -> patreon.com triggers a LuLu alert and he allows it; then from a clean terminal he runs `curl example.com` and `ping google.com`, and each new outgoing program triggers a Block/Allow alert.
- His own terminal comments frame the purpose: "How remove appple , macos without apple", "Apple track you", "remove it". No on-screen text in this segment states a reason beyond that; the LuLu web page says it "blocks unknown outgoing connections, protecting your privacy and your Mac."
- The vendor screenshot on the LuLu page shows "Safari is connecting to configuration.apple.co..." which echoes the theme (watching Apple service traffic), but it is the vendor's sample image.
- The Login Items & Extensions lists show Apple sharing extensions (Books, Freeform, Journal, Messages, News, Notes, Reminders, Shortcuts) and Podcasts background activity; he scrolls through them but toggles none in this segment. He remains signed out of an Apple Account.
- No Block rules are created in this segment. Style: direct screen demonstration, almost no typing, short terminal comments used as notes. The webcam overlay only shows him speaking and carries no text.
