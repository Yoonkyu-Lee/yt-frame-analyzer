# Segment 3 (05:09 - 08:17)

## Timeline
Layout: left Terminal window (title "Terminal", tabs "~/lab — vim main.c" and a second tab), right Terminal window (notes file in vim, lines 15-35 visible, cursor 35,1 Bot, unchanged all segment). Webcam "Human" overlay ignored.

- [05:09] Left: vim main.c, `-- INSERT --` 3,3 All, 14 lines. Line 3 being typed: `//` (comment start). Line 4 `#define MAX_LINEs 1024`, line 5 `#define MAX_LINE_LEN 1024`. Rest: `int main(void) {`, `while (1) {`, `}`, `return 0;`, `}`.
- [05:24] Normal mode, 4,1 All. The `//` line deleted: file now 13 lines (1 `#include <stdio.h>`, 2 blank, 3 `#define MAX_LINEs 1024`, 4 `#define MAX_LINE_LEN 1024`, 5 blank, 6 `int main(void) {`, 7 blank, 8 `  while (1) {`, 9 blank, 10 `  }`, 11 blank, 12 `  return 0;`, 13 `}`). Cursor on `#` of line 4.
- [05:38] INSERT 7,22. New line 7 `  char cmd[MAX_LINE_L` being typed (file 14 lines).
- [05:49] Tab bar now visible: tabs "~/lab — vim main.c" (active) and "Terminal" (spinner). Line 7 completed: `  char cmd[MAX_LINE_LEN];`. INSERT 10,5, cursor in while body (line 10 empty). View scrolled (lines 3-14 shown).
- [05:50] Switched to second tab "~/lab — -zsh": prompt `lab $ man ` being typed.
- [05:52] Second tab title "~/lab — less ◂ man 3 fgets": FGETS(3) Library Functions Manual. NAME: fgets, gets — get a line from a stream. LIBRARY: Standard C Library (libc, -lc). SYNOPSIS: `#include <stdio.h>`; `char * fgets(char * restrict str, int size, FILE * restrict stream);`; `char * gets(char *str);`. DESCRIPTION text. Prompt `:`.
- [05:56] Same page; mouse selecting the fgets() prototype (3 lines highlighted).
- [05:58] Same page, selection cleared, mouse right of prototype.
- [06:06]-[06:07] Scrolling down man page (RETURN VALUES: "pointer to the string. If end-of-file occurs before any characters are read, they return NULL and the buffer contents remain unchanged. If an error occurs, they return NULL and the buffer contents are indeterminate.").
- [06:15]-[06:16] Further scroll: "The fgets() and gets() functions do not distinguish between end-of-file and error, and callers must use feof(3) and ferror(3) to determine which occurred."; ERRORS [EBADF] "The given stream is not a readable stream."; fgets "may also fail and set errno for any of the errors specified for the routines fflush(3), fstat(2), read(2), or malloc(3)."; gets() "...getchar(3)."; SECURITY CONSIDERATIONS.
- [06:23] End of page: SECURITY CONSIDERATIONS ("The gets() function cannot be used securely. ... It is strongly suggested that the fgets() function be used in all cases. (See the FSA.)"); SEE ALSO feof(3), ferror(3), fgetln(3), fgetws(3), getline(3); STANDARDS "conform to ISO/IEC 9899:1999 ("ISO C99")."; footer `macOS 26.7   June 4, 1993   macOS 26.7`; `(END)`.
- [06:24] Scrolled back to top of man page.
- [06:27]-[06:29] macOS Dock appears at bottom (Finder, Launchpad, ..., Firefox hovered with tooltip "Firefox", Terminal, ..., "exec" icon, Trash). Clicks Firefox.
- [06:30] Firefox window opens; URL bar typed `beej.us/` with suggestion "Beej's Web Page beej.us".
- [06:31] Dropdown: Firefox Suggest list (YouTube items "How we (Human) learn better than AI? - YouTube" youtube.com/watch?v=48DqkAx86Vg incl. &t=390s and &t=12s variants, "Channel analytics - YouTube Studio", Proton Mail inbox <mail address>, "The Creation of Chain - Linked List. Conjured..." youtube.com/watch?v=iz2j9_AwQdQ, "Beej's Guide to C Programming Library beej.us/guide/bgclr/html/split/", "Beej's Guides beej.us/guide/").
- [06:32] URL typed `beej.us/guide/`.
- [06:33] URL typed `beej.us/guide/bgclr/`.
- [06:34]-[06:35] Page loaded: "Beej's Guide to C Programming—Library Reference" (second volume); follows link "HTML" to beej.us/guide/bgclr/html/split/: title page "Beej's Guide to C Programming, Library Reference, Brian "Beej Jorgensen" Hall, v0.9.13, Copyright © October 15, 2026"; contents list.
- [06:37] Firefox find bar open, typed `f`.
- [06:38] Find bar `fgets`; contents scrolled to "23 <stdio.h> Standard I/O Library", item 23.15 `gets(), fgets()` highlighted.
- [06:39]-[06:41] Opened .../split/stdio.html#man-gets: "23.15 gets(), fgets() — Read a string from console or file". Synopsis: `#include <stdio.h>` / `char *fgets(char *s, int size, FILE *stream);` / `char *gets(char *s);`. Description (don't use gets(); fgets reads at most size-1 characters and NUL-terminates; gets discards newline, fgets stores it).
- [06:43]-[06:46] Scrolling: example `char s[100];` / `gets(s);  // don't use this--read a line (from stdin)` / `fgets(s, sizeof(s), stdin); // read a line from stdin`; then the guide's `ngets()` helper:
  ```
  #include <stdio.h>
  #include <string.h>

  char *ngets(char *s, int size)
  {
      char *rv = fgets(s, size, stdin);

      if (rv == NULL)
          return NULL;

      char *p = strchr(s, '\n');  // Find a newline

      if (p != NULL)  // if there's a newline
          *p = '\0';  // truncate the string there

      return s;
  }
  ```
  "So, in summary, use fgets() to read a line of text from the keyboard or a file, and don't use gets()." Return Value: "Both gets() and fgets() return a pointer to the string passed. On error or end-of-file, the functions return NULL." Then "Example" heading.
- [07:03] Terminal (man-page tab) brought to front over Firefox.
- [07:04] Switched to tab "~/lab — vim main.c": INSERT 10,5, line 10 empty (cursor in while body). Second tab title now "~/lab — less • man 3 fgets".
- [07:27] Right terminal in front. vim main.c now 15 lines, INSERT 11,5: line 10 `    if (fgets(cmd, MAX_LINE_LEN, stdin)) break;`, line 11 blank (cursor).
- [07:42] Second tab "~/lab — -zsh": `lab $ man 3 fgets`, blank, new prompt `lab $ ` (man quit).
- [07:50] vim main.c: line 11 now `    puts(cmd);` (cursor on `;`, 11,14). Message `"main.c" [New] 15L, 212B written`.
- [08:02] vim main.c: line 10 changed to `    if (!fgets(cmd, MAX_LINE_LEN, stdin)) break;`; command line shows `:` (pending :w).
- [08:03] -zsh tab: `lab $ cc main.c`, `lab $ ./a.out`, `hello`, new prompt (see Shell).
- [08:13] vim main.c, normal mode, cursor 10,9 on `!`: message `"main.c" 15L, 213B written`.
- [08:16] vim main.c INSERT 11,5: new blank indented line 11 opened above `puts(cmd);` (now line 12); file 16 lines.
- [08:17] -zsh tab: second compile/run session output (see Shell). Prompt `lab $ ma` being typed (likely `man`).

## File states

### main.c (vim) at end of segment (08:16 frame; last saved version is 15L, 213B written at 08:13)
```
 1| #include <stdio.h>
 2| 
 3| #define MAX_LINEs 1024
 4| #define MAX_LINE_LEN 1024
 5| 
 6| int main(void) {
 7|   char cmd[MAX_LINE_LEN];
 8| 
 9|   while (1) {
10|     if (!fgets(cmd, MAX_LINE_LEN, stdin)) break;
11|     
12|     puts(cmd);
13|   }
14| 
15|   return 0;
16| }
```
Line 11 is the new blank line opened in INSERT mode (cursor col 5, unsaved). Indent is 2 spaces inside main (`char`, `while`, `return`), 4 spaces inside the while body (`if`, `puts`). Typo `MAX_LINEs` on line 3 never corrected in this segment.

Intermediate at 07:50 (saved, 15L, 212B): line 10 `    if (fgets(cmd, MAX_LINE_LEN, stdin)) break;`, line 11 `    puts(cmd);`, 12 `  }`, 13 blank, 14 `  return 0;`, 15 `}`; rest as above.

Intermediate at 05:09 (14 lines): 1 `#include <stdio.h>`, 2 blank, 3 `//`, 4 `#define MAX_LINEs 1024`, 5 `#define MAX_LINE_LEN 1024`, 6 blank, 7 `int main(void) {`, 8 blank, 9 `  while (1) {`, 10 blank, 11 `  }`, 12 blank, 13 `  return 0;`, 14 `}`.

### Notes/reference file (right terminal), unchanged this segment
```
 1-14| ?? (not visible)
15|     - runs via scripts/pipes without a human observer.
16|     - AI love ed
17| * Evolution tree:
18|     - ed -> sed -> ex -> vi -> vim.
19| 
20| ----------------------------------------
21| [num] - go to line
22|   $ - last line
23| 
24| i - Insert text before the current line
25| a - Append text after the current line
26|   . - Stop edit
27| 
28| d - Delete the current line
29| p - Print the current line
30|   ,p - Print all lines
31|   1,$p - Print all lines
32| w - Write the buffer to a file
33| q - Quit the editor
34| 
35| ed [path] - Read file to buffer
```
(Dashes in the description separators render as en-dash glyphs; cursor 35,1 Bot.)

## Shell & outputs
- [05:50] `lab $ man ` (typing, tab 2 -zsh)
- [05:52] `man 3 fgets` -> less, tab title "~/lab — less ◂ man 3 fgets"; FGETS(3) paged to `(END)` at 06:23 (footer `macOS 26.7  June 4, 1993  macOS 26.7`).
- [07:42] after quitting less:
  ```
  lab $ man 3 fgets

  lab $ 
  ```
- [08:03]
  ```
  lab $ man 3 fgets

  lab $ cc main.c
  lab $ ./a.out
  hello
  lab $ 
  ```
  (no compiler warnings/errors shown; with the first buggy `if (fgets(...)) break;` the program echoes `hello` once then exits)
- [08:17]
  ```
  lab $ man 3 fgets

  lab $ cc main.c
  lab $ ./a.out
  hello
  lab $ cc main.c
  lab $ ./a.out
  hello
  hello

  abc
  abc

  def
  def

  q
  q

  ^C
  lab $ ma
  ```
  Second run (after `!` fix): each typed line is echoed once by puts plus a blank line (retained newline + puts newline); `q` does not quit; ended with Ctrl-C.
- Browser URL bar (not shell): `beej.us/`, `beej.us/guide/`, `beej.us/guide/bgclr/`, `beej.us/guide/bgclr/html/split/`, `.../stdio.html#man-gets`.

## Observations
- Segment starts with the skeleton (MAX_LINE_LEN define, main, while(1)) already present; he deletes a stray `//` line, adds `char cmd[MAX_LINE_LEN];`.
- First feature step: the fgets read loop. He consulted `man 3 fgets` (read through return values, errors, gets() security note) and then Beej's Guide to C Library Reference section 23.15 (including the ngets() newline-chop helper), foreshadowing newline stripping.
- Built an echo loop first (`puts(cmd)`) and tested with `cc main.c` + `./a.out`. First version `if (fgets(...)) break;` was inverted (exits right after first line); fixed to `!fgets(...)` and saved (213B). Incremental build-and-test style.
- Output showed doubled newlines (fgets keeps newline, puts adds one); he then opens a blank line above `puts(cmd);` (08:16), probably to add newline handling/command parsing.
- Notes file (ed command list) serves as the spec; not edited this segment.
- Style: 2-space indent, K&R braces, `while (1)` with break, `!fgets` NULL check, MAX_LINE_LEN constant; leftover unused typo define `MAX_LINEs`. Compiles with plain `cc main.c`.
- Checksums: `"main.c" [New] 15L, 212B written` (07:50); `"main.c" 15L, 213B written` (08:13).
