# Segment 4 (10:41 - 15:46) forensic transcript

Frames viewed: all 39 in seg4_frames.txt. Tabs: `~/lab — vim main.c` (tab 1), `~/lab — -zsh` (tab 2; its title becomes `~/lab — less ◂ man 3 <topic>` while a man page is open, and stays so after quitting less until the next command).

## Timeline

- [10:41] vim tab, main.c = the finished "int" Slice version (69 lines). Command line shows `:w` (typed, not yet executed). Visible lines 49-69 (sl_print tail + main).
- [10:42] Switched to `-zsh` tab: empty prompt `lab $ `.
- [10:46] Back in vim. Status: `"main.c" 69L, 1194B written`, ruler `65,3  Bot`. (CHECKSUM of int version: 69L/1194B.)
- [10:48] Normal mode, ruler `42,0-1  Bot` (view 42-69). Scrolling up.
- [10:49] `25,1  61%` (view 25-54): sl_append with comments visible.
- [10:50] `12,3  28%`: view 12-41.
- [10:51] `2,3  Top`: top of file; banner still `/* HARD VERSION */` (line 4), `int *data; // array data` (line 9).
- [11:32] INSERT `9,18  Top`. Banner edited to `/* HARD HARD VERSION - Any data type */`. New line 9 is `//` + space (a comment being typed, indented under line 8's comment column); `int *data` replaced by `void *data; // array data` (now line 10). sl_init now at 13.
- [11:49] INSERT `19,3  Top`. Line 9 is now `  size_t elm_size;`. `Slice *sl_init(size_t cap, size_t elm_size) {` (line 13). New blank line 19 (cursor, indented 2) between `sl->cap = cap;` (18) and `sl->data = malloc(sizeof(int) * cap);` (20); about to type the elm_size assignment.
- [12:12] INSERT `12,1  Top`. `sl->elm_size = elm_size;` at line 20; `sl->data = malloc(elm_size * cap);` at 21 (sizeof(int) gone). Blank lines 12-13 (cursor on 12): space opened above sl_init for a new function.
- [12:13] INSERT `13,5  Top`: typed `void` at line 13.
- [12:26] INSERT `15,2  Top`: `void *sl_get(const Slice *sl, size_t index) {` at 13, empty line 14, `}` at 15 (matching `{` highlighted).
- [12:40] INSERT `15,3  Top`: line 14 `  char *data = sl->data;`, cursor on empty line 15.
- [13:05] Normal `46,26  34%` (view 17-46). sl_append still `int val`, `int *tmp = realloc(sl->data, sizeof(int) * sl->cap);` (line 40), `sl->data[sl->len] = val;` (46). sl_get already above sl_init (sl_init at 18).
- [13:06] `52,11  47%` (view 23-52), reviewing sl_free (line 50).
- [13:07] `56,37  56%` (view 27-56), reviewing sl_print (line 55).
- [13:43] Normal `1,1  Top`. sl_get complete: line 15 `  return &data[index * sl->elm_size];`.
- [13:44] INSERT `2,1  Top`: a blank line inserted between the two #include lines (line 2 empty, `#include <stdlib.h>` now line 3, everything below +1). Probably accidental; never seen removed.
- [13:49] Normal `29,5  59%` (view 29-58). sl_append signature now `void sl_append(Slice *sl, void *val) {` (line 35). realloc line (41): `int *tmp = realloc(sl->data, sl->elm_size * sl->cap);` (sizeof(int) removed; `int *tmp` unchanged). `sl->data[sl->len] = val;` at 47, `sl->len++;` 48.
- [13:55] INSERT `48,5  58%`: line 47 `//sl->data[sl->len] = val;` (commented out, no space after `//`); line 48 `//` being typed; `sl->len++;` pushed to 49.
- [14:00] INSERT `49,7  57%`: line 48 `  strcmp(` ; line 49 blank, cursor col 7; `sl->len++;` at 50.
- [14:20] INSERT `50,7  56%`:
  ```
  48    strcmp(
  49        sl_get(sl, len),
  50        |
  ```
- [14:24] Switched to `-zsh` tab:
  ```
  lab $ cc main.c && ./a.out
  11 22 33
  11 22 33 44
  lab $ 
  ```
  (scrollback of an earlier run of the int version; mouse I-beam cursor visible.)
- [14:27] zsh tab title `~/lab — less ◂ man 3 strcmp`: `STRCMP(3)  Library Functions Manual`, NAME "strcmp, strncmp — compare strings", LIBRARY "Standard C Library (libc, -lc)", SYNOPSIS `#include <string.h>` / `int strcmp(const char *s1, const char *s2);` / `int strncmp(const char *s1, const char *s2, size_t n);`; DESCRIPTION "lexicographically compare the null-terminated strings s1 and s2" (strncmp: "characters that appear after a '\0' character are not compared"); RETURN VALUES integer greater than, equal to, or less than 0.
- [14:35] Shell:
  ```
  lab $ cc main.c && ./a.out
  11 22 33
  11 22 33 44
  lab $ man 3 strcmp
  lab $ man 3 
  ```
  (typing next man command).
- [14:36] Title `less ◂ man 3 string`: `STRING(3)`; NAME "index, rindex, stpcpy, strcasecmp, strcat, strchr, strcmp, strcpy, strcspn, strerror, strlen, strncasecmp, strncat, strncmp, strncpy, strpbrk, strrchr, strsep, strspn, strstr, strtok — string specific functions"; SYNOPSIS `#include <strings.h>` ... `#include <string.h>` `char * stpcpy(char *dst, const char *src);`.
- [14:45] Shell: history adds `lab $ man 3 string`; now typing `lab $ man`.
- [14:47] Title `less ◂ man 3 strcpy`: `STRCPY(3)`, NAME "stpcpy, stpncpy, strcpy, strncpy — copy strings"; ADMONITION "YOU SHOULD ALMOST CERTAINLY USE strlcpy() INSTEAD. See "Examples" below."
- [14:53] Shell (tab title still `less ◂ man 3 strcpy`):
  ```
  lab $ cc main.c && ./a.out
  11 22 33
  11 22 33 44
  lab $ man 3 strcmp
  lab $ man 3 string
  lab $ man 3 strcpy
  lab $ 
  ```
- [14:57] Title `less ◂ man 3 memcpy`: `MEMCPY(3)`, "memcpy — copy memory area"; SYNOPSIS `#include <string.h>` / `void * memcpy(void *restrict dst, const void *restrict src, size_t n);` DESCRIPTION "copies n bytes from memory area src to memory area dst. If dst and src overlap, behavior is undefined. Applications in which dst and src might overlap should use memmove(3) instead."; RETURN VALUES "returns the original value of dst."; SEE ALSO bcopy(3), memccpy(3), memmove(3), strcpy(3), wmemcpy(3); STANDARDS "conforms to ISO/IEC 9899:1990 ("ISO C90")." (The `man 3 memcpy` command line itself not seen; inferred from title.)
- [15:13] Back in vim, INSERT `50,7  56%`; tab 2 title `~/lab — less ◂ man 3 memcpy`. Code still `strcmp(` / `sl_get(sl, len),` / empty line 50. Mouse I-beam over line 42.
- [15:20] INSERT `50,7  54%`: `strcmp(` replaced with `memcpy(`; lines 50 and 51 empty; `sl->len++;` at 52.
- [15:24] INSERT `51,7  53%`: line 50 `        val,` added.
- [15:30] INSERT `52,7  52%`: line 51 `        sl->elm_size` added.
- [15:36] Normal `34,0-1  45%` (view 25-54): memcpy call closed with `  );` at line 52 (see File states E).
- [15:37] Normal `53,0-1  98%` (view 53-82): sl_free (57-60), sl_print (62-66, int-printing loop intact), main (68-) still int-based.
- [15:42] VISUAL LINE `63,1  98%`, count `1`: line 63 (`for (size_t i ...`) highlighted.
- [15:43] Normal `63,1  Bot`, message `3 fewer lines`: loop lines (for / printf / puts) deleted from sl_print; sl_print now `{` `}` only (62-63).
- [15:46] Normal `75,3  98%`, message `3 more lines`: the 3 lines pasted into main after `sl_print(sl);` (see File states F). Segment ends here; no further save/compile in this segment.

## File states

Notation: `N| code`. `??` = not seen.

### State A (10:46, saved: `"main.c" 69L, 1194B written`) — int Slice
Lines 1-11 assembled from the 10:51 frame, 12-69 from 10:41-10:50.
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
27| void sl_append(Slice *sl, int val) {
28|   // on max size table
29|   // so, we need move to big table
30|   if (sl->len == sl->cap) {
31|     // you don't want call resize every time call append
32|     sl->cap = sl->cap * 2;
33|     int *tmp = realloc(sl->data, sizeof(int) * sl->cap);
34|     // add err handle
35|
36|     sl->data = tmp;
37|   }
38|
39|   sl->data[sl->len] = val;
40|   sl->len++;
41| }
42|
43| void sl_free(Slice *sl) {
44|   free(sl->data);
45|   free(sl);
46| }
47|
48| void sl_print(const Slice *sl) {
49|   for (size_t i = 0; i < sl->len; i++)
50|     printf("%d ", sl->data[i]);
51|   puts("");
52| }
53|
54| int main(void) {
55|   Slice *sl = sl_init(3);
56|   if (!sl) return 1;
57|
58|   sl_append(sl, 11);
59|   sl_append(sl, 22);
60|   sl_append(sl, 33);
61|
62|   sl_print(sl);
63|
64|   sl_append(sl, 44);
65|   sl_print(sl);
66|
67|   sl_free(sl);
68|   return 0;
69| }
```

### State B (11:32-12:40, mid-edit)
11:32 (INSERT, top):
```
 4| /* HARD HARD VERSION - Any data type */
 ...
 7|   size_t len; // such as table have 6 person
 8|   size_t cap; // such as table can have max 12 person
 9|                // |
10|   void *data; // array data
11| } Slice;
```
12:12:
```
 9|   size_t elm_size;
10|   void *data; // array data
11| } Slice;
12| |(cursor, INSERT)
13|
14| Slice *sl_init(size_t cap, size_t elm_size) {
...
20|   sl->elm_size = elm_size;
21|   sl->data = malloc(elm_size * cap);
...
30| void sl_append(Slice *sl, int val) {
```
12:26:
```
13| void *sl_get(const Slice *sl, size_t index) {
14|
15| }
16|
17| Slice *sl_init(size_t cap, size_t elm_size) {
```

### State C (13:43, Normal `1,1  Top`) — sl_get done (top 30 lines)
```
 1| #include <stdio.h>
 2| #include <stdlib.h>
 3|
 4| /* HARD HARD VERSION - Any data type */
 5|
 6| typedef struct {
 7|   size_t len; // such as table have 6 person
 8|   size_t cap; // such as table can have max 12 person
 9|   size_t elm_size;
10|   void *data; // array data
11| } Slice;
12|
13| void *sl_get(const Slice *sl, size_t index) {
14|   char *data = sl->data;
15|   return &data[index * sl->elm_size];
16| }
17|
18| Slice *sl_init(size_t cap, size_t elm_size) {
19|   Slice *sl = malloc(sizeof(Slice));
20|   if (!sl) return NULL;
21|
22|   sl->len = 0;
23|   sl->cap = cap;
24|   sl->elm_size = elm_size;
25|   sl->data = malloc(elm_size * cap);
26|   if (!sl->data) {
27|     free(sl);
28|     return NULL;
29|   }
30|
```
Lines 31-56 as of 13:05-13:07 (before the 13:44 blank-line shift): 31 `  return sl;` 32 `}` 33 blank 34 `void sl_append(Slice *sl, int val) {` 35-36 comments, 37 `if (sl->len == sl->cap) {` 38 comment 39 `sl->cap = sl->cap * 2;` 40 `int *tmp = realloc(sl->data, sizeof(int) * sl->cap);` 41 `// add err handle` 42 blank 43 `sl->data = tmp;` 44 `}` 45 blank 46 `sl->data[sl->len] = val;` 47 `sl->len++;` 48 `}` 49 blank 50 `void sl_free(Slice *sl) {` 51 `free(sl->data);` 52 `free(sl);` 53 `}` 54 blank 55 `void sl_print(const Slice *sl) {` 56 `for (size_t i = 0; i < sl->len; i++)`.

### State D (13:44 onward) — stray blank line 2, all lines below +1
```
 1| #include <stdio.h>
 2|
 3| #include <stdlib.h>
 4|
 5| /* HARD HARD VERSION - Any data type */
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
```
The blank line was never seen removed through 15:46 (sl_init `sl->elm_size = elm_size;` is at 25 in 15:36 frame vs 24 at 13:43).

### State E (15:36, Normal `34,0-1  45%`) — lines 25-54 (most complete sl_append)
```
25|   sl->elm_size = elm_size;
26|   sl->data = malloc(elm_size * cap);
27|   if (!sl->data) {
28|     free(sl);
29|     return NULL;
30|   }
31|
32|   return sl;
33| }
34| |(cursor on blank line)
35| void sl_append(Slice *sl, void *val) {
36|   // on max size table
37|   // so, we need move to big table
38|   if (sl->len == sl->cap) {
39|     // you don't want call resize every time call append
40|     sl->cap = sl->cap * 2;
41|     int *tmp = realloc(sl->data, sl->elm_size * sl->cap);
42|     // add err handle
43|
44|     sl->data = tmp;
45|   }
46|
47|   //sl->data[sl->len] = val;
48|   memcpy(
49|       sl_get(sl, len),
50|       val,
51|       sl->elm_size
52|   );
53|
54|   sl->len++;
```
Note line 34 is shown blank with cursor at `34,0-1`; line 33 `}` directly followed by line 34 blank then `void sl_append` at 35. Indentation of memcpy args: 6 spaces (args), `);` at 2 spaces. `len` (not `sl->len`) in `sl_get(sl, len)`. No `#include <string.h>` seen anywhere.

### State F (END, 15:37 -> 15:46)
15:37 (lines 53-82, before the move):
```
53|
54|   sl->len++;
55| }
56|
57| void sl_free(Slice *sl) {
58|   free(sl->data);
59|   free(sl);
60| }
61|
62| void sl_print(const Slice *sl) {
63|   for (size_t i = 0; i < sl->len; i++)
64|     printf("%d ", sl->data[i]);
65|   puts("");
66| }
67|
68| int main(void) {
69|   Slice *sl = sl_init(3);
70|   if (!sl) return 1;
71|
72|   sl_append(sl, 11);
73|   sl_append(sl, 22);
74|   sl_append(sl, 33);
75|
76|   sl_print(sl);
77|
78|   sl_append(sl, 44);
79|   sl_print(sl);
80|
81|   sl_free(sl);
82|   return 0;
```
15:43 (`3 fewer lines`, lines 53-80):
```
62| void sl_print(const Slice *sl) {
63| }
64|
65| int main(void) {
66|   Slice *sl = sl_init(3);
67|   if (!sl) return 1;
68|
69|   sl_append(sl, 11);
70|   sl_append(sl, 22);
71|   sl_append(sl, 33);
72|
73|   sl_print(sl);
74|
75|   sl_append(sl, 44);
76|   sl_print(sl);
77|
78|   sl_free(sl);
79|   return 0;
80| }
```
15:46 (`3 more lines`, END of segment; view 53-82):
```
53|
54|   sl->len++;
55| }
56|
57| void sl_free(Slice *sl) {
58|   free(sl->data);
59|   free(sl);
60| }
61|
62| void sl_print(const Slice *sl) {
63| }
64|
65| int main(void) {
66|   Slice *sl = sl_init(3);
67|   if (!sl) return 1;
68|
69|   sl_append(sl, 11);
70|   sl_append(sl, 22);
71|   sl_append(sl, 33);
72|
73|   sl_print(sl);
74|   for (size_t i = 0; i < sl->len; i++)
75|     printf("%d ", sl->data[i]);
76|   puts("");
77|
78|   sl_append(sl, 44);
79|   sl_print(sl);
80|
81|   sl_free(sl);
82|   return 0;
   ?? (83 `}` presumably)
```
main still calls `sl_init(3)` (one arg) and `sl_append(sl, 11)` (int literals), i.e. not yet adapted to the generic API at segment end.

## Shell & outputs
- [10:42] `lab $ ` (empty, zsh tab). Prompt format `lab $ `.
- [14:24]
```
lab $ cc main.c && ./a.out
11 22 33
11 22 33 44
lab $ 
```
(no warnings; this is output of the int version, run before the segment; the run itself was not in this segment's frames.)
- [14:27] `man 3 strcmp` -> STRCMP(3) (less prompt `:`).
- [14:35] `lab $ man 3 strcmp` in history; typing `lab $ man 3 `.
- [14:36] `man 3 string` -> STRING(3).
- [14:45] `lab $ man 3 string` in history; typing `lab $ man`.
- [14:47] `man 3 strcpy` -> STRCPY(3).
- [14:53] history `man 3 strcmp`, `man 3 string`, `man 3 strcpy`; empty prompt.
- [14:57] `man 3 memcpy` (via tab title) -> MEMCPY(3).
- No compile run in this segment other than scrollback; no `#` shell comments.

## Observations
- Build order: (1) save int version baseline (`"main.c" 69L, 1194B written`); (2) retitle banner to `/* HARD HARD VERSION - Any data type */`; (3) struct: `int *data` -> `void *data`, add `size_t elm_size;` (started as a `//` comment line, then replaced); (4) sl_init gets `size_t elm_size` param, stores it, allocates `malloc(elm_size * cap)`; (5) add `void *sl_get(const Slice *sl, size_t index)` above sl_init: `char *data = sl->data; return &data[index * sl->elm_size];` (byte-pointer arithmetic); (6) sl_append takes `void *val`; realloc uses `sl->elm_size * sl->cap` (still `int *tmp`); old assignment commented out; replaced with `memcpy(sl_get(sl, len), val, sl->elm_size);` after a detour through `strcmp(` and the man pages strcmp, string, strcpy, memcpy; (7) begins decoupling printing: cuts the print loop out of sl_print and pastes it into main (sl_print left empty).
- Design: growth policy unchanged (cap *= 2 when len == cap, comment "you don't want call resize every time call append"); error handling still a placeholder (`// add err handle`, tmp never checked); sl_init handles malloc failure properly and frees struct on data failure; caller checks `if (!sl) return 1;`. Ownership: Slice heap-allocated; sl_free frees data then struct. Elements are copied by value (memcpy) into slice storage.
- Style: 2-space indent, K&R braces, size_t, lowercase informal analogy comments with non-native phrasing ("on max size table", "so, we need move to big table"), `elm_size` naming, plain vim with line numbers + ruler; commenting out old code by prefixing `//` rather than deleting it; learning via `man 3 <fn>` lookups in a second tab (tab title shows `less ◂ man 3 X`).
- Oddities/bugs for later: stray blank line 2 between includes; `len` instead of `sl->len` in memcpy call; no `#include <string.h>`; `int *tmp` retained; the loop pasted into main (probably meant to paste elsewhere or intentionally moving printing out of the library).
- Webcam: presenter holding a cigarette in many frames; ignore.
