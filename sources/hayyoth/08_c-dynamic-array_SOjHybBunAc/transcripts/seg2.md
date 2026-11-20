# Segment 2 (03:56 - 06:56) transcript

All 39 frames viewed. Tabs throughout: `~/lab — vim main.c` (active), `~/lab — -zsh`. Terminal window left, painting wallpaper (Last Supper) behind, webcam bottom-right (ignored). No save message, no shell command, no compile, no man page appears in this segment.

## Timeline

- [03:56] Full-screen slide (no terminal): Last Supper painting with a drawn table/row of 12+ slots; slots numbered 1..6 filled; arrow labelled `len` spanning the 6 filled slots; line labelled `cap` spanning the whole row. This is the table/person analogy for len vs cap.
- [03:57] Terminal appears. vim main.c, normal mode, ruler `8,13  Top`, cursor on `;` of line 8. File (30 lines visible) is the "HARD VERSION" header stage: `/* HARD VERSION */` at line 4, typedef struct Slice at 6-10 with NO comments on len/cap yet (only `int *data; // array data`), then old-style main() with raw `arr` + malloc/realloc (lines 12+).
- [04:20] INSERT, `8,50`. Added trailing comments: line 7 `size_t len; // such as table have 6 person`; line 8 being typed `size_t cap; // such as table can have max 12 pe` (cursor mid-word "pe").
- [04:26] INSERT `12,1`. Line 8 now complete `// such as table can have max 12 person`. Blank lines opened at 11-13 (cursor on line 12, empty); `int main` pushed to line 14.
- [04:39] INSERT `13,3`. Typed `Slice *sl_init(size_t cap) {` on line 12; cursor on line 13 (indented 2); `int main` at line 15.
- [04:40] NORMAL `14,1  Top`. Closing `}` now at line 14 with matching-brace highlight on `{` line 12; line 13 blank; main at 16. (auto-closed brace)
- [04:53] INSERT `14,6`. Line 13 `Slice *sl = malloc(sizeof(Slice));`; line 14 `if ` being typed.
- [04:59] INSERT `15,3`. Line 14 `if (!sl) return NULL;`; line 15 blank cursor; `}` at 16; main at 18.
- [05:00] INSERT `16,4`. Line 15 blank, line 16 `s` (start of `sl->len`).
- [05:04] INSERT `17,5`. Line 16 `sl->len = 0;`; line 17 `sl` being typed.
- [05:07] INSERT `18,3`. Line 17 `sl->cap = cap;`; line 18 empty cursor.
- [05:24] INSERT `19,3`. Line 18 `sl->data = malloc(sizeof(int) * cap);`; cursor line 19.
- [05:29] INSERT `20,6`. Line 19 `if (!sl->data) {`; line 20 `f` typed (start of free).
- [05:32] INSERT `21,5`. Line 20 `free(sl);`; line 21 cursor.
- [05:35] INSERT `22,4`. Line 21 `return NULL;`; line 22 `}` (matching-brace highlight line 19/22).
- [05:36] INSERT `23,3`. Cursor on blank line 23 after the if-block; `}` of function at 24; main at 26.
- [05:37] INSERT `24,5`. Line 24 `re` typed (return); blank line 23 above; `}` line 25.
- [05:45] NORMAL, view scrolled (top line = 7), ruler `35,0-1  23%`. Line 24 now `return sl;`, `}` line 25, main at 27. Cursor on blank line 35 inside main (between arr[2] line 34 and for-loop 36).
- [05:47] INSERT `27,2  21%`. New line 27 `v` typed (beginning of `void`), blank line 28, main at 29. (Added blank 26, new function begins at 27.)
- [05:54] INSERT `28,3  20%`. Line 27 `void sl_free(Slice *sl) {`; cursor line 28; main at 30.
- [06:01] INSERT `29,6  20%`. Line 28 `free(sl->data);`; line 29 `fre` typed.
- [06:03] INSERT `30,3  19%`. Line 29 `free(sl);`; cursor on line 30 (about to close; closing `}` not yet shown, line 31 blank, main at 32).
- [06:07] NORMAL, view top line = 15, `44,0-1  45%`. sl_free now closed: line 30 `}`. main at 32. Cursor at line 44 (blank, after `puts("");` line 43).
- [06:08] NORMAL `34,1  48%`. Cursor on line 34 (`int *arr = malloc(...)`), scrolled top = 16.
- [06:10] VISUAL LINE `35,1  48%`, selection count `3`: lines 33-35 highlighted (`size_t cap = 3;`, `int *arr = malloc(sizeof(int) * cap);`, `if (!arr) return 1;`).
- [06:11] INSERT `33,3  51%`. Those 3 lines deleted (replaced by blank); main now: line 32 `int main(void) {`, 33 cursor (indented), 34 blank, 35 `arr[0] = 11;` ...
- [06:22] INSERT `34,5  50%`. Line 33 `Slice *sl = sl_init(3);`; line 34 `if` typed.
- [06:28] NORMAL `56,11  86%` (top line = 27). Line 34 `if (!sl) return 1;`. Rest of main still uses the old `arr`/`cap`/`tmp` code (lines 36-56). Cursor at 56 on `;`.
- [06:29] NORMAL `57,0-1  Bot` (top = 31). File ends line 60. Line 58 `free(arr);`, 59 `return 0;`, 60 `}`.
- [06:36] NORMAL `30,1  96%` (top = 30). Line 58 now `sl_free(sl);` (edited from `free(arr);` between 06:29 and 06:36); 59 `return 0;`. Cursor on line 30 `}`.
- [06:37] NORMAL `17,14  53%`. Cursor on `cap` in line 17 `sl->cap = cap;` (reviewing sl_init, top = 17).
- [06:38] NORMAL `13,14  40%`. Cursor at col 14 of line 13 (on space after `=`; text appears as `Slice *sl =malloc(` with block cursor on the space). Top = 13.
- [06:40] INSERT `32,1  37%`. New blank lines inserted after sl_free: line 31 blank, 32 cursor, 33 blank, `int main` at 34.
- [06:49] INSERT `33,3  36%`. Line 32 `void sl_print(const Slice *sl) {`; cursor line 33; line 34 blank; main at 35.
- [06:51] NORMAL `47,3  57%` (top = 20). Cursor on line 47 (`// how we add 44`, on first `/`). sl_print body still empty (33, 34 blank; no closing `}` visible).
- [06:52] VISUAL LINE `44,3  57%`, count `2`: lines 43-44 (`for (...)` / `printf("%d ", arr[i]);`) highlighted. Yanking for-loop (plan: move into sl_print).
- [06:53] NORMAL `38,0-1  63%`; message `3 fewer lines` (lines 43-45, the for/printf/puts, deleted in main; likely `dd` x3 or `3dd`). Main now lines 35-: 36 `Slice *sl = sl_init(3);`, 37 `if (!sl) return 1;`, 38 blank, 39-41 `arr[0..2] = ...`, 42-43 blank, 44 `// how we add 44`.
- [06:55] NORMAL `34,3  57%`; message `3 more lines` (paste). Pasted into sl_print: line 33 blank, 34 `for (size_t i = 0; i < cap; i++)`, 35 `printf("%d ", arr[i]);`, 36 `puts("");`, 37 blank, 38 `int main(void) {`. Note: paste landed AFTER blank line 33 and the sl_print `}` is NOT yet present. Cursor on `f` of for (line 34).
- [06:56] NORMAL `33,3  59%`. Blank line 33 removed; for/printf/puts now at 33-35, line 36 blank, `int main` line 37. Still no closing `}` for sl_print; loop still uses `cap` and `arr[i]` (not yet converted to `sl->`).

## File states

### State A: 03:57 (start; lines 1-30 visible)
```
 1| #include <stdio.h>
 2| #include <stdlib.h>
 3|
 4| /* HARD VERSION */
 5|
 6| typedef struct {
 7|   size_t len;
 8|   size_t cap;
 9|   int *data; // array data
10| } Slice;
11|
12| int main(void) {
13|   size_t cap = 3;
14|   int *arr = malloc(sizeof(int) * cap);
15|   if (!arr) return 1;
16|
17|   arr[0] = 11;
18|   arr[1] = 22;
19|   arr[2] = 33;
20|
21|   for (size_t i = 0; i < cap; i++)
22|     printf("%d ", arr[i]);
23|   puts("");
24|
25|   // how we add 44
26|   cap++;
27|   int *tmp = realloc(arr, sizeof(int) * cap);
28|   if (!tmp) {
29|     free(arr);
30|     return 1;
```
(rest of main not visible until later frames; see State C.) Indent is 2 spaces; the `printf` under a brace-less `for` is indented 4.

### State B: 05:35-05:45 (sl_init complete)
```
 1| #include <stdio.h>
 2| #include <stdlib.h>
 3|
 4| /* HARD VERSION */
 5|
 6| typedef struct {
 7|   size_t len; // such as table have 6 person
 8|   size_t cap; // such as table can have max 12 person
 9|   int *data; // array data
10| } Slice;
11|
12| Slice *sl_init(size_t cap) {
13|   Slice *sl = malloc(sizeof(Slice));
14|   if (!sl) return NULL;
15|
16|   sl->len = 0;
17|   sl->cap = cap;
18|   sl->data = malloc(sizeof(int) * cap);
19|   if (!sl->data) {
20|     free(sl);
21|     return NULL;
22|   }
23|
24|   return sl;
25| }
26|
27| int main(void) {
28|   size_t cap = 3;
29|   int *arr = malloc(sizeof(int) * cap);
30|   if (!arr) return 1;
```

### State C: END of segment (06:56)
Lines 1-19 as in State B (unchanged; line 13 may display `Slice *sl =malloc(` at 06:38 only because the block cursor sat on the space). Lines 20-49 visible at 06:56, line 50+ from earlier frames (06:29/06:36) with line numbers shifted +7 (main at 37 now vs 32 then... exact tail below reconstructed from 06:36 plus offset, mark uncertain).
```
 1| #include <stdio.h>
 2| #include <stdlib.h>
 3|
 4| /* HARD VERSION */
 5|
 6| typedef struct {
 7|   size_t len; // such as table have 6 person
 8|   size_t cap; // such as table can have max 12 person
 9|   int *data; // array data
10| } Slice;
11|
12| Slice *sl_init(size_t cap) {
13|   Slice *sl = malloc(sizeof(Slice));
14|   if (!sl) return NULL;
15|
16|   sl->len = 0;
17|   sl->cap = cap;
18|   sl->data = malloc(sizeof(int) * cap);
19|   if (!sl->data) {
20|     free(sl);
21|     return NULL;
22|   }
23|
24|   return sl;
25| }
26|
27| void sl_free(Slice *sl) {
28|   free(sl->data);
29|   free(sl);
30| }
31|
32| void sl_print(const Slice *sl) {
33|   for (size_t i = 0; i < cap; i++)
34|     printf("%d ", arr[i]);
35|   puts("");
36|
37| int main(void) {
38|   Slice *sl = sl_init(3);
39|   if (!sl) return 1;
40|
41|   arr[0] = 11;
42|   arr[1] = 22;
43|   arr[2] = 33;
44|
45|
46|   // how we add 44
47|   cap++;
48|   int *tmp = realloc(arr, sizeof(int) * cap);
49|   if (!tmp) {
50|     free(arr);
51|     return 1;
52|   }
53|
54|   arr = tmp;
55|   arr[3] = 44;
56|   for (size_t i = 0; i < cap; i++)
57|     printf("%d ", arr[i]);
58|   puts("");
59|
60|   sl_free(sl);
61|   return 0;
62| }
```
Lines 50-62 are INFERRED by shifting the 06:36 view (where lines 36-59 were identical text, +7 offset after for-loop move: net loss of 3 lines in main, gain of 3 in sl_print => file tail shifted by 0 relative to line 38 shift +1... only lines 20-49 directly seen at 06:56; the rest `??` regarding exact numbers). Treat lines 50+ numbering as `??`. Note: sl_print `}` missing (line 36 is blank, no `}`) as of 06:56, and main body still references undefined `arr`/`cap` (work in progress, would not compile).

### Intermediate snapshot 06:36 (before sl_print work; main lines 31-59)
```
30| }
31|
32| int main(void) {
33|   Slice *sl = sl_init(3);
34|   if (!sl) return 1;
35|
36|   arr[0] = 11;
37|   arr[1] = 22;
38|   arr[2] = 33;
39|
40|   for (size_t i = 0; i < cap; i++)
41|     printf("%d ", arr[i]);
42|   puts("");
43|
44|   // how we add 44
45|   cap++;
46|   int *tmp = realloc(arr, sizeof(int) * cap);
47|   if (!tmp) {
48|     free(arr);
49|     return 1;
50|   }
51|
52|   arr = tmp;
53|   arr[3] = 44;
54|   for (size_t i = 0; i < cap; i++)
55|     printf("%d ", arr[i]);
56|   puts("");
57|
58|   sl_free(sl);
59|   return 0;
(60| } at 06:29 with `free(arr);` on line 58)
```

## Shell & outputs

None in this segment. No `:w` save message visible (no `"main.c" NL, NB written`). Only vim messages: `3 fewer lines` (06:53), `3 more lines` (06:55). Shell tab `~/lab — -zsh` never activated.

## Observations

- Build order: slide (len vs cap analogy) -> annotate struct fields with the table-seating analogy -> `sl_init` (alloc struct, alloc data buffer, init len=0, cap=cap) -> `sl_free` -> start `sl_print` (by moving old for-loop out of main). main is being converted from raw `arr` array to Slice incrementally: first replaced the 3 setup lines with `Slice *sl = sl_init(3); if (!sl) return 1;`, and `free(arr)` at end became `sl_free(sl)`; the middle (arr[0..2] assignments, realloc block) is still old code.
- Design: Slice is heap-allocated and returned by pointer (`Slice *sl_init(size_t cap)`); two allocations (struct then data), with proper partial-failure cleanup (`free(sl); return NULL;` when data malloc fails). `len` starts at 0 and `cap` is the requested cap (so main's old cap=3 maps to sl_init(3), with len=0 meaning old arr[0..2] writes will need an append). Ownership: `sl_free` frees data first then the struct; caller owns the pointer. Data type still `int *` (typed version; "HARD VERSION" label in line-4 comment). Error handling: NULL return + caller check `if (!sl) return 1;` style, consistent with existing `if (!arr) return 1;`.
- Style: 2-space indent, `Type *name` star attached to name, brace on same line, one-line `if (!x) return ...;` guards, `const Slice *sl` on read-only print. Typedef anonymous struct `Slice`. `size_t` for len/cap. Uses `sizeof(int) * cap` (not `sizeof *data`).
- Comments (analogy-driven, informal English): `// such as table have 6 person` (len), `// such as table can have max 12 person` (cap), `// array data`, `// how we add 44` (old). Slide at 03:56 introduces the visual: row of seats = capacity, filled seats = length.
- Editing habits: vim, types code inline with auto-closing braces, uses VISUAL LINE + delete/paste to move code (`3 fewer lines` / `3 more lines`), blank line before return, jumps with scroll percentages visible in the ruler. Cursor col at 06:38 sitting on a space in `=malloc` (visual quirk, not a typo).
- Philosophy signals: incremental refactor (keep program in a half-working state while migrating), error-handling first-class on each alloc, visual analogy before code.
