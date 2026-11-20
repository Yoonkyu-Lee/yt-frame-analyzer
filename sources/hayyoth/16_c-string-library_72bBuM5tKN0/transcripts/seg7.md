# Segment 7 of 8 (14:43 - 16:38) - frames only, webcam "Human" ignored

## Timeline
- [14:43] Layout: vim in macOS Terminal (left, title "Terminal"; status `35,17  79%`, cursor on the `;` of `while (*p) p++;` in hstrcat). Firefox (right) on `beej.us/guide/bgclr/html/split/stringref.…` showing "26.4 strcat(), strncat()" (synopsis as rendered: `int strcat(const char *dest, const char *src);` / `int strncat(const char *dest, const char *src, size_t n);`; Description: "Concatenate", for those not in the know, means to "stick together"... storing the result in the first string; "These functions don't take the size of the first string into account..."; "Technical banter aside, your boss and/or professor will be irate."). Behind: a Google Sheets window (`docs.google.com/spreadsheets/d/1Eb…`). A third Terminal window is behind.
- [14:44] Desktop-overview view (Desktop 1/2/3 bar, Dock visible). Sheets tab `Sheet5` open with a comment popup near cell E6: author <account>, "1:29 AM Today", text `go to here`, plus a "Reply or add others with @" box. Sheet content: merged header `"hello"`, row 4 indices `0 1 2 3 4 5`, row 5 chars `h e \0 l o \0` (third cell `\0`, fourth `l`). Third Terminal (lower left) shows: `lab $ cc main.c` / `lab $ ./a.out` / `Go` / `lab $ man 3 strcat` / `lab $ `
- [14:45] Sheets window in front (toolbar visible, Sheet5). Comment-corner markers at D6/E6. Row 5 still `h e \0 l o \0`. Cell J4 selected.
- [14:54] Terminal raised over Sheets; vim normal mode `35,17  79%`, cursor on `;` of `while (*p) p++;`. Sheets scrolled/column M selected.
- [14:57] Sheets in front. Row 5 now `h e l l o \0` (index-2 `\0` replaced with `l`). Comment markers remain at D6/E6. Vim behind shows `-- VISUAL --` with `while (*p) p++;` highlighted.
- [14:59] New cell comment being typed on G6 (author <account>), text so far `go `. Tab badge `3` on Sheet5.
- [15:00] Hovering the author name opens a profile card (private details not recorded).
- [15:05] Comment text now `go to` (selection moved to B9; G6 highlighted yellow).
- [15:08] Comment posted: <account> "1:39 AM Today" `go to here` on G6 (the cell under the index-5 `\0`).
- [15:10] Terminal in front. `-- VISUAL --  15  35,3  79%`: `while (*p) p++;` selected.
- [15:12] `-- INSERT --  36,3  76%`: a new blank indented line opened after `while (*p) p++;` in hstrcat.
- [15:27] Typing hstrcat's second loop; buffer scrolled (`19,0-1  60%`). Visible: `while ((*p++ = *src++));` (hstrcpy body) above, and in hstrcat `while ((*p++ = *` mid-typing (he left insert mode and moved up to line 19).
- [15:28] `18,17  56%`, cursor on `;` after `char *p = dest` in hstrcpy; half-typed `while ((*p++ = *` still in hstrcat.
- [15:29] `-- VISUAL --  24  20,3  56%`: `while ((*p++ = *src++));` in hstrcpy selected (to yank).
- [15:31] Message line `Type  :qa!  an...s and exit Vim   25,30   56%` (Ctrl-C hint; vim not exited).
- [15:32] `36,18  60%`, cursor on the `*` of the half-typed `while ((*p++ = *` in hstrcat.
- [15:40] hstrcat now `while ((*p++ = *s2++));` (`45,20  80%`, cursor on `;` of `hstrncpy(t, s, 2);` in main).
- [15:41] Scrolled to bottom of main (`49,0-1  93%`): `hstrncpy(t, s, 2);` / `t[2] = '\0';` / blank / `printf("%s\n", t);` / blank line 49.
- [15:42] Firefox raised on the strcat/strncat page (top, same text as 14:43); Terminal inactive behind.
- [15:43] Beej page scrolled: "You can actually only concatenate the first n characters of the second string by using strncat() and specifying the maximum number of characters to copy." "Return Value: Both functions return a pointer to the destination string, like most of the string-oriented functions." "Example" listing with line numbers.
- [15:44] Example listing fully visible (lines 1-17, see Shell & outputs). Text below: "Notice I mixed and matched pointer and array notation there with src and numbers; this is just fine with string functions." See Also: `strlen()`.
- [15:46] Author drag-selects example lines 6-13 (`char dest[30]...` through `printf("dest after strcat: ...`).
- [15:48] Terminal active (`46,14  93%`): cursor on `;` of `t[2] = '\0';`. Beej selection stays highlighted.
- [15:50] `-- INSERT --  50,1  74%`: Beej lines 6-13 pasted into main with the browser's indentation (first line `      char dest[30] = "Hello";` more indented than the rest):
  `char dest[30] = "Hello";` / `char *src = ", World!";` / `char numbers[] = "12345678";` / blank / `printf("dest before strcat: \"%s\"\n", dest); // "Hello"` / blank / `strcat(dest, src);` / `printf("dest after strcat:  \"%s\"\n", dest); // "Hello, …` (two spaces after `strcat:`).
- [15:51] `51,1  76%`: the old `  char s[100] = "Goats";` line visible below the pasted block.
- [15:52] `-- VISUAL LINE --  10  60,1  Bot`: old main body (`char s[100] = "Goats";`, `char t[100];`, blank, `hstrncpy(t, s, 2);`, `t[2] = '\0';`, blank, `printf("%s\n", t);`, blank, `return 0;`, `}`) selected.
- [15:53] After delete: `7 fewer lines  51,0-1  Bot`; `return 0;` and `}` now follow the pasted block (the author deleted 7 lines of his old test, keeping return/brace), `~` filler below.
- [15:55] `-- VISUAL LINE --  5  46,1  Bot`: lines of the pasted block (dest/src/numbers/blank/printf-before) selected.
- [15:56] `-- VISUAL LINE --  8  49,1  Bot`: selection extended through `strcat(dest, src);` and the printf-after line; he re-indents (pasted lines shift left by 2).
- [15:57] Indentation fixed (2 spaces), cursor `44,1  Bot` on `char numbers[] = "12345678";`.
- [16:00] Scrolled: top shows hstrcat; main starts `int main(void) {` / `char dest[30] = "Hello";` / `char *src = ", World!";` / `char numbers[] = "12345678";` (`33,3  96%`).
- [16:06] Edits: `char numbers[] = "12345678";` commented out as `//char numbers[] = "12345678";`; the `printf("dest before strcat: …")` line deleted (cursor on a blank line `46,0-1  Bot`).
- [16:07] `-- INSERT --  46,3  Bot`: cursor at start of `strcat(dest, src);`, about to prepend `h`.
- [16:19] Saved: `"main.c" 51L, 811B written` (partly hidden by a second Terminal window, visible as `"main.c" 51L, 811B `). main now calls `hstrcat(dest, src);` (cursor on the `h`). A second Terminal window (title "Terminal") in front shows `lab $ cc main.c` / `lab $ ./a.out` / `Go` / `lab $ man 3 strcat` / `lab $ `.
- [16:25] Second Terminal after rerun: `lab $ cc main.c` / `lab $ ./a.out` / `dest after strcat:  "Hello, World!"` / `lab $ ` (the quoted `"Hello, World!"` highlighted by mouse selection).
- [16:26] Vim in front again (`"main.c" 51L, 811B written   46,3  Bot`); second terminal behind and blank-looking. Mouse over `while ((*p++ = *s2++));`.
- [16:28] `-- VISUAL --  13  36,22`: `(*p++ = *s2++)` highlighted in hstrcat. Firefox raised, still at example listing.
- [16:29] Firefox on the Beej TOC (`beej.us/guide/bgclr/html/split/`): chapter 26 `<string.h> String Manipulation` entries 26.1-26.15 (see below), then 27 `<tgmath.h>` and 28 `<threads.h>`. Mouse on 26.5 (status bar link `...stringref.html#man-strcmp`).
- [16:31] Page 26.5 `strcmp(), strncmp(), memcmp()` - "Compare two strings or memory regions and return a difference"; synopsis shown.
- [16:35] TOC again; mouse on 26.4 (link `...#man-strcat`).
- [16:38] Back on 26.5, scrolled slightly. Vim behind still `-- VISUAL --` with `(*p++ = *s2++)` highlighted. End of segment.

## File states
main.c (vim). Line numbers inferred from cursor positions (hstrcpy body at 18-22, hstrncpy starts 25, hstrcat 33, main 41; file is 51 lines at the 16:19 save). Only hstrcat/main and the tail of hstrcpy/hstrncpy are visible.

### Intermediate at ~15:00 (before editing hstrcat)
```
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
   ?? (following lines: t[2] = '\0'; blank; printf("%s\n", t); blank; return 0; } not visible then)
```

### End of segment (16:38)
```
?? (lines 1-16 not visible this segment; hstrcpy signature at ~17 not visible)
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
41| int main(void) {
42|   char dest[30] = "Hello";
43|   char *src = ", World!";
44|   //char numbers[] = "12345678";
45|
46|   hstrcat(dest, src);
47|   printf("dest after strcat:  \"%s\"\n", dest); // "Hello, w ⟨?⟩ (rest of comment cut off)
48|
49|
50|   return 0;
51| }
```
Saved with `"main.c" 51L, 811B written`.

## Shell & outputs
- [14:44] Third Terminal: `lab $ cc main.c`, `lab $ ./a.out`, output `Go`, `lab $ man 3 strcat`, prompt.
- [16:19] Second Terminal (same history): `lab $ cc main.c` / `lab $ ./a.out` / `Go` / `lab $ man 3 strcat` / `lab $ `
- [16:25] Second Terminal after rerun with hstrcat: `lab $ cc main.c` / `lab $ ./a.out` / `dest after strcat:  "Hello, World!"` / `lab $ `. No compiler errors or warnings visible.
- Vim message lines seen: `-- INSERT --`, `-- VISUAL --`, `-- VISUAL LINE --`, `7 fewer lines`, `Type  :qa!  an...s and exit Vim`, `"main.c" 51L, 811B written`.
- Firefox, Beej 26.4 example listing (as rendered):
```
 1  #include <stdio.h>
 2  #include <string.h>
 3
 4  int main(void)
 5  {
 6      char dest[30] = "Hello";
 7      char *src = ", World!";
 8      char numbers[] = "12345678";
 9
10      printf("dest before strcat: \"%s\"\n", dest); // "Hello"
11
12      strcat(dest, src);
13      printf("dest after strcat:  \"%s\"\n", dest); // "Hello, wo…
14
15      strncat(dest, numbers, 3); // strcat first 3 chars of numbe…
16      printf("dest after strncat: \"%s\"\n", dest); // "Hello, wo…
17  }
```
- Firefox, Beej TOC chapter 26 `<string.h> String Manipulation`: 26.1 memcpy(), memccpy(), memmove(); 26.2 strcpy(), strncpy(); 26.3 strdup(), strndup(); 26.4 strcat(), strncat(); 26.5 strcmp(), strncmp(), memcmp(); 26.6 strcoll(); 26.7 strxfrm(); 26.8 strchr(), strrchr(), memchr(); 26.9 strspn(), strcspn(); 26.10 strpbrk(); 26.11 strstr(); 26.12 strtok(); 26.13 memset(), `memset_explicit; 26.14 strerror(); 26.15 strlen(). Then 27 `<tgmath.h>` Type-Generic Math Functions (27.1 Example); 28 `<threads.h>` Multithreading Functions (28.1 call_once() ... 28.6 cnd_timedwait()).
- Firefox 26.5 page (16:38): synopsis `int strcmp(const char *s1, const char *s2);` / `int strncmp(const char *s1, const char *s2, size_t n);` / `int memcmp(const void *s1, const void *s2, size_t n);`. Description: "All these functions compare chunks of bytes in memory. strcmp() and strncmp() operate on NUL-terminated strings, whereas memcmp() will compare the number of bytes you specify, brazenly ignoring any NUL characters it finds along the way. strcmp() compares the entire string down to ... strncmp() compares the first n characters of the strings... It's a little funky what they return. Basically it's ... strings are the same, it'll return zero (since th... zero if the strings differ; basically it will find th..." (cut off by the webcam overlay).
- Google Sheets "Sheet5": merged cell `"hello"`; index row 0-5; char row `h e l l o \0` (earlier `h e \0 l o \0`); cell comments by <account> reading `go to here` (timestamps 1:29 AM and 1:39 AM Today).

## Observations
- hstrcat design: first `while (*p) p++;` walks p to the NUL of s1, then reuses the strcpy idiom `while ((*p++ = *s2++));` to copy s2 including its NUL; returns s1. He types the second loop half-way, jumps up to hstrcpy to yank `while ((*p++ = *src++));` and adapts `src` to `s2`.
- Naming inconsistency: hstrcat uses s1/s2 (const s2) while hstrcpy/hstrncpy use dest/src (non-const src).
- He used the Sheets memory table (`"hello"`, indices 0-5, `h e l l o \0`) with cell comments `go to here` as a visual aid for pointer position (the NUL spot); he repaired the table row (index-2 `\0` -> `l`) during this segment.
- Test strategy: copies the Beej example lines 6-13 into main as a harness, deletes his earlier hstrncpy test ("Goats", t[100], t[2]='\0'), comments out the `numbers[]` line (strncat part not yet done), swaps `strcat` for `hstrcat`, removes the "before" printf, saves, compiles with `cc main.c`, runs `./a.out`; output `dest after strcat:  "Hello, World!"` confirms it works on first run.
- Habits: re-indents pasted code to 2 spaces, deletes leftovers with visual-line mode, reads Beej ch. 26 as the spec; after strcat he heads to 26.5 strcmp/strncmp/memcmp as the next function (hstrncat not yet written).
