# Segment 4 (08:14 - 10:20), video 72bBuM5tKN0

Screen layout throughout: vim in a macOS Terminal window (top-left, titled "Terminal"), a second Terminal window (shell, prompt `lab $ `) that overlaps it when focused, a Firefox window on beej.us (Beej's Guide to C library reference, stringref page), and a Firefox window behind it on a Google Sheets URL (docs.google.com/spreadsheets/d/1Eb..., contents never visible). Webcam overlay "Human" ignored.

## Timeline

[08:14] Shell window focused over vim. Output of `cc main.c`:
```
lab $ cc main.c
main.c:21:11: error: use of undeclared identifier 't'
   21 |   hmemcpy(t, s, 6);
      |           ^
main.c:23:18: error: use of undeclared identifier 't'
   23 |   printf("%s\n", t);
      |                  ^
2 errors generated.
lab $
```
Vim behind shows (visible portion) hmemcpy with for-loop, `int main(void) {`, `char s[100] = ...`, `char t[100];`, `hmemcpy(t, s, ...`, `printf("%s\n"...`, status `"main.c" 28L, 3...`. (The error says `t` undeclared although `char t[100];` is visible in the vim window; the vim buffer was apparently saved after the compile, i.e. the file compiled was an earlier state. Not otherwise explained on screen.) Browser: Beej memcpy() Example (lines 1-11): `char s[100] = "Goats";` `char t[100];` (line 6 highlighted), `memcpy(t, s, 6);       // Copy non-overlapping memory`, `memmove(s + 2, s, 6);  // Copy overlapping memory`. Heading ", strncpy()" (26.2) partially visible below.
[08:15] Shell: `lab $ cc main.c` then `lab $ ` (clean compile, no output).
[08:18] Vim focused. Status line: `"main.c" 28L, 399B written     22,0-1   71%`. Visible:
```
  return (size_t)(p - s) - 1;
}

void *hmemcpy(void *s1, const void *s2, size_t n) {
  char *c1 = s1;
  const char *c2 = s2;
  for (size_t i = 0; i < n; i++)
    c1[i] = c2[i];

  return s1;
}


int main(void) {
  char s[100] = "Goats";
  char t[100];

  hmemcpy(t, s, 6);

  printf("%s\n", t);
```
[08:23] Shell: `cc main.c`, `./a.out` printed `Goats`.
[08:27] Vim: the 6 changed to 2 (cursor on it, highlighted): `hmemcpy(t, s, 2);`, status still shows the old `"main.c" 28L, 399B written` text, cursor `23,17`, 71%. (Not yet saved.)
[08:29] Vim scrolled to the bottom: after `hmemcpy(t, s, 2);` blank, `printf("%s\n", t);`, blank, `return 0;`, `}`. Cursor `24,0-1`, `Bot`.
[08:33] INSERT mode (`-- INSERT --  24,17  87%`): author typed a comment line under the call: `// it not copy` (verbatim, typo-ish).
[08:44] Shell shows second build/run:
```
lab $ cc main.c
lab $ ./a.out
Goats
lab $ cc main.c
lab $ ./a.out
Go?o
lab $
```
The `?` is a rendered garbage/non-printable byte. Vim behind: the comment line has been replaced by `t[2] = '\0';` (cursor on its `;`), status `"main.c" 29L, 4...`. (The `Go?o` output was from n=2 without a terminator; only the two copied chars `Go` then uninitialized bytes.)
[08:49] Vim: `"main.c" 29L, 414B written     24,14   87%`:
```
  hmemcpy(t, s, 2);
  t[2] = '\0';

  printf("%s\n", t);

  return 0;
```
[08:51] Author jumped toward the top (`3,14   25%`, cursor within `hstrlen`):
```
size_t hstrlen(const char *s) {
  const char *p = s;
  while (*p++);
  return (size_t)(p - s) - 1;
}

void *hmemcpy(void *s1, const void *s2, size_t n) {
  ...for-loop version...
```
[08:52] Cursor `8,0-1  Top`; whole top visible: `#include <stdio.h>` (line 1), blank, hstrlen (lines 3-7), blank (8), hmemcpy from line 9.
[08:55] `-- VISUAL LINE --  1  12,14  Top`: the line `for (size_t i = 0; i < n; i++)` selected (about to delete the loop).
[08:56] Both loop lines (`for ...` and `c1[i] = c2[i];`) deleted; cursor `12,0-1` on an empty line between `const char *c2 = s2;` and `return s1;`.
[08:57] INSERT `12,3`: typing a new body at indent 2 on that line.
[09:17] hmemcpy rewritten (cursor `25,20`, `57%`, normal mode, on the `;` of printf):
```
void *hmemcpy(void *s1, const void *s2, size_t n) {
  char *c1 = s1;
  const char *c2 = s2;
  while (n--) *c1++ = *c2++;

  return s1;
}
```
main below: `hmemcpy(t, s, 2);` / `t[2] = '\0';` / `printf("%s\n", t);` shown consecutively with the blank line between `t[2]` and `printf` missing at this moment (the line numbering shows 25 as printf).
[09:18] Terminal shows third run:
```
lab $ cc main.c
lab $ ./a.out
Goats
lab $ cc main.c
lab $ ./a.out
Go?o
lab $ cc main.c
lab $ ./a.out
Go
lab $
```
(The `Go` is highlighted/selected.) Vim behind shows `}` at bottom highlighted and `int main(void) {` brace highlighted (matching brace).
[09:19] Shell: `lab $ cc main.c` typed, cursor at end, Enter not yet pressed (screen was cleared).
[09:24] Shell: `lab $ cc main.c`, `lab $ ./a.out`, `Go`, `lab $ `. Firefox (beej) in front: bottom of the memcpy section: `See Also` `strcpy(), strncpy()`, then heading `26.2 strcpy(), strncpy()`, `Copy a string`.
[09:25] Page scrolled: `26.2 strcpy(), strncpy()`, `Copy a string`, `Synopsis`, `#include <string.h>`, `char *strcpy(char *dest, char *src);` visible.
[09:26] Author went to the TOC (URL beej.us/guide/bgclr/html/split/). Chapter `26 <string.h> String Manipulation`:
26.1 memcpy(), memccpy(), memmove(); 26.2 strcpy(), strncpy(); 26.3 strdup(), strndup(); 26.4 strcat(), strncat(); 26.5 strcmp(), strncmp(), memcmp(); 26.6 strcoll(); 26.7 strxfrm(); 26.8 strchr(), strrchr(), memchr(); 26.9 strspn(), strcspn(); 26.10 strpbrk(); 26.11 strstr(); 26.12 strtok(); 26.13 memset(), `memset_explicit (as rendered, stray backtick); 26.14 strerror(); 26.15 strlen().
Then `27 <tgmath.h> Type-Generic Math Functions` (27.1 Example), `28 <threads.h> Multithreading Functions` (28.1 call_once(), 28.2 cnd_broadcast(), 28.3 cnd_destroy(), 28.4 cnd_init(), 28.5 cnd_signal(), 28.6 cnd_timedwait(), 28.7 cnd_wait(), 28.8 mtx_destroy()...).
[09:27] Mouse pointer (hand) over link 26.1 memcpy(); status bar `beej.us/guide/bgclr/html/split/stringref.html#man-memcpy`.
[09:30] Page 26.2:
```
#include <string.h>

char *strcpy(char *dest, char *src);

char *strncpy(char *dest, char *src, size_t n);
```
Description text: "These functions copy a string from one address to another, stopping at the NUL terminator on the src string." "strncpy() is just like strcpy(), except only the first n characters are actually copied. Beware that if you hit the limit, n before you get a NUL terminator on the src string, your dest string won't be NUL-terminated. Beware! BEWARE!" "(If the src string has fewer than n characters, it works just like strcpy().)" "You can terminate the string yourself by sticking the '\0' in there yourself:" then code starting `char s[10];` `char foo = "My hovercraft is full of e...`.
[09:31] Slight scroll; mouse I-beam at end of `char *strcpy(char *dest, char *src);`.
[09:34] Vim focused: cursor `15,1  Bot` on `}` ending hmemcpy. In browser the `char *strcpy(char *dest, char *src);` line is highlighted (selected). main:
```
int main(void) {
  char s[100] = "Goats";
  char t[100];

  hmemcpy(t, s, 2);
  t[2] = '\0';

  printf("%s\n", t);

  return 0;
}
```
[09:35] `-- INSERT (paste) --  17,36  77%`: new line `char *strcpy(char *dest, char *src)` (paren pair highlighted) pasted below hmemcpy (the trailing `;` not present).
[09:36] `char *strcpy(char *dest, char *src) {` with cursor on an indented empty line, INSERT `18,3` 70%.
[09:37] Closing brace auto-present: lines `char *strcpy(...) {` / (cursor line) / `}`; normal mode `18,0-1  63%`.
[09:44] INSERT `19,3  58%`: `char *p = dest;` typed, new empty line below.
[09:45] INSERT `20,4  53%`: after a blank line, `r` typed.
[09:48] Browser focused. Vim shows:
```
char *strcpy(char *dest, char *src) {
  char *p = dest;

  return dest;
}
```
(status `20,14  53%`).
[09:49] Beej page scrolled down: strncpy example `char s[10];` / `char foo = "My hovercraft is full of eels."; // more than 10 chars` / `strncpy(s, foo, 9); // only copy 9 chars into positions 0-8` / `s[9] = '\0';        // position 9 gets the terminator`. `Return Value`: "Both functions return dest for your convenience, at no extra charge." `Example`: `#include <string.h>`, `int main(void) {`, `char *src = "hockey hockey hockey hockey hockey hockey hocke...`, `char dest[20];`, `int len;`, `strcpy(dest, "I like "); // ...` (clipped).
[09:52] Scrolled back up to Synopsis/Description.
[09:53] Header `26.2 strcpy(), strncpy()` at top, Synopsis; mouse near `char *src` in strcpy.
[09:55] Further small scroll up (Synopsis at top).
[09:56] Vim focused, cursor `18,17  53%` on `;` of `char *p = dest;` (highlighted); `return dest;` with `}` below.
[09:57] INSERT `20,6  46%`: typing `whi` on the blank line between `char *p = dest;` and `return dest;`.
[10:13] strcpy complete; normal mode `31,14  66%` cursor on `;` of `t[2] = '\0';`:
```
  const char *c2 = s2;
  while (n--) *c1++ = *c2++;

  return s1;
}

char *strcpy(char *dest, char *src) {
  char *p = dest;

  while (*p++ = *src++);

  return dest;
}


int main(void) {
  char s[100] = "Goats";
  char t[100];

  hmemcpy(t, s, 2);
  t[2] = '\0';
```
[10:15] `-- VISUAL LINE --  2  28,15  66%`: lines `char s[100] = "Goats";` and `char t[100];` selected.
[10:16] Same selection; browser Description area (scroll).
[10:17] Browser scrolled to the strncpy example with mouse pointer; selection still there.
[10:20] Vim focused, NORMAL `28,14  66%`, selection gone, cursor on `;` after `t[100]`. No save message observed after the strcpy edits.

## File states

main.c at end of segment (10:20). Line numbers: 1-9 seen directly at 08:52; later numbers derived from vim cursor positions (hmemcpy `}` = line 15 at 09:34; cursor 18,17 on `char *p = dest;` at 09:56; 20,14 on return line before while added; 28,14 on `char t[100];`; 31,14 on `t[2]` line). Lines after 31 not visible at 10:20; shown from earlier frames.
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
17| char *strcpy(char *dest, char *src) {
18|   char *p = dest;
19|
20|   while (*p++ = *src++);
21|
22|   return dest;
23| }
24|
25|
26| int main(void) {
27|   char s[100] = "Goats";
28|   char t[100];
29|
30|   hmemcpy(t, s, 2);
31|   t[2] = '\0';
?? (lines 32-36 not visible at 10:20; earlier frames show: blank, `  printf("%s\n", t);`, blank, `  return 0;`, `}`)
```
Intermediate version (08:49-08:52, saved as `"main.c" 29L, 414B written`):
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
23|   hmemcpy(t, s, 2);
24|   t[2] = '\0';
25|
26|   printf("%s\n", t);
27|
28|   return 0;
29| }
```

## Shell & outputs

- [08:14] `cc main.c` -> `main.c:21:11: error: use of undeclared identifier 't'` (line 21 `hmemcpy(t, s, 6);`), `main.c:23:18: error: use of undeclared identifier 't'` (line 23 `printf("%s\n", t);`), `2 errors generated.`
- [08:15] `cc main.c` -> no output.
- [08:23] `./a.out` -> `Goats` (n = 6).
- [08:44] `cc main.c`; `./a.out` -> `Go?o` (n = 2, t not terminated).
- [09:18] `cc main.c`; `./a.out` -> `Go` (hmemcpy 2 bytes plus `t[2] = '\0'`, while-loop hmemcpy).
- [09:24] after clear: `cc main.c`; `./a.out` -> `Go`.
- Vim saves: `"main.c" 28L, 399B written` (08:18), `"main.c" 29L, 414B written` (08:49).
- Browser: Beej's Guide to C library reference, stringref page and chapter TOC (see timeline for visible text). Google Sheets window not shown.

## Observations

- Author uses a test-in-main loop: edit, `:w`, `cc main.c`, `./a.out`; compares to Beej's reference example for memcpy (Goats / 6 bytes) and then deliberately copies only 2 bytes to see missing-terminator behavior (`Go?o`), then adds `t[2] = '\0';` (mirrors the strncpy "Beware! BEWARE!" paragraph he reads right after).
- Typed a short comment `// it not copy` at 08:33 before replacing it with the terminator fix.
- Style: 2-space indent, `size_t`, pointer-walking idioms. hmemcpy refactored from an indexed `for` loop into `while (n--) *c1++ = *c2++;`. strcpy written as `while (*p++ = *src++);` with `char *p = dest;` and `return dest;` (still unprefixed name `strcpy` and Beej's `dest/src` parameter names, signature pasted directly from the Beej synopsis, not `h`-prefixed yet at 10:20).
- hstrlen: `while (*p++);` then `(size_t)(p - s) - 1`.
- References consulted: Beej 26.1 (memcpy/memmove example), 26.2 (strcpy/strncpy synopsis, description, return value, example), chapter 26 TOC (shows what string.h functions remain: strdup, strcat, strcmp, strchr, strspn, strpbrk, strstr, strtok, memset, strerror, strlen).
- No sticky notes seen in this segment.
