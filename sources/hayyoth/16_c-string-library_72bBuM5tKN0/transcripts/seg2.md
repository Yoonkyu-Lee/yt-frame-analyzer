# Segment 2 (03:27 - 05:24) - frame-only transcription

## Timeline
- [03:27] Vim in Terminal window (top-left, ~25 rows) editing `main.c`. Cursor `24,11  75%` on `return 0;`. Contains `vip_strlen` (pointer-walk version), `normal_strlen` (for loop, still BUGGY: uses `s[i]` / `i++`), `main` calling `vip_strlen(mess)`. A second Terminal window behind shows `lab $ ./a.out` -> `He` / `2`. Firefox (right) on `beej.us/guide/bgclr/html/split/stringref...` page "26.15 strlen()" ("Returns the length of a string"; Synopsis `#include <string.h>` / `size_t strlen(const char *s);` highlighted; Description: "This function returns the length of the passed null-terminated string (not counting the NUL character at the end). It does this by walking down the string and counting the bytes until the NUL character, so it's a little time consuming. If you have to get the length of the same string repeatedly, save it off in a variable somewhere."; Return Value: "Returns the number of bytes in the string. Note that this might be different than the number of characters in a multibyte string."). Webcam window "Human" ignored.
- [03:28] Scrolled; cursor `21,13  Bot` on the `printf("%s\n", mess);` line (on the `\`). Same code.
- [03:34] Bottom terminal in front. Vim status shows `"main.c" 25L, 3` (cut off). Terminal history: `lab $ cc main.c` -> warning `main.c:4:9: warning: initializing 'char *' with an expression of type 'const char *' discards qualifiers [-Wincompatible-pointer-types-discards-qualifiers]` showing `4 |   char *p = s;` with caret; `1 warning generated.` Then `lab $ cc main.c` (clean), `lab $ ./a.out` -> `He` / `2`. (So `const` was added to `p` in vip_strlen before this; the first compile was of the older non-const version.)
- [03:35] Terminal cleared; typing `lab $ cc mai` (in progress).
- [03:36] Compile output of `cc main.c`: `main.c:12:19: error: use of undeclared identifier 'i'` (`12 |   for (len = 0; s[i] != '\0'; i++);`) and `main.c:12:31: error: use of undeclared identifier 'i'`, `2 errors generated.` (the buggy normal_strlen was being compiled/called.)
- [03:38] Vim in front: `"main.c" 25L, 393B written`, cursor `22,31`. main now calls `printf("%zu\n", normal_strlen(mess));` (was vip_strlen). The `for` line still `s[i]`, `i++`.
- [03:43] Edited loop to `for (len = 0; s[len] != '\0'; len++);` (cursor on `len++`). Terminal cleared: `lab $ `. Status `"main.c" 25L, 3...` (stale).
- [03:48] `"main.c" 25L, 397B written`, cursor `12,35`. Line 12: `for (len = 0; s[len] != '\0'; len++);`
- [03:50] Terminal: `lab $ cc main.c` (no output) / `lab $ ./a.out` -> `He` / `2`.
- [03:58] Vim, cursor `12,30` on the closing `'` of `'\0'`; `"main.c" 25L, 397B written` still shown.
- [04:03] Condition simplified: `for (len = 0; s[len]; len++);` (the `!= '\0'` removed; matching-bracket highlight on `[`). Shell now shows an extra typed line `lab $ '\0' == 0 == false` and then `lab $ cc main.c` running (cursor on next line).
- [04:07] `"main.c" 25L, 389B written`, cursor `12,22`. (397-8=389: removed ` != '\0'`.) Terminal prompt `lab $ ` below.
- [04:09] A comment line was added above vip_strlen: `// what is this code` (cursor `3,20  50%`, end of the comment). Screen scrolled so the comment is the top visible line.
- [04:12] `-- VISUAL LINE --` 2 lines selected: `  size_t len;` and `  for (len = 0; s[len]; len++);` (`12,22`).
- [04:13] Normal mode, `12,22  50%`, bracket highlight on `[`/`]`.
- [04:18] `-- VISUAL LINE --` 3 lines selected: `size_t normal_strlen(const char *s) {`, `  size_t len;`, `  for (len = 0; s[len]; len++);` (selection extended upward to the function head; the visual-line count shows `3`, cursor `12,22`).
- [04:19] Whole `normal_strlen` function deleted (selection continued to the closing `}`): message `6 fewer lines`, cursor `10,0-1  Bot`. Two blank lines now between vip_strlen's `}` and `int main(void) {`; `~` filler lines at end.
- [04:20] One blank line removed: cursor `9,0-1  Bot`; zero visible blank lines between `}` and `int main` (the cursor sits on the empty line directly below `}`).
- [04:22] Top of file visible: line 1 `#include <stdio.h>`, line 2 blank, line 3 `// what is this code`. Cursor `2,0-1  All`. One blank line between vip_strlen `}` and `int main`. NOTE: `main` still calls `normal_strlen(mess)` which no longer exists (not recompiled in this segment).
- [04:27] `-- VISUAL --` selecting `(*p++);` in `while (*p++);` (`6,10`, 6 chars).
- [04:28] Mission-Control view (Desktop 1 / Desktop 2 / Desktop 3 at top; Dock visible). Shows four windows: vim Terminal (`-- VISUAL --`, `6,9`, selecting `(*p++);`), shell Terminal (history `cc main.c`, `./a.out` -> `He` `2`, `'\0' == 0 == false`, `cc main.c`, `./a.out` -> `He` `2`), Firefox beej.us 26.15 strlen(), Firefox window titled "Blank - Google Sheets" (`docs.google.com/spreadsheets/d/1Eb...`, sheet tab `Sheet5`).
- [04:29] Google Sheets window brought to front. Content: row 3 B3:G3 boxed header `"hello"`; row 4 B..G indices `0 1 2 3 4 5`; row 5 characters `h`, `e`, `\0`, `l`, `o`, `\0`. Cell E7 selected.
- [04:39] Adding a cell comment on D6 (below the `\0` in column D, index 2): empty comment input ("Comment or add others with @"). Sheet tab shows badge `1`.
- [04:40] Hover on commenter name shows a contact card (<account>, <mail address>) - redacted.
- [04:44] Comment text typed: `*p`.
- [04:45] Sheets window dragged left/up; comment box still `*p`.
- [04:51] Comment text: `*p == 0 here ` (mouse over Post Comment tooltip).
- [04:52] Comment posted: `*p == 0 here`, by <account>, `1:29 AM Today`, on D6.
- [04:53] Comment closed, C8 selected; D6 shows a small orange corner marker.
- [04:54] D6 comment re-opened (text `*p == 0 here`).
- [04:55] Reply field open ("Reply or add others with @").
- [05:01] Reply cancelled; K4 selected; marker still at D6.
- [05:04] Sheet scrolled one row (rows start at 2). New comment box on E7; tab badge `2`. Empty input.
- [05:05] Typed `go` in that comment box.
- [05:08] Comment cancelled; J5 selected; badge back to `1`.
- [05:10] New comment box on E6 (column E, i.e. under `l`/index 3); badge `2`. Empty input.
- [05:11] Typed `go ` in E6 comment.
- [05:15] Comment posted on E6: `go to h⟨?⟩` (rest hidden by cursor; at 05:18 only `here` is visible at the end, so likely `go to here`), `1:29 AM Today`.
- [05:18] Vim window brought forward over Sheets: `-- VISUAL --` `7  6,9` selecting `(*p++);`; Sheets comment popup partially visible behind (`...here`, reply input).
- [05:19] Vim `-- INSERT --` `7,3  All`: a blank (indented) line inserted between `while (*p++);` and `return (size_t)(p - s) - 1;`; cursor on it.
- [05:24] Normal mode `6,3  All`: the inserted blank line is gone again; cursor on `w` of `while`.

## File states

### main.c at start of segment (03:27, reconstructed; lines 1-3 not visible; 25L)
```
1| ?? (lines 1-3 not visible; later seen as #include <stdio.h> / blank / comment line added)
2| ??
3| ??
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
(Line numbers inferred from status positions 24,11 / 21,13 / 12,35 and `25L`.)

### Intermediate: lines 10-25 at 03:48 (saved `25L, 397B`)
```
10| size_t normal_strlen(const char *s) {
11|   size_t len;
12|   for (len = 0; s[len] != '\0'; len++);
13|
14|   return len;
15| }
16|
17| int main(void) {
18|   char mess[] = "Hello";
19|   mess[2] = '\0';
20|
21|   printf("%s\n", mess);
22|   printf("%zu\n", normal_strlen(mess));
23|
24|   return 0;
25| }
```
At 04:07 (saved `25L, 389B`) line 12 became `  for (len = 0; s[len]; len++);`

### main.c at END of segment (05:24; edits after the 04:07 write are unsaved)
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
(normal_strlen deleted at 04:19 but the call in main still references it, so compiling now would fail.)

### Google Sheets "Sheet5" ("hello" memory diagram)
```
row3: [B3:G3 boxed] "hello"
row4:  0    1    2    3    4    5       (columns B..G)
row5:  h    e    \0   l    o    \0
comment on D6 (<account>, 1:29 AM Today): *p == 0 here
comment on E6 (<account>, 1:29 AM Today): go to h⟨?⟩ (likely "go to here")
```

## Shell & outputs
- [03:27] `lab $ ./a.out` -> `He`, `2`.
- [03:34]
```
lab $ cc main.c
main.c:4:9: warning: initializing
      'char *' with an expression of type 'const char *'
      discards qualifiers
      [-Wincompatible-pointer-types-discards-qualifiers]
    4 |   char *p = s;
      |         ^   ~
1 warning generated.
lab $ cc main.c
lab $ ./a.out
He
2
lab $
```
- [03:36]
```
lab $ cc main.c
main.c:12:19: error: use of undeclared
      identifier 'i'
   12 |   for (len = 0; s[i] != '\0'; i++);
      |                   ^
main.c:12:31: error: use of undeclared
      identifier 'i'
   12 |   for (len = 0; s[i] != '\0'; i++);
      |                               ^
2 errors generated.
lab $
```
- [03:50]
```
lab $ cc main.c
lab $ ./a.out
He
2
lab $
```
- [04:03 - 04:28]
```
lab $ cc main.c
lab $ ./a.out
He
2
lab $ '\0' == 0 == false
lab $ cc main.c
lab $ ./a.out
He
2
lab $
```
(`'\0' == 0 == false` was typed at the shell as a note to self; no error text visible. Compile after removing `!= '\0'` was silent; output stayed `He` / `2`.)

Vim save messages: `"main.c" 25L, 393B written` (03:38), `"main.c" 25L, 397B written` (03:48), `"main.c" 25L, 389B written` (04:07). Delete message `6 fewer lines` (04:19).

## Observations
- Two strlen implementations side by side: `vip_strlen` (pointer walk: `while (*p++); return (size_t)(p - s) - 1;`) and `normal_strlen` (index loop), tested with `"Hello"` where `mess[2] = '\0'` truncates it to `He`, length 2 - demonstrates strlen stops at the first NUL.
- Added comment `// what is this code` above the pointer version, framing it as the thing to explain.
- Fixed `const char *p` (qualifier-discard warning) and the undeclared `i` bug (`s[len]`/`len++`); then simplified the loop to `s[len]`, relying on NUL == 0 being false (noted in the shell as `'\0' == 0 == false`).
- Deleted `normal_strlen` to focus on the pointer version; used a Google Sheets memory table for "hello" (indices 0-5 over `h e \0 l o \0`) with cell comments (`*p == 0 here`, `go to h...`) to show where `*p` hits 0 and why `p` ends one past the NUL (hence `- 1`).
- Reference consulted: Beej's Guide to C Library Reference, 26.15 strlen(). Uses Mac Mission Control to switch among windows on Desktop 1.
