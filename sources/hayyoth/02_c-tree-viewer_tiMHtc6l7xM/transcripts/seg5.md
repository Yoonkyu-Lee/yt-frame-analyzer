# Segment 5 (10:06 - 12:55)

Note: vim tab title always reads "~/lab — vim README.md" but the buffer contains C code. The shell's `ls` output (12:41) shows files `main.c`, `README.md`, `a.out`, `.main.c.swp`, `abc`, and the compiler is run on `main.c`, so the edited buffer is almost certainly main.c (tab title stale or he opened it as README.md earlier; not verifiable). Indent is 2 spaces per level.

## Timeline

- [10:06] Terminal, vim tab. Normal mode, cursor on empty line 5 (inside `print_file` body). File is 41+ lines (more below). print_file has empty body (lines 4-6). Mouse I-beam at top. Code identical to file state below (v1).
- [10:07] INSERT mode. Two new blank lines inserted after line 3 (cursor on new line 4; lines 4,5 empty). print_file now starts at line 6; everything shifts +2.
- [10:12] INSERT mode. Line 4 now typed: `#define BLUE \e[1;34m` (cursor highlight on the `;`). Unquoted define, no string quotes. Line 5 empty, line 6 empty, print_file at line 7.
- [10:29] NORMAL. Status: "3 fewer lines; before #32  22 seconds ago" (he undid/deleted: the #define line and blank lines gone; `u` undo). File back to v1 (print_file at line 4, empty body lines 4-6; main... `int err = walk(path);` at line 41). Cursor at end of line 2 (`>` of `<dirent.h>`). So the `#define BLUE` approach was abandoned.
- [10:45] INSERT. print_file body being typed: line 5 `  if (is_dir)`, line 6 `    pr` (cursor after `pr`). Function now lines 4-7; walk at line 9.
- [10:47] INSERT. Line 6: `    printf("` (cursor after opening quote). (All text red = unterminated string highlight.)
- [10:52] INSERT. Line 6: `    printf("%s\n", ` (cursor after comma+space).
- [11:07] NORMAL, scrolled to bottom of file. Shows lines 40-45: `  }`, blank, `  int err = walk(path);`, blank, `  return err;`, `}` (line 45), then `~` filler. Cursor on `}` line 40. (Confirms file ends at line 45.)
- [11:08] NORMAL, scrolled: lines 25-45 visible, cursor at line 25 column in `// is dir`. Same code.
- [11:09] NORMAL, top of file. Line 6 now: `    printf("%s%s%s\n",` (format string changed to three %s; cursor on `n` of #include line 1, col 2). Line 6 ends at comma, no args yet.
- [11:14] INSERT. Line 6: `    printf("%s%s%s\n", '\e[1;34m', ` — note SINGLE quotes around `\e[1;34m` (cursor highlighted on `;`). Cursor at end after `, `.
- [11:15] Mission-control / desktop overview: Terminal on right (same vim state), Firefox-like browser window bottom left. Browser tab 2: "The entire table of ANSI color codes working in C! · GitHub" gist.github.com/RabaDabaDoba/145049536f815903c79944599c6f952a. Tab 1: "The Open Group Base Specifica..." (POSIX spec). Table showing `\e[1;34m Blue` row highlighted/selected under "Bold".
- [11:16] Browser window in front (over terminal). Page "Bold" table: \e[1;30m Black, \e[1;31m Red, \e[1;32m Green, \e[1;33m Yellow, \e[1;34m Blue (text selected), \e[1;35m Purple, \e[1;36m Cyan, \e[1;37m White. Above: \e[0;34m Blue ... \e[0;37m White (Regular). Below: "Underline".
- [11:17-11:18] Scrolling slightly in the same Bold table; Underline table (\e[4;30m Black, \e[4;31m Red ...) starts to appear.
- [11:19] Scrolled to Background table: \e[40m Black, \e[41m Red, \e[42m Green, \e[43m Yellow, \e[44m Blue, \e[45m Purple, \e[46m Cyan, \e[47m White (after Underline ending \e[4;37m White). Next heading "High Intensty" (sic), first row \e[0;90m Black.
- [11:20] High Intensity table (\e[0;90m ... \e[0;97m), then heading "Bold High Intensty", rows \e[1;90m Black .. \e[1;95m Purple...
- [11:21] Scrolled to Reset table: \e[0;106m Cyan, \e[0;107m White (end of High Intensity Background); "Reset": \e[0m Reset / CRESET / COLOR_RESET. Below: testmain.c listing from the gist:
  `#include "ANSI-color-codes.h"`, `#include <stdio.h>`, `int main(){`, printf(BRED "Hey this is the color red, and it's bold! \n" reset); etc. (lines 1-10).
- [11:22] Same, scrolled a bit more; "Load earlier comments..." visible.
- [11:27] Back to Terminal vim (browser hidden), INSERT mode, same line 6: `    printf("%s%s%s\n", '\e[1;34m', ` cursor at end.
- [11:28] Same as 11:27, mouse I-beam at right of line 6.
- [12:11] INSERT. Line 6 rewritten: `    printf("\e[1;34m%s\e[0m\n", name);` (single printf with escape codes inside the format string; double-quoted). New line 7 empty with cursor (indent 2 spaces `  `). Line 8 `}`. i.e., so far `if (is_dir)` then printf then a new line 7 being opened. Whole file shifted by +1 (walk at 10).
- [12:13] INSERT. Line 7 `  else`, line 8 empty with cursor indented 4 spaces. print_file closes at line 9; walk at 11.
- [12:16] INSERT. Line 8: `    printf("%s` (typing).
- [12:20] NORMAL. Line 8: `    printf("%s\n", name);` cursor on `;`. print_file complete (lines 4-9).
- [12:22] NORMAL, scrolled so line 5 at top; showing lines 5-45. Line 45 is empty-looking (blank) following `int err = walk(path);` at line 44 — near-bottom: line 44 `  int err = walk(path);`, line 45 blank. (View is scrolled to show end of file past 45? Lines 45+ cut off.)
- [12:25] NORMAL. In walk loop, the `// is dir` comment line (old line 27) deleted: lines 26-30 now `    if (dp->d_type == DT_DIR ) {` / `    } else {` / `      // is file` (cursor at col of line 28 in indentation) / `    }` / `  }`. File ends: 43 `  int err = walk(path);`, 44 blank, 45 `  return err;`.
- [12:26] NORMAL. `// is file` comment line also deleted. Lines 26-28: `if (dp->d_type == DT_DIR ) {`, `} else {`, `}` — empty bodies. Cursor on `if` (line 26). walk now ends at line 33; main at 35; `int err = walk(path);` line 42.
- [12:27] INSERT. A new empty line 27 inserted inside the `if` branch (cursor at indent 6 spaces `      |`), `} else {` line 28, `}` line 29, `}` line 30.
- [12:36] INSERT. Line 27: `      print_file(name, 1);`. Else branch: line 29 empty with cursor (indent 6) — about to type `print_file(name, 0);`.
- [12:41] Switched to zsh tab. Prompt `lab $`. Output of earlier `./a.out` (listing): see Shell. Then `man 3 strcmp` run and exited; new empty prompt.
- [12:42] zsh: typing `cc` at fresh (cleared) prompt.
- [12:43] zsh: `cc main.c` produced warnings and an error (see Shell). Notably `strcmp` undeclared => `string.h` not included yet; `\e` non-standard escape warnings.
- [12:49] Same compiler output still on screen.
- [12:50] Mission control view: Terminal (zsh, compiler output) and browser behind (ANSI table "Reset" section, `\e[0m` COLOR_RESET cell selected/highlighted). Webcam dimmed.
- [12:51] Browser in front: Reset table with \e[0m COLOR_RESET row text selected; testmain.c listing visible. Terminal behind shows compile output.
- [12:52] Browser scrolled up slightly: High-intensity background end (\e[0;104m Blue .. \e[0;107m White), Reset, testmain.c lines 1-7.
- [12:53] Scrolled up: Underline table \e[4;30m .. \e[4;37m, Background table \e[40m..\e[44m.
- [12:54] Scrolled up: Regular \e[0;30m Black ... \e[0;37m White, "Bold" heading, \e[1;30m..\e[1;35m.
- [12:55] Scrolled down in the code-section of the gist (the ANSI-color-codes.h listing, lines 51-80):
  51 `#define YELHB "\e[0;103m"`, 52 BLUHB `\e[0;104m`, 53 MAGHB `\e[0;105m`, 54 CYNHB `\e[0;106m`, 55 WHTHB `\e[0;107m`, 57 `//High intensty text`, 58 HBLK `"\e[0;90m"` ... 65 HWHT `"\e[0;97m"`, 67 `//Bold high intensity text`, 68 BHBLK `"\e[1;90m"` ... 75 BHWHT `"\e[1;97m"`, 77 `//Reset`, 78 `#define reset "\e[0m"`, 79 `#define CRESET "\e[0m"`, 80 `#define COLOR_RESET "\e[0m"`. Next file "bash-colors.md" with Raw button.

## File states

### Intermediate v1 (10:06, 10:29) — print_file empty
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
43|   return err;
44| }
```
(Lines 42-44 confirmed at 11:07 as 42 `int err = walk(path);`, 44 `return err;`, 45 `}` after print_file grew by 1 line — numbering below is after print_file body added.)

### Abandoned attempt (10:12): line 4 `#define BLUE \e[1;34m` (unquoted), undone by 10:29.

### Intermediate (11:14-11:28)
```
 4| void print_file(const char *name, int is_dir) {
 5|   if (is_dir)
 6|     printf("%s%s%s\n", '\e[1;34m', 
 7| }
```
(single-quoted escape: bug, later rewritten)

### State at END of segment (12:36, vim last seen; line numbers from 12:22-12:36 views)
```
 1| #include <stdio.h>
 2| #include <dirent.h>
 3|
 4| void print_file(const char *name, int is_dir) {
 5|   if (is_dir)
 6|     printf("\e[1;34m%s\e[0m\n", name);
 7|   else
 8|     printf("%s\n", name);
 9| }
10|
11| int walk(const char *path) {
12|   DIR *dir = opendir(path);
13|   if (!dir) {
14|     perror("opendir");
15|     return 1;
16|   }
17|
18|   struct dirent *dp;
19|   while ((dp = readdir(dir)) != NULL) {
20|     const char *name = dp->d_name;
21|
22|     // ignore files
23|     if (strcmp(name, ".") == 0 || strcmp(name, "..") == 0)
24|       continue;
25|
26|     if (dp->d_type == DT_DIR ) {
27|       print_file(name, 1);
28|     } else {
29|       ⟨cursor, empty line being typed; likely print_file(name, 0); not seen⟩
30|     }
31|   }
32|
33|   closedir(dir);
34|   return 0;
35| }
36|
37| int main(int argc, char **argv) {
38|   const char *path = "."; // default path is current dir
39|
40|   if (argc > 1) {
41|     path = argv[1];
42|   }
43|
44|   int err = walk(path);
45|
46|   return err;     ⟨from 12:25 view: line 45 return err when 1 line fewer; at 12:36 line 45 blank, 46 not shown⟩
47| }
```
Caveat: at 12:36 the view shows line 44 `int err = walk(path);` and 45 blank, so `return err;` is line 46 and `}` line 47 (inferred, not visible).

Note `#include <string.h>` is NOT present at 12:43 (compile error says so). Note `// ignore files` comment kept.

## Shell & outputs

[12:41] (zsh tab "~/lab — -zsh", prompt `lab $`)
```
lab $ ./a.out
.
..
abc
.main.c.swp
README.md
a.out
main.c
lab $ man 3 strcmp
lab $ 
```
(a.out from an earlier build: prints all entries incl `.` and `..`, so earlier binary)

[12:42] cleared screen; typing `lab $ cc`

[12:43]
```
lab $ cc main.c
main.c:6:13: warning: use of non-standard escape character '\e' [-Wpedantic]
    6 |     printf("\e[1;34m%s\e[0m\n", name);
      |             ^~
main.c:6:23: warning: use of non-standard escape character '\e' [-Wpedantic]
    6 |     printf("\e[1;34m%s\e[0m\n", name);
      |                       ^~
main.c:23:9: error: call to undeclared library function 'strcmp' with type 'int (const char *, const char *)'; ISO C99
      and later do not support implicit function declarations [-Wimplicit-function-declaration]
   23 |     if (strcmp(name, ".") == 0 || strcmp(name, "..") == 0)
      |         ^
main.c:23:9: note: include the header <string.h> or explicitly provide a declaration for 'strcmp'
2 warnings and 1 error generated.
lab $ 
```
This confirms the buffer is main.c, and that at compile time line 6 was the escape printf and line 23 the strcmp line (i.e., the file was saved with else-branch print_file(name,0) region possibly incomplete, still compiled).

## Observations

- File being edited is `main.c` (compile uses main.c, line numbers match), though vim tab title says README.md.
- Build order: skeleton (includes, walk with opendir/readdir/closedir, main with argv default ".") -> then print_file helper (name, is_dir) -> wire print_file into walk's d_type branch (DT_DIR vs else).
- Color iteration: tried `#define BLUE \e[1;34m` (abandoned, undone), then `printf("%s%s%s\n", '\e[1;34m', ...)` with %s placeholders for color/name/reset (single-quote mistake), looked up the ANSI table on gist (Bold Blue \e[1;34m, Reset \e[0m), then simplified to inline literal escapes in one format string `"\e[1;34m%s\e[0m\n"`. Minimalism: no color header, no #defines; hard-coded escape in printf; plain `%s\n` for files.
- `\e` triggers -Wpedantic warning in clang (non-standard); he did not (yet) change it to `\033`. Compiler warns strcmp undeclared; needed `#include <string.h>` (not yet added at 12:55).
- He checked `man 3 strcmp` (12:41) shortly before compile.
- Style: 2-space indent, K&R braces, braceless single-statement if/else in print_file (`if (is_dir)` / `else` with indented printf on next line), braces in walk. `const char *` everywhere, int return codes (0/1), `perror("opendir")`. Comments lowercase, short (`// ignore files`, `// is dir`, `// is file`, `// default path is current dir`). `print_file(name, 1)` passes is_dir as int flag 1/0 (no bool).
- Uses vim keyboard heavily: undo (`u`) to drop abandoned approach, `dd` to delete placeholder comments, `o` to open lines.
- Still unrecursive: walk does not yet recurse into directories at this point (print_file only prints name; no indentation/prefix yet).
