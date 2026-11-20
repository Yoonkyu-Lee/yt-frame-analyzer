# Segment 6 (13:04 - 14:43), frames only

## Timeline
- [13:04] vim in a macOS Terminal window (left, title "Terminal") showing hstrncpy and main; status line `"main.c" 44L, 615B written    25,7    Bot`. Firefox (right) at `beej.us/guide/bgclr/html/split/` shows the contents list "26 <string.h> String Manipulation": 26.1 memcpy(), memccpy(), memmove(); 26.2 strcpy(), strncpy(); 26.3 strdup(), strndup(); 26.4 strcat(), strncat(); 26.5 strcmp(), strncmp(), memcmp(); 26.6 strcoll(); 26.7 strxfrm(); 26.8 strchr(), strrchr(), memchr(); 26.9 strspn(), strcspn(); 26.10 strpbrk(); 26.11 strstr(); 26.12 strtok(); 26.13 memset(), memset_explicit; 26.14 strerror(); 26.15 strlen(); then 27 <tgmath.h>, 28 <threads.h>. A Google Sheets tab (docs.google.com/spreadsheets/d/...) is behind, plus a blank white Terminal window behind vim. Webcam overlay ignored.
- [13:05] Mouse hovers 26.3 strdup(); status-bar link `beej.us/guide/bgclr/html/split/stringref.html#man-strdup`.
- [13:07] Page loading: URL `beej.us/guide/bgclr/html/split/stringref...`, status "Transferring data from beej.us...". Shows "26 <string.h> String Manipulation" function/description table (memccpy, memchr, memcmp, memcpy, memmove, memset, memset_explicit, strcat, strchr...).
- [13:08] Back at contents list; mouse on 26.4; link `...stringref.html#man-strcat`.
- [13:09] Page "26.4 strcat(), strncat()": "Concatenate two strings into a single string". Synopsis:
  `#include <string.h>` / `int strcat(const char *dest, const char *src);` / `int strncat(const char *dest, const char *src, size_t n);`
  Description: "Concatenate", for those not in the know, means to "stick together". These functions take two strings, and stick them together, storing the result in the first string. / These functions don't take the size of the first string into account when it does the concatenation. What this means in practical terms is that you can try to stick a 2 megabyte string into a 10 byte space. This will lead to unintended consequences, unless you intended to lead to unintended consequences, in which case it will lead to intended unintended consequences. / Technical banter aside, your boss and/or professor will be irate. / If you want to make sure you don't overrun the first string, be sure to check the lengths of the strings first and use some highly technical subtraction to make sure things fit. / You can actually only concatenate the first n characters of the second string by using strncat() and specifying the maximum number of characters to copy. Return Value: "Both functions return a pointer to the destination string, like most of the string-oriented functions." Example begins `1 #include <stdio.h>`.
- [13:10]-[13:13] Slight scrolling; cursor over `*src`.
- [13:17] Dock visible, tooltip "Stickies" on the Stickies icon.
- [13:19] Blank yellow Stickies note appears top-left.
- [13:21]-[13:22] Note dragged to the middle over the Firefox page.
- [13:23] Typing in note: `sp|`.
- [13:31] Note text: `spotted , beej guide wrong here`. A second Terminal window (shell, prompt `lab $`) is in front of vim; contents: `lab $ cc main.c`, `lab $ ./a.out`, `Go`, `lab $ m|` (typing "man").
- [13:34] That window now shows the STRCAT(3) man page (see Shell).
- [13:35] Man-page window dragged upward (title bar y~199).
- [13:38] In the man page the author highlights `char *` / `strcat(char *restrict s1, const char *restrict s2);`.
- [13:39] Firefox in front; in Beej's synopsis `int strcat(const char *dest` is highlighted. Man window behind.
- [13:43] Stickies dialog: "If you don't save this note, its contents will be lost. Are you sure you want to discard this Stickies note?" Buttons "Save...", "Delete Note", "Cancel" (fading in; mouse near Delete Note).
- [13:44] Dialog and sticky note gone; Firefox in front.
- [13:45] Man page in front again, same selection highlighted.
- [13:46] Man window moved (title bar y~307).
- [13:48] Selection of the strcat prototype lines; mouse at start of `char *`.
- [13:49] Man page quit: shell shows `lab $ cc main.c` / `lab $ ./a.out` / `Go` / `lab $ man 3 strcat` / `lab $ `.
- [13:50] vim window in front, normal mode, cursor 25,7 on `*hstrncpy`. Status `"main.c" 44L, 615B written    25,7    Bot`.
- [13:52] vim `-- INSERT --  33,1  95%`: a blank line opened after hstrncpy's closing `}` (cursor on empty line 33; a trailing blank line still precedes main).
- [13:53] Pasted man synopsis visible (normal mode, 34,53, 92%):
  `char *` / `    strcat(char *restrict s1, const char *restrict s2` (line shown ends at `s2`; `s` of s2 highlighted as cursor; `);` not visible).
- [13:57] Joined into one line: `char *strcat(char *restrict s1, const char *restrict s2` (INSERT 33,7 95%, cursor right after `*`).
- [14:09] Now `char *hstrcat(char *s1, const char *s2) {` with an empty indented line below it (INSERT 35,3 88%); closing `}` not yet visible below the blank line at that frame.
- [14:12] Normal mode (35,1, 88%): body is `{` / blank / `}` with cyan matching-brace highlight on `{` and `}`. Firefox in front with `int strcat(const char *dest` still selected.
- [14:14] vim in front, mouse I-beam over `*s1`.
- [14:18] Firefox in front; vim INSERT 34,8 88% with body line `  char []` (cursor block after `char `).
- [14:19]-[14:21] Beej page scrolled to Description / Return Value text (quoted above).
- [14:23]-[14:24] vim in front: `  char |` (INSERT 34,8 88%). Firefox scrolled to top of 26.4.
- [14:25]-[14:26] Mouse over synopsis; `int strcat(const char *dest` highlighted; vim still `  char `.
- [14:30] vim: `  char *p = s1;` then blank line (INSERT 35,3 85%).
- [14:31] vim INSERT 37,7 79%: `  char *p = s1;`, two blank lines, then `  retu|` being typed.
- [14:43] vim normal mode 35,17 79%: hstrcat body is `  char *p = s1;` / `  while (*p) p++;` (final `;` highlighted as cursor) / blank / `  return s1;` / `}`. The copy loop from s2 not yet written. No new save message after `"main.c" 44L, 615B written`.

## File states
main.c at 14:43. Line numbers inferred from vim cursor positions (hstrncpy at line 25; cursor 35,17 on the `while` line). Lines 1-24 not visible.

```
?? (lines 1-24 not visible)
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
36|
37|   return s1;
38| }
39|
40| int main(void) {
41|   char s[100] = "Goats";
42|   char t[100];
43|
44|   hstrncpy(t, s, 2);
45|   t[2] = '\0';
46|
47|   printf("%s\n", t);
48|
49|   return 0;
50| }
?? (further lines not visible)
```
Note: the original file was 44L before this work (hstrncpy signature at line 25 per `25,7`); the numbering above shifts main by the new function, so main's numbers are approximate and a blank-line count between `}` and `char *hstrcat` / `int main` is uncertain (one blank line each visible at 14:43; two blank lines originally before main).

Intermediate versions:
13:53 (lines 33-34 only)
```
33| char *
34|     strcat(char *restrict s1, const char *restrict s2
```
13:57
```
33| char *strcat(char *restrict s1, const char *restrict s2
```
14:09
```
33| char *hstrcat(char *s1, const char *s2) {
34|
35|   |
36| ?? (closing brace not visible)
```

## Shell & outputs
- [13:31] Second Terminal window, prompt `lab $`:
```
lab $ cc main.c
lab $ ./a.out
Go
lab $ m
```
(`Go` is the program output: hstrncpy(t, s, 2) on "Goats" then t[2]='\0'.) No compiler warnings shown.
- [13:34] `man 3 strcat` (less prompt `:` at bottom):
```
STRCAT(3)        Library Functions Manual        STRCAT(3)

NAME
     strcat, strncat — concatenate strings

LIBRARY
     Standard C Library (libc, -lc)

SYNOPSIS
     #include <string.h>

     char *
     strcat(char *restrict s1, const char *restrict s2);

     char *
     strncat(char *restrict s1, const char *restrict s2,
         size_t n);

ADMONISHMENT
             YOU SHOULD ALMOST CERTAINLY USE strlcat()
:
```
- [13:49] After quitting man:
```
lab $ cc main.c
lab $ ./a.out
Go
lab $ man 3 strcat
lab $ 
```
- Beej web pages: contents list of chapter 26 (13:04) and the 26.4 strcat()/strncat() page (13:09 onward), described in Timeline.

## Observations
- Workflow: read Beej's reference for strcat/strncat, then cross-check with the macOS man page `man 3 strcat`.
- Sticky note "spotted , beej guide wrong here": Beej's synopsis shows `int strcat(const char *dest, ...)` (return type int, const dest) whereas the man page has `char *strcat(char *restrict s1, const char *restrict s2)`. He then discarded the note via the "Delete Note" dialog.
- He copies the man-page prototype into vim and edits it into his style: `char *hstrcat(char *s1, const char *s2) {` (drops `restrict`, keeps s1/s2 names, "h" prefix).
- Approach for hstrcat: pointer p to s1, advance to the NUL with `while (*p) p++;`, return s1 (original destination). Copy step still pending at end of segment.
- Style: two-space indent, terse K&R-like loops; main() harness unchanged from hstrncpy test.
