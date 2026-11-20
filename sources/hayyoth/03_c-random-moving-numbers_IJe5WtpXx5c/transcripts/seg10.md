# Segment 10 (22:48 - 24:40) transcript

## Timeline

### Run of the finished (v1) program, 22:48 - 23:09
Tab bar: tab 1 `~/lab — vim main.c`, tab 2 `~/lab — a.out` (active, spinner = running). Full-screen Terminal, no prompt visible, block cursor at bottom-left (~row 24 at y~763). Webcam bottom-right (ignored).

Animation behavior (not transcribed per number): a sparse field in the upper-left of the screen (never wider than ~col 30; starts ~25 rows tall, fills the screen height by ~23:05). Mostly single `1`s scattered. Two columns of `F` (food) near col 4: `F` at rows ~2-4 and one at ~row 10. Numbers adjacent to `F` grow over time (food-adjacent counters reach 7, 9, 10, 11, 12, 13, 20). A dense cluster in the middle at ~row 10 prints multi-digit numbers with no spaces (e.g. `33`, `30`, `131`, `29`, `128`, `25 2`, `22 2`, `21 2`, `20 1`, `17 1`, `115`, `213`, `119`, `291`); it looks like it jitters/shrinks, and neighbouring digits run together (e.g. `1`+`28`). Cell values seen: F,1-13,15,17,18,20,25,29,30,33. Density of `1`s grows and spreads downward row by row, reaching the last row by ~23:04.

- [22:48] `F 3 1` / `2 F 1 2` / `4 F` top-left; `33` at row ~10 center; ~25 rows used.
- [22:49] center `30`; `3 F`; `2 F 2 1 1 1`.
- [22:50] center `131`; `1 4 F 1 1`; `1 F 1 1 1`.
- [22:51] `131 1 1 1 1`; `1 2 F 3`; `1 7 F 2`.
- [22:52] center `29`; `112 F 2`; `F 3 2`.
- [22:53] `128`; `215 F 1 1`; `F 4 1`; top row `3 1 6 1`.
- [22:54] `25 2`; `1 113 F`; `1 F 5`; top row `5 7 2 1`. Webcam man leans toward camera.
- [22:55] `22 2`; `112 F`; `1 1 F 5 2 1 1`; top row `5 1 9`.
- [22:56] `21 2`; `1 15 F`; `F 7 2`; top row `2 4 4 9`.
- [22:57] `121`; `14 F 1 2`; `F 6 3 1`; top row `2 4 7 9 1`.
- [22:58] `20 1 1`; `1 4 710`; `113 F 2 2 1`; `F 6 4`.
- [22:59] `22 1`; `1 5 7 9 2`; `2 113 F 3 2`; `2 F 7 7 1`.
- [23:00] `119`; `2 9 611 1`; `2 3 3 F 1 1 1`; `211 F 1 4`; `1 5 F 6 7 2`.
- [23:01] `17 1`; `2 9 7 9`; `2 313 F 4 4`; `1 2 5 F 3 6`.
- [23:02] `115`; `110 611 1 1`; `4 1 5 F 2`; `116 F 3 1`; `3 7 F 4 6`.
- [23:03] `213`; `210 713 1`; `2 2 6 F`; `15 F 5 1`; `4 5 F 6 5 4`. Field reaches ~row 27.
- [23:04] `11 1`; `110 813`; `2 210 F 5`; `17 F 3 2`; `1 2 6 F 6 3 2 1`; a `1` right above the cursor row (field fills screen).
- [23:05] `119`; `1 4 9 815`; `1 12 F 3 1`; `1 18 F 4 2 1`; `2 6 F 7 4 1 1`.
- [23:06] `291`; `3 1 8 713`; `1 212 F 4 1 3`; `119 F 9 1`; `1 1 8 F 6 2 1`. Accidental mouse text selection (light-gray highlight) over rows 2-10 while mouse I-beam is at ~row 10 col 4.
- [23:07] selection gone; `3 213 512 1 1`; `210 F 8`; `1 220 F11`; `8 F 8 1 1`.
- [23:08] Mouse arrow at tab 1 (small `x` close button shows on tab 1). `3 215 513 2 1`; `1 2 9 F10`; `22 F 9 1`; `9 F 6 3 2`; mid `8`.
- [23:09] Webcam man sits upright. `3 117 314 1`; `4 5 F13 1 1`; `1 20 F10 3`; `2 6 F12`; `6 F 4 2`. Mouse I-beam mid-screen.

### Back in vim, 23:10 - 24:06
- [23:10] Switched to tab 1: `~/lab — vim main.c` active, tab 2 now titled `~/lab — -zsh`. Vim shows lines 35-72. Bottom status line: `"main.c" 86L, 1504B written`. Mouse arrow on tab 1. Cursor (highlighted char) on the `0` of line 51 `if (state[j][i] > 0) {`. (This saved v1 has line 59 `next[j][i]++;`.)
- (no keyframes 23:11 - 23:55)
- [23:56] Same view (35-72). Status line: `Type  :qa!  and press <Enter> to abandon all changes and exit Vim` (a Ctrl-C was pressed in vim normal mode). Line 59 is now `next[nj][ni]++;` (edited from `next[j][i]++;`); cursor on the `n` of `ni` in line 59.
- [23:57] Line 59 duplicated: lines 59 AND 60 both `next[nj][ni]++;`; `} else {` is line 61, `next[j][i]--;` 62, `next[nj][ni]++;` 63, `}` 64; `memcpy(state, next, ...)` line 69, `}` 70, blank 71, `void clear_term(void) {` 72. Cursor at start of line 59. No message on bottom line.
- (no keyframes 23:58 - 24:04)
- [24:05] Scrolled to lines 43-80. Line 59 now `state[nj][ni]++;`, line 60 `next[nj][ni]++;`. Lines 72-80: `void clear_term(void) {`, `usleep(300000);`, `system("clear");`, `}`, blank, `int main(void) {`, `srand(time(NULL));`, `init();`, line 80 empty with cursor block on it (bottom of window).
- [24:06] Scrolled one more line (44-81): line 81 `  while (1) {` visible. Cursor block at line 76 col 0. Code unchanged. No save message seen.

### Compile and second run, 24:10 - 24:32
- [24:10] Tab 2 `~/lab — -zsh` active. Terminal still shows the last frame of the earlier animation (same field as 23:09: `3 117 314 1`, `4 5 F13 1 1`, ...), then `^C` and prompt `lab $ ` with block cursor.
- [24:11] Screen cleared; typed `lab $ cc main.c` (cursor right after `main.c`, Enter not yet shown).
- (no keyframes 24:12 - 24:22; run command presumably `./a.out`, not visible)
- [24:23] Tab 2 `~/lab — a.out` (spinner). New run of the edited program: fresh-looking field, mostly `1`s, an `F` near col 4 (~row 9) and a `44` near the middle row ~10. I-beam top-left.
- [24:24-24:31] Second-run animation. Behavior: same upper-left ~25x28 area, but now the food `F`s disappear and the big counters dissolve instead of growing: center cluster values `236` (24:24), `130` (24:25), `27 1 1 1` + `4 3` (24:26), `122` (24:27), `515` (24:28), `213` (24:29), `1 9` (24:30), `4` (24:31). Two `F`s at 24:24, one at 24:25-24:29, none from 24:30. The field becomes widely dispersed `1`s with a few `2`s, max single values ~3-9; density roughly 60-80 `1`s over the screen.
- [24:32] Tab title `~/lab — -zsh`; animation stopped with `^C`; last frame dispersed `1`s/`2`s over the whole screen; prompt `lab $ ` with block cursor.
- (no keyframes 24:33 - 24:39)
- [24:40] macOS menu bar visible (Apple, Terminal, Shell, Edit, View, Window, Help; clock `Sat Oct 17 5:13 PM`). Webcam man faces camera. Prompt line: `lab $ # ok , like this, you write your rule` (still being typed; block cursor right after `rule`). Last frame of this segment.

## File states

### main.c at 23:10 (saved: `"main.c" 86L, 1504B written`), lines 35-72 visible
```
35|   state[3][3] = -2;
36|   state[1][3] = -1;
37|   state[9][3] = -7;
38| 
39|   memcpy(next, state, sizeof(int) * W * H);
40| }
41| 
42| const int moves[8][2] = {
43|   {-1, -1}, {-1, 0}, {-1, 1},
44|   {0, -1},           {0, 1},
45|   {1, -1}, {1, 0}, {1, 1}
46| };
47| 
48| void update(void) {
49|   for (size_t j = 0; j < H; j++) {
50|     for (size_t i = 0; i < W; i++) {
51|       if (state[j][i] > 0) {
52|         const int *move = moves[rand() % 8];
53|         int nj = j + move[0];
54|         int ni = i + move[1];
55| 
56|         if (nj >= 0 && nj < H && ni >= 0 && ni < W) {
57|           if (state[nj][ni] < 0) {
58|              // if meet food
59|             next[j][i]++;
60|           } else {
61|             next[j][i]--;
62|             next[nj][ni]++;
63|           }
64|         }
65|       }
66|     }
67|   }
68|   memcpy(state, next, sizeof(int) * W * H);
69| }
70| 
71| void clear_term(void) {
72|   usleep(300000);
?? (lines 1-34 and 73-86 not visible in this segment)
```
Notes: 2-space indent; line 44 has a wide gap between `{0, -1},` and `{0, 1},`; line 45 `{1, 1}` has no trailing comma; line 58 comment has one extra space of indentation. Food cells are the negative hard-coded ones (`state[3][3] = -2; state[1][3] = -1; state[9][3] = -7;`), shown as `F`.

### Intermediate: 23:56 (unsaved edit)
Line 59 changed to `            next[nj][ni]++;` (rest as in v1).

### Intermediate: 23:57
```
57|           if (state[nj][ni] < 0) {
58|              // if meet food
59|             next[nj][ni]++;
60|             next[nj][ni]++;
61|           } else {
62|             next[j][i]--;
63|             next[nj][ni]++;
64|           }
65|         }
66|       }
67|     }
68|   }
69|   memcpy(state, next, sizeof(int) * W * H);
70| }
71| 
72| void clear_term(void) {
```

### Latest state, 24:05-24:06 (lines 43-81)
```
43|   {-1, -1}, {-1, 0}, {-1, 1},
44|   {0, -1},           {0, 1},
45|   {1, -1}, {1, 0}, {1, 1}
46| };
47| 
48| void update(void) {
49|   for (size_t j = 0; j < H; j++) {
50|     for (size_t i = 0; i < W; i++) {
51|       if (state[j][i] > 0) {
52|         const int *move = moves[rand() % 8];
53|         int nj = j + move[0];
54|         int ni = i + move[1];
55| 
56|         if (nj >= 0 && nj < H && ni >= 0 && ni < W) {
57|           if (state[nj][ni] < 0) {
58|              // if meet food
59|             state[nj][ni]++;
60|             next[nj][ni]++;
61|           } else {
62|             next[j][i]--;
63|             next[nj][ni]++;
64|           }
65|         }
66|       }
67|     }
68|   }
69|   memcpy(state, next, sizeof(int) * W * H);
70| }
71| 
72| void clear_term(void) {
73|   usleep(300000);
74|   system("clear");
75| }
76| 
77| int main(void) {
78|   srand(time(NULL));
79|   init();
80| 
81|   while (1) {
?? (lines 1-42 and 82-86+ not visible in this view; line 43 onward as above)
```
Semantics of the edit: when a walker steps onto a food cell (negative), the food is eaten one unit (`state[nj][ni]++`, moving toward 0) and the target next cell gets +1 (the source is not decremented, so walkers multiply). In v1 the walker incremented its own cell (`next[j][i]++`) and food was never consumed, so the counters beside `F` kept growing. No save message was visible for the edited version; since it was compiled at 24:11, it was presumably saved between 24:06 and 24:11 (not visible).

## Shell & outputs
- [23:10] vim: `"main.c" 86L, 1504B written`
- [23:56] vim: `Type  :qa!  and press <Enter> to abandon all changes and exit Vim`
- [24:10] zsh: previous animation frame left on screen, then `^C`, then `lab $ `
- [24:11] `lab $ cc main.c` (typed; no compiler output seen; the run command is not visible, a.out is running by 24:23)
- [24:23-24:31] a.out running (tab title `~/lab — a.out`); animation as described above
- [24:32] `^C` then `lab $ `
- [24:40] `lab $ # ok , like this, you write your rule` (typed in progress)
- Prompt format is `lab $ `. Menu-bar clock at 24:40: `Sat Oct 17 5:13 PM`.

## Observations
- Build order here: the finished v1 program runs (walkers grow around static food) -> back to vim -> change the food-meeting rule (first `next[j][i]++` -> `next[nj][ni]++`, then duplicate line, then first copy -> `state[nj][ni]++` so food depletes) -> compile with plain `cc main.c` -> run -> new behavior: food `F`s vanish one by one and counts disperse into a wide field of `1`s and `2`s -> Ctrl-C -> closing comment in shell.
- Core design: `moves[8][2]` neighbor table, `rand() % 8` pick, bounds check on `nj/ni`, double buffering via `next` + `memcpy(state, next, ...)`; negative cell = food (`F`); `clear_term()` = `usleep(300000); system("clear");`; `main` = `srand(time(NULL)); init(); while (1) {`.
- Style: 2-space indent, `size_t j`/`i` loops (j row, i column), minimal comments (`// if meet food`).
- Closing message (verbatim as visible, typed as a zsh comment): `# ok , like this, you write your rule` (visible text ends at `rule` at 24:40; may continue past this frame). Message intent: invite viewers to write their own rules.
- Accidental mouse text selection at 23:06; mouse used for switching tabs. Webcam ignored.
- Keyframe gaps: 23:11-23:55, 23:58-24:04, 24:12-24:22, 24:33-24:39.
