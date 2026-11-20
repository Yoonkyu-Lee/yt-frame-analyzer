# Segment 9 (21:05 - 24:08) - frames k_21m05s .. k_24m08s

Layout: left Terminal window = vim (editor); middle/right Terminal window = shell (test-data / lab); a browser window behind showing a code listing (a recursive `walk(const char *path, int indent)` with opendir/readdir, line numbers 9-31; URL partially visible "...youtube.com/watch?v=tiMHtc6l7x..."). Webcam "Human" overlay ignored.

## Timeline

- [21:05] Vim window (main.c, lines 13-42 visible, `-- INSERT --`, `26,2  24%`) behind; front shell Terminal shows `lab $ cd test-data` then `test-data $ ls -a` with output (`.vscode  index.js  run.html  style.css` / `CNAME  LICENSE  run.js` / `.secret  index.html  README.md  src`; `.` and `..` entries partly covered by cursor). Purpose: showing hidden entries (`.`, `..`, `.secret`, `.vscode`).
- [21:13] Vim window raised. main.c lines 13-42: cat body, is_ignore stub (`// ignore '.' file`), walkcat start. `-- INSERT --`, `26,2  24%`.
- [21:21] Vim now shows a fresh scratch buffer: `1 #include <stdio.h>` / `3 int main(void) {` / `4 (cursor)` / `5   return 0;` / `6 }`; `-- INSERT --  4,3  All`.
- [21:42] Buffer: line 4 `const char *path = "abc/def/.secret.txt";`, line 5 `const char *name =` (incomplete), line 6 `return 0;`, line 7 `}`. Mode `-- (insert) VISUAL --  13  4,31` (mouse selection of `/.secret.txt`).
- [21:44] Shell Terminal in front: `lab $ cd test-data` / `test-data $ ls -a` output fully shown: `.   .vscode  index.js  run.html  style.css` / `..  CNAME  LICENSE  run.js` / `.secret  index.html  README.md  src`. Vim status `(insert) VISUAL  14  4,30`. Author highlights `.vscode`.
- [21:47] Shell shows man page in pager (`:` prompt): `STRCHR(3)  Library Functions Manual  STRCHR(3)`; NAME `strchr, strrchr, strchrnul - locate character in string`; LIBRARY `Standard C Library (libc, -lc)`; SYNOPSIS `#include <string.h>`, `char * strchr(const char *s, int c);`, `char * strrchr(const char *s, int c);`, `char * strchrnul(const char *s, int c);`.
- [21:49-21:51] Scrolls to DESCRIPTION: "The strchr() function locates the first occurrence of c (converted to a char) in the string pointed to by s. The terminating null character is considered part of the string; therefore if c is '\0', the functions locate the terminating '\0'. The strrchr() function is identical to strchr() except it locates the last occurrence of c." The strrchr synopsis block is mouse-highlighted at 21:51.
- [21:53-21:58] Scrolls further: "The strchrnul() function is identical to strchr() except that if c is not found in s a pointer to the terminating '\0' is returned." The strrchr sentence highlighted at 21:58.
- [22:01] Vim in front: line 4 `const char *path = "abc/def/.secret.txt";` (`/.secret.txt` selected), line 5 `const char *name =`; still `(insert) VISUAL  14  4,30`.
- [22:02] Man page in front again; vim shows line 5 `const char *name = str` typed, `-- INSERT --  5,25`.
- [22:09] Vim: line 5 `const char *name = str`, `5,25` (still typing; frame is of vim in front).
- [22:19] Vim buffer: 1 `#include <stdio.h>`, 2 `#incl` (typing, insert `2,6`), 4 `int main(void) {`, 5 `const char *path = "abc/def/.secret.txt";`, 6 `const char *name = strrchr(path, '/');`, 7 `return 0;`, 8 `}`.
- [22:20] Vim saved: status `"test.c" [New] 10L, 181B written`, `8,23 All`. Buffer has `#include <string.h>` and new line 8 `printf("%s\n", name);`. The browser window briefly in front showing the `walk()` listing (reference code, lines 9-31).
- [22:21] Man page (strrchr) in front again; vim status still `"test.c" [New] 10L, 181B written`.
- [22:22] Shell Terminal moved to upper-right: shows `lab $ cd test-data`, `test-data $ ls -a`, listing, `test-data $ man 3 strchr`, `test-data $ ` (prompt).
- [22:23] Shell cleared; typing `test-data $ cc`.
- [22:35] Shell: `test-data $ cd ..` / `lab $ cc test.c` / `lab $ ./a.out` / output `/.secret.txt` (highlighted) / `lab $ `. (strrchr returns pointer to the slash, so output starts with `/`.)
- [22:38] Vim path edited (slashes removed): line 5 `const char *path = "abcdef.secret.txt";`, saved: `"test.c" 10L, 179B written`, `5,29 All`.
- [22:44] Shell: another `lab $ cc test.c` / `lab $ ./a.out` -> `(null)` (highlighted) / `lab $ `. Vim still shows `"abcdef.secret.txt"`, status `"test.c" 10L, 179B written`. Demonstrates strrchr returns NULL when no `/`.
- [22:47] Vim restores path to `"abc/def/.secret.txt"` and starts a new line 7 `if` (insert `7,5`); file now 11 lines (printf line 9, return 10, `}` 11).
- [22:53] Vim line 7: `if (name) name++;` (insert `7,20`).
- [22:57] Vim line 8: `e` typed (insert `8,4`); 12 lines total.
- [23:03] Vim line 8 `else name = path;`; saved: `"test.c" 12L, 221B written`, `8,19 All`. Shell being cleared, typing `lab $ c`.
- [23:09] Shell: `lab $ cc test.c` / `lab $ ./a.out` / `.secret.txt` (highlighted) / `lab $ `. Vim same, `"test.c" 12L, 221B written`.
- [23:13] Vim path edited to `"abcdef.secret.txt"`, saved: `"test.c" 12L, 219B written`, `5,26 All`. Shell shows previous run.
- [23:18] Shell: `lab $ cc test.c` / `lab $ ./a.out` / `.secret.txt` / `lab $ cc test.c` / `lab $ ./a.out` / `abcdef.secret.txt` (highlighted) / `lab $ `. (else branch: name = path, prints whole string.)
- [23:19] Vim status: `Type  :qa  and pr...nter> to exit Vim` (Ctrl-C pressed), `5,26 All`.
- [23:20] Vim `-- VISUAL LINE --  3  8,20`: lines 6-8 (`const char *name = strrchr(path, '/');`, `if (name) name++;`, `else name = path;`) selected (yanked).
- [23:22] Vim command line `:e ` typed (opening another file).
- [23:24] Vim opened main.c: status `"main.c" 79L, 1358B`, `9,1 Top`. Content lines 1-30 shown (see File states).
- [23:25] Scrolled: `39,1 18%` (lines 10-39).
- [23:26] `28,1 20%`, cursor on `int is_ignore(const char *path) {`.
- [23:27] Pasted 3 lines into is_ignore: status `3 more lines`, `30,3 19%`; is_ignore now: line 29 `// ignore '.' file`, 30-32 pasted `const char *name = strrchr(path, '/');` / `if (name) name++;` / `else name = path;`, 33 `}`.
- [23:28] The comment line `// ignore '.' file` deleted; is_ignore is lines 28-32 (`int is_ignore(const char *path) {` / `const char *name = strrchr(path, '/');` / `if (name) name++;` / `else name = path;` / `}`). `29,3 19%`. walkcat starts at line 34.
- [23:30] `47,3 35%`; scrolled to walkcat; line 46 `// is folder`, 47 `DIR *dir = opendir(path);` (cursor on D), 48 `if (!dir) {`.
- [23:32] Insert in is_ignore after `else name = path;`: blank line 32, line 33 `if` (typing). `-- INSERT --  33,5  33%`.
- [23:39] Line 33 `if (name[0] == '.')`, line 34 `retu` (typing). `34,9`.
- [23:41] Line 34 `return 1;`, new empty indented line 35. `35,3  32%`.
- [23:44] Line 36 now `return 0;` (is_ignore: 33 blank with cursor; 34 `if (name[0] == '.')`, 35 `return 1;`, 36 `return 0;`, 37 `}`) - blank line 33 opened for a comment. `33,3  32%`.
- [23:55] Comment present: line 33 `// ignore all file or folder start with '.'`. Normal mode, `60,35 53%`: walkcat on screen (lines 31-60).
- [23:57] Insert at line 40 (blank line opened at top of walkcat), `40,3 52%`.
- [24:04] walkcat line 40 `if (is_ignore(path))` and line 41 cursor (indented further), `41,5 51%`.
- [24:07] Line 41 `return;`; readdir loop lines 62-66 added: `const char *name = dp->d_name;`, `size_t len = strlen(path) + strlen(name) + 2;`, `char sub_path[len];`, `snprintf(sub_path, len, "%s/%s", path, name);`; cursor line 67, `67,0-1 63%`.
- [24:08] Lines 68-75 completed: `if (dp->d_type == DT_DIR) {` / `walkcat(sub_path);` / `} else {` / `cat(sub_path);` / `}` / `}` / `closedir(dir);` / `}`. `72,5 79%`, matching brace `{` on line 61 highlighted cyan.

NOTE: no `:w` save message for main.c visible within this segment after the edits (last main.c message seen: `"main.c" 79L, 1358B` on open at 23:24).

## File states

### test.c (scratch file in `lab/`), as of 23:19
```
 1| #include <stdio.h>
 2| #include <string.h>
 3|
 4| int main(void) {
 5|   const char *path = "abcdef.secret.txt";
 6|   const char *name = strrchr(path, '/');
 7|   if (name) name++;
 8|   else name = path;
 9|
10|   printf("%s\n", name);
11|   return 0;
12| }
```
Save checksums: `"test.c" [New] 10L, 181B written` (22:20); `"test.c" 10L, 179B written` (22:38); `"test.c" 12L, 221B written` (23:03); `"test.c" 12L, 219B written` (23:13).

Intermediate version at 22:20 (10L, 181B):
```
 1| #include <stdio.h>
 2| #include <string.h>
 3|
 4| int main(void) {
 5|   const char *path = "abc/def/.secret.txt";
 6|   const char *name = strrchr(path, '/');
 7|
 8|   printf("%s\n", name);
 9|   return 0;
10| }
```
At 23:03/23:09 (12L, 221B) line 5 was `"abc/def/.secret.txt"` with lines 7-8 `if (name) name++;` / `else name = path;`.

### main.c (opened at 23:24: `"main.c" 79L, 1358B`), state at 24:08 (edited, save not seen)
```
 1| #include <stdio.h>
 2| #include <string.h>
 3| #include <sys/stat.h>
 4| #include <dirent.h>
 5|
 6| #define BUFFER_SIZE 1024
 7|
 8| void cat(const char *path) {
 9|   printf("\n============ %s ============\n", path);
10|
11|   FILE *fp = fopen(path, "r");
12|   if (!fp) {
13|     perror(path);
14|     return;
15|   }
16|
17|   char buffers[BUFFER_SIZE];
18|   size_t nitems;
19|
20|   while ((nitems = fread(buffers, 1, BUFFER_SIZE, fp)))
21|     fwrite(buffers, 1, nitems, stdout);
22|
23|   printf("\n");
24|
25|   fclose(fp);
26| }
27|
28| int is_ignore(const char *path) {
29|   const char *name = strrchr(path, '/');
30|   if (name) name++;
31|   else name = path;
32|
33|   // ignore all file or folder start with '.'
34|   if (name[0] == '.')
35|     return 1;
36|   return 0;
37| }
38|
39| void walkcat(const char *path) {
40|   if (is_ignore(path))
41|     return;
42|   struct stat st;
43|   if (lstat(path, &st)) {
44|     perror(path);
45|     return;
46|   }
47|
48|   if (!S_ISDIR(st.st_mode)) {
49|     // if file -> cat
50|     cat(path);
51|   }
52|
53|   // is folder
54|   DIR *dir = opendir(path);
55|   if (!dir) {
56|     perror(path);
57|     return;
58|   }
59|
60|   struct dirent *dp;
61|   while ((dp = readdir(dir)) != NULL) {
62|     const char *name = dp->d_name;
63|
64|     size_t len = strlen(path) + strlen(name) + 2;
65|     char sub_path[len];
66|     snprintf(sub_path, len, "%s/%s", path, name);
67|
68|     if (dp->d_type == DT_DIR) {
69|       walkcat(sub_path);
70|     } else {
71|       cat(sub_path);
72|     }
73|   }
74|   closedir(dir);
75| }
76|
?? (lines 77+ not visible; main() presumably follows)
```
Notes: In the pre-edit version (23:24) is_ignore was lines 28-30 (`int is_ignore(const char *path) {` / `  // ignore '.' file` / `}`), walkcat began at line 32 (`struct stat st;` 33, `if (lstat(path, &st)) {` 34, `if (!S_ISDIR(st.st_mode)) {` 39, `// if file -> cat` 40, `cat(path);` 41, `}` 42). In walkcat the `cat(path);` branch shows no `return;` after it in the visible frames (lines 49-51). The `=` count in line 9 banner (12 each side) is approximate.

## Shell & outputs

- [21:05] `lab $ cd test-data`
  `test-data $ ls -a`
  ```
  .           .vscode    index.js   run.html   style.css
  ..          CNAME      LICENSE    run.js
  .secret     index.html README.md  src
  ```
  (`.vscode` and `src` colored cyan as directories.)
- [21:47] `man 3 strchr` (page STRCHR(3), Library Functions Manual) - text quoted in Timeline.
- [22:22] `test-data $ man 3 strchr`, then back at prompt `test-data $ `.
- [22:35] `test-data $ cd ..`, `lab $ cc test.c`, `lab $ ./a.out` -> `/.secret.txt`
- [22:44] `lab $ cc test.c`, `lab $ ./a.out` -> `(null)` (path "abcdef.secret.txt", no slash)
- [23:09] (after adding if/else) `lab $ cc test.c`, `lab $ ./a.out` -> `.secret.txt` (path "abc/def/.secret.txt")
- [23:18] `lab $ cc test.c`, `lab $ ./a.out` -> `abcdef.secret.txt` (path without slash)
No compiler errors or warnings displayed.

## Observations

- Author builds the `is_ignore` helper by prototyping in a throwaway `test.c`: first `strrchr(path, '/')` to extract the basename, observed output `/.secret.txt` (includes the slash, hence `name++`), then tested the no-slash case giving `(null)` (hence `else name = path`), then verified both. Test-first, empirical verification of library semantics, after reading `man 3 strchr`.
- Design: ignore any file or folder whose base name starts with `.` (comment `// ignore all file or folder start with '.'`) - motivated by `ls -a` showing `.`, `..`, `.secret`, `.vscode` in test-data (covers `.`/`..` to prevent infinite recursion, and skips hidden files).
- `is_ignore(path)` is wired as an early return at top of `walkcat`; readdir loop builds `sub_path` with a VLA (`len = strlen(path)+strlen(name)+2`) and `snprintf(sub_path, len, "%s/%s", path, name)`, recurses on `DT_DIR`, else `cat(sub_path)`; `closedir(dir)` afterwards.
- Style: 2-space indent, short lowercase intent comments (`// is folder`, `// if file -> cat`), unbraced single-statement ifs in helper, mouse selection plus visual-line yank/paste to move code from test.c into main.c. A browser window shows a similar `walk()` recursion used as a visual reference.
- No sticky notes visible in this segment.
