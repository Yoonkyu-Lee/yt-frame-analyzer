# Segment 3 (02:59 - 06:25) - "C is Flat" (wj9os_e8Hs4)

## Timeline
Screen layout throughout: Firefox-like window with Google Sheets "Sheet4" (docs.google.com/spreadsheets/d/1Eb5CWWEF...), a Terminal window ("lab $") behind it at left, two sticky notes at top right (cyan quote note, green claims note), a yellow sticky note that is edited live, and the "Human" webcam overlay at bottom right (only the man's posture changes; ignored).

- [02:59] Sheet: A2 "Address" header, merged "Memory" header over B-E (borders drawn), A3 holds 0, column D has 1 in rows 3-10. Yellow sticky is empty. Cyan note: "If everything is flat, you can't go wrong. - Professor Gilbert Strang" (name highlighted grey). Green note partly hidden behind the yellow sticky ("Memory hardware is flat / ... hardware / ...t / ... nvention").
- [03:04] Address column filled by drag: 0..7 in A3:A10 (selection shows Sum: 28).
- [03:08] Borders menu opened (toolbar), cursor on the "all borders" icon.
- [03:10] Address column now bordered: 0..7 in rows 3-10; D3:D10 all 1.
- [03:12] Whole column A selected (clicked column header).
- [03:13] Right-click context menu on column header: Cut, Copy, Paste, Paste special, "Insert 1 column left" (hovered), Insert 1 column right, Delete column, Clear column, Hide column, Resize column, Create a filter, Sort sheet A to Z, Sort sheet Z to A, Conditional formatting.
- [03:14] A new blank column A inserted; table shifted right: Address in B, Memory header over C-F, the 1s are now in column E.
- [03:15] Cell B10 selected; B column still wide.
- [03:17] Dragging the B/C column border to narrow the Address column.
- [03:18] Address column narrower still; table now B..F.
- [03:22] Clicked into the yellow sticky (empty, cursor blinking); sheet cell G4 selected. "Address" and "Memory" headers now bold.
- [03:24] Yellow sticky dragged left (x~850-1150), revealing full green note: "Memory hardware is flat / C near hardware / C is flat / ---- / just convention".
- [03:28] Yellow sticky now reads: "this is computer memory".
- [03:46] Memory cells filled with 0s in C3:F10 (paste popup shown, F3:F10 selected, Sum: 0). Addresses still 0..7.
- [03:48] Address column narrowed further (column A narrow); cursor resizing B. All memory cells 0.
- [04:08] Addresses now 1..8 (rows 3-10); F3 = 1 (bits 0 0 0 1), others 0. Text in yellow sticky "this is computer memory" selected (grey highlight).
- [04:18] Yellow sticky replaced with "what have in address 1 -> 1".
- [04:21] E4 set to 1 (row for address 2: 0 0 1 0). Sticky unchanged.
- [04:34] Second line added: "what have in address 2 -> 1 0".
- [04:35] Typing cursor at end of second line.
- [04:41] Third line added: "or we can call it 2".
- [04:54] Column G being formatted (borders menu open, G3:G10 selected, Sum: 3); G3 = 1, G4 = 2 shown.
- [04:55] G2 header "Decimal" added; G3=1, G4=2.
- [05:03] Row for address 3 is now 0 0 1 1 and G5 = 3. Yellow sticky text fully selected (grey).
- [05:04] Yellow sticky cleared (empty). Green note being edited (cursor after "C is flat ").
- [05:18] Yellow sticky now: "just convention" / (blank) / "we set the rule". Green note's last line "just convention" highlighted grey.
- [05:20] Same sticky text selected (grey) in edit mode.
- [05:28] Terminal brought to front: shows "lab $ ls" then new prompt "lab $ ". Yellow sticky: "simulation this in C".
- [05:32] Terminal now in vim with new file "main.c" [New], position 2,0-1; contents: #include <stdio.h> / blank / int main(void) { / blank / return 0; / }.
- [05:44] Spreadsheet in front again (cell F3 selected); terminal behind shows vim buffer with 8 lines ("3,19 All").
- [05:46] Terminal front: vim buffer now has global `int bytes[4] = {0};` on line 3. Sheet range C3:F3 selected (Sum: 1) - the 4 cells of a row are mapped to the 4-element array.
- [05:49] Vim in INSERT mode, blank line 3 inserted above `int bytes[4] = {0};` (now line 4); file is 9 lines (3,1).
- [06:07] Comment typed on line 3: `// RULE: each cell can storage 0 or 1`. Yellow sticky being typed: "for spe" (incomplete).
- [06:20] Yellow sticky cleared (empty). Vim in VISUAL LINE mode, all 9 lines selected (9,1).
- [06:21] Normal mode: "9 lines yanked" (cursor 1,1). Sticky empty.
- [06:22] Spreadsheet in front; terminal behind still shows "9 lines yanked".
- [06:23] Yellow sticky dragged to lower right (x~1075-1440, y~265-465), partially covering the green note; cursor in it, empty.
- [06:24] Yellow sticky moved slightly (x~1090-1460), empty.
- [06:25] Browser address bar opened: typed "ge" autocompletes to "gemini.google.com/"; dropdown shows Google Gemini and Firefox Suggest entries (see Texts). Page behind is blank white.

## Texts on screen

### Cyan sticky note (whole segment, 02:59-06:25)
```
If everything is flat, you
can't go wrong.

- Professor Gilbert Strang
```
(the "- Professor Gilbert Strang" line has grey highlight behind it)

### Green sticky note (whole segment; fully readable from 03:24; partly covered 02:59-03:22 and 06:23-06:25)
```
Memory hardware is flat
C near hardware
C is flat
----
just convention
```
(from 05:18 "just convention" has a grey selection highlight)

### Yellow sticky note - successive versions
03:28 - 04:08:
```
this is computer memory
```
04:18 - 04:21:
```
what have in address 1 -> 1
```
04:34 - 05:03 (final of this version):
```
what have in address 1 -> 1
what have in address 2 -> 1 0
or we can call it 2
```
05:04: empty

05:18 - 05:20:
```
just convention

we set the rule
```
05:28 - 05:49:
```
simulation this in C
```
06:07 (typing in progress):
```
for spe⟨typing, cut off⟩
```
06:20 onward: empty.

### Spreadsheet "Sheet4" (final state of segment, 06:25)
Column A is an empty narrow spacer; table occupies B-G. Addresses were first 0..7 and were later changed to 1..8.

| row | B | C | D | E | F | G |
|---|---|---|---|---|---|---|
| 2 | **Address** | **Memory** (merged C:F) | | | | **Decimal** |
| 3 | 1 | 0 | 0 | 0 | 1 | 1 |
| 4 | 2 | 0 | 0 | 1 | 0 | 2 |
| 5 | 3 | 0 | 0 | 1 | 1 | 3 |
| 6 | 4 | 0 | 0 | 0 | 0 | |
| 7 | 5 | 0 | 0 | 0 | 0 | |
| 8 | 6 | 0 | 0 | 0 | 0 | |
| 9 | 7 | 0 | 0 | 0 | 0 | |
| 10 | 8 | 0 | 0 | 0 | 0 | |

Earlier states: 02:59 - Address A3 = 0, Memory block with 1s in column D rows 3-10; 03:04 - addresses 0..7; 03:46 - Memory C-F all 0, addresses 0..7; 04:08 - addresses 1..8, F3 = 1.

### Browser address bar dropdown (06:25)
```
gemini.google.com/   (typed "ge", autocompleted)
Google Gemini - gemini.google.com
Firefox Suggest:
Community - YouTube Studio - studio.youtube.com/channel/UC0Cww7KegobDLLLaK...
Google Gemini - gemini.google.com/app
Simple Ed Text Editor in C - Google Gemini - gemini.google.com/app/a5d6b76e2faf5684
(1) Inbox | <mail address> | Proton M... - mail.proton.me/u/1/inbox/...
Giải mã ý nghĩa comment YouTube - Google G... - gemini.google.com/app/bba4d849912556cd
Huyền thoại trình soạn thảo ed - Google Gemi... - gemini.google.com/app/357d9a6780d2da9e
Video comments - YouTube Studio - studio.youtube.com/video/gQRmSlcTq9c/comments/i...
Cử chỉ phóng to trên Mac - Google Gemini - gemini.google.com/app/456ff50029ce38e6
Untitled design - YouTube Thumbnail - canva.com/design/DAHZAe8AzfQ/Z8GeuBKbqaWxsJ...
```
(translation) Vietnamese Gemini chat titles: "Decoding the meaning of a YouTube comment"; "The legend of the ed text editor"; "Zoom gesture on Mac".

## Code & shell

Terminal (prompt `lab $`), 05:28:
```
lab $ ls
lab $
```
(ls printed nothing - empty directory)

vim, file "main.c" [New], first state (05:32):
```
1 #include <stdio.h>
2
3 int main(void) {
4
5   return 0;
6 }
```

05:46 (after adding global array):
```
1 #include <stdio.h>
2
3 int bytes[4] = {0};
4
5 int main(void) {
6
7   return 0;
8 }
```

Final state seen (06:07-06:25):
```
1 #include <stdio.h>
2
3 // RULE: each cell can storage 0 or 1
4 int bytes[4] = {0};
5
6 int main(void) {
7
8   return 0;
9 }
```
Status line sequence: "main.c" [New] 2,0-1 -> 3,19 All -> -- INSERT -- 3,1 -> 3,37 -> -- VISUAL LINE -- 9,1 -> "9 lines yanked" 1,1 All. (The 9 lines were yanked with V, presumably to paste elsewhere.)

## Observations
- He builds a memory model by hand in a spreadsheet: a column of Addresses, a "Memory" block of cells that each hold one bit (0 or 1), and a "Decimal" column interpreting the bit pattern in a row as a number (0001 -> 1, 0010 -> 2, 0011 -> 3). Addresses were first 0..7, then renumbered 1..8.
- The yellow sticky works as his live spoken-script / caption: "this is computer memory"; "what have in address 1 -> 1 / what have in address 2 -> 1 0 / or we can call it 2" (the same bits can be read as binary or called a decimal number); then "just convention / we set the rule" (the meaning of bit patterns is a human convention); then "simulation this in C".
- The cyan and green notes are the pre-written thesis: "If everything is flat, you can't go wrong" (Strang); memory hardware is flat, C is near hardware, so C is flat; the "----" separator followed by "just convention" marks that interpretation of bits is convention, not hardware.
- He then moves from the spreadsheet to C: opens main.c in vim and declares `int bytes[4] = {0};`, mapping the 4 memory cells of a row to an array, with a comment stating the rule "each cell can storage 0 or 1" (his spelling). A new sticky "for spe..." is started and discarded.
- At the end he yanks the whole file (9 lines) and opens Gemini in the address bar, apparently to paste the code into an LLM; URL-bar history shows chats like "Simple Ed Text Editor in C" and "Huyền thoại trình soạn thảo ed", i.e. he uses AI as a helper alongside hand-writing in vim.
- Style: minimal tools (sticky notes, spreadsheet, terminal/vim), first-principles teaching (bits -> numbers -> array in C), with a "convention vs hardware" philosophy.
