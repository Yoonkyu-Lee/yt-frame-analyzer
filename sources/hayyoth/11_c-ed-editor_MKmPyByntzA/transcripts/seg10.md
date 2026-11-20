# Segment 10 (34:14 - 40:27)

## Timeline
Layout throughout: left Terminal window (tab 1 "~/lab — vim main.c", tab 2 varies), right Terminal window (reference real `ed` sessions) over a browser showing beej.us bgclr stdlib.html#man-strtol. Webcam "Human" ignored (programmer smokes from ~39:29).

- [34:14] vim main.c INSERT, cursor 23,7 (empty line inside `if (!fp) {`). Tab 2 "~/lab — less ◂ man 3 fopen". Right terminal: `lab $ cat count.txt` -> `this is line 1/2/3`; `lab $ ed count.txt` -> `45`, then typed `1`,`this is line 1`,`2`,`this is line 2`,`3`,`this is line 3`,`q`.
- [34:18] typed `prin` at line 23 (23,11).
- [34:19] right terminal cleared (`lab $ `).
- [34:39] right terminal: `lab $ ed hello.txt` / `19` / `q` / `lab $ ed haha.txt` / `haha.txt: No such file or directory`. vim line 23 `prin`.
- [34:47] vim INSERT (paste)... line 23: `printf("%s: ` (23,19).
- [34:50] line 23: `printf("%s: No such file or directory` (23,44), mode "-- INSERT (paste) --"; "No such file or directory" highlighted in right terminal (copied from it).
- [35:02] line 23 = `printf("%s: No such file or directory\n", path);`; new empty line 24 (24,7).
- [35:11] `} else {` at line 24; empty line 25 (25,7); `}` 26; `}` 27.
- [35:12] NORMAL, cursor on `}` line 26 (26,5), bracket match highlighted.
- [35:35] INSERT 25,7; line 26 `while (fgets(` (incomplete).
- [36:04] line 25 `char line[MAX_LINE_LEN];`, line 26 `while (fgets(line, MAX_LINE_LEN, fp)) {`, empty line 27 (27,9), `}` at 28.
- [36:05] NORMAL, bracket match (28,7).
- [36:26] line 27 `current_line++;`, cursor on new line 28 (28,9).
- [36:38] line 28 `count_line++;`, cursor line 29 (29,9).
- [37:14] line 29 `lines[current_line] = strdup(line);` added. NORMAL, view top = line 4, cursor 38,0-1, 6%.
- [37:33] left terminal tab 2 shows `man 3 fopen` pager end: RETURN VALUES, ERRORS ([EINVAL] ...), SEE ALSO `open(2), fclose(3), fileno(3), fseek(3), funopen(3)`, STANDARDS; prompt `:`.
- [37:34] tab 2 now "~/lab — -zsh": `lab $ man 3 fopen` / `lab $ `.
- [37:36] `lab $ cc main.c` -> warning (see Shell).
- [37:44] `lab $ ./a.out`, `lab $ ls` -> `a.out  ai.c  count.txt  hello.txt  main.c  notes.txt`, `lab $ cat count.txt` -> three lines.
- [38:01] vim: `"main.c" 79L, 1721B written` (cursor 31,5, 6%). Tab 2 title "~/lab — a.out count.txt".
- [38:02] view top line 16, cursor 51,20, 34%: file-load loop (lines 27-30: `current_line++; count_line++; lines[current_line] = strdup(line);`), main loop lines 36-51 visible.
- [38:09] cursor 27,5 NORMAL. A line `cmd[strcspn(cmd, "\r\n")] = 0;` has been pasted as new line 27 inside the file-load while loop with wrong indentation (col 5) - file now 80 lines (still `cmd`, not `line`).
- [38:20] tab 2 running `./a.out count.txt`: typed `p` -> `this is line 3` + blank line; `1` -> `this is line 1` + blank; `2` -> `this is line 2` + blank; `q` typed (cursor after q). Shows trailing-newline bug.
- [38:22] tab 2 (-zsh): `lab $ ./a.out count.txt` being re-typed.
- [38:26] `^C`, then `lab $ cc main.c` same warning (18:8 unused variable 'filename').
- [38:34] tab 2 "a.out count.txt": `./a.out count.txt` run; typed `1`,`2`,`3`,`1` give `1`/`this is line 1`/`2`/`this is line 2`/`3`/`this is line 3`/`1`/`this is line 1` with no blank lines (bug fixed); cursor on empty line.
- [38:46] tab 2 cleared: `lab $ `.
- [38:47] vim line 27 now `line[strcspn(line, "\r\n")] = 0;` (indent 8); `"main.c" 80L, 1762B written` (cursor 27,25, 34%).
- [38:49] view top line 15, cursor 15,23, 31%.
- [38:50] view top line 14, cursor 14,25, 29%.
- [38:52] VISUAL mode (shows "20" selected chars), line 18 `char filename[255];` selected (cursor 18,2).
- [39:29] NORMAL, cursor 18,1; line 18 still present.
- [39:31] scrolled; lines 28-63 visible, cursor 63,1, 61%. Shows `p`, `d`, `w` branches and `char *endptr;`.
- [39:32] lines 37-72 visible: strtol numeric-line branch; cursor 72,1, 81%.
- [39:35] cursor 61,1 (line `else if (mode(cmd, "w")) { // save`), right terminal now focused (block cursor), still old haha.txt output.
- [39:37] right terminal cleared: `lab $ `.
- [40:06] right terminal real ed session (see Shell); vim idle at 61,1.
- [40:11] same, mouse pointer near line 61.
- [40:15] vim INSERT, new line 62 `// ` opened under the `// save` comment (cursor 62,37), so `} else {` shifts to 63 (file one line longer); 80%.
- [40:23] tab 2 (-zsh) focused: `lab $ ` (mouse over tab title, X close icon shown).
- [40:27] tab 2 shows `man 3 strncmp` (STRCMP(3) Library Functions Manual: NAME strcmp, strncmp — compare strings; SYNOPSIS `#include <string.h>`, `int strcmp(const char *s1, const char *s2);`, `int strncmp(const char *s1, const char *s2, size_t n);`; RETURN VALUES; SEE ALSO bcmp(3), memcmp(3), strcasecmp(3), strcoll(3), strxfrm(3), wcscmp(3)). Tab title "~/lab — man 3 strncmp".

## File states

### main.c at end of segment (as of ~40:15; lines 1-72 as seen; the pending comment at new line 62 not yet complete)
Lines 1-13 from 34:14 view; 14-72 from 38:52/39:32 views, with the 40:15 insertion noted below.
```
 1| #include <stdio.h>
 2| #include <string.h>
 3| #include <stdlib.h>
 4| 
 5| #define MAX_LINES 1024
 6| #define MAX_LINE_LEN 1024
 7| 
 8| int mode(char *cmd, char *mode) {
 9|   return (strcmp(cmd, mode) == 0);
10| }
11| 
12| int main(int argc, char **argv) {
13|   char cmd[MAX_LINE_LEN];
14|   char *lines[MAX_LINES];
15|   int current_line = 0;
16|   int count_line = 0; // count line in lines
17| 
18|   char filename[255];
19|   if (argc > 1) {
20|     const char *path = argv[1];
21|     FILE *fp = fopen(path, "r");
22|     if (!fp) {
23|       printf("%s: No such file or directory\n", path);
24|     } else {
25|       char line[MAX_LINE_LEN];
26|       while (fgets(line, MAX_LINE_LEN, fp)) {
27|         line[strcspn(line, "\r\n")] = 0;
28|         current_line++;
29|         count_line++;
30|         lines[current_line] = strdup(line);
31|       }
32|     }
33|   }
34| 
35|   int on_edit = 0;
36|   while (1) {
37|     if (!fgets(cmd, MAX_LINE_LEN, stdin)) break;
38|     cmd[strcspn(cmd, "\r\n")] = 0;
39| 
40|     if (on_edit) {
41|       if (mode(cmd, "."))
42|         on_edit = 0;
43|       else {
44|         current_line++;
45|         lines[current_line] = strdup(cmd);
46|         count_line++;
47|       }
48|     } else {
49|       if (mode(cmd, "q"))
50|         break;
51|       else if (mode(cmd, "a")) // append
52|         on_edit = 1;
53|       else if (mode(cmd, "p")) { // print
54|         if (count_line > 0 )
55|           puts(lines[current_line]);
56|         else
57|           puts("?");
58|       }
59|       else if (mode(cmd, "d")) { // delete
60|       }
61|       else if (mode(cmd, "w")) { // save
62|       } else {
63|         char *endptr;
64|         int num = strtol(cmd, &endptr, 10);
65| 
66|         if (*endptr == '\0' && num > 0 && num <= count_line) {
67|           current_line = num;
68|           puts(lines[current_line]);
69|         } else {
70|           puts("?");
71|         }
72|       }
?? (lines 73+ not visible)
```
After the 40:15 insertion (INSERT, cursor 62,37), a new line 62 containing only whitespace + `// ` (aligned under `// save`) exists; `} else {` is line 63, `char *endptr;` 64, `int num = strtol(...)` 65, blank 66, `if (*endptr...` 67, `current_line = num;` 68, `puts(lines[current_line]);` 69, `} else {` 70, `puts("?");` 71, `}` 72. Line 66/67 `if (...) {` runs to the window edge.

### Intermediate: file-load block at 37:14 (before newline strip), saved at 38:01 (79L, 1721B)
```
24|     } else {
25|       char line[MAX_LINE_LEN];
26|       while (fgets(line, MAX_LINE_LEN, fp)) {
27|         current_line++;
28|         count_line++;
29|         lines[current_line] = strdup(line);
30|       }
31|     }
32|   }
```
After strip line added at 27: saved `"main.c" 80L, 1762B written` (38:47).

### Other files
- count.txt: `this is line 1` / `this is line 2` / `this is line 3`.
- hello.txt: 19 bytes by real ed (content not shown).
- text.txt (written by real ed at ~40:06): `this is text` (13 bytes).
- ai.c, notes.txt exist (ls) but not opened.

## Shell & outputs
- [34:14] `lab $ cat count.txt` -> `this is line 1`,`this is line 2`,`this is line 3`; `lab $ ed count.txt` -> `45`; typed `1`/`this is line 1`/`2`/`this is line 2`/`3`/`this is line 3`/`q`.
- [34:39] `lab $ ed hello.txt` -> `19`; `q`; `lab $ ed haha.txt` -> `haha.txt: No such file or directory`.
- [37:34] `lab $ man 3 fopen`
- [37:36] `lab $ cc main.c`
  ```
  main.c:18:8: warning: unused variable
        'filename' [-Wunused-variable]
     18 |   char filename[255];
        |        ^~~~~~~~
  1 warning generated.
  ```
- [37:44] `lab $ ./a.out`; `lab $ ls` -> `a.out  ai.c  count.txt  hello.txt  main.c  notes.txt`; `lab $ cat count.txt` -> 3 lines.
- [38:20] `lab $ ./a.out count.txt`; `p` -> `this is line 3` + blank; `1` -> `this is line 1` + blank; `2` -> `this is line 2` + blank; `q`.
- [38:26] `lab $ ./a.out count.txt` `^C`; `lab $ cc main.c` (same warning, 1 warning generated).
- [38:34] `lab $ ./a.out count.txt` ; `1`/`this is line 1`/`2`/`this is line 2`/`3`/`this is line 3`/`1`/`this is line 1`.
- [40:06] right terminal:
  ```
  lab $ ed
  this is text
  ?
  a
  this is text
  .
  w
  ?
  w text.txt
  13
  q
  lab $ cat text.txt
  this is text
  lab $
  ```
- [40:27] `man 3 strncmp` in left tab 2.

## Observations
- Built next: file loading from argv[1] (fopen/fgets/strdup into lines[]); error message copied verbatim from real ed's `haha.txt: No such file or directory`.
- Constantly compares against the real `ed` in the right terminal (byte count on load `45`/`19`; `w` with no filename -> `?`; `w text.txt` -> `13`; typed text outside `a` -> `?`).
- Bug found by testing: file lines kept "\n" so `p` printed blank lines; fixed by copying the strcspn idiom and renaming `cmd` -> `line` (80L, 1762B).
- Unused `filename[255]` warning persists (selected in VISUAL at 38:52, not deleted) - presumably for `w`.
- `d` and `w` branches still empty at end; programmer starts a multi-line `// ` comment under `// save` at 40:15, then goes to read man 3 strncmp.
- Numeric line selection uses strtol(cmd, &endptr, 10), range check `num > 0 && num <= count_line`, else `?`; beej strtol docs open in browser.
- Style: 2-space indent, trailing `// append`-style comments, same-line braces, `else` on its own line after a braced block in places.
- References: man 3 fopen, man 3 strncmp, beej guide strtol.
