# Segment 6 (12:55 - 13:46)

## Timeline
- [12:55] Browser (Firefox-like, floating over Terminal) tab 2 "The entire table of ANSI color c..." at gist.github.com/RabaDabaDoba/145049536f815903c79944599c6f952a (ANSI-color-codes.h), scrolled to lines 51-80: `#define YELHB "\e[0;103m"` ... `WHTHB "\e[0;107m"` (51-55); `//High intensty text` (57, sic) HBLK..HWHT `"\e[0;90m"`..`"\e[0;97m"` (58-65); `//Bold high intensity text` BHBLK..BHWHT `"\e[1;90m"`..`"\e[1;97m"` (68-75); `//Reset` (77) `#define reset "\e[0m"`, `CRESET "\e[0m"`, `COLOR_RESET "\e[0m"` (78-80). Behind the browser, the zsh Terminal still shows the earlier compile output (visible left edge: "lab $ cc", "main.c:6", "main.c:2", "2 warnin", "lab $"; right edge "ISO C99").
- [12:56-12:57] Scrolled to top of gist: "GitHubGist", title "RabaDabaDoba / ANSI-color-codes.h", "Forked from iamnewton/bash-colors.md", "The entire table of ANSI color codes working in C!"; lines 1-13: license comment ("This is free and unencumbered software released into the public domain. ... <https://unlicense.org>"), `//Regular text`, `#define BLK "\e[0;30m"`, RED 31m, GRN 32m, YEL 33m, BLU 34m, MAG 35m...
- [12:59-13:00] Navigated back to Google results for "c print ansi color" (www.google.com/search?client=firefox-b-d&q=c+print+ansi+color). Results: StackOverflow "How can I use ANSI Escape Codes for outputting colored ..." (+ sub-links "stdlib and colored output in C", "ANSI colors in C and ncurses"), GitHub gist "The entire table of ANSI color codes working in C!" (visited/purple), c-for-dummies.com "Color Text, Part I", Medium "Adding Color to Your Output From C | by Selvaraj kuppusamy". Google: "Can't generate an AI overview right now."
- [13:01-13:02] Clicked the StackOverflow result (stackoverflow.com/questions/7414983/how-can-i-use-ansi-escape-codes-for-outputting-colored-text-in-c-and-c). Question "How can I use ANSI Escape Codes for outputting colored text in C and C++?" asked 15 years ago, 14 votes.
- [13:03] Question body: C++ attempt with `printf("\155\32m %d",a);` (no ESC char); `textcolor(10);` mention.
- [13:04-13:05] Scrolled past question; top answer (14 votes, accepted): "I'm afraid you forgot the ESC character:" code `#include <cstdio>` / `int main()` / `{` / `    printf("%c[%dmHELLO!\n", 0x1B, 32);` / `}`. "Unfortunately it will only work on consoles that support ANSI escape sequences (like a console using bash, or old Windows consoles that used ansi.sys)". Comment from goldenmean: prints `<-[32mHELLO!`.
- [13:07] Comments scrolled: ded32 "Alternatively, you can use \xNN notation to define a character code for ESC in the string... 0x1B == 27 decimal... printf ("\x1B" "[%dm" "HELLO!\n", 32);" ; TJ Bandrowsky about Windows 10+ supporting it.
- [13:08-13:09] Second answer (8 votes, Baltasarq): `static const char * CSI = "\33[";` / `printf( "%s%s", CSI, "31m" );   // RED`. Comments about Windows consoles needing API calls.
- [13:10-13:12] Went back to Google results; scrolled down: DEV Community "Enhancing Your C++ Terminal Output with ANSI Escape ...", Scribd "ANSI Color Codes in C Programming | PDF".
- [13:13-13:15] Clicked Medium result "Adding Color to Your Output From C" (medium.com/@selvarajk/adding-color-to-your-output-from-c-58f1a4dc4...), Dec 22, 2021, 3 min read. Loading.
- [13:16-13:17] Medium sign-up popup ("Be the first to hear about new stories from Selvaraj kuppusamy") appeared; mouse moves to close (X) at [13:17].
- [13:18] Article body: "Simple Hello world in Color" ... code starts `include<stdio.h>` (sic).
- [13:19] Article bullets: `printf("\033[0;31m"); //Set the text to the color red`, `printf("Hello\n"); //Display Hello in red`, `printf("\033[0m"); //Resets the text to default color`, "Escape is: \033", "Color code is: [0;31m".
- [13:20-13:21] Article code: `include<stdio.h>` `int main()` `{` `printf("\033[1;33m");` `printf("Hello world\n");` `printf("\033[0m");` `return 0;` `}` ; screenshot "selvaraj@selvaraj-ubuntu:~$ ./selva / Hello world" in yellow. I-beam hovers on `\033[1;33m`. "For BOLD just replace the initial 0 before the ; with a 1. BOLD green would be \033[1;32m".
- [13:23-13:24] Article color list: Black \033[0;30m, Red [0;31m, Green [0;32m, Yellow [0;33m, Blue [0;34m, Purple [0;35m, Cyan [0;36m, White [0;37m.
- [13:27] Text `\033[0;34m` (Blue entry) selected on the page (Medium Highlight/Share popup shown).
- [13:28] "Using Functions" section: "This is where functions can help. It is simple to create functions for red, yellow etc. Let's take a look, assuming we create the source file color.c." code `#include <stdio.h>` `void yellow () {` `printf("\033[1;33m");` `}` `void red () {` `printf("\033[1;31m");` ...
- [13:29-13:31] Rest of article code: `void reset () { printf("\033[0m"); }`, `int main () { yellow(); printf("Selva"); red(); printf("raj\n"); reset(); return 0; }`; screenshot `$ gcc color.c -o color` / `$ ./color` / "Selvaraj" (Selva yellow, raj red).
- [13:32-13:35] Scrolling: "We can use the newly created functions as many time as we want and it is as simple as yellow(); reset(); or red(); ... Using the functions the way we have we do not need to return any values. The functions result only in printing the ANSI color codes to the terminal." (tags C Programming, Linux; 6 claps). Scrolled back up to the `void red/reset/main` code.
- [13:37] Switched to Terminal, zsh tab ("~/lab -- zsh"; other tab "~/lab -- vim README.md"). Shows previous compile output (see Shell section): `cc main.c` with 2 warnings about non-standard escape `\e` and 1 error for undeclared `strcmp`.
- [13:38] Switched to vim tab. Shows lines 6-45 (see file state). Cursor at end of line 29 (`;` highlighted). Tab title says "vim README.md" but content is C code (title likely stale/wrong; compiler output says main.c).
- [13:39] Scrolled to top: lines 1-40. Line 6 still `\e[1;34m%s\e[0m\n`. Cursor still on `;` of line 29.
- [13:44] EDIT: line 6 now `printf("\033[0;34m%s\e[0m\n", name);` -- first `\e[1;34m` replaced with `\033[0;34m` (regular blue instead of bold blue; value as in Medium list). Second escape (`\e[0m`) still `\e`. Cursor block on the `m` after `34`.
- [13:45] Desktop/Mission-Control overview: browser window (Medium) bottom-left, Terminal with vim top-right (line 6 as at 13:44); tooltip "Adding Color to Your Output From C | by Selvaraj kuppusamy | Medium".
- [13:46] Back to browser (Medium, code void red/reset/main), vim visible behind at left edge; I-beam on `reset`. No further vim change seen.

## File states
File: main.c (vim tab titled "vim README.md"; filename inferred from compiler output). State at END of segment (13:44-13:46). 2-space indent. Lines 41-45 taken from the 13:38 frame; line 46+ never visible.

```
 1| #include <stdio.h>
 2| #include <dirent.h>
 3| 
 4| void print_file(const char *name, int is_dir) {
 5|   if (is_dir)
 6|     printf("\033[0;34m%s\e[0m\n", name);
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
29|       print_file(name, 0);
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
?? (lines 46+ not visible in this segment)
```
Intermediate version (13:38-13:39): identical except line 6 = `    printf("\e[1;34m%s\e[0m\n", name);`.
Notes: `#include <string.h>` absent (causes the strcmp error). Line 26 has a stray space `DT_DIR ) {`. Line 22 comment "// ignore files" (refers to "." and ".."). Line 38 comment "// default path is current dir".

## Shell & outputs
Zsh tab, prompt `lab $ `. No new commands typed in this segment; the output below was on screen from before (partly visible at left edge from 12:55, fully visible at 13:37). Verbatim at [13:37]:
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
(Caret column positions approximate.)

## Observations
- Clang -Wpedantic flags `\e` as non-standard; the programmer researches portable alternatives (gist table of `\e` defines, StackOverflow answers using 0x1B / `\x1B` / `\33[`, Medium article using `\033`) and picks `\033[0;34m` (regular blue) from the Medium list. Only the first escape is changed by 13:44; the `\e[0m` reset is still `\e` at end of segment (likely next edit).
- Changed bold blue (`1;34`) to regular blue (`0;34`).
- The strcmp error (missing `<string.h>`) is still unfixed at end of segment.
- Style: 2-space indent, braces on same line, brace-less single-statement if/else bodies, early return with perror, `const char *`, magic 0/1 int flag for is_dir, hardcoded ANSI literals inline (he saw a #define color header and a helper-function approach in the Medium article but added neither in this segment). Only stdio.h and dirent.h so far. Minimalist.
