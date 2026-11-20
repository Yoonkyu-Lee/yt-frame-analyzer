# Segment 5 (4:55 - 6:05) - macOS - but better - No Apple (final segment)

## Timeline
- [04:55] Terminal window (history: `curl example.com` printed HTML; `~ $ ping google.com` running, "Request timeout for icmp_seq 0 / 1"). A **LuLu Alert** pops up over it: icon "exec", **ping** "is connecting to 142.250.197.238", buttons Block / Allow, "Details & Options". The alert is shown moving/animating (04:55-04:56) with Allow highlighted as default.
- [04:58-04:59] Alert gone. Terminal now shows `ping google.com` ended with 3 timeouts (icmp_seq 0, 1, 2) and a fresh prompt `~ $`. (The ping was blocked/timed out; the alert was dismissed - the click itself is not visible.)
- [05:00] Terminal moved up/left; a new LuLu Alert: shield icon **LuLu** "is connecting to objective-see.org" (LuLu's own update check), Block / Allow greyed (just appearing).
- [05:01-05:02] Same alert, now active; cursor goes to **Block** (05:02). He blocks LuLu's own connection to objective-see.org.
- [05:03-05:05] Alert gone; Terminal shows the ping output; he selects the ping output text with the mouse (selection of the "PING google.com ... Request timeout" lines, then also the `~ $ ping google.com` line).
- [05:06] Menu bar visible (Terminal app, Wed Oct 28 3:21 PM). Cursor goes to the LuLu shield icon in the menu bar.
- [05:07] LuLu menu open: "LuLu: enabled", "Profile: Default", Disable, Rules >, Profiles >, Settings... (cmd+,), Network Monitor..., Quit LuLu, Uninstall LuLu, Support LuLu (heart).
- [05:08] Rules submenu: Show..., Add..., Export..., Import..., Cleanup. Cursor on "Add..."
- [05:09] "LuLu Rules" window (Current Profile: Default, "All Rules") with "(re)loading rules..." spinner.
- [05:10-05:12] Rules list loaded (sorted by program): apsd, assistantd, automount, curl, Firefox, helpd, LuLu ... each with "any address:any port" and a green checkmark "Allow". Window shifted slightly left at 05:12.
- [05:14] "Add Rule (for outgoing connections)" dialog opens: Program path (empty, focused), Browse, Remote address/domain (regex? checkbox unticked) "*", Remote port "*", radio Block (selected) / Allow, Cancel / Add.
- [05:16] Program path now `*`; focus moves to Remote address/domain field.
- [05:21] After adding: new top group **Any program** with rule `*.apple.com:any port` -> red **Block**. (So he typed `*.apple.com` as remote address, program `*`, Block, then Add.)
- [05:22] Add Rule dialog opened again (Program path being typed).
- [05:24] Program path `*`, focus in Remote address/domain (value being typed, not yet readable).
- [05:29] "(re)loading rules..." spinner after pressing Add.
- [05:30] Rules list now has under Any program two block rules: `*.apple.com:any port` Block and `*.icloud.com:any port` Block.
- [05:31] Back to a clean desktop with Terminal showing the earlier ping output (cursor in terminal).
- [05:32] Empty desktop, Dock appearing at bottom, tooltip "Stickies". Dock icons: Finder, Launchpad-like grid, Stickies, Firefox, Terminal, QuickTime, exec, Notes-like icon, a folder/stack, Trash.
- [05:33] Launchpad-style "Applications" app grid (Apps view): categories Productivity & Finance, Utilities, Information & Reading, Creativity, Entertainment, Social, Other. First row: LuLu, System Settings, VSCodium, Firefox, Terminal, Notes, Stickies. Then App Store, Books, Calculator, Calendar (Wed 28), Chess, Clock, Contacts, FaceTime, Find My, Firefox, Freeform, Games, Home, iPhone Mirroring, Journal, LuLu, Magnifier, Mail, Maps, Messages, Music, ... (more below).
- [05:34] Search typed "tips" -> results: Tips, Time Machine, FaceTime, QuickTime Player. Tips is highlighted ("Open").
- [05:35] Tips window opens, loading spinner.
- [05:36-05:37] Tips shows an error: "Content is Unavailable - Try opening the Tips app later." (Apple server blocked by the LuLu rules.) Menu bar shows app name Tips.
- [05:38] Spotlight Search bar appears over it (empty).
- [05:40] Apple Maps opens: sidebar with search "Apple Maps", map area blank (cream colored), controls.
- [05:41] Maps: sidebar Places (Pinned, Saved Places, Guides, Routes), Recents (two place entries, "Clear Recents"), "Terms & Conditions", map canvas blank dark grid (no map tiles), temperature badge "83°".
- [05:43] LuLu Alert: **com.apple.geod** (generic block icon) "is connecting to cdn.apple-mapkit.com"; cursor on Block.
- [05:46] Alert gone; Maps still shows only an empty dark grid with a scale bar "0 50 100 150 ft". (Map tiles never load.)
- [05:48] Dock reappears with tooltip "Stickies"; Dock now includes Tips (lightbulb) and Maps icons.
- [05:49-05:50] A new **yellow Stickies note** opens at top-left, empty, cursor blinking; Dock tooltip "Firefox".
- [05:51] Sticky moved to center of the screen, still empty.
- [05:52] Sticky text begins: "apple".
- [05:57] Sticky text: "apple can't track you anymore". Maps window closed/ gone; plain desktop.
- [05:58] Same note, moved slightly left.
- [05:59] Cursor near the note; same text.
- [06:03] Note text changed to "this is clean mac" (Dock visible, tooltip "Stickies").
- [06:05] Menu bar visible with Stickies menus (File, Edit, Font, Color, Window, Help); clock Wed Oct 28 3:22 PM. Final frame: sticky "this is clean mac" on empty grey desktop, webcam overlay.

## Texts on screen

### Sticky note (05:57 - 06:00), after retyping from "apple"
```
apple can't track you anymore
```

### Sticky note (06:03 - 06:05), final text
```
this is clean mac
```

### LuLu Rules, final state seen (05:30)
| Program | Remote address:port | Action |
|---|---|---|
| Any program | `*.apple.com:any port` | Block |
| Any program | `*.icloud.com:any port` | Block |
| apsd (com.apple.apsd, signer: Apple Proper) | any address:any port | Allow |
| assistantd (com.apple.assistantd, signer: Apple Proper) | any address:any port | Allow |
| automount (com.apple.automount, signer: Apple Proper) | any address:any port | Allow |
| curl (com.apple.curl, signer: Apple Proper) | any address:any port | Allow |
| Firefox (org.mozilla.firefox, signer: Developer ID Application: Mozilla Corporation (43AQ936H96)) | any address:any port | Allow |
| helpd (com.apple.helpd) | any address:any port | Allow (seen at 05:10) |
| LuLu | ?? (below fold) | ?? |

### LuLu alerts seen
- `ping` is connecting to 142.250.197.238 (Block / Allow) - 04:55-04:56
- `LuLu` is connecting to objective-see.org (Block / Allow) - 05:00-05:02 (he clicks Block)
- `com.apple.geod` is connecting to cdn.apple-mapkit.com (Block / Allow) - 05:43 (cursor on Block)

### LuLu Add Rule dialog fields (05:14)
```
Add Rule (for outgoing connections)
Program path: *
Remote address/domain  [ ] regex?   *   (typed: *.apple.com, then *.icloud.com)
Remote port: *
(o) Block   ( ) Allow
[Cancel] [Add]
```

### Tips app error (05:36 - 05:40)
```
Content is Unavailable
Try opening the Tips app later.
```

## Code & shell
Terminal session (04:55 onward, all prior output retained):
```
~ $ curl example.com
<!doctype html><html lang=en><head><meta charset=utf-8><link rel=icon href=data:,><meta name=viewport content="width=device-width,initial-scale=1"><title>Example Domain</title><style>html{color-scheme:light dark;background:light-dark(#eee,#222)}body{font:16px/1.6 system-ui,sans-serif;max-width:26em;margin:auto;padding:25vh 2em 2em;text-align:center}</style></head><body><p>This domain is for use in documentation examples without needing permission. This is not a service; avoid relying on it for testing and monitoring purposes.</p><script src=/s.js></script></body></html>
~ $ ping google.com
PING google.com (142.250.197.238): 56 data bytes
Request timeout for icmp_seq 0
Request timeout for icmp_seq 1
Request timeout for icmp_seq 2
~ $
```
(curl example.com worked; ping to google.com timed out, i.e. blocked at the LuLu alert.)

## Observations
- This is the demonstration/conclusion segment: he proves the setup works. First, LuLu in action: a ping to google.com triggers an alert and is blocked; curl to example.com already succeeded (curl has an Allow rule "any address"). LuLu even asks about its own connection to objective-see.org (its update check), and he blocks that too.
- He then opens the LuLu rules and adds two wildcard rules for "Any program": Block `*.apple.com` and Block `*.icloud.com` (any port). That is the core move: block all of Apple's domains at the firewall, while leaving selected system daemons (apsd, assistantd, automount, curl, helpd) and Firefox with allow rules.
- Proof by use: Tips shows "Content is Unavailable" (needs Apple servers); Apple Maps loads with a blank grid because com.apple.geod to cdn.apple-mapkit.com is blocked. The Apple apps become useless, the system itself keeps running.
- Closing statement is typed live on a Stickies note: "apple can't track you anymore", then replaced by "this is clean mac". He uses the sticky note as the on-screen punchline, no slides, simple hands-on demo.
- No new System Settings changes were shown in this segment; all content is LuLu rules and a demo of blocked Apple apps. Philosophy: control what leaves your machine with a free open-source tool rather than trusting vendor settings; accept loss of Apple cloud features for privacy.
- Webcam overlay "Human" is present throughout; the author looks at the screen, no relevant on-screen change from it.
