# Segment 6 (21:13 - 24:10) transcript

Note on tab titles: the left window's tab 1 is titled `~/lab — vim touch.c` for the whole segment, but the buffer is really **mv.c** (vim says `"mv.c" 65L, 1289B written`). The title is stale. At 23:28 the buffer shows `"cat.c" 25L, 395B`, again with the same title. Other tabs: `~/lab — less ◂ man strchr` (tab 2, never opened in this segment) and `~/lab — -zsh` (tab 3).
notes.txt (right top) stays in `-- VISUAL --` with the mv row 2 (`if target is dir , move`, position `25`) highlighted until ~23:32, then the rm rows are selected.

## Timeline
- [21:13] vim (mv.c), lines 37-65 visible, cursor on blank line 37. notes.txt: VISUAL, `if target is dir , move` highlighted, position `25`. (See File states, intermediate versions.)
- [21:14] scrolled up, lines 26-63 visible.
- [21:15] lines 23-60 visible, cursor line 25 (`if (rename(path, target)) {`).
- [21:16] `-- VISUAL LINE -- 4` selects lines 25-28 (the `if (rename(path, target)) { perror(path); return 1; }` block).
- [21:17] `4 lines yanked`, cursor line 29.
- [21:19] scrolled to bottom (lines 28-65); cursor on line 63 (blank, between loop `}` at 61 and `return 0;` at 64).
- [21:21] block pasted into the for loop (file now 69 lines); `-- VISUAL LINE -- 2` selecting pasted lines 61-62 (deeper indent, about to re-indent).
- [21:22] `-- VISUAL LINE -- 4` selecting pasted lines 61-64.
- [21:23] after re-indent: lines 61-64 `if (rename(path, target)) {` / `perror(path);` / `return 1;` / `}` at loop-body indent, `}` line 65, blanks 66-67, `return 0;` 68, `}` 69.
- [21:38] edit: `rename(path, target)` became `rename(path, new_path)` (line 60); `printf("new_path %s\n", new_path);` deleted. File 68 lines. Cursor on blank line 59.
- [21:40] `printf("name: %s\n", name);` deleted (cursor on blank line 51). File 67 lines.
- [21:41] one more blank removed; 66 lines, cursor line 55 (snprintf line).
- [21:42] another blank removed; 65 lines; ex prompt `:` open at bottom (about to write).
- [21:44] switched to tab 3 (-zsh). Scrollback: `./mv ~/lab/my-xyz.txt abc` -> `name: my-xyz.txt` / `new_path abc/my-xyz.txt`; `./mv my-xyz.txt abc` -> `name: y-xyz.txt` / `new_path abc/y-xyz.txt` (bug: unconditional name++ chopped first char); `# correct now, ok just rename`.
- [21:50] `cc mv.c -o mv`, `./mv my-xyz.txt abc` (no output), `ls abc` -> `abc.txt def     def.txt` (def cyan = dir).
- [21:58] `./mv my-xyz.txt abc` again, `ls` -> files listing (see Shell).
- [22:10] `ls abc` -> `abc.txt def def.txt y-xyz.txt` (y-xyz.txt highlighted by mouse selection; leftover from the buggy run).
- [22:11] same screen, selection gone.
- [22:17] back to vim (mv.c), lines 32-65, cursor line 56; message `"mv.c" 65L, 1289B written`.
- [22:23] INSERT mode: `if (!name)` now has `name = path;` on line 50, `name++; // remove slash` on 51. File 66 lines.
- [22:25] INSERT, blank line 51 with cursor; `name++; // remove slash` pushed to 52. File 67 lines.
- [22:27] `else` added: line 51 `else`, line 52 `name++; // remove slash` (indented). Message line: `Type  :qa!  and press <Ent...n all changes and exit Vim` (Ctrl-C pressed in normal mode).
- [22:31] -zsh tab; `ls abc` output `abc.txt def def.txt y-xyz.txt`, then `lab $ ls abc` typed (not run).
- [22:36] -zsh cleared (Cmd-K); `lab $ ls` typed.
- [22:39] `ls` output, then `lab $ ./mv ` + TAB completion list `mkdir*  mv*`.
- [22:48] `./mv cat abc` (mouse cursor over abc); `ls` shows `cat` binary gone (moved into abc/).
- [22:58] vim tab: lines 32-67, `"mv.c" 67L, 1307B written` (else-fix saved; cursor on `n` of `name++` line 52). No compile of this version visible.
- [23:00] scrolled to lines 24-61.
- [23:01] top of file, lines 1-38 (header/usage).
- [23:03] lines 37-67, cursor line 37.
- [23:28] buffer switched to **cat.c** (25L): `"cat.c" 25L, 395B`; cursor line 1.
- [23:32] notes.txt window focused (title active); rm rows selected, VISUAL `2`; left window dimmed, command line shows `:e ` with cursor box pending.
- [23:36] left buffer shows a leading blank line 1 (26 lines, `#include <stdio.h>` on line 2), message `25 more lines` (cat.c content read/pasted into a new empty buffer). Notes: rm rows still highlighted.
- [23:37] buffer is 25 lines again (leading blank gone).
- [23:50] rm.c work: line 6... (in the 25-line buffer) line 5 is now `printf("Useage: rm [path]...\n");` (cat changed to rm). notes.txt selection narrowed to `lstat` (VISUAL `5`) with mouse pointer.
- [23:55] notes: selection `unlink, rmdir` + second row (VISUAL `2`).
- [24:03] vim focused again; notes rm rows highlighted (VISUAL `2`).
- [24:07] `-- VISUAL LINE -- 2` on lines 11-12.
- [24:08] `-- VISUAL LINE -- 11` selecting lines 11-21 (fopen ... fclose).
- [24:09] deleted: `11 fewer lines`; file 14 lines.
- [24:10] INSERT mode, new blank line 2 inserted (file 15 lines).

## File states

### mv.c (tab title says touch.c) - state at 22:58-23:03 (`"mv.c" 67L, 1307B written` at 22:58)
Lines 1-38 seen at 23:01, 24-61 at 23:00, 32-67 at 22:58, 37-67 at 23:03. Complete.
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
15|          //
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
Indent 2 spaces. Line 15 `//` is aligned under the comment of line 14 (exact column uncertain). Line 8/9 spacing in usage strings: line 9 is `printf("     : mv <path>... <dir>\n");` (about 5 spaces after the quote).

Intermediate versions:
- 21:13-21:21 (65 lines, before paste): line 49 `if (!name) name = path;`, 50 `name++; // remove slash`, 51 `printf("name: %s\n", name);`, 52-53 blank, 54 `// make new path`, 55 `size_t path_len = strlen(dir) + strlen(name) + 2;`, 56 `char new_path[path_len];`, 57 `snprintf(new_path, path_len, "%s/%s", dir, name);`, 58 blank, 59 `printf("new_path %s\n", new_path);`, 60 blank, 61 `}`, 62-63 blank, 64 `return 0;`, 65 `}`. Lines 23-47 as in final.
- 21:21-21:23: yanked lines 25-28 pasted after the printf inside the loop, re-indented.
- 21:38-21:42: `rename(path, new_path)`; debug printfs deleted; blanks trimmed -> saved 65L, 1289B (version: `if (!name) name = path;` newline `name++;`).
- 22:23-22:27: `if (!name)`/`name = path;` split on two lines, then `else` + `name++;` -> saved 22:58 as 67L, 1307B.

### cat.c (opened at 23:28; `"cat.c" 25L, 395B`)
```
 1| #include <stdio.h>
 2| 
 3| int main(int argc, char *argv[]) {
 4|   if (argc == 1) {
 5|     printf("Useage: cat [path]...\n");
 6|     return 0;
 7|   }
 8| 
 9|   for (int i = 1; i < argc; i++) {
10|     const char *path = argv[i];
11|     FILE *fp = fopen(path, "r");
12|     if (!fp) {
13|       perror(path);
14|       return 1;
15|     }
16| 
17|     char c;
18|     while ((c = fgetc(fp)) != EOF)
19|       putchar(c);
20| 
21|     fclose(fp);
22|   }
23| 
24|   return 0;
25| }
```
("Useage" sic.)

### rm.c (derived from cat.c in the same vim window, still titled touch.c) - state at 24:10
```
 1| #include <stdio.h>
 2| 
 3| 
 4| int main(int argc, char *argv[]) {
 5|   if (argc == 1) {
 6|     printf("Useage: rm [path]...\n");
 7|     return 0;
 8|   }
 9| 
10|   for (int i = 1; i < argc; i++) {
11|     const char *path = argv[i];
12|   }
13| 
14|   return 0;
15| }
```
At 24:10 the cursor is in INSERT on empty line 2 (file just got an extra blank line). At 24:09 (before) the file was the 14-line version without the extra blank line. Not yet saved/named in frames (no write message visible).

### notes.txt (right top, right edge cropped)
```
Build linux files utils in C

+---------+----------------------+---...
|Command  |  Function use        |              Algorithm
+---------+----------------------+---...
|touch    | fopen                |
+---------+----------------------+
|cat      | fopen, fgetc, putchar|     get path and do
+---------+----------------------+
|mkdir    | mkdir                |
+---------+----------------------+---...
|mv       | stat, rename         | if target is file or not exit, rename
|         |                      | if target is dir , move
+---------+----------------------+---...
|rm       | lstat, unlink, rmdir | walk path, if file remove
|         | opendir, readdir     | if dir, open child, if dir walk path
+---------+----------------------+---...
|find     | lstat, strstr        | walk path, if match print
|         | opendir, readdir     | if dir, open child, if dir walk path
+---------+----------------------+---...
```
(The tree row is not visible; the bottom of the window is cropped.) Highlight sequence: mv row 2 `if target is dir , move` (position 25) until ~23:28; rm rows (VISUAL 2) at 23:32-23:37 and 24:03-24:10; `lstat` (VISUAL 5) at 23:50; `unlink, rmdir...` selection at 23:55. No edits to the notes content.

## Shell & outputs
Prompt `lab $ ` (dir ~/lab).
- [21:44] scrollback:
```
lab $ ./mv ~/lab/my-xyz.txt abc
name: my-xyz.txt
new_path abc/my-xyz.txt
lab $ ./mv my-xyz.txt abc
name: y-xyz.txt
new_path abc/y-xyz.txt
lab $ # correct now, ok just rename
```
- [21:50]
```
lab $ cc mv.c -o mv
lab $ ./mv my-xyz.txt abc
lab $ ls abc
abc.txt def     def.txt
```
- [21:58]
```
lab $ ./mv my-xyz.txt abc
lab $ ls
abc          cat.c     mkdir.c   mv.c      touch     xyz
cat          mkdir     mv        notes.txt touch.c
```
- [22:10]
```
lab $ ls
lab $ ls abc
abc.txt def     def.txt  y-xyz.txt
```
- [22:39]
```
lab $ ls
abc  cat.c  mkdir.c  mv.c  touch  xyz
cat  mkdir  mv  notes.txt  touch.c
lab $ ./mv <TAB>
mkdir*  mv*
```
- [22:48]
```
lab $ ./mv cat abc
lab $ ls
abc      mkdir    mv        notes.txt touch.c
cat.c    mkdir.c  mv.c      touch     xyz
```
(The `cat` binary was moved into abc/.) No compiler warnings or errors seen; the only compile line is `cc mv.c -o mv` (no -Wall).

## Observations
- Build order: mv.c finished (usecase 1 rename to non-existent target, usecase 2 move into dir) -> cat.c reopened as a template -> rm.c started by copying cat.c's skeleton (usage + for loop) and stripping fopen/fgetc body. Vim tab title stays "touch.c" while buffers change, so titles are unreliable.
- mv algorithm mirrors notes.txt: last arg is target; `stat(target)`; ENOENT with exactly one source -> `rename(path, target)`; non-dir target -> `fprintf(stderr, "%s is not dir or not exit\n", dir)`; else loop sources, `strrchr(path, '/')` for the basename, build `new_path` with `snprintf` into a VLA of size `strlen(dir)+strlen(name)+2`, then `rename(path, new_path)`. `argc--` trick removes the target from the sources.
- Debug-by-printf: temporary `printf("name: %s\n", name)` and `printf("new_path %s\n", new_path)` used to verify, then deleted. The `y-xyz.txt` output exposed the basename bug (strrchr returns NULL without '/', and unconditional `name++` chopped a char); fixed with `if (!name) name = path; else name++;`. A shell comment documents the state: `# correct now, ok just rename`.
- Error style: `perror(path)` + `return 1` for syscall failures; fprintf(stderr, ...) for custom messages; "usage" printed with `return 0` when argc == 1. Typos "exit"/"Useage" left in place.
- Comments are informal (`// have something error`, `// usecase 2`, `// remove target from path`).
- Heavy vim visual-line yank/paste/delete/re-indent and copying finished programs as templates. Indent is 2 spaces.
- No man page opened in this segment (the `less ◂ man strchr` tab stays idle).
