# Segment 9 (24:07 - 27:33) transcript

## Timeline
Layout throughout: LEFT Terminal, tabs `~/lab — vim test.c` (active) | `~/lab — -zsh`. RIGHT Terminal (vim, the other C file; a browser/terminal window edge is visible behind it at 24:07). A thin yellow bar (sliver of a sticky note, ~y=520, x 1090-1525) is visible over the right terminal in nearly every frame; it is cropped/hidden behind the window, so NO sticky text is legible in this segment. Webcam ignored (at 27:31 he holds a cigarette).

- [24:07] LEFT vim `-- VISUAL --`, 11 lines selected, cursor `37,1`, `86%`: lines 37-47 (the `arr[hash("abc")] = (Node) {...};` and `arr[hash("cba")].next = &(Node) {...};` blocks) highlighted. RIGHT vim VISUAL highlights lines 61-68 of `destroy()` (`Entry *e = table[i];` ... `while (e) {` ... `free(e->key); free(e); e = next; }`), view lines 34-69.
- [24:08] RIGHT highlight gone; cursor at blank line 51.
- [24:09] RIGHT scrolled to top: lines 1-36 (includes, `#define SIZE 101`, struct Entry, `table[SIZE]`, hash(), start of put()).
- [24:10] RIGHT lines 4-39; mouse I-beam near `unsigned long h = 5381;` (cursor on `;` of line 16). Reading djb2 hash.
- [24:12] RIGHT lines 16-51; cursor on space before `h` in line 16; mouse on `*e` in line 34 `Entry *e = malloc(sizeof *e);`.
- [24:13] RIGHT lines 19-54; cursor on `33` in line 19.
- [24:14] RIGHT VISUAL: lines 26-47 highlighted (put() body).
- [24:15] RIGHT VISUAL: highlight spans lines 24-47 = whole `put()`. Mouse on `const` in line 24. Stays highlighted for the rest of the segment.
- [24:18] LEFT focused (normal mode), cursor `32,2`, `86%`, lines 20-54; mouse I-beam on LEFT.
- [24:20] LEFT scrolled: lines 17-51, cursor `17,12` (on `;` of `h += *c;`), `72%`.
- [24:24] LEFT `-- INSERT --` `23,1` `69%`: an extra blank line inserted after hash()'s closing `}` (blank lines 23 and 24; get now at 25).
- [24:25] LEFT INSERT `24,6`: line 24 `void ` typed.
- [24:45] LEFT INSERT `25,3`, `64%`: line 24 `void set(Node *arr, const char *key, int value) {`, line 25 blank indented, line 26 blank. (He names it `set`, not `put`.)
- [24:46] LEFT INSERT `26,4`, `61%`: a stray `  {` on line 26 (typed, then removed).
- [24:54] LEFT NORMAL `25,0-1`, `59%`: line 25 blank (cursor), line 26 `  size_t index = hash(key);`, line 27 `}`.
- [24:55] LEFT NORMAL `25,3`, `61%`: line 25 `  size_t index = hash(key);` line 26 `}` (green cursor on `s`).
- [25:23] LEFT `26,3`, `59%`: line 26 `  const Node *p = &arr[index];` (cursor on `c`). set() = lines 24-27. (Copies first two lines of his get().)
- [25:43] LEFT scrolled lines 23-57, `81%`: get() 29-37, main 39, printf at 57 `printf("%d\n", get(arr, "abc"));`; mouse near `{0};` line 40.
- [25:57] LEFT back at lines 17-51, `59%`, `26,3`; mouse in get().
- [26:05] LEFT INSERT `27,3` `57%`: blank line 27 added inside set() (`}` now 28).
- [26:06] LEFT INSERT `28,3` `55%`: another blank (27 blank, 28 cursor, 29 `}`).
- [26:13] LEFT INSERT `30,5` `51%`: line 28 `  if (!p->key) {`, 29 blank, 30 cursor (indented 4), 31 `  }`, 32 `}`.
- [26:15] LEFT INSERT `28,3` `50%`: line 28 blank cursor line opened ABOVE `if`; 29 `  if (!p->key) {`, 30 blank, 31 `  }`, 32 `}`.
- [26:33] LEFT INSERT `28,47`: line 28 `  // this index is empty, just put value to it`. Mouse arrow at bottom-left of right window.
- [26:34] All windows slid off / desktop revealed: wallpaper hash-table diagram (keys John Smith, Lisa Smith, Sam Doe, Sandra Dee, Ted Baker; buckets 000, 001, 002, :, 151, 152 (red), 153, 154, :, 253, 254, 255; entries Lisa Smith 521-8976 under 001, John Smith 521-1234 -> Sandra Dee 521-9655 chained under 152, Ted Baker 418-4165 under 153, Sam Doe 521-50? under 254). "Screen Recording 2026-10...0.00 PM" icon top right. Pointer at bottom center.
- [26:35] Wallpaper; pointer on bucket `001`/`002` boundary (at 001-002).
- [26:41] Wallpaper; pointer at bucket 001-002 area (hidden/same as 26:35: arrow just left of 001 box, near 001... in frame 26:41 pointer is on Sam Doe entry ~(755,760)); NOTE: frames 26:35 (pointer at 390,340 near 001/002) and 26:41-26:42 (pointer at 755,760 on Sam Doe entry).
- [26:43] Windows back; LEFT INSERT `28,47` `50%`; mouse arrow near line 36 of get(); cursor block at end of line 28 comment.
- [26:53] LEFT same, mouse I-beam at line 29.
- [27:01] LEFT INSERT `31,5` `48%`: line 30 `    p->value = value;`, 31 cursor blank.
- [27:03] LEFT INSERT `32,5` `47%`: line 31 `    p->key = key;`, 32 blank cursor.
- [27:11] LEFT INSERT `32,5` `55%`, lines 20-54 view, same text.
- [27:12] LEFT INSERT `34,3` `54%`: line 32 `    p->next = NULL;`, 33 `  }`, 34 blank cursor, 35 `}`.
- [27:24] LEFT INSERT `35,6` `52%`: line 35 `  // ` (typing).
- [27:25] LEFT INSERT `35,53`: line 35 `  // if it not empty -> go to last node and add next`. Mouse arrow bottom of right window.
- [27:26] Wallpaper again, pointer at bottom (~725,900).
- [27:31] Wallpaper; pointer on bucket `001`.
- [27:32] Wallpaper; pointer on `521-9655` (Sandra Dee, chained second entry).
- [27:33] Windows back (two frames): LEFT INSERT `35,53`, line-35 comment shown with a trailing box cursor; -zsh tab shows `...` indicator; LEFT view lines 32-66 (`86%`), then lines 35-69 (`94%`) showing main with `printf("%d\n", get(arr, "abc"));` and `printf("%d\n", get(arr, "cba"));` at 66-67.

No sticky text legible. No shell commands in this segment. No vim `:w` messages seen. No browser page visible (only wallpaper).

## File states

### LEFT (test.c) as of 27:33 (lines 1-16 not visible in this segment)
```
 1..16| ?? (not visible here)
17|     h += *c;
18|     c++;
19|   }
20|
21|   return h % SIZE;
22| }
23|
24| void set(Node *arr, const char *key, int value) {
25|   size_t index = hash(key);
26|   const Node *p = &arr[index];
27|
28|   // this index is empty, just put value to it
29|   if (!p->key) {
30|     p->value = value;
31|     p->key = key;
32|     p->next = NULL;
33|   }
34|
35|   // if it not empty -> go to last node and add next
36| }
37|
38| int get(const Node *arr, const char *key) {
39|   size_t index = hash(key);
40|   const Node *p = &arr[index];
41|   for (; p; p = p->next)
42|    if (strcmp(p->key, key) == 0)
43|     return p->value;
44|
45|  return 0;
46| }
47|
48| int main(void) {
49|   Node arr[SIZE] = {0};
50|
51|   arr[hash("abc")] = (Node) {
52|     .value = 22,
53|     .key = "abc",
54|     .next = NULL
55|   };
56|   // because abc and cba have same index
57|   arr[hash("cba")].next = &(Node) {
58|     .value = 77,
59|     .key = "cba",
60|     .next = NULL
61|   };
62|
63|   // how we get access
64|   // get arr "abc" , get arr "def"
65|
66|   printf("%d\n", get(arr, "abc"));
67|   printf("%d\n", get(arr, "cba"));
68|
69|
```
Indentation as rendered: body lines 2 spaces; designated initializers 4 spaces; get()'s `if` at 3 spaces, `return p->value;` at 4, ` return 0;` at 1 space (odd indent present in his code). Pre-segment version (24:07): same but without set(); get at 24, main 34, abc/cba blocks 37-47, printf 52-53.

Intermediate versions of set(): 24:45 empty body; 25:23 two copied lines; 26:13 `if (!p->key) {` `}` skeleton; 26:33 comment; 27:03 value/key; 27:12 next = NULL; 27:25 second comment.

### RIGHT file (unchanged in this segment; only scrolled/highlighted)
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
24| void put(const char *key, int value) {
25|     unsigned long i = hash(key);
26|
27|     for (Entry *e = table[i]; e; e = e->next) {
28|         if (strcmp(e->key, key) == 0) {
29|             e->value = value;
30|             return;
31|         }
32|     }
33|
34|     Entry *e = malloc(sizeof *e);
35|     if (!e) return;
36|
37|     e->key = malloc(strlen(key) + 1);
38|     if (!e->key) {
39|         free(e);
40|         return;
41|     }
42|
43|     strcpy(e->key, key);
44|     e->value = value;
45|     e->next = table[i];
46|     table[i] = e;
47| }
48|
49| int *get(const char *key) {
50|     unsigned long i = hash(key);
51|
52|     for (Entry *e = table[i]; e; e = e->next)
53|         if (strcmp(e->key, key) == 0)
54|             return &e->value;
55|
56|     return NULL;
57| }
58|
59| void destroy(void) {
60|     for (int i = 0; i < SIZE; i++) {
61|         Entry *e = table[i];
62|
63|         while (e) {
64|             Entry *next = e-⟨?⟩;   (partly hidden by webcam)
65|             free(e->key);
66|             free(e);
67|             e = next;
68|         }
69|     (rest hidden)
```
Highlights by him in right file: 24:07 destroy() lines 61-68; 24:14 put() body 26-47; 24:15 onward whole put() 24-47 (VISUAL) held to end of segment.

## Shell & outputs
None in this segment. No compile, no `:w` message. (-zsh tab only shows `...` at 27:33.)

## Observations
- His new `set()` mirrors the right-hand `put()` conceptually but is written independently and simpler: right `put` walks the chain to update an existing key then mallocs a new Entry and pushes at head; his `set` stores the first node inline in `arr[index]` when `!p->key` and plans (comment only) to "go to last node and add next" (tail append). He copies the first two lines from his own get().
- Reads the right file top-down (24:09-24:15: djb2 hash 5381/33, put with malloc, strlen+1, free on failure), highlights whole put() as a unit, then goes back to his own file and types set() by hand, not copying.
- Writes intent as plain-English comments first, then code ("this index is empty, just put value to it", "if it not empty -> go to last node and add next").
- Uses the wallpaper diagram at 26:34-26:42 and 27:26-27:32: pointing at bucket 001 (single entry) and at Sandra Dee (second node in 152's chain) to reason about empty bucket vs collision chain, which matches the comments he is writing.
- Origin of right-hand code: no direct evidence here (right window title just `Terminal`; no filename, browser tab, or legible sticky). The unreadable yellow sticky sliver persists; a "Screen Recording 2026-10..." desktop icon is his own recording file. Style (4-space indent, `sizeof *e`, `(unsigned char)*s++`, put/get/destroy) differs from his 2-space style and looks externally produced, but that is inference only.
- Philosophy: learn by reading a finished reference, then rebuild his own smaller version incrementally in vim, comment-first, typing everything by hand.
- Note: my 26:41/26:42 pointer positions: 26:35 pointer at (390,340) near bucket 001/002; 26:41 pointer near (390,300)? (frame 26:41 shows pointer at 001-ish ~(335,305) is actually from 27:31); the 26:41 frame shows pointer at ~(755,760) on Sam Doe. Treat exact pointer positions as approximate.
