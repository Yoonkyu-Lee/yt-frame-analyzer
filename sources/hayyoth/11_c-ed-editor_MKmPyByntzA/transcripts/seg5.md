# Segment 5 (12:27 - 19:39)

## Timeline
Windows: left Terminal (tabs "~/lab — vim main.c" and "~/lab — -zsh"), right Terminal showing the notes file (static, lines 15-35 visible, cursor 35,1 Bot; unchanged until a click ~16:13), "Human" webcam ignored.

- [12:27] vim main.c, INSERT, cursor 20,28. Line 20 being typed: `else if (mode(cmd, "a")` (auto-paired `)` highlighted; incomplete). File is 24 lines. `#define MAX_LINEs 1024` (lowercase s typo) on line 4, `#define MAX_LINE_LEN 1024` line 5. Lines 15-16: `if (!fgets(cmd, MAX_LINE_LEN, stdin)) break;` / `cmd[strcspn(cmd, "\r\n")] = 0;`; lines 18-19: `if (mode(cmd, "q"))` / `break;`.
- [12:51] Normal 24,5, 30 lines. Lines 20-22: `else if (mode(cmd, "a")) {` / blank / `}`. Lines 23-24: `else if (mode(cmd, "p")) {` / `}`. Line 25 `else` (dangling), 26 blank, 27 `}` (closes while).
- [12:54] VISUAL LINE selecting lines 23-24 (`else if (mode(cmd, "p")) {` and `}`) to yank; file 29 lines (dangling `else` removed). Cursor 24,1.
- [12:55] Normal 24,1, selection ended (yanked), 29 lines.
- [12:57] After pasting twice: 33 lines; lines 23-28 are three `else if (mode(cmd, "p")) {` / `}` blocks. Line 27 is the 3rd. Cursor 27,5.
- [13:29] Conditions edited: line 23 "p", line 25 "d", line 27 "w", line 29 still `"p"` highlighted in VISUAL LINE (1 line selected). File 35 lines. Cursor 29,5.
- [13:30] Normal 28,1; file 33 lines: branches q, a, p, d, w (line 27 `else if (mode(cmd, "w")) {`, 28 `}`), 29 blank, 30 `}`, 31 blank, 32 `return 0;`, 33 `}` (extra 4th p copy deleted).
- [14:02] View jumped: only `35 }` shown, cursor 35,1 Bot (file 35 lines at this moment, so something with `else { puts("?"); }` was added just before).
- [14:03] View top at 15; `else if (mode(cmd, "w")) { // save` on 27; lines 29-31 `else {` / `puts("?");` / `}`; 32 `}`; 34 `return 0;`; 35 `}`. Cursor 15,1 Bot.
- [14:04] View top at 6; same content; cursor 6,0-1.
- [14:05] View top at 5; cursor 5,1 (mouse I-beam visible). Same content, 35 lines.
- [14:35] INSERT 30,15. Comments added: line 20 `else if (mode(cmd, "a")) { // append`, 23 `... "p")) { // print`, 25 `... "d")) { // delete`, 27 `... "w")) { // save`, 29 `else { // number`, 30 `//` (second comment line being typed inside else, cursor after `// `), 31 `puts("?");`. File 36 lines.
- [14:44] INSERT 14,3: a blank indented line inserted at 14 (between blank 13 and `while (1) {` now line 15). In the else-block the second `//` comment line is gone (line 31 blank, 32 `puts("?");`). File 37 lines.
- [15:15] Normal 23,5. Line 14 now `  int on_edit = 0;` directly before `while (1) {` (15). "a" branch made braceless: line 21 `else if (mode(cmd, "a")) // append`, line 22 `on_edit = 1;` (indented). File 36 lines.
- [15:23] INSERT 19,5: new blank line 19 inserted after blank 18, before `if (mode(cmd, "q"))` (now 20). File 37 lines.
- [15:29] INSERT 21,7: `if (on_edit) {` typed on line 19, blank 20, cursor line 21 (indented), `if (mode(cmd, "q"))` at 22. File 39 lines.
- [15:32] Normal 21,5: block closed: 19 `if (on_edit) {`, 20 blank, 21 `}`, 22 blank, 23 `if (mode(cmd, "q"))`. 40 lines.
- [15:34] INSERT 21,7, 80%: a blank line inside the on_edit block at 21, `}` on 22 (the body is being opened up for typing). 
- [15:54] Switched to tab "~/lab — -zsh": visible: `lab $ man 3 strcmp`, `lab $ cc main.c`, `lab $ ./a.out`, then `a` `b` `c` `q`, prompt `lab $ `. (Earlier run: a, b, c, q echoed input, exit on q.)
- [16:05] Tab title now "~/lab — a.out" (program running): `lab $ cc main.c`, `lab $ ./a.out`, `a`, `hello they`, `on add hello they`, then cursor on empty line.
- [16:13] Same tab: `a`, `hello they`, `on add hello they`, `what`, `on add what`, `abc def`, `on add abc def`, blank cursor. Right terminal window now active (colored traffic lights), mouse over notes line 25.
- [16:18] Back in vim main.c (tab "vim main.c"; notes cursor now at 26,3 on the `.` of line 26). Save message verbatim: `"main.c" 41L, 700B written`. Cursor 20,33 80%. Code: line 19 `if (on_edit) {`, line 20 `printf("on add %s\n", cmd);` line 21 `continue;` line 22 `}`.
- [16:23] INSERT 21,7: new blank line at 21 between printf and `continue;` (now 22). 66%.
- [16:54] INSERT 22,9: line 21 `if (mode(cmd, "."))`, line 22 blank (cursor, indented), line 23 `continue;`. 57%.
- [17:04] INSERT 23,7: 22 `on_edit = 0;` (indented 8), blank line 23 (cursor), 24 `continue;`. 50%.
- [17:08] INSERT 24,9: `else` added at 23 after `on_edit = 0;`, line 24 blank cursor, 25 `continue;`. 44%.
- [17:15] INSERT 25,8: `else {` on 23, blank 24, `}` on 25 (matched pair highlighted), blank 26, `continue;` 27, `}` 28. printf line still on 20 (the `printf("on add %s\n", cmd);`). 36%.
- [17:19] Normal 22,7 (40%): printf line DELETED. Block lines 19-27: `if (on_edit) {` / `if (mode(cmd, "."))` / `on_edit = 0;` / `else {` / blank / `}` / blank / `continue;` / `}`.
- [17:35] Normal 2,19, Top. Line 23 now `// add cmd to buffer` inside the else. Cursor on `>` of `<string.h>` line 2 (jumped to top with gg-ish).
- [17:37] INSERT 13,3: blank indented line at 13 (after `char cmd[MAX_LINE_LEN];` line 12); `int on_edit = 0;` now line 15.
- [17:55] INSERT 14,3: line 13 `  char lines[MAX_LINES];` typed.
- [18:01] INSERT 15,6: line 14 `  int current_line = 0;`, typing `int` on line 15.
- [18:28] INSERT 26,9: line 15 `  int count_line = 0; // count line in lines`; blank new line 26 inside else-block above `// add cmd to buffer` (now line 27).
- [18:33] INSERT 27,9: line 26 `current_line++;` added, blank line 27, comment on 28.
- [18:45] Tab "~/lab — a.out": still the old run output (a / hello they / on add hello they / what / on add what / abc def / on add abc def), blank cursor.
- [18:47] Tab "~/lab — -zsh": cleared, `lab $ ma` being typed (-> man).
- [18:50] Tab title "~/lab — less ◂ man 3 strdup": man page STRDUP(3) Library Functions Manual. NAME `strdup, strndup — save a copy of a string`; LIBRARY `Standard C Library (libc, -lc)`; SYNOPSIS `#include <string.h>`, `char *`, `strdup(const char *s1);`, `char *`, `strndup(const char *s1, size_t n);`. DESCRIPTION: "The strdup() function allocates sufficient memory for a copy of the string s1, does the copy, and returns a pointer to it. The pointer may subsequently be used as an argument to the function free(3). If insufficient memory is available, NULL is returned and errno is set to ENOMEM. The strndup() function copies at most n characters from the string s1 always NUL terminating the copied string." SEE ALSO free(3), malloc(3). HISTORY: strdup() first appeared in 4.4BSD. strndup() was added in FreeBSD 7.2. Prompt `:`.
- [18:59] Same man page; mouse-selected (highlighted) the `char *` / `strdup(const char *s1);` synopsis lines.
- [19:00] Back to vim main.c (INSERT 27,31, Top): line 27 `      lines[current_line] = ` typed up to `= `, `// add cmd to buffer` on line 28.
- [19:07] INSERT 17,19, 40%: view scrolled (top = line 7); line 27 `lines[current_line] =` (value not yet shown); cursor shown at end of line 17 `int on_edit = 0;|`.
- [19:39] Normal 28,7, 42%: line 27 now `lines[current_line] = strdup(cmd);`. No further save message seen after the 16:18 one.

## File states
### main.c at end of segment (19:39). Lines 1-42 visible; 43+ inferred (not visible at 19:39)
```
 1| #include <stdio.h>
 2| #include <string.h>
 3| 
 4| #define MAX_LINEs 1024
 5| #define MAX_LINE_LEN 1024
 6| 
 7| int mode(char *cmd, char *mode) {
 8|   return (strcmp(cmd, mode) == 0);
 9| }
10| 
11| int main(void) {
12|   char cmd[MAX_LINE_LEN];
13|   char lines[MAX_LINES];
14|   int current_line = 0;
15|   int count_line = 0; // count line in lines
16| 
17|   int on_edit = 0;
18|   while (1) {
19|     if (!fgets(cmd, MAX_LINE_LEN, stdin)) break;
20|     cmd[strcspn(cmd, "\r\n")] = 0;
21| 
22|     if (on_edit) {
23|       if (mode(cmd, "."))
24|         on_edit = 0;
25|       else {
26|         current_line++;
27|         lines[current_line] = strdup(cmd);
28|       }
29| 
30|       continue;
31|     }
32| 
33|     if (mode(cmd, "q"))
34|       break;
35|     else if (mode(cmd, "a")) // append
36|       on_edit = 1;
37|     else if (mode(cmd, "p")) { // print
38|     }
39|     else if (mode(cmd, "d")) { // delete
40|     }
41|     else if (mode(cmd, "w")) { // save
42|     }
43|     ?? (lines 43-50 not visible; from earlier frames expected: `else { // number`, blank, `puts("?");`, `}`, `}` (closes while), blank, `return 0;`, `}`)
```
Note: `#define MAX_LINEs` has lowercase "s"; line 13 uses `MAX_LINES` and `char lines[MAX_LINES]` (char array, not yet char*[]). Indentation = 2 spaces.

Intermediate: at 16:18 (saved, `"main.c" 41L, 700B written`): on_edit block was
```
19|     if (on_edit) {
20|       printf("on add %s\n", cmd);
21|       continue;
22|     }
```
(mode q/a/p/d/w branches at lines 24-33, `else { // number` at 34, blank 35, `puts("?");` 36, `}` 37, `}` 38, blank 39, `return 0;` 40, `}` 41; line 14 `int on_edit = 0;`, line 15 `while (1) {`.)

### Notes file (right terminal), lines 15-35, static throughout
```
15|     - runs via scripts/pipes without a human observer.
16|     - AI love ed
17| * Evolution tree:
18|     - ed -> sed -> ex -> vi -> vim.
19| 
20| ----------------------------------------
21| [num] - go to line
22|   $ - last line
23| 
24| i - Insert text before the current line
25| a - Append text after the current line
26|   . - Stop edit
27| 
28| d - Delete the current line
29| p - Print the current line
30|   ,p - Print all lines
31|   1,$p - Print all lines
32| w - Write the buffer to a file
33| q - Quit the editor
34| 
35| ed [path] - Read file to buffer
```
(lines 1-14 not visible. Cursor 35,1 until ~16:13, then at 26,3 on the `.` of line 26.)

## Shell & outputs
- [15:54] zsh history visible: `lab $ man 3 strcmp`; `lab $ cc main.c`; `lab $ ./a.out`; input/echo `a`, `b`, `c`, `q`; then `lab $ `.
- [16:05-16:13] `lab $ cc main.c`, `lab $ ./a.out`, then:
```
a
hello they
on add hello they
what
on add what
abc def
on add abc def
```
(tab title "~/lab — a.out"; no `.` typed in this run, so the printf line shows each added line; the `a` command itself prints nothing)
- [18:47] `lab $ ma` (typing man), [18:50] `man 3 strdup` (less) - content in Timeline.
- vim save: `"main.c" 41L, 700B written` at 16:18.

## Observations
- Dispatcher built top-down: q first, then `a` stub, then copy-pasted `else if` blocks via VISUAL LINE yank for p, d, w, then an `else { puts("?"); }` fallback (ed prints `?` for unknown commands).
- Descriptive end-of-line comments (`// append`, `// print`, `// delete`, `// save`, `// number`) used as planning scaffolding.
- Models ed's modal input with an `on_edit` flag: while set, each line is consumed (`continue`) until a line equal to `.` resets it; debug `printf("on add %s\n", cmd)` used to test then removed.
- Reuses `mode()` (strcmp wrapper) for the `.` check.
- Style: 2-space indent, braceless single-statement ifs sometimes, `else` on its own line after `}`.
- Data: `char lines[MAX_LINES]` + `current_line` + `count_line`; consults `man 3 strcmp` earlier and `man 3 strdup` here before writing `lines[current_line] = strdup(cmd);` (array type is still char, a latent bug; `MAX_LINEs` typo also persists vs `MAX_LINES` use). Not compiled after these changes in this segment.
- Notes file is only referenced, not edited, in this segment.
