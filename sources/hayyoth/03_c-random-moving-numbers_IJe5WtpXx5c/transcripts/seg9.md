# Segment 9 (18:50 - 22:48)

## Timeline
- [18:50] Tab `~/lab — vim main.c` (2nd tab `~/lab — -zsh`). Normal mode, view starts at line 4. File is 41 lines. init() lines 26-34: `state[10][10] = 90;`, comment `// < 0 is food` (line 29), then THREE identical lines 30-32 `state[2][3] = -4;` (just yanked/pasted), line 33 `memcpy(next, state, sizeof(int) * W * H);`, 34 `}`. `moves[8][2]` at lines 36-40, line 41 empty. Cursor on line 32.
- [18:51] A fourth copy: lines 30-33 all `state[2][3] = -4;`, memcpy line 34, moves `};` at line 41. Cursor on line 31.
- [19:07] Food lines edited to distinct values: 30 `state[2][3] = -4;`, 31 `state[3][3] = -2;`, 32 `state[1][3] = -1;`, 33 `state[9][3] = -7;`; new empty line 34 (cursor there), memcpy line 35, `}` 36, `const int moves[8][2] = {` line 38.
- [19:15] INSERT mode in render(): the else is now `else {` (line 19), empty lines 20-21 (cursor on 21), then `printf("%2d", state[j][i]);` line 22, `}` line 23. File shifted +2 (init at 28, moves at 40).
- [19:32] Normal mode: line 19 `else {`, line 20 `if (state[j][i] > 0)`, line 21 `printf("%2d", state[j][i]);` (cursor on `p`, not yet indented deeper), 22 `}`, 23 `}`.
- [19:34] INSERT: printf line 21 re-indented deeper; typing `el` on line 22 (about to write else).
- [19:37] INSERT: line 22 `else`, line 23 `prin` being typed.
- [19:47] Normal mode, view lines 10-47. render() complete (line 23 `printf("%2c", 'F');`). File 47 lines, cursor on empty line 47 after moves `};` (46).
- [19:48] Scrolled; new function at line 48: `void update(void) {`, 49 `for (size_t j = 0; j < H; j++) {`, 50 `for (size_t i = 0; i < W; i++) {` (cursor on `;` after `W`).
- [19:51] update() body (lines 48-61): `if (state[j][i]) {`, `const int *move = moves[rand() % 8];`, `int nj = j + move[0];`, `int ni = i + move[1];`, blank, `if (nj >= 0 && nj < H && ni >= 0 && ni < W) {`, `next[j][i]--;`, `next[nj][ni]++;`, closing braces. Cursor on `}` line 61 (matching `{` highlighted on line 50).
- [19:52] update() complete: `memcpy(state, next, sizeof(int) * W * H);` line 63, `}` 64. `void clear_term(void) {` line 66, `usleep(300000);` line 67 (cursor on `;`).
- [19:56] clear_term done: 66 `void clear_term(void) {`, 67 `usleep(300000);`, 68 `system("clear");`, 69 `}`; line 71 `int main(void) {` (cursor on `{`), line 72 `srand(time(NULL));`. Rest of main not visible.
- [20:09] INSERT mode: new empty line 57 inserted inside the bounds-check `if`, before `next[j][i]--;` (lines shift +1: next-- 58, memcpy 64, clear_term 67, main 72).
- [20:38] INSERT: `if (next[nj][ni] < 0) {` typed on line 57, auto-closed `}` on line 59, empty line 58 between; `next[j][i]--;` 60, `next[nj][ni]++;` 61.
- [20:58] Condition changed to `if (state[nj][ni] < 0) {`. INSERT: line 58 `// if meet food` (one col deeper than expected), line 59 `// ` being typed. `}` line 60, `next[j][i]--;` 61, `next[nj][ni]++;` 62.
- [21:06] VISUAL LINE mode (count "1") on line 61 `next[j][i]--;`. Line 59 emptied: 58 `// if meet food`, 59 blank, 60 `}`.
- [21:08] Normal mode, cursor line 58 col 0; same text.
- [21:09] Lines 59-60 now `next[j][i]--;` / `next[nj][ni]++;` (duplicated copy pasted into the if body, still at the brace indent), 61 `}`, then original 62 `next[j][i]--;`, 63 `next[nj][ni]++;`. Cursor line 59.
- [21:10] VISUAL LINE on line 59 (count 1).
- [21:11] VISUAL LINE on lines 59-60 (count 2); text is shifted right (indented to comment column).
- [21:12] Normal mode; lines 59-60 indented to the column of the `// if meet food` comment; cursor col just before `next`.
- [21:42] Food branch reduced: line 59 `next[j][i]++;` only (the `next[nj][ni]++` copy removed); `}` line 60; `next[j][i]--;` 61; `next[nj][ni]++;` 62; `}` 63; memcpy at 67. Cursor on `}` line 60; `{` of line 57 highlighted.
- [21:47] Line 60 now `} else {`; lines 61-62 still old indent; `}` line 63 (cursor; matching `{` on line 60 highlighted).
- [21:48] VISUAL LINE on lines 61-62 (count 2) to re-indent.
- [21:49] Lines 61-62 indented one level (aligned with line 59 `next[j][i]++;`). Ex prompt `:` open at the bottom (empty so far).
- [21:53] Switched to tab `~/lab — -zsh` (now highlighted): prompt `lab $ cc` (typing; command incomplete in frame). Terminal otherwise blank.
- [22:01] Back in vim tab. Message: `"main.c" 86L, 1500B written`. Cursor line 62. Text as in File states.
- [22:13] zsh tab: empty prompt `lab $ ` (screen cleared; no compiler output visible, so presumably compiled cleanly; full compile command not seen).
- [22:38-22:48] Tab title `~/lab — a.out` with spinner: program running. 11 animation frames, described under Shell & outputs.

## File states
main.c as of 22:01 (`"main.c" 86L, 1500B written`).

```
 1| ?? (lines 1-3 not visible in this segment)
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
18|         printf("%2c", ' ');
19|       else {
20|         if (state[j][i] > 0)
21|           printf("%2d", state[j][i]);
22|         else
23|           printf("%2c", 'F');
24|       }
25|     }
26|     printf("\n");
27|   }
28| }
29|
30| void init(void) {
31|   state[10][10] = 90;
32|
33|   // < 0 is food
34|   state[2][3] = -4;
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
51|       if (state[j][i]) {
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
73|   system("clear");   (seen at 21:42; hidden by status line at 22:01)
74| }
75|
76| int main(void) {
77|   srand(time(NULL));
78| ?? (lines 78-86 not visible: rest of main, ~9 lines)
```
Indentation is 2 spaces per level as shown on screen; the `// if meet food` comment appears one column right of the following line. In moves[], the second row has a wide gap (~11 spaces) between `{0, -1},` and `{0, 1},`. Line 73 inferred from 19:56/21:42 frames (`system("clear");` visible at line 68/72 then).

Intermediate versions:
- 18:50 init has 3 duplicate `state[2][3] = -4;`; 18:51 four; 19:07 distinct (-4, -2, -1, -7).
- 19:15-19:47 render else branch grew from a single printf to nested if/else with `'F'` for negative (food).
- 19:47-19:56 update(), clear_term(), start of main() written.
- 20:38: `if (next[nj][ni] < 0) {`, changed to `state[nj][ni]` by 20:58.
- 21:09-21:49: food branch first duplicated both next[] lines, then reduced to `next[j][i]++;` with the move in `} else { ... }`.

## Shell & outputs
- [21:53] `lab $ cc` (typing; rest not seen).
- [22:13] `lab $ ` empty prompt after compile (no errors or warnings visible).
- [22:38-22:48] `./a.out` running (tab title `a.out`, spinner). Grid 50x30, `%2d`/`%2c` so each cell is 2 chars wide; only the left ~25-28 columns are populated. Initial frame: `F` markers at column ~4 (rows 1, 2, 3 and row ~9) with small numbers next to them; a big pile near row 9 col ~10 and a diffuse cloud of mostly `1`s around it. Over the 11 frames (about 0.3 s apart in video terms, seen at 22:38-22:48):
  - 22:38 pile reads `145` (row 9); `F 4` beside the first F; ~50 scattered `1`s, one `2`.
  - 22:39 pile `145`; `F 5`; a `3` and a few `2`s.
  - 22:40 pile `43` (really shrinking/merging with neighbor digits); `F 1`, `F 2`.
  - 22:41 pile `141` (looks like `14`+`1`); `F 2`, `F 2`; `2`s around.
  - 22:42 pile `137`; `1 3 1 1`, `2 1` clusters near it.
  - 22:43 pile `34` (probably `3`,`4` or neighbor overlap); `F 2`, `F 1`, `F 2`.
  - 22:44 `1 133 1`; `F 2 1 1`; a `3` lower left.
  - 22:45 `134`; `F 2 2`; `3 2 1` right of pile.
  - 22:46 `133`; `2 F 1 1`, `1 F 1 1`; a `3`.
  - 22:47 `35` with `4` above; `F 3`; `1 5 F`; cloud a bit wider (to ~col 25).
  - 22:48 `33`; `F 3 1`; `4 F`; `2 F 1 2`; cloud widest, ~60+ `1`s.
  Overall behavior: the pile (starting 90) emits one `1` per step to random neighbors; the `1`s random-walk outward in all 8 directions, the cloud expands, numbers occasionally clump into 2-5. Pile reading is blurred because adjacent cells print as adjacent 2-char fields (e.g. `1 3 3`). `F` cells stay put and the cells next to them accumulate small numbers (2-5), consistent with "eat food: source cell increments". The F count stays at 4 (rows 1,2,3 + row 9) throughout.

## Observations
- Build order: finish init() food values; refine render() so positive numbers print `%2d` and negative (food) prints `F`; write update() (random move from `moves[rand() % 8]`, bounds check, write into `next[]`), memcpy back to `state`; clear_term with `usleep(300000)` + `system("clear")`; main with `srand(time(NULL))`; add the food rule; save, compile in zsh tab, run.
- Why `next[]`: updates are read from `state` and written to `next` so a unit that moves is not processed twice in the same pass; init copies state into next, update copies next back to state.
- Rules: each nonzero cell picks one random neighbor. Positive = units, negative = food. If the target cell is food (`state[nj][ni] < 0`) the source cell is incremented (`next[j][i]++`) instead of moving (the unit stays, count grows by one); otherwise one unit moves: `next[j][i]--; next[nj][ni]++`. Food cells themselves are also nonzero and take part in the loop; the food rule as written never changes food cells to be consumed (not analyzed in this segment).
- Style: 2-space indent, `size_t` loop indices, `const int *move`, lowercase comments (`// < 0 is food`, `// if meet food`), vim visual-line + `>`/put used to duplicate and re-indent; typing `{` auto-closes `}`.
- Workflow: `:w` in vim, switch to zsh tab, compile with `cc ...` (command not fully seen), run `./a.out` (tab title `a.out`).
- No man pages or browsers consulted; no shell `#` comments.
