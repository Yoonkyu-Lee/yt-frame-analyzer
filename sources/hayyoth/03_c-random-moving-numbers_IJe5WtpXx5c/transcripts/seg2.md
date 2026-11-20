# Segment 2 (03:43 - 08:11)

## Timeline
- [03:43] Tab `~/lab — vim main.c` active (other tab `~/lab — -zsh`). Normal mode, cursor on `{` of main (line 33), view lines 5-42. Status: `"main.c" 42L, 522B written`. Code: W 50, H 30, `int state[H][W] = {0};`, render() with single `printf("%d", state[j][i]);` in inner loop, init() sets `state[10][10] = 1;`, update() empty (blank line inside), clear_term() `usleep(500000); system("clear");`, main: `init();` `while (1) { clear_term(); render(); update(); }` `return 0;`.
- [03:47] INSERT. New line 14 being typed in render inner loop: `if` (old printf pushed to line 15).
- [03:55] INSERT. Line 14 `if (!state[j][i])`, cursor on new blank line 15 (indent 8); old printf now line 16.
- [04:02] INSERT. Line 15 `printf("%c", ' ');` typed; cursor on blank line 16 (indent 6).
- [04:04] Normal. Line 16 `else`, line 17 `printf("%d", state[j][i]);` (indent 8). Cursor on `p` of line 17. Message line: `Type  :qa!  and press <Enter> to abandon all changes and exit Vim` (shown after Ctrl-C in normal mode).
- [04:08] Scrolled; file now 45 lines (line 45 = `}` of main). Render has if/else form.
- [04:14] Tab `~/lab — -zsh` active: empty screen, prompt `lab $ `.
- [04:22] Back in vim. Cursor on `}` line 25 (matching `{` line 23 highlighted). Status: `"main.c" 45L, 586B written`.
- [04:29] update() now: line 28 blank, 29 `for (size_t j = 0; j < H; j++) {`, 30 `for (size_t i = 0; i < W; i++) {` (the two for-headers pasted from render), 31 `}` (update's close). Cursor on `f` of line 29.
- [04:30] Blank line 28 removed: update 28 `for j`, 29 `for i`, 30 `}`. Cursor on `f` line 28.
- [04:32] INSERT: line 30 blank cursor (indent 8), 31 `}`.
- [04:33] INSERT: 30 `}` (indent 4), 31 blank cursor (indent 4), 32 `}`.
- [04:36] INSERT: 28 `for j {`, 29 `for i {`, 30 blank cursor (indent 6), 31 `}`, 32 `}`, 33 `}`. Properly closed nested loops with empty body.
- [04:43] Normal, top of file (lines 1-38). Includes `<stdio.h>`, `<stdlib.h>`, `<unistd.h>`, `<time.h>` (lines 1-4); cursor on `#` of line 7 (`#define H 30`).
- [04:44] INSERT: new line 10 with `i` typed after `int state[H][W] = {0};` (line 9). Lines shift by +1 (render at 12).
- [04:48] Normal: line 10 is `int state[H][W] = {0};` duplicated (cursor on `i`; line yanked/pasted), blank lines 11-12, render at line 13. (about to be renamed to `next`).
- [04:57] INSERT. Line 11 is now `int next[H][W] = {0};`; line 10 `int state[H][W] = {0};`. New line 5 `#incl` being typed (after `#include <time.h>`); defines now lines 7-8, blank 12-13, render 14.
- [05:02] INSERT. Line 5 `#include <string.h>`. Cursor on new blank line 28 in init() (indent 2) after `state[10][10] = 1;`.
- [05:23] Normal. init line 28: `memcpy(next, state, sizeof(int) * W * H);`. Cursor on `{` of `int main(void) {` (line 44).
- [05:24] Scrolled by one line; cursor on `;` of `init();` (line 45), line 46 blank.
- [05:29] Normal: update() now has at line 37 (after inner loops, line 36 `}`) `memcpy(next, state, sizeof(int) * W * H);` (copy of init line), cursor on `m`; update closes at line 38.
- [05:44] INSERT. That update line (now line 38) reads `memcpy(state, next, sizeof(int) * W * H);` (args swapped). New line 31 `co` being typed above `void update(void) {` (now line 32).
- [05:53] INSERT. Line 31 `const int moves[8][2] = {`, line 32 blank (cursor area), line 33 `}` (braces highlighted as match).
- [05:54] INSERT. Cursor on blank line 34 after `}` of line 33; `void update` at line 35.
- [05:59] Tab `~/lab — -zsh`. A program run has been Ctrl-C'd: screen shows a single `1` (about row 9, col ~10 of terminal), mouse I-beam visible, then `^C` and prompt `lab $ ` at bottom. (The command line itself not visible; a.out was run between 05:54 and 05:59.)
- [06:01] Second tab title changed to `~/lab — vim`: bare `vim` launched in that tab: splash screen `VIM - Vi IMproved`, `version 9.1.1752`, `by Bram Moolenaar et al.`, `Vim is open source and freely distributable`, `Help poor children in Uganda!`, `type  :help iccf<Enter>       for information`, `type  :q<Enter>               to exit`, `type  :help<Enter>  or  <F1>  for on-line help`, `type  :help version9<Enter>   for version info`. Empty buffer, line number 1 shown.
- [06:02] Same empty bare vim buffer in INSERT mode (scratch).
- [06:07] Back to `vim main.c` tab. INSERT. moves array line 32 `  {A` (typing in progress, possibly a stray char).
- [06:25] INSERT. line 32 `  {-1, -1}, {-1, 0}, {-1, 1},`; line 33 `  {` (bracket-matched highlight); line 34 `}`.
- [06:36] INSERT. line 33 `  {0, -1},           {0, 1},` (wide gap where {0,0} would be); line 34 blank cursor (indent 2); line 35 `}`.
- [06:48] Normal. Array complete: line 34 `  {1, -1}, {1, 0}, {1, 1}`, line 35 `}` (no semicolon). Cursor at line 50 (blank, end of what is on screen: view lines 13-50).
- [06:49] Scrolled: view 19-56. main at line 51: `int main(void) {`, 52 `init();`, 53 blank, 54 `while (1) {`, 55 `clear_term();`, 56 `render();` (cursor on `;`).
- [07:02] INSERT. In update(): line 40 `if (state[j][i]) {` (new), 41 blank, 42 blank cursor (indent 8), 43 `}` (indent 4, old inner-for closer), 44 `}`, 45 memcpy line.
- [07:03] Normal. Closers re-indented: 43 `}` (indent 6), 44 `}` (indent 4), 45 `}` (indent 2), 46 `memcpy(state, next, sizeof(int) * W * H);`, 47 `}`. Cursor block at column 0 line 42.
- [07:37] Normal. Line 41: `const int *move = moves[rand() % 2];` (indent 8). Line 42 blank, 43 `}`. View lines 24-61; cursor at end of line 60 `update();`.
- [07:39] INSERT. In main, new line 55 `s` being typed (before `init();` now line 56): start of `srand(time(NULL));`.
- [07:47] Tab `~/lab — -zsh`: cleared screen, `lab $ `.
- [07:49] zsh: `lab $ cc main.c` with compiler output (see Shell). Corner glyphs `[` top-left of command line and `]` top-right (shell-integration prompt mark).
- [07:54] Vim normal, message `"main.c" 64L, 1013B written`. Line 35 here (still) `}` (no `;` visible? frame shows `}` on 35), line 55 `srand(time(NULL));`, 56 `init();`; cursor on `t` of `next` in line 46. (Frame ordering odd: this save appears after the failing compile; line 35 still lacks `;`.)
- [08:11] INSERT. Line 35 is now `};`. update(): 41 `const int *move = moves[rand() % 2];`, 42 `int nj = j + move[0];` (new), cursor on blank line 43 (indent 8). File shifted: update ends 48, clear_term 50-53, main 55 (srand 56, init 57, while 59).

## File states

### main.c at 08:11 (end of segment). Lines 1-23 from 05:02-05:23 frames; 24-61 visible at 08:11.
```
 1| #include <stdio.h>
 2| #include <stdlib.h>
 3| #include <unistd.h>
 4| #include <time.h>
 5| #include <string.h>
 6|
 7| #define W 50
 8| #define H 30
 9|
10| int state[H][W] = {0};
11| int next[H][W] = {0};
12|
13|
14| void render(void) {
15|   for (size_t j = 0; j < H; j++) {
16|     for (size_t i = 0; i < W; i++) {
17|       if (!state[j][i])
18|         printf("%c", ' ');
19|       else
20|         printf("%d", state[j][i]);
21|     }
22|     printf("\n");
23|   }
24| }
25|
26| void init(void) {
27|   state[10][10] = 1;
28|   memcpy(next, state, sizeof(int) * W * H);
29| }
30|
31| const int moves[8][2] = {
32|   {-1, -1}, {-1, 0}, {-1, 1},
33|   {0, -1},           {0, 1},
34|   {1, -1}, {1, 0}, {1, 1}
35| };
36|
37| void update(void) {
38|   for (size_t j = 0; j < H; j++) {
39|     for (size_t i = 0; i < W; i++) {
40|       if (state[j][i]) {
41|         const int *move = moves[rand() % 2];
42|         int nj = j + move[0];
43|         ⟨cursor, blank, indent 8⟩
44|       }
45|     }
46|   }
47|   memcpy(state, next, sizeof(int) * W * H);
48| }
49|
50| void clear_term(void) {
51|   usleep(500000);
52|   system("clear");
53| }
54|
55| int main(void) {
56|   srand(time(NULL));
57|   init();
58|
59|   while (1) {
60|     clear_term();
61|     render();
62|     update();
?? (lines 63-65 not visible this segment; presumably `  }` / `  return 0;` / `}`)
```
Notes: line 33 gap between `{0, -1},` and `{0, 1},` is ~11 spaces. File was 64L at 07:54 save (before `int nj` line added); 08:11 would be 65L.

### Intermediate versions
- 03:43 (42L, 522B): render prints `%d` only; init `state[10][10] = 1;`; update empty.
- 04:22 (45L, 586B): render uses braceless `if (!state[j][i]) printf("%c", ' '); else printf("%d", state[j][i]);`.
- 04:36: update():
```
27| void update(void) {
28|   for (size_t j = 0; j < H; j++) {
29|     for (size_t i = 0; i < W; i++) {
30|
31|     }
32|   }
33| }
```
- 05:02: `#include <string.h>` (line 5), `int next[H][W] = {0};` (line 11).
- 05:23: init has `memcpy(next, state, sizeof(int) * W * H);`.
- 05:44: update ends with `memcpy(state, next, sizeof(int) * W * H);` (first pasted as `(next, state, ...)` at 05:29, then args swapped).
- 06:48: complete `moves` table, no `;` after `}`.
- 07:54: `"main.c" 64L, 1013B written`.
- 08:11: `};` fixed, `int nj = j + move[0];` begun.

## Shell & outputs
- [04:14] zsh tab: `lab $ ` (empty).
- [05:59] zsh tab after running (command not visible): static screen with only a single `1` near row 9/col ~10 (`state[10][10] = 1`, no movement since update() did nothing), then `^C` and prompt `lab $ `.
- [06:01] bare `vim` in a tab (title `~/lab — vim`): Vim 9.1.1752 splash.
- [07:47] `lab $ ` blank.
- [07:49]:
```
lab $ cc main.c
main.c:35:2: error: expected ';' after top level declarator
   35 | }
      |  ^
      |  ;
main.c:41:20: warning: unused variable 'move' [-Wunused-variable]
   41 |         const int *move = moves[rand() % 2];
      |                    ^~~~
1 warning and 1 error generated.
lab $
```
(clang via `cc`, default output a.out.)

## Observations
- Build order: grid `state` -> render (space for 0, `%d` otherwise) -> update skeleton (double for-loop copy-pasted from render) -> second buffer `next` (line duplicated and renamed) -> `memcpy` in init (next <- state) and at end of update (state <- next) -> `moves[8][2]` neighbor-offset table -> `if (state[j][i])` selects `moves[rand() % 2]` (placeholder; only first 2 rows used) -> `srand(time(NULL))` in main -> compile error (missing `;` after table) -> start computing destination `int nj = j + move[0];`.
- Design: double buffering with `next[]` so that a moved number is not moved again within the same scan; `const int *move` points at a row of the table. `size_t` loop indices with int offsets.
- Style: 2-space indent, K&R braces, braceless if/else in render, `void` param lists, `usleep(500000)` + `system("clear")`.
- Editor habits: vim with line numbers, yank/paste, a scratch bare `vim` in the second tab (06:01-06:02), saves frequently (checksums 42L 522B -> 45L 586B -> 64L 1013B).
- Compiled with plain `cc main.c`; ran a.out once (static `1`) and Ctrl-C'd.
- Prompt is `lab $ `. No `#` shell comments or on-screen text. Webcam ignored.
