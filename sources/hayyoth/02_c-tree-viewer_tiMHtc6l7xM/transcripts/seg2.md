# Segment 2 (03:52 - 06:35)

Note: the vim tab title always reads "~/lab -- vim README.md" but the buffer is C code and the save message at 06:29 says `"main.c" 31L, 488B written`. A `.main.c.swp` is also listed in the directory. So the file being edited is really main.c (tab title is stale/misleading). Code uses 2-space indent. Window layout: Terminal (2 tabs: vim, zsh) + browser showing POSIX spec.

## Timeline
- [03:52] Browser in front: pubs.opengroup.org/onlinepubs/9799919799/ (POSIX Issue 8), index search box has "opendir". Reading opendir() EXAMPLES: "Open a Directory Stream" (`#include <dirent.h>`, `DIR *dir; struct dirent *dp;`, `if ((dir = opendir (".")) == NULL) { perror ("Cannot open ."); exit (1); }`, `while ((dp = readdir (dir)) != NULL) {`), then "Find And Open a File". Terminal behind shows main.c lines 1-17 (partially hidden).
- [04:01] Terminal in front. main.c (17 lines), normal mode, block cursor on empty line 4 inside walk(). Mouse I-beam on tab bar. Tabs: "vim README.md", "-zsh".
- [04:02] INSERT mode. A new blank line added after line 1 (cursor on line 2, line 3 blank), so file is 18 lines; walk now starts at line 4. (About to add `#include <dirent.h>`.)
- [04:14] `#include <dirent.h>` at line 2; walk body: line 5 `  DIR *dir = opendir(path);`, cursor in INSERT on line 6 (indented). 19 lines.
- [04:18] Added line 6 `  if (!dir) {`, cursor on line 7 (INSERT, indented 4); line 8 `}` is the closer (auto or typed). 20 lines.
- [04:35] Typed `    perror("opendir");` (line 7) and `    retur` (line 8, mid-typing). Line 9 `}`. 21 lines.
- [04:36] Line 8 `    return 1;`, cursor INSERT on blank indented line 9; line 10 `}`. 22 lines.
- [04:40-04:42] Switched to browser (terminal behind; vim line 9 `}` brace-match highlighted). Browser scrolls the opendir example; `(dir = opendir ("."))== NULL` is text-selected/highlighted; scrolls down to "Find And Open a File" example.
- [04:48] Back in vim, normal mode, I-beam near line 9. Brace pair `{` line 6 and `}` line 9 highlighted. The file is 22 lines (unchanged): lines 8 `return 1;`, 9 `  }`, 10 `}`.
- [04:49] INSERT: new lines added after the if block: line 10 blank, line 11 `  s` typed (about to be `struct dirent *dp;`). 24 lines.
- [04:53] Line 11 `  struct dirent *dp;`, line 12 `  w` typing. 25 lines.
- [05:13] Line 12 `  while ((dp = readdir(dir)) != NULL) {`, line 13 blank, line 14 INSERT cursor (indented 4), line 15 `}` (walk closer). 27 lines. (At 05:16 the terminal behind shows line 14 `}` highlighted, i.e. the while's closing brace exists at 14 -- see 05:29 for resolved layout.)
- [05:16-05:27] Browser: reading fdopendir example in full: `int main(int argc, char *argv[])`, `struct stat statbuf; DIR *d; struct dirent *dp; int dfd, ffd;`, `if ((d = fdopendir((dfd = open("./tmp", O_RDONLY)))) == NULL) { fprintf(stderr, "Cannot open ./tmp directory\n"); exit(1); }`, `while ((dp = readdir(d)) != NULL) { if (dp->d_name[0] == '.') continue; /* there is a possible race condition here as the file could be renamed between the readdir and the open */ if ((ffd = openat(dfd, dp->d_name, O_RDONLY)) == -1) { perror(dp->d_name); continue; } if (fstat(ffd, &statbuf) == 0 && statbuf.st_size > (1024*1024)) { /* found it ... */ printf("%s: %jdK\n", dp->d_name, (intmax_t)(statbuf.st_size / 1024)); } close(ffd); } closedir(d); // note this implicitly closes dfd  return 0; }`. APPLICATION USAGE: "The opendir() function should be used in conjunction with readdir(), closedir(), and rewinddir() to examine the contents of the directory (see the EXAMPLES section in readdir()). This method is recommended for portability." Mouse I-beam hovers over `dp->d_name[0]` (05:23) and `closedir(d);` (05:27).
- [05:29] Back in vim INSERT: line 12 while header, line 13 blank, line 14 `  }`, line 15 blank with INSERT cursor (indented 2), line 16 `}`. 28 lines.
- [05:30] Line 16 `  clos` being typed (blank line 15 above). 29 lines.
- [05:33] Line 16 `  closedir(dir);`, cursor INSERT on line 17 (indented blank), line 18 `}`.
- [05:36] Normal mode; line 17 `  return 0;` (cursor on `;`). Terminal title bar shifted ~1px (window being moved/zoomed); mouse arrow at title bar. Last lines clipped (29 visible) by window size.
- [05:37] Terminal window full-screen (no browser behind, webcam still overlay). File now shows lines 1-30 (line 18 `}` end of walk; `main` lines 20-30). Cursor on `;` of `return 0;`.
- [05:38] Same; status line: `Type  :qa!  and press <Enter> to abandon all changes and exit Vim` (Ctrl-C pressed in normal mode).
- [05:40] Same message; cursor moved to line 13 column 0 (empty line inside while).
- [05:41] Window-switch transition (zoom animation): terminal at left, browser appearing, showing fdopendir example with `if (fstat(ffd, &statbuf) == 0 && statbuf.st_size > (1024*1024)) {` and `((ffd = openat(dfd, dp->d_name, O_RDONLY)) == -1) {` visible. Mouse on webcam overlay.
- [05:42] Browser in front (terminal behind at full width showing vim, same status message), still same example; I-beam on `dp->d_name[0]`.
- [05:46] Back in vim full screen; cursor on line 13; I-beam at `readdir`.
- [05:47] INSERT: line 13 `    con` typing (about to be `const char *name = dp->d_name;`).
- [05:53] Line 13 `    const char *name = dp->d_name;`, blank INSERT line 14 (indented 4). Total 31 lines.
- [06:03] zsh tab (full screen) showing earlier history (from before this segment's edits): `cc main.c`, `./a.out` -> `path .`; `./a.out abc` -> `path ./a.out`; `./a.out abc` (no output); `cc main.c`; `./a.out abc` -> `path abc`; `./a.out ~/sources` -> `path /Users/ctsotc/sources`. (Earlier debug `path %s` printing; this is stale scrollback.) Prompt `lab $ `.
- [06:04] zsh cleared; typing `lab $ cc `.
- [06:11] zsh: `cc main.c`, `./a.out` printed (readdir order): `.`, `..`, `.main.c.swp`, `README.md`, `a.out`, `main.c`; then `ls -a`: `.  ..  .main.c.swp  a.out(red)  main.c  README.md`. Cursor I-beam near output.
- [06:21] zsh cleared (empty prompt).
- [06:29] vim tab: line 14 is `    printf("%s\n", name);` (cursor on `;`), status `"main.c" 31L, 488B written`.
- [06:33] zsh: after clear, `./a.out` output: `.`, `..`, `abc`, `.main.c.swp`, `README.md`, `a.out`, `main.c` (new entry `abc` has appeared; created between runs, e.g. via touch/mkdir in another tab; not visible).
- [06:34] vim again, same, `"main.c" 31L, 488B written`.
- [06:35] macOS Mission Control / window picker (title "Desktop"), Dock visible. Terminal with main.c (31 lines) at left, browser at lower right (tooltip "The Open Group Base Specifications Issue 8") highlighted for selection; browser shows fdopendir example with `dp->d_name` highlighted/selected.

## File states
main.c (vim tab labelled README.md) as of 06:35 (all lines seen at 06:29/06:34):
```
 1| #include <stdio.h>
 2| #include <dirent.h>
 3|
 4| int walk(const char *path) {
 5|   DIR *dir = opendir(path);
 6|   if (!dir) {
 7|     perror("opendir");
 8|     return 1;
 9|   }
10|
11|   struct dirent *dp;
12|   while ((dp = readdir(dir)) != NULL) {
13|     const char *name = dp->d_name;
14|     printf("%s\n", name);
15|   }
16|
17|   closedir(dir);
18|   return 0;
19| }
20|
21| int main(int argc, char **argv) {
22|   const char *path = "."; // default path is current dir
23|
24|   if (argc > 1) {
25|     path = argv[1];
26|   }
27|
28|   int err = walk(path);
29|
30|   return err;
31| }
```
Written as 31L, 488B.

Start of segment (03:52/04:01), 17 lines:
```
 1| #include <stdio.h>
 2|
 3| int walk(const char *path) {
 4|
 5| }
 6|
 7| int main(int argc, char **argv) {
 8|   const char *path = "."; // default path is current dir
 9|
10|   if (argc > 1) {
11|     path = argv[1];
12|   }
13|
14|   int err = walk(path);
15|
16|   return err;
17| }
```
Intermediate 05:37 (before name/printf), walk body lines 4-19:
```
 4| int walk(const char *path) {
 5|   DIR *dir = opendir(path);
 6|   if (!dir) {
 7|     perror("opendir");
 8|     return 1;
 9|   }
10|
11|   struct dirent *dp;
12|   while ((dp = readdir(dir)) != NULL) {
13|
14|   }
15|
16|   closedir(dir);
17|   return 0;
18| }
```
Note: At 05:53 (name added) file is 31 lines; printf line added after 05:53 and before 06:29.

## Shell & outputs
- [06:03] visible scrollback:
```
lab $ cc main.c
lab $ ./a.out
path .
lab $ ./a.out abc
path ./a.out
lab $ ./a.out abc
lab $ cc main.c
lab $ ./a.out abc
path abc
lab $ ./a.out ~/sources
path /Users/ctsotc/sources
lab $
```
- [06:04] `lab $ cc ` (typing)
- [06:11]
```
lab $ cc main.c
lab $ ./a.out
.
..
.main.c.swp
README.md
a.out
main.c
lab $ ls -a
.            ..           .main.c.swp a.out        main.c       README.md
lab $
```
- [06:33]
```
lab $ ./a.out
.
..
abc
.main.c.swp
README.md
a.out
main.c
lab $
```
No compiler errors/warnings seen. Prompt format `lab $ `.

## Observations
- Build order: skeleton (main parses argv, default "."; walk stub) -> add dirent.h, opendir with NULL check -> readdir loop (`struct dirent *dp; while ((dp = readdir(dir)) != NULL)`) -> closedir; return 0 -> `const char *name = dp->d_name;` -> printf each name. Incremental, compile and run after each step.
- He consults the POSIX spec (opendir/readdir EXAMPLES) while typing, but does not copy verbatim: replaces `exit(1)` with `perror("opendir"); return 1;` and propagates errors via return value; does not use fdopendir/openat.
- Style: 2-space indent, K&R braces on same line, `int walk(const char *path)` returning int status, `const char *` for strings, `if (!dir)` instead of `== NULL` for opendir (but `!= NULL` for readdir as in spec), braces always used, hand-typed, one short comment (`// default path is current dir`), only <stdio.h> and <dirent.h> headers, mid-function declarations (C99).
- Output at this stage is unsorted readdir order and includes `.` and `..` (not yet filtered); no tree indentation, no d_type use yet.
- Minimalism: single file main.c, compiled with plain `cc main.c` (no Makefile, no flags), a.out in dir.
- Vim tab title stays "README.md" though buffer is main.c (swap file `.main.c.swp` confirms main.c).
