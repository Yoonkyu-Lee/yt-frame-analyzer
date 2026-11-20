# Segment 2 transcript (03:12 - 06:16), 31 frames viewed

Layout constant: LEFT Terminal with tabs `~/lab — vim main.c` and `~/lab — -zsh`; RIGHT-TOP Chez Scheme terminal (static the whole segment); no sticky notes in this segment; no man pages.

## Timeline
- [03:12] LEFT tab `~/lab — -zsh` active. Ran `cc main.c`. Output (clang warnings on printList's for loop, line 9):
  ```
  lab $ cc main.c
  main.c:9:10: warning: variable 'p' used in loop
        condition not modified in loop body [-Wfor-loop-analysis]
      9 |   for (; p; p->next)
        |             ^
  main.c:9:16: warning: expression result unused
        [-Wunused-value]
      9 |   for (; p; p->next)
        |             ~  ^~~~
  2 warnings generated.
  lab $
  ```
  (bug: for-increment was `p->next` instead of `p = p->next`.) RIGHT terminal (static all segment): see REPL transcript below.
- [03:14] zsh tab: cleared; `lab $ ./a.out ` typed (not yet entered; no output visible).
- [03:18] Vim tab. Normal mode. Message `"main.c" 25L, 337B written`. Line 9 now `for (; p; p = p->next)` (fixed). File = v1 below.
- [03:20] INSERT mode; blank line opened: 7 blank, 8 blank (cursor), 9 blank; printList now at line 10; 27L.
- [03:27] Normal mode (block cursor on line 8); 26 lines (7,8 blank, printList at 9). (Insert then Esc, one blank apparently removed.)
- [03:28] INSERT, line 8 has `i` typed (start of `int`/identifier; ends up `Node *create...`); 27 lines, printList at 10.
- [03:39] INSERT. Line 8 `Node *create(int val) {`, line 9 `  ` (cursor), 10 blank, printList at 11; 28 lines. (Note line 8 has no leading indent; column width suggests `Node` starts one cell left of other lines' text: a slight offset only because of rendering.)
- [03:42] INSERT. Cursor on new line 2 typing `#i`; create at line 9, printList at 12; 29 lines.
- [04:02] INSERT. Line 2 `#include <stdlib.h>`. `10|  Node *p = malloc(sizeof Node);` line 11 cursor. 30 lines.
- [04:08] INSERT. Line 11 `  if (!p) exit(1);`, 12 blank, 13 cursor. 32 lines.
- [04:20-ish, actually 04:12] INSERT. 13 `  p->val = val;`, 14 `  p->` (typing). 33 lines.
- [04:20] INSERT. 14 `  p->next = NULL;`, 15 `  retu` (typing). 34 lines.
- [04:22] INSERT. 15 `  return p;`, 16 `}` (bracket-match highlight on `{` of line 9 and `}` line 16). 35 lines.
- [04:27] INSERT. Line 18 `vo` typed (new function above printList, which is at 20).
- [04:38] INSERT. 18 `void append(Node *p, int val) {`, 19 `  ` cursor, 20 blank, printList 21.
- [04:39] Normal mode. 19 empty (block cursor), 20 `}` closing brace now present, 21 blank, printList 22.
- [05:06] INSERT. Line 19 `  while (p->next) p = p->next; // go to last node`, line 20 auto-continued comment `// ` (cursor after it), 21 `}`.
- [05:18] Normal, view scrolled (top = line 6). Line 19 comment unchanged; line 20 `  p->next = create(val);`; 21 `}`. Cursor on `;` of line 32 `Node n3 = {.val=30};`. 40 lines.
- [05:20] `-- VISUAL LINE --`, count `1`: line 31 `Node n2 = {.val=20};` highlighted.
- [05:21] VISUAL LINE, count `4`: lines 31-34 highlighted (n2, n3, blank, `n1.next = &n2;`).
- [05:22] Normal; message `5 fewer lines`. main now: 30 `  Node n1 = {.val=10};`, 31 blank (cursor), 32 `  printList(&n1);`, 34 `  return 0;`, 35 `}`. (lines n2, n3, blank, n1.next, n2.next deleted.)
- [05:35] INSERT. 30 `  Node *head = create(10);`, 31 cursor, 32 blank, 33 `  printList(&n1);`. 36 lines.
- [05:47] Normal. 32 `  append(head, 20);` (cursor on `;`), 34 `  printList(&n1);`. 37 lines.
- [05:49] Normal. Line duplicated: 32 and 33 both `  append(head, 20);` (cursor on the space before `20` on line 33). 38 lines.
- [05:57] zsh tab: cleared, empty prompt `lab $ `.
- [05:59] zsh: `cc main.c` ->
  ```
  lab $ cc main.c
  main.c:10:26: error: expected parentheses around type
        name in sizeof expression
     10 |   Node *p = malloc(sizeof Node);
        |                           ^
        |                           (    )
  1 error generated.
  lab $
  ```
- [06:01] Vim tab, normal. `"main.c" 38L, 558B written`. main: 32 `append(head, 20);`, 33 `append(head, 30);`, 35 `printList(head);` (cursor on `d` of `head`). (edits `20`->`30` and `&n1`->`head` happened between 05:49 and 06:01, unseen.)
- [06:09] zsh tab: same error output as 05:59 still displayed.
- [06:10] zsh: screen cleared and `cc main.c` rerun -> `lab $ cc main.c` / `lab $ ` : compiles cleanly (no output).
- [06:15] Vim tab: `"main.c" 38L, 559B written`. Line 10 now `  Node *p = malloc(sizeof(Node));` (cursor on `)`, matching `(` highlighted). Mouse I-beam visible.
- [06:16] Vim scrolled (top = line 3). Same content. End of segment.

## File states
### v1 (03:18, `"main.c" 25L, 337B written`)
```
 1| #include <stdio.h>
 2|
 3| typedef struct Node {
 4|   int val;
 5|   struct Node *next;
 6| } Node;
 7|
 8| void printList(Node *p) {
 9|   for (; p; p = p->next)
10|     printf("%d ", p->val);
11|   printf("\n");
12| }
13|
14| int main(void) {
15|   Node n1 = {.val=10};
16|   Node n2 = {.val=20};
17|   Node n3 = {.val=30};
18|
19|   n1.next = &n2;
20|   n2.next = &n3;
21|
22|   printList(&n1);
23|
24|   return 0;
25| }
```

### Final (06:15, `"main.c" 38L, 559B written`)
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
29| int main(void) {
30|   Node *head = create(10);
31|
32|   append(head, 20);
33|   append(head, 30);
34|
35|   printList(head);
36|
37|   return 0;
38| }
```
Line 38 `}` is inferred from the 38L count (not visible in the 06:16 frame, which is scrolled; it was visible at 06:01).

Intermediate notes:
- 06:01 version (558B): line 10 was `  Node *p = malloc(sizeof Node);` (error). 06:15 (559B) has parens. Byte delta is +1 although two parens were added, so some other 1-byte change in between is possible but unseen (uncertain).
- 04:39: append had empty body, `}` on line 20. 05:06: comment-continuation line `// ` on line 20, later removed.
- 05:35: main still had `printList(&n1);` (stale), changed to `printList(head);` by 06:01.

### REPL transcript (RIGHT terminal, unchanged throughout segment)
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
>
```

## Shell & outputs
- [03:12] `cc main.c` -> 2 warnings: `main.c:9:10: warning: variable 'p' used in loop condition not modified in loop body [-Wfor-loop-analysis]`, `main.c:9:16: warning: expression result unused [-Wunused-value]`; `2 warnings generated.`
- [03:14] `./a.out ` typed, output not captured in sampled frames.
- [05:59] `cc main.c` -> `main.c:10:26: error: expected parentheses around type name in sizeof expression`; `1 error generated.`
- [06:10] `cc main.c` -> clean (no output). `a.out` not run in this segment.
- Vim saves: `"main.c" 25L, 337B written` (03:18); `"main.c" 38L, 558B written` (06:01); `"main.c" 38L, 559B written` (06:15). Other message: `5 fewer lines` (05:22).

## Observations
- Build order: Node struct + printList + manual n1/n2/n3 linking (from seg1) -> clang warns for-loop bug, fixed -> `create()` (malloc, null check `if (!p) exit(1);`, set val, next=NULL, return p) with `#include <stdlib.h>` -> `append()` (walk to last node, `p->next = create(val)`) -> main rewritten: manual stack nodes deleted, replaced with `create(10)` + `append(head, 20/30)` -> compile error for `sizeof Node` fixed to `sizeof(Node)`.
- Design: heap nodes via create(); append mutates and requires non-NULL head (no empty-list handling visible); error handling is terse exit(1).
- Style: 2-space indent, K&R braces, `Node *create`, one-line while without braces, the only comment is `// go to last node`; vim auto-continued the `//` comment and he removed it. No `#`/`;` comments, no sticky notes here.
- Lisp-vs-C: right terminal holds `(append my-list '(40 50))` -> `(10 20 30 40 50)` as the Lisp counterpart of C append; static during this segment (no new REPL input). Video is silent so narration unavailable.
- Vim workflow: uses `o`/insert, visual-line + delete (`5 fewer lines`), yank/paste line duplication (`append(head, 20);` twice, then edit).
