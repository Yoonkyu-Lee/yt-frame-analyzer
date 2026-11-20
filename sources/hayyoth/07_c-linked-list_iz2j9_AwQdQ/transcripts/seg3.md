# Segment 3 (06:16 - 10:14), 31 frames, all viewed

Layout: LEFT Terminal, tabs `~/lab — vim main.c` / `~/lab — -zsh`. RIGHT-TOP Terminal (Chez Scheme). No sticky notes and no man pages appear in this segment.

## Timeline
- [06:16] LEFT tab `~/lab — vim main.c`, normal mode, scrolled to top = line 3. Lines 3-37 visible; file ends `return 0;` at line 37 (so `}` is line 38 offscreen). Cursor on `)` after `Node` at line 10 (bracket-match highlight of `sizeof(Node)`). RIGHT Chez REPL scrollback:
  `lab $ # lisp list` / `lab $ chez` / `Chez Scheme Version 10.4.1` / `Copyright 1984-2026 Cisco Systems, Inc.` / `> (define my-list '(10 20 30))` / `> my-list` -> `(10 20 30)` / `> (append my-list '(40 50))` -> `(10 20 30 40 50)` / `> ` prompt.
- [06:17] Scrolled to top: line 1 `#include <stdio.h>`, 2 `#include <stdlib.h>`. Shows lines 1-35. No edits.
- [06:43] RIGHT terminal cleared (only `> ` prompt with solid block cursor). LEFT unchanged (lines 1-35). Left window title dimmed (focus on right).
- [07:10] RIGHT after clear: `> my-list` -> `(10 20 30)`; `> (map (lambda (x) (* x 2)) my-list)` -> `(20 40 60)`; mid-typing `> (map (lambda ` (cursor). LEFT unchanged (lines 1-35).
- [07:30] RIGHT: `> (map (lambda (x) (* x 10)) my-list)` -> `(100 200 300)`, then `> ` prompt. LEFT focused again; vim top = line 4; file visible to end: line 37 `  return 0;`, line 38 `}`. Cursor (teal highlight) on `{` of `int main(void) {` line 29 (matching brace highlight).
- [07:36] LEFT: top = line 5. New lines in main: 37 `  // add free`, 38 blank, 39 `  return 0;` (cursor on `;`).
- [07:37] Top = line 6. Now 37 `  // add free`, 38 `  return 0;` (cursor on `r`), 39 `}`, then `~`. (Blank between comment and return removed; file 39 lines.)
- [07:39] Top = line 2; cursor in `typedef` line 4 col 3 (normal). main body ends line 35 `  printList(head);`, line 36 blank visible (file scrolled; lines 36+ below). 
- [07:42] `-- INSERT --`. Extra blank lines inserted between printList() and main(): 28 blank, 29 empty (cursor), 30 blank, `int main(void) {` now at line 31.
- [07:44] `-- VISUAL LINE --` (count `2`) selecting lines 23-24 (`void printList(Node *p) {` and `  for (; p; p = p->next)`), main still at 31. Reusing them as the start of map().
- [07:46] Normal mode; cursor block on blank line 28; lines 28, 29, 30 blank; main at 31. Selection gone.
- [08:14] `-- INSERT --`. Line 29 `void map(Node *p, int (*f)(int)) {`, line 30 `  ` (cursor, indent 2), 31 blank, 32 `int main(void) {`.
- [08:15] `-- INSERT --`. 29 `void map(Node *p, int (*f)(int)) {` (teal-highlighted `{`), 30 blank, 31 `}` (teal highlight), 32 blank, 33 `int main(void) {`.
- [08:19] Normal. 30 blank (block cursor), 31 `  for (; p; p = p->next)` (pasted from printList), 32 `}`, 33 blank, 34 `int main(void) {`.
- [08:20] Blank line deleted: 30 `  for (; p; p = p->next)` (cursor on `f`), 31 `}`, 32 blank, 33 `int main(void) {`. Normal.
- [08:24] `-- INSERT --`. 30 `  for (; p; p = p->next)`, 31 `    ` (cursor, 4 spaces indent), 32 `}`, main at 34.
- [08:45] Normal, top = line 10. map complete: 29 `void map(Node *p, int (*f)(int)) {`, 30 `  for (; p; p = p->next)`, 31 `    p->val = f(p->val);`, 32 `}`, 33 blank, 34 `int main(void) {`; 40 `  printList(head);` (cursor on `;`), 42 `  // add free`, 43 `  return 0;`, 44 `}`. No save message seen.
- [08:47] `-- INSERT --`. After map: 33 blank, 34 empty (cursor), 35 blank, 36 `int main(void) {`; `  // add free` at 44.
- [08:57] Normal. Line 34 `int x2(int x) { return x * 2;}`, line 35 duplicate `int x2(int x) { return x * 2;}` (cursor on `i`), 36 blank, 37 `int main(void) {`. (yy/p duplicate, about to edit.)
- [09:06] Line 35 now `int x10(int x) { return x * 10;}`. Top = 13. Line 45 `  // add free` (cursor after `free`), 46 `  return 0;`, 47 `}`.
- [09:16] Top = 16. main calls added: 43 `  printList(head);`, 44 blank, 45 `  map(head, x2);`, 46 `  printList(head);`, 47 blank, 48 `  // add free` (cursor on first `/`), 49 `  return 0;`, 50 `}`. Normal.
- [09:17] `-- VISUAL LINE --` (2) on lines 45-46 (`  map(head, x2);`, `  printList(head);`).
- [09:18] Normal, cursor line 46 col 1; selection gone (yank done, no message visible).
- [09:26] Tab switched to `~/lab — -zsh`. Shell: `lab $ cc main.c` / `lab $ ./a.out` -> `10 20 30` / `lab $ ` (cursor).
- [09:34] zsh tab: second round: `lab $ cc main.c` / `lab $ ./a.out` -> `10 20 30` / `20 40 60` / `200 400 600` / `lab $ `. The block from `lab $ ./a.out` through `200 400 600` is mouse-highlighted (gray). I-beam mouse at ~(198,288). (Webcam: smoking; ignore.)
- [10:01] RIGHT focused; REPL additions: `> (apply + my-list)` -> `60`; `> (apply - my-list)` -> `-40`; `> ` block cursor. LEFT zsh tab still shows highlighted output block.
- [10:06] LEFT back to tab `~/lab — vim main.c`, top = 16. Save message `"main.c" 53L, 801B written`. Lines 43-49: 43 `  printList(head);`, 44 blank, 45 `  map(head, x2);`, 46 `  printList(head);`, 47 blank, 48 `  map(head, x10);`, 49 `  printList(head);` (cursor on `a` of head), 50 blank. Mouse arrow over tab bar (tab shows hover `x`).
- [10:10] `-- VISUAL LINE --` (count `5`) selecting lines 29-33 (map function plus trailing blank line).
- [10:11] `4 lines yanked`. Cursor line 30 col 1, normal. Content unchanged.
- [10:13] `4 more lines` (after `p`). Copy of map pasted at 37-40, directly before main: 37 `void map(Node *p, int (*f)(int)) {` (cursor on `v`), 38 `  for (; p; p = p->next)`, 39 `    p->val = f(p->val);`, 40 `}`, 41 `int main(void) {` (no blank separator). main shifted +4 (create at 42, append 44/45, printList 47, map(head, x2) 49, printList 50).
- [10:14] `-- INSERT --`. New empty line 41 (cursor) between `}` (40) and `int main(void) {` (42). He is about to rewrite the pasted copy (likely an apply/fold analogue to `(apply + my-list)`; content unseen).

## File states

### main.c at 10:14 (INSERT mode; unsaved edits since last save at 10:06). Lines 1-15 taken from the 06:17 frame (unchanged since).
```
 1| #include <stdio.h>
 2| #include <stdlib.h>
 3|
 4| typedef struct Node {
 5|   int val;
 6|   struct Node *next;
 7| } Node;
 8|
 9| Node *create(int val) {
10|   Node *p = malloc(sizeof(Node));
11|   if (!p) exit(1);
12|
13|   p->val = val;
14|   p->next = NULL;
15|   return p;
16| }
17|
18| void append(Node *p, int val) {
19|   while (p->next) p = p->next; // go to last node
20|   p->next = create(val);
21| }
22|
23| void printList(Node *p) {
24|   for (; p; p = p->next)
25|     printf("%d ", p->val);
26|   printf("\n");
27| }
28|
29| void map(Node *p, int (*f)(int)) {
30|   for (; p; p = p->next)
31|     p->val = f(p->val);
32| }
33|
34| int x2(int x) { return x * 2;}
35| int x10(int x) { return x * 10;}
36|
37| void map(Node *p, int (*f)(int)) {      <- pasted duplicate (10:13), to be rewritten
38|   for (; p; p = p->next)
39|     p->val = f(p->val);
40| }
41|                                          <- new empty line (cursor, INSERT) at 10:14
42| int main(void) {
43|   Node *head = create(10);
44|
45|   append(head, 20);
46|   append(head, 30);
47|
48|   printList(head);
49|
50|   map(head, x2);
51| ??   printList(head);
52| ??
53| ??   map(head, x10);
54| ??   printList(head);
55| ??
56| ??   // add free
57| ??   return 0;
58| ?? }
```
Lines 51-58 are `??` (inferred by +4 shift from the 10:06 frame, not directly viewed at 10:14).

### main.c as saved at 10:06 (`"main.c" 53L, 801B written`); lines 16-50 seen directly
```
16| }
17|
18| void append(Node *p, int val) {
19|   while (p->next) p = p->next; // go to last node
20|   p->next = create(val);
21| }
22|
23| void printList(Node *p) {
24|   for (; p; p = p->next)
25|     printf("%d ", p->val);
26|   printf("\n");
27| }
28|
29| void map(Node *p, int (*f)(int)) {
30|   for (; p; p = p->next)
31|     p->val = f(p->val);
32| }
33|
34| int x2(int x) { return x * 2;}
35| int x10(int x) { return x * 10;}
36|
37| int main(void) {
38|   Node *head = create(10);
39|
40|   append(head, 20);
41|   append(head, 30);
42|
43|   printList(head);
44|
45|   map(head, x2);
46|   printList(head);
47|
48|   map(head, x10);
49|   printList(head);
50|
51| ??   // add free
52| ??   return 0;
53| ?? }
```
(51-53 inferred from line count 53L.)

### Intermediate versions
- 06:16/07:30: main = create(10); append 20; append 30; blank; printList(head); blank; return 0; } (38 lines).
- 07:36-07:37: `// add free` inserted directly above `return 0;` (39 lines).
- 09:06: x2 and x10 defined; no map calls yet; `// add free` at 45, `return 0;` 46, `}` 47.
- 09:16: `map(head, x2); printList(head);` added at 45-46 (50 lines).

### Chez REPL transcript (RIGHT-TOP)
```
lab $ # lisp list
lab $ chez
Chez Scheme Version 10.4.1
Copyright 1984-2026 Cisco Systems, Inc.

> (define my-list '(10 20 30))
> my-list
(10 20 30)
> (append my-list '(40 50))
(10 20 30 40 50)
> [screen cleared ~06:43]
> my-list
(10 20 30)
> (map (lambda (x) (* x 2)) my-list)
(20 40 60)
> (map (lambda (x) (* x 10)) my-list)
(100 200 300)
> (apply + my-list)
60
> (apply - my-list)
-40
>
```
Scheme `append` and `map` are non-destructive: `my-list` stays `(10 20 30)`.

## Shell & outputs
- [09:26] `lab $ cc main.c` (no output) ; `lab $ ./a.out` -> `10 20 30`
- [09:34] `lab $ cc main.c` ; `lab $ ./a.out` ->
  ```
  10 20 30
  20 40 60
  200 400 600
  ```
- No compiler warnings/errors seen. Prompt is `lab $ `. Compile: `cc main.c` -> `a.out`.
- vim messages: `"main.c" 53L, 801B written` (10:06); `4 lines yanked` (10:11); `4 more lines` (10:13).

## Observations
- Build order: Lisp demo (map with lambda x2, x10) -> `// add free` TODO comment in main -> C `map()` with function-pointer param `int (*f)(int)` -> `x2`, `x10` -> `map(head, x2)`/`map(head, x10)` calls -> compile/run -> REPL `(apply + ...)` / `(apply - ...)` -> begins a copy of map as template for a new function (unseen).
- map design: `void map(Node *p, int (*f)(int))` mutates in place (`p->val = f(p->val)`), unlike Scheme map returning a new list. Output `20 40 60` then `200 400 600` shows x10 applied to the already doubled list.
- Style: 2-space indent, braceless `for`/`while` with single statement (for body indented 4 via copy of printList's), one-line functions `int x2(int x) { return x * 2;}` (no space before `}`), inline comment `// go to last node`, TODO-ish `// add free` (free not implemented; leak left on purpose so far). `int main(void)`.
- Workflow: heavy vim reuse: visual-line yank of printList header/for to start map; yy-duplicate of x2 line edited into x10; duplicated `map(head, x2); printList(head);` pair edited into x10; map function itself duplicated at 10:13. Two Terminal tabs (vim / zsh), manual `cc main.c` then `./a.out`.
- Lisp-vs-C: lambda inline vs named C functions + function pointers; `(append my-list '(40 50))` vs C `append(head, 20)`; `(apply + my-list)` -> 60 and `(apply - my-list)` -> -40 (10-20-30) foreshadow a fold/apply function in C.
- No sticky notes, man pages, or REPL `;` comments in this segment (`# lisp list` is a shell comment visible only in scrollback at 06:16).
- Uncertainties: no frames between 09:18 and 09:26, so when the second map block was typed/saved is not seen; the first run at 09:26 prints only `10 20 30`, so that compile reflects a file state before map calls were in main (or map calls absent). Lines 51-58 at 10:14 not viewed.
