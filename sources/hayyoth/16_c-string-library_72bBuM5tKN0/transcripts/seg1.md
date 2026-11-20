# Segment 1 (frames 0:00 - 3:27) - 72bBuM5tKN0

Setup: vim inside a macOS Terminal window (window title "Terminal", top-left). A second Terminal window (prompt `lab $`) is used for compiling/running and is moved around. Behind them: Firefox window 1 at `beej.us/guide/bgclr/html/split/` and Firefox window 2 with a Google Sheets tab (`Sheet5`). Webcam overlay "Human" ignored.

## Timeline
- [00:00] Empty Terminal, prompt `lab $ `. Behind: Sheets tab "Sheet5" with a hand-drawn table: merged header cell `"hello"`, index row `0 1 2 3 4 5`, char row `h e l l o \0` (row 5, columns B-G). Firefox beej.us tab behind it.
- [00:04] vim, INSERT mode `4,11`. Typing: `#include <stdio.h>` / blank / `int main(void) {` / `  char *me` (typing) / `  return 0;` / `}`. Indent 2 spaces.
- [00:18] Status `"main.c" [New] 8L, 100B written` (6,0-1). Code now has `char *mess = "Hello";` and `printf("%s\n", mess);` plus a blank line (cursor on line 6) before `return 0;`.
- [00:19] Second Terminal window appears over vim, prompt shows `lab $ cc` being typed.
- [00:24] Back in vim (same save message, 6,0-1).
- [00:26] INSERT `2,7`: new line 2, typing `#inclu` (becomes `#include <string.h>`).
- [00:40] Status `"main.c" 10L, 153B written` (7,32). Code: `#include <stdio.h>` / `#include <string.h>` / blank / `int main(void) {` / `  char *mess = "Hello";` / `  printf("%s\n", mess);` / `  printf("%zu\n", strlen(mess));` / blank / `  return 0;` / `}`.
- [00:41] Terminal 2 fresh `lab $ `.
- [00:44] Terminal 2: `lab $ cc main.c` then new `lab $ ` (compiled, no output). Terminal window repositioned.
- [00:48] vim, same 10L, 153B written message.
- [00:49] INSERT `6,3`: blank indented line inserted between `char *mess = "Hello";` and the printfs.
- [00:56] INSERT `7,3`: line `  mess[2] = '\0';` added after `char *mess = "Hello";`.
- [01:02] Trailing comment `// don't do that` added after `mess[2] = '\0';`. Terminal 2 shows: `lab $ cc main.c` / `lab $ ./a.out` / `Hello` / `5` / `lab $ c` (typing). vim shows "main.c" 12L.
- [01:07] Status `"main.c" 12L, 189B written` (5,13). Cursor between `*mess` and `=` (about to edit declaration). Comment still `// don't do that`.
- [01:18] Declaration changed to `char mess[] = "Hello";`, comment removed: `  mess[2] = '\0';`. Terminal 2: `lab $ cc main.c` / `lab $ ./a.out` / `Hello` / `5` / `lab $ cc main.c` / `lab $ ./a.out` / `zsh: bus error  ./a.out` / `lab $ c` (typing). (The bus error came from running the `char *mess` version that wrote to mess[2].)
- [01:22] Terminal 2 additionally shows: `lab $ cc main.c` / `lab $ ./a.out` / `He` / `2` / `lab $ `. Google Sheets window brought to front, mouse pointer over the `o` cell in row 5 (explaining).
- [01:31] vim: `"main.c" 12L, 174B written` (6,18). Sheets row 5 now bold and edited: `h e \0 l o \0` (index 2 `l` replaced by `\0`), cell selection E7.
- [01:37] vim INSERT `3,1`: blank lines at 2 and 3; `#include <string.h>` is gone from line 2. Code shifted down.
- [01:40] Same state (INSERT 3,1).
- [01:41] macOS Mission Control view (Desktop 1/2/3). Shows vim terminal, Terminal 2, Sheets window, and the Firefox beej.us page: "26 <string.h> String Manipulation" TOC: 26.1 memcpy(), memccpy(), memmove(); 26.2 strcpy(), strncpy(); 26.3 strdup(), strndup(); 26.4 strcat(), strncat(); 26.5 strcmp(), strncmp(), memcmp(); 26.6 strcoll(); 26.7 strxfrm(); 26.8 strchr(), strrchr(), memchr(); 26.9 strspn(), strcspn(); 26.10 strpbrk(); 26.11 strstr(); 26.12 strtok(); 26.13 memset(), `memset_explicit; 26.14 strerror(); 26.15 strlen(). Then 27 <tgmath.h> Type-Generic Math Functions (27.1 Example), 28 <threads.h> Multithreading Functions (28.1 call_once() ...).
- [01:42] Firefox beej page in front, same chapter-26 TOC.
- [01:44] Page navigated to `.../split/stringref...` section "26.15 strlen()": "Returns the length of a string". Synopsis: `#include <string.h>` / `size_t strlen(const char *s);`. Description: "This function returns the length of the passed null-terminated string (not counting the NUL character at the end). It does this by walking down the string and counting the bytes until the NUL character, so it's a little time consuming. If you have to get the length of the same string repeatedly, save it off in a variable somewhere." Return Value: "Returns the number of bytes in the string. No[te ...] the number of characters in a multibyte string" (partly occluded by webcam). Then "Example".
- [01:52] Back in vim, INSERT `3,1`; the synopsis line `size_t strlen(const char *s);` is highlighted in the browser.
- [01:54] vim line 3 now `size_t strlen(const char *s) {` (INSERT 5,3) with an empty indented body line; closing `}` present.
- [02:04] Renamed and started body: `size_t vip_strlen(const char *s) {` / `  char *p = s;` / `  while` (typing, 5,8) / `}`.
- [02:11] `  while (*p++);` then new line; INSERT 6,3.
- [02:26] Body complete: `  return (size_t)(p - s) - 1;`. Status line shows "main.c" 17L (saved). Terminal 2 reopened with `lab $ c`.
- [02:27] Terminal 2 compile output (verbatim, see Shell section): 1 warning for `char *p = s;`.
- [02:29] vim `"main.c" 17L, 257B written` (14,22). Call site edited to `printf("%zu\n", vip_strlen(mess));`.
- [02:33] Fix in progress: line 5 now `  const char *p = s;` (typed `const `). Terminal 2 still shows the previous warning.
- [02:38] `"main.c" 17L, 263B written` (4,8), with `const char *p = s;` and `vip_strlen(mess)` in main.
- [02:39] INSERT `3,1`: a blank line inserted above `size_t vip_strlen`.
- [02:44] Comment added at line 3: `// what is this code`. INSERT `10,1`: blank line added after vip_strlen closing brace.
- [02:53] Line 10: `size_t normal_strlen(const char *s) {` with empty body line (12,3 Top).
- [03:00] Body: `  size_t len;` then blank line (12,3).
- [03:17] `  for (len = 0; s[i] != '\0'; i++)` (note: uses `i` not `len`, a bug as typed). 13,5.
- [03:21] `  for (len = 0; s[i] != '\0'; i++);` semicolon added, INSERT 12,36.
- [03:22] Blank line added below the for (14,3).
- [03:27] Scrolled view (NORMAL, 24,11, 75%): normal_strlen finished with `  return len;`. No save message visible.

## File states
### main.c at end of segment (about 3:27; line numbers reconstructed, cursor on line 24)
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
10| size_t normal_strlen(const char *s) {
11|   size_t len;
12|   for (len = 0; s[i] != '\0'; i++);
13| 
14|   return len;
15| }
16| 
17| int main(void) {
18|   char mess[] = "Hello";
19|   mess[2] = '\0';
20| 
21|   printf("%s\n", mess);
22|   printf("%zu\n", vip_strlen(mess));
23| 
24|   return 0;
25| }
```
Notes: `#include <string.h>` was removed at about 1:37. Last save message seen: `"main.c" 17L, 263B written` (2:38); later edits (comment, normal_strlen) were unsaved in frames.

### Intermediate version (0:40-0:48, `"main.c" 10L, 153B written`)
```
 1| #include <stdio.h>
 2| #include <string.h>
 3| 
 4| int main(void) {
 5|   char *mess = "Hello";
 6|   printf("%s\n", mess);
 7|   printf("%zu\n", strlen(mess));
 8| 
 9|   return 0;
10| }
```

### Intermediate version (1:18-1:31, `"main.c" 12L, 174B written` at 1:31)
```
 1| #include <stdio.h>
 2| #include <string.h>
 3| 
 4| int main(void) {
 5|   char mess[] = "Hello";
 6|   mess[2] = '\0';
 7| 
 8|   printf("%s\n", mess);
 9|   printf("%zu\n", strlen(mess));
10| 
11|   return 0;
12| }
```
(At 1:07 the saved version, 189B, had `char *mess = "Hello";` and `mess[2] = '\0'; // don't do that`.)

### Google Sheets (Sheet5), at 1:31
Row 3: merged header `"hello"` (cols B-G). Row 4: `0 1 2 3 4 5`. Row 5: `h e \0 l o \0` (bold; the original `l` at index 2 replaced by `\0`).

## Shell & outputs
- [00:19] `lab $ cc` (typing)
- [00:44] `lab $ cc main.c` (no output)
- [01:02] `lab $ ./a.out` -> `Hello` / `5`
- [01:18] `lab $ cc main.c`, `lab $ ./a.out` -> `zsh: bus error  ./a.out` (char *mess; writing mess[2] to a string literal)
- [01:22] `lab $ cc main.c`, `lab $ ./a.out` -> `He` / `2` (char mess[] version)
- [02:27] 
```
lab $ cc main.c
main.c:4:9: warning: initializing
      'char *' with an expression of type 'const char *'
      discards qualifiers
      [-Wincompatible-pointer-types-discards-qualifiers]
    4 |   char *p = s;
      |         ^   ~
1 warning generated.
lab $ 
```
- No compile of the final normal_strlen or of the const fix shown in this segment.

## Observations
- Author starts by demonstrating C string fundamentals: string literal via `char *` vs `char[]`, the NUL terminator, and that strlen stops at the first `\0` (writing `mess[2] = '\0'` yields "He"/2), with a spreadsheet diagram of memory cells for "hello" and a modified diagram with `\0` at index 2.
- Demonstrates a bus error from modifying a string literal (`// don't do that`), then fixes it via an array.
- Starts rewriting strlen by copying the signature from Beej's Guide 26.15 (the reference is the source of the prototype).
- First implementation is named `vip_strlen` (pointer-walk: `while (*p++); return (size_t)(p - s) - 1;`), with a `// what is this code` comment, apparently contrasting a terse idiomatic version against a `normal_strlen` loop version. Prefix is `vip_`/`normal_` here, not yet the `h` prefix.
- The compiler warning on `char *p = s;` is fixed by adding `const` (const-correctness following the Beej prototype).
- The `normal_strlen` for loop mixes `len` and `i` (bug as typed, unfixed in this segment).
- Workflow: edit in vim, `:w`, compile with plain `cc main.c`, run `./a.out`, using a second Terminal window; Mission Control used to switch windows.
