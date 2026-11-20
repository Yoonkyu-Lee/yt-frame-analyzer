# Segment 8 (26:49 - 30:41)

## Timeline
- [26:49] Terminal window (tabs: "~/lab — vim main.c" active, "~/lab — a.out"). vim shows main.c lines 20-55, normal mode, cursor 46,33 (95%). Lines 45-47: `else { // number` / `// get num and go to line` / `puts("?");`. Behind it: a browser window (beej.us) and another Terminal window; browser page is Beej's C guide strtod "endptr" section (endptr highlighted pink by find-in-page).
- [26:50-26:52] Browser raised: URL `beej.us/guide/bgclr/html/split/stdlib.html#man-` (strtod section). Find bar text `endptr`, "Highlight All" checked. Page text: "Firstly, leading whitespace is skipped. Then the functions attempt to convert characters into the floating point result. Finally, when an invalid character (or NUL character) is reached, they set endptr to point to the invalid character. Set endptr to NULL if you don't care where the first invalid character is. If you didn't set endptr to NULL, it will point to a NUL character if the translation didn't find any bad characters. That is:" then code block:
  `if (*endptr == '\0') {` / `printf("What a perfectly-formed number!\n");` / `} else {` / `printf("I found badness in your number: \"%s\"\n", endptr);` / `}`. Also NaN/Infinity text and `NAN(foobar_3490)`. The printf "What a perfectly-formed..." line is grey-selected.
- [26:52] vim window clicked/raised again (mouse I-beam over line 41).
- [26:54] vim window repositioned slightly left/up; same content; vim 46,33.
- [26:57] vim `-- INSERT --`; new line 47 `//` typed under the line-46 comment (cursor 47,12); `puts("?");` pushed to line 48.
- [27:03] Line 47 now `char *endptr;`; line 48 blank (cursor 48,9, INSERT); `puts("?");` line 49.
- [27:04-27:07] Browser raised over vim; vim unchanged (INSERT 48,9). Browser scrolled a bit; user selects the `if (*endptr == '\0') {` line in the web page's code block (reference to copy by hand).
- [27:08] vim: line 48 typed `if (*endptr == '\0') {`, line 49 blank, line 50 `}` (auto-closed brace, highlighted), `puts("?");` line 51 (INSERT, cursor 50,10, 79%).
- [27:10] (no frame content differences noted beyond 27:08; frame k_27m10s is the 27:08-state/after).
- [27:22] vim normal mode scrolled to bottom, cursor 59,11 on `return 0;`. Lines 45-52: `else { // number` / `// get num and go to line` / `char *endptr;` / `if (*endptr == '\0') {` / blank / `} else` / `puts("?");` / `}` . So `}` at old line 50 became `} else` and the following `puts("?");` is its unbraced body. File 60 lines (line 59 `return 0;`, line 60 `}` seen next frame).
- [27:23] Cursor 47,11 on `char`; "Bot".
- [27:25] Line `else { // number` deleted: line 45 now `// get num and go to line` (indented 6 spaces, cursor 45,9, `~` after last line); file 59 lines.
- [27:27] Cursor 50,5; file 58 lines (one more line, a `}`? removed -- lines 45-51: comment, `char *endptr;`, `if (...) {`, blank, `} else`, `puts("?");`, `}`; then `}` 52, `}`... see states), "Bot".
- [27:29-27:30] `-- VISUAL LINE --` selecting lines 45-49 (5 lines, cursor 49,5), then 45-50 (6 lines, cursor 50,5).
- [27:31] After `<` (dedent) the block (lines 45-50) is shifted left 2 columns (comment at column of `else if`); cursor 48,0-1 normal mode.
- [27:32] INSERT at 45,7: a blank line 45 opened between `}` (line 44, closing `else if ... "w"`) and `// get num and go to line` (now 46).
- [27:40] Line 50 changed to `} else {` (INSERT, cursor 52,8); `puts("?");` line 51, auto-closed `}` at line 52 (highlighted), `}` 53, `}` 54; file 60 lines (return 0; at 59, `}` at 60).
- [27:51] Browser raised on top (find bar `endptr`). vim: normal mode cursor 49,0-1 on the blank line inside the `if`; `} else {` line 50.
- [27:53] Find bar changed to `strtol`; browser scrolled to end of strtod section: output block `Converted string to 123.456700` / `Encountered bad characters: beej` / `Ignoring bad chars: 987.654321` / `No bad chars: 11.223300`; "See Also: atof(), strtol()"; then heading "24.5 strtol(), strtoll(), strtoul(), strtoull()" "Convert a string to an integer", "Synopsis".
- [27:54-27:59] Browser scrolled over Synopsis: `#include <stdlib.h>`; `long int strtol(const char * restrict nptr, char ** restrict endptr, int base);`; strtoll; `unsigned long int strtoul(...)`; `unsigned long long int strtoull(...)`. Description: "These convert a string to an integer like atoi(), but they have a few more bells and whistles. Most notable, they can tell you where con[version stopped]... characters, if any, appear. Leading spaces ... number. The basic idea is that if things go well, the ... contained in the strings. And if you pass in ... the NUL at the end of the string." (28:00 webcam: programmer leaning back, eyes closed.)
- [28:03] vim in front, INSERT cursor 49,1 (blank line 49).
- [28:06] INSERT: new line 48 `int` typed (cursor 48,10) between `char *endptr;` (47) and `if (*endptr == '\0') {` (now 49).
- [28:24] Line 48 now `int num = strtol(cmd, &endptr, 10);`; line 49 blank; `if (*endptr == '\0') {` at 50 (normal mode, cursor 49,0-1, 92%). File 60+ lines.
- [28:33] Cursor jumped to top (`1,1 Top`); header visible (see states): `#include <stdio.h>`, `#include <string.h>`, blank, `#define MAX_LINES 1024`, `#define MAX_LINE_LEN 1024`. (No `#include <stdlib.h>` yet.)
- [28:34] INSERT at 3,1: a new blank line inserted (lines 3 and 4 blank, defines move to 5-6). Nothing typed yet (cursor 3,1). Possibly intending to add an include.
- [28:40] Cursor 35,9 normal, view of lines 35-63 with `~` filler (file 63 lines at this point): `int num = strtol(cmd, &endptr, 10);` line 49, `if (*endptr == '\0') {` 51, blank 52, `} else {` 53, `puts("?");` 54. (The extra blank line 3 is still in file; no include added in 1-3 as of 28:34.)
- [29:48] INSERT cursor 53,8: line 51 now `if (*endptr == '\0' && num > 0 && num < count_line) {`, line 52 `current_line = num;`, line 53 empty with cursor. File 64 lines.
- [30:06] Tab "~/lab — a.out" active: terminal content (old run): `lab $ ./a.out` / `a` / `this is line 1` / `this is line 2` / `this is line 3` / `.` / `p` / `this is line 3` and block cursor on a new line.
- [30:11] Second tab renamed "~/lab — -zsh": `lab $ cc main.c` / `lab $ ./a.out ` (cursor after, ready); compile printed nothing.
- [30:24] Back in vim tab; saved: `"main.c" 64L, 1331B written`; line 53 now `puts(lines[current_line]);` (cursor 53,34). Tab 2 title is "~/lab — -zsh". `~/lab — vim main.c` tab active.
- [30:26] Cursor 28,34 (92%), lines 27-62 visible (`strdup` word cursor).
- [30:38] INSERT cursor 57,8 (89%): line 45 changed from `}` to `} else {` (highlight shows pair), new `}` line 57 auto-inserted: the number-handling block is being wrapped in a final `else {` after the `w` branch. File now 65 lines.
- [30:40] `-- VISUAL LINE --` selecting lines 46-56 (11 lines, cursor 46,0-1).
- [30:41] After `>` the selected lines 47-56 are indented one level (2 spaces) further. Last frame of the segment (not yet saved after this edit).

## File states

### main.c, lines 1-36 (as at 28:34, after the extra blank line was inserted at line 3)
```
1| #include <stdio.h>
2| #include <string.h>
3| 
4| 
5| #define MAX_LINES 1024
6| #define MAX_LINE_LEN 1024
7| 
8| int mode(char *cmd, char *mode) {
9|   return (strcmp(cmd, mode) == 0);
10| }
11| 
12| int main(void) {
13|   char cmd[MAX_LINE_LEN];
14|   char *lines[MAX_LINES];
15|   int current_line = 0;
16|   int count_line = 0; // count line in lines
17| 
18|   int on_edit = 0;
19|   while (1) {
20|     if (!fgets(cmd, MAX_LINE_LEN, stdin)) break;
21|     cmd[strcspn(cmd, "\r\n")] = 0;
22| 
23|     if (on_edit) {
24|       if (mode(cmd, "."))
25|         on_edit = 0;
26|       else {
27|         current_line++;
28|         lines[current_line] = strdup(cmd);
29|         count_line++;
30|       }
31|     } else {
32|       if (mode(cmd, "q"))
33|         break;
34|       else if (mode(cmd, "a")) // append
35|         on_edit = 1;
36|       else if (mode(cmd, "p")) { // print
```
(2-space indent per level. Before 28:34 the same lines were numbered one lower: defines at 4-5, `int mode` at 7.)

### main.c, END of segment (30:41), lines 27-62 seen; 63-65 inferred (see note)
```
27|         current_line++;
28|         lines[current_line] = strdup(cmd);
29|         count_line++;
30|       }
31|     } else {
32|       if (mode(cmd, "q"))
33|         break;
34|       else if (mode(cmd, "a")) // append
35|         on_edit = 1;
36|       else if (mode(cmd, "p")) { // print
37|         if (count_line > 0 )
38|           puts(lines[current_line]);
39|         else
40|           puts("?");
41|       }
42|       else if (mode(cmd, "d")) { // delete
43|       }
44|       else if (mode(cmd, "w")) { // save
45|       } else {
46| 
47|         // get num and go to line
48|         char *endptr;
49|         int num = strtol(cmd, &endptr, 10);
50| 
51|         if (*endptr == '\0' && num > 0 && num < count_line) {
52|           current_line = num;
53|           puts(lines[current_line]);
54|         } else {
55|           puts("?");
56|         }
57|       }
58|     }
59|   }
60| 
61|   for (int i = 1; i < count_line; i++)
62|     free(lines[i]);
63| ?? (lines 63-65 not visible at 30:41; before the wrap edit: blank, `return 0;`, `}`)
```
Note: at 30:41 the visible indentation of lines 47-56 was the result of `>` on the visual selection (one extra level vs. 30:40). Brace columns for 57-59 as seen at 30:40/30:41 (57 `}` at col of `else if`, 58 `}`, 59 `}`).

### main.c as saved at 30:24 (`"main.c" 64L, 1331B written`), lines 35-64
```
35|         on_edit = 1;
36|       else if (mode(cmd, "p")) { // print
37|         if (count_line > 0 )
38|           puts(lines[current_line]);
39|         else
40|           puts("?");
41|       }
42|       else if (mode(cmd, "d")) { // delete
43|       }
44|       else if (mode(cmd, "w")) { // save
45|       }
46| 
47|       // get num and go to line
48|       char *endptr;
49|       int num = strtol(cmd, &endptr, 10);
50| 
51|       if (*endptr == '\0' && num > 0 && num < count_line) {
52|         current_line = num;
53|         puts(lines[current_line]);
54|       } else {
55|         puts("?");
56|       }
57|     }
58|   }
59| 
60|   for (int i = 1; i < count_line; i++)
61|     free(lines[i]);
62| 
63|   return 0;
64| }
```
(This saved version: number block at 6-space indent directly after the `w` branch's `}`, no wrapping else.)

### Intermediate (27:40, number block before strtol), lines 44-54
```
44|       else if (mode(cmd, "w")) { // save
45|       }
46| 
47|       // get num and go to line
48|       char *endptr;
49|       if (*endptr == '\0') {
50| 
51|       } else {
52|         puts("?");
53|       }
54|     }
```
(Numbering approximate; at 27:40 frame the lines show 45 blank, 46 comment, ... as printed above.)

### Other files
No notes/reference file or sticky notes seen; only the Beej web page in a browser.

## Shell & outputs
- [30:06] tab "~/lab — a.out":
```
lab $ ./a.out
a
this is line 1
this is line 2
this is line 3
.
p
this is line 3
```
(cursor waiting on next line)
- [30:11] tab "~/lab — -zsh":
```
lab $ cc main.c
lab $ ./a.out 
```
No compiler output shown.
- vim message: `"main.c" 64L, 1331B written` (30:24).

## Observations
- Implementing the bare-number command (go to line N). Reads Beej's C guide: strtod's `endptr` idiom, then jumps to strtol (24.5). Writes `char *endptr; int num = strtol(cmd, &endptr, 10);` then `if (*endptr == '\0' && num > 0 && num < count_line) { current_line = num; puts(lines[current_line]); } else { puts("?"); }`; prints the new current line like ed, `?` on error.
- Mirrors the doc idiom (`if (*endptr == '\0')`), copied by hand (selected it in the browser first).
- Initially put number handling in `else { // number` block, then removed it and later re-wrapped as `} else {` after the `w` branch (45 `} else {`) and re-indented with vim visual `<`/`>`; frequent use of vim auto-closing braces, visual-line mode.
- `d` and `w` branches still empty stubs with `// delete` / `// save` labels.
- Tests with `cc main.c` then `./a.out` (no flags) and typed `a`, lines, `.`, `p`.
- Possible off-by-one: lines stored at index 1..N with `count_line` = N, but check is `num < count_line` and free loop `i < count_line` (misses last line). `<stdlib.h>` not included at 28:34 though strtol is used.
