# Segment 5 (15:46 - 19:33) transcript

Tabs: `~/lab — vim main.c` (tab 1). Tab 2 is `~/lab — less ◂ man 3 memcpy` in the 15:46-16:08 frames (while man is open), and `~/lab — -zsh` from 16:56 on. No tab 2 title change is visible between 16:09 and 16:55 in frames (tab shows less/man at 16:55).

## Timeline
- [15:46] vim main.c, NORMAL, ruler `75,3  98%`, msg `3 more lines`. main() at this point: line 65 `int main(void) {`, 66 `Slice *sl = sl_init(3);`, 67 `if (!sl) return 1;`, 69-71 `sl_append(sl, 11/22/33);`, 73 `sl_print(sl);`, 74-75 for-loop printing `printf("%d ", sl->data[i]);`, 76 `puts("");`, 78 `sl_append(sl, 44);`, 79 `sl_print(sl);`, 81 `sl_free(sl);`, 82 `return 0;`. sl_print is at 62-63 with an empty body: `void sl_print(const Slice *sl) {` / `}`.
- [15:47] `-- VISUAL LINE --` (shows `3`), ruler `80,0-1  98%`, selecting lines 78-80 (`sl_append(sl, 44);`, `sl_print(sl);`, blank).
- [15:49] Deleted: msg `3 fewer lines`, ruler `77,0-1  Bot`. File ends line 80 `}`; main at 62.. shifted (sl_free at 78, return 0 at 79).
- [15:51] msg `3 fewer lines`, ruler `62,1  Bot`. The empty `sl_print` function (old lines 62-63 + blank) is now gone: `int main(void) {` is at line 62, `Slice *sl = sl_init(3);` at 63. File 77 lines.
- [16:08] INSERT, ruler `65,3  Bot`. Line 63 now `Slice *sl = sl_init(3, sizeof(int));`. Line 65 new indented blank (cursor), 66 blank; 67-69 `sl_append(sl, 11/22/33);`; file 78 lines. Webcam: speaker looking at camera (not smoking now).
- [16:09] INSERT, ruler `66,7  Bot`, line 66 `  int ` being typed; file 79 lines.
- [16:51] NORMAL. 66 `  int a = 11, b = 22, c = 33;`; 68-70 `sl_append(sl, &a);` `&b` `&c`; 72 `sl_print(sl);`; 73-74 `for (size_t i = 0; i < sl->len; i++)` / `printf("%d ", *(int *)sl_get(sl, i));`. Msg `"main.c" 79L, 1462B written`, ruler `74,26  Bot`.
- [16:55] Tab 2: `man 3 memcpy` in less (`:` prompt, whole page visible; DESCRIPTION paragraph highlighted). Text in Shell & outputs.
- [16:56] Tab 2 `~/lab — -zsh`, empty prompt `lab $ ` (man closed).
- [16:59] zsh: `cc main.c && ./a.out` -> 3 errors (see below).
- [17:02] vim tab: still `"main.c" 79L, 1462B written`, `74,26  Bot`. Tab 2 title `~/lab — -zsh`.
- [17:04] zsh tab again (same 3 errors on screen). [17:06] vim again (same screen as 17:02).
- [17:08] vim: `49,7  97%`, top line 49 `      sl_get(sl, len),` (6-space indent), 50 `      val,`, 51 `      sl->elm_size`, 52 `  );`.
- [17:09] `42,7  83%`: 42 `    //add err handle` (cursor), 44 `    sl->data = tmp;`, 45 `  }`, 47 `  //sl->data[sl->len] = val;`, 48 `  memcpy(`, 49 `sl_get(sl, len),` ...
- [17:10] `49,7  79%`: 40 `    sl->cap = sl->cap * 2;`, 41 `    int *tmp = realloc(sl->data, sl->elm_size * sl->cap);`, 42 `    // add err handle`, 44 `    sl->data = tmp;`.
- [17:16] zsh tab: same 3 errors displayed. [17:17] zsh: re-run shows 1 error only (sl_print undeclared at 72:3).
- [17:21] vim: line 49 now `      sl_get(sl, sl->len),`. Msg `"main.c" 79L, 1466B written`, ruler `49,21  79%`. (Mouse I-beam visible.)
- [17:22] vim ruler `65,0-1  Bot`, view starts at 65. Line 72 `sl_print(sl);` still present here (65 blank, 66 `int a = ...`, ..., 72 `sl_print(sl);`, 73 for, 74 printf, 75 puts, 77 sl_free, 78 return, 79 `}`).
- [17:24] `:w` typed at command line; the `sl_print(sl);` line has been deleted: file is 78 lines (72 `for (...)`, 73 printf, 74 puts, 76 sl_free, 77 return 0, 78 `}`).
- [17:26] zsh: `lab $ cc main.c && ./a.out` typed, not yet run.
- [17:31] vim: msg `"main.c" 78L, 1450B written`, ruler `72,3  Bot`.
- [17:32] vim: view from 59, ruler `59,3  Bot`; 78 lines (62 main, 63 sl_init(3, sizeof(int)), 66 `int a = 11, b = 22, c = 33;`, 68-70 appends &a/&b/&c, 72 for, 78 `}`).
- [17:35] vim: ruler `66,3  Bot`, file 76 lines: the `int a = 11, b = 22, c = 33;` line and one blank removed; lines now 66 `sl_append(sl, &a);` 67 `&b` 68 `&c` (a,b,c undeclared at this instant), 70 for, 71 printf `%d` `*(int *)sl_get(sl, i)`, 72 `puts("");`, 74 `sl_free(sl);`, 75 `return 0;`, 76 `}`.
- [17:53] zsh: `cc main.c && ./a.out` -> `11 22 33` (output highlighted).
- [17:59] vim: 66-68 now compound literals `sl_append(sl, &(int){11});` / `{22}` / `{33}`. Msg `"main.c" 76L, 1443B written`, ruler `68,26  Bot`.
- [18:14] INSERT, `68,20`: line 63 `Slice *sl = sl_init(3, sizeof(float));`; 66 `sl_append(sl, &(float){1.1});`, 67 `&(float){2.2}`, 68 mid-edit `&(f){3.3}`; line 71 still `"%d "` / `*(int *)`.
- [18:21] NORMAL, 68 `&(float){3.3}`; 71 `printf("%f ", *(float *)sl_get(sl, i));`. Msg `"main.c" 76L, 1456B written`, `71,14  Bot`.
- [18:22] zsh tab shows the older output `11 22 33` (screen before re-run).
- [18:27] vim `67,14  Bot`, msg still `"main.c" 76L, 1456B written`, same float file.
- [18:36] INSERT `66,29`: line 63 `sl_init(3, sizeof(char *));`; 66 mid-edit `sl_append(sl, &(float){"He});` (string highlighting in red); 67-68 still floats; 71 still `%f`/`float *`.
- [18:42] NORMAL `68,31`: 66 `&(float){"Hello"}`, 67 `&(float){"World"}`, 68 `&(float){3.3}`; 63 `sizeof(char *)`.
- [19:01] INSERT `71,21`: 66 `sl_append(sl, &(char *){"Hello"});`, 67 `&(char *){"World"}`, 68 `&(char *){"Hayyoth"}`; line 71 mid-edit `printf("%f ", *( *)sl_get(sl, i));` (type text removed).
- [19:09] zsh: `cc main.c && ./a.out` -> `1.100000 2.200000 3.300000` (this is the float version's run; shown at 19:09).
- [19:18] vim: line 71 `printf("%s ", *(char **)sl_get(sl, i));`. Msg `"main.c" 76L, 1474B written`, ruler `71,14  Bot`.
- [19:19] Jumped to top: ruler `1,1  Top`, msg still `"main.c" 76L, 1474B written`. Line 5 `/* HARD HARD VERSION - Any data type */`.
- [19:22] INSERT `5,6  Top`: line 5 `/* HAHARD HARD VERSION - Any data type */` (typing).
- [19:26] `-- VISUAL --` (shows `11`), `5,39  Top`; line 5 `/* HARD HARD HARD VERSION - Any data type */`, selection on `Any data ty`.
- [19:33] NORMAL `5,44  Top`: line 5 `/* HARD HARD HARD VERSION - You implement it */`. macOS menu bar now visible (Terminal Shell Edit View Window Help; clock `Wed Oct 21 10:35 PM`). This is the last frame; no save message or run after this edit, no other closing text visible.

## File states

### main.c at the end (19:19-19:33 top lines; 40-76 from 17:10-19:18 frames). Lines 31-39 not seen.
```
 1| #include <stdio.h>
 2| #include <string.h>
 3| #include <stdlib.h>
 4|
 5| /* HARD HARD HARD VERSION - You implement it */
 6|
 7| typedef struct {
 8|   size_t len; // such as table have 6 person
 9|   size_t cap; // such as table can have max 12 person
10|   size_t elm_size;
11|   void *data; // array data
12| } Slice;
13|
14| void *sl_get(const Slice *sl, size_t index) {
15|   char *data = sl->data;
16|   return &data[index * sl->elm_size];
17| }
18|
19| Slice *sl_init(size_t cap, size_t elm_size) {
20|   Slice *sl = malloc(sizeof(Slice));
21|   if (!sl) return NULL;
22|
23|   sl->len = 0;
24|   sl->cap = cap;
25|   sl->elm_size = elm_size;
26|   sl->data = malloc(elm_size * cap);
27|   if (!sl->data) {
28|     free(sl);
29|     return NULL;
30|   }
31| ??
..| ??   (31-39 unseen: rest of sl_init, sl_append signature, `if (sl->len == sl->cap) {`)
39| ??
40|     sl->cap = sl->cap * 2;
41|     int *tmp = realloc(sl->data, sl->elm_size * sl->cap);
42|     // add err handle
43|
44|     sl->data = tmp;
45|   }
46|
47|   //sl->data[sl->len] = val;
48|   memcpy(
49|       sl_get(sl, sl->len),
50|       val,
51|       sl->elm_size
52|   );
53|
54|   sl->len++;
55| }
56|
57| void sl_free(Slice *sl) {
58|   free(sl->data);
59|   free(sl);
60| }
61|
62| int main(void) {
63|   Slice *sl = sl_init(3, sizeof(char *));
64|   if (!sl) return 1;
65|
66|   sl_append(sl, &(char *){"Hello"});
67|   sl_append(sl, &(char *){"World"});
68|   sl_append(sl, &(char *){"Hayyoth"});
69|
70|   for (size_t i = 0; i < sl->len; i++)
71|     printf("%s ", *(char **)sl_get(sl, i));
72|   puts("");
73|
74|   sl_free(sl);
75|   return 0;
76| }
```
Notes: indentation 2 spaces (4 inside the `if`, memcpy args 6 spaces and `);` at 2). Line 5 was last edited at 19:33 and no `written` message is seen after it (the last save shown, 19:18, predates the banner edit, so saved/unsaved status of line 5 is unknown). sl_print no longer exists. Line 41 retains `int *tmp` (not `void *`).

### Intermediate main() versions
15:46 (int-only, 82+ lines): see timeline.

16:51 (generic int, 79L, 1462B):
```
62| int main(void) {
63|   Slice *sl = sl_init(3, sizeof(int));
64|   if (!sl) return 1;
65|
66|   int a = 11, b = 22, c = 33;
67|
68|   sl_append(sl, &a);
69|   sl_append(sl, &b);
70|   sl_append(sl, &c);
71|
72|   sl_print(sl);
73|   for (size_t i = 0; i < sl->len; i++)
74|     printf("%d ", *(int *)sl_get(sl, i));
75|   puts("");
76|
77|   sl_free(sl);
78|   return 0;
79| }
```
17:59 (76L, 1443B): lines 66-68 `sl_append(sl, &(int){11});` `{22}` `{33}`.
18:21 (76L, 1456B): `sl_init(3, sizeof(float))`, `&(float){1.1}`/`{2.2}`/`{3.3}`, `printf("%f ", *(float *)sl_get(sl, i));`.
19:18 (76L, 1474B): the final char * version above.

## Shell & outputs
- [16:59] tab `~/lab — -zsh`:
```
lab $ cc main.c && ./a.out
main.c:49:18: error: use of undeclared identifier 'len'
   49 |       sl_get(sl, len),
      |                  ^~~
main.c:49:18: error: use of undeclared identifier 'len'
   49 |       sl_get(sl, len),
      |                  ^~~
main.c:72:3: error: call to undeclared function 'sl_print'; ISO C99
      and later do not support implicit function declarations
      [-Wimplicit-function-declaration]
   72 |   sl_print(sl);
      |   ^
3 errors generated.
lab $
```
- [17:17] after fixing `len` (re-run):
```
lab $ cc main.c && ./a.out
main.c:72:3: error: call to undeclared function 'sl_print'; ISO C99
      and later do not support implicit function declarations
      [-Wimplicit-function-declaration]
   72 |   sl_print(sl);
      |   ^
1 error generated.
lab $
```
- [17:53] `lab $ cc main.c && ./a.out` -> `11 22 33`
- [19:09] `lab $ cc main.c && ./a.out` -> `1.100000 2.200000 3.300000`
- No run of the final char * ("Hello World Hayyoth") version appears in my frame list.

man 3 memcpy at 16:55 (less): header `MEMCPY(3)   Library Functions Manual   MEMCPY(3)`; NAME `memcpy — copy memory area`; LIBRARY `Standard C Library (libc, -lc)`; SYNOPSIS `#include <string.h>` / `void * memcpy(void *restrict dst, const void *restrict src, size_t n);`; DESCRIPTION (highlighted) "The memcpy() function copies n bytes from memory area src to memory area dst.  If dst and src overlap, behavior is undefined.  Applications in which dst and src might overlap should use memmove(3) instead."; RETURN VALUES "The memcpy() function returns the original value of dst."; SEE ALSO `bcopy(3), memccpy(3), memmove(3), strcpy(3), wmemcpy(3)`; STANDARDS "The memcpy() function conforms to ISO/IEC 9899:1990 ("ISO C90")."

Vim save messages (checksums): 16:51 `"main.c" 79L, 1462B written`; 17:21 `"main.c" 79L, 1466B written`; 17:31 `"main.c" 78L, 1450B written`; 17:59 `"main.c" 76L, 1443B written`; 18:21 `"main.c" 76L, 1456B written`; 19:18 `"main.c" 76L, 1474B written`.

## Observations
- Compiler-driven loop: the two errors (`len` undeclared, `sl_print` undeclared) were fixed by editing `len` -> `sl->len` and by deleting the `sl_print(sl);` call (the empty sl_print function was removed earlier at 15:49-15:51). Printing is done by a for loop in main with a cast, `*(T *)sl_get(sl, i)`.
- Dropped the `sl_append(sl, 44)` / extra sl_print demo lines (15:47-15:49).
- Generic call style progression: named locals with `&a` -> C99 compound literals `&(int){11}` -> `float` -> `char *` strings ("Hello", "World", "Hayyoth" = channel name) with `*(char **)` cast. Mid-edit states had mismatched types (`&(float){"Hello"}`) before being corrected.
- Used man 3 memcpy to check the semantics (overlap undefined, memmove alternative) before relying on it for the generic element copy.
- Informal comments in struct: "such as table have 6 person", "such as table can have max 12 person", "array data". Leftover `// add err handle` (realloc result unchecked) and `int *tmp` in the generic path.
- Closing text (verbatim): line 5 of main.c becomes `/* HARD HARD HARD VERSION - You implement it */` (edited from `/* HARD HARD VERSION - Any data type */`). It is an exercise prompt to the viewer. No other closing message is visible; video ends at 19:33 with vim in NORMAL at `5,44  Top`.
