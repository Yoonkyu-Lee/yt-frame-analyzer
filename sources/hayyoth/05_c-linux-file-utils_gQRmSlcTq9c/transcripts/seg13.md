# Segment 13 (44:21 - 49:04)

## Timeline
Layout constant: RIGHT TOP notes.txt in `-- VISUAL --`, the second line of the `find` row (`if dir, open child, if dir walk path`) highlighted, cursor line indicator `37`. It never changes in this segment. Webcam ignored.

Left terminal tabs: until 48:30 three tabs `~/lab — vim touch.c` | `~/lab — vim` | `~/lab — -zsh`. At 48:33 they read `vim touch.c` | `-zsh` | `-zsh` (middle tab now zsh, the scratch `vim` tab was closed). From 48:34 only two tabs: `~/lab — vim touch.c` and `~/lab — -zsh`.
IMPORTANT: the tab titled "vim touch.c" holds a buffer that is really the rm.c program being mutated into find.c (no touch.c content). `find.c` appears in `ls` at 48:35. The save/rename command is never visible.

- [44:21] vim buffer = rm.c-style code (function `walkrm`), cursor at line 41 `// if folder, walk into sub`. Bottom message `5 fewer lines` (a delete just happened). Visible lines 19-56 (see File states, rm version).
- [44:22] Line 41 `// if folder, walk into sub` highlighted (visual select) for deletion.
- [44:23] Lines 41-42 (the comment and `walkrm(new_path);`) deleted; else-branch is now `} else {` / `}` (lines 40-41). `// after remove all files child , we remove dir` at line 45; `if (rmdir(path)) {` 46. main top: `if (argc == 1) {` / `printf("Useage: rm [path]...\n");` (lines 55-56).
- [44:25] The comment line deleted; `if (rmdir(path)) {` at line 45, cursor line 46. main shows `return 0;` line 56 after printf.
- [44:26] Scrolled (lines 21-58). `if (argc == 1) {` line 54 (brace match highlight), `}` line 57.
- [44:28] `-- VISUAL LINE --` `3` : lines 45-47 selected (`if (rmdir(path)) {`, `perror(path);`, `return 1;`).
- [44:29] After delete: message `4 fewer lines`; cursor at line 44. File now: line 43 `}`, 44 blank (cursor), 45 blank, 46 `return 0;`, 47 `}`, 49 `int main(...)`; for loop at 55-58 (`for (int i = 1; i < argc; i++) {`, `const char *path = argv[i];`, `if (walkrm(path))`, `return 1;`).
- [44:30] `-- INSERT --` on line 45 (blank), about to type.
- [44:36] Lines 26-63 view: line 45 `closedir(dir);`, 47 `return 0;`, 48 `}`; main 50-63, call still `walkrm(path)` (line 58); cursor on final `}` line 63.
- [44:51] Usage string being edited: `printf("Useage: find [patter] [path]...\n");` (cursor on `[patter]`, bottom shows vim hint `Type  :qa!  and press <Ent...n all changes and exit Vim`, i.e. he pressed Ctrl-C). The call is still `walkrm(path)` here.
- [45:02] Scrolled up: lines 19-56; usage string confirmed `"Useage: find [patter] [path]...\n"`.
- [45:05] Top region: line 6 (cursor, blank), line 7 `int walkrm(const char *path) {`, lines 8-43 (see File states).
- [45:10] lines 10-48; cursor on final `}` line 48. Line 47 not displayed in this frame (rendering shows 45 `closedir(dir);`, 46 blank, 48 `}`). NOTE: this frame appears to be the file scrolled with main hidden; compare 45:11 where main exists at line 50. (Possible glitch; treat as unreliable.)
- [45:11] Lines 26-63: `if (walkfind(path))` (line 58) - the call renamed from `walkrm` to `walkfind`; usage still `argc == 1`.
- [45:29] INSERT: line 51 `if (argc < 3) {`; a new line 56 containing `c` (typing next statement); `for (int i = 1; i < argc; i++) {` pushed to line 58, `if (walkfind(path))` at line 60.
- [45:40] Line 56 `const char *patter = argv[1];`; line 57 `for (int i = 1; i < argc; i++) {`; line 59 `if (walkfind(path))`; `return 0;` line 63 (no closing `}` visible, cut off).
- [45:42] Scrolled (lines 27-64): same, cursor line 59; closing `}` line 64.
- [46:14] `for (int i = 2; i < argc; i++) {` (changed from 1). Call `if (walkfind(path, pattern))` (line 59). Declaration still `const char *patter = argv[1];` (name mismatch). Cursor highlight on the `0` in `strcmp(name, ".") == 0`.
- [46:15] Lines 7-44: `int walkfind(const char *path) {` (cursor on `(`), still single-parameter.
- [46:16] Lines 1-38: includes (lines 1-5, line 1 and 2 seen) and `int walkfind(const char *path) {` at line 7.
- [46:25] Lines 14-51; cursor on `{` of `if (argc < 3) {` line 51.
- [46:27] Lines 21-58: line 56 `const char *patter = argv[1];`, 57 `for (int i = 2; i < argc; i++) {`, 58 `const char *path = argv[i];` (cursor at the `;`).
- [46:28] Lines 27-64: line 59 `if (walkfind(path, pattern))`, 60 `return 1;`, 61 `}`, 63 `return 0;`, 64 `}` (cursor). Declaration still `patter` at 56.
- [46:57] Lines 16-53: cursor at line 16 (blank, top of view). Nothing changed.
- [46:58] Line 7 now `int walkfind(const char *path, const char *pattern) {` (second parameter added). Lines 3-40 visible; line 3 `#include <unistd.h>`.
- [47:35] INSERT inside `if (!S_ISDIR(st.st_mode)) {`: new line 15 (empty, indented); all following lines shift +1 (`DIR *dir` now at line 18, `}` closing at 16).
- [47:39] Line 15: `printf("` (typing).
- [47:43] Line 15: `printf("on check %s %s\n"`.
- [47:54] Line 15: `printf("on check %s %s\n", path, pattern);` completed (in the non-directory-of-parent... i.e. top-level `st` block). Lines 13-50; `}` line 49 closing function, line 50 cursor empty.
- [47:55] Lines 20-57: function closes at 49; main at 51; usage `if (argc < 3) {` 52-55; line 57 `const char *pattern = argv[1];` (variable renamed from `patter` to `pattern`).
- [47:59] INSERT mode: lines 20-57. Line 40 `if (!S_ISDIR(st_child.st_mode)) {`, line 41 `printf("on check %s %s\n", path, pattern);` with an over-indent (about 10 spaces), cursor on blank line 42, line 43 `} else {`, 44 `}`. (He pasted the printf into the child branch.)
- [48:00] Normal mode: line 40 `if (!S_ISDIR(st_child.st_mode)) {` (brace match highlighted with `}` at line 42 start of `} else {`), line 41 the over-indented printf, blank line removed.
- [48:19] INSERT: lines 20-57. Line 41 `printf("on check %s %s\n", name, pattern);` (indent fixed to 4 spaces, `path` replaced by `name`), `} else {` line 42, new line 43 with `w` typed (about to type `walkfind(new_path, pattern);`, not confirmed).
- [48:29] Switched to the `-zsh` tab (3rd): scrollback of earlier rm tests (see Shell).
- [48:30] Switched to the middle `vim` tab: scratch buffer, `-- INSERT --`, stray `:q` typed in line 5 (see File states).
- [48:32] Same buffer, message `E37: No write since last change (add ! to override)`.
- [48:33] Middle tab is now a zsh: history of mkdir tests and `man strchr` (see Shell).
- [48:34] Two tabs; zsh tab shows the rm test history again (the former 3rd tab).
- [48:35] zsh tab after `clear`: `ls` listing, prompt.
- [48:49] `cc find.c -o find` and `./find cat .` ran; output listed (see Shell). No compiler errors/warnings shown.
- [49:04] Screen cleared, empty prompt `lab $ `.

## File states

### find.c (in tab "vim touch.c"), most complete reconstruction as of 48:19
Line numbers follow the 48:19 frame where visible (lines 20-57). Lines 1-19 from 46:16/46:58 frames shifted +1 for the added printf (printf line 15). Marked as such.
```
1| #include <stdio.h>
2| #include <sys/stat.h>
3| #include <unistd.h>
4| #include <dirent.h>
5| #include <string.h>
6|
7| int walkfind(const char *path, const char *pattern) {
8|   struct stat st;
9|   if (lstat(path, &st)) {
10|     perror(path);
11|     return 1;
12|   }
13|
14|   if (!S_ISDIR(st.st_mode)) {
15|     printf("on check %s %s\n", path, pattern);
16|   }
17|
18|   DIR *dir = opendir(path);
19|   if (!dir) {
20|     perror(path);
21|     return 1;
22|   }
23|
24|   struct dirent *entry;
25|   while ((entry = readdir(dir)) != NULL) {
26|     const char *name = entry->d_name;
27|     if (strcmp(name, ".") == 0 || strcmp(name, "..") == 0)
28|       continue;
29|
30|     size_t path_len = strlen(path) + strlen(name) + 2;
31|     char new_path[path_len];
32|     snprintf(new_path, path_len, "%s/%s", path, name);
33|
34|     struct stat st_child;
35|     if (lstat(new_path, &st_child)) {
36|       perror(new_path);
37|       return 1;
38|     }
39|
40|     if (!S_ISDIR(st_child.st_mode)) {
41|       printf("on check %s %s\n", name, pattern);
42|     } else {
43|       w⟨being typed⟩
44|     }
45|
46|   }
47|
48|   closedir(dir);
49|
50|   return 0;
51| }
52|
53| int main(int argc, char *argv[]) {
54|   if (argc < 3) {
55|     printf("Useage: find [patter] [path]...\n");
56|     return 0;
57|   }
58|
59|   const char *pattern = argv[1];
60|   for (int i = 2; i < argc; i++) {          ?? (lines 58-66 not visible at 48:19; inferred from 46:28/47:55: line 57 in that frame = `const char *pattern = argv[1];`)
61|     const char *path = argv[i];
62|     if (walkfind(path, pattern))
63|       return 1;
64|   }
65|
66|   return 0;
67| }
```
Caveat: lines 14-16 and the printf at line 15 in the top-level `!S_ISDIR(st.st_mode)` block: at 47:54 the file shows line 15 printf with `path, pattern`; at 48:00-48:19 the frame starts at line 20 so line 15 is not visible; whether it was deleted/moved into the child branch at 47:59 is not confirmed (the line numbers in 48:19 -- `main` at 53 -- match the version WITH the line-15 printf kept, since 47:55 had main at 51 with line 15 present and one printf added in child gives +2... checked: 47:55 main=51 (before child printf), 48:00 main=53 -> child printf added 2 lines (printf + blank?), consistent with both printfs existing). So the top-level printf probably stayed.
Also: at 48:19 the child-branch `printf` line has no trailing blank; main at line 53.

Intermediate versions (key diffs):
- 44:21 rm.c version: function `int walkrm(const char *path) {` (line 7); else branch (lines 39-43):
```
39|     if (!S_ISDIR(st_child.st_mode)) {
40|     } else {
41|       // if folder, walk into sub
42|       walkrm(new_path);
43|     }
44|
45|   }
46|
47|   // after remove all files child , we remove dir
48|   if (rmdir(path)) {
49|     perror(path);
50|     return 1;
51|   }
52|
53|   return 0;
54| }
55|
56| int main(int argc, char *argv[]) {
57|   if (argc == 1) {
58|     printf("Useage: rm [path]...\n");   (seen at 44:23 as line 55-56 after deletions)
```
  (Lines 19-38 identical to find.c version except no pattern.) The earlier rm body also had `if (!S_ISDIR(st.st_mode)) {` / `}` empty at lines 14-15 (visible in 45:05), i.e. the file-removal (unlink) code is absent in this buffer (already stripped, or never present in this buffer).
- 44:29: lines 44-47 after deleting rmdir block; 44:36 `closedir(dir);` line 45.
- 45:11: call renamed to `walkfind`.
- 45:29: usage check `if (argc < 3) {`; 45:40: `const char *patter = argv[1];` inserted at line 56.
- 46:14: `for (int i = 2; ...` and `walkfind(path, pattern)`.
- 46:58: signature `int walkfind(const char *path, const char *pattern) {`.
- 47:54: top-level `printf("on check %s %s\n", path, pattern);`.
- 47:55: `const char *pattern = argv[1];`.
- 48:00: child printf `printf("on check %s %s\n", path, pattern);` over-indented.
- 48:19: child printf `printf("on check %s %s\n", name, pattern);`.

### Scratch notes buffer (tab `~/lab — vim`, no filename shown), 48:30-48:32 verbatim
```
1|
2|
3| rm abc.txt def.txt --> if all is file just remove it
4|
5| :qrm abc --> if is folder , we must remove all file child then remove folder abc
6| if have file is link -> nofollow it ,
7|
8|
9| to remove file , use unlink
```
Line 5 wraps in the display; the `:q` at the start of line 5 is stray literal text typed in insert mode; the `E37: No write since last change (add ! to override)` error at 48:32 confirms he tried to quit it.

### notes.txt (unchanged), highlighted line 37
```
|Command | Function use           | Algorithm
|touch   | fopen                  |
|cat     | fopen, fgetc, putchar  |   get path and do
|mkdir   | mkdir                  |
|mv      | stat, rename           | if target is file or not exit, rename
|        |                        | if target is dir , move
|rm      | lstat, unlink, rmdir   | walk path, if file remove
|        | opendir, readdir       | if dir, open child, if dir walk path
|find    | lstat, strstr          | walk path, if match print
|        | opendir, readdir       | if dir, open child, if dir walk path   <-- highlighted
|tree    |                        | watch my previous video
```
(column spacing approximate; separators are `+----+` lines.)

### touch.c, cat.c, mkdir.c, mv.c, rm.c
Not edited/viewed in this segment (only listed by `ls`).

## Shell & outputs
- [48:29] zsh tab scrollback (prompt shown as `lab $`):
```
lab $ ./rm rm
rm: No such file or directory
lab $ ls
cat.c     mkdir.c   mv.c      rm.c
mkdir     mv        notes.txt touch.c
lab $ cc rm.c -o rm
lab $ ./rm mkdir
mkdir: No such file or directory
lab $ ls
cat.c     mv        notes.txt rm.c
mkdir.c   mv.c      rm        touch.c
lab $ # use fix this bug
lab $
```
  Interpretation: `./rm rm` deleted the rm binary itself (then `rm: No such file or directory` is the second-pass error, or the first walk), then recompiled `cc rm.c -o rm`, `./rm mkdir` deleted the `mkdir` binary and printed `mkdir: No such file or directory` (a bug: error printed even though removal worked, plausibly because after unlinking, the code falls through to opendir/lstat on the removed path). Third listing shows `mkdir` binary gone. He notes `# use fix this bug`.
- [48:33] other zsh tab scrollback:
```
lab $ cc mkdir.c -o mkdir
lab $ ./mkdir abc
lab $ ls
abc       cat       def.txt   mkdir.c   touch     xyz.txt
abc.txt   cat.c     mkdir     notes.txt touch.c
lab $ ./mkdir abc/def xyz
lab $ ls
abc       cat       def.txt   mkdir.c   touch     xyz
abc.txt   cat.c     mkdir     notes.txt touch.c   xyz.txt
lab $ ls abc
def
lab $ e
lab $ man strchr
lab $ e
lab $
```
  (`e` is not a standard command; probably a typo or alias; `abc`, `def`, `xyz` directories shown colored cyan.)
- [48:35]
```
lab $ ls
cat.c     mkdir.c   mv.c      rm        touch.c
find.c    mv        notes.txt rm.c
lab $
```
- [48:49]
```
lab $ ls
cat.c     mkdir.c   mv.c      rm        touch.c
find.c    mv        notes.txt rm.c
lab $ cc find.c -o find
lab $ ./find cat .
on check notes.txt cat
on check .find.c.swp cat
on check rm.c cat
on check cat.c cat
on check .notes.txt.swp cat
on check mkdir.c cat
on check mv cat
on check mv.c cat
on check find cat
on check touch.c cat
on check find.c cat
on check rm cat
lab $
```
  No compiler warnings/errors visible. The `.find.c.swp` file proves find.c is open in vim while running. The `.` entries aren't printed, and the line is printed by the child branch (name, pattern) with bare file names (not full paths).
- [49:04] screen cleared, `lab $ `.

## Observations
- Build order: rm.c -> find.c (find row of notes table highlighted). He copies the finished rm walker and strips rm-specific logic (comments, rmdir, unlink) to bootstrap find; renames `walkrm` -> `walkfind`, adds `pattern` argument and shifts argv indexing (`argc < 3`, `i = 2`, `pattern = argv[1]`).
- Incremental debugging: inserts `printf("on check %s %s\n", ...)` to see traversal before implementing the `strstr` match; compiles `cc find.c -o find` and runs `./find cat .`.
- Style: 2-space indent, `perror(path); return 1;` error style, VLA for the child path, `snprintf` with `%s/%s`, skips `.` and `..` via `strcmp`, comments in broken English (`// after remove all files child , we remove dir`, `// if folder, walk into sub`), typos left in strings (`Useage`, `patter`).
- rm tests: `./rm rm` and `./rm mkdir` deleted binaries; he left shell comment `# use fix this bug` (note-to-self about an rm bug where an error message is printed after removal).
- Scratch notes buffer (`vim` tab) for rm: files just removed; folder -> remove all children then folder; links -> nofollow (lstat); use `unlink` for files.
- Man pages: `man strchr` typed in history at 48:33 (page contents not shown in this segment; probably looking for substring search, ends up using strstr per notes table).
- No vim save messages (`written`) visible in this segment.
