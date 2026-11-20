# Segment 8 (16:38 - 19:56) - hstrcmp written and tested

## Timeline
- [16:38-16:39] Layout: vim (macOS Terminal window titled "Terminal") at left, `-- VISUAL --` mode, selection `(*p++ = *s2++)` on the second `while` of hstrcat (position `36,22`, "13" chars selected). Firefox window (beej.us/guide/bgclr/html/split/stringref...) at right, Beej section "strcmp(), strncmp(), memcmp()": "Compare two strings or memory regions and return a difference". Synopsis: `int strcmp(const char *s1, const char *s2);`, `int strncmp(const char *s1, const char *s2, size_t n);`, `int memcmp(const void *s1, const void *s2, size_t n);`. A second Firefox window behind has a docs.google.com/spreadsheets URL (not in view). A second (shell) Terminal window sits behind. Webcam overlay ignored.
- [16:42-16:43] Terminal focused. He selected the strcmp prototype on the Beej page (highlighted) and pasted it into vim (`-- INSERT (paste) --`, `41,43`, 96%): new line `int strcmp(const char *s1, const char *s2)` (no `{` yet; still `strcmp`, not `hstrcmp`). hstrcat second while now reads `while ((*p++ = *s2++));`.
- [16:45] Added ` {` and a closing `}` on its own line: `int strcmp(const char *s1, const char *s2) {` / blank / `}` (cursor `43,1`, 91%).
- [16:46-16:48] Switches to Firefox: heading "26.5 strcmp(), strncmp(), memcmp()". He highlights the first sentence "Compare two strings or memory regions and return a difference". Description visible: "All these functions compare chunks of bytes in memory. strcmp() and strncmp() operate on NUL-terminated strings, whereas memcmp() will compare the number of bytes you specify, brazenly ignoring any NUL characters it finds along the way." "strcmp() compares the entire string down to ... compares the first n characters of the strings ..." "It's a little funky what they return. Basically it's ... strings are the same, it'll return zero (since th... zero if the strings differ; basically it will find th..." (cut off by windows).
- [16:50] Back in vim INSERT (`43,3`, 88%): typed `  while (*s1 && *s2 && (*s1 == *s2));` (empty-body while ending in `;`), cursor on the next line.
- [17:10-17:11] Typing `re` on a new line after the while (`44,5`, 86%), start of `return`.
- [17:28] New comment line `  //return *s1 - *s2; // it can have negative`, cursor on next line (`45,3`, 83%).
- [17:43] Body complete: `  return (unsigned char)*s1 - (unsigned char)*s2;` below the commented-out `//return *s1 - *s2; // it can have negative`. Cursor `58,1` on last `}` of file (`Bot`) - he scrolled to end; main() still holds the hstrcat test (`char dest[30] = "Hello";`, `char *src = ", World!";`, `//char numbers[] = "12345678";`, `hstrcat(dest, src);`, `printf("dest after strcat:  \"%s\"\n", dest); // "Hello, w...` truncated, `return 0;`).
- [17:48] Function renamed to `int hstrcmp(const char *s1, const char *s2) {`. In main(), the first 4 lines (dest, src, numbers comment, blank) are selected (`-- VISUAL LINE --`, `4`, `52,0-1`).
- [17:49] After delete: status `6 fewer lines` (`49,0-1`). main() reduced to `int main(void) {`, blank (cursor), blank, `return 0;`, `}`; `~` filler lines show file end.
- [18:05] Test typed: `  printf("%d\n", hstrcmp("hello", "hello"));` Status: `"main.c" 52L, 863B written` (`49,43`). A second (shell) Terminal window is in front: `lab $ cc main.c` / `lab $ ./a.out` / `dest after strcat:  "Hello, World!"` / `lab $ cc` (typing). That output is from the OLD binary.
- [18:23] Shell window moved. Content: `lab $ ./a.out` / `^C` / `lab $ cc main.c` / `lab $ ./a.out` / blank with cursor (hanging; infinite loop from empty-body while). Vim status still `"main.c" 52L, 863B written`.
- [18:25] Back to vim, shell window behind (partially blank). Same code, cursor `49,43`.
- [18:31] Fix in progress: `  while (*s1 && *s2 && (*s1 == *s2)) {` then `    ;` (cursor `43,5`, INSERT).
- [18:35] Typing `    s1++;` then `    s2` (INSERT, `44,7`).
- [18:37] Body complete: `    s1++;` / `    s2++;` / `  }` (matching brace highlighted, `45,3`, normal mode).
- [18:39] Status `"main.c" 55L, 888B written`. Shell in front: `lab $ ./a.out` / `^C` / `lab $ cc main.c` / `lab $ ./a.out` / `^C` / `lab $ ` (second Ctrl-C of the hang before recompile).
- [18:46] Vim only, `"main.c" 55L, 888B written`, `45,3`.
- [18:53] Test changed to `hstrcmp("hello world", ...` (cursor on `d` of "world"). Status `"main.c" 55L, 894B written`. Shell: `lab $ cc main.c` / `lab $ ./a.out` / `0` / `lab $ 0 is same` / `lab $ ` (he typed "0 is same" at the prompt as a spoken note; no error visible).
- [18:57] Vim: `printf("%d\n", hstrcmp("hello world", "hello"));` (`52,37`, 894B written).
- [19:01] Shell: `lab $ cc main.c` / `lab $ ./a.out` / `0` / `lab $ 0 is same` / `lab $ cc main.c` / `lab $ ./a.out` / `32` / `lab $ ./a.out` (typed again, cursor at end).
- [19:04] Vim: args swapped: `printf("%d\n", hstrcmp("hello", "hello world"));` (`52,46`, 894B written).
- [19:09] Shell: ... `32` / `lab $ cc main.c` / `lab $ ./a.out` / `-32` / `lab $ cc main.c` / `lab $ ` (recompiling). Vim now `hstrcmp("hellO", "hello")` (5th char of first arg under block cursor, appears as capital `O` ⟨?⟩ versus digit 0; status `"main.c" 55L, 888B written`).
- [19:13] Vim: `printf("%d\n", hstrcmp("hellO", "hello"));` (`52,31`).
- [19:14] Terminal window resized (title `Terminal - 66x23`).
- [19:15-19:16] Terminal window made full-screen; Firefox/shell hidden. 19:16 shows file bottom from `char *hstrncpy` to end of main (`52,31`, Bot).
- [19:19] Scrolled to top (`40,0-1`, Top): `#include <stdio.h>`, hstrlen, hmemcpy, hstrcpy, hstrncpy, hstrcat visible.
- [19:20] Visual selection of hstrlen lines 3-8 (`-- VISUAL --`, `5`, `3,2`) - recap/review.
- [19:27] Cursor `10,16` on `char *c1 = s1;` area (hmemcpy), reviewing.
- [19:29] Scrolled to hstrcmp, cursor `45,3`, 33%.
- [19:47] Back to top (`40,0-1`); webcam: author resting cheek on hand.
- [19:48] Scrolled again, `45,3`, 33%; hstrcmp partially visible (only `s1++;` visible mid-scroll render).
- [19:53] Cursor `52,29` (80%) on the `hello` string in main.
- [19:56] Scrolled to bottom (`55,1`, Bot). Last frame: macOS menu bar visible (Terminal, Shell, Edit, View, Window, Help; clock "Thu Oct 29 1:44 AM"), cursor `48,25` on `*s1` in the return statement; mouse pointer near the recording-stop icon in the menu bar (end of video).

## File states
main.c at end of segment (55 lines, `"main.c" 55L, 888B written`). Line numbers are inferred from the saved line count and the vim cursor positions (45,3 on `}` of while; 48,25 in return; 52,31 in printf; 55,1 last `}`); the editor does not show line numbers.

```
1| #include <stdio.h>
2|
3| size_t hstrlen(const char *s) {
4|   const char *p = s;
5|   while (*p++);
6|   return (size_t)(p - s) - 1;
7| }
8|
9| void *hmemcpy(void *s1, const void *s2, size_t n) {
10|   char *c1 = s1;
11|   const char *c2 = s2;
12|   while (n--) *c1++ = *c2++;
13|
14|   return s1;
15| }
16|
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
33| char *hstrcat(char *s1, const char *s2) {
34|   char *p = s1;
35|   while (*p) p++;
36|   while ((*p++ = *s2++));
37|
38|   return s1;
39| }
40|
41| int hstrcmp(const char *s1, const char *s2) {
42|   while (*s1 && *s2 && (*s1 == *s2)) {
43|     s1++;
44|     s2++;
45|   }
46|
47|   //return *s1 - *s2; // it can have negative
48|   return (unsigned char)*s1 - (unsigned char)*s2;
49| }
50|
51| int main(void) {
52|   printf("%d\n", hstrcmp("hellO", "hello"));
53|
54|   return 0;
55| }
```
Note: `O` in "hellO" may be digit 0 ⟨?⟩ (capital O matches the -32 difference).

Intermediate 17:43 (58 lines; hstrcmp still named strcmp until 17:48):
```
?? (lines 1-40 not visible at this point; same as above through hstrcat)
41| int strcmp(const char *s1, const char *s2) {
42|   while (*s1 && *s2 && (*s1 == *s2));
43|
44|   //return *s1 - *s2; // it can have negative
45|   return (unsigned char)*s1 - (unsigned char)*s2;
46| }
47|
48| int main(void) {
49|   char dest[30] = "Hello";
50|   char *src = ", World!";
51|   //char numbers[] = "12345678";
52|
53|   hstrcat(dest, src);
54|   printf("dest after strcat:  \"%s\"\n", dest); // "Hello, w ?? (truncated)
55|
56|
57|   return 0;
58| }
```
(Line numbers here approximate; page shows return/hstrcat tail above and cursor at 58,1 on last `}`; the function was at lines ~41-46 in this version if the file is 58L.)

Intermediate 18:05 (`52L, 863B`): hstrcmp's loop was `while (*s1 && *s2 && (*s1 == *s2));` (empty body, hangs) and main printed `hstrcmp("hello", "hello")`.

## Shell & outputs
- [18:05] `lab $ cc main.c` / `lab $ ./a.out` / `dest after strcat:  "Hello, World!"` / `lab $ cc` (typing).
- [18:23] `lab $ ./a.out` / `^C` / `lab $ cc main.c` / `lab $ ./a.out` / (hangs).
- [18:39] `... lab $ ./a.out` / `^C` / `lab $ `.
- [18:53] `lab $ cc main.c` / `lab $ ./a.out` / `0` / `lab $ 0 is same` / `lab $ `.
- [19:01] `... lab $ cc main.c` / `lab $ ./a.out` / `32` / `lab $ ./a.out` (typed).
- [19:09] `... lab $ ./a.out` / `-32` / `lab $ cc main.c` / `lab $ `.
Prompt format: `lab $ `. Compiler: `cc main.c`; no warnings or errors displayed. Which exact arguments produced the `-32` run is inferred (hstrcmp("hello", "hello world")).

## Observations
- Workflow: copies the prototype from Beej's reference, pastes into vim (paste mode), prefixes `h`, and fills the body.
- First hstrcmp had an empty-body `while (...);` that hung (needed ^C twice); fixed by braces and `s1++; s2++;`. Bug found by running.
- Keeps the naive `//return *s1 - *s2; // it can have negative` as a comment and uses `(unsigned char)` casts, i.e. strcmp compares bytes as unsigned char.
- Tests: equal strings -> 0 (typed "0 is same" in shell), "hello world" vs "hello" -> 32, reversed -> -32, then "hellO" vs "hello" for case difference.
- Deleted the hstrcat test code from main to make room for strcmp tests.
- Ends by maximizing Terminal and scrolling through the whole file as a recap; recording stop icon approached. hstrncmp/hmemcmp not implemented in this segment.
- No sticky notes and no spreadsheet view in this segment.
