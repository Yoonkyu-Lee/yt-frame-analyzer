# Segment 3 (06:56 - 10:41) transcript

Tabs throughout: `~/lab — vim main.c` (active) and `~/lab — -zsh`. Terminal window narrow (right edge ~858px) until 09:15, widened by 09:17 (~944px). No man pages in this segment. Lines 1-7 of main.c never visible in this segment (line 8 is first seen at 07:23).

## Timeline
- [06:56] vim, normal mode, 33,3, 59%. sl_print is still the OLD broken form: `for (size_t i = 0; i < cap; i++)` / `printf("%d ", arr[i]);` / `puts("");` with NO closing `}` (line 36 blank, `int main(void) {` at 37). main: `Slice *sl = sl_init(3);` `if (!sl) return 1;` then `arr[0] = 11; arr[1] = 22; arr[2] = 33;` then raw-array block `// how we add 44`, `cap++;`, `int *tmp = realloc(arr, sizeof(int) * cap);`, `if (!tmp) {`.
- [07:14] INSERT, 36,3, 57%. sl_print fixed to `for (size_t i = 0; i < sl->len; i++)` / `printf("%d ", sl->data[i]);` / `puts("");`; cursor on blank line 36 (about to type `}`). main now at line 38.
- [07:21] normal 16,1, 45%. sl_print closed with `}` (line 36). main still has `arr[0] = 11;` etc. (lines 42-44).
- [07:22] normal 9,1, 24%. Shows line 9 `int *data; // array data`, 10 `} Slice;`.
- [07:23] normal 17,1, 21%. Line 8: `size_t cap; // such as table can have max 12 person`.
- [07:24] INSERT 26,1, 20%: blank lines 26-27 opened between sl_init and sl_free (sl_free now at 28).
- [07:25] INSERT 27,5: typing `void` on line 27.
- [07:38] INSERT 28,3, 19%: `void sl_append(Slice *sl, int val) {` at line 27; cursor on blank line 28. No closing brace yet (sl_free at 30).
- [07:52] INSERT 28,3, 18%: line 29 `  if (sl->len == sl->cap)` typed (no brace yet), line 28 still empty.
- [07:58] INSERT 29,6: line 28 `// on max size table`, line 29 `//` (typing), if now line 30.
- [08:09] INSERT 32,4, 17%: line 29 `// so, we need move to big table`; 30 `if (sl->len == sl->cap) {`; 31 blank; 32 `}` (brace match highlighted).
- [08:28] INSERT 31,7: line 31 `//` typing; line 32 `sl->cap = sl->cap * 2;` already present; 33 `}`.
- [08:50] INSERT 33,5, 16%: line 31 `// you don't want call resize every time call append`; 32 `sl->cap = sl->cap * 2;`; 33 blank cursor; 34 `}`.
- [09:15] INSERT 34,5: line 33 `int *tmp = realloc(sl->data, sizeof(int) * sl->cap);`. Mouse pointer visible.
- [09:17] Terminal window widened; same content; I-beam over line 29.
- [09:34] normal mode, 37,3, 15%: line 34 `// add err handle`, 35 blank, 36 `sl->data = tmp;`, 37 `}`.
- [09:35] normal 38,0-1, 28%, scrolled (top line 14). Line 38 blank, sl_free at 39.
- [09:41] INSERT 38,3: blank line 38 opened, blank 39, sl_free at 40.
- [09:47] INSERT 39,11: `sl->data` typed on line 39.
- [09:54] INSERT 40,6: line 39 `sl->data[sl->len] = val;`, line 40 `sl-` typing.
- [09:58] normal 41,1, 26%: line 40 `sl->len++;`, 41 `}` (matching `{` at line 27 highlighted).
- [09:59] normal 46,1, 32%: cursor on `}` line 46 (end of sl_free).
- [10:00] normal 61,0-1, 63%: main lines 54-60 as before (arr[0..2] assignments at 58-60), 61 blank.
- [10:01] normal 67,1, 77%: old block visible at 63-68: `// how we add 44`, `cap++;`, `int *tmp = realloc(arr, sizeof(int) * cap);`, `if (!tmp) {`, `free(arr);`, `return 1;`.
- [10:03] VISUAL LINE (2 lines), 59,1: lines 58-59 highlighted (selection extending over arr[0..2]).
- [10:04] normal 57,0-1, 82%, message `3 fewer lines` (arr[0..2] deleted). 57-59 blank, `// how we add 44` at 60.
- [10:05] INSERT 58,3, 80%: typing on blank line 58.
- [10:12] normal 59,3, 79%: lines 58-59 `sl_append(sl, 11);` x2 (second to be edited).
- [10:13] normal 59,3, 77%: three lines `sl_append(sl, 11);` (58-60).
- [10:18] normal 75,11, 91%: now `sl_append(sl, 11);` `sl_append(sl, 22);` `sl_append(sl, 33);`. Old block lines 63-75 still there: `// how we add 44` / `cap++;` / `int *tmp = realloc(arr, sizeof(int) * cap);` / `if (!tmp) {` / `free(arr);` / `return 1;` / `}` / blank / `arr = tmp;` (71) / `arr[3] = 44;` (72) / `for (size_t i = 0; i < cap; i++)` / `printf("%d ", arr[i]);` / `puts("");` (75).
- [10:19] normal 63,18, 97%: cursor on `44` of comment line 63; 76 blank, 77 `sl_free(sl);`, 78 `return 0;`.
- [10:20] VISUAL LINE, 9 lines (63-71), cursor 71,13.
- [10:21] VISUAL LINE, 14 lines (63-76), cursor 76,0-1.
- [10:22] normal 61,0-1, `Bot`, message `14 fewer lines`: old block deleted. main: 58-60 sl_append x3, 61 blank (cursor), 62 blank, 63 `sl_free(sl);`, 64 `return 0;`, 65 `}`; tildes after.
- [10:26] normal 62,15 Bot, message `"main.c" 66L, 1156B written` (CHECKSUM). Line 62 `sl_print(sl);` added; 63 blank; 64 `sl_free(sl);`; 65 `return 0;`; 66 `}`.
- [10:28] Switched to `~/lab — -zsh` tab. Prompt text `lab $ cc main.c && ./a.out` typed, cursor at end; Enter not captured in the frames; no output seen.
- [10:33] Back on vim tab, same state, message `"main.c" 66L, 1156B written` still showing; I-beam over line 55.
- [10:34] INSERT 64,4, Bot: new lines inserted after line 62; line 64 has `s` typed; sl_free now 66, return 0 at 67, `}` 68.
- [10:41] command-line `:w` typed (not yet entered). main: 62 `sl_print(sl);`, 63 blank, 64 `sl_append(sl, 44);`, 65 `sl_print(sl);`, 66 blank, 67 `sl_free(sl);`, 68 `return 0;`, 69 `}`.

## File states

### main.c at 06:56 (start of segment; print function broken, only lines seen)
```
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
```
(no closing `}` for sl_print at this moment; `arr`/`cap` in main/print are stale from the pre-Slice version, would not compile.)

### main.c at 09:58 (sl_append complete; main still stale)
```
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
58|   arr[0] = 11;
59|   arr[1] = 22;
60|   arr[2] = 33;
61|
62|
63|   // how we add 44
64|   cap++;
65|   int *tmp = realloc(arr, sizeof(int) * cap);
66|   if (!tmp) {
67|     free(arr);
68|     return 1;
69|   }
70|
71|   arr = tmp;
72|   arr[3] = 44;
73|   for (size_t i = 0; i < cap; i++)
74|     printf("%d ", arr[i]);
75|   puts("");
76|
77|   sl_free(sl);
78|   return 0;
   ?? (79 `}` not seen)
```

### main.c at 10:41 (END of segment; lines 1-7 not seen = ??)
```
 1| ??
 7| ??
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
Intermediate (10:26, saved as 66L): identical but lines 64-65 absent (`sl_free(sl);` at 64, `return 0;` 65, `}` 66).

## Shell & outputs
- [10:28] zsh tab, prompt `lab $`, typed `cc main.c && ./a.out` (Enter not captured). No compiler output or program output in this segment.
- vim messages: [10:04] `3 fewer lines`; [10:22] `14 fewer lines`; [10:26] and [10:33] `"main.c" 66L, 1156B written`; [10:41] `:w` pending.

## Observations
- Build order: fix sl_print to use sl->len/sl->data and close its brace; add sl_append between sl_init and sl_free (condition first, then comments, brace, `cap * 2`, realloc into `tmp`, `sl->data = tmp`, then store `sl->data[sl->len] = val; sl->len++;`); convert main from the stale raw-array code (arr[...] and the manual "how we add 44" realloc block) to sl_append calls; delete the 14-line manual block; add sl_print(sl); then add `sl_append(sl, 44)` + print to demonstrate growth.
- Growth policy: doubling via `sl->cap = sl->cap * 2` (not `*=`); initial cap 3 so 44 forces 3 -> 6. cap is updated BEFORE realloc succeeds, and error handling is only a TODO comment `// add err handle` with a blank line 35 (tmp assigned to data unchecked at this point) -- inconsistent state if realloc fails. The manual block in main had `if (!tmp) { free(arr); return 1; }` which is the pattern he'll presumably port.
- Ownership/error handling: sl_init returns NULL and frees the struct if the data malloc fails (two-stage cleanup); sl_free frees data then struct; main checks `if (!sl) return 1;`. sl_append returns void (no error propagation).
- Style: 2-space indent, same-line braces; braceless single-statement `for` body; `size_t i`; `sizeof(int) * cap` (type form); variable `sl`; `puts("")` for newline; `const Slice *sl` for print.
- Comments: table/people analogies and conversational broken English, sometimes written before the code: `// such as table can have max 12 person`, `// on max size table`, `// so, we need move to big table`, `// you don't want call resize every time call append`, `// add err handle`, `// how we add 44`.
- Workflow: one vim buffer; visual-line + `d` to delete blocks; frequent `:w`; compile with plain `cc main.c && ./a.out` (no -Wall/-g) in a second tab.
- The "cap" text at the bottom of the frames is wallpaper art; ignored.
