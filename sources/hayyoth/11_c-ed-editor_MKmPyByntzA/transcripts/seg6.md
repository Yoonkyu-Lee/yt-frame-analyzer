# Segment 6 (19:39 - 24:32)

Layout: left Terminal window with two tabs (tab 1 "~/lab — vim main.c"; tab 2 initially "~/lab — less ◂ man 3 strdup", later "~/lab — -zsh", later "~/lab — a.out" while the program runs). Right Terminal window shows the notes file in vim (lines 15-35 visible, status 26,3 Bot); it does not change during this segment. Webcam ignored.

## Timeline
- [19:39] vim main.c, lines 7-42 visible, cursor 28,7, 42%. Structure: mode() helper; main with `char cmd[MAX_LINE_LEN]; char lines[MAX_LINES];` (no star yet); while(1) loop; `if (on_edit) {... continue; }` branch; then flat chain q / a / p / d / w (p, d, w bodies empty `{ // print`, `}` etc.).
- [19:42] Cursor 29,7 (on `continue;`), 46%. Line 42 now visible as `else { // number`.
- [19:53] Bottom of file, cursor 48,7 on `return 0;`, 92%. Lines 42-48: `else { // number` / blank / `puts("?");` / `}` / `}` / blank / `return 0;`. (line 49 `}` seen at 19:54, cursor 49,1 Bot.)
- [20:00] INSERT mode, cursor 32,6, 86%: he restructured: line 29 `continue;` followed by line 30 `} else {`, blank 31, `}` 32 (the `continue;`/old closing is being converted into if/else). Chain still at 34+ below.
- [20:02] Normal, cursor 29,5, 92%. Now lines 28-31: `}` (closes inner else), `} else {` (29), blank (30), `}` (31); `continue;` is gone. Chain at lines 33-42 still outside the new else.
- [20:04] VISUAL LINE selection of lines 33-42 (10 lines; `if (mode(cmd, "q"))` through the `}` of the w branch), cursor 42,5. (`else { // number` block at 43 not selected.)
- [20:05] After cut: message `14 fewer lines`; file reduced to 36 lines: 29 `} else {`, 30 blank, 31 `}`, 32 blank, 33 `}`, 34 blank, 35 `return 0;`, 36 `}`; cursor 33,3 Bot. (The remaining stray `}` at 33 is the while-loop close.)
- [20:07] After paste into the else: message `14 more lines`; cursor 30,0-1 (empty line 30 above pasted block). Chain now at lines 31-44 with `if` at 4-space indent; extra `}` lines 44-45, blank 46, `}` 47, blank 48, `return 0;` 49.
- [20:08] Cursor 30,5 on `if`: blank line 30 removed, `if (mode(cmd, "q"))` is line 30. Lines: 40 `else { // number`, 41 blank, 42 `puts("?");`, 43 `}`, 44 `}`, 45 blank, 46 `}`, 47 blank, 48 `return 0;`, 49 `}`.
- [20:09] VISUAL LINE lines 30-40 (11 lines) selected, cursor 40,5.
- [20:10] VISUAL LINE lines 30-43 (14 lines) selected, cursor 43,5; block shifted one indent level deeper (`>` indent) — `if` now at col 6.
- [20:11] Normal, cursor 45,0-1. Chain indented to 6 spaces inside `} else {` (lines 30-43), line 44 `}`, 45 blank, 46 `}`, 47 blank, 48 `return 0;`, 49 `}`.
- [20:15] Tab 2: less viewing `man 3 strdup`: STRDUP(3) Library Functions Manual. NAME strdup, strndup — save a copy of a string. LIBRARY Standard C Library (libc, -lc). SYNOPSIS `#include <string.h>`; `char *` / `strdup(const char *s1);` (highlighted) ; `char *` / `strndup(const char *s1, size_t n);`. DESCRIPTION: "The strdup() function allocates sufficient memory for a copy of the string s1, does the copy, and returns a pointer to it. The pointer may subsequently be used as an argument to the function free(3). If insufficient memory is available, NULL is returned and errno is set to ENOMEM. The strndup() function copies at most n characters from the string s1 always NUL terminating the copied string." SEE ALSO free(3), malloc(3). HISTORY "The strdup() function first appeared in 4.4BSD. The strndup() function was added in FreeBSD 7.2." Prompt `:`.
- [20:16] Tab 2 title now "~/lab — -zsh": `lab $ man 3 strdup` then `lab $ c` (typing).
- [20:18] `cc main.c` run: error "use of undeclared identifier 'MAX_LINES'" (see Shell).
- [20:20] Back in vim tab 1 (tab 2 title -zsh). Message `"main.c" 47L, 916B written`. Cursor 41,9 on `puts`. The blank line after `else { // number` was removed: 40 `else { // number`, 41 `puts("?");`, 42 `}`, 43 `}`, 44 `}`, 45 blank, 46 `return 0;`, 47 `}`; chain `if` at 4 spaces.
- [20:22] Top of file (cursor 1,9). **Line 4: `#define MAX_LINEs 1024`** (typo, lowercase s). Line 5 `#define MAX_LINE_LEN 1024`.
- [20:28] Tab 2: second `cc main.c`: now error at 27:29 (incompatible pointer to integer conversion assigning to 'char' from 'char *') and warning unused variable 'count_line'. (So the MAX_LINEs typo was already fixed before this compile — vim line 4 later shows MAX_LINES.)
- [20:49] vim: line 4 `#define MAX_LINES 1024` (cursor 4,17 on the S). Status shows stale `"main.c" 47L, 916B written`. Line 13 still `char lines[MAX_LINES];`.
- [20:56] Tab 2: screen cleared, `lab $ cc main.c` typed.
- [20:57] Tab 2 output: only the unused variable 'count_line' warning (see Shell) — error fixed.
- [20:58] vim main.c at top: line 13 now `char *lines[MAX_LINES];` (cursor 13,8 on `*`), message `"main.c" 47L, 917B written`.
- [21:13] vim tab: [file order: 20:56 and 20:57 and 20:58 are the three frames k_20m56s, k_20m57s, k_20m58s; k_21m13s is the vim top-of-file frame with 917B] — i.e. the 917B write (adding `*`) is visible at 21:13 (cursor 13,8). Note ordering above is by timestamp: 20:56 shell cleared w/ `cc main.c` typed, 20:57 shell warning-only output (compiled with warning, apparently after fix), 21:13 vim shows saved 917B.
- [21:15] Scrolled to bottom, cursor 47,1; lines 12-47 visible. 40 `else { // number`, 41 `puts("?");`, 42 `}`, 43 `}`, 44 `}`, 45 blank, 46 `return 0;`, 47 `}`.
- [21:21] INSERT, cursor 35,37, 91%: new line 35 inside the `p` branch begun with `// ` (comment, then replaced). Line 34 `else if (mode(cmd, "p")) { // print`.
- [22:05] INSERT, 36,11, 84%: line 35 `if (count_line > 0 )` (note space before `)`), line 36 blank (cursor).
- [22:24] INSERT, 37,9, 78%: line 36 `puts(lines[current_line]);`, line 37 blank cursor.
- [22:30] INSERT, 38,11, 73%: line 37 `else`, line 38 blank cursor.
- [22:38] Normal, `"main.c" 51L, 1017B written`, cursor 44,20. `p` branch complete (`else` / `puts("?");`).
- [22:56] Tab 2 -zsh: cleared, `lab $ c` typing.
- [23:00] Tab 2 title "~/lab — a.out"; vim in INSERT, 28,9, 68%: new blank line 28 inserted after `lines[current_line] = strdup(cmd);`.
- [23:05] Tab 2 (a.out running; after `cc main.c`, `./a.out`): typed `p` -> `?`, `p` -> `?`, `a`, `hello they`, `.`, `p` -> `?` (bug: count_line never incremented).
- [23:23] vim: line 28 `count_line++;` added; `"main.c" 52L, 1039B written`; cursor 29,7, 68%. Tab 2 title back to -zsh.
- [23:26] Scrolled to bottom, lines 17-52, cursor 49,3: blank 50, `return 0;` 51, `}` 52.
- [23:50] INSERT 52,5: before `return 0;` new `for (int i = 0; i < count_line; i++)` at line 51, line 52 blank cursor (indent 4). File 55 lines.
- [24:06] INSERT 51,4: a stray `/` on line 51, line 52 `for (int i = 1; i < count_line; i++)` (changed to i = 1), line 53 `free(lines[i]);`, 54 blank, 55 `return 0;`, 56 `}`.
- [24:16] Normal 51,3: `/` removed: 51 `for (int i = 1; i < count_line; i++)`, 52 `free(lines[i]);`, 53 blank, 54 `return 0;`, 55 `}`.
- [24:23] Scrolled up, cursor 26,3, showing lines 26-55 (same content).
- [24:29] Tab 2 (-zsh): fixed program run: `cc main.c`, `./a.out`: `p` / `?` / `a` / `hello they` / `.` / `p` / `hello they` / `p` / `hello they` / `q` / `lab $`.
- [24:30] Tab 2 cleared: `lab $ `.
- [24:32] A third Terminal window opens overlapping the others (title "Terminal"): `lab $ ed` being typed (system ed, presumably for comparison).

## File states

### main.c at end of segment (last editor view 24:23; lines 1-25 from 23:50 / 20:49 frames)
```
1| #include <stdio.h>
2| #include <string.h>
3| 
4| #define MAX_LINES 1024
5| #define MAX_LINE_LEN 1024
6| 
7| int mode(char *cmd, char *mode) {
8|   return (strcmp(cmd, mode) == 0);
9| }
10| 
11| int main(void) {
12|   char cmd[MAX_LINE_LEN];
13|   char *lines[MAX_LINES];
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
28|         count_line++;
29|       }
30|     } else {
31|       if (mode(cmd, "q"))
32|         break;
33|       else if (mode(cmd, "a")) // append
34|         on_edit = 1;
35|       else if (mode(cmd, "p")) { // print
36|         if (count_line > 0 )
37|           puts(lines[current_line]);
38|         else
39|           puts("?");
40|       }
41|       else if (mode(cmd, "d")) { // delete
42|       }
43|       else if (mode(cmd, "w")) { // save
44|       }
45|       else { // number
46|         puts("?");
47|       }
48|     }
49|   }
50| 
51|   for (int i = 1; i < count_line; i++)
52|     free(lines[i]);
53| 
54|   return 0;
55| }
```
Indent is 2 spaces/level. No `<stdlib.h>` include seen (lines 1-2 only stdio, string). Last visible save checksum: `"main.c" 52L, 1039B written` (23:23); the for/free addition (55L) has no visible save message.

### Intermediate: 19:39 version (before restructure; lines 7-42 visible)
```
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
?? (43+ : `else { // number`, blank, `puts("?");`, `}`, `}` closing while, blank, `return 0;`, `}`)
```

### Notes/reference file (right window; unchanged; lines 15-35 visible, status 26,3 Bot, cursor on the boxed `.` of line 26)
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
?? (lines 1-14 not visible)
```

## Shell & outputs
- [20:15] `less` of `man 3 strdup` (content in Timeline).
- [20:16] `lab $ man 3 strdup` / `lab $ c`
- [20:18]
```
lab $ man 3 strdup
lab $ cc main.c
main.c:13:14: error: use of undeclared
      identifier 'MAX_LINES'
   13 |   char lines[MAX_LINES];
      |              ^~~~~~~~~~
1 error generated.
lab $
```
- [20:28]
```
lab $ cc main.c
main.c:27:29: error: incompatible pointer
      to integer conversion assigning to 'char' from 'char *';
      dereference with * [-Wint-conversion]
   27 |         lines[current_line] = strdup(cmd);
      |                             ^ ~~~~~~~~~~~~~
      |                               *
main.c:15:7: warning: unused variable
      'count_line' [-Wunused-variable]
   15 |   int count_line = 0; // count line in lines
      |       ^~~~~~~~~~
1 warning and 1 error generated.
lab $
```
(earlier MAX_LINES error output remains above it on screen)
- [20:56] `lab $ cc main.c` (screen cleared)
- [20:57]
```
lab $ cc main.c
main.c:15:7: warning: unused variable
      'count_line' [-Wunused-variable]
   15 |   int count_line = 0; // count line in lines
      |       ^~~~~~~~~~
1 warning generated.
lab $
```
- [23:05] `lab $ cc main.c`, `lab $ ./a.out`, then input/output: `p`, `?`, `p`, `?`, `a`, `hello they`, `.`, `p`, `?` (cursor on next line).
- [24:29] `lab $ cc main.c`, `lab $ ./a.out`: `p`, `?`, `a`, `hello they`, `.`, `p`, `hello they`, `p`, `hello they`, `q`, `lab $`.
- [24:32] third window: `lab $ ed` (typing).

## Observations
- Feature order: input loop + append mode (`a` ... `.`) with strdup into lines[]; `p` prints current line; `q` quits; stubs for d, w, number with `puts("?")` as ed's error reply.
- 20:00-20:11 refactor: replaced `continue;`-based edit mode with if (on_edit) {...} else {...}; moved the command chain into the else by cut/paste and `>` re-indent.
- Consulted `man 3 strdup` (return type char *) before compile; fixed two compile errors: typo `MAX_LINEs` -> `MAX_LINES`, and `char lines[...]` -> `char *lines[...]`.
- `p` guards with `count_line > 0` and prints `?` on empty buffer (ed behavior). Initial bug: count_line never incremented; fixed with `count_line++;`. lines[] is 1-based (current_line++ before assignment) so cleanup loop starts at i = 1 (changed from 0); condition `i < count_line` may be off by one (not verified).
- Cleanup `for ... free(lines[i]);` added at end. Tests by running ./a.out and typing commands interactively; at the end opens a new Terminal to run the real `ed` for comparison.
- Style: 2-space indent, K&R braces, brace-less single-statement if/else, short `//` labels, no stdlib.h include yet.
