# Segment 5 (10:20 - 13:04) transcription

## Timeline
Setup throughout: vim in a macOS Terminal window (top-left, file `main.c`), a second Terminal (shell prompt `lab $`) that pops up in front when compiling, a Google Sheets window behind (tab "Sheet5"), and a Firefox window on beej.us (Beej's Guide to C library reference, section 26.2 `strcpy(), strncpy()`). Webcam overlay ("Human") ignored.

- [10:20] vim `main.c`, normal mode, cursor 28,14 (66%). Visible: end of `hmemcpy` (`const char *c2 = s2;` / `while (n--) *c1++ = *c2++;` / blank / `return s1;` / `}`), then `char *strcpy(char *dest, char *src) {` (NOT yet renamed to hstrcpy at this moment) with `char *p = dest;`, `while (*p++ = *src++);`, `return dest;`. main has `char s[100] = "Goats";`, `char t[100];`, `hmemcpy(t, s, 2);`, `t[2] = '\0';`. Browser shows beej 26.2 Description (strncpy text, "terminate the string yourself" example).
- [10:25] Function is now `char *hstrcpy(char *dest, char *src) {` (h prefix added). Cursor 30,7 (80%) on `hmemcpy(t, s, 2);` line; following `t[2] = '\0';` then `printf("%s\n", t);`.
- [10:36] main body now `hstrcpy(t, s);`, blank, `printf("%s\n", t);`. Status line: `"main.c" 35L, 475B written` (cursor 31,0-1, 85%).
- [10:37] Second Terminal appears in front: `lab $ cc main.c` / `lab $ ./a.out` / output `Go` / `lab $ cc ` (typing next command). (`Go` is output of the previous hmemcpy(t,s,2) build.)
- [10:38] Terminal shows compile of the new code: `main.c:20:15: warning: using the result of an assignment as a condition without parentheses [-Wparentheses]`, source line `20 |   while (*p++ = *src++);`, notes "place parentheses around the assignment to silence this warning" and "use '==' to turn this assignment into an equality comparison", `1 warning generated.`
- [10:43] Focus back on vim (`"main.c" 35L, 475B written`, 31,0-1); mouse near `*dest`.
- [10:50] Line 20 is now `while ((*p++ = *src++));` (extra parens added; matching parens highlighted cyan). Terminal cleared to just `lab $ `.
- [10:55] vim: `"main.c" 35L, 477B written`, cursor 20,24.
- [10:57] vim VISUAL mode (selection count 5, cursor 20,15) highlighting `*p++ ` (5 chars) in the while condition.
- [10:58] macOS Mission Control overview (Desktop 1, Desktop 2, Desktop 3). Shows: Terminal with code, Google Sheets, a Terminal with `lab $ cc main.c` / `lab $ ./a.out` / `Goats` (highlighted) / `lab $`, and Firefox beej. So the parenthesized code compiled cleanly and printed `Goats`.
- [10:59] Google Sheets window in front, Sheet5: row 3 merged header `"hello"`; row 4 indexes `0 1 2 3 4 5`; row 5 cells `h e \0 l o \0` (columns B..G). A cell comment popup is open on cell E6 (yellow marker): commenter `<account>`, "1:29 AM Today", text: "go to here".
- [11:01] Back to vim (VISUAL, `*p++ ` highlighted, 20,15); Sheets comment popup partially visible behind.
- [11:07] vim normal mode, cursor 21,0-1; file unchanged (`while ((*p++ = *src++));`). Browser (beej 26.2) in front.
- [11:08] Browser at table of contents (URL beej.us/guide/bgclr/html/split/): "26 <string.h> String Manipulation": 26.1 memcpy(), memccpy(), memmove(); 26.2 strcpy(), strncpy(); 26.3 strdup(), strndup(); 26.4 strcat(), strncat(); 26.5 strcmp(), strncmp(), memcmp(); 26.6 strcoll(); 26.7 strxfrm(); 26.8 strchr(), strrchr(), memchr(); 26.9 strspn(), strcspn(); 26.10 strpbrk(); 26.11 strstr(); 26.12 strtok(); 26.13 memset(), `memset_explicit; 26.14 strerror(); 26.15 strlen(). Then 27 <tgmath.h> (27.1 Example), 28 <threads.h> (call_once, cnd_* ...).
- [11:10] Browser at "26.2 strcpy(), strncpy()", "Copy a string". Synopsis: `#include <string.h>` / `char *strcpy(char *dest, char *src);` / `char *strncpy(char *dest, char *src, size_t n);`.
- [11:12] Scrolled slightly; mouse at end of strncpy prototype.
- [11:14] strncpy prototype line selected/highlighted in the browser (copied).
- [11:15] vim: cursor 23,1 on closing `}` of hstrcpy (matching `{` highlighted), 85%.
- [11:16] vim `-- INSERT (paste) --`: new line pasted: `char *strncpy(char *dest, char *src, size_t n)` (cursor 25,47, 75%), no semicolon.
- [11:18] Line 25 is `char *strncpy(char *dest, char *src, size_t n) {` with `}` on line 27; cursor 27,1 (66%).
- [11:22] INSERT mode: line 26 `  char *p = dest;` typed, cursor 29,3 (57%).
- [11:45] vim normal, cursor 41,0-1 (90%). strncpy body: `char *p = dest;` / blank / `while (n--) *p++ = *src++;` / blank / `return dest;` / `}`. main: `hstrcpy(t, s);` / `printf("%s\n", t);`.
- [11:46] Scrolled to end: `return 0;` after printf; cursor 42,11 (95%).
- [11:50] Browser in front, scrolling 26.2 page (Description text; example `char s[10];` / `char foo = "My hovercraft is full of eels."; // more than 10 chars` / `strncpy(s, foo, 9); // only copy 9 chars into positions 0-8` / `s[9] = '\0';        // position 9 gets the terminator`). Terminal title bar greyed.
- [11:51] Scrolling; mouse over "n before you get a NUL terminator on the src string".
- [11:56] Mouse at end of `"My hovercraft is full of eels."`.
- [11:57] Scrolled to Return Value ("Both functions return dest for your convenience, at no extra charge.") and start of Example (`1 #include <string.h>`, `3 int main(void)`, `4 {`, `5 char *src = "hockey hockey h...`, `6 char dest[20];`).
- [12:05] Focus on vim; browser has `s[9] = '\0';` highlighted. vim unchanged, cursor 42,11.
- [12:21] vim INSERT: main call changed to `hstrncpy(t, s, 2);`; next line being typed `t[2]` (cursor 39,7, 91%) -> `t[2] = '\0';`.
- [12:26] vim `:` command line open (about to write); view of lines ~25-44: main has `hstrncpy(t, s, 2);` / `t[2] = '\0';` / blank / `printf("%s\n", t);` / blank / `return 0;` / `}`. Definition is still named `strncpy` (no h).
- [12:27] Status `"main.c" 44L, 6..` (cut off by Terminal). Terminal: `lab $ cc main.c` / `lab $ ./a.out` / `Goats` (highlighted) / `lab $ `.
- [12:29] New compile in Terminal: warning for incompatible redeclaration of library function 'strncpy' and error for undeclared `hstrncpy` (verbatim in Shell section).
- [12:31] vim in front: `"main.c" 44L, 614B written` (cursor 44,1, "Bot").
- [12:35] Fix: line 25 now `char *hstrncpy(char *dest, char *src, size_t n) {` (cursor on the `h`, col 7). Terminal still shows previous errors.
- [12:36] Terminal shows `lab $ cc main.c` and a cursor on the next line (no output visible).
- [12:41] vim: `"main.c" 44L, 615B written`, cursor 25,7, "Bot". Terminal behind cleared.
- [12:42-12:56] Browser in front, scrolling/re-reading strncpy Description and example (`s[9] = '\0';` highlighted). vim unchanged (`"main.c" 44L, 615B written`).
- [13:04] Browser back at TOC "26 <string.h> String Manipulation" (26.1-26.15). vim unchanged behind.

## File states
main.c at end of segment (13:04). Line numbers derived from vim cursor/status and compiler diagnostics (hstrncpy def at line 25, call at line 38, 44 lines total). Lines 1-16 not visible in this segment.

```
?? (lines 1-16 not visible; includes hmemcpy, ending `return s1;` `}`)
17| char *hstrcpy(char *dest, char *src) {
18|   char *p = dest;
19|
20|   while ((*p++ = *src++));
21|
22|   return dest;
23| }
24|
25| char *hstrncpy(char *dest, char *src, size_t n) {
26|   char *p = dest;
27|
28|   while (n--) *p++ = *src++;
29|
30|   return dest;
31| }
32|
33|
34| int main(void) {
35|   char s[100] = "Goats";
36|   char t[100];
37|
38|   hstrncpy(t, s, 2);
39|   t[2] = '\0';
40|
41|   printf("%s\n", t);
42|
43|   return 0;
44| }
```
Note: line 25 read `char *strncpy(...)` (no h) from 11:16 to about 12:35 (compiler flagged redeclaration of a builtin), then renamed `hstrncpy`.

Intermediate version at 10:38 (hstrcpy test, before parens fix; only lines seen):
```
17| char *hstrcpy(char *dest, char *src) {
18|   char *p = dest;
19|
20|   while (*p++ = *src++);
21|
22|   return dest;
23| }
?? (main follows; line numbers not confirmed; main: s[100]="Goats", t[100], hstrcpy(t, s); printf("%s\n", t); ... 35 lines total)
```

Google Sheets "Sheet5" (10:59):
```
row3 (merged B-G): "hello"
row4: 0 | 1 | 2 | 3 | 4 | 5      (columns B-G)
row5: h | e | \0 | l | o | \0    (columns B-G)
cell E6: highlighted, comment "go to here"
```

## Shell & outputs
- [10:37] `lab $ cc main.c` / `lab $ ./a.out` / `Go` / `lab $ cc ` (partial)
- [10:38]
```
main.c:20:15: warning: using the result
      of an assignment as a condition without parentheses
      [-Wparentheses]
   20 |   while (*p++ = *src++);
      |          ~~~~~^~~~~~~~~
main.c:20:15: note: place parentheses
      around the assignment to silence this warning
   20 |   while (*p++ = *src++);
      |               ^
      |          (             )
main.c:20:15: note: use '==' to turn
      this assignment into an equality comparison
   20 |   while (*p++ = *src++);
      |               ^
      |               ==
1 warning generated.
lab $
```
- [10:58] `lab $ cc main.c` / `lab $ ./a.out` / `Goats` / `lab $` (clean build after adding parens).
- [12:27] `lab $ cc main.c` / `lab $ ./a.out` / `Goats` / `lab $` (old result).
- [12:29]
```
lab $ cc main.c
main.c:25:7: warning: incompatible
      redeclaration of library function 'strncpy'
      [-Wincompatible-library-redeclaration]
   25 | char *strncpy(char *dest, char *src, size_t n) {
      |       ^
main.c:25:7: note: 'strncpy' is a
      builtin with type 'char *(char *, const char *,
      unsigned long)'
main.c:38:3: error: call to undeclared
      function 'hstrncpy'; ISO C99 and later do not support
      implicit function declarations
      [-Wimplicit-function-declaration]
   38 |   hstrncpy(t, s, 2);
      |   ^
1 warning and 1 error generated.
lab $
```
- [12:36] `lab $ cc main.c` then cursor on next line (result not visible).
- vim save messages: `"main.c" 35L, 475B written` (10:36); `"main.c" 35L, 477B written` (10:55); `"main.c" 44L, 614B written` (12:31); `"main.c" 44L, 615B written` (12:41).
- Browser: beej.us/guide/bgclr/html/split/stringref... section 26.2 (Synopsis, Description, Return Value, Example) and the TOC page (11:08, 13:04) as described above.

## Observations
- Workflow: writes the function by hand, copies the prototype from Beej's Synopsis and pastes it in vim paste-insert mode, then adds the `h` prefix. Slip: the pasted `strncpy` collided with the libc builtin (warning) and the `hstrncpy` call was undeclared (error); fixed by renaming the definition to `hstrncpy`.
- Compiler lesson: `while (*p++ = *src++);` triggers `-Wparentheses`; silenced with double parentheses `while ((*p++ = *src++));`. He highlights `*p++ ` in visual mode, apparently explaining pointer increment and assignment.
- The Google Sheets "hello" memory table (indexes 0-5, `h e \0 l o \0`) with a comment "go to here" is used as a visual aid for NUL terminators in a char array. Matching the Beej advice "You can terminate the string yourself", he adds `t[2] = '\0';` after `hstrncpy(t, s, 2);`.
- strncpy implementation is the simplest K&R-style loop `while (n--) *p++ = *src++;` (does not stop at NUL or pad with zeros like real strncpy); he reads the doc "Beware! BEWARE!" text about missing NUL.
- Test harness reused: `char s[100] = "Goats"; char t[100];` with `printf("%s\n", t);`. hmemcpy test printed `Go`, hstrcpy printed `Goats`.
- References: Beej's Guide chapter 26 TOC (26.2 strcpy/strncpy; 26.4 strcat/strncat and 26.15 strlen visible, hinting at upcoming functions).
