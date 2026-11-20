# Segment 12 (41:24 - 44:21)

Tabs (left terminal): `~/lab — vim touch.c` | `~/lab — vim` | `~/lab — -zsh`. The first tab title stays "vim touch.c" even though the file edited is rm.c (title not updated; unreliable).

## Timeline
- [41:24] Tab 3 `-zsh` active. Scrollback shows `cc rm.c -o rm`, `./rm abc` (output `path abc/cat`, `path abc/def`: an earlier debug print), `man rmdir`, `man 3 rmdir` -> `No manual entry for rmdir`, `man 2 rmdir`, `cc rm.c -o rm`, `ls`, `ls abc`, `./rm abc`, `ls` (abc gone). Right terminal: notes.txt, `-- VISUAL --`, counter `38`, highlight on rm row line 2 cell `if dir, open child, if dir walk path`.
- [41:29] `./rm xyz`, `ls` -> xyz gone.
- [41:46] `./rm touch mkdir mv` -> `touch: No such file or directory`; `ls` -> `touch` binary gone, `mkdir` and `mv` still present (processing stopped after the first error).
- [41:56] Switch to tab 1 (vim, actually rm.c). Status line `"rm.c" 81L, 1562B written`. Viewing lines 31-68; cursor on last char of `dir` in line-59 comment.
- [41:58-42:01] Scrolling up (normal mode): lines 20-56, 8-44, 7-43, 3-39 (cursor on `>` of `#include <unistd.h>`, line 3).
- [42:07] Brief switch to -zsh (mouse on tab, tab shows "x"); same scrollback. [42:14] back to vim, lines 3-39.
- [42:24] INSERT mode: new blank line opened inside `if (lstat(path, &st)) {` block (line 10; `perror(path);` now line 11). `-- INSERT --`.
- [42:26] Typing `printf("in ` on line 10.
- [42:31] Line 10 reads `printf("on this case\n")` (typed text changed from "in ..."; `;` not typed yet).
- [42:33] -zsh tab: prompt shows recalled `lab $ ./rm touch mkdir mv ` (not yet run; after a rebuild presumably).
- [42:39] Shell: `cc rm.c -o rm` then `./rm touch mkdir mv` -> `on this case` / `touch: No such file or directory`.
- [42:55] Screen cleared; typing `./rm`.
- [43:04] `./rm rm` -> `rm: No such file or directory`; `ls` -> `cat.c mkdir.c mv.c rm.c / mkdir mv notes.txt touch.c` (rm binary removed itself; no printf output, so lstat succeeded and the error came from the later opendir).
- [43:15] `cc rm.c -o rm`, `./rm mkdir` -> `mkdir: No such file or directory`; `ls` -> `cat.c mv notes.txt rm.c / mkdir.c mv.c rm touch.c` (mkdir binary gone).
- [43:28] Vim tab: `"rm.c" 82L, 1592B written` (debug printf line saved; line 10 `printf("on this case\n");`, cursor on `;`). A pending `g` visible at right of command line.
- [43:29] `-- VISUAL LINE --`, count `82`: whole file selected (gg V G), showing lines 45-82 (end of file, main).
- [43:30] After yank, `:e` typed at command line (file name not visible yet); view jumped to lines 1-37 with `#include <stdio.h>` on line 1.
- [43:33] Message `82 more lines`: in the new empty buffer the 82 yanked lines were pasted below blank line 1 (content now lines 2-83).
- [43:34] Blank line 1 deleted; file starts at line 1 `#include <stdio.h>`; cursor on `#`. New file name NOT visible (likely find.c next in notes order; unverified).
- [43:35] Scrolled, lines 36-73.
- [43:44] Focus moved to the right terminal (left window greyed): VISUAL highlight gone, mouse pointer at `find` row. Counter gone.
- [43:47] Right: mouse-drag VISUAL selection over find row (counter `2`).
- [43:49] Right: selection collapsed, cursor between `match` and `print`.
- [43:52] Right: VISUAL highlight on find row line 2 cell `if dir, open child, if dir walk path` (counter `37`).
- [43:55] Focus back to left; lines 35-72 (cursor block on blank line 35).
- [43:56-43:57] Scrolling up in the new buffer: lines 19-55 then 6-42 (cursor on line 6). Debug printf still present at line 10 at 43:57.
- [43:59] Line 10 `printf("on this case\n");` deleted -> line 10 is `perror(path);` (cursor on `p`); `int walkrm` still line 7.
- [44:07] Comment `// it is file` deleted; line 14 `if (!S_ISDIR(st.st_mode)) {`, line 15 `if (unlink(path)) {` (cursor on `i`).
- [44:08] `-- VISUAL LINE --` over lines 15-17 (count 3), extended to include `}` (lines 15-18).
- [44:10] After delete: `4 fewer lines`. Lines 14-15 now `if (!S_ISDIR(st.st_mode)) {` / `}`; comment `// rm abc --> ...` on line 17.
- [44:11] Cursor on line 17 col 0 region (`DIR` line 17 visible as `IR` w/ cursor over D... the DIR line is line 18? see File states; shown numbering: 15 `}`, 16 blank, 17 comment (wrapped), 18 `DIR *dir...`).
- [44:13] Scrolled to lines 19-56 (cursor block on line 56).
- [44:15] Lines 19-56; the `// we have name, now get path` comment has been removed (line 28 blank, 29 `size_t path_len...`), cursor on `s` of `size_t`.
- [44:19] `-- VISUAL LINE --` count 1 on line 40 `// is file just remove it`.
- [44:20] Extended to 5 lines (40-44): comment + `if (unlink(new_path)) {` ... `}`.
- [44:21] After delete: `5 fewer lines`. Lines 39-40 now `if (!S_ISDIR(st_child.st_mode)) {` / `} else {` (empty if-branch); file now 56 lines (`int main` on line 56). Segment ends.

## File states

### notes.txt (right terminal; unchanged this segment, only highlighted)
```
|Command | Function use           |                Algorithm
+--------+------------------------+------------------------
|touch   | fopen                  |
|cat     | fopen, fgetc, putchar  |   get path and do
|mkdir   | mkdir                  |
|mv      | stat, rename           | if target is file or not exit, rename
|        |                        | if target is dir , move
|rm      | lstat, unlink, rmdir   | walk path, if file remove
|        | opendir, readdir       | if dir, open child, if dir walk path
|find    | lstat, strstr          | walk path, if match print
|        | opendir, readdir       | if dir, open child, if dir walk path
|tree    |                        | watch my previous video
```
(Column spacing approximate; separators `+---` lines between rows.) Highlight: rm row line 2 (41:24-43:35); find row (43:47-43:57).

### rm.c, version at 41:56 (`"rm.c" 81L, 1562B written`), lines 1-2 not visible
Indent width 2. Lines 3-67 as seen at 41:56-42:14; lines 68+ partly (main at line 68).
```
 1| ?? (not visible at this version; at 43:30 = #include <stdio.h>)
 2| ?? (at 43:30 = #include <sys/stat.h>)
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
59|   // after remove all files child , we remove dir
60|   if (rmdir(path)) {
61|     perror(path);
62|     return 1;
63|   }
64|
65|   return 0;
66| }
67|
68| int main(int argc, char *argv[]) {
```
(The line-22 comment is one long source line wrapped by the terminal at "child th|en remove folder abc".)

### rm.c final version of this segment (82L, 1592B, saved 43:28)
Same as above with `printf("on this case\n");` inserted as new line 10 (all later lines +1), and main (seen 43:29, line numbers as of 82L):
```
69| int main(int argc, char *argv[]) {
70|   if (argc == 1) {
71|     printf("Useage: rm [path]...\n");
72|     return 0;
73|   }
74|
75|   for (int i = 1; i < argc; i++) {
76|     const char *path = argv[i];
77|     if (walkrm(path))
78|       return 1;
79|   }
80|
81|   return 0;
82| }
```
Header (43:30): `1| #include <stdio.h>`, `2| #include <sys/stat.h>`, then unistd.h, dirent.h, string.h as above. `walkrm` at line 8, comment `// rm abc --> ...` line 24.

### New buffer (copy of rm.c; name not visible), state at END (44:21)
Line numbers from 44:11 and 44:21 frames (lines 1-6 presumed unchanged includes + blank; not re-seen after edits).
```
 6|
 7| int walkrm(const char *path) {
 8|   struct stat st;
 9|   if (lstat(path, &st)) {
10|     perror(path);
11|     return 1;
12|   }
13|
14|   if (!S_ISDIR(st.st_mode)) {     <- (44:11 state; this block's later fate not seen)
15|   }
16|
17|   // rm abc --> if is folder , we must remove all file child then remove folder abc
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
41|       // is file just remove it          <- deleted at 44:21 (5 lines 40-44 incl. unlink block)
```
Exact 44:21 frame (lines 19-56; this is after the 5-line delete, and shows that numbering was 1 lower than 44:11 listing because the earlier `// we have name` comment line was also deleted):
```
19|     perror(path);
20|     return 1;
21|   }
22|
23|   struct dirent *entry;
24|   while ((entry = readdir(dir)) != NULL) {
25|     const char *name = entry->d_name;
26|     if (strcmp(name, ".") == 0 || strcmp(name, "..") == 0)
27|       continue;
28|
29|     size_t path_len = strlen(path) + strlen(name) + 2;
30|     char new_path[path_len];
31|     snprintf(new_path, path_len, "%s/%s", path, name);
32|
33|     struct stat st_child;
34|     if (lstat(new_path, &st_child)) {
35|       perror(new_path);
36|       return 1;
37|     }
38|
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
```
Note: the 44:11 frame numbers (e.g. DIR at line 18/17-comment) vs 44:21 (opendir not visible; line 19 = `perror(path);`) are consistent with `opendir` block at 17-21: i.e. 44:21 line 19 `perror(path);` follows `if (!dir) {` on 18, `DIR *dir = opendir(path);` on 17, comment on 16 (wrapped) - the file lost one more line between 44:11 and 44:15 (the `// we have name` comment line deleted: 44:15 shows line 28 blank / 29 size_t with same upper numbering, so the numbering at 44:15 matches 44:11+0 above line 28... UNRESOLVED +/-1 on lines 14-22; do not rely on exact numbers above line 23).

## Shell & outputs
```
[41:24] lab $ cc rm.c -o rm
        lab $ ./rm abc
        path abc/cat
        path abc/def
        lab $ man rmdir
        lab $ man 3 rmdir
        No manual entry for rmdir
        lab $ man 2 rmdir
        lab $ cc rm.c -o rm
        lab $ ls
        abc       mkdir     mv        notes.txt rm.c      touch.c
        cat.c     mkdir.c   mv.c      rm        touch     xyz
        lab $ ls abc
        cat def
        lab $ ./rm abc
        lab $ ls
        cat.c     mkdir.c   mv.c      rm        touch     xyz
        mkdir     mv        notes.txt rm.c      touch.c
[41:29] lab $ ./rm xyz
        lab $ ls
        cat.c     mkdir.c   mv.c      rm        touch
        mkdir     mv        notes.txt rm.c      touch.c
[41:46] lab $ ./rm touch mkdir mv
        touch: No such file or directory
        lab $ ls
        cat.c     mkdir.c   mv.c      rm        touch.c
        mkdir     mv        notes.txt rm.c
[42:39] lab $ cc rm.c -o rm
        lab $ ./rm touch mkdir mv
        on this case
        touch: No such file or directory
[42:55] (cleared) lab $ ./rm
[43:04] lab $ ./rm rm
        rm: No such file or directory
        lab $ ls
        cat.c     mkdir.c   mv.c      rm.c
        mkdir     mv        notes.txt touch.c
[43:15] lab $ cc rm.c -o rm
        lab $ ./rm mkdir
        mkdir: No such file or directory
        lab $ ls
        cat.c     mv        notes.txt rm.c
        mkdir.c   mv.c      rm        touch.c
```
(Colors: directories cyan, executables red in `ls`.) Vim messages: `"rm.c" 81L, 1562B written` [41:56]; `"rm.c" 82L, 1592B written` [43:28]; `82 more lines` [43:33]; `4 fewer lines` [44:10]; `5 fewer lines` [44:21].

## Observations
- Test method: destructive test of rm on scratch dirs (`abc` with `cat`, `def`; `xyz`) and then on his own built binaries (touch, mkdir, mv, rm itself) as disposable files. Earlier debug print `path abc/cat` already removed from source by 41:56.
- Known bug exposed: for a plain file, walkrm unlinks it and then FALLS THROUGH to opendir, which fails and prints `<name>: No such file or directory`, returning 1 (so `./rm touch mkdir mv` removed only `touch` and aborted). Explained by the `on this case` debug print: on the second run touch was already gone, so lstat failed and the printf (placed in the lstat-error branch) fired. He did not fix the fall-through in rm.c within this segment (as far as visible); `./rm rm` and `./rm mkdir` repeat the behavior (unlink succeeds, then error).
- man usage: `man rmdir` (section 1) then `man 3 rmdir` (nothing) then `man 2 rmdir` (syscall) before 41:24: he tries section numbers to find the C API page.
- Reuse strategy: at 43:29-43:34 he yanks all of rm.c (`ggVG` + yank), `:e <new file>`, pastes, deletes blank line 1, then prunes rm-specific code (debug printf, the early-file unlink block, `// it is file`, the unlink branch for children). Right terminal highlight moves from rm row to find row (43:47-43:57), indicating the new file is find (walk path skeleton shared: "if dir, open child, if dir walk path"). File name of the new buffer is not visible in this segment.
- Style: 2-space indent; `//` comments in loose English with spaces before commas (`remove all files child , we remove dir`); VLA `char new_path[path_len]` with `snprintf(... "%s/%s" ...)`; error handling `perror(path); return 1;` everywhere; usage string `Useage: rm [path]...` (sic) returning 0 when `argc == 1`; `for (int i = 1; i < argc; i++)` aborting on first failure; recursive call result ignored (`walkrm(new_path);`).
- Vim habits: scrolling by Ctrl-U/Ctrl-D style jumps, `V` + `j` + `d` for line deletions, mouse for switching windows and highlighting notes rows, tab title not updated after `:e`.
