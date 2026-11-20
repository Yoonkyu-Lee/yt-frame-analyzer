# Segment 9 (31:29 - 34:28) transcript

Layout constant throughout: LEFT terminal with 3 tabs: `~/lab — vim touch.c` | `~/lab — vim` | third tab (`...less ◂ man opendir` / `...less ◂ man readdir` / `~/lab — -zsh`). RIGHT TOP notes.txt table unchanged, VISUAL mode, `readdir` highlighted in rm row (Function use col, line 2 of rm row), ruler shows `7`. Webcam ignored.

Note: tab 1 is titled "vim touch.c" but the buffer is actually rm.c (vim saved as `"rm.c" 53L, 884B written`, shell compiles `cc rm.c -o rm`). He reuses the same vim session/buffer from touch.c.

## Timeline

- [31:29] Tab 3 = `...less ◂ man opendir` (man page DIRECTORY(3) section 3; NAME: opendir, fdopendir, readdir, readdir_r, telldir, seekdir, rewinddir, closedir, fdclosedir, dirfd — directory operations). SYNOPSIS visible, `opendir(const char *filename);` line highlighted by selection. Less prompt `:`.
- [31:30] Tab 3 now `~/lab — -zsh`. Shell shows history:
  ```
  lab $ man 2 opendir
  No manual entry for opendir
  lab $ man opendir
  lab $ man█
  ```
  (He tried section 2 first - no entry; then `man opendir` worked; typing `man` again - presumably `man readdir`.)
- [31:33] Tab 3 = `...less ◂ man readdir` (same DIRECTORY(3) page; top of page, SYNOPSIS to closedir visible).
- [31:37] Scrolled to DESCRIPTION: "The readdir_r() interface is deprecated because it cannot be used correctly unless {NAME_MAX} is a fixed value." opendir/fdopendir/readdir paragraphs ("The readdir() function returns a pointer to the next directory entry... returns NULL upon reaching the end of the directory or on error ... order of the directory entries ... is not specified. Some filesystems may return entries in lexicographic sort").
- [31:38] Scrolled to EXAMPLES: sample code (dirp = opendir("."); while ((dp = readdir(dirp)) != NULL) { if (dp->d_namlen == len && strcmp(dp->d_name, name) == 0) {...} }) and start of ERRORS (EACCES, ELOOP, ENAMETOOLONG).
- [31:44] ERRORS further: ENOENT, ENOTDIR visible (scrollbar at bottom).
- [31:45] ERRORS/fdopendir errors (EBADF, ENOTDIR), readdir may fail per getdents(2), telldir->realloc(3), closedir->close(2); SEE ALSO: close(2), lseek(2), open(2), read(2), dir(5); STANDARDS (POSIX.1-2008; d_off, d_reclen, d_type non-standard).
- [31:46] Scrolling back up (EXAMPLES / ERRORS visible).
- [31:47] EXAMPLES block; mouse cursor over `readdir(dirp)` line.
- [31:55] Switched to tab 1 (vim, INSERT mode). rm.c content visible lines 1-37 (see File states). Cursor on empty line 28 (new blank lines 28-30 inside a gap before `closedir(dir);` at 31).
  - Line 22 `DIR *dir = opendir(path);` ; 23-26 `if (!dir) { perror(dir); return 1; }`.
  - Lines 28-30 blank, `closedir(dir);` at 31, `return 0;` at 33, `}` at 34, main at 36.
- [31:58] Now typed at line 28: `while ((dp = readdir(dirp)) != NULL) {` , line 29 empty, line 30 `}` (bracket-match highlight cyan). Still INSERT. Lines shifted: closedir at 33, return 0 at 35, `}` at 36.
- [32:00] Normal mode (no mode text), scrolled so line 6 on top; cursor block on line 34. Visible main: line 38 `int main(int argc, char *argv[]) {`, 39 `if (argc == 1) {`, 40 `printf("Useage: rm [path]...\n");`, 41 `return 0;`, 42 `}`.
- [32:07] Back to tab 3 (man readdir), EXAMPLES; the line `while ((dp = readdir(dirp)) != NULL) {` is selected/highlighted (he copied/used it as reference).
- [32:08] man readdir scrolled: seekdir/rewinddir/closedir/fdclosedir/dirfd paragraphs and EXAMPLES; ERRORS start.
- [32:11 - 32:16] Scrolling the man page up (DESCRIPTION top, readdir_r descriptions, telldir/seekdir/rewinddir/closedir text), ending at the top SYNOPSIS (32:16 cursor I-beam at `<dirent.h>`). At 32:12: readdir text "... Also note that not all filesystems will provide a value for d_type and may instead set the field to DT_UNKNOWN." readdir_r: "The buffer must be large enough for a struct dirent with a d_name array with {NAME_MAX} + 1 elements."
- [32:20] SYNOPSIS + start of DESCRIPTION; scrollbar at bottom.
- [32:27] Tab 1 vim, normal mode, cursor on `dir` in line 28, bracket highlighting: `while ((dp = readdir(dir)) != NULL) {` (he changed `dirp` -> `dir`; the hover over tab shows `x` close button). Line 29 empty, 30 `}`.
- [32:29] Tab 1, INSERT mode; added line 28 `struct dirent *entry` (no semicolon yet, cursor after it). Line 29 `while ((dp = readdir(dir)) != NULL) {` (dp not yet renamed), 31 `}`, closedir at 34, return 0 at 36, `}` 37, main 39, printf at 41 now reads `"Useage: rm [path]...\n"` (note: still "Useage" spelling... at 32:29 text reads `printf("Useage: rm [path]...\n");`).
- [32:58] Switched to tab 2 `~/lab — vim` scratch buffer (INSERT mode): usage notes (see File states). Cursor at start of line 5.
- [32:59] Tab 3 man readdir: SYNOPSIS; `struct dirent *entry,` portion of readdir_r selected (highlight) — he's reading the struct dirent type name for the variable.
- [33:00 - 33:03] Scrolling man readdir: readdir_r/telldir/seekdir/closedir text, EXAMPLES, ERRORS (EACCES, ELOOP, ENAMETOOLONG, ENOENT, ENOTDIR; fdopendir will fail if:).
- [33:08] Tab 1, INSERT mode: lines 28-31 now:
  ```
  struct dirent *entry;
  while ((entry = readdir(dir)) != NULL) {
    const char *name = entry.█
  ```
  (typing `entry.` – will become `->`).
- [33:18] Tab 1, INSERT: line 30 `const char *name = entry->d_name;` ; line 31 `print█` (typing printf).
- [33:31] Tab 1, normal mode: lines 9-45 visible. Line 31 `printf("%s\n", name);` Line 32 `}`. closedir at 35, return 0 at 37, `}` 38, main 40. Cursor on `*` of `*argv[]` (line 40). Webcam moved (ignore).
- [33:34] Tab 2 scratch notes (INSERT, same content as 32:58).
- [33:35] Tab 3 man readdir, EXAMPLES/ERRORS; `dp->d_name` selected (highlighted) in `strcmp(dp->d_name, name)`: he is confirming the field name `d_name`.
- [33:36] Tab 3 now `~/lab — -zsh`, prompt `lab $ ls█` (being typed).
- [33:47] zsh tab history:
  ```
  lab $ ls abc
  cat def
  lab $ ./rm
  lab $ cc rm.c -o rm
  rm.c:24:12: warning: incompatible pointer
        types passing 'DIR *' to parameter of type 'const char *'
        [-Wincompatible-pointer-types]
     24 |     perror(dir);
        |            ^~~
  /Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/usr/include/_stdio.h:264:26: note:
        passing argument to parameter here
    264 | void    perror(const char *) __cold;
        |                            ^
  1 warning generated.
  lab $ █
  ```
  (`cat` is red = file? and `def` cyan = directory, per ls colors: abc contains file `cat` and dir `def`. `./rm` ran with no args and printed nothing visible in the retained history—scroll may have been cleared; the "Useage" print isn't shown.) The cc line: `cc rm.c -o rm`.
- [33:57] Tab 1 vim normal mode, cursor block at line 34 (blank). Same code (lines 9-45).
- [34:04] Tab 3 zsh – same output as 33:47.
- [34:23] Tab 1 vim normal: cursor on `h` of `path` in line 24 `perror(path);` (changed from `perror(dir)` to fix the warning). Vim message bottom: `"rm.c" 53L, 884B written`. Lines 9-45 visible.
- [34:26] Tab 1 INSERT: new blank line inserted at line 31 after `const char *name = entry->d_name;` (cursor at line 31), `printf("%s\n", name);` pushed to line 32. closedir(dir) at 36, return 0 at 38, `}` 39, main at 41, printf Useage at 43.
- [34:28] Tab 3 zsh: after rebuild and run:
  ```
  lab $ cc rm.c -o rm
  lab $ ./rm abc
  .
  ..
  cat
  def
  lab $ █
  ```
  (no compiler warning on second compile.)

## File states

### notes.txt (right top) - unchanged all segment
Rows (VISUAL mode, ruler `7`, `readdir` highlighted in rm row):
```
|Command |  Function use      |                 Algorithm
+--------+--------------------+---------------------------------
|touch   | fopen              |
+--------+--------------------+---------------------------------
|cat     | fopen, fgetc, putchar |        get path and do
+--------+--------------------+---------------------------------
|mkdir   | mkdir              |
+--------+--------------------+---------------------------------
|mv      | stat, rename       | if target is file or not exit, rename
|        |                    | if target is dir , move
+--------+--------------------+---------------------------------
|rm      | lstat, unlink, rmdir | walk path, if file remove
|        | opendir, readdir   | if dir, open child, if dir walk path
+--------+--------------------+---------------------------------
|find    | lstat, strstr      | walk path, if match print
|        | opendir, readdir   | if dir, open child, if dir walk path
+--------+--------------------+---------------------------------
|tree    |                    | watch my previous video
+--------+--------------------+---------------------------------
```
(Column rules approximate; text is verbatim, including typo "not exit".)

### rm.c (buffer titled "vim touch.c") - state at 34:26 (INSERT, end of segment)
Lines 1-8 seen at 31:55; lines 9-45 seen at 34:23-34:26 (line numbers below as of 34:26, with the blank insert at 31). Indentation = 2 spaces per level.
```
 1| #include <stdio.h>
 2| #include <sys/stat.h>
 3| #include <unistd.h>
 4| #include <dirent.h>
 5|
 6| int walkrm(const char *path) {
 7|   struct stat st;
 8|   if (lstat(path, &st)) {
 9|     perror(path);
10|     return 1;
11|   }
12|
13|   if (!S_ISDIR(st.st_mode)) {
14|     // it is file
15|     if (unlink(path)) {
16|       perror(path);
17|       return 1;
18|     }
19|   }
20|
21|   // rm abc --> if is folder , we must remove all file child then remove folder abc
22|   DIR *dir = opendir(path);
23|   if (!dir) {
24|     perror(path);
25|     return 1;
26|   }
27|
28|   struct dirent *entry;
29|   while ((entry = readdir(dir)) != NULL) {
30|     const char *name = entry->d_name;
31|     █            <- new blank line being typed at 34:26
32|     printf("%s\n", name);
33|   }
34|
35|
36|   closedir(dir);
37|
38|   return 0;
39| }
40|
41| int main(int argc, char *argv[]) {
42|   if (argc == 1) {
43|     printf("Useage: rm [path]...\n");
44|     return 0;
45|   }
46| ?? (lines 46-53 not visible; file is 53L total at 34:23 save, so ~8 lines after line 45 incl. presumably loop over argv - not seen)
```
Note: before the 34:26 blank insert (34:23 state, 53L written) the numbering was: printf at 31, `}` 32, closedir 35, return 0 37, `}` 38, main 40, `}` 44, line 45 blank. With the insert, main shifts by one. Lines 46-53 were not on screen at any time in this segment.

Intermediate versions:
- 31:55: lines 28-30 blank (empty gap), `closedir(dir);` line 31, `return 0;` 33, `}` 34, main 36; line 24 `perror(dir);`.
- 31:58: line 28 `while ((dp = readdir(dirp)) != NULL) {`, 30 `}` (copied from man page example, `dp`/`dirp` names).
- 32:27: `while ((dp = readdir(dir)) != NULL) {` (dirp->dir).
- 32:29: added `struct dirent *entry` at 28 (no `;`), loop still uses `dp`.
- 33:08: loop `while ((entry = readdir(dir)) != NULL) {` / `const char *name = entry.`
- 33:18-33:31: `const char *name = entry->d_name;` then `printf("%s\n", name);`
- 34:23: `perror(dir)` -> `perror(path)` on line 24; saved `"rm.c" 53L, 884B written`.

### Scratch notes buffer (tab 2 `~/lab — vim`), seen 32:58 and 33:34 (INSERT mode)
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
~
```
(Line 5 wraps on screen: "...then re" / "move folder abc". Line 3 and 5 begin with a leading space-like offset? Cursor `|` at start of line 5; lines 3,5 text starts at column 0.)

## Shell & outputs

- [31:30] `lab $ man 2 opendir` -> `No manual entry for opendir`; `lab $ man opendir` (opened less ◂ man opendir, DIRECTORY(3)); `lab $ man` ... (then `man readdir`, opened at 31:33).
- [33:36] `lab $ ls` (typing; completed as `ls abc`).
- [33:47] `lab $ ls abc` -> `cat def` (cat red, def cyan).
- `lab $ ./rm` (no visible output).
- `lab $ cc rm.c -o rm` ->
```
rm.c:24:12: warning: incompatible pointer
      types passing 'DIR *' to parameter of type 'const char *'
      [-Wincompatible-pointer-types]
   24 |     perror(dir);
      |            ^~~
/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/usr/include/_stdio.h:264:26: note:
      passing argument to parameter here
  264 | void    perror(const char *) __cold;
      |                            ^
1 warning generated.
```
- [34:28] `lab $ cc rm.c -o rm` (no output), `lab $ ./rm abc` ->
```
.
..
cat
def
```
Shell prompt format: `lab $ `. Compiler: Apple clang via `cc`, macOS SDK.

## Observations

- Segment is rm.c development (tab titled vim touch.c; he recycles the same vim for each utility, saves with `:w rm.c`/`:w` after renaming buffer). He implements `walkrm(const char *path)`: lstat -> if not dir, unlink; else opendir, readdir loop, closedir. Recursion into children and rmdir not yet written in this segment (loop only prints names as a debug step).
- Man page use: `man 2 opendir` fails ("No manual entry"), so `man opendir` / `man readdir` (section 3, DIRECTORY(3)). He reads SYNOPSIS, copies the EXAMPLES loop `while ((dp = readdir(dirp)) != NULL) {` into his code, then adapts names (dirp->dir, dp->entry), checks `struct dirent *` type from readdir_r synopsis, and confirms `d_name` field by highlighting `dp->d_name` in the example. Scrolls through ERRORS, SEE ALSO, STANDARDS but mostly the EXAMPLES section.
- Debugging style: print each entry with `printf("%s\n", name)` to verify readdir behaviour; output includes `.` and `..` (he will presumably need to skip them - not yet handled in this segment).
- Error handling: `perror(path)` + `return 1;` pattern after each failing syscall (lstat, unlink, opendir). Compiler warning caught a bug (`perror(dir)` passing DIR*); fixed to `perror(path)`.
- Style: 2-space indentation, `if (lstat(...))` truthiness instead of `== -1`, comments in informal English (`// it is file`, `// rm abc --> if is folder , we must remove all file child then remove folder abc`), usage message misspelled "Useage: rm [path]...\n", returns 0 when argc == 1.
- Scratch notes buffer documents design: multiple args (`rm abc.txt def.txt --> if all is file just remove it`), directories need children removed first, symlinks should not be followed (`if have file is link -> nofollow it ,` -> hence lstat), `to remove file , use unlink`.
- notes.txt table unchanged; rm row's `readdir` highlighted in VISUAL mode (he is currently working on the readdir step).
