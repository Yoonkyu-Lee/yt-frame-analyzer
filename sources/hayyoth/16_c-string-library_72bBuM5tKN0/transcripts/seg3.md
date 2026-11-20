# Segment 3 (05:24 - 08:14) - Rewriting C String Library by Hand

Editor: vim in macOS Terminal window (top, titled "Terminal"), file `main.c` (per save message). Line numbers are NOT displayed in the editor (nonumber); line numbers below are derived from the ruler (`row,col`) and are consistent with the `26L` / `28L` save messages. A second Terminal window ("lab $" prompt) is the shell. Firefox windows: Google Sheets (`docs.google.com/spreadsheets/d/1Eb...`, tab "Sheet5", with a comment popup from the author, text ending "here") and Beej's guide (`beej.us/guide/bgclr/html/split/stringref...`). Webcam "Human" ignored.

## Timeline

- [05:24] Vim, normal mode, cursor 6,3 `All`. Buffer still has the previous segment's state: `vip_strlen` with comment `// what is this code` above it, `main` with `char mess[] = "Hello";`, `mess[2] = '\0';`, `printf("%s\n", mess);`, `printf("%zu\n", normal_strlen(mess));`. Google Sheets behind with a comment popup (author, "1:29 AM Today", text partially hidden "...here", reply box "...ply or add others with @"). Beej page behind shows partial text "...d null-terminated string (not counting ... walking down the string and ... it's a little time consuming. If you ... eatedly, save it off in a variable" (strlen description).
- [05:25] Cursor 3,1 (on `size_t vip_strlen`). The comment line `// what is this code` is gone (deleted); layout: 1 `#include <stdio.h>`, 2 blank, 3 `size_t vip_strlen(...)`.
- [05:26] `-- INSERT --` at 3,1; a new empty line opened above `size_t vip_strlen` (two blank lines temporarily).
- [05:44] Normal mode, cursor 3,1. Extra blank removed. Function RENAMED: `size_t hstrlen(const char *s) {` (h prefix replaces `vip_`). Body unchanged. main still has the mess/printf test code.
- [05:54] `-- VISUAL LINE --` (ruler `4  13,8`): selecting `char mess[] = "Hello";` through `printf("%s\n", mess);` (incl. blank line); the `%zu` printf line is not highlighted at this moment.
- [05:55] After delete: message `5 fewer lines`, cursor 9,1. `main` is now: `int main(void) {` / blank / `  return 0;` / `}` (old test code and `normal_strlen` call deleted).
- [05:58] Switched to Firefox with Beej's page "26.15 strlen()" - "Returns the length of a string". Synopsis: `#include <string.h>` / `size_t strlen(const char *s);` (signature highlighted). Description: "This function returns the length of the passed null-terminated string (not counting the NUL character at the end). It does this by walking down the string and counting the bytes until the NUL character, so it's a little time consuming. If you have to get the length of the same string repeatedly, save it off in a variable somewhere." Return Value: "Returns the number of bytes in the string. No[te ...] the number of characters in a multibyte string" (partly hidden by webcam). Then "Example".
- [05:59] Navigated to table of contents (`beej.us/guide/bgclr/html/split/`): "26 <string.h> String Manipulation": 26.1 memcpy(), memccpy(), memmove(); 26.2 strcpy(), strncpy(); 26.3 strdup(), strndup(); 26.4 strcat(), strncat(); 26.5 strcmp(), strncmp(), memcmp(); 26.6 strcoll(); 26.7 strxfrm(); 26.8 strchr(), strrchr(), memchr(); 26.9 strspn(), strcspn(); 26.10 strpbrk(); 26.11 strstr(); 26.12 strtok(); 26.13 memset(), `memset_explicit; 26.14 strerror(); 26.15 strlen(). Then 27 <tgmath.h> Type-Generic Math Functions, 28 <threads.h> Multithreading Functions. Mouse hovering over 26.1.
- [06:00] Opened "26.1 memcpy(), memccpy(), memmove()" - "Copy bytes of memory from one location to another". Synopsis: "memccpy() is new in C23!" and
  `#include <string.h>` / `void *memcpy(void * restrict s1, const void * restrict s2, size_t n);` / `void *memccpy(void * restrict s1, const void * restrict s2, int c, size_t n);` (wrapped, `size_t n);` on next line) / `void *memmove(void *s1, const void *s2, size_t n);`. Description starts "These functions copy memory - as many bytes as you want! From source to destination for as many bytes as you specify!"
- [06:02] Scrolled slightly. Description: "memccpy() gives you the additional option of stopping after a certain character is encountered in the source." "The main difference between the memcpy() [and memmove() ...] former cannot safely copy overlapping memo[ry]..." "I'm not sure why you'd want to ever use memc[cpy ...] but it's possibly more performant" (partly hidden).
- [06:03] The memcpy prototype line is highlighted (selected) in the browser.
- [06:04] Terminal focused again (vim, normal, 9,1, `5 fewer lines` still shown).
- [06:06] Vim (ruler `10,0-1`): prototype pasted as new function head at line 9: `void *memcpy(void * restrict s1, const void * restrict s2, s` (runs off right edge), blank line 10 below it, blank, then `int main(void) {`.
- [06:19] `-- INSERT --` 12,2. Prototype edited (restrict removed) and body skeleton: `void *memcpy(void *s1, const void *s2, size_t n) {` / blank / blank / `}` (cyan matching-brace highlight), blank(s), `int main(void) {`.
- [06:28] `-- INSERT --` 10,10: body started: `  while (` (cursor after paren).
- [06:31] Same: `  while (` at 10,10.
- [06:52] `-- INSERT --` 12,31. While start replaced; body in progress: `  // normal version` / `  //` / `  for (size_t i = 0; i < n; i+` (typing).
- [07:06] `-- INSERT --` 11,13. Comments gone; body in progress: `  char *c1 = s1;` / `  const char` (typing) / `  for (size_t i = 0; i < n; i++)` / `    s1[i]`.
- [07:21] Normal mode 16,0-1 `Top`. Body complete: `  char *c1 = s1;` `  const char *c2 = s2;` `  for (size_t i = 0; i < n; i++)` `    c1[i] = c2[i];` blank `  return s1;` `}`.
- [07:25] Function renamed: `void *hmemcpy(void *s1, const void *s2, size_t n) {` (ruler 9,7; `h` inserted after the `*`).
- [07:26] Normal mode, view scrolled (first visible line is `size_t hstrlen`), cursor 22,1 `Bot` on final `}` of main (matching `{` highlighted cyan).
- [07:27] Switched to Firefox; Beej 26.1 page.
- [07:28] Scrolled in Beej page: Return Value "Both functions return whatever you passed in for parameter s1 for your convenience." (above it, partial: "remember this order because it behaves like an `=` assignment: the destination is on the left."). Example (page code block with line numbers): 1 `#include <string.h>` / 3 `int main(void)` / 4 `{` / 5 `    char s[100] = "Goats";` / 6 `    char t[100];` / 8 `    memcpy(t, s, 6);       // Copy non-overlapping memory` / 10 `    memmove(s + 2, s, 6);  // Copy overlapping memory` / 11 `}`. See Also: strcpy(), strncpy(); next section "26.2 strcpy(), strncpy() - Copy a string".
- [07:29] Same example, cursor near `"Goats"`.
- [07:33] Browser line 5 `char s[100] = "Goats";` highlighted (copied). Vim (normal, 20,0-1 `Bot`) cursor on empty line 20 inside `main`: `int main(void) {` / blank / `  return 0;` / `}`.
- [07:36] `-- INSERT (paste) --` 21,3 `50%`: pasted `  char s[100] = "Goats";` into main.
- [07:39] Browser line 8 `memcpy(t, s, 6);` highlighted. Vim `-- INSERT (paste) --` 21,19: main has `  char s[100] = "Goats";` and `  memcpy(t, s, 6);` (pasted), then `  return 0;`.
- [07:40] Vim `-- INSERT --` 23,3: new blank line below memcpy call.
- [07:42] Normal mode, 22,0-1 `Bot`. main: `int main(void) {` / `  char s[100] = "Goats";` / `  memcpy(t, s, 6);` / blank / (cursor line) / `  return 0;` / `}`.
- [07:52] Window switch: lower Terminal (shell) raised, `lab $ ` empty; vim status line behind shows `"main.c" 26L, 3` (cut off). Vim main shows `char s[100] =`, `memcpy(t, s,`, `printf("%s\n"`, `return 0;`. File was saved just before.
- [07:54] Shell: `cc main.c`; 3 errors (verbatim in Shell section): implicit declaration of `memcpy` (no `<string.h>`), and use of undeclared identifier `t` (twice).
- [07:56] Vim raised, normal 23,20 `80%`. Save message: `"main.c" 26L, 382B written`. main: `char s[100] = "Goats";` / `memcpy(t, s, 6);` / blank / `printf("%s\n", t);` / blank / `return 0;`. Cursor on the `;` of printf (highlighted). Mouse near start of memcpy line.
- [07:59] Shell window again; vim behind shows `hmemcpy(t, s,` -- call renamed to `hmemcpy` (cursor on `h`). Shell prompt empty `lab $ `.
- [08:00] Shell: `cc main.c` again; now 2 errors, both `use of undeclared identifier 't'` (memcpy error gone).
- [08:05] Vim: `"main.c" 26L, 383B written`, cursor 21,3 `80%` on the `h` of `hmemcpy(t, s, 6);`.
- [08:06] Vim, pending `k` in showcmd, 26,1 `Bot` (cursor on last `}`); view scrolled so `return (size_t)(p - s) - 1;` is the first visible line; main: `int main(void) {` (cyan brace) / `  char s[100] = "Goats";` / `  hmemcpy(t, s, 6);` / blank / `  printf("%s\n", t);` / blank / `  return 0;` / `}`.
- [08:08] Beej page raised (example shown), vim cursor 21,3.
- [08:11] Beej line 6 `char t[100];` highlighted. Vim normal 21,3 `Bot`.
- [08:12] `-- INSERT (paste) --` 21,15 `83%`: pasted `  char t[100];` right after `char s[100] = "Goats";`.
- [08:13] Command-line mode: bottom shows `:` with block cursor (about to write). Buffer: `char s[100] = "Goats";` / `char t[100];` / blank / `hmemcpy(t, s, 6);` / blank / `printf("%s\n", t);` (blank line inserted between `char t[100];` and call).
- [08:14] After `:w`: status `"main.c" 28L, 3...` (truncated by overlapping shell window). Lower shell window raised, still showing the OLD compile output (2 errors; not yet re-run). Vim cursor on blank line near line 22.

## File states

### main.c at 06:19 (intermediate: memcpy skeleton, before body)
```
 1| #include <stdio.h>
 2| 
 3| size_t hstrlen(const char *s) {
 4|   const char *p = s;
 5|   while (*p++);
 6|   return (size_t)(p - s) - 1;
 7| }
 8| 
 9| void *memcpy(void *s1, const void *s2, size_t n) {
10| 
11| 
12| }
13| 
14| 
15| int main(void) {
16| 
17|   return 0;
18| }
```
(Line 12 `}` confirmed by ruler `12,2`. Blank-line count between `}` and `int main` ?? roughly two; main's later position (line 19 at end, with body 10-16) is consistent with 17-18 blank.)

### main.c at END of segment (08:13/08:14, 28L)
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
12|   for (size_t i = 0; i < n; i++)
13|     c1[i] = c2[i];
14| 
15|   return s1;
16| }
17| 
18| 
19| int main(void) {
20|   char s[100] = "Goats";
21|   char t[100];
22| 
23|   hmemcpy(t, s, 6);
24| 
25|   printf("%s\n", t);
26| 
27|   return 0;
28| }
```
Indentation: 2 spaces; for-body 4 spaces. Only `#include <stdio.h>` (no `<string.h>`). Line numbers derived from ruler and the 28L count.

### main.c at 05:24 (start of segment)
```
 1| #include <stdio.h>
 2| 
 3| // what is this code
 4| size_t vip_strlen(const char *s) {
 5|   const char *p = s;
 6|   while (*p++);
 7|   return (size_t)(p - s) - 1;
 8| }
 9| 
10| int main(void) {
11|   char mess[] = "Hello";
12|   mess[2] = '\0';
13| 
14|   printf("%s\n", mess);
15|   printf("%zu\n", normal_strlen(mess));
16| 
17|   return 0;
18| }
```

## Shell & outputs

[07:54] first compile (prompt `lab $ `):
```
lab $ cc main.c
main.c:21:3: error: call to undeclared
      library function 'memcpy' with type 'void *(void *,
      const void *, unsigned long)'; ISO C99 and later do
      not support implicit function declarations
      [-Wimplicit-function-declaration]
   21 |   memcpy(t, s, 6);
      |   ^
main.c:21:3: note: include the header
      <string.h> or explicitly provide a declaration for
      'memcpy'
main.c:21:10: error: use of undeclared
      identifier 't'
   21 |   memcpy(t, s, 6);
      |          ^
main.c:23:18: error: use of undeclared
      identifier 't'
   23 |   printf("%s\n", t);
      |                  ^
3 errors generated.
lab $ 
```
[08:00] after renaming call to `hmemcpy`:
```
lab $ cc main.c
main.c:21:11: error: use of undeclared
      identifier 't'
   21 |   hmemcpy(t, s, 6);
      |           ^
main.c:23:18: error: use of undeclared
      identifier 't'
   23 |   printf("%s\n", t);
      |                  ^
2 errors generated.
lab $ 
```
Vim messages: `5 fewer lines` (05:55), `"main.c" 26L, 382B written` (07:56), `"main.c" 26L, 383B written` (08:05), `"main.c" 28L, 3...` (08:14, truncated).

Web pages (Beej's Guide to C library reference, chapter 26 <string.h>): described in the timeline at 05:58 (26.15 strlen), 05:59 (TOC), 06:00-06:03 (26.1 synopsis/description), 07:28 (26.1 example).

## Observations

- Workflow: delete previous test scaffolding, rename the function with `h` prefix (hstrlen, hmemcpy), look up the real function's signature in Beej's guide, paste the prototype into vim, strip `restrict`, write the body by hand. Comments like `// normal version` were typed then removed.
- hmemcpy is the plain byte-copy: `char *c1 = s1; const char *c2 = s2; for (size_t i = 0; i < n; i++) c1[i] = c2[i]; return s1;`, mirroring the docs' "return whatever you passed in for parameter s1". (`c1` is `char *`, not `unsigned char *`.)
- Test code copied from Beej's example (`char s[100] = "Goats"; char t[100]; memcpy(t, s, 6);`) and adapted. He first compiled with `memcpy` (no `<string.h>` included, producing the implicit-declaration error), then renamed to `hmemcpy`; the `t` errors were then fixed by pasting `char t[100];`.
- Only `<stdio.h>` is included; his own functions replace string.h.
- Edit style: heavy paste from browser (`INSERT (paste)`), visual-line delete, frequent `:w`, switching between vim window and a separate shell window.
- The Google Sheets tab with a comment sits unused behind in this segment.
