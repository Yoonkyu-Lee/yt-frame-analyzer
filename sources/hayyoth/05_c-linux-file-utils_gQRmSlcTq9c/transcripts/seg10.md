# Segment 10 (34:28 - 37:28): rm.c recursive walk in progress, mv.c consulted

Note: left terminal tab 1 is titled `~/lab — vim touch.c` for the whole segment, but the buffer is actually rm.c (and briefly mv.c via `:e`). The tab title is stale: one vim session is reused and buffers are switched with `:e`. Right-top notes.txt stays in `-- VISUAL --`, no edits; only the highlight moves. No man pages opened in this segment.

## Timeline

- [34:28] Tab 3 `~/lab — -zsh` active. Shell history: `ls abc` -> `cat def`; `./rm` (no output); `cc rm.c -o rm` -> warning rm.c:24:12 (`perror(dir)` with DIR*); `cc rm.c -o rm` again (clean); `./rm abc` prints `.` `..` `cat` `def` (debug listing of readdir, incl. dot entries). notes.txt: VISUAL, `readdir` highlighted in rm row, status col `7`.
- [34:32] Tab 1 vim, rm.c, INSERT. Lines 9-45 visible. Line 31 is blank cursor line inside the readdir while loop (`const char *name = entry->d_name;` at 30, `printf("%s\n", name);` at 32). Usage line 43: `printf("Useage: rm [path]...\n");` (sic "Useage").
- [34:47] INSERT. Typing the dot-skip check on line 31: `if (strcmp(name) == "." || strcmp == "..")`, line 32 being typed `con`. printf now at 33.
- [34:54] Line 32 `continue;`, 33 blank, 34 cursor (INSERT), 35 `}`. printf line gone (deleted/replaced).
- [35:09] After `:e` to mv.c: message `"mv.c" 67L, 1307B`. mv.c lines 1-38 visible, cursor at line 1.
- [35:10] mv.c scrolled to lines 37-67 (see File states).
- [35:12] mv.c VISUAL LINE selecting lines 55-57 (`size_t path_len ...`, `char new_path[path_len];`, `snprintf(...)`), status `3`: copying the path-join snippet.
- [35:13] Command line `:e ` typed.
- [35:16] `:e rm` with completion list `rm    rm.c`, then `:e rm.` typed.
- [35:17] rm.c loaded: `"rm.c" 56L, 956B`. Lines 1-37 visible; line 34 `// we have name, now get path`, `}` at 35.
- [35:19] rm.c scrolled to lines 11-47; cursor on line 47 (`}` of the usage block in main). walkrm ends line 41, main at 43.
- [35:22] INSERT, blank line 35 (cursor) after the comment line 34.
- [35:23] Pasted from mv.c (still `dir`): 35 `    size_t path_len = strlen(dir) + strlen(name) + 2;` (first line over-indented), 36 `char new_path[path_len];`, 37 `snprintf(new_path, path_len, "%s/%s", dir, name);`; cursor on blank 38.
- [35:52] Edited to `path`: 35 `size_t path_len = strlen(path) + strlen(name) + 2;`, 36 `char new_path[path_len];`, 37 `snprintf(new_path, path_len, "%s/%s", path, name);`, 38 blank, 39 typing `p` (printf). INSERT.
- [36:01] Switched to tab 3 -zsh. Same screen as 34:28 (history incl. `./rm abc` -> `.` `..` `cat` `def`); lines `.` and `..` highlighted by mouse selection (pointing out that dot entries are listed and must be skipped).
- [36:03] zsh scrolled: `cc rm.c -o rm`, `./rm abc` (`.` `..` `cat` `def`), then `cc rm.c -o rm` with 3 errors + 2 warnings (see Shell).
- [36:06] Vim: line 39 `printf("path %s\n", new_path);`. Saved: `"rm.c" 61L, 1135B written`.
- [36:07] Top of file: includes lines 1-4 (stdio, sys/stat, unistd, dirent), walkrm at line 6; cursor on line 3.
- [36:08] INSERT on new line 5, typing `#i` (adding `#include <string.h>`); walkrm now at line 7.
- [36:12] zsh cleared: only `lab $ `.
- [36:13] zsh: `cc rm.c -o rm` -> 1 error + 2 warnings (string.h fixed, strcmp(name) misuse remains). See Shell.
- [36:17] Vim redraw artifact: a line `40     printf("path %s\n", new_path);` appears between line 4 and 6. This is a rendering glitch (screen not fully redrawn), not file content.
- [36:18] Bottom of file: walkrm closes line 47; main at 49 (see File states). Cursor on line 34.
- [36:19] Cursor at line 24 (on `if`).
- [36:36] INSERT editing line 32: `if (strcmp(name, ".") == 0 || strcmp(name, "..")` (syntax colours broken while typing; cyan paren-match marks).
- [36:37] Line 32 `if (strcmp(name, ".") == 0 || strcmp(name, ".." )` (cursor on the `)`).
- [36:45] zsh tab still showing the earlier errors (from 36:13).
- [36:46] zsh: `cc rm.c -o rm` then clean prompt (compiles).
- [36:53] zsh: `cc rm.c -o rm`, `./rm abc` -> `path abc/cat`, `path abc/def` (selected grey). Focus moved to notes window (its titlebar active, I-beam over row rm).
- [36:55] notes.txt: mouse selection in VISUAL of the text `if dir, open child, if dir walk path` (status col `37`).
- [37:01] Vim: line 32 `if (strcmp(name, ".") == 0 || strcmp(name, "..") == 0)`; saved `"rm.c" 62L, 1167B written`. Mouse over tab 1 (shows x). notes status `38`, same selection.
- [37:04] Lines 14-50 visible; cursor on `{` of line 14 with bracket-match highlight on line 20 `}`.
- [37:11] INSERT: blank line 41 after printf at 40; `}` of while at 42 (about to write the recursion / next logic). Lines 14-50 visible.
- [37:17] Lines 4-40 visible (`#include <dirent.h>` 4, `#include <string.h>` 5); cursor on line 4.
- [37:19] VISUAL LINE selecting lines 8-12 (the `struct stat st; if (lstat...) {...}` block), status `5`.
- [37:20] Message `5 lines yanked`. Cursor on line 9.
- [37:22] Scrolled to lines 16-52; cursor line 51.
- [37:24] After `p`: pasted block (5 lines) after line 41: lines 42-47 show `struct stat st;` / `if (lstat(path, &st)) {` / `perror(path);` / `return 1;` / `}` / `}`; message `5 more lines`. printf still on line 40 (and blank 41).
- [37:25] VISUAL LINE over lines 42-44 (status `3`); line 44 `perror(path);` marked.
- [37:26] Visual ended; same text; cursor on line 46 (`}`).
- [37:28] printf debug line deleted; pasted block now at lines 40-44, closing `}` of while at 45, blank 46-47, `closedir(dir);` 48, `return 0;` 50, `}` 51, blank 52. No save message after this edit (last saved 37:01).

## File states

### notes.txt (right-top): unchanged in this segment
Visible table (right edge clipped at screen boundary; spacing approximate):
```
|Command | Function use           |                Algorithm
+--------+------------------------+------------------------------------------
|touch   | fopen                  |
+--------+------------------------+------------------------------------------
|cat     | fopen, fgetc, putchar  |            get path and do
+--------+------------------------+------------------------------------------
|mkdir   | mkdir                  |
+--------+------------------------+------------------------------------------
|mv      | stat, rename           | if target is file or not exit, rename
|        |                        | if target is dir , move
+--------+------------------------+------------------------------------------
|rm      | lstat, unlink, rmdir   | walk path, if file remove
|        | opendir, readdir       | if dir, open child, if dir walk path
+--------+------------------------+------------------------------------------
|find    | lstat, strstr          | walk path, if match print
|        | opendir, readdir       | if dir, open child, if dir walk path
+--------+------------------------+------------------------------------------
|tree    |                        | watch my previous video
+--------+------------------------+------------------------------------------
-- VISUAL --
```
Highlight: `readdir` (rm row, 34:28-36:53, col 7); then `if dir, open child, if dir walk path` (36:55 col 37, 37:01+ col 38).

### rm.c (tab titled "vim touch.c"): state at 37:28 (unsaved since 37:01 `"rm.c" 62L, 1167B written`)
Lines 1-4 seen at 36:07; line 5 added at 36:08-ish (string.h) and confirmed at 37:17. Lines 53+ not visible at 37:28 (main shown at the 36:18 version below).
```
 1| #include <stdio.h>
 2| #include <sys/stat.h>
 3| #include <unistd.h>
 4| #include <dirent.h>
 5| #include <string.h>
 6|
 7| int walkrm(const char *path) {
 8|   struct stat st;
 9|   if (lstat(path, &st)) {
10|     perror(path);
11|     return 1;
12|   }
13|
14|   if (!S_ISDIR(st.st_mode)) {
15|     // it is file
16|     if (unlink(path)) {
17|       perror(path);
18|       return 1;
19|     }
20|   }
21|
22|   // rm abc --> if is folder , we must remove all file child then remove folder abc
23|   DIR *dir = opendir(path);
24|   if (!dir) {
25|     perror(path);
26|     return 1;
27|   }
28|
29|   struct dirent *entry;
30|   while ((entry = readdir(dir)) != NULL) {
31|     const char *name = entry->d_name;
32|     if (strcmp(name, ".") == 0 || strcmp(name, "..") == 0)
33|       continue;
34|
35|     // we have name, now get path
36|     size_t path_len = strlen(path) + strlen(name) + 2;
37|     char new_path[path_len];
38|     snprintf(new_path, path_len, "%s/%s", path, name);
39|
40|     struct stat st;
41|     if (lstat(path, &st)) {
42|       perror(path);
43|       return 1;
44|     }
45|   }
46|
47|
48|   closedir(dir);
49|
50|   return 0;
51| }
52|
?? (line 53+ not visible at 37:28)
```
Line 22 is one long source line (wraps on screen at "child th|en remove folder abc"). Note: pasted block lines 40-44 still use `path` (not `new_path`) and the file has `if (lstat(path, &st))` on child: work in progress (bug/intermediate).

main as of 36:18 (walkrm closed at 47 then; main text unchanged since):
```
49| int main(int argc, char *argv[]) {
50|   if (argc == 1) {
51|     printf("Useage: rm [path]...\n");
52|     return 0;
53|   }
54|
55|   for (int i = 1; i < argc; i++) {
56|     const char *path = argv[i];
57|     if (walkrm(path))
58|       return 1;
59|   }
60|
61|   return 0;
62| }
```
(In the 37:28 version main begins at line 53 or so, after net +4 line shift: not visible.)

Intermediate versions of the loop:
- 34:54 (before mv.c detour):
```
29|   while ((entry = readdir(dir)) != NULL) {
30|     const char *name = entry->d_name;
31|     if (strcmp(name) == "." || strcmp == "..")
32|       continue;
33|
34|     |
35|   }
```
- 35:17 `"rm.c" 56L, 956B`: includes only stdio, sys/stat, unistd, dirent (4 lines); `walkrm` at line 6; line 34 `// we have name, now get path`.
- 36:06 `"rm.c" 61L, 1135B written` (with `printf("path %s\n", new_path);` at line 39).
- 37:01 `"rm.c" 62L, 1167B written` (string.h include + fixed strcmp). printf at line 40.

### mv.c (viewed 35:09-35:12 via `:e`; `"mv.c" 67L, 1307B`)
```
 1| #include <stdio.h>
 2| #include <sys/stat.h>
 3| #include <errno.h>
 4| #include <string.h>
 5|
 6| int main(int argc, char *argv[]) {
 7|   if (argc == 1) {
 8|     printf("Usage: mv <path> <target>\n");
 9|     printf("     : mv <path>... <dir>\n");
10|     return 0;
11|   }
12|
13|   const char *target = argv[argc - 1];
14|   argc--; // remove target from path
15|           //
16|
17|   struct stat st;
18|   if (stat(target, &st)) {
19|     // have something error
20|     //
21|     // if error is file not exit
22|     if (errno == ENOENT && argc == 2) {
23|       // do rename
24|       const char *path = argv[1];
25|       if (rename(path, target)) {
26|         perror(path);
27|         return 1;
28|       }
29|
30|       return 0;
31|     }
32|
33|     perror(target);
34|     return 1;
35|   }
36|
37|
38|   // if dir not exit, return error
39|   // usecase 2
40|   const char *dir = target;
41|   if (!S_ISDIR(st.st_mode)) {
42|     fprintf(stderr, "%s is not dir or not exit\n", dir);
43|     return 1;
44|   }
45|
46|   for (int i = 1; i < argc; i++) {
47|     const char *path = argv[i];
48|     const char *name = strrchr(path, '/');
49|     if (!name)
50|       name = path;
51|     else
52|       name++; // remove slash
53|
54|     // make new path
55|     size_t path_len = strlen(dir) + strlen(name) + 2;
56|     char new_path[path_len];
57|     snprintf(new_path, path_len, "%s/%s", dir, name);
58|
59|     if (rename(path, new_path)) {
60|       perror(path);
61|       return 1;
62|     }
63|   }
64|
65|
66|   return 0;
67| }
```
(Line 15 is a lone `//` aligned under line 14's trailing comment; whitespace in line 9's string `"     : mv <path>... <dir>\n"` is roughly 5 spaces, approximate.)

## Shell & outputs

Prompt `lab $ ` (tab `~/lab — -zsh`).

34:28 (history on screen; also 36:01):
```
lab $ ls abc
cat def
lab $ ./rm
lab $ cc rm.c -o rm
rm.c:24:12: warning: incompatible pointer types passing 'DIR *' to parameter of type 'const char *' [-Wincompatible-pointer-types]
   24 |     perror(dir);
      |            ^~~
/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/usr/include/_stdio.h:264:26: note: passing argument to parameter here
  264 | void    perror(const char *) __cold;
      |                            ^
1 warning generated.
lab $ cc rm.c -o rm
lab $ ./rm abc
.
..
cat
def
lab $
```
(The first line shows the exact text "incompatible pointer / types passing 'DIR *'..." wrapped across lines; `ls abc` output `cat def` is coloured: cat red, def teal.)

36:03:
```
lab $ cc rm.c -o rm
lab $ ./rm abc
.
..
cat
def
lab $ cc rm.c -o rm
rm.c:31:9: error: call to undeclared library function 'strcmp' with type 'int (const char *, const char *)'; ISO C99 and later do not support implicit function declarations [-Wimplicit-function-declaration]
   31 |     if (strcmp(name) == "." || strcmp == "..")
      |         ^
rm.c:31:9: note: include the header <string.h> or explicitly provide a declaration for 'strcmp'
rm.c:31:20: error: too few arguments to function call, expected 2, have 1
   31 |     if (strcmp(name) == "." || strcmp == "..")
      |         ~~~~~~      ^
rm.c:31:39: warning: result of comparison against a string literal is unspecified (use an explicit string comparison function instead) [-Wstring-compare]
   31 |     if (strcmp(name) == "." || strcmp == "..")
      |                                       ^  ~~~~
rm.c:31:39: warning: comparison of distinct pointer types ('int (*)(const char *, const char *)' and 'char *') [-Wcompare-distinct-pointer-types]
   31 |     if (strcmp(name) == "." || strcmp == "..")
      |                                ~~~~~~ ^  ~~~~
rm.c:35:23: error: call to undeclared library function 'strlen' with type 'unsigned long (const char *)'; ISO C99 and later do not support implicit function declarations [-Wimplicit-function-declaration]
   35 |     size_t path_len = strlen(path) + strlen(name) + 2;
      |                       ^
rm.c:35:23: note: include the header <string.h> or explicitly provide a declaration for 'strlen'
2 warnings and 3 errors generated.
lab $
```
(Caret/tilde underline positions approximate.)

36:13 / 36:45 (after adding `#include <string.h>`; error line numbers shift to 32):
```
lab $ cc rm.c -o rm
rm.c:32:20: error: too few arguments to function call, expected 2, have 1
   32 |     if (strcmp(name) == "." || strcmp == "..")
      |         ~~~~~~      ^
rm.c:32:39: warning: result of comparison against a string literal is unspecified (use an explicit string comparison function instead) [-Wstring-compare]
   32 |     if (strcmp(name) == "." || strcmp == "..")
      |                                       ^  ~~~~
rm.c:32:39: warning: comparison of distinct pointer types ('int (*)(const char *, const char *)' and 'char *') [-Wcompare-distinct-pointer-types]
   32 |     if (strcmp(name) == "." || strcmp == "..")
      |                                ~~~~~~ ^  ~~~~
2 warnings and 1 error generated.
lab $
```
36:46 (after fix):
```
lab $ cc rm.c -o rm
lab $
```
36:53:
```
lab $ cc rm.c -o rm
lab $ ./rm abc
path abc/cat
path abc/def
lab $
```
(Only first-level children printed; no recursion yet. `abc` contains files `cat`, `def`.)

Vim command-line interactions: `:e ` (35:13), `:e rm` + completion `rm    rm.c` (35:16), `:e rm.` (35:16). Vim messages: `"mv.c" 67L, 1307B`, `"rm.c" 56L, 956B`, `"rm.c" 61L, 1135B written`, `"rm.c" 62L, 1167B written`, `5 lines yanked`, `5 more lines`.

## Observations

- Build order: rm.c developed incrementally with printf-debugging: (1) list readdir entries, (2) skip `.` and `..`, (3) build child path `path/name`, (4) in progress at 37:28: stat the child (pasted lstat check) before recursing into `walkrm(new_path)` and removing dir with rmdir (not yet written). Matches notes algorithm "if dir, open child, if dir walk path".
- Heavy reuse by copy/paste across utilities: the path-join snippet (`strlen + strlen + 2`, VLA `new_path[path_len]`, `snprintf "%s/%s"`) yanked from mv.c via `:e` buffer switch and VISUAL LINE, then adapted `dir` -> `path`; the lstat error-check block (lines 8-12) yanked and pasted into the loop (37:19-37:28), still referencing `path`. The notes row is read aloud-ish: at 36:55 he selects `if dir, open child, if dir walk path` right before writing recursion.
- Errors left in on purpose/made live: `strcmp(name) == "."` (misuse), missing `#include <string.h>`; he lets clang (`cc`) diagnostics drive fixes, two compile rounds. Earlier (before segment) `perror(dir)` with DIR* triggered -Wincompatible-pointer-types.
- Style: 2-space indent; `if (call(...)) { perror(path); return 1; }` (non-zero = error); `return 0` for success; main loops argv and returns 1 on first failure; usage printed when `argc == 1` and exits 0. Typos kept (`Useage`, `exit` for "exist"). Conversational English comments (`// rm abc --> if is folder , we must remove all file child then remove folder abc`, `// we have name, now get path`, `// it is file`). Uses `lstat` in rm (not following symlinks) per notes; walks children with VLA buffers, no malloc.
- Terminal use: clear between compile rounds, mouse-select to highlight output (`.`/`..` and `path abc/...` lines) for the camera. notes.txt used as a visual checklist only, never edited here.
- Vim tab title `vim touch.c` is not reliable: buffer switching with `:e` means the title does not reveal the active file. No man pages in this segment; no `#` shell comments.
