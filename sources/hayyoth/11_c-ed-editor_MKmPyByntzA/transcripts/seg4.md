# Segment 4 (08:17 - 12:27)

Layout throughout: left Terminal window with two tabs ("~/lab — vim main.c" and a second tab whose title changes: "-zsh", "less ◂ man 3 strcspn", "a.out", "less ◂ man 3 strcmp"); right Terminal window (vim, static) showing the notes file lines 15-35 (35,1 Bot); webcam "Human" overlay ignored.

## Timeline

- [08:17] Left, tab 2 "~/lab — -zsh". Scrollback: `lab $ man 3 fgets`, `lab $ cc main.c`, `lab $ ./a.out`, `hello`, `lab $ cc main.c`, `lab $ ./a.out`, `hello`,`hello`, blank, `abc`,`abc`, blank, `def`,`def`, blank, `q`,`q`, blank, `^C`, then prompt `lab $ ma` (typing "man ...").
- [08:23] Tab 2 title "~/lab — less ◂ man 3 strcspn". Shows STRSPN(3) page: NAME "strspn, strcspn — span a string"; SYNOPSIS `#include <string.h>`, `size_t strspn(const char *s, const char *charset);`, `size_t strcspn(const char *s, const char *charset);`. Bottom `:` prompt of less.
- [08:29]-[08:31] Scrolling in man page: SEE ALSO `memchr(3), strchr(3), strpbrk(3), strrchr(3), strsep(3), strstr(3), strtok(3), wcsspn(3)`; STANDARDS "conform to ISO/IEC 9899:1990 ("ISO C90")". Mouse hovering over `strspn` in synopsis at 08:31.
- [08:36]-[08:44] Man page scrolled so LIBRARY is at top. 08:38 strspn description paragraph selected with mouse; 08:41 strcspn paragraph ("The strcspn() function spans the initial part of the null-terminated string s as long as the characters from s do not occur in the null-terminated string charset (it spans the complement of charset). In other words, it computes the string array index of the first character of s which is also in charset, else the index of the first null character.") highlighted; 08:43 the word `strcspn` highlighted. Return values: "return the number of characters spanned."
- [08:44] Switched to tab 1 "~/lab — vim main.c", INSERT mode at 11,5. main.c (16 lines): line 3 `#define MAX_LINEs 1024` (lowercase "s"), line 4 `#define MAX_LINE_LEN 1024`, `char cmd[MAX_LINE_LEN];`, while loop with `if (!fgets(cmd, MAX_LINE_LEN, stdin)) break;`, empty line 11 (cursor, indented), `puts(cmd);`, `return 0;`.
- [09:07] Back to man strcspn tab; `strcspn` in second paragraph highlighted.
- [09:11] vim main.c, "-- INSERT (paste) --" at 11,30. Line 11 is `cmd[strcspn(cmd, "\r\n")[` (the last `[` apparently a typo/in progress, `] = 0;` not yet), `puts(cmd);` on line 12 (the `;` shown with red highlight).
- [09:24] main.c 17 lines, INSERT at 11,7. Line 11 `//` (comment being started), line 12 `cmd[strcspn(cmd, "\r\n")] = 0;` (fixed), line 13 `puts(cmd);`.
- [09:33] main.c 18 lines, INSERT at 2,4. Line 2 being typed `#in` (to add #include), line 12 `// remove "\n" from cmd` (completed comment), line 13 `cmd[strcspn(cmd, "\r\n")] = 0;`, line 14 `puts(cmd);`. Line 4 `#define MAX_LINEs 1024`.
- [09:37] Tab 2 shows man strcspn page again (strcspn highlighted).
- [09:38] Tab 2 "-zsh": scrollback as at 08:17, then `lab $   man 3 strcspn` (history echo) and new prompt `lab $ `.
- [09:39] Tab 2 cleared; `lab $ cc m` (typing).
- [09:49] Tab 1 vim, status `"main.c" 18L, 296B written` at 2,19. Line 2 now `#include <string.h>`. Tab 2 title "~/lab — a.out".
- [10:00] INSERT at 15,31. main.c 19 lines; new line 15: `printf("%s - %zu\n", cmd, ` (typing, to add strlen). Line 14 still `puts(cmd);`. (Dash rendered as en-dash glyph in this font; ASCII `-` assumed, cannot distinguish.)
- [10:05] main.c 18 lines, normal mode 14,5: line 12 `// remove "\n" from cmd`, line 13 `cmd[strcspn(cmd, "\r\n")] = 0;`, line 14 `printf("%s - %zu\n", cmd, strlen(cmd));` (puts line deleted).
- [10:06] main.c 17 lines, normal mode 12,5: line 11 fgets, line 12 `cmd[strcspn(cmd, "\r\n")] = 0;`, line 13 `printf("%s - %zu\n", cmd, strlen(cmd));`. (Comment line deleted.)
- [10:08] Tab 2 "~/lab — a.out" running: `lab $ cc main.c`, `lab $ ./a.out`, then lines `abc abc def def hello hello q q q▮` (older puts-echo binary output; cursor after the third q).
- [10:19] Tab 2 "-zsh": same, `q^C`, `lab $ cc main.c`, `lab $ ./a.out`, output `abc`, `abc - 3`, `q`, `q - 1`, `ajsskl`, `ajsskl - 6`, `q`, `q - 1`, `^C`, `lab $ `.
- [10:20] Tab 1 vim: `"main.c" 17L, 297B written` at 12,5. File same as 10:06.
- [10:30] vim main.c 18 lines: after line 12 (`cmd[strcspn...] = 0;`), line 13 blank, line 14 blank (cursor 14,0-1), printf line removed.
- [10:38] Tab 2 zsh with same scrollback as 10:19.
- [10:40] Tab 2 cleared: `lab $ man 3 ` (typing).
- [10:42] Tab 2 "~/lab — less ◂ man 3 strcmp": STRCMP(3): NAME "strcmp, strncmp — compare strings"; SYNOPSIS `int strcmp(const char *s1, const char *s2);` `int strncmp(const char *s1, const char *s2, size_t n);`. DESCRIPTION: "The strcmp() and strncmp() functions lexicographically compare the null-terminated strings s1 and s2. The strncmp() function compares not more than n characters. Because strncmp() is designed for comparing strings rather than binary data, characters that appear after a '\0' character are not compared." RETURN VALUES: "return an integer greater than, equal to, or less than 0, according as the string s1 is greater than, equal to, or less than the string s2. The comparison is done using unsigned characters, so that '\200' is greater than '\0'." SEE ALSO bcmp(3), memcmp(3), strcasecmp(3), strcoll(3), strxfrm(3), wcscmp(3).
- [10:48] Same page, sentence "compare the null-terminated strings s1 and s2." being selected.
- [10:50] vim main.c INSERT at 14,16: line 12 `cmd[strcspn(cmd, "\r\n")] = 0;`, line 13 blank, line 14 `if (strcmp(` (typing). 18 lines.
- [11:06] man strcmp page, full sentence highlighted.
- [11:07] Tab 2 zsh: `lab $ man 3 strcmp`, `lab $ cc m` (typing).
- [11:15] vim main.c: `"main.c" 18L, 292B written` at 14,37. Line 14 `if (strcmp(cmd, "q") == 0) break;`.
- [11:26] INSERT at 8,1. main.c 20 lines: lines 6,7,8 blank (making room above main), line 9 `int main(void) {`, line 16 `if (strcmp(cmd, "q") == 0) break;`.
- [11:37] INSERT 8,5. main.c 21 lines: line 7 `int mode(char *cmd, char *mode) {`, line 8 `re` (typing return), line 9 blank, line 10 `int main(void) {`.
- [11:54] Normal mode 9,5, 22 lines: line 8 `  return (strcmp(cmd, mode) == 0);`, line 9 `    printf("%s - %zu\n", cmd, strlen(cmd));` (stray old printf line pasted, indented 4), line 10 blank, line 11 `int main(void) {`; line 18 still `if (strcmp(cmd, "q") == 0) break;`.
- [11:55] Normal mode 8,10: status `1 line less; be...28  1 second ago` (undo); 21 lines: stray printf gone, line 9 blank, line 10 `int main(void) {`.
- [11:56] INSERT 9,3: cursor on line 9, about to type `}`.
- [12:16] Normal mode 18,17, 23 lines: line 9 `}` added. Line 18 `if (mode(cmd, "q")) break;`, line 19 `else if (` (typing).
- [12:27] INSERT 20,28: loop is `18 if (mode(cmd, "q"))`, `19 break;`, `20 else if (mode(cmd, "a")` (typing; `)` after "a" highlighted as matching paren).

## File states

### main.c at end of segment (~12:27)
```
 1| #include <stdio.h>
 2| #include <string.h>
 3| 
 4| #define MAX_LINEs 1024
 5| #define MAX_LINE_LEN 1024
 6| 
 7| int mode(char *cmd, char *mode) {
 8|   return (strcmp(cmd, mode) == 0);
 9| }
10| 
11| int main(void) {
12|   char cmd[MAX_LINE_LEN];
13| 
14|   while (1) {
15|     if (!fgets(cmd, MAX_LINE_LEN, stdin)) break;
16|     cmd[strcspn(cmd, "\r\n")] = 0;
17| 
18|     if (mode(cmd, "q"))
19|       break;
20|     else if (mode(cmd, "a")⟨?⟩   <- incomplete, being typed
21|   }
22| 
23|   return 0;
24| }
```
Indentation 2 spaces per level.

Last saved checkpoint (11:15, `"main.c" 18L, 292B written`):
```
 1| #include <stdio.h>
 2| #include <string.h>
 3| 
 4| #define MAX_LINEs 1024
 5| #define MAX_LINE_LEN 1024
 6| 
 7| int main(void) {
 8|   char cmd[MAX_LINE_LEN];
 9| 
10|   while (1) {
11|     if (!fgets(cmd, MAX_LINE_LEN, stdin)) break;
12|     cmd[strcspn(cmd, "\r\n")] = 0;
13| 
14|     if (strcmp(cmd, "q") == 0) break;
15|   }
16| 
17|   return 0;
18| }
```

Intermediate 09:49 (`"main.c" 18L, 296B written`): lines 1-10 as above, then
```
11|     if (!fgets(cmd, MAX_LINE_LEN, stdin)) break;
12|     // remove "\n" from cmd
13|     cmd[strcspn(cmd, "\r\n")] = 0;
14|     puts(cmd);
15|   }
16| 
17|   return 0;
18| }
```
Intermediate 10:20 (`"main.c" 17L, 297B written`): lines 11-14:
```
11|     if (!fgets(cmd, MAX_LINE_LEN, stdin)) break;
12|     cmd[strcspn(cmd, "\r\n")] = 0;
13|     printf("%s - %zu\n", cmd, strlen(cmd));
14|   }
```

### Notes/reference file (right terminal, static; lines 1-14 not visible)
```
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
(cursor 35,1 Bot; unchanged all segment; exact leading spaces on lines 15-18 approximate.)

## Shell & outputs
- [08:17] scrollback: `lab $ man 3 fgets`; `lab $ cc main.c`; `lab $ ./a.out` -> `hello`; `lab $ cc main.c`; `lab $ ./a.out` -> `hello hello`, `abc abc`, `def def`, `q q`, `^C`. Then `lab $ ma...` -> `man 3 strcspn` (less pager).
- [09:38]-[09:39] `lab $   man 3 strcspn` in history; `lab $ cc m...`.
- [10:08] `lab $ cc main.c`, `lab $ ./a.out`; puts-echo output: `abc abc def def hello hello q q q` then `^C`.
- [10:19] `lab $ cc main.c`; `lab $ ./a.out`:
  ```
  abc
  abc - 3
  q
  q - 1
  ajsskl
  ajsskl - 6
  q
  q - 1
  ^C
  lab $
  ```
- [10:40]-[10:42] `lab $ man 3 ` -> `man 3 strcmp` (less).
- [11:07] `lab $ man 3 strcmp`, then `lab $ cc m` (typing; result of the q-quits build not seen in this segment).
- No compiler errors/warnings visible.

## Observations
- Build order: fgets read loop -> strip newline with `cmd[strcspn(cmd, "\r\n")] = 0;` (consults man 3 strcspn at length, selecting text with mouse) -> debug print `printf("%s - %zu\n", cmd, strlen(cmd))` to verify stripping (lengths 3,1,6,1) -> debug removed -> `if (strcmp(cmd, "q") == 0) break;` after reading man 3 strcmp -> refactor to helper `int mode(char *cmd, char *mode) { return (strcmp(cmd, mode) == 0); }` -> chain `if (mode(cmd, "q")) break; else if (mode(cmd, "a") ...` (start of command dispatch).
- Frequent tab switching between vim and shell; compile with plain `cc main.c`, `./a.out`, stop with ^C.
- Style: 2-space indent, K&R braces, `break;` on its own line in if/else chain; `MAX_LINEs` macro typo (lowercase s) retained and unused so far; parameter `mode` shadows function name `mode`.
- Comment `// remove "\n" from cmd` written then deleted; vim paste mode appeared once.
