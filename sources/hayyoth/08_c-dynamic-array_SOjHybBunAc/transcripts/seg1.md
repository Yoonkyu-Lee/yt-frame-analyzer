# Segment 1 (0:00 - 3:56) transcript

Frames viewed: all 38 in seg1_frames.txt. Layout: Terminal window, two tabs (tab A = vim, tab B = `~/lab — -zsh`), shell prompt is `lab $ `. Vim has `number` on, 2-space indent, cream theme, ruler on.

## Timeline

- [00:00] Tab A `~/lab — -zsh`, tab B `~/lab — -zsh`. Empty prompt `lab $ `.
- [00:02] Tab A title `~/lab — vim`. Vim splash ("VIM - Vi IMproved, version 9.2.1200, by Bram Moolenaar et al.", "Help poor children in Uganda!"). Already `-- INSERT --`, ruler `0,1  All`, line 1 empty (unnamed buffer).
- [00:03] Typing intro note in the unnamed buffer: line 1 `This video ` (ruler 1,12).
- [00:16] INSERT, ruler `3,31 All`, typing in progress:
  ```
  1 This video will help you understand
  2 - dynamic array in C
  3 - dynamic array in any languag
  ```
  (line 3 cut off mid-typing at `languag`)
- [00:28] Tab A title back to `~/lab — -zsh` (vim exited, note discarded). Shell: `lab $ e` then fresh `lab $ ` (stray `e`, no output).
- [00:30] Tab A title `~/lab — vim main.c`. Status line `"main.c" [New]`, ruler `4,2  All`. Template (normal mode, cursor on line 4):
  ```
  1 #include <stdio.h>
  2
  3 int main(void) {
  4
  5   return 0;
  6 }
  ```
- [00:40] Normal mode, ruler `7,3 All`. Line 4 `  int arr[3];`; lines 6-8 all `  arr[0] = 11;` (yank/put duplicate in progress, about to edit indexes); `return 0;` line 9, `}` line 10.
- [00:58] INSERT, ruler `11,11 All`: lines 6-8 now `arr[0] = 11; arr[1] = 22; arr[2] = 33;`, line 10 `  for (size_t i = 0; i < 3; i++)`, line 11 `    printf` (typing).
- [01:11] Normal. Message `<ain.c" [New] 15L, 188B written` (left-truncated display of `"main.c" [New] 15L, 188B written`), ruler `12,11 All`. Line 11 `    printf("%d ", arr[i]);`, line 12 `  puts("");` (cursor on its `;`).
- [01:13] Tab B active (tab A title `~/lab — vim main.c`). STALE scrollback from an earlier (rehearsal) program, not from this file:
  ```
  lab $ cc main.c && ./a.out
  main.c:13:24: warning: comparison of integers of different signs: 'size_t' (aka 'unsigned long') and 'int' [-Wsign-compare]
     13 |   for (size_t i = 0; i < cap; i++) {
        |                      ~ ^ ~~~
  main.c:28:24: warning: comparison of integers of different signs: 'size_t' (aka 'unsigned long') and 'int' [-Wsign-compare]
     28 |   for (size_t i = 0; i < cap; i++) {
        |                      ~ ^ ~~~
  2 warnings generated.
  11 22 33
  11 22 33 44
  lab $
  ```
- [01:14] Tab B cleared: `lab $ c` (typing).
- [01:19] Tab B:
  ```
  lab $ cc main.c && ./a.
  zsh: no such file or directory: ./a.
  lab $ cc main.c && ./a.out
  ```
  (typo `./a.` first; retyped command not yet entered at this frame.)
- [01:22] Back in tab A, normal, same `<ain.c" [New] 15L, 188B written`, `12,11 All`.
- [01:30] INSERT `2,5 All`: typing `#inc` on new line 2. Also `  // how we add 44` now at line 15 (blank 14 and 16). File 18 lines.
- [01:45] INSERT `5,3 All`. Line 2 `#include <stdlib.h>`. Line 6 `  int *arr = malloc(sizeof(int) *;` (mid-edit). 19 lines.
- [01:54] INSERT `7,3 All`: line 5 `  size_t cap = 3;`, line 6 `  int *arr = malloc(sizeof(int) * cap);`, cursor on empty line 7. For loop still `i < 3`.
- [02:11] Normal. `"main.c" 21L, 309B written`, `19,12 All`. Line 7 `  if (!arr) return 1;`, loop now `i < cap`, line 19 `  free(arr);`.
- [02:12] Tab B: `lab $ cc main.c && ./a.out` typed, not yet run.
- [02:17] Tab A normal, same save msg, `19,12 All`. Mouse I-beam over text.
- [02:20] INSERT `18,6 All`: line 18 `  cap` (typing under `// how we add 44`). 22 lines.
- [02:34] INSERT `19,45 All`: line 18 `  cap++;`, line 19 `  int *tmp = realloc(arr, sizeof(int) * cap)` (no `;` yet; paren match highlighted). 23 lines.
- [02:44] INSERT `25,3 All`: lines 19-23 realloc + `if (!tmp) {` / `free(arr);` / `return 1;` / `}`; cursor on empty line 25. 29 lines.
- [02:54] `-- VISUAL LINE --  3  15,12 All`: lines 13-15 (for/printf/puts) selected. Lines 25-26 now `  arr = tmp;` / `  arr[3] = 44;`. 30 lines.
- [02:55] `3 lines yanked`, `23,1 All`.
- [02:57] `3 more lines`, `30,0-1 Top`: for/printf/puts pasted at lines 27-29 after `arr[3] = 44;`, cursor on line 30 (blank).
- [02:58] `:w` typed on command line (not yet entered). View scrolled to line 4-33; final lines 31 `free(arr);`, 32 `return 0;`, 33 `}`.
- [03:00] Tab B active: 
  ```
  lab $ cc main.c && ./a.out
  11 22 33
  lab $
  ```
  (Tab B contents are consistent with a run of the 21L version, i.e. before the realloc section; frame order only shows the tab at this time.)
- [03:07] Tab A normal: `"main.c" 33L, 514B written`, `33,1 Bot`.
- [03:09] Normal `2,1 Top`, cursor on `#` of line 2 (jumped to top).
- [03:10] INSERT `4,1 Top`: two blank lines inserted after include block; `int main(void) {` now line 6.
- [03:16] INSERT `6,1 Top`: line 4 `/* HARD VERSION */`, 5 blank, cursor line 6, 7 blank, `int main` line 8.
- [03:22] INSERT `7,3 Top`: line 6 `typedef struct {`, cursor line 7 (indented), 8 blank, `int main` at 9.
- [03:23] INSERT `8,3 Top`: line 8 `} ` (typed closing brace, trailing cursor), `int main` at 10.
- [03:38] INSERT `7,3 Top`: line 8 `  int *data; // array data`, line 9 `} Slice;`, cursor on empty line 7.
- [03:46] INSERT `8,7 Top`: line 7 `  size_t len;`, line 8 `  size` (typing), line 9 `  int *data; // array data`, line 10 `} Slice;`.
- [03:51] Normal `8,13 Top`: line 8 `  size_t cap;`. Mouse pointer visible. Struct is `len`, `cap`, `*data`.
- [03:52] Terminal hidden; full-screen wallpaper diagram: Last Supper strip, a table with cells numbered 1..6 (six cells filled, others empty), arrow `len` spanning cells 1-6, long line labelled `cap` spanning wider below.
- [03:56] Same diagram; mouse near cell 4/5.

## File states

### 01:11 (15L, 188B written) fixed-array version
```
1| #include <stdio.h>
2|
3| int main(void) {
4|   int arr[3];
5|
6|   arr[0] = 11;
7|   arr[1] = 22;
8|   arr[2] = 33;
9|
10|  for (size_t i = 0; i < 3; i++)
11|    printf("%d ", arr[i]);
12|  puts("");
13|
14|  return 0;
15| }
```

### 02:11 (21L, 309B written) malloc + cap version
```
1| #include <stdio.h>
2| #include <stdlib.h>
3|
4| int main(void) {
5|   size_t cap = 3;
6|   int *arr = malloc(sizeof(int) * cap);
7|   if (!arr) return 1;
8|
9|   arr[0] = 11;
10|  arr[1] = 22;
11|  arr[2] = 33;
12|
13|  for (size_t i = 0; i < cap; i++)
14|    printf("%d ", arr[i]);
15|  puts("");
16|
17|  // how we add 44
18|
19|  free(arr);
20|  return 0;
21| }
```

### 03:07 (33L, 514B written) realloc grow-by-one
```
1| #include <stdio.h>
2| #include <stdlib.h>
3|
4| int main(void) {
5|   size_t cap = 3;
6|   int *arr = malloc(sizeof(int) * cap);
7|   if (!arr) return 1;
8|
9|   arr[0] = 11;
10|  arr[1] = 22;
11|  arr[2] = 33;
12|
13|  for (size_t i = 0; i < cap; i++)
14|    printf("%d ", arr[i]);
15|  puts("");
16|
17|  // how we add 44
18|  cap++;
19|  int *tmp = realloc(arr, sizeof(int) * cap);
20|  if (!tmp) {
21|    free(arr);
22|    return 1;
23|  }
24|
25|  arr = tmp;
26|  arr[3] = 44;
27|  for (size_t i = 0; i < cap; i++)
28|    printf("%d ", arr[i]);
29|  puts("");
30|
31|  free(arr);
32|  return 0;
33| }
```

### 03:51 end of segment (unsaved; +6 lines at top; window shows through line 30 only)
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
??| (lines 31+ not shown in this frame; presumably unchanged from 03:07 version, shifted by 6)
```

## Shell & outputs

- [00:28] `lab $ e` (stray, no output).
- [01:13] stale scrollback (rehearsal program): `-Wsign-compare` warnings at `main.c:13:24` and `main.c:28:24` (`i < cap`, `size_t` aka `unsigned long` vs `int`), `2 warnings generated.`, then `11 22 33` / `11 22 33 44`. Not from the file being written (that file's loop is `i < 3` with no `{`).
- [01:19] `lab $ cc main.c && ./a.` -> `zsh: no such file or directory: ./a.`; retyped `cc main.c && ./a.out`.
- [02:12] `lab $ cc main.c && ./a.out` typed, awaiting enter.
- [03:00] `lab $ cc main.c && ./a.out` -> `11 22 33`.
- Compile command is always `cc main.c && ./a.out` (macOS clang, no flags).
- Vim messages: `"main.c" [New]` (00:30); `<ain.c" [New] 15L, 188B written` (01:11, 01:22); `"main.c" 21L, 309B written` (02:11, 02:17); `3 lines yanked` (02:55); `3 more lines` (02:57); `"main.c" 33L, 514B written` (03:07).

## Observations

- Build order: scratch note (dynamic array in C / in any language) -> fixed `int arr[3]` with manual assigns and print loop -> `malloc(sizeof(int) * cap)` with `size_t cap = 3`, add `<stdlib.h>`, null-check, `free(arr)` -> "how we add 44": `cap++; realloc` via temp pointer -> start of `/* HARD VERSION */` with `typedef struct { size_t len; size_t cap; int *data; // array data } Slice;`.
- Growth policy so far: grow by exactly one (`cap++`); cap conflated with length (loop bound is `cap`). The len vs cap distinction is introduced by the full-screen diagram at 03:52 (table with 6 filled seats, `len` arrow, wider `cap`).
- Error handling: `if (!arr) return 1;` after malloc; realloc via `tmp`, on failure `free(arr); return 1;`, then `arr = tmp;`.
- Style: 2-space indent, K&R braces, `int *arr` (star on name), `size_t` loop index, braceless single-statement for body, `puts("")` for newline, `printf("%d ", ...)`. Comments casual lowercase (`// how we add 44`, `// array data`); `/* HARD VERSION */` block comment as stage header (appears before the sl_* functions exist; Slice typedef is `typedef struct {...} Slice;`). No `#` shell comments in this segment.
- Editing habits: vim, yank/put to duplicate lines then edit indexes, visual-line yank/put to duplicate print loop, `:w` frequently, switches to tab B for `cc main.c && ./a.out`.
- Prepared (rehearsed) material exists: stale scrollback and the later HARD VERSION label show he has a script.
