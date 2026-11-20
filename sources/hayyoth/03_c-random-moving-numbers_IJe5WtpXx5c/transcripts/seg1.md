# Segment 1 (00:00 - 03:43) transcript

## Timeline
- [00:00] Full-screen Terminal (title "Terminal"), no tab bar yet. vim NORMAL, skeleton (7 lines): `1| #include <stdio.h>`, 2-3 blank, `4| int main(void) {`, 5 blank, `6|   return 0;`, `7| }`. Status line: `Type  :qa!  and press <Enter> to abandon all changes and exit Vim` (Ctrl-C pressed). Cursor on `}` line 7.
- [00:02] INSERT. Blank line opened: 1 include, 2 blank, `3| |` (cursor), 4 blank, `5| int main(void) {`, 6 blank, `7|   return 0;`, `8| }`.
- [00:16] INSERT. `3| #define W 50`, `4| #define H 30`, 5 blank, `6| int state` (typing), 7 blank, main at 8-11.
- [00:31] INSERT. `6| int state[H][W] = {0};`, `8| void render() {`, `9|   for (size_t j =` (typing), `10| }`, main at 12-15.
- [00:43] INSERT. `9|   for (size_t j = 0; j < H; j++) {`, `10|     for (size_t i = 0; i < W; i+` (typing), `11| }`, main at 13-16.
- [00:59] INSERT. `11|       printf("%d", state[j]` (typing), `12| }`, main at 14-17.
- [01:09] NORMAL. render complete (lines 8-15, see File states v1); cursor on `}` line 14. main: `17| int main(void) {`, 18 blank, `19|   return 0;`, `20| }`. (Webcam: cigarette; ignore.)
- [01:17] Switched to tab 2. Tab bar visible: `~/lab — vim main.c` | `...` | `~/lab — -zsh` (active). Prompt `lab $ `; typing `cc`.
- [01:18] `lab $ cc main.c` output (verbatim):
```
main.c:8:12: warning: a function declaration without a prototype is deprecated in all versions of C
      [-Wstrict-prototypes]
    8 | void render() {
      |            ^
      |             void
1 warning generated.
lab $
```
- [01:20] `lab $ ./a.out` prints a static 30-row x 50-col grid of bold `0` (all zeros), then prompt. NOTE: the last editor frame (01:09) had no `render();` in main, but the output proves it was added between 01:09 and 01:20 (not captured; frame 01:26 shows it).
- [01:26] Back to vim tab (NORMAL; mouse on tab bar, tab shows close "x"). `17| int main(void) {`, `18|   render();` (cursor on `;`), `19|   return 0;`, `20| }`. Line 8 still `void render() {`.
- [01:34] INSERT. Line 8 now `void render(void) {` (warning fix). Typing at line 17: `v`; 18 blank; main at 19-22.
- [01:42] INSERT. `17| void init(void) {`, `18|   state[` (typing), 19 blank, main at 20-23 (red unmatched-brace highlighting).
- [01:47] NORMAL. init complete: `17| void init(void) {`, `18|   state[10][10] = 1;`, `19| }`, 20 blank, main at 21-24 (`22|   render();`, `23|   return 0;`).
- [01:48] zsh tab (scrolled view of old run): tail of compile warning, `lab $ ./a.out`, 30 zero rows, prompt.
- [01:49] zsh tab cleared; typing `cc m`.
- [01:52] zsh: `lab $ cc main.c` / `lab $ ./a.out` / 30x50 grid all `0` / `lab $`. No warning now. Still all zeros (init() not called from main yet).
- [01:56] vim, NORMAL. Save message `"main.c" 24L, 312B written`.
- [01:57] vim VISUAL: selection of line 20 (blank) through `21| int main(void) {` ("2" in selection indicator).
- [01:58] INSERT. Blank indented line inside main: `22|   |` (cursor), `23|   render();`, `24|   return 0;`, `25| }`.
- [02:01] zsh tab, screen shows old run output (cc main.c; ./a.out; all zeros).
- [02:02] zsh: typing `cc main.c` on cleared screen.
- [02:04] zsh output after rerun: `lab $ cc main.c`, `lab $ ./a.out`, 30x50 grid of `0`, with a single `1` in the 11th row (0-based row 10) at 0-based column 10 (ten 0s then `1`). = state[10][10]=1 from init().
- [02:08] vim NORMAL, mouse at tab bar. Save message `"main.c" 25L, 322B written`. main: `21| int main(void) {`, `22|   init();` (cursor on `;`), `23|   render();`, `24|   return 0;`, `25| }`.
- [02:17] INSERT. Blank line opened between init `}` (19) and main: `20| |` (cursor), 21 blank, main at 22-26.
- [02:20] INSERT. `21| void update(void` (typing), main at 23-27, red brace highlight.
- [02:22] NORMAL. `21| void update(void) {`, 22 blank, `23| }`, `24|` (cursor block), main at 25-29 (`26|   init();`, `27|   render();`).
- [02:24] INSERT. `25| v` typed, 26 blank, main at 27-31.
- [02:29] INSERT. `25| void clear_term(void) {`, 26 blank, `27|   |` (cursor), 28 blank, main at 29-33.
- [02:52] INSERT. `26|   usleep(500000);`, `27|   sys` (typing), `28| }`, main at 30-34. (Webcam: man leaned/moving.)
- [02:57] INSERT. clear_term complete (`26| void clear_term(void) {`, `27|   usleep(500000);`, `28|   system("clear");`, `29| }`). Cursor on new blank line 2 at top: `1| #include <stdio.h>`, `2| |`, 3 blank, `4| #define W 50`, `5| #define H 30`; main at 31-35 (`32| init();`, `33| render();`, `34| return 0;`).
- [03:01] INSERT. `2| #include <stdlib.h>`, `3| #in` (typing); defines at 5-6; main at 32-36.
- [03:05] INSERT. `3| #include <unistd.h>`, `4| #` (typing); defines at 6-7.
- [03:17] INSERT. Viewport starts at line 2. `4| #include <time.h>`. main: `33| int main(void) {`, `34|   init();`, 35 blank, `36|   while (1) {`, 37 blank, 38 blank, `39|     |` (cursor; rest below view). (Frame also shows render() not visible; rest unknown.)
- [03:18] NORMAL. Viewport starts at line 5. `33| int main(void) {` (`{` highlighted), `34|   init();`, 35 blank, `36|   while (1) {`, 37 blank, 38 blank, `39|   }`, `40|   render();`, `41|   return 0;`, `42| }` (render() still after loop).
- [03:26] NORMAL with `Type  :qa!  and press <Enter> to abandon all changes and exit Vim` and pending `d` in command area. `36|   while (1) {`, `37|` (cursor block), `38|     render();`, 39 blank, `40|   }`, `41|   return 0;`, `42| }` (render moved into loop).
- [03:27] INSERT. `36|   while (1) {`, `37|     render();`, `38|   |` (cursor), `39|   }`, `40|   return 0;`, `41| }` (41 lines).
- [03:43] NORMAL. Save message `"main.c" 42L, 522B written`. Loop body: `37|     clear_term();`, `38|     render();`, `39|     update();`, `40|   }`, `41|   return 0;`, `42| }`. End of segment. Program not re-run after the loop was added within this segment.

## File states
Frame-verified snapshots (line numbers as shown by vim).

### main.c at 01:09 (v1, NORMAL, before first compile)
```
1| #include <stdio.h>
2|
3| #define W 50
4| #define H 30
5|
6| int state[H][W] = {0};
7|
8| void render() {
9|   for (size_t j = 0; j < H; j++) {
10|     for (size_t i = 0; i < W; i++) {
11|       printf("%d", state[j][i]);
12|     }
13|     printf("\n");
14|   }
15| }
16|
17| int main(void) {
18|
19|   return 0;
20| }
```
(By 01:26: `18|   render();`, `19|   return 0;`.)

### main.c at 02:17 (saved at 02:08 as `"main.c" 25L, 322B written`)
```
1| #include <stdio.h>
2|
3| #define W 50
4| #define H 30
5|
6| int state[H][W] = {0};
7|
8| void render(void) {
9|   for (size_t j = 0; j < H; j++) {
10|     for (size_t i = 0; i < W; i++) {
11|       printf("%d", state[j][i]);
12|     }
13|     printf("\n");
14|   }
15| }
16|
17| void init(void) {
18|   state[10][10] = 1;
19| }
20|
21| int main(void) {
22|   init();
23|   render();
24|   return 0;
25| }
```
(Earlier save at 01:56: `"main.c" 24L, 312B written` = same without `init();` in main.)

### main.c at END of segment (03:43, `"main.c" 42L, 522B written`)
Lines 1-4 seen at 03:17/03:18 (line 1 stdio.h seen in earlier frames); lines 5-42 seen at 03:43.
```
1| #include <stdio.h>
2| #include <stdlib.h>
3| #include <unistd.h>
4| #include <time.h>
5|
6| #define W 50
7| #define H 30
8|
9| int state[H][W] = {0};
10|
11| void render(void) {
12|   for (size_t j = 0; j < H; j++) {
13|     for (size_t i = 0; i < W; i++) {
14|       printf("%d", state[j][i]);
15|     }
16|     printf("\n");
17|   }
18| }
19|
20| void init(void) {
21|   state[10][10] = 1;
22| }
23|
24| void update(void) {
25|
26| }
27|
28| void clear_term(void) {
29|   usleep(500000);
30|   system("clear");
31| }
32|
33| int main(void) {
34|   init();
35|
36|   while (1) {
37|     clear_term();
38|     render();
39|     update();
40|   }
41|   return 0;
42| }
```

## Shell & outputs
- [01:17-01:18] `lab $ cc main.c` ->
```
main.c:8:12: warning: a function declaration without a prototype is deprecated in all versions of C
      [-Wstrict-prototypes]
    8 | void render() {
      |            ^
      |             void
1 warning generated.
```
- [01:20] `lab $ ./a.out` -> 30 lines x 50 chars of `0` (static, no animation), back to prompt.
- [01:49-01:52] `cc main.c` (no warnings), `./a.out` -> same all-zero grid (init() not yet called from main).
- [02:04-02:08] `cc main.c`, `./a.out` -> 30x50 grid of zeros with one `1` at row index 10 (11th row), column index 10 (ten 0s then a `1`).
- Prompt `lab $ `; tabs titled `~/lab — vim main.c` and `~/lab — -zsh`. Compiler is Apple clang via `cc`, no flags (output a.out). No animation seen yet in this segment (program has no loop until 03:43 save; not run again by 03:43).

## Observations
- Build order: skeleton main -> `#define W 50`, `#define H 30` -> global `int state[H][W] = {0};` -> render() with nested size_t loops printing `%d` per cell and `\n` per row -> compile/run (static zeros) -> call render in main -> fix `void render()` to `void render(void)` after clang -Wstrict-prototypes warning -> init() seeding state[10][10]=1 -> call init() before render -> empty update() stub -> clear_term() (`usleep(500000); system("clear");`) -> add includes stdlib.h, unistd.h, time.h (time.h hints at later `srand(time(NULL))`) -> `while (1)` loop in main with clear_term(); render(); update();.
- Style: 2-space indent, K&R braces, `void f(void)`, size_t loop indices j (rows, H) and i (cols, W), state indexed [j][i] = [row][col]. Per-cell `%d` with no separator.
- Test-as-you-go: compiles and runs after each small step.
- vim tricks seen: Ctrl-C status message shows at 00:00 and at 03:26; VISUAL line select at 01:57; render() moved from after the loop into the loop at ~03:18-03:27 (cut with `d...`, paste).
- No man pages, browser, or shell `#` comments in this segment. The webcam window is ignored.
- Uncertainties: render() call being added to main between 01:09 and 01:17 not captured; Lines 1-4 at the end are inferred from 03:17-03:18 frames (not visible at 03:43 because the viewport starts at line 5).
