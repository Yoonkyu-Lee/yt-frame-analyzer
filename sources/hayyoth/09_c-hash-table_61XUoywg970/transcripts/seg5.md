# Segment 5 (09:30 - 13:23), frames k_09m30s .. k_13m23s (40 frames)

## Timeline

- [09:30] LEFT terminal window (title "Terminal", tabs `~/lab — vim test.c` (active) and `~/lab — -zsh`), vim shows the file with status line `"main.c" 27L, 342B written   24,4  All`. NOTE: tab title says `vim test.c` but the vim message names `main.c` (file is main.c; tab title stale or test.c was renamed). Content at this point is the 27-line Node/arr program (see File states v1). RIGHT window is an empty second Terminal showing prompt `lab $ ` with block cursor. A thin yellow sticky note (collapsed, rolled up to title bar only, no text visible) sits at about x=1128-1560, y=228. Window positions: left window moved/shifted slightly between 09:30 and 09:31.
- [09:31] Left window repositioned (now x=38). Cursor at 24,4. Right Terminal empty `lab $ `.
- [09:32-09:33] Right Terminal window dragged left/up (title bar grab); still empty `lab $ `. Mouse I-beam in it at 09:33.
- [09:36] RIGHT terminal now shows vim with `"test.c" 91L, 1554B` at `1,1   Top`. Line numbers on. This is the hash-table file (header lines 1-21 visible). So the right file is named test.c (91 lines, 1554 bytes), a different directory/ file from left `main.c`. Opened presumably via `vim test.c` in the right terminal (command itself not seen).
- [09:39] Right window retitled `Terminal — 60×36` (window being resized); vim display scrolled/redrawn showing lines 1-34 with cursor at top (line numbering shifted down by one row = redraw artifact). Webcam overlay covers lower right part of the right code.
- [09:40] Right window `Terminal`, vim lines 1-36 visible. Cursor on line 1 col 1 (`#include`).
- [09:41] Right window moved down slightly (title bar at y=64). Lines 1-36 visible.
- [09:44] Right vim scrolled to bottom: lines 69-91 visible, ending with main() (see right file). Cursor block at line 69. Tildes after line 91. Sticky note still collapsed bar.
- [09:45] Back to top: lines 1-36 visible, cursor at line 36 (block at start of line 36, blank).
- [09:52] Right window inactive (greyed title); LEFT window active. Right vim in `-- VISUAL --` mode with line 9 `    int value;` highlighted (visual selection from line 9 col 5 to end `int value;`). Left mouse I-beam over left window at (652,220). Left still `"main.c" 27L, 342B written 24,4`.
- [10:02] Left vim in `-- INSERT --` at 20,5: he opened a new line 20 below `//arr[7 % 5] = 77;` containing `//` (typing a comment). Left file now 28 lines. Right still VISUAL with line 9 highlighted.
- [10:08] Left INSERT 21,6. Lines: 20 `  // 7 % 5`, 21 `  // |` (typing next comment). 29 lines.
- [10:20] Left INSERT at 8,1: he has finished comments: line 22 `  // it is hash function`; and has opened a blank line 8 plus blank 9 (extra blank line inserted before `int main`). Lines: 21 `  // 7 % 5`, 22 `  // it is hash function`. 30 lines. Cursor at line 8 col 1.
- [10:21] Left INSERT 9,1: another blank line added (two blank lines 8,9 then 10 blank, `int main` at 11). 31 lines. (He is making room above main to write a hash function.)
- [10:50] Left INSERT 10,3: wrote at line 9 `size_t hash(int key) {` and cursor at line 10 inside body (indent 2). main now at line 12. 32 lines total.
- [10:56] Left INSERT 3,1: cursor on new blank line 3 (inserting a line above `typedef`, presumably for `#define`). Line 11 `size_t hash(int key) {`, line 12 `  return key %` (incomplete). 34 lines.
- [11:06] Left normal mode 13,1, 35 lines: line 3 `#define SIZE 5`, line 11 `size_t hash(int key) {`, 12 `  return key % SIZE;`, 13 `}` (the `{` on line 11 is highlighted cyan = matching brace, cursor on `}`). Comments still present in main.
- [11:13] Left normal 18,3, line 16 now `  Node arr[SIZE] = {0};` (was `arr[5]`). Line 18 `  arr[2] = (Node) {` - the `//arr[2] = 22;` comment line was deleted. File 34 lines.
- [11:26] Left `-- VISUAL LINE --` (selected 1 line) at 24,18: line 24 `  //arr[7 % 5] = 77;` selected. Lines 18-21 now: `arr[hash(2)] = (Node) {`, `.value = 22,`, `.key = hash(2),`, `.next = NULL`.
- [11:27] Left shows `3 fewer lines` (he deleted the 3 comment lines 24-26: `//arr[7 % 5] = 77;`, `// 7 % 5`, `// it is hash function`). 31 lines. Cursor 24,3 on `arr[2].next = &(Node) {` (still arr[2]).
- [11:40] Left tab switched to `-zsh` (active tab) in left window. Shell output verbatim:
  ```
  lab $ cc main.c && ./a.out
  11
  lab $ cc main.c && ./a.out
  77
  77
  lab $ cc main.c
  lab $ 
  ```
  (Prompt lines show `[` and `]` markers at the edges, i.e. zsh/terminal prompt marks.) The first run printed `11`, the second printed `77` twice. Last `cc main.c` produced no output (compiled clean). These runs belong to an earlier code state that is not on screen (the programs printing 11, 77/77 came from earlier versions); no source of the printf is visible in this segment's frames.
- [11:43] Back to vim tab. Left status `"main.c" 31L, 393B written   25,16  All`. Lines 24-27 now: `arr[hash(7)].next = &(Node) {`, `.value = 77,`, `.key = hash(7),`, `.next = NULL` (changed from `arr[2]` / `.key = 7`). Written (saved) with 31 lines, 393 bytes.
- [11:47] Left cursor 9,7 (on `;` of `} Node;`). Right-hand: a NEW Stickies note opened (full-size yellow note at x=1128-1560, y=222-508, empty, text cursor at top-left, with note controls). He clicked in it to type.
- [11:55] Sticky note now rolled to title bar with typed text (visible in the title bar): `now refactor key to string` (the collapsed note shows a checkbox-like icon at left and the text in the title bar). Mouse at the note. Left status still `"main.c" 31L, 393B written 9,7`.
- [11:57] Sticky shows `now refactor key to string` (collapsed). In right vim, VISUAL highlight moved to line 8 `    char *key;` (selected `char *key;`; mouse I-beam over `char`).
- [12:02] Left normal 7,3 (cursor on line 7 `struct Node *next;`, 30 lines). Left file now WITHOUT blank line 2... actually the lines shift: `#define SIZE 5` at line 3, `typedef struct Node {` line 5, `int value;` line 6, `struct Node *next;` line 7, `} Node;` line 8; hash at line 10. i.e. line `int key;` was deleted (dd on line 6 originally `int key;`). 30 lines. Right: line 8 highlighted `char *key;`.
- [12:03] Left INSERT 6,4: line 6 is `  c` (typing `char *key;` at the top of the struct); `int value;` line 7, `struct Node *next;` line 8, `} Node;` line 9. Hash at line 11. 31 lines.
- [12:33] Left INSERT 12,3: struct now `char *key; int value; struct Node *next;`. hash signature changed to `size_t hash(const char *key) {`; line 12 blank with cursor (indent 2); line 13 `  return key % SIZE;`. main at 16. 32 lines. Webcam: he is holding a cigarette (ignore).
- [12:42] Left INSERT 13,3: line 12 `  size_t h = 0;`, line 13 blank cursor, line 14 `  return key % SIZE;`. 33 lines.
- [12:44] Left normal 13,0-1 (Esc'd). Right vim: VISUAL (line-wise) selection of lines 15-22 hash function (from line 15 `unsigned long hash(const char *s) {` through `}` at line 22), mouse I-beam at (830,418). Right window active (title bold).
- [12:45] Right VISUAL selection now spans lines 14-22 (starts at blank line 14; mouse at (930,460) over `while (*s)`). Left unchanged.
- [12:48] Left INSERT 14,3: an extra blank line inserted (lines 13, 14 blank, `return key % SIZE;` now line 15). 34 lines. Right selection lines 14-22 unchanged.
- [12:49] Left INSERT 14,3: another blank (13,14,15 blank; `return` at line 16). 35 lines.
- [12:55] Left INSERT 15,3 `Top`: line 14 `  const char *c = key;`, line 15 blank cursor, line 16 blank, `return key % SIZE;` line 17. Window scrolled (Top; 35+ lines).
- [13:05] Left INSERT 16,5: lines 14 `  const char *c = key;`, 15 `  while (*c) {`, 16 cursor (indent 4, blank). `return key % SIZE;` at line 18 (blank 17).
- [13:11] Left INSERT 17,5: line 16 `    h += *c;`, line 17 cursor (blank, indent 4).
- [13:13] Left INSERT 18,5: line 16 `    h += *c;`, line 17 `    c++;`, line 18 cursor.
- [13:22] Left normal 20,10 `Top`, status `"main.c" 39L, 485B written`. Hash function complete: line 15 `while (*c) {`, 16 `h += *c;`, 17 `c++;`, 18 `}`, 19 blank, 20 `  return h % SIZE;` (cursor on the `h`), 21 `}`. The sticky note (`now refactor key to string`, collapsed bar) was dragged OVER the left window (now at x=455-888, y=194), mouse on it.
- [13:23] Sticky note expanded in the middle of the screen (x=453-885, y=187-475): body text `now refactor key to string` in large text (single line, text cursor after "string"). Confirms sticky text.

## File states

### LEFT file (`main.c`; tab labeled `~/lab — vim test.c`) as of 13:23 (end of segment)
Vim status: `"main.c" 39L, 485B written   20,10   Top`. Only lines 1-35 visible (rest scrolled off; file is 39 lines). Lines 36-39 not seen in this segment (expected remainder of arr[hash(7)] block, `return 0;`, `}` — NOT verified).
```
 1| #include <stdio.h>
 2|
 3| #define SIZE 5
 4|
 5| typedef struct Node {
 6|   char *key;
 7|   int value;
 8|   struct Node *next;
 9| } Node;
10|
11| size_t hash(const char *key) {
12|   size_t h = 0;
13|
14|   const char *c = key;
15|   while (*c) {
16|     h += *c;
17|     c++;
18|   }
19|
20|   return h % SIZE;
21| }
22|
23| int main(void) {
24|   Node arr[SIZE] = {0};
25|
26|   arr[hash(2)] = (Node) {
27|     .value = 22,
28|     .key = hash(2),
29|     .next = NULL
30|   };
31|
32|   arr[hash(7)].next = &(Node) {
33|     .value = 77,
34|     .key = hash(7),
35|     .next = NULL
36| ?? (lines 36-39: presumably `  };`, blank, `  return 0;`, `}` — not visible)
```
Indentation: 2 spaces. Note main body still uses old int-key calls (`hash(2)`, `.key = hash(2)`) which no longer match `const char *key`; he hasn't refactored yet (sticky: "now refactor key to string"). Also `hash` still has the leftover original `h` logic: `h += *c; c++;` (a simple additive char sum, not djb2).

Intermediate versions:
v1 (09:30, 27 lines, `"main.c" 27L, 342B written`):
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
v2 (10:20, comments added before the second assignment):
```
19|   //arr[7 % 5] = 77;
20|   // 7 % 5
21|   // it is hash function
22|   arr[2].next = &(Node) {
```
v3 (11:06): `#define SIZE 5` at line 3; 
```
11| size_t hash(int key) {
12|   return key % SIZE;
13| }
```
Both `//arr[2] = 22;` removed by 11:13; `Node arr[SIZE] = {0};`.
v4 (11:13): `arr[2] = (Node) {` still with `.key = 2,`.
v5 (11:26-11:27): `arr[hash(2)] = (Node) {`, `.key = hash(2),`; comments at old lines 24-26 deleted (`3 fewer lines`), 31 lines.
v6 (11:43, saved 31L 393B): `arr[hash(7)].next = &(Node) {` ... `.key = hash(7),`
```
 1| #include <stdio.h>
 2|
 3| #define SIZE 5
 4|
 5| typedef struct Node {
 6|   int value;
 7|   int key;
 8|   struct Node *next;
 9| } Node;
10|
11| size_t hash(int key) {
12|   return key % SIZE;
13| }
14|
15| int main(void) {
16|   Node arr[SIZE] = {0};
17|
18|   arr[hash(2)] = (Node) {
19|     .value = 22,
20|     .key = hash(2),
21|     .next = NULL
22|   };
23|
24|   arr[hash(7)].next = &(Node) {
25|     .value = 77,
26|     .key = hash(7),
27|     .next = NULL
28|   };
29|
30|   return 0;
31| }
```
v7 (12:33): struct key changed to `char *key;` (replaces `int key;`, put as first member), `size_t hash(const char *key) {` with blank body line then `return key % SIZE;`.
v8 (12:42): added `size_t h = 0;`.

### RIGHT file (`test.c`, 91 lines, 1554 bytes; "test.c" 91L, 1554B at 1,1 Top, read-only usage / highlighting)
Seen lines 1-36 and 69-91 in this segment. Lines 37-68 NOT seen. Indent is 4 spaces.
```
 1| #include <stdio.h>
 2| #include <stdlib.h>
 3| #include <string.h>
 4|
 5| #define SIZE 101
 6|
 7| typedef struct Entry {
 8|     char *key;
 9|     int value;
10|     struct Entry *next;
11| } Entry;
12|
13| Entry *table[SIZE];
14|
15| unsigned long hash(const char *s) {
16|     unsigned long h = 5381;
17|
18|     while (*s)
19|         h = h * 33 + (unsigned char)*s++;
20|
21|     return h % SIZE;
22| }
23|
24| void put(const char *key, in?? value) {          <- right part hidden by webcam; likely `int value) {`
25|     unsigned long i = hash(k??                   <- hidden by webcam (likely `hash(key);`)
26|
27|     for (Entry *e = table[i]??                   <- hidden
28|         if (strcmp(e->key, k??                   <- hidden
29|             e->value = value??                   <- hidden (partly visible: `e->value = value`)
30|             return;
31|         }
32|     }
33|
34|     Entry *e = malloc(sizeof??                   <- hidden
35|     if (!e) return;
36|
37..68| ?? not seen
69|
70|         table[i] = NULL;
71|     }
72| }
73|
74| int main(void) {
75|     put("apple", 10);
76|     put("banana", 20);
77|     put("orange", 30);
78|     put("apple", 99);
79|
80|     int *value = get("apple");
81|
82|     if (value)
83|         printf("%d\n", *value);  // 99
84|
85|     value = get("banana");
86|
87|     if (value)
88|         printf("%d\n", *value);  // 20
89|
90|     destroy();
91| }
```
Line 31 `}` at 8 spaces indentation shows in the frame as closing the if; line 30 `return;` is at 12 spaces. Line 69 is a blank line with the cursor.

## Shell & outputs

- [09:30-09:33] Right terminal prompt: `lab $ ` (empty, nothing run).
- [09:36] Right terminal runs vim; status `"test.c" 91L, 1554B` (opened right-hand file named test.c). The vim-open command is not visible.
- [10:02-13:23] No compile commands in vim windows except below.
- [11:40] Left window, `-zsh` tab:
```
lab $ cc main.c && ./a.out
11
lab $ cc main.c && ./a.out
77
77
lab $ cc main.c
lab $ 
```
Interpretation: these are leftover history from earlier edits (outputs 11, 77 77), then a bare `cc main.c` to compile (no errors). No printf is on screen in the left file at 11:40 (visible left code has no printf), so the shell history shown is of earlier versions of main.c (before this segment) — or the lines were from earlier before he deleted printfs; uncertain.
- Vim save messages (verbatim): `"main.c" 27L, 342B written` (09:30-11:13 persisting), `"main.c" 31L, 393B written` (11:43-11:57), `"main.c" 39L, 485B written` (13:22-13:23). Vim read message on right: `"test.c" 91L, 1554B`. Others: `3 fewer lines` (11:27).

## Observations

- ORIGIN of the right-hand code: no explicit evidence in this segment. File name is `test.c` (91 lines, 1554 bytes) in a second Terminal (prompt `lab $`, same ~/lab style prompt name), distinct from the left `main.c` (tab label still says `vim test.c`). No browser, no AI chat window, no visible sticky note naming a source. The code style (4-space indent, `unsigned long hash(const char *s)` djb2 with 5381, 33, `(unsigned char)*s++`, `table[SIZE]`, `destroy()`, comments `// 99` `// 20`) looks like a pre-generated reference solution (typical of ChatGPT-style output), but this is inference, not seen evidence. The left tab/title `vim test.c` vs. file `main.c` suggests the filename test.c is also used for the reference file. He appears to have created/prepared the reference file beforehand (it is already open at 09:36).
- Also note the wallpaper hash-table diagram (keys / entries, Lisa Smith, John Smith, Sam Doe, Sandra Dee, Ted Baker with phone numbers 418-4165, 521-502x) is visible behind the windows; he does not point at it in this segment.
- Learning approach: bottom-up and incremental. He keeps the right-hand reference file open, scrolls through it (top, bottom main(), top), and uses VISUAL mode to highlight specific parts as he reads them (09:52-11:55: line 9 `int value;`; 11:57-12:03: line 8 `char *key;`; 12:44-13:23: lines 14/15-22, the hash function). He then rebuilds the same piece in his own left file in his own style (2-space indent, `Node` instead of `Entry`, `size_t`, `h += *c; c++;` rather than djb2). So the right file is a reference/target, the left file is his own re-derivation, step by step.
- Concept steps he takes in his own code: (1) hash function = replaces literal `7 % 5` with `key % SIZE` (comment: `// 7 % 5` / `// it is hash function`), using `#define SIZE 5`, `Node arr[SIZE]`; (2) uses `arr[hash(2)]`, `.key = hash(2)`, `arr[hash(7)].next = &(Node){...}` — still collision chaining via `.next` (key 7 % 5 collides with 2 % 5 = 2); (3) then decides to move from int keys to string keys: sticky note `now refactor key to string`, changes `int key` to `char *key`, `hash(const char *key)`, and starts a string-iterating loop (`const char *c = key; while (*c) { h += *c; c++; }`, `return h % SIZE;`). Note the `*c` iteration mirrors the right-hand `while (*s)`.
- The sticky note (Stickies) is created at 11:47 (empty new note), contents typed by 11:55: `now refactor key to string`. It is a TODO-style note acting as a plan marker; he later (13:22) drags it over the left window and re-expands it at 13:23 (still same single-line text). The earlier collapsed yellow bar (from 09:30 onward) was an older rolled-up note whose text is not visible; it is probably the same note or a pre-existing one rolled up (unknown).
- Philosophy signals: explicit incremental refactoring, keeps old code as comments before deleting; writes by hand rather than pasting from the reference; manually typed (no copy/paste of hash body). Uses `cc main.c && ./a.out` run-and-check cycle. Webcam shows him thinking, smoking a cigarette at 12:33-12:42 (ignored).
- Unclear / not seen: contents of lines 36-39 of the left file; lines 37-68 of the right file; which program produced output `11` and `77 77` in the shell.
