# Segment 7 (13:46 - 17:50)

Note: vim tab title always reads "~/lab — vim README.md" but the buffer is main.c (":w" messages say "main.c"). Second tab is "~/lab — -zsh" (prompt `lab $ `). Webcam overlay ignored.

## Timeline

- [13:46] Browser (Firefox-like, 2 tabs: "The Open Group Base Specifica..." and "Adding Color to Your Output Fr..." = medium.com/@selvarajk/adding-color-to-your-output-from-c-58f1a4dc4...) in front of Terminal. Medium article code visible: `void red () { printf("\033[1;31m"); }`, `void reset() { printf("\033[0m"); }`, `int main () { yellow(); printf("Selva"); red(); printf("raj\n"); reset(); return 0; }`. Mouse over "reset". Behind: vim main.c lines 1-40 (partially hidden).
- [13:48] Same page; user has selected `\033[0m` in the reset() printf (Highlight/Share popup).
- [13:49] macOS Mission Control view (Desktop): browser and Terminal as separate windows. Vim main.c visible with line 6 in a MID-EDIT/different state: `printf("\033[0;34m%s\e[0m\n", name);` — i.e. first `\033` was typed (he's replacing `\e` with `\033`), second still `\e`; cursor around col of `m` after `34`.
- [13:50] Terminal full-screen, vim tab. Line 6: `    printf("\033[0;34m%s\e[0m\n", name);` (cursor/highlight on `m` after `34`; second escape is still `\e`). Rest of file as in File states (47 lines then). Line 26: `if (dp->d_type == DT_DIR ) {` (note space before `)`).
- [13:57] zsh tab. Output of earlier `cc main.c` (command line `lab $ cc main.c`), with the old `\e` version:
  warnings about non-standard escape `\e` at main.c:6:13 and 6:23 (printf("\e[1;34m%s\e[0m\n", name)), error at main.c:23:9 for implicit `strcmp` (needs `<string.h>`). "2 warnings and 1 error generated." (Shows he was using `\e[1;34m` before; now changed to `\033[0;34m`.)
- [13:59] Back in vim: line 6 now `printf("\033[0;34m%s\033[0m\n", name);`. Status line `"main.c" 47L, 807B written`. Cursor on `(is_dir)` line 5 (matching paren highlight).
- [14:00] Vim INSERT mode; a blank line inserted at line 3 (cursor on line 3, line 4 blank), so lines shifted by 1 (print_file now line 5, walk line 12, main line 38). About to type `#include <string.h>`.
- [14:05] zsh tab: previous output as at 13:57, and a new command being typed: `lab $ cc cc mai` (typo: "cc cc mai", mid-typing).
- [14:08] zsh cleared (screen only `lab $ ./a`) — typing `./a.out`.
- [14:15] Vim: line 3 now `#include <string.h>`; status `"main.c" 48L, 827B written`. Lines 1-40 visible.
- [14:19] Vim scrolled to end (lines 9-48). main() fully visible: lines 38-48. Cursor on space after `*path` (line 39). No change.
- [14:25] Vim INSERT: inside `if (dp->d_type == DT_DIR ) {` after `print_file(name, 1);` (line 28) he inserted blank lines; cursor on line 30 (empty, indented 6). `} else {` now on line 31. File is 50 lines.
- [14:31] Comments added: line 29 blank, line 30 `      // walk to sub path`, line 31 `      //` (cursor after `// `). `} else {` on line 32.
- [14:58] Line 31 replaced: `size_t len = strlen(path) + strlen(name) + 2;` (indent 6). Line 32 blank (cursor).
- [15:15] Line 32 added: `char sub_path[len];` (VLA). Line 33 blank cursor.
- [15:31] zsh tab: `lab $ ./a.out` output:
  `abc` (in blue), `.main.c.swp`, `README.md`, `a.out`, `main.c`. (Directories printed blue, flat; no recursion yet.)
- [15:35] zsh tab became `less` showing `man snprintf` (PRINTF(3) Library Functions Manual, page 1, synopsis of printf/fprintf/sprintf/snprintf/asprintf/dprintf/vprintf/vfprintf). Prompt `:`.
- [15:42] Vim INSERT: line 33 `      snprintf(sub_path, ` (typing).
- [16:05] Line 33 complete: `        snprintf(sub_path, len, "%s/%s", path, name);` (indent 6). Line 34 blank (cursor).
- [16:09] Line 35: `      walk` typed (cursor after `walk`).
- [16:25] zsh tab: shows (screen cleared) `lab $ ./a.out` output as before (abc, .main.c.swp, README.md, a.out, main.c), then `lab $ man snprintf` and prompt (man exited).
- [16:28] zsh tab (title "a.out" — program running): `lab $ cc main.c` (no warnings/errors printed), `lab $ ./a.out` then blank cursor line (program hanging/running - likely infinite recursion or waiting).
- [16:32] zsh: after `./a.out`, output: `abc` (blue), `def.txt`, `.main.c.swp`, `README.md`, `a.out`, `main.c`. Then `lab $ ls abc` -> `def.txt`. So recursion works but output is flat (sub-dir entry `def.txt` printed after `abc`, no indentation). Order is readdir order.
- [16:37] zsh: after `ls abc`, `lab $ tree` output:
  ```
  .
  ├── a.out        (green)
  ├── abc          (blue/bold)
  │   └── def.txt
  ├── main.c
  └── README.md

  2 directories, 4 files
  ```
  (Shown for comparison with real tree; tree output sorted alphabetically, a.out green because executable, abc blue.)
- [16:40] Vim: line 35 `      walk(sub_path);` (cursor at `;`). Status `"main.c" 55L, 1013B written`.
- [16:46] Vim top of file (lines 1-40): unchanged except print_file at line 5 still `void print_file(const char *name, int is_dir) {`.
- [16:53] INSERT: print_file signature changed to `void print_file(const char *name, int is_dir, int indent) {` (line 5); a new blank line 6 with indent 2 inserted (cursor). if(is_dir) now line 7.
- [16:56] Line 6: `  printf("` typed. (Search highlight artifacts: red syntax - an unterminated string quote makes the rest of file appear red.)
- [17:06] Line 6: `  printf("%.*c"` typed (displayed `"%.*c"`).
- [17:26] Vim scrolled (lines 9-48); NORMAL mode, cursor at line 48 col 0 (`48 ` empty at end of view). Shows walk and main unchanged. Note print_file(name, 1) / print_file(name, 0) calls (lines 29, 38) NOT yet updated with indent argument.
- [17:38] zsh tab, prompt `lab $ c` (typing).
- [17:39] Tab title: "ld ◂ clang -Wall -Wextra -pedantic -Wno-unused-parameter main.c" — i.e. `cc main.c` aliased/wrapped with these flags (running). Output:
  ```
  lab $ cc main.c
  main.c:6:13: warning: precision used with 'c' conversion specifier, resulting in undefined behavior [-Wformat]
      6 |   printf("%.*c", indent * 2, ' ');
        |             ~~^~
  1 warning generated.
  ```
  So line 6 was completed as `  printf("%.*c", indent * 2, ' ');`.
- [17:43] Same output, prompt `lab $ ` returned. Dock visible, mouse on Terminal icon (switching windows).
- [17:44] Browser comes to front (Medium page still), with zsh output behind.
- [17:45] Browser new tab, typing "c" in the URL bar (Firefox Suggest dropdown).
- [17:46] Typing `c printf ` in URL bar.
- [17:47] Query now `c printf indent`; "Looking up www.google.com...".
- [17:48] Google search "c printf indent" loading (URL www.google.com/search?client=firefox-b-d&q=c+printf+indent+); top result Stack Overflow "printf - Output indentation in C".
- [17:49] Results: Stack Overflow "printf - Output indentation in C" (3 answers, 5 years ago; also "Left-pad printf with 40 spaces before each line ..." and "Indentation (and wrap-indentation) of debug strings ..."), CPlusPlus.com "Printf" ("Note regarding the c specifier: it takes an int (or wint_t) as argument, but performs the proper conversion to a char value ...").
- [17:50] Opening stackoverflow.com/questions/67089739/output-indentation-in-c in a new tab (hover link status shows that URL; new tab "stackoverflow.com/que..." loading).

## File states

### main.c at 13:50 (before string.h; 47 lines, 807B)
```
 1| #include <stdio.h>
 2| #include <dirent.h>
 3|
 4| void print_file(const char *name, int is_dir) {
 5|   if (is_dir)
 6|     printf("\033[0;34m%s\e[0m\n", name);        // at 13:50; mid-edit (\e -> \033)
 7|   else
 8|     printf("%s\n", name);
 9| }
10|
11| int walk(const char *path) {
12|   DIR *dir = opendir(path);
13|   if (!dir) {
14|     perror("opendir");
15|     return 1;
16|   }
17|
18|   struct dirent *dp;
19|   while ((dp = readdir(dir)) != NULL) {
20|     const char *name = dp->d_name;
21|
22|     // ignore files
23|     if (strcmp(name, ".") == 0 || strcmp(name, "..") == 0)
24|       continue;
25|
26|     if (dp->d_type == DT_DIR ) {
27|       print_file(name, 1);
28|     } else {
29|       print_file(name, 0);
30|     }
31|   }
32|
33|   closedir(dir);
34|   return 0;
35| }
36|
37| int main(int argc, char **argv) {
38|   const char *path = "."; // default path is current dir
39|
40|   if (argc > 1) {
41|     path = argv[1];
42|   }
43|
44|   int err = walk(path);
45|
46|   return err;
47| }
```
(Lines 41-47 confirmed from 14:19 view after +1 shift: lines 41-48 = `if (argc > 1) {`, `path = argv[1];`, `}`, blank, `int err = walk(path);`, blank, `return err;`, `}`.)

### main.c at END of segment (approx. 17:26-17:50, ~56 lines; last full write was 16:40 at 55L, 1013B, then edits to print_file)
Lines 1-8 from 16:56/17:06 view, 9-48 from 17:26 view, 49-? inferred (not visible after 17:26; main unchanged since 14:19 view except shift +9).
```
 1| #include <stdio.h>
 2| #include <dirent.h>
 3| #include <string.h>
 4|
 5| void print_file(const char *name, int is_dir, int indent) {
 6|   printf("%.*c", indent * 2, ' ');        // typed by 17:06; compiler warns (see Shell)
 7|   if (is_dir)
 8|     printf("\033[0;34m%s\033[0m\n", name);
 9|   else
10|     printf("%s\n", name);
11| }
12|
13| int walk(const char *path) {
14|   DIR *dir = opendir(path);
15|   if (!dir) {
16|     perror("opendir");
17|     return 1;
18|   }
19|
20|   struct dirent *dp;
21|   while ((dp = readdir(dir)) != NULL) {
22|     const char *name = dp->d_name;
23|
24|     // ignore files
25|     if (strcmp(name, ".") == 0 || strcmp(name, "..") == 0)
26|       continue;
27|
28|     if (dp->d_type == DT_DIR ) {
29|       print_file(name, 1);
30|
31|       // walk to sub path
32|       size_t len = strlen(path) + strlen(name) + 2;
33|       char sub_path[len];
34|       snprintf(sub_path, len, "%s/%s", path, name);
35|
36|       walk(sub_path);
37|     } else {
38|       print_file(name, 0);
39|     }
40|   }
41|
42|   closedir(dir);
43|   return 0;
44| }
45|
46| int main(int argc, char **argv) {
47|   const char *path = "."; // default path is current dir
48|
49|   ?? (lines 49-56 not visible at end; earlier: if (argc > 1) { path = argv[1]; } blank; int err = walk(path); blank; return err; })
```
Note: print_file calls on lines 29 and 38 still have 2 args (will not compile with 3-param print_file until updated; walk has no indent param yet).

### Intermediate: main.c at 14:15 (48L, 827B)
Same as 13:50 version but with `#include <string.h>` inserted at line 3 (blank at 4), line 6-7 etc. shifted +1, line 7 = `printf("\033[0;34m%s\033[0m\n", name);`.

### Intermediate: main.c at 16:40 (55L, 1013B)
Same as final except print_file still two-arg with no line 6 printf; walk block lines (shifted -1 vs final): `print_file(name, 1);` L28, blank L29, `// walk to sub path` L30, `size_t len = strlen(path) + strlen(name) + 2;` L31, `char sub_path[len];` L32, `snprintf(sub_path, len, "%s/%s", path, name);` L33, blank L34, `walk(sub_path);` L35.

README.md: not seen in this segment.

## Shell & outputs

- [13:57] (earlier session output, scrollback) `lab $ cc main.c`:
```
main.c:6:13: warning: use of non-standard escape character '\e' [-Wpedantic]
    6 |     printf("\e[1;34m%s\e[0m\n", name);
      |             ^~
main.c:6:23: warning: use of non-standard escape character '\e' [-Wpedantic]
    6 |     printf("\e[1;34m%s\e[0m\n", name);
      |                       ^~
main.c:23:9: error: call to undeclared library function 'strcmp' with type 'int (const char *, const char *)'; ISO C99 and later do not support implicit function declarations [-Wimplicit-function-declaration]
   23 |     if (strcmp(name, ".") == 0 || strcmp(name, "..") == 0)
      |         ^
main.c:23:9: note: include the header <string.h> or explicitly provide a declaration for 'strcmp'
2 warnings and 1 error generated.
lab $
```
- [14:05] typing `cc cc mai` (typo, abandoned); [14:08] screen cleared, typing `./a`.
- [15:31] `lab $ ./a.out`:
```
abc
.main.c.swp
README.md
a.out
main.c
```
(`abc` colored blue.)
- [15:35] `man snprintf` -> PRINTF(3) man page via less (tab title "less ◂ man snprintf").
- [16:25] `lab $ man snprintf` exited back to prompt.
- [16:28] `lab $ cc main.c` (silent success), `lab $ ./a.out` (running, no output yet).
- [16:32]:
```
lab $ cc main.c
lab $ ./a.out
abc
def.txt
.main.c.swp
README.md
a.out
main.c
lab $ ls abc
def.txt
```
- [16:37] `lab $ tree`:
```
.
├── a.out
├── abc
│   └── def.txt
├── main.c
└── README.md

2 directories, 4 files
```
- [17:39]/[17:43] `lab $ cc main.c` (tab shows clang -Wall -Wextra -pedantic -Wno-unused-parameter main.c):
```
main.c:6:13: warning: precision used with 'c' conversion specifier, resulting in undefined behavior [-Wformat]
    6 |   printf("%.*c", indent * 2, ' ');
      |           ~~^~
1 warning generated.
```
- Browser (not shell): Google search `c printf indent` [17:48]; StackOverflow link https://stackoverflow.com/questions/67089739/output-indentation-in-c opened [17:50].

## Observations

- Build order: color first via ANSI SGR (`\033[0;34m` ... `\033[0m`), after trying `\e` (non-standard, -Wpedantic warnings) and consulting the Medium article/ANSI table; he standardized on `\033`. Colors applied only to directories (blue).
- Then recursion: for DT_DIR entries, print entry then build `sub_path` via `snprintf(sub_path, len, "%s/%s", path, name)` into a VLA `char sub_path[len]`, len = strlen(path)+strlen(name)+2 (slash + NUL +1 extra), then `walk(sub_path)`. He looked up `man snprintf` before writing it. Recursion verified with a test dir `abc/def.txt`; compared against real `tree`.
- Next step (in progress): indentation by adding an `indent` param to `print_file`; first attempt `printf("%.*c", indent * 2, ' ')` triggered -Wformat warning (precision with %c is UB); he then googled "c printf indent" and opened Stack Overflow "Output indentation in C" (likely to find `%*s` approach with "" argument).
- Style: 2-space indent, K&R braces same line, `// comments` lowercase, `perror` + `return 1` error handling, no wrapper headers beyond stdio/dirent/string, no malloc (VLA), single file `main.c`, compiled with plain `cc main.c` (wrapper adds -Wall -Wextra -pedantic -Wno-unused-parameter), iterative edit-compile-run loop with small increments, relies on man pages/web docs not AI. Minor sloppiness: `DT_DIR )` with space, `// ignore files` comment mislabeled (it ignores . and ..).
- Vim tab title still says README.md, so README.md was opened first and then main.c opened in same vim session.
- Directory entries printed in readdir order (not sorted), unlike `tree`.
