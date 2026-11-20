# Segment 8 (28:37 - 31:29): rm.c (written in the buffer tab titled `vim touch.c`)

Tabs: tab1 `~/lab — vim touch.c` (actually editing rm.c; the buffer was saved as "rm.c"), tab2 `~/lab — vim` (scratch notes), tab3 `-zsh` / `less < man 2 unlink` / `less < man opendir`.

## Timeline

- [28:37] Tab3 shows `man 2 unlink` (UNLINK(2) System Calls Manual) in less. `unlink(const char *path);` line highlighted (less search/highlight). Also visible: `unlinkat(int fd, const char *path, int flag)`, AT_REMOVEDIR. Prompt `:` at bottom. notes.txt: cursor (box) at "re" of "remove" in rm row line 1.
- [28:38] Tab1 `vim touch.c`, INSERT mode. Buffer is rm.c, 35 lines. Cursor at end of line 2 `#include <sys/stat.h>` (about to add `#include <unistd.h>` for unlink). Code: walkrm with lstat + unlink for non-dir; main loop. (full text below)
- [28:39] INSERT, new line 3 being typed: `#i` (becomes `#include <unistd.h>`). File now 36 lines.
- [28:57] Tab2 `~/lab — vim` scratch buffer, INSERT mode, cursor at end of line 9 `to remove file , use unlink`. Contents (lines 1-9):
  1| (empty)
  2| (empty)
  3| rm abc.txt def.txt --> if all is file just remove it
  4| (empty)
  5| rm abc --> if is folder , we must remove all file child then re(wrap)move folder abc
  6| if have file is link -> nofollow it ,
  7| (empty)
  8| (empty)
  9| to remove file , use unlink
- [28:58] Tab3 `-zsh` (was earlier session history). Output:
  ```
  lab $ ls
  abc        cat.c      mkdir.c    mv.c       touch      xyz
  cat        mkdir      mv         notes.txt  touch.c
  lab $ ./mv cat abc
  lab $ ls
  abc        mkdir      mv         notes.txt  touch.c
  cat.c      mkdir.c    mv.c       touch      xyz
  lab $ ls abc
  abc.txt    cat        def        def.txt    y-xyz.txt
  lab $ # ok mv donema
  lab $ man 2 unlink
  lab $
  ```
  (colors: abc, xyz, def cyan = dirs; cat, mkdir, mv, touch, rm executables red). `ls` columns as seen; spacing approximate.
- [28:59] Tab3: prompt `lab $ cc ` being typed (screen cleared).
- [29:03] `lab $ cc rm.c -o rm`, then `lab $ ls` -> `abc mkdir mv notes.txt rm.c touch.c / cat.c mkdir.c mv.c rm touch xyz`. Typing `./`.
- [29:14] `lab $ ./rm abc/def.txt ` ; tab completion listing shown: `abc/ mkdir* mv* notes.txt rm.c touch.c / cat.c mkdir.c mv.c rm* touch* xyz/`.
- [29:17] `lab $ ./rm abc/def.txt abc/y-xyz.txt ` ; tab completion of abc/: `abc.txt cat* def/ def.txt y-xyz.txt`.
- [29:34] Command executed (no output, success). Then `lab $ ls abc` -> `abc.txt cat def` (def.txt and y-xyz.txt gone; def is dir). Then `lab $ ./rm abc/abc.txt` (executed, no output). Webcam: speaker leaning; ignore.
- [29:51] Tab1 vim touch.c, normal mode (box cursor on empty line 19). Status line: `"rm.c" [New] 36L, 544B written`. Code has `#include <unistd.h>` line 3, 36 lines. Mouse I-beam cursor in window.
- [29:55] Focus on notes.txt (right terminal active, title bolder). notes.txt in VISUAL mode (`-- VISUAL --`, count 38), selection = rm row line 2 text `if dir, open child, if dir walk path`. Walk line 1 now reads `walk path, if file remove` (cursor box gone).
- [30:00] Tab2 scratch vim (INSERT, same notes) active, mouse in window.
- [30:04] Tab2: `-- (insert) VISUAL --`, count 2: selecting (mouse drag) lines 5 wrap + 6 (`move folder abc` / `if have file is link -> nofollow it ,`).
- [30:05] Tab2: selection extended to all of line 5 through 6 (`-- (insert) VISUAL --` 2 lines): the `rm abc --> if is folder , we must remove all file child then remove folder abc` + line 6. (copying text to paste into rm.c as comment.)
- [30:08] Tab2: selection reduced to 1 char at end of line 5 wrap (`abc`), count 1 (mouse selection ended; copy done, presumably).
- [30:10] Tab1: rm.c normal mode, cursor on empty line 19. Status `"rm.c" [New] 36L, 544B written` still shown. Mouse near line 15.
- [30:11] Tab1 INSERT: new blank lines; file 37 lines. Line 20 is a new empty line with cursor (opened with `o` at line 19, i.e. now lines 19, 20 empty, `return 0;` at 21).
- [30:13] Tab1: pasted comment on line 20 (wrapping to 2 display lines): `  // rm abc --> if is folder , we must remove all file child then remove folder abc` (blank line 21 with cursor, 37+ lines; `return 0;` line 22).
- [30:15] Tab1 INSERT: new line 21 typing `//` (return 0; moved to line 23). Just `  //` with cursor.
- [30:18] notes.txt window focused: selection changed in VISUAL mode, count 18, highlights `opendir, readdir` (row rm line 2 col 2) with mouse cursor. Left tab1 shows empty line 21 `  ` (comment `//` gone; empty indented line, box cursor).
- [30:20] notes.txt VISUAL selection `opendir, readdir` count 18. Tab1: line 21 empty with cursor (INSERT).
- [30:26] Tab3 `-zsh`: screen history shown:
  ```
  lab $ cc rm.c -o rm
  lab $ ls
  abc        mkdir      mv         notes.txt  rm.c       touch.c
  cat.c      mkdir.c    mv.c       rm         touch      xyz
  lab $ ./rm abc/def.txt abc/y-xyz.txt
  lab $ ls abc
  abc.txt cat     def
  lab $ ./rm abc/abc.txt
  lab $ ls abc
  cat def
  lab $ # remove file work
  ```
  (cursor after `work`).
- [30:27] Tab3 cleared: `lab $ `.
- [30:33] Tab3 now `...less < man opendir` : DIRECTORY(3) Library Functions Manual. SYNOPSIS `#include <dirent.h>`, `DIR * opendir(const char *filename);`, `DIR * fdopendir(int fd);`, `struct dirent * readdir(DIR *dirp);`, `int readdir_r(DIR *dirp, struct dirent *entry, struct dirent **result);`, `long telldir(DIR *dirp);`, `void seekdir(DIR *dirp, long loc);`, `void rewinddir(DIR *dirp);`, `int closedir(DIR *dirp);` ; LIBRARY: Standard C Library (libc, -lc).
- [30:38] man opendir scrolled down by one line (scrollbar at bottom; `int` of next entry at bottom). [30:39] scrolled further: `int fdclosedir(DIR *dirp);` visible.
- [30:42] Tab1 rm.c, INSERT, cursor on empty line 21 (indented), blank-line layout; tab bar shows tab3 `...less < man opendir`. Same buffer as 30:15 minus `//`.
- [30:44] Tab1 INSERT: new empty line 4 added (cursor at line 4), so line numbers shift: includes lines 1-3, 4 `|` cursor, 5 empty, walkrm at 6. (about to add `#include <dirent.h>`.) Comment now line 21-22, `return 0;` line 24.
- [30:48] Tab3 man opendir again, top of page.
- [30:53] Tab1 normal mode; line 4 now `#include <dirent.h>`; box cursor on empty line 22 (below comment, wrapped). File 37+ lines.
- [31:01] INSERT: line 22 `  DIR *dir = opendir(path);` , line 23 empty with cursor; `return 0;` moves to line 25.
- [31:04] INSERT: line 23 `  if (!dir) {` ; line 24 cursor.
- [31:08] INSERT: line 24 `    perror(dir);` ; line 25 `    re` typing.
- [31:10] line 25 `    return 1;` ; line 26 cursor (indented 4).
- [31:11] line 26 `  }` ; line 27 empty with cursor (indent 2); `return 0;` line 29.
- [31:12] Several blank lines added: lines 27, 28, 29 blank; cursor line 30; `return 0;` line 32.
- [31:18] Tab3 man opendir: lines `DIR *` / `opendir(const char *filename);` highlighted (less search highlight or selection).
- [31:20] Tab1 INSERT: line 30 `  closedir(dir)` (matching-paren highlight on `(dir)`), typed without `;` yet.
- [31:23] Tab1: line 31 `  closedir(dir);` (cursor line 28, i.e. user went back up to insert code in blank lines 27-30), blank lines 27, 29, 30. File: lines 22-26 opendir block, lines 27-30 blank, 31 closedir(dir);, 32 blank, 33 return 0;, 34 }.
- [31:29] Tab3 man opendir, same page; notes.txt in VISUAL (count 7) with selection `readdir` (rm row line 2, second function name). Next step: readdir loop.

## File states

### rm.c (edited in tab titled `vim touch.c`). State at 31:23 (end of segment; lines 36+ not visible but known from earlier frames, shifted)
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
21|   // rm abc --> if is folder , we must remove all file child then remove folder abc   (one long line, wrapped at "th|en")
22|   DIR *dir = opendir(path);
23|   if (!dir) {
24|     perror(dir);
25|     return 1;
26|   }
27|   ⟨cursor line 28 at 31:23; 27-30 blank⟩
28|
29|
30|
31|   closedir(dir);
32|
33|   return 0;
34| }
35|
36| int main(int argc, char *argv[]) {
37|   if (argc == 1) {
38|     printf("Useage: rm [path]...\n");
39|     return 0;
40|   }
41|
42|   for (int i = 1; i < argc; i++) {
43|     const char *path = argv[i];
44|     if (walkrm(path))
45|       return 1;
46|   }
47|
48|   return 0;
49| }
```
Lines 38-49 inferred from the earlier frame (28:38) with line-number shift (+13); not visible in last frames. Note: at the 31:23 frame the visible part is lines 1-37. Indent width is 2 spaces. Note `perror(dir);` (passes DIR* — likely bug; perror expects const char*).

### Version at 28:38 (35 lines, before unistd.h include)
```
1| #include <stdio.h>
2| #include <sys/stat.h>
3|
4| int walkrm(const char *path) {
5|   struct stat st;
6|   if (lstat(path, &st)) {
7|     perror(path);
8|     return 1;
9|   }
10|
11|   if (!S_ISDIR(st.st_mode)) {
12|     // it is file
13|     if (unlink(path)) {
14|       perror(path);
15|       return 1;
16|     }
17|   }
18|
19|   return 0;
20| }
21|
22| int main(int argc, char *argv[]) {
23|   if (argc == 1) {
24|     printf("Useage: rm [path]...\n");
25|     return 0;
26|   }
27|
28|   for (int i = 1; i < argc; i++) {
29|     const char *path = argv[i];
30|     if (walkrm(path))
31|       return 1;
32|   }
33|
34|   return 0;
35| }
```
Saved at 29:51 as `"rm.c" [New] 36L, 544B written` (with `#include <unistd.h>` on line 3, 36 lines).

### Scratch notes (tab2 `~/lab — vim`), 9 lines
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
(in INSERT mode at 28:57; unchanged through 30:10)

### notes.txt
Design table unchanged; rm row now reads:
```
|rm   | lstat, unlink, rmdir | walk path, if file remove
|     | opendir, readdir     | if dir, open child, if dir walk path
```
(earlier "walk path, if file remove" had cursor on 'remove'.) Visual highlights: 29:55-30:15 line 2 col 3 `if dir, open child, if dir walk path` (count 38); 30:18-31:23 `opendir, readdir` (count 18); 31:29 `readdir` (count 7). Other rows: touch/fopen; cat/fopen, fgetc, putchar/"get path and do"; mkdir/mkdir; mv/stat, rename/"if target is file or not exit, rename" + "if target is dir , move"; find/lstat, strstr, opendir, readdir/"walk path, if match print" + "if dir, open child, if dir walk path"; tree/(blank)/"watch my previous video".

## Shell & outputs
- [28:58] `./mv cat abc` (earlier test; no output) then `ls`; `ls abc` -> `abc.txt cat def def.txt y-xyz.txt`; `# ok mv donema`; `man 2 unlink`.
- [28:59] `cc ` -> [29:03] `cc rm.c -o rm` (no output, success); `ls` shows `rm` binary and `rm.c`.
- [29:14] `./rm abc/def.txt` ; [29:17] `./rm abc/def.txt abc/y-xyz.txt` ; [29:34] `ls abc` -> `abc.txt cat def`; `./rm abc/abc.txt`; [30:26] `ls abc` -> `cat def`; `# remove file work`.
- No compiler errors or runtime output seen. Man pages: `man 2 unlink` (28:37), `man opendir` (DIRECTORY(3), 30:33-31:29).

## Observations
- Build order: rm.c follows mv; file-only removal first, tested with `./rm abc/def.txt abc/y-xyz.txt` and `./rm abc/abc.txt`; then directory recursion started (opendir...). Note design table notes `rmdir` and readdir next.
- Pattern: one function `walkrm` per util plus main with argc==1 usage message ("Useage:" typo, kept) and for loop over argv; returns 1 on error; perror(path) for syscall failures; `lstat` rather than stat to avoid following symlinks (scratch note: "if have file is link -> nofollow it").
- Man page usage: reads man 2 unlink (SYNOPSIS for header `<unistd.h>`), then man opendir for header `<dirent.h>`, signature of opendir, closedir; highlights synopsis lines. Adds the needed include right after reading.
- Scratch vim buffer holds algorithm notes; selected and pasted into the C file as comment `// rm abc --> ...` (comment-driven development).
- Shell comments as notes: `# ok mv donema`, `# remove file work`.
- Same vim tab (`vim touch.c`) reused for a new file; saved as rm.c with `:w rm.c` (message `[New]`).
- Frames not seen: exact text of lines 38-49 at end; the readdir loop not yet written by 31:29.
