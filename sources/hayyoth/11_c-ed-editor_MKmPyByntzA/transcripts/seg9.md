# Segment 9 (30:41 - 34:14)

## Timeline
- [30:41] Left Terminal window, tab "~/lab — vim main.c" active (other tab "~/lab — -zsh"). vim in `-- VISUAL LINE --` (11 lines selected, cursor 46,0-1, 89%) over old lines 46-56: a blank line 46, then `// get num and go to line` ... through the closing `}` of the else-branch (the strtol block). Right: browser at beej.us/guide/bgclr/html/split/stdlib.html#man-strtol (strtol/strtoll synopsis), find bar "strtol".
- [30:42] Selection gone; cursor 46,9 (92%). Line 46 is `// get num and go to line`, blank line before it is gone (the selected blank line was deleted/collapsed; file now ends line 62 blank).
- [30:45] Comment line `// get num and go to line` deleted; `char *endptr;` now line 46. Status shows `"main.c"` (partial message). File shows `return 0;` at line 62.
- [30:46] Switched to tab "~/lab — -zsh": empty prompt `lab $ `.
- [31:02] Tab title now "~/lab — a.out". `lab $ cc main.c`, `lab $ ./a.out`; ed session: `a`, `this is line 1/2/3`, `.`, `p` -> `this is line 3`.
- [31:18] More: `1` -> `this is line 1`, `p` -> `this is line 1`, `2` -> `this is line 2`, `p` -> `this is line 2`, `3` -> `?`, `p` -> `this is line 2` (bug: line 3 rejected).
- [31:21] A second Terminal window (title "Terminal") appears over the browser running real `ed` for comparison (`lab $ ed`): a / 3 lines / . / p / 2 / 1 / 2 / p / 3 / p, real ed accepts `3`.
- [31:22] Real ed window scrolled; extra `3` -> `this is line 3`.
- [31:25] Real ed: another `3` -> `this is line 3`. Left a.out window: `3` -> `?` again after `p`/`this is line 2`.
- [31:27] Back on vim main.c tab; message `"main.c" 63L, 1329B written`, cursor 47,9, 96%, line 47 `int num = strtol(cmd, &endptr, 10);` (cursor on `i`). Line 49 still `num < count_line)`.
- [31:37] Terminal window widened (right edge ~785px). Cursor 49,48: line 49 is now `if (*endptr == '\0' && num > 0 && num <= count_line) {` (`<` -> `<=`). No save message visible.
- [31:54] Cursor 26,12 (92%), lines 26-61 shown; `<=` fix visible at line 49.
- [32:02] Scrolled up: cursor 7,0-1 (22%), lines 7-42.
- [32:03] Cursor 2,19 (3%), lines 2-37.
- [32:04] Same view; focus on the right ed Terminal window.
- [32:13] Real-ed window: `w count.txt` -> `45`, `q`, back to `lab $ `.
- [32:14] Right Terminal cleared: `lab $ cat ` typed.
- [32:19] `lab $ cat count.txt` -> `this is line 1/2/3`; `lab $ ed count.txt` -> `45`; `1` -> `this is line 1`; `2` -> `this is line 2`.
- [32:23] Real ed: `3` -> `this is line 3`, `q`, `lab $ `. Focus back on vim window.
- [32:24] vim cursor 1,18 (Top): line 1 `#include <stdio.h>`.
- [32:29] vim `-- INSERT --` 17,26: a new line `// ` typed under the `// count line in lines` comment on line 17 (blank 18; `int on_edit = 0;` now 19).
- [32:32] INSERT 18,3: line 17 blank, cursor on blank line 18, 19 blank, `int on_edit = 0;` line 20 (the `//` line removed).
- [32:44] INSERT 19,3: line 18 `char filename[255];`, cursor on 19, `int on_edit = 0;` line 21.
- [33:03] INSERT 21,4: line 12 changed to `int main(int argc, char **argv) {`; lines 19-21: `if (argc > 1) {` / blank / `}`; on_edit now line 23.
- [33:22] INSERT 21,5: line 20 `const char *path = argv[1];`, cursor on blank 21, `}` line 22.
- [33:38] INSERT 22,5: line 21 `FILE *fp = fopen(path, "r");`, cursor blank 22, `}` line 23.
- [33:42] Switched to left tab "~/lab — a.out": old session still displayed (ends with `3`, `?`).
- [33:44] Tab "~/lab — -zsh": `lab $ `.
- [33:46] Back on vim (INSERT 22,5), same code.
- [33:47] zsh tab: `lab $ man 3 ` typed.
- [33:49] Tab "~/lab — less ◂ man 3 fopen": FOPEN(3) Library Functions Manual; NAME fopen, fdopen, freopen, fmemopen — stream open functions; LIBRARY Standard C Library (libc, -lc); SYNOPSIS `#include <stdio.h>` FILE * fopen(const char * restrict path, const char * restrict mode); fdopen(int fildes, const char *mode); freopen(const char *path, const char *mode, FILE *stream); fmemopen(void * restrict buf, size_t size, const char * restrict mode); DESCRIPTION; mode "r": Open for reading ... Fail if the file does not exist.
- [33:54] scrolled: "w" Open for writing / truncate to zero length / create.
- [33:55] "a" append paragraph visible, start of "An optional “+” ...".
- [33:57] "+", "x", "e" options paragraph.
- [33:58] "b" letter, S_IRUSR... (0666) umask, read/write intermix paragraph.
- [33:59] ... POSIX.1 paragraph, fdopen paragraph.
- [34:00] RETURN VALUES (returns FILE pointer, otherwise NULL and errno set), ERRORS ([EINVAL] ...), SEE ALSO heading at bottom.
- [34:03] SEE ALSO `open(2), fclose(3), fileno(3), fseek(3), funopen(3)`.
- [34:04] STANDARDS: fopen() and freopen() conform to ISO/IEC 9899:1990 ("ISO C90"), "x" mode option to C11.
- [34:07] Back to vim main.c tab (other tab "~/lab — less ◂ man 3 fopen"): INSERT 22,5; same code as 33:38.
- [34:14] vim INSERT 23,7: line 22 `if (!fp) {`, cursor on blank line 23 (indented 6 spaces), line 24 `}`.

## File states
main.c at end of segment (34:14). Lines 1-36 verbatim from the 34:14 frame; lines 37-70 reconstructed from earlier frames (30:45-32:03) shifted by +8 lines (old line N -> N+8), marked ⟨reconstructed⟩.
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
23|       ⟨cursor, blank⟩
24|   }
25| 
26|   int on_edit = 0;
27|   while (1) {
28|     if (!fgets(cmd, MAX_LINE_LEN, stdin)) break;
29|     cmd[strcspn(cmd, "\r\n")] = 0;
30| 
31|     if (on_edit) {
32|       if (mode(cmd, "."))
33|         on_edit = 0;
34|       else {
35|         current_line++;
36|         lines[current_line] = strdup(cmd);
37|         count_line++;            ⟨reconstructed from here⟩
38|       }
39|     } else {
40|       if (mode(cmd, "q"))
41|         break;
42|       else if (mode(cmd, "a")) // append
43|         on_edit = 1;
44|       else if (mode(cmd, "p")) { // print
45|         if (count_line > 0 )
46|           puts(lines[current_line]);
47|         else
48|           puts("?");
49|       }
50|       else if (mode(cmd, "d")) { // delete
51|       }
52|       else if (mode(cmd, "w")) { // save
53|       } else {
54|         char *endptr;
55|         int num = strtol(cmd, &endptr, 10);
56| 
57|         if (*endptr == '\0' && num > 0 && num <= count_line) {
58|           current_line = num;
59|           puts(lines[current_line]);
60|         } else {
61|           puts("?");
62|         }
63|       }
64|     }
65|   }
66| 
67|   for (int i = 1; i < count_line; i++)
68|     free(lines[i]);
69| 
70|   return 0;
71| ?? (closing `}` presumably line 71, not seen)
```
Note: at 34:14 the `if (!fp) {` at line 22 has no closing brace yet; the `}` at line 24 is the closing brace of `if (argc > 1) {` originally typed at 33:03.

Intermediate version, 30:41 (before comment deletion; old numbering, lines 27-62, verbatim):
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
```
(At 30:42 the blank line 46 was already gone and the comment was on 46; at 30:45 comment deleted.)

count.txt (written by real `ed`, 45 bytes):
```
1| this is line 1
2| this is line 2
3| this is line 3
```
(45 bytes is what `w count.txt` printed; each line is 15 bytes with newline.)

## Shell & outputs
- [31:02] left window: `lab $ cc main.c` ; `lab $ ./a.out` ; `a` / `this is line 1` / `this is line 2` / `this is line 3` / `.` / `p` -> `this is line 3`.
- [31:18] `1` -> `this is line 1`; `p` -> `this is line 1`; `2` -> `this is line 2`; `p` -> `this is line 2`; `3` -> `?`; `p` -> `this is line 2`. [31:25] `3` -> `?` again.
- [31:21] real ed window: `lab $ ed`, `a`, 3 lines, `.`, `p` -> `this is line 3`, `2` -> `this is line 2`, `1` -> `this is line 1`, `2` -> `this is line 2`, `p` -> `this is line 2`, `3` -> `this is line 3`, `p` -> `this is line 3`; [31:22] `3` -> `this is line 3`; [31:25] `3` -> `this is line 3`.
- [32:13] `w count.txt` -> `45`; `q`; `lab $ `.
- [32:19] `lab $ cat count.txt` -> `this is line 1`, `this is line 2`, `this is line 3`; `lab $ ed count.txt` -> `45`; `1` -> `this is line 1`; `2` -> `this is line 2`; [32:23] `3` -> `this is line 3`; `q`; `lab $ `.
- [33:47-34:04] `lab $ man 3 fopen` (tab title "less ◂ man 3 fopen"): FOPEN(3) page read: modes r/w/a/+/x/e/b, RETURN VALUES, ERRORS, SEE ALSO `open(2), fclose(3), fileno(3), fseek(3), funopen(3)`, STANDARDS.
- vim save message: `"main.c" 63L, 1329B written` at 31:27. (`"main.c"` partial at 30:45.)
- Browser (strtol man at beej.us): "These convert a string to an integer like atoi(), but they have a few more bells and whistles."

## Observations
- Programmer tests number-goto (`3` gives `?` for 3 lines), compares to real `ed`, finds off-by-one, changes `num < count_line` to `num <= count_line`.
- Removes explanatory comment `// get num and go to line`.
- Uses real `ed` to learn file I/O: `w count.txt` prints byte count `45`; `ed count.txt` prints `45` on load.
- Begins file loading from argv: `int main(int argc, char **argv)`, `char filename[255];`, `if (argc > 1) { const char *path = argv[1]; FILE *fp = fopen(path, "r"); if (!fp) {` ; reads `man 3 fopen` (modes, NULL on failure).
- Style: 2-space indents, `//` comments, names `current_line`, `count_line`, `on_edit`, `lines[]`; errors reported as `?` like ed.
