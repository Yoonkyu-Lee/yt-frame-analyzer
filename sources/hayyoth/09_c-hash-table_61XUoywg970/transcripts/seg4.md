# Segment 4 (06:11 - 09:30) transcript

NOTE: In this whole segment there is NO right-hand vim window with a hash-table file. The right side shows (a) a browser with Wikipedia "Hash table" until ~06:45, (b) then only the wallpaper (Tower of Babel drawing with green phone-number-like boxes "21-8976", "21-1234", "21-9655", "18-4165", "21-50?"), (c) a yellow Stickies note, (d) at the very end a second blank Terminal window. All 40 frames viewed.

## Timeline

[06:11] LEFT Terminal, tab `~/lab — vim test.c` (active) / `~/lab — -zsh`. vim NORMAL mode, cursor 3,0-1 "All". Buffer is 18 lines (see State A). Cursor on blank line 3. Right: Chrome-like browser, tab "Wikipedia", URL `en.wikipedia.org/wiki/Hash_table`, text about "independence results ... collision resolution schemes such as linear probing", "K-independence can prove a hash function works ...", "hash function which", "search keys hash to the same", "unseen given data", "for collision resolution are", "n addressing.[7]:458", "-value pairs for", "the collided items", "gh a single linked", "during lookup of a", "resolution through", "a common method". Wikipedia diagram (keys: John Smith, Lisa Smith, Sam Doe) lower right. An EMPTY yellow sticky note (no text) sits over the browser at about x 862-1295, y 172-460. Mouse arrow near top of editor.
[06:13] vim INSERT mode, 3,1; a new blank line was opened (now 19 lines; line 3 blank, line 4 blank, `int main` on line 5). Sticky still empty.
[06:25] INSERT, cursor 3,63 (line wraps). Line 3 typed: `// in each cell (each index) -> we storage linked list to have` (wraps at "t|o have"). Total 19 lines.
[06:31] NORMAL, 2,1. Comment now sits on line 2 and reads `// in each cell (each index) -> we storage linked list to have multiple value` (wraps "t|o have multiple value"); line 3 blank; `int main(void) {` on line 4. 18 lines. (Cursor on the first `/` of line 2.) The comment appears to have been moved up/edited ("multiple value" added).
[06:32] NORMAL, 2,0-1: comment line DELETED. Buffer back to 17 lines; line 2 blank (cursor block), `int main(void) {` on line 3. (State B.)
[06:33] INSERT, 2,1; new blank line opened (18 lines; lines 2 and 3 blank, main on line 4).
[06:34] INSERT, 3,3: typed `ty` on line 3 (19 lines; blank line 2, ty on 3, blank 4, main on 5).
[06:38] INSERT, 4,3: line 3 `typedef struct Node {`, line 4 indented (2 spaces) empty. 20 lines. Webcam: speaking.
[06:45] Browser has been hidden/closed; wallpaper revealed (Tower of Babel with green boxes). Empty yellow sticky still at same position (no text). vim INSERT, 4,12: line 4 `  int value` (cursor box after `value`, no semicolon yet). Terminal window title bar greyed (unfocused, mouse arrow over wallpaper at ~(1035,478)).
[06:47] Sticky note collapsed ("rolled up") to a thin yellow bar at y~180 (x 862-1295). vim INSERT 4,12 `  int value` (no semicolon). Mouse I-beam near (597,183).
[06:49] INSERT 5,3: line 4 `  int value;`, line 5 blank indented. 21 lines.
[06:53] INSERT 6,3: line 4 `  int value;`, line 5 `  int key;`. 22 lines.
[07:08] INSERT 7,2: lines 4-7 are `  int value;` / `  int key;` / `  struct Node *next;` / `}` (the `}` is highlighted in cyan together with the `{` on line 3 = vim matching-bracket highlight; the `}` has no `Node;` yet). 23 lines.
[07:20] NORMAL 13,3: struct now closed `} Node;` (line 7). main on line 9: `  Node arr[5] = {0};` (line 10; NOTE `Node` appears at column 1 of the indent, i.e. one space less than the `int arr` previously; indent looks like 1 space; ⟨?⟩). `int arr[5] = {0};` was changed to `Node arr[5] = {0};`. Lines `arr[1] = 11;` deleted; remaining `arr[2] = 22;` (line 12) and `arr[3] = 33;` (line 13, cursor on `a`). 22 lines total.
[07:21] NORMAL 12,1: `arr[3] = 33;` deleted too. Now line 12 `arr[2] = 22;`, line 13 blank, line 14 `// can i do this`, 15 `arr[7 % 5] = 77;`. 21 lines.
[07:22] INSERT 13,3: opened new indented line 13 under `arr[2] = 22;` (22 lines).
[07:29] INSERT 13,3: line 12 commented: `  //arr[2] = 22;` ; line 13 blank indented; lines 14,15 blank; line 16 `// can i do this`. 23 lines.
[07:41] INSERT 14,5: line 13 `  arr[2] = (Node) {`, line 14 empty (4-space indent). 24 lines.
[07:50] INSERT 15,5: line 14 `    .value = 22,`; line 15 empty. 25 lines.
[07:56] INSERT 16,6: line 14 `    .value = 22,` line 15 `    .key = 2,` line 16 `    .` (typing next). 26 lines.
[08:01] INSERT 17,4: compound literal complete: 
  `  arr[2] = (Node) {` / `    .value = 22,` / `    .key = 2,` / `    .next = NULL` / `  }` (the `}` just typed; `{` and `}` highlighted cyan by bracket match). No trailing `;` yet. 27 lines.
[08:05] NORMAL 19,0-1: `  };` now has semicolon (line 17). Line 18, 19 blank; line 20 `  arr[7 % 5] = 77;`. 26 lines.
[08:06] NORMAL 20,0-1: line 19 `  arr[7 % 5] = 77;`, line 20 blank (cursor), printf lines on 21,22. 25 lines (one blank line removed).
[08:07] VISUAL LINE (selection "3" lines): lines 20-22 selected (blank, `printf("%d\n", arr[2]);`, `printf("%d\n", arr[7 % 5]);`). Cursor 22,3.
[08:08] INSERT 20,3: the two printf lines deleted (selection deleted). Buffer: line 19 `  arr[7 % 5] = 77;`, 20 empty indented (cursor), 21 blank, 22 `  return 0;`, 23 `}`. 23 lines.
[08:37] INSERT 23,15. Typing the second node. Buffer lines 19-23:
  `  //arr[7 % 5] = 77;`
  `  arr[2].next = &(Node) {`
  `    .value = 77,`
  `    .key = 7,`
  `    .next = NU|` (mid-typing "NULL")
  line 24 blank, 25 `  return 0;`, 26 `}`. 26 lines.
[08:41] Switched to the `~/lab — -zsh` tab (tab bar shows `~/lab — vim test.c` and active `~/lab — -zsh`). Shell output (see Shell section): two `cc main.c && ./a.out` runs; first printed `11`, second printed `77` and `77` (the `77` first line highlighted/selected grey). Prompt `lab $ `. (Run of `main.c` -- file is main.c, not test.c; tab title says test.c stale.) Sticky still collapsed bar.
[08:44] Back to vim tab. NORMAL 24,4; status line shows `"main.c" 27L, 342B written`. Buffer complete (State C, final). Cursor on `;` of line 24 `  };`. Mouse I-beam at ~(418,462). Sticky still collapsed thin bar.
[08:46] Mouse arrow clicked sticky: it expands to full yellow sticky at (862-1295, 172-432) with a text cursor, empty, title bar showing small control squares. Cursor in note at start. (Note is empty at this moment.) Editor unchanged.
[08:47] Sticky being dragged leftwards/down: now at x 706-1139, y 222-510. Empty.
[08:48] Sticky moved to x 652-1085, y 224-512. Text typed: `this` (cursor after).
[09:03] STICKY NOTE text: 
```
this is very minimal hashtable
with key is int and |
```
(typing continues; wraps at "hashtable / with key is int and ").
[09:08] Sticky now empty (text cleared) with cursor at top-left. (Previous note text deleted.)
[09:20] STICKY NOTE text (typing in progress):
```
we can replace key int to key
string

look at chatgp|
```
(visible text truncated at "chatgp"; presumably "chatgpt" being typed -- final letters ⟨t?⟩ not seen.)
[09:22] Sticky empty again (text cleared), cursor at top-left.
[09:25] Sticky collapsed to thin yellow bar (x 652-1085, y 232) with cursor/mouse on it.
[09:27] Sticky bar moved to the far right (x 1128-1562, y 228). Editor unchanged. Mouse I-beam at (753,487). Terminal title bar: active.
[09:28] A NEW second Terminal window (title "Terminal", no tabs) appears, overlapping at x 270-957, y 17-560, with a prompt `lab $ ` and a block cursor. It is being dragged/placed (animation in progress, window sliding). Sticky bar still at right.
[09:29] The second Terminal window moves to the right: x 872-1557, y 45-588, `lab $ `, empty. Mouse arrow on its title bar. Left window unchanged.
[09:30] Second Terminal at x 823-1508, y 38-581 (still blank `lab $ `). Mouse arrow at (700,66) on left window title bar. Sticky collapsed bar at (1128-1562,228).

## File states

### Left file (test.c per tab title; vim status line says `main.c`) -- END of segment (09:30)
Indent: 2 spaces. Line numbers from vim (frames 08:44-09:30). `⟨?⟩` marks uncertain chars.
```
 1| #include <stdio.h>
 2|
 3| typedef struct Node {
 4|   int value;
 5|   int key;
 6|   struct Node *next;
 7| } Node;
 8|
 9| int main(void) {
10|   Node arr[5] = {0};
11|
12|   //arr[2] = 22;
13|   arr[2] = (Node) {
14|     .value = 22,
15|     .key = 2,
16|     .next = NULL
17|   };
18|
19|   //arr[7 % 5] = 77;
20|   arr[2].next = &(Node) {
21|     .value = 77,
22|     .key = 7,
23|     .next = NULL
24|   };
25|
26|   return 0;
27| }
```
Status line: `"main.c" 27L, 342B written`   (cursor 24,4, All). Tab title still `~/lab — vim test.c`.
Note: the `printf` lines were deleted (08:07-08:08) so the program prints nothing now; the `// can i do this` comment and old arr[1..3] lines are gone. Line 10 `Node arr[5]` appeared at ~1-space indent at 07:20 but at 08:01 onwards it renders with the same 2-space indent as other lines in main (text starts at x=210) -- treat as 2 spaces.

### Intermediate versions
A) 06:11 (18 lines): 
```
 1| #include <stdio.h>
 2|
 3|
 4| int main(void) {
 5|   int arr[5] = {0};
 6|
 7|   arr[1] = 11;
 8|   arr[2] = 22;
 9|   arr[3] = 33;
10|
11|   // can i do this
12|   arr[7 % 5] = 77;
13|
14|   printf("%d\n", arr[2]);
15|   printf("%d\n", arr[7 % 5]);
16|
17|   return 0;
18| }
```
(line 3 is blank with cursor; there is one blank line extra vs. 17-line file at 06:32.)

B) 06:25 comment typed (then removed at 06:32):
`// in each cell (each index) -> we storage linked list to have multiple value`

C) 07:08 struct before typedef name: lines 3-7 `typedef struct Node {` / `  int value;` / `  int key;` / `  struct Node *next;` / `}` ; main on 9.

D) 07:20 main body after conversion to Node: 
```
 9| int main(void) {
10|   Node arr[5] = {0};
11|
12|   arr[2] = 22;       (arr[1] and arr[3] deleted one after another)
13|   ...
14|   // can i do this
15|   arr[7 % 5] = 77;
16|   printf("%d\n", arr[2]);
17|   printf("%d\n", arr[7 % 5]);
```

### Right-hand file
Not visible in this segment (no vim window with the right-hand hash-table code). Only a Wikipedia page (06:11-06:38) and wallpaper.

## Shell & outputs

[08:41] tab `~/lab — -zsh`, prompt `lab $ `:
```
lab $ cc main.c && ./a.out
11
lab $ cc main.c && ./a.out
77
77
lab $ 
```
(These are earlier runs: first run printed `11` -- from an earlier version with `arr[1] = 11` and printf of arr[1]; second printed `77` `77`, from printf("%d\n", arr[2]) and arr[7 % 5] of the array version where arr[2]=77 aliasing 7%5=2.) Note: the `]` right-edge marks next to the prompt lines are zsh/Terminal prompt marks. No compiler errors/warnings shown. This history pre-dates the typedef Node changes (printf lines already removed in the current file) -- he was just showing the earlier result: arr[7 % 5] and arr[2] hit the same cell (collision).

[09:28-09:30] A second Terminal window shows only `lab $ ` (no commands run).

Vim messages: `"main.c" 27L, 342B written` (visible 08:44 through 09:30).

## Sticky notes (verbatim)
1. Empty sticky (06:11-06:45): no text.
2. 08:48: `this` -> 09:03: `this is very minimal hashtable` / `with key is int and ` (continuing, text cut off by frame).
3. 09:08: empty (text erased/replaced).
4. 09:20: `we can replace key int to key` / `string` / (blank) / `look at chatgp` (cursor after "chatgp"; likely "chatgpt").
5. 09:22: emptied again. 09:25 collapsed to bar.
Final collapsed bar at far right at 09:27-09:30 (contents hidden).

## Observations

- Learning flow: starts from a plain int array `arr[5]` demonstrating index = key % size (`arr[7 % 5] = 77` aliasing `arr[2]`; comment `// can i do this`). Then, on the collision (7 % 5 == 2 collides with index 2), he writes a comment: `// in each cell (each index) -> we storage linked list to have multiple value` -- the chaining idea -- then deletes it and converts to code: `typedef struct Node { int value; int key; struct Node *next; } Node;`, changes the array to `Node arr[5] = {0};`, removes arr[1], arr[3] and the printf lines, comments out `arr[2] = 22;` and replaces with compound literal `arr[2] = (Node){ .value = 22, .key = 2, .next = NULL };`, then comments out `arr[7 % 5] = 77;` and writes `arr[2].next = &(Node){ .value = 77, .key = 7, .next = NULL };` -- manually chaining a second node into the same bucket. This is his hand-built mental model of separate chaining before using any reference code.
- The Wikipedia "Hash_table" page was on screen at the start of the segment (06:11-06:38) as a reference (a collision-resolution/separate chaining section is visible in the page: "...through a single linked...", "collision resolution through", "a common method"). Browser then disappears; the wallpaper shows.
- Origin evidence for right-hand code: NONE in this segment (no right-hand file). Only a hint: sticky note says `we can replace key int to key string` / `look at chatgp[t]` -- i.e. he plans to "look at ChatGPT" for the next step (generalizing key from int to string), strongly suggesting the right-hand hash-table code (with `struct Entry`, `hash()`, `put()`, `get()`, `destroy()`) in later segments originates from ChatGPT. Note that he erased the sticky text after typing it (09:22), so it was a working note to himself. Another note says: `this is very minimal hashtable with key is int and ...` (his description of his own manual structure).
- File naming quirk: the tab title says `vim test.c` but the write message says `"main.c" 27L, 342B written` and shell runs `cc main.c`, so the file is actually main.c (tab title stale, or the file was `:w main.c` / saved-as).
- Philosophy signals: builds up from the simplest thing (array) and manually writes out the chained structure with compound literals so he can see how nodes link, typing every line himself, deleting code that is no longer needed (printf lines, arr[1], arr[3]); uses comments as thinking aids and the sticky note as a to-do/plan; "very minimal hashtable".
- Second blank Terminal window opened at 09:28-09:30 (probably to host the next file / ChatGPT output; next segment may show).
