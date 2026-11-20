# Segment 4 (07:40 - 10:06)

Note: vim tab title always reads "~/lab -- vim README.md" even though the buffer shows C code. The shell `ls` ([08:06]) shows `.main.c.swp`, i.e. main.c is open in vim, so the buffer is almost certainly main.c (title stale/misleading).

## Timeline
- [07:40] Mission-Control-style view: Terminal window (tabs "~/lab -- vim README.md", "~/lab -- -zsh") with vim in INSERT mode, plus browser window "The Open Group Base Specifications Issue 8" at pubs.opengroup.org/onlinepubs/9799919799/ showing dirent.h: struct posix_dent members (ino_t d_ino; reclen_t d_reclen; unsigned char d_type "File type or unknown-file-type indication."; char d_name[]), then DT_BLK, DT_CHR, DT_DIR ("Directory." highlighted), DT_FIFO, DT_LNK, DT_REG, DT_SOCK, DT_UNKNOWN. Code is 33 lines; line 16 is `    if (dp->d_type |` being typed.
- [07:41] Browser full-size; cursor selecting "Directory." under DT_DIR. Webcam overlay appears.
- [07:45] Back in vim. Line 16 `    if (dp->d_type |` (cursor after trailing space), INSERT.
- [07:49] Line 16 is `    if (dp->d_type == DT_DIR ) {` (note stray space before `)`), line 17 empty, line 18 `    }` (matching brace highlighted). File 35 lines.
- [07:51] Line 18 now `    } else {`, line 19 empty, line 20 cursor (indented 6), line 21 `    }`. File 37 lines.
- [08:05] Comments added: line 17 `      // is dir`, line 19 `      // is file`. VISUAL mode, line 14 `printf("%s\n", name);` selected (from "intf" to `;`), status shows count "18".
- [08:06] zsh tab: ran `./a.out` (output below in Shell section): `.`, `..`, abc, .main.c.swp, README.md, a.out, main.c.
- [08:10] Back in vim, VISUAL, same selection on line 14.
- [08:11] INSERT: blank lines inserted after printf; cursor on new line 15 (empty, indented 4), line 16 blank, `if` now on line 17. File 38 lines.
- [08:12] Line 16 `    // ` being typed. `if` at line 18.
- [08:14] Line 16 `    // ignore files`, line 17 `    // |` typing more comment. `if` at 19.
- [08:17] The printf line is GONE. Lines: 13 `const char *name = dp->d_name;`, 14 blank, 15 `    // ignore files`, 16 (block cursor at col 0, blank; normal mode), 17 blank, 18 `if (dp->d_type == DT_DIR ) {`. The extra `// ` line was deleted along with the printf.
- [08:31] zsh tab: old ./a.out output still shown; typed `lab $ man` (partial).
- [08:34] Tab title "~/lab -- less < man 3 strcmp"; man page STRCMP(3): NAME strcmp, strncmp - compare strings; LIBRARY Standard C Library (libc, -lc); SYNOPSIS `#include <string.h>`, `int strcmp(const char *s1, const char *s2);`, `int strncmp(const char *s1, const char *s2, size_t n);`; RETURN VALUES: "return an integer greater than, equal to, or less than 0, according as the string s1 is greater than, equal to, or less than the string s2. The comparison is done using unsigned characters, so that '\200' is greater than '\0'." STANDARDS ISO C90. macOS 26.7, October 11, 2001. (END).
- [08:41]-[08:42] Same man page; webcam window being dragged around.
- [08:45] Back to vim INSERT, webcam window now lower-right. Line 16 `    if (strcmp(name, ".") |` being typed (red bracket-mismatch highlights on lines 18/20/22 braces because new `if (` unclosed).
- [09:03] Line 16: `    if (strcmp(name, ".") == 0 || strcmp(name, "..") == 0) {`, line 17 cursor (indented 6), INSERT.
- [09:19] Revised: line 16 `    if (strcmp(name, ".") == 0 || strcmp(name, "..") == 0)` (NO brace, outer `(` highlighted, cursor on final `)`), line 17 `      continue;`. Terminal window narrower momentarily (resize animation). File 40 lines.
- [09:21] Window full width again; scrolled/visible through line 40 `}`; normal mode. Lines 39 `  return err;`, 40 `}`.
- [09:22] Status line: "Type  :qa!  and press <Enter> to abandon all changes and exit Vim" (he pressed Ctrl-C in normal mode; nothing quit).
- [09:25] INSERT: new blank lines at top; cursor on line 4 (empty), line 5 empty, `int walk` now on line 6. File shifted by 2.
- [09:40] New function above walk: line 4 `void print_file(const char *name, int is_dir) {`, line 5 empty, line 6 `}` (braces highlighted). walk at line 8. INSERT.
- [09:44] Same; normal mode, cursor line 5; mouse at Dock (Terminal tooltip) - switching to browser.
- [09:45] Browser again on opengroup dirent page, "DT_DIR" highlighted.
- [09:46] New Firefox tab; URL bar has "c " typed, suggestion dropdown.
- [09:47] URL bar "c print an".
- [09:49] Google search "c print ansi color" (www.google.com/search?client=firefox-b-d&q=c+print+ansi+color) loading; "Thinking" AI Mode placeholder; first result Stack Overflow.
- [09:50]-[09:51] Results: Stack Overflow "How can I use ANSI Escape Codes for outputting colored ..." (sublinks "stdlib and colored output in C", "ANSI colors in C and ncurses"); GitHub gist (RabaDabaDoba) "The entire table of ANSI color codes working in C!"; c-for-dummies.com "Color Text, Part I | C For Dummies Blog"; Medium "Adding Color to Your Output From C | by Selvaraj kuppusamy".
- [09:52] Clicking gist link: gist.github.com/RabaDabaDoba/145049536f815903c79944599c6f952a.
- [09:53] Gist loading.
- [09:54] Gist "The entire table of ANSI color codes working in C!" file ANSI-color-codes.h: unlicense header; `//Regular text`: `#define BLK "\e[0;30m"`, RED 31, GRN 32, YEL 33, BLU 34, MAG 35, CYN 36, WHT 37; `//Regular bold text`: BBLK..BWHT `"\e[1;30m"`..`"\e[1;37m"`.
- [09:55] Scrolled: `//Regular underline text` UBLK..UWHT `"\e[4;3Xm"`; `//Regular background` BLKB `"\e[40m"`, REDB `"\e[41m"`, GRNB `"\e[42m"`.
- [09:58]-[10:02] Scrolled down to the prose tables: Regular table (Value/Color: \e[0;30m Black, \e[0;31m Red, \e[0;32m Green, \e[0;33m Yellow, \e[0;34m Blue, \e[0;35m Purple, \e[0;36m Cyan, \e[0;37m White), then "Bold" table (\e[1;30m ... \e[1;37m same colors), then "Underline" heading. Lingers on Bold table (cursor over Red, then Blue at 10:02).
- [10:06] Back in vim: code unchanged from 09:44 (print_file stub still empty; cursor on line 5; normal mode). Nothing from the color research typed yet.

## File states

Buffer (tab says README.md; content is the C program, very likely main.c). State at end of segment (10:06). Lines 42+ not visible at 10:06; from earlier frame (09:21, before the 2-line insert) the tail is a blank line, `  return err;`, `}`.

```
 1| #include <stdio.h>
 2| #include <dirent.h>
 3|
 4| void print_file(const char *name, int is_dir) {
 5|
 6| }
 7|
 8| int walk(const char *path) {
 9|   DIR *dir = opendir(path);
10|   if (!dir) {
11|     perror("opendir");
12|     return 1;
13|   }
14|
15|   struct dirent *dp;
16|   while ((dp = readdir(dir)) != NULL) {
17|     const char *name = dp->d_name;
18|
19|     // ignore files
20|     if (strcmp(name, ".") == 0 || strcmp(name, "..") == 0)
21|       continue;
22|
23|     if (dp->d_type == DT_DIR ) {
24|       // is dir
25|     } else {
26|       // is file
27|     }
28|   }
29|
30|   closedir(dir);
31|   return 0;
32| }
33|
34| int main(int argc, char **argv) {
35|   const char *path = "."; // default path is current dir
36|
37|   if (argc > 1) {
38|     path = argv[1];
39|   }
40|
41|   int err = walk(path);
42|
43|   ?? (lines 42+ not visible at 10:06; inferred: line 43 `  return err;`, line 44 `}`)
```
Notes: indentation 2 spaces. `#include <string.h>` NOT present yet though strcmp is used. Stray space in `DT_DIR )`. Comment "// ignore files" is really about skipping `.` and `..`.

Intermediate versions:
- 07:49 (35 lines): walk loop had printf at line 14 and `if (dp->d_type == DT_DIR ) {` / blank / `}` at lines 16-18.
- 08:05 (37 lines):
```
12|   while ((dp = readdir(dir)) != NULL) {
13|     const char *name = dp->d_name;
14|     printf("%s\n", name);
15|
16|     if (dp->d_type == DT_DIR ) {
17|       // is dir
18|     } else {
19|       // is file
20|     }
21|   }
```
  printf removed by 08:17.
- 09:03: line 16 was `if (strcmp(name, ".") == 0 || strcmp(name, "..") == 0) {` with cursor on empty line 17; by 09:19 changed to brace-less form with `continue;`.

## Shell & outputs
- [08:06] zsh tab (prompt `lab $ `):
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
(Output of an earlier build which printed every name; `.main.c.swp` implies main.c is open in vim. Order is raw readdir order. This command was likely run earlier; frame at 08:06 just shows the tab.)
- [08:31] `lab $ man` (typing; completes to `man 3 strcmp`). [08:34] tab title "less < man 3 strcmp", man page shown (see Timeline).
- No compile commands seen in this segment.

## Browser
- [07:40-07:41, 09:45] pubs.opengroup.org/onlinepubs/9799919799/ (POSIX Issue 8), index search box "dirent", dirent.h section with DT_* constants.
- [09:46-10:02] Google "c print ansi color" -> gist.github.com/RabaDabaDoba/145049536f815903c79944599c6f952a (ANSI-color-codes.h: Regular `\e[0;3Xm`, Bold `\e[1;3Xm`, Underline `\e[4;3Xm`, background `\e[4Xm`).

## Observations
- Build order: walk() with opendir/perror/readdir/closedir first, printing every name; then d_type check (DT_DIR looked up in POSIX spec), if/else skeleton with `// is dir` / `// is file` placeholder comments (comment-driven stubs); removed debug printf; skip `.` and `..` via strcmp (checked man 3 strcmp for return semantics: 0 when equal); then stubbed `print_file(name, is_dir)` helper above walk and went to research ANSI color escape codes, implying colored/styled tree output (dirs vs files) is next.
- Style: 2-space indent, K&R same-line braces, braces omitted for single-statement `if ... continue;` (but braces used for the perror/return block), `const char *` for non-mutated pointers, `int err = walk(path); return err;` pattern, error handling via perror + return 1, only stdio.h/dirent.h so far (string.h still missing). Small typos left (`DT_DIR )`).
- Learns from primary docs (POSIX spec, man pages, gist) rather than AI; uses plain vim (visual-mode line cut, no plugins). Minimalist: no headers/structs beyond what is needed, incremental stubbing with comments.
