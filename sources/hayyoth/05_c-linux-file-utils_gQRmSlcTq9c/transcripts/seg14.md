# Segment 14 (49:04 - 52:26) FINAL: find.c finishing, tests, closing comment

Layout constant all segment: left Terminal tabs `~/lab — vim touch.c` (this tab actually holds find.c; title is stale) and second tab (`~/lab — -zsh` / `~/lab — less ◂ man 3 strstr`). Right-top notes.txt in VISUAL mode, line `37`, highlighted text `if dir, open child, if dir walk path` (find row, 2nd algorithm line); never changes in this segment. Webcam ignored.

## Timeline
- [49:04] Tab 2 `-zsh`, empty prompt `lab $ `. notes.txt VISUAL highlight on find row line 2.
- [49:10] Shell: `lab $ mv cat.c abc` then `lab $ ls`, output:
  ```
  abc         find.c    mv          notes.txt rm.c
  find        mkdir.c   mv.c        rm          touch.c
  ```
  (`abc` cyan = directory; `find`, `mv`, `rm` red = executables.) cat.c was moved INTO dir `abc` (test fixture). No `cat`, `touch`, `mkdir` binaries at top level.
- [49:18] `lab $ ./find cat .` (older debug build of find, prints every non-dir entry with "on check"):
  ```
  on check cat.c cat
  on check notes.txt cat
  on check .find.c.swp cat
  on check rm.c cat
  on check .notes.txt.swp cat
  on check mkdir.c cat
  on check mv cat
  on check mv.c cat
  on check find cat
  on check touch.c cat
  on check find.c cat
  on check rm cat
  ```
  Screen shows `mv cat.c abc`, `ls`, `./find cat .` together; the output lists `cat.c` at top level, which is odd after the mv (⟨?⟩ the output as seen is transcribed; the old binary also printed no entries from inside abc). This debug build prints "on check <name> <pattern>" for every non-directory entry: i.e. no matching yet.
- [49:43] Cleared prompt: typed `lab $ am` (stray, probably start of `man`).
- [49:47] Tab 2 title `less ◂ man 3 strstr`. STRSTR(3) Library Functions Manual: NAME `strstr, strcasestr, strnstr — locate a substring in a string`; LIBRARY `Standard C Library (libc, -lc)`; SYNOPSIS `#include <string.h>`, `char * strstr(const char *haystack, const char *needle);`, strcasestr same, `strnstr(const char *haystack, const char *needle, size_t len);`, `#include <xlocale.h>` `strcasestr_l(..., locale_t loc);`. DESCRIPTION.
- [49:53] [49:56] [50:00] scrolling man page to DESCRIPTION and RETURN VALUES: "If needle is an empty string, haystack is returned; if needle occurs nowhere in haystack, NULL is returned; otherwise a pointer to the first character of the first occurrence of needle is returned." (strnstr "is a FreeBSD specific API, it should only be used when portability is not a concern.")
- [50:03] Switched to tab 1 (vim). Cursor on `;` of line 43 (`walkfind(new_path, pattern);`). Message line: `"find.c" [New] 67L, 1308B written` (stale message from earlier save). View lines 20-57; line 55 `printf("Useage: find [patter] [path]...\n");`.
- [50:05] vim scrolled to top, lines 1-38, cursor on line 8. NORMAL.
- [50:09] INSERT: opened a blank line 15 inside `if (!S_ISDIR(st.st_mode)) {`; printf now line 16.
- [50:13] [50:14] [50:21] [50:27 no] Back to tab 2 man page (top/bottom scroll) between edits (50:13, 50:14, 50:21).
- [50:27] vim INSERT: line 14 `if (!S_ISDIR(st.st_mode))` (` {` hidden by I-beam mouse pointer), line 15 `    if (strstr` being typed.
- [50:39] INSERT: line 14 `if (!S_ISDIR(st.st_mode)) {`, 15 `if (strstr(path, pattern)) {`, 16 blank, 17 blank (cursor), 18 `printf("on check %s %s\n", path, pattern);`, 19 `}`.
- [50:41] NORMAL (matching brace highlight): line 14 `{` cyan-highlighted matching line 18 `}`; lines 15 `if (strstr(path, pattern)) {`, 16 blank, 17 `}`, 18 `}`. The printf was cut (deleted/yanked) from the block.
- [50:43] printf pasted back at line 16 inside the strstr if: `printf("on check %s %s\n", path, pattern);` (indent 2 less than target), blank 17, `}` 18, `}` 19.
- [50:44] printf re-indented one level (`>>`): at line 16, 6 spaces. Bottom message: `Type  :qa!  and press <Ent...n all changes and exit Vim` (Ctrl-C pressed).
- [50:46] INSERT: editing format string, `printf("| check %s %s\n", path, pattern);` (deleted `on`).
- [50:53] Format now `"find %s %s\n"`: lines 14 `if (!S_ISDIR(st.st_mode)) {`, 15 `if (strstr(path, pattern)) {` (VISUAL LINE selected, `1`), 16 `printf("find %s %s\n", path, pattern);`, 17 `}`, 18 `}`.
- [50:54] `3 lines yanked` (lines 15-17 block, VISUAL LINE; the block is `if (strstr(path, pattern)) {` / printf / `}`).
- [50:56] Scrolled to lines 18-55; child branch (lines 42-46): `if (!S_ISDIR(st_child.st_mode)) {` / `printf("on check %s %s\n", name, pattern);` / `} else {` / `walkfind(new_path, pattern);` / `}`; cursor line 55 `int main(int argc, char *argv[]) {` (file was 3 lines shorter at that moment, main at 55).
- [50:58] After paste (`3 more lines`): child branch now line 42 `if (!S_ISDIR(st_child.st_mode)) {`, 43 `printf("on check %s %s\n", name, pattern);`, 44 `if (strstr(path, pattern)) {`, 45 `printf("find %s %s\n", path, pattern);`, 46 `}`, 47 `} else {`, 48 `walkfind(new_path, pattern);`, 49 `}` ... 53 `closedir(dir);`, 55 `return 0;`. Pasted block mis-indented (2 spaces too few).
- [50:59] VISUAL LINE on lines 44-46 (`3`).
- [51:00] After `>`: 44 `    if (strstr(path, pattern)) {` (8 spaces), 45 `      printf("find %s %s\n", path, pattern);`, 46 `    }`.
- [51:05] Edit: line 43 `    if (strstr(name, pattern)) {`, 44 `      printf("find %s %s\n", path, pattern);`, 45 `    }`, 46 `} else {`; the `printf("on check ...")` line deleted. File now ends `55 }`.
- [51:08] printf at 44 deleted: 43 `if (strstr(name, pattern)) {`, 44 `}`, 45 `} else {`, 46 `walkfind(new_path, pattern);`, 47 `}`; ... 53 `return 0;`, 54 `}`.
- [51:09] INSERT at new blank line 44 inside `if (strstr(name, pattern)) {` (body not yet typed; cursor col 6).
- [51:28] vim scrolled to lines 4-41; cursor end of line 4 `#include <dirent.h>`. Top-level block still `printf("find %s %s\n", path, pattern);` at line 16.
- [51:29] Line 1-38 view; cursor on `o` of `const` line 7 col; top block lines 14-18 unchanged.
- [51:31] top-level printf line 16 deleted: 15 `if (strstr(path, pattern)) {`, 16 `}` (highlighted bracket match).
- [51:32] INSERT line 16: `print` typed. [51:33] `printf("` (rest of file turns red because of open string). [51:35] `printf("%s\n", pat` — new top-level body prints path with `%s\n`.
- [51:38] Switched to tab 2 man page again (scrolled to LIBRARY/SYNOPSIS).
- [51:39] Tab 2 `-zsh`: `lab $ man 3 strstr` / `lab $ ` (man quit; stale command echoed).
- [51:52] Shell (ctrl-l screen):
  ```
  lab $ man 3 strstr
  lab $ cc find.c -o find
  lab $ ./find cat abc
  abc/cat.c
  lab $ ./find cat .
  ./abc/cat.c
  lab $
  ```
  (no compiler output = clean compile; vim edits had been saved off-frame, no `written` message in any frame after 50:03).
- [52:00] Added:
  ```
  lab $ ./find main.c ~/sources
  /Users/ctsotc/sources/htree/main.c
  /Users/ctsotc/sources/float-cam/main.c
  /Users/ctsotc/sources/hcat/main.c
  /Users/ctsotc/sources/hserve/main.c
  /Users/ctsotc/sources/hrm/main.c
  /Users/ctsotc/sources/hsearch/main.c
  /Users/ctsotc/sources/hmv/main.c
  /Users/ctsotc/sources/htime/main.c
  /Users/ctsotc/sources/mkey/main.c
  /Users/ctsotc/sources/hmk/main.c
  ```
- [52:12] Added:
  ```
  lab $ ./find READ ~/sources
  /Users/ctsotc/sources/htree/README.md
  /Users/ctsotc/sources/hayyoth-lab/learn/README.md
  /Users/ctsotc/sources/hayyoth-lab/README.md
  /Users/ctsotc/sources/hayyoth/README.md
  /Users/ctsotc/sources/nanoserve/README.md
  /Users/ctsotc/sources/hayyoth.org-0/README.md
  /Users/ctsotc/sources/hcat/README.md
  /Users/ctsotc/sources/hserve/README.md
  /Users/ctsotc/sources/hrm/README.md
  /Users/ctsotc/sources/hsearch/README.md
  /Users/ctsotc/sources/hmv/README.md
  /Users/ctsotc/sources/htime/README.md
  /Users/ctsotc/sources/mkey/README.md
  /Users/ctsotc/sources/hmk/README.md
  /Users/ctsotc/sources/tele/README.md
  ```
- [52:20] Screen cleared: `lab $ `.
- [52:26] Final frame: `lab $ ## ok all work , good luck ` (typed, not shown executed). macOS menu bar visible (Terminal Shell Edit View Window Help), clock `Mon Oc⟨?⟩ 9  3:37 PM` (partly hidden by mouse pointer). Video ends here.

## File states

### notes.txt (unchanged this segment; only VISUAL highlight on find row line 2; vim line 37 shown bottom right)
```
|Command | Function use          | Algorithm
|touch   | fopen                 |
|cat     | fopen, fgetc, putchar | get path and do
|mkdir   | mkdir                 |
|mv      | stat, rename          | if target is file or not exit, rename
|        |                       | if target is dir , move
|rm      | lstat, unlink, rmdir  | walk path, if file remove
|        | opendir, readdir      | if dir, open child, if dir walk path
|find    | lstat, strstr         | walk path, if match print
|        | opendir, readdir      | if dir, open child, if dir walk path   <- highlighted
|tree    |                       | watch my previous video
```
(`+----+` separator lines between rows; right edge of table cut off by window; column padding approximate.)

### find.c (open in tab titled `vim touch.c`), state around 51:35-52:00 (compiled version)
Top and child sections from frames 51:29/51:08 (line numbers reconciled: top-level block gives child section line numbers shifting +0 as per 51:29/51:35 frames, i.e. the child strstr block at 42-48 below matches the 50:56 frame shifted by 2 lines once top block returned to 5 lines):
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
15|     if (strstr(path, pattern)) {
16|       printf("%s\n", pat⟨h?? typing cut off at "pat"⟩);
17|     }
18|   }
19|
20|   DIR *dir = opendir(path);
21|   if (!dir) {
22|     perror(path);
23|     return 1;
24|   }
25|
26|   struct dirent *entry;
27|   while ((entry = readdir(dir)) != NULL) {
28|     const char *name = entry->d_name;
29|     if (strcmp(name, ".") == 0 || strcmp(name, "..") == 0)
30|       continue;
31|
32|     size_t path_len = strlen(path) + strlen(name) + 2;
33|     char new_path[path_len];
34|     snprintf(new_path, path_len, "%s/%s", path, name);
35|
36|     struct stat st_child;
37|     if (lstat(new_path, &st_child)) {
38|       perror(new_path);
39|       return 1;
40|     }
41|
42|     if (!S_ISDIR(st_child.st_mode)) {
43|       if (strstr(name, pattern)) {
44|         ⟨?? body not visible; 51:09 frame shows empty line 44 in INSERT. Output "abc/cat.c" for ./find cat abc implies it prints new_path; probably printf("%s\n", new_path);⟩
45|       }
46|     } else {
47|       walkfind(new_path, pattern);
48|     }
49|
50|   }
51|
52|   closedir(dir);
53|
54|   return 0;
55| }
56|
?? (lines 57+ not visible in final state)
```
main (seen 50:03, older version, lines then 53-57):
```
53| int main(int argc, char *argv[]) {
54|   if (argc < 3) {
55|     printf("Useage: find [patter] [path]...\n");
56|     return 0;
57|   }
?? (lines 58-67 not visible; file was 67L total at its last visible save)
```
Argument order is `find PATTERN PATH` (`./find cat abc`).
Save messages: `"find.c" [New] 67L, 1308B written` (visible 50:03, stale); no later written message visible. Also `3 lines yanked` (50:54), `3 more lines` (50:58).

Intermediate versions: see Timeline [50:09]-[51:35]. Evolution of top block: `printf("on check %s %s\n", path, pattern)` -> wrapped in `if (strstr(path, pattern)) {` -> `"find %s %s\n"` -> printf removed, retyped as `printf("%s\n", pat...`. Child: `printf("on check %s %s\n", name, pattern);` -> pasted strstr block with `path` -> `strstr(name, pattern)` -> printf emptied/retyped.

## Shell & outputs
Verbatim in Timeline: [49:10] `mv cat.c abc`, `ls`; [49:18] `./find cat .`; [49:43] stray `am`; [51:39] `man 3 strstr`; [51:52] `cc find.c -o find`, `./find cat abc` -> `abc/cat.c`, `./find cat .` -> `./abc/cat.c`; [52:00] `./find main.c ~/sources`; [52:12] `./find READ ~/sources`; [52:26] `## ok all work , good luck`.
End-state `ls` (49:10): `abc find find.c mkdir.c mv mv.c notes.txt rm rm.c touch.c`. Binaries present: find, mv, rm (red). Hidden swap files `.find.c.swp`, `.notes.txt.swp` shown by 49:18 find output (vim open). cat.c inside `abc/` (confirmed by `abc/cat.c`).

## Observations
- Build order: find was the last utility (tree skipped: "watch my previous video"). Table in notes.txt is the design doc; find row highlighted throughout.
- find design: recursive `walkfind(path, pattern)` with `lstat` (symlinks not followed), `strstr` substring match (no glob/regex), usage `find PATTERN PATH...`. Prints matched non-directory entries (directories themselves never printed); at top level matches whole path, at child level matches `name` and prints the full path.
- Debugged with `printf("on check ...")` placeholders, then converted to match output; checked `man 3 strstr` several times (NULL if not found, returns pointer if found).
- Style: 2-space indent, `if (lstat(...)) { perror(path); return 1; }` error handling (nonzero return rather than exit), VLA with `snprintf`, `+ 2` for slash and NUL, `closedir`. Typo left: `Useage: find [patter] [path]...`. Uses vim yank/paste/indent to restructure.
- Real-world final test over `~/sources` (user dir `/Users/ctsotc/sources`): projects htree, float-cam, hcat, hserve, hrm, hsearch, hmv, htime, mkey, hmk, hayyoth-lab, hayyoth, nanoserve, hayyoth.org-0, tele.
- Closing shell comment verbatim: `## ok all work , good luck` (typed at prompt, not executed). 
