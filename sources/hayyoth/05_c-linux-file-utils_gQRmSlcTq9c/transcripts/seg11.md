# Segment 11 (37:28 - 41:24): rm.c recursive walk (tab titled "vim touch.c")

## Timeline
Tabs throughout: `~/lab — vim touch.c` | `~/lab — vim` | `~/lab — -zsh`. NOTE: tab 1 is titled "vim touch.c" but its content is the rm program (walkrm), and he compiles `cc rm.c -o rm`. The buffer is probably rm.c (tab title stale, or touch.c reused); not verifiable. notes.txt (right top) is static all segment: VISUAL mode, row `rm`, second algorithm line `if dir, open child, if dir walk path` highlighted, `38` at bottom right. No edits to notes.txt. No vim `:w` message visible in this segment.

- [37:28] vim, view lines 16-52, NORMAL. Line 40 `struct stat st;` (cursor on `s`), 41 `if (lstat(path, &st)) {`, 42 `perror(path);`. Loop ends line 45 `}`, 48 `closedir(dir);`, 50 `return 0;`, 51 `}`.
- [37:47] Edited: 40 `struct stat st_child;`, 41 `if (lstat(new_path, &st_child)) {`; line 42 still `perror(path);` (`;` highlighted). Status line: `Type  :qa!  and press <Ent...n all changes and exit Vim` (Ctrl-C pressed).
- [38:03] 42 now `perror(new_path);`. INSERT, new blank line 45 opened inside the while body (after the `}` of the lstat check); file now 52 lines (`closedir(dir);` at 49, `return 0;` 51, `}` 52).
- [38:06] scrolled to lines 7-43: top of file, `int walkrm(const char *path) {` at line 7, first-level lstat / S_ISDIR / unlink logic lines 8-20. NORMAL.
- [38:11] view 22-58: blank 45-46, `}` at 47; main at 55 `int main(int argc, char *argv[]) {`, 56 `if (argc == 1) {`, 57 `printf("Useage: rm [path]...\n");`, 58 `return 0;`.
- [38:13] Added at line 46 `if (!S_ISDIR(st.st_mode)) {` (cursor on `if`; still `st`), 47 blank, 48 `}`.
- [38:15] INSERT; brace-match highlight on 46/47 `{` `}`.
- [38:27] INSERT. 46 now `if (!S_ISDIR(st_child.st_mode)) {` (fixed), 47 `} else {`, 48 blank, 49 cursor (inside else), 51 `}`.
- [38:40] NORMAL. 47 `// is file just remove it`, 48 blank (cursor), 49 `} else {`, 50 blank, 51 `}`.
- [38:42] scrolled up to line 12 (`}` highlighted).
- [38:44] VISUAL LINE selecting lines 16-18 (shows `3`) then 4 lines (16-19).
- [38:46] `4 lines yanked` (`if (unlink(path)) {` / `perror(path);` / `return 1;` / `}`), cursor line 30.
- [38:47] scrolled to 23-60; cursor at line 60 (blank end).
- [38:48] view 30-67; cursor at 54 (blank after else-block closing).
- [38:49] Pasted block at lines 49-52 (`if (unlink(path)) {`, `perror(path);`, `return 1;`, `}`) before `} else {` (53). Status `4 more lines`. Indent at 4 spaces (misaligned).
- [38:50] VISUAL LINE `4` on pasted lines 49-52 (to indent).
- [38:51] Indented one level: `if (unlink(path)) {` at 6 spaces, `perror(path);` 8, `return 1;` 8, `}` at 4 (cursor col), 53 `} else {`.
- [38:53] Blank line 48 removed: 47 comment, 48 `if (unlink(path)) {` (cursor on `if`), 49 `perror(path);`, 50 `return 1;`, 51 `}`, 52 `} else {`, 53 blank, 54 `}`, 56 `}`.
- [39:24] INSERT. 48 `if (unlink(new_path)) {`, 49 `perror(new_path);`; 52 `} else {`, 53 `// if folder, walk into sub`, 54 cursor (typing).
- [39:44] INSERT; scrolled to 18-54, same content.
- [39:45] gg: lines 1-37 visible (includes + first part of walkrm). INSERT.
- [39:46] scrolled 25-62. INSERT.
- [39:47] scrolled 31-68, mouse I-beam over line 50; INSERT. 65 `int main(...)`, 66 `if (argc == 1) {`, 67 `printf("Useage: rm [path]...\n");`, 68 `return 0;`.
- [40:16] Switched to tab 2 `~/lab — vim` (scratch notes buffer, INSERT, cursor line 5). See File states.
- [40:17] Tab 3 `-zsh`: ran (earlier) `cc rm.c -o rm`, `./rm abc` -> printed `path abc/cat`, `path abc/def` (two lines highlighted/selected). This output came from a debug print not in the current code (so it is from a previous version of the code; not seen).
- [40:20] Tab title `...— less ◂ man rmdir`: `RMDIR(1) General Commands Manual`, NAME `rmdir — remove directories`, SYNOPSIS `rmdir [-pv] directory ...`, DESCRIPTION "The rmdir utility removes the directory entry specified by each directory argument, provided it is empty." Options -p, -v, EXIT STATUS, EXAMPLES. Section 1 (wrong page).
- [40:23] Shell: `lab $ man rmdir` entered; man quit, new prompt.
- [40:31] Tab title `...less ◂ man 2 rmdir`: `RMDIR(2) System Calls Manual`, `#include <unistd.h>`, `int rmdir(const char *path);`, DESCRIPTION "rmdir() removes a directory file whose name is given by path. The directory must not have any entries other than '.' and '..'." RETURN VALUES "A 0 is returned if the remove succeeds; otherwise a -1 is returned and an error code is stored in the global location errno." ERRORS begins [EACCES]...
- [40:36] scrolled: ERRORS through [EIO] "An I/O error occurs while deleting the directory entry or deallocating the inode." (scrollbar at bottom)
- [40:37] scrolled: [ELOOP], [ENAMETOOLONG], [ENOENT] "The named directory does not exist."
- [40:38] scrolled back to top of man 2 rmdir.
- [40:41] Back to vim tab 1 (tab 3 title `...— less ◂ man 2 rmdir`). View 31-68. NORMAL. Line 54 now `walkrm(new_path);` added (recursion). Cursor on `;` of `closedir(dir);` (line 60).
- [40:43] `closedir(dir);` line deleted; line 60 empty (cursor), `return 0;` at 61, `}` 62.
- [40:50] INSERT. Typed line 59 `  if (rmdir(path)) {`, cursor on blank line 60.
- [40:53] 60 `perror(path);`, 61 `retur` (typing).
- [40:54] 61 `return 1;`, cursor on line 62 (INSERT), `return 0;` pushed to 64.
- [41:01] INSERT. A line 59 `//` + space (comment being typed above rmdir); 60 `if (rmdir(path)) {`, 61 `perror(path);`, 62 `return 1;`, 63 `}`, 65 `return 0;`, 66 `}`, 68 `int main(int argc, char *argv[]) {`.
- [41:09] Tab 3: `man 2 rmdir` page top (re-checking, RMDIR(2)).
- [41:10] Tab 3 shell history shows `man rmdir`, `man 3 rmdir` -> `No manual entry for rmdir`, `man 2 rmdir`; fresh prompt.
- [41:14] Shell: `cc rm.c -o rm` then `ls` (listing, abc and xyz dirs present).
- [41:24] Shell: `ls abc` -> `cat def`; `./rm abc`; `ls` -> abc gone. Test success.

## File states

### rm.c (buffer in tab "vim touch.c"), state at ~41:01 (last full view; lines 7-68 reconstructed from overlapping frames)
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
40|     struct stat st_child;
41|     if (lstat(new_path, &st_child)) {
42|       perror(new_path);
43|       return 1;
44|     }
45|
46|     if (!S_ISDIR(st_child.st_mode)) {
47|       // is file just remove it
48|       if (unlink(new_path)) {
49|         perror(new_path);
50|         return 1;
51|       }
52|     } else {
53|       // if folder, walk into sub
54|       walkrm(new_path);
55|     }
56|
57|   }
58|
59|   // ⟨text typed after `// ` not visible⟩
60|   if (rmdir(path)) {
61|     perror(path);
62|     return 1;
63|   }
64|
65|   return 0;
66| }
67|
68| int main(int argc, char *argv[]) {
69|   if (argc == 1) {
70|     printf("Useage: rm [path]...\n");
71|     return 0;
72|   }
73|
74|   for (int i = 1; i < argc; i++) {
75| ?? (rest of main not visible; at 38:49 line after `}` + blank is `for (int i = 1; i < argc; i++) {`)
```
Notes: 2-space indent; line 22 comment is one long line (wraps on screen). `closedir(dir);` was DELETED at 40:43 and not re-added by 41:01 (the version before had it at 60, between `}` of while and `return 0;`). The file-case block (lines 14-20) has no `return 0;` after a successful unlink, so a plain file falls through to opendir.

Intermediate versions:
- 37:28: loop had `struct stat st;` / `if (lstat(path, &st)) {` / `perror(path);`.
- 37:47-38:03: renamed to st_child / new_path.
- 38:13-38:40: S_ISDIR block added then `} else {`.
- 38:49-39:24: unlink block yanked (4 lines) pasted into the if-branch, indented, path -> new_path.
- 40:41: `walkrm(new_path);` added.
- 40:43-41:01: closedir removed; rmdir block added.

### Scratch notes buffer (tab 2 `~/lab — vim`) at 40:16, INSERT, cursor line 5 col 1
```
1|
2|
3| rm abc.txt def.txt --> if all is file just remove it
4|
5| rm abc --> if is folder , we must remove all file child then remove folder abc
6| if have file is link -> nofollow it ,
7|
8|
9| to remove file , use unlink
```
(rest of window `~`). Line 5 is the source of comment on code line 22; line 6 explains lstat.

### notes.txt (right top), unchanged
```
|Command | Function use          | Algorithm
|touch   | fopen                 |
|cat     | fopen, fgetc, putchar | get path and do
|mkdir   | mkdir                 |
|mv      | stat, rename          | if target is file or not exit, rename
|        |                       | if target is dir , move
|rm      | lstat, unlink, rmdir  | walk path, if file remove
|        | opendir, readdir      | if dir, open child, if dir walk path   <- highlighted (VISUAL)
|find    | lstat, strstr         | walk path, if match print
|        | opendir, readdir      | if dir, open child, if dir walk path
|tree    |                       | watch my previous video
-- VISUAL --                                                   38
```

## Shell & outputs
- [40:17]
```
lab $ cc rm.c -o rm
lab $ ./rm abc
path abc/cat
path abc/def
lab $
```
- [40:20] `man rmdir` -> RMDIR(1) General Commands Manual (see Timeline).
- [40:23]
```
lab $ man rmdir
lab $
```
- [40:31-40:38] `man 2 rmdir` in less (RMDIR(2) System Calls Manual).
- [41:10]
```
lab $ cc rm.c -o rm
lab $ ./rm abc
path abc/cat
path abc/def
lab $ man rmdir
lab $ man 3 rmdir
No manual entry for rmdir
lab $ man 2 rmdir
lab $
```
- [41:14] (continues)
```
lab $ cc rm.c -o rm
lab $ ls
abc      mkdir    mv       notes.txt  rm.c     touch.c
cat.c    mkdir.c  mv.c     rm         touch    xyz
```
(abc, xyz cyan = directories; mkdir, mv, rm, touch red = executables.)
- [41:24]
```
lab $ ls abc
cat def
lab $ ./rm abc
lab $ ls
cat.c    mkdir.c  mv.c     rm       touch    xyz
mkdir    mv       notes.txt  rm.c   touch.c
lab $
```
abc (with `cat`, `def`) removed. No compiler warnings/errors displayed.

## Observations
- Build order: rm.c recursive deletion implemented straight from the notes.txt algorithm row ("walk path, if file remove; if dir, open child, if dir walk path"). `walkrm(path)` is recursive; main loops argv.
- Design: `lstat` (not `stat`) so symlinks are not followed (scratch note "if have file is link -> nofollow it"); VLA `char new_path[path_len]` sized strlen+strlen+2, `snprintf(... "%s/%s" ...)`; skip `.` and `..` with strcmp; child handled inline (unlink for non-dirs, recurse for dirs), then rmdir the parent after the loop.
- Error handling: uniform `if (syscall(...)) { perror(path); return 1; }`; recursion's return value ignored; no errno inspection.
- Editing technique: yank/paste block (`V` + `y`, `p`), `>` to reindent, rename variables afterward; fixes bugs in passing (st -> st_child, perror(path) -> perror(new_path)).
- Debug: earlier test printed `path abc/cat`, `path abc/def` (debug print since removed or in older code).
- Man page use: `man rmdir` gave section 1 (utility), then `man 3 rmdir` (no entry), then `man 2 rmdir` for the syscall. Reads SYNOPSIS include (`<unistd.h>`), RETURN VALUES (0 / -1 + errno) and scrolls through ERRORS. Key fact used: directory must not have entries other than `.`/`..`, hence children first then rmdir.
- Style: 2-space indent, `//` comments in plain broken English ("we must remove all file child then remove folder abc", "is file just remove it", "if folder, walk into sub"), typo "Useage". Scratch notes in a second vim tab (`~/lab — vim`) for algorithm thinking.
- Issues visible at end: closedir removed (DIR leak across recursion depth, and open dir during rmdir); plain-file case falls through to opendir; `// ` comment above rmdir left unfinished at 41:01.
- Webcam ignored.
