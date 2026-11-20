# Segment 3 (07:14 - 11:41) transcript

Context: he is writing `mv.c`. The vim tab title stays `~/lab — vim touch.c` the whole segment, but the first save message is `"mv.c" [New] 37L, 658B written` (10:33), so the buffer was the touch.c/mkdir-style code, stripped and saved as mv.c (e.g. `:w mv.c`). Tab title never updated. Webcam ignored.

## Timeline
- [07:14] Tabs: `~/lab — vim touch.c` | `~/lab — vim` (active). Scratch notes buffer (normal mode, cursor on empty line 5):
  ```
  1|
  2| Use 1 , file2
  3|
  4| mv file1.txt file2.txt ---> it is rename
  5|
  6|
  7|
  8| not this
  9| mv file1.txt file2.txt file3.txt ---> error (3 file)
  10|
  11|
  12| use 2 -> move to folder - folder must exit
  13|
  14| mv file1.txt file2.txt abc
  15| mv file1.txt abc
  16| mv abc def
  ```
  notes.txt (top right) shows the design table, no row highlighted.
- [07:25] Switched to tab `vim touch.c`. Holds the old mkdir-style program (20 lines, see "initial buffer"), but usage text is already `Usage: mv <path> <target>\n` / `     : mv <path>... <dir>\n`. Cursor on `(` of line 6 printf (paren match highlighted).
- [07:27] `-- VISUAL LINE --` starting at line 11 (`for (int i = 1; ...`), count `1`; matching `}` line 17 highlighted.
- [07:28] VISUAL LINE selection lines 11-17, count `7` (the whole for loop).
- [07:29] After delete: status `7 fewer lines`. File is 13 lines: blank 10, 11; `return 0;` line 12; `}` line 13. Cursor line 9.
- [07:46] INSERT. Line 11 `  const char *target = argv[argc - 1];`, line 12 `  argc--; // rem` (typing). 16 lines.
- [08:00] Line 12 `  argc--; // remove target from path`, line 13 `          //`. 19 lines, INSERT, cursor box on line 15. Focus moved to notes.txt window (left dimmed).
- [08:03] notes.txt scrolled to "Helper" section; VISUAL mode count `14`; grey bar at very top of window (selected/highlighted line at line 1, off-screen content). Content: see File states.
- [08:04] Scrolled further (Helper at screen row 164), VISUAL count `17`.
- [08:06] VISUAL selection on `check path, if file is link, follow link` part of the `- stat :` line, count `37` (he is explaining stat vs lstat).
- [08:08] Selection cleared; mouse at end of `// not have file`.
- [08:09] VISUAL selection over three lines `struct stat st;` / `stat(path, &st) == -1   // have error` / `- errno == ENOENT       // not have file`, count `3`.
- [08:25] Back in vim touch.c INSERT; lines 15-16: `  struct stat st;` / `  if (stat(p` (typing). 20 lines total. notes.txt selection (3 lines) persists.
- [08:43] Lines 15-19: `struct stat st;`, `if (stat(target, &st)) {`, `    // have something error`, `    //`, `    // if error is file not` (typing); `}` line 20. 24 lines total.
- [08:59] Scratch tab `~/lab — vim` INSERT: line 5 now `file2 not exit ` (typed).
- [09:03] Scratch tab, `-- (insert) VISUAL --` count `52`; line 9 selected (`mv file1.txt file2.txt file3.txt ---> error (3 file)`).
- [09:11] touch.c INSERT: line 19 `    // if error is file not exit`; line 20 `    if (errno == ENOENT && ` (typing). 25 lines.
- [09:35] Scratch tab `-- (insert) VISUAL --`, lines 8-9 (`not this` + the 3-file line) selected, pending key `m` in corner.
- [09:36] New third tab: `~/lab — vim touch.c` | `~/lab — vim` | `~/lab — -zsh`. Prompt `lab $ man` (typing).
- [09:38] Tab 3 title `...less ◂ man rename`: `RENAME(2)  System Calls Manual` (section 2). NAME: `rename, renameat, renamex_np, renameatx_np — change the name of a file`. SYNOPSIS: `#include <stdio.h>`; `int rename(const char *old, const char *new);` + renameat / renamex_np / renameatx_np variants. DESCRIPTION (visible): "The rename() system call causes the link named old to be renamed as new. If new exists, it is first removed. Both old and new must be of the same type (that is, both must be either directories or non-directories) and must reside on the same file system. The rename() system call guarantees that an instance of new will always exist, even if the system should crash in the middle of the operation. If the final component of old is a symbolic link, the symbolic link is renamed, not the file or directory to which it points." less prompt `:`.
- [09:46] touch.c INSERT (tabs now `vim touch.c` | `vim` | `...less • man rename`): line 20 `    if (errno == ENOENT && argc == ⟨?⟩) {` (mouse cursor over the digit), line 21 `      // do rename`, line 22 `      rename(`, `}` 23, `}` 24 ... 28 lines total.
- [10:02] Line 20 `if (errno == ENOENT && argc == 2) {`; 22 `const char *path = argv[1];`; 23 `if (rename(path, target))` with paren match. 29 lines.
- [10:20] Rename block complete (lines 23-29: `if (rename(path, target)) {` / `perror(path);` / `return 1;` / `}` / blank / `return 0;` / `}`). 34 lines; `{` of line 20 highlighted.
- [10:33] `perror(target); return 1;` and closing `}` added (see mv.c 10:33). 37 lines. Normal mode. Status: `"mv.c" [New] 37L, 658B written`.
- [10:36] Tab 3 (man rename): same page; `new` in `*new` of synopsis highlighted with grey block (cursor); `:` prompt.
- [10:37] Tab 3 now `~/lab — -zsh` with empty prompt `lab $ `.
- [10:41] Compile errors (see Shell).
- [10:43] Back to vim touch.c, normal mode, unchanged, same status message.
- [10:45] INSERT: blank line added at line 3 (all lines shifted +1; 38 lines); `#include <errno.h>` not yet typed.
- [10:50] zsh tab shows previous errors again (not yet re-run).
- [10:53] zsh: re-ran `cc mv.c -o mv` (clean), then `ls`.
- [11:07] `./mv xyz.txt my-xyz.txt`, `ls` shows rename succeeded.
- [11:13] vim: line 3 `#include <errno.h>` (end `>` shown highlighted); status `"mv.c" 38L, 677B written`.
- [11:17] Right window active: notes.txt back at table top (title "Build linux files utils in C"); VISUAL count `20` with the bottom `+----` line of table highlighted.
- [11:18] Mouse over `stat, rename` cell of mv row; no selection.
- [11:21] VISUAL (count `39`) highlights mv row algorithm line 1: `if target is file or not exit, rename`.
- [11:25] vim INSERT: new blank lines opened after line 34 `}`; cursor at line 37 (indented col). 38 lines then.
- [11:26] View scrolled (19-41): blank lines 35-39, `return 0;` line 40, `}` line 41, cursor line 37, INSERT.
- [11:39] Typed lines 37-38: `  // if dir not exit, return error` and `  // usecase 2`; line 39 cursor box. 43 lines. INSERT. notes.txt still highlights mv row.
- [11:40] notes.txt scrolled to Helper section, VISUAL count `17`, grey bar at top (a long highlighted line at line 1). Left unchanged.
- [11:41] Helper VISUAL count `1`, small grey block under `S_ISDIR(st.st_mode)` line. Left unchanged. (Heading toward the S_ISDIR directory case.)

## File states

### Initial buffer (07:25), tab `vim touch.c`, mkdir-style, before edits
```
1| #include <stdio.h>
2| #include <sys/stat.h>
3|
4| int main(int argc, char *argv[]) {
5|   if (argc == 1) {
6|     printf("Usage: mv <path> <target>\n");
7|     printf("     : mv <path>... <dir>\n");
8|     return 0;
9|   }
10|
11|   for (int i = 1; i < argc; i++) {
12|     const char *path = argv[i];
13|     if (mkdir(path, 0777)) {
14|       perror(path);
15|       return 1;
16|     }
17|   }
18|
19|   return 0;
20| }
```

### mv.c at 10:33 (`"mv.c" [New] 37L, 658B written`, no errno.h yet)
```
1| #include <stdio.h>
2| #include <sys/stat.h>
3|
4| int main(int argc, char *argv[]) {
5|   if (argc == 1) {
6|     printf("Usage: mv <path> <target>\n");
7|     printf("     : mv <path>... <dir>\n");
8|     return 0;
9|   }
10|
11|   const char *target = argv[argc - 1];
12|   argc--; // remove target from path
13|           //
14|
15|   struct stat st;
16|   if (stat(target, &st)) {
17|     // have something error
18|     //
19|     // if error is file not exit
20|     if (errno == ENOENT && argc == 2) {
21|       // do rename
22|       const char *path = argv[1];
23|       if (rename(path, target)) {
24|         perror(path);
25|         return 1;
26|       }
27|
28|       return 0;
29|     }
30|
31|     perror(target);
32|     return 1;
33|   }
34|
35|
36|   return 0;
37| }
```

### mv.c at end of segment (11:41; last write `"mv.c" 38L, 677B written` at 11:13; lines 37-43 are unsaved edits)
```
1| #include <stdio.h>
2| #include <sys/stat.h>
3| #include <errno.h>
4|
5| int main(int argc, char *argv[]) {
6|   if (argc == 1) {
7|     printf("Usage: mv <path> <target>\n");
8|     printf("     : mv <path>... <dir>\n");
9|     return 0;
10|   }
11|
12|   const char *target = argv[argc - 1];
13|   argc--; // remove target from path
14|           //
15|
16|   struct stat st;
17|   if (stat(target, &st)) {
18|     // have something error
19|     //
20|     // if error is file not exit
21|     if (errno == ENOENT && argc == 2) {
22|       // do rename
23|       const char *path = argv[1];
24|       if (rename(path, target)) {
25|         perror(path);
26|         return 1;
27|       }
28|
29|       return 0;
30|     }
31|
32|     perror(target);
33|     return 1;
34|   }
35|
36|
37|   // if dir not exit, return error
38|   // usecase 2
39|   ⟨empty, cursor⟩
40|
41|
42|   return 0;
43| }
```
(The frame at 11:39 shows lines 19-43 only; lines 1-18 taken from the 11:13/11:17 frames. Line 13 `//` is aligned under the `//` of line 12.)

### Scratch notes buffer (tab `~/lab — vim`) as of 09:35
```
1|
2| Use 1 , file2
3|
4| mv file1.txt file2.txt ---> it is rename
5| file2 not exit
6|
7|
8| not this
9| mv file1.txt file2.txt file3.txt ---> error (3 file)
10|
11|
12| use 2 -> move to folder - folder must exit
13|
14| mv file1.txt file2.txt abc
15| mv file1.txt abc
16| mv abc def
```
(07:14 version identical except line 5 empty.)

### notes.txt
Top table (unchanged; right edge is cut off in the frame, border widths approximate):
```
Build linux files utils in C

+---------+-------------------------+------------------------------------+
|Command  | Function use            |            Algorithm
+---------+-------------------------+
|touch    | fopen                   |
+---------+-------------------------+     get path and do
|cat      | fopen, fgetc, putchar   |
+---------+-------------------------+
|mkdir    | mkdir                   |
+---------+-------------------------+----------------------------------
|mv       | stat, rename            | if target is file or not exit, rename
|         |                         | if target is dir , move
+---------+-------------------------+----------------------------------
|rm       | lstat, unlink, rmdir    | walk path, if file remove
|         | opendir, readdir        | if dir, open child, if dir walk path
+---------+-------------------------+----------------------------------
|find     | lstat, strstr           | walk path, if match print
|         | opendir, readdir        | if dir, open child, if dir walk path
+---------+-------------------------+----------------------------------
```
Helper section (further down in same file; lines between table and Helper not visible):
```
Helper
- stat : check path, if file is link, follow link
- lstat: check path, if file is link, return link

struct stat st;
stat(path, &st) == -1   // have error
- errno == ENOENT       // not have file

S_ISDIR(st.st_mode)     // is dir
```

## Shell & outputs
- [09:36] new tab, `lab $ man` ... then `man rename` (less, RENAME(2)).
- [10:41]
```
lab $ cc mv.c -o mv
mv.c:20:9: error: use of undeclared identifier 'errno'
   20 |     if (errno == ENOENT && argc == 2) {
      |         ^~~~~
mv.c:20:18: error: use of undeclared identifier 'ENOENT'
   20 |     if (errno == ENOENT && argc == 2) {
      |                  ^~~~~~
2 errors generated.
lab $
```
- [10:53]
```
lab $ cc mv.c -o mv
lab $ ls
abc        cat.c     mkdir.c   notes.txt  xyz
abc.txt    def.txt   mv        touch      xyz.txt
cat        mkdir     mv.c      touch.c
lab $
```
(compile silent = success; abc, xyz are dirs (cyan); cat, mkdir, mv, touch executables (red))
- [11:07]
```
lab $ ./mv xyz.txt my-xyz.txt
lab $ ls
abc        cat.c     mkdir.c   my-xyz.txt  touch.c
abc.txt    def.txt   mv        notes.txt   xyz
cat        mkdir     mv.c      touch
lab $
```
(xyz.txt gone, my-xyz.txt present: rename case works.)
- vim messages: 07:29 `7 fewer lines`; 10:33 `"mv.c" [New] 37L, 658B written`; 11:13 `"mv.c" 38L, 677B written`.

## Observations
- Build order context: cat.c, mkdir.c, touch.c (and binaries) already exist in ~/lab; now mv.c, bootstrapped by editing the mkdir-style buffer: VISUAL LINE select lines 11-17, delete (`7 fewer lines`), write new logic, `:w mv.c`.
- Design for mv: last argv is target, `argc--` removes it from the source list. `stat(target,&st)` failing with ENOENT and exactly one source (`argc == 2` after decrement) => `rename(path, target)` (scratch use case 1: file2 not exist = rename). Otherwise `perror(target); return 1`. Next step (as of 11:41): `// if dir not exit, return error` / `// usecase 2` (move into folder; he heads to S_ISDIR in the Helper notes). 3-file case with non-dir target should be an error (scratch notes).
- He keeps a scratch vim buffer of use cases and a Helper section in notes.txt (stat vs lstat, ENOENT, S_ISDIR) and uses VISUAL highlighting in notes.txt to point at what he is explaining; copies code from the helper into the source.
- Comment-first style: writes `// have something error`, `// if error is file not exit`, `// do rename`, `// if dir not exit, return error`, `// usecase 2` before code. Spelling "exit" for "exist" everywhere.
- Error handling: `if (syscall(...)) { perror(path); return 1; }`, treating 0/nonzero return as boolean; usage printed and `return 0` when argc == 1.
- Man page: `man rename` (section 2, macOS/BSD) in third tab to check parameter order (old, new); he highlighted `*new`. Compiler (clang) errors led to adding `#include <errno.h>`.
- Style: 2-space indent, K&R braces, `//` comments, side-effect `argc--;`. Test loop: `cc mv.c -o mv`, `ls`, `./mv xyz.txt my-xyz.txt`, `ls`.
- Tab title `vim touch.c` persists despite saving as mv.c.
