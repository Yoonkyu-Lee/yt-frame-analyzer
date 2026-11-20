# Segment 4 (11:41 - 16:30): building mv.c

IMPORTANT: tab 1 is titled `~/lab — vim touch.c` for the whole segment, but the buffer is mv.c (vim message `"mv.c" 47L, 862B written`; compiler errors name mv.c; shell runs `cc mv.c -o mv`). The tab title is stale or reflects the original vim launch. Treat tab 1 as mv.c.

Tabs: tab1 `~/lab — vim touch.c` (mv.c), tab2 `~/lab — vim` (scratch notes; later `-zsh`, later `...lab — less • man strchr`), tab3 `~/lab — -zsh` (build/test; at 16:30 it too shows `...— less • man strchr`).

## Timeline

- [11:41] Tab 1 INSERT, lines 19-43 visible; line 39 empty with cursor (below `// usecase 2`). Right window shows the notes.txt "Helper" section (not the table), in `-- VISUAL --` (status `1`), I-beam over `S_ISDIR(st.st_mode)`, i.e. he is selecting/copying it.
- [11:47] Line 40 typed: `if (S_ISDIR(st.st_mode) ` (file now 44 lines; `}` line 44). INSERT. notes.txt back in normal mode, cursor on `S` of S_ISDIR.
- [12:12] Line 40 now `if (!S_ISDIR(st.st_mode)) {`; line 41 in progress: `fprintf(stderr, "%s is not dir or not exit\` (typing `\n`). File 45 lines.
- [12:33] Block done and saved: `"mv.c" 47L, 862B written`. Lines 39-43: `const char *dir = target;` / `if (!S_ISDIR(st.st_mode)) {` / `fprintf(stderr, "%s is not dir or not exit\n", dir);` / `return 1;` / `}`. Matching-brace highlight on line 40 `{` and 43 `}`.
- [12:45] Tab 3 (zsh): scrollback with errno/ENOENT errors then fixed build, `./mv xyz.txt my-xyz.txt` test (see Shell). notes.txt Helper view has an extra blank line after the S_ISDIR line.
- [12:47] Tab 3 cleared: just `lab $ `.
- [12:48] Back on tab 1, same view as 12:33 (`"mv.c" 47L, 862B written` still displayed).
- [13:01] INSERT. Line 45 being typed: `for (int i = 1; i < argc; i++_)` (the `_` is the cursor cell inside autopaired/highlighted parens, cyan `(` `)` highlight). File 49 lines. Right window raised: notes.txt table at top.
- [13:15] Lines 45-48: `for (int i = 1; i < argc; i++) {` / `const char *path = argv[i];` / blank (cursor box) / `}`. File 52 lines. notes.txt table visible, I-beam over mkdir row.
- [13:18] notes.txt in VISUAL, `25` chars selected: mv row 2nd line `if target is dir , move`. Left unchanged.
- [13:35] INSERT; comments added at lines 47-48: `// move to dir , like rename but get new path` and `// ` (typing). File 53 lines.
- [14:18] Line 48 `// new path = dir + "/" + file name`; line 49 `printf("%s\n", path);`. File 54 lines. `:` command line open at bottom (about to :w / switch).
- [14:24] Tab 2 (`~/lab — vim`) = scratch usage notes (see File states). Mode line `-- (insert) VISUAL --` with `m` pending; lines 8-9 highlighted. File not modified visibly.
- [14:25] Tab 3: empty screen, typing `cc`.
- [14:43] Tab 3: `cc mv.c -o mv`, `./mv abc.txt def.txt abc` prints `path: abc.txt` / `path: def.txt`; typing `mv ~/lab/`. (So printf format was changed to `"path: %s\n"` between 14:18 and 14:43; that save not in frames.)
- [14:55] `mv ~/lab/abc.txt abc` (silent), then `mv ~/lab/abc.txt def.txt abc` -> `mv: /Users/ctsotc/lab/abc.txt: No such file or directory`. He is using the system mv to move files back/forth for test setup.
- [14:58] Cleared; typing `ls`.
- [15:02] `ls` output, typing `./mv `.
- [15:14] `./mv ~/lab/my-xyz.txt abc` -> `path: /Users/ctsotc/lab/my-xyz.txt` (output highlighted by mouse selection).
- [15:15] Tab 1: `"mv.c" 54L, 1056B written`. Line 49 now `printf("path: %s\n", path);` (grey cursor block on the space after `path:`). Mouse I-beam near line 29.
- [15:16] Scrolled view lines 25-54, same content.
- [15:17] Scrolled to 31-55, blank line 50 added, INSERT, 55 lines (`return 0;` line 54, `}` line 55). About to type.
- [15:24] Tab 2 scratch vim again, identical to 14:24.
- [15:29] Tab 2 now titled `~/lab — -zsh`, with mkdir-test scrollback (see Shell). (Order of tab usage: he flipped tabs; this tab 2 shell is a separate shell.)
- [15:33] Tab 2 title `...lab — less • man strchr`: STRCHR(3) page, `:` prompt at bottom. Webcam: rubbing eyes.
- [15:37] Man page scrolled a bit (NAME at top); bottom shows `strchrnul() returns a pointer to the terminating '\0' if the`. I-beam near strchrnul synopsis.
- [15:39] Tab 1 INSERT, view lines 31-55: line 50 `const char *name = strchr` (typing; cursor after `strchr`). Mouse cursor over tab 1 (he just clicked back to it).
- [15:54] Tab 3 zsh: ls output, `./mv ~/lab/my-xyz.txt abc` -> `path: /Users/ctsotc/lab/my-xyz.txt`, empty prompt (mouse I-beam).
- [15:59] Tab 1 NORMAL, view 31-56: line 50 `const char *name = strchr("/");`, line 51 `printf("name: %s\n", name);`; block cursor on final `e` of `name` at line 51. File 56 lines.
- [16:01] Tab 3: `ls` typed after previous output.
- [16:04] Tab 3: `cc mv.c -o mv` -> 2 errors (implicit strchr; too few args) (see Shell).
- [16:07] Tab 1 NORMAL, view 31-56. Line 50 `const char *name = strchr("/");`; line 51 displays as `21 if (errno == ENOENT && argc == 2) {` with line number `21` (should be 51). Transient: he apparently yanked/pasted or vim redraw glitch; line 52 `}`. In 16:19 frame line 51 is back to `printf("name: %s\n", name);`? See below (16:19 shows 51 = strchr, 52 = printf name; so this is just a redraw artifact or undone paste).
- [16:08] Tab 1 view at top (lines 1-38), INSERT. Line 4 blank with cursor (about to add `#include <string.h>`).
- [16:12] Tab 3 shows the same 2-error output (re-read).
- [16:14] Tab 3: second compile -> 1 error `mv.c:51:34: ... too few arguments` (string.h now included; line numbers shifted by 1).
- [16:16] Tab 1 view lines 1-38: `#include <string.h>` on line 4 (cursor highlight on `>`); `"mv.c" 57L, 1144B written`.
- [16:18] Tab 1 view lines 11-48; NORMAL, grey block cursor between `dir` and `,` in line 48 comment.
- [16:19] Tab 1 view lines 20-57, cursor block at line 55 col 0 (empty line). Line 51 still `const char *name = strchr("/");`, 52 `printf("name: %s\n", name);`.
- [16:25] Tab 3 cleared.
- [16:26] Tab 3: compile error mv.c:51:37 incompatible pointer to integer conversion passing 'char[2]' (he now wrote `strchr(path, "/")`; fix of arg count done but still string literal; see Shell).
- [16:30] Tab 3 (title now `...— less • man strchr`): STRCHR(3) page again (to check that `c` is an int/char).

## File states

### notes.txt (right window)
Table section (13:01 - 16:30), columns after "Function use" cut off by window edge; no vim line numbers visible:
```
Build linux files utils in C

+---------+-----------------------+-----------------------------------------
|Command | Function use           | Algorithm
+---------+-----------------------+-----------------------------------------
|touch   | fopen                  |
+---------+-----------------------+
|cat     | fopen, fgetc, putchar  |     get path and do
+---------+-----------------------+
|mkdir   | mkdir                  |
+---------+-----------------------+-----------------------------------------
|mv      | stat, rename           | if target is file or not exit, rename
|        |                        | if target is dir , move
+---------+-----------------------+-----------------------------------------
|rm      | lstat, unlink, rmdir   | walk path, if file remove
|        | opendir, readdir       | if dir, open child, if dir walk path
+---------+-----------------------+-----------------------------------------
|find    | lstat, strstr          | walk path, if match print
|        | opendir, readdir       | if dir, open child, if dir walk path
+---------+-----------------------+-----------------------------------------
```
(tree row not visible. Exact `+---+` dash counts uncertain.) The "touch/cat/mkdir" Algorithm cell is merged: `get path and do`. VISUAL highlight from 13:18 to 16:30: `if target is dir , move` (25 chars) = the current mv row.

"Helper" section (11:41-12:48), apparently below the table in the same file:
```
Helper
- stat : check path, if file is link, follow link
- lstat: check path, if file is link, return link

struct stat st;
stat(path, &st) == -1   // have error
- errno == ENOENT       // not have file

S_ISDIR(st.st_mode)     // is dir
```

### mv.c (tab 1 "vim touch.c"): state at END of segment (~16:19-16:26)
Numbering from the post-`#include <string.h>` frames (16:16-16:19). Lines 1-38 seen 16:08/16:16, 11-48 at 16:18, 20-57 at 16:19. Compile error at 16:26 shows line 51 = `const char *name = strchr(path, "/");`, so the 16:19 line 51 was edited afterwards.
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
48|     // move to dir , like rename but get new path
49|     // new path = dir + "/" + file name
50|     printf("path: %s\n", path);
51|     const char *name = strchr("/");      // 16:19 version; by 16:26: strchr(path, "/")
52|     printf("name: %s\n", name);
53|   }
54| 
55| 
56|   return 0;
57| }
```
Notes: indent 2 spaces; `argc--; // ...` line 14 followed by line 15 containing only a `//` aligned under the first `//` (col ~11); line 9 usage string leading spaces count ⟨?⟩ (looks like 5 spaces after the opening quote). Line 18 `if (stat(target, &st)) {` as seen 16:08 (no `== -1`). Lines 7-11 usage message prints to stdout and returns 0.

Save checksums: `"mv.c" 47L, 862B written` (12:33/12:48), `"mv.c" 54L, 1056B written` (15:15), `"mv.c" 57L, 1144B written` (16:16).

Intermediate versions (line numbers are PRE-include, i.e. without string.h line, as seen then):
- 11:41 (43 lines): 19 `//`; 20 `// if error is file not exit`; 21 `if (errno == ENOENT && argc == 2) {`; 22 `  // do rename`; 23 `const char *path = argv[1];`; 24 `if (rename(path, target)) {`; 25 `perror(path);`; 26 `return 1;`; 27 `}`; 28 blank; 29 `return 0;`; 30 `}`; 31 blank; 32 `perror(target);`; 33 `return 1;`; 34 `}`; 35-36 blank; 37 `// if dir not exit, return error`; 38 `// usecase 2`; 39 blank (cursor); 40-41 blank; 42 `return 0;`; 43 `}`.
- 11:47 line 40: `if (S_ISDIR(st.st_mode) ` (no `!` yet). 12:12 `if (!S_ISDIR(st.st_mode)) {` + `fprintf(stderr, "%s is not dir or not exit\`.
- 12:33 (47L): lines 39-43 as final form above (without the `dir` var earlier, `const char *dir = target;` added at line 39 between 12:12 and 12:33).
- 13:15 (52L): lines 45-48 `for (...) {`, `const char *path = argv[i];`, empty cursor, `}`.
- 14:18 (54L): 47 `// move to dir , like rename but get new path`; 48 `// new path = dir + "/" + file name`; 49 `printf("%s\n", path);`.
- 15:15 (54L, 1056B): 49 `printf("path: %s\n", path);`.
- 15:17 (55L): blank line 50 inserted. 15:39: `const char *name = strchr` typing. 15:59 (56L): 50 `const char *name = strchr("/");` 51 `printf("name: %s\n", name);`.
- 16:16 (57L, 1144B): `#include <string.h>` added line 4.
- Earlier in segment the file lacked `#include <errno.h>` at the time of the 12:45 scrollback error (the error reported line 20 col 9; after adding include, `if (errno...` moved to line 21, consistent with later 21->22 shift with string.h).

### Scratch notes buffer (tab 2 `~/lab — vim`, 14:24 and 15:24 identical)
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
Lines 8-9 highlighted (VISUAL). Mode line `-- (insert) VISUAL --`, pending `m`. `--->` is three hyphens + `>`; line 2 may read `Use 1 , file2` ("use case 1: file2 target") exactly as seen.

### Other files
touch.c, cat.c, mkdir.c exist in `~/lab` (ls). rm.c, find.c not yet created. Executables present: cat, mkdir, mv, touch. No other file content shown this segment.

## Shell & outputs

Prompt `lab $ `. Home `/Users/ctsotc`; cwd `~/lab`. Compiler is Apple clang via `cc` (macOS SDK).

[12:45]
```
lab $ cc mv.c -o mv
mv.c:20:9: error: use of undeclared identifier
      'errno'
   20 |     if (errno == ENOENT && argc == 2) {
      |         ^~~~~
mv.c:20:18: error: use of undeclared
      identifier 'ENOENT'
   20 |     if (errno == ENOENT && argc == 2) {
      |                  ^~~~~~
2 errors generated.
lab $ cc mv.c -o mv
lab $ ls
abc         cat.c       mkdir.c     notes.txt   xyz
abc.txt     def.txt     mv          touch       xyz.txt
cat         mkdir       mv.c        touch.c
lab $ ./mv xyz.txt my-xyz.txt
lab $ ls
abc         cat.c       mkdir.c     my-xyz.txt  touch.c
abc.txt     def.txt     mv          notes.txt   xyz
cat         mkdir       mv.c        touch
lab $ 
```
(Column alignment of ls approximated; `my-xyz.txt` highlighted in grey by mouse. After the second `ls`, `xyz.txt` is gone, replaced by `my-xyz.txt`.) The first build errors show `mv.c:20:9` with a different indent (line 20 = `if (errno...` at 4-space indent in this display) - errno.h was added right after.

[12:47] `lab $ ` (clear)

[14:25] `lab $ cc`

[14:43]
```
lab $ cc mv.c -o mv
lab $ ./mv abc.txt def.txt abc
path: abc.txt
path: def.txt
lab $ mv ~/lab/
```
[14:55]
```
lab $ mv ~/lab/abc.txt abc
lab $ mv ~/lab/abc.txt def.txt abc
mv: /Users/ctsotc/lab/abc.txt: No such file or directory
lab $ 
```
(preceded by the 14:43 lines)

[14:58] `lab $ ls` (typed after clear)

[15:02]
```
lab $ ls
abc         cat.c       mkdir.c     mv.c        notes.txt   touch.c
cat         mkdir       mv          my-xyz.txt  touch       xyz
lab $ ./mv 
```
[15:14]
```
lab $ ./mv ~/lab/my-xyz.txt abc
path: /Users/ctsotc/lab/my-xyz.txt
lab $ 
```
[15:29] tab 2 (`~/lab — -zsh`) scrollback:
```
lab $ cc mkdir.c -o mkdir
lab $ ./mkdir abc
lab $ ls
abc         cat         def.txt     mkdir.c     touch       xyz.txt
abc.txt     cat.c       mkdir       notes.txt   touch.c
lab $ ./mkdir abc/def xyz
lab $ ls
abc         cat         def.txt     mkdir.c     touch       xyz
abc.txt     cat.c       mkdir       notes.txt   touch.c     xyz.txt
lab $ ls abc
def
lab $ e
lab $ 
```
(`abc` and `xyz` shown as cyan directories; `./mkdir abc/def xyz` created `abc/def` and `xyz` = mkdir takes multiple paths.) Note: no `mv` binary in this listing, so this tab is earlier-session scrollback (before mv.c).

[15:33/15:37/16:30] man page (macOS, `man strchr`, tab title `less • man strchr`): STRCHR(3) "Library Functions Manual". Sections visible: NAME (`strchr, strrchr, strchrnul — locate character in string`), LIBRARY (`Standard C Library (libc, -lc)`), SYNOPSIS (`#include <string.h>`; `char *strchr(const char *s, int c);` `char *strrchr(const char *s, int c);` `char *strchrnul(const char *s, int c);`), DESCRIPTION (strchr locates first occurrence of c (converted to a char) in string s; terminating null is part of the string; strrchr = last occurrence; strchrnul returns pointer to terminating '\0' if not found), RETURN VALUES (pointer to located character, or NULL if not found; strchrnul returns terminating '\0' pointer). `:` less prompt at bottom.

[15:54/16:01]
```
lab $ ls
abc         cat.c       mkdir.c     mv.c        notes.txt   touch.c
cat         mkdir       mv          my-xyz.txt  touch       xyz
lab $ ./mv ~/lab/my-xyz.txt abc
path: /Users/ctsotc/lab/my-xyz.txt
lab $ ls
```
[16:04 / 16:12]
```
lab $ cc mv.c -o mv
mv.c:50:24: error: call to undeclared library
      function 'strchr' with type 'char *(const char *, int)'; ISO
      C99 and later do not support implicit function declarations
      [-Wimplicit-function-declaration]
   50 |     const char *name = strchr("/");
      |                        ^
mv.c:50:24: note: include the header
      <string.h> or explicitly provide a declaration for 'strchr'
mv.c:50:34: error: too few arguments to
      function call, expected 2, have 1
   50 |     const char *name = strchr("/");
      |                        ~~~~~~   ^
2 errors generated.
lab $ 
```
[16:14]
```
lab $ cc mv.c -o mv
mv.c:51:34: error: too few arguments to
      function call, expected 2, have 1
   51 |     const char *name = strchr("/");
      |                        ~~~~~~   ^
1 error generated.
lab $ 
```
[16:25] clear. [16:26]
```
lab $ cc mv.c -o mv
mv.c:51:37: error: incompatible pointer to
      integer conversion passing 'char[2]' to parameter of type
      'int' [-Wint-conversion]
   51 |     const char *name = strchr(path, "/");
      |                                     ^~~
/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/usr/include/_string.h:88:46: note:
      passing argument to parameter '__c' here
   88 | char *_LIBC_CSTR strchr(const char *__s, int __c);
      |                                              ^
1 error generated.
lab $ 
```
Next expected step (after the segment): change `"/"` to `'/'` (he goes back to the man page at 16:30).

## Observations

- Build order: touch.c, cat.c, mkdir.c exist; mv.c is the current one (notes row highlighted). The mv algorithm follows the notes.txt table: argc==1 -> usage; last arg is target and `argc--`; `stat(target)` fails with ENOENT and one source -> `rename`; otherwise target must be a directory (`S_ISDIR`) -> loop over sources, new path = dir + "/" + file name. The basename extraction via `strchr` (first occurrence) is being tried; the man page shows strrchr (last occurrence) is the likely correct function (not yet visible in this segment).
- Error handling style: `perror(path)` / `perror(target)` for syscall failures, `fprintf(stderr, "%s is not dir or not exit\n", dir)` for own errors; always `return 1` on failure, `return 0` on success; no `exit()`. Flat early-return style, 2-space indent.
- Comments-first workflow: plain-English pseudo-code comments written before code (`// have something error`, `// if error is file not exit`, `// do rename`, `// if dir not exit, return error`, `// usecase 2`, `// move to dir , like rename but get new path`, `// new path = dir + "/" + file name`). Consistent misspelling "exit" for "exist". Space before commas in comments (`dir , like`).
- Notes-driven: copies the "Helper" snippets (`struct stat st;`, `stat(path,&st)`, `errno == ENOENT`, `S_ISDIR(st.st_mode)`) and design table rows from notes.txt; VISUAL highlight in notes.txt marks the current step. Scratch vim buffer (tab 2) holds usecases for mv (rename vs move to folder, error with 3 files).
- Debug by printing: temporary `printf("path: %s\n", path)` / `printf("name: %s\n", name)` to verify values before implementing the move.
- Man pages: reads `man strchr` (section 3, macOS "Library Functions Manual") in a separate tab (title `less • man strchr`) for the function signature and header; consults it again after a type error. Headers (errno.h, string.h) are added only after compiler errors tell him they are missing (errors reported by clang: implicit declaration, too few arguments, int-conversion).
- Testing: `cc mv.c -o mv`, `./mv ...` compared against the system `mv` for set-up; test files abc (dir), abc.txt, def.txt, xyz, xyz.txt, my-xyz.txt; also tests absolute path args `~/lab/my-xyz.txt`. Terminal is cleared frequently. Home user dir appears as `/Users/ctsotc`.
- No shell `#` comments or tree/rm/find work appeared in this segment.
