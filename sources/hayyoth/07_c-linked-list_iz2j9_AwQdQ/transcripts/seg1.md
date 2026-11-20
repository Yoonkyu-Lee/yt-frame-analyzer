# Segment 1 transcript (00:00 - 03:12)

Frames viewed: all 30 in seg1_frames.txt. Layout: LEFT terminal (tabs `~/lab — -zsh` x2, later tab1 becomes `~/lab — vim main.c`), RIGHT-TOP terminal (zsh then Chez Scheme), webcam bottom-right (ignored).

## Timeline
- [00:00] Left terminal tabs: `~/lab — -zsh` | `~/lab — -zsh`. Left shows `lab $ ls` (no output) then `lab $ `. Right terminal: `lab $ ` empty. Yellow sticky note: `this video help you:`
- [00:06] Sticky note typing: `this video help you:` / `- understand linked lis` (cursor mid-typing, "list" being typed)
- [00:14] Sticky note body erased (empty, cursor only).
- [00:15] Sticky gone. Right terminal focused, `lab $ `, I-beam mouse in right terminal.
- [00:20] Right terminal:
  ```
  lab $ # lisp list
  lab $ chez
  Chez Scheme Version 10.4.1
  Copyright 1984-2026 Cisco Systems, Inc.

  > 
  ```
- [00:40] Chez REPL:
  ```
  > (define my-list '(10 20 30))
  > my-list
  (10 20 30)
  > (append my-list '(40 50))
  (10 20 30 40 50)
  > 
  ```
- [00:43] Spotlight overlay open with `s` typed (results Screenshot, Script Editor, Screen Sharing, ...) - launching Stickies.
- [00:44] New blank yellow sticky (cursor only) over left terminal. REPL unchanged.
- [00:52] Sticky gone; REPL unchanged; left still `lab $ ls` / `lab $ `. (Second note's content not seen in sampled frames.)
- [00:56] Left tab1 now `~/lab — vim main.c`, tab2 `~/lab — -zsh`. Status line `"main.c" [New]`. Buffer (cursor block on blank line 4):
  ```
  1 #include <stdio.h>
  2
  3 int main(void) {
  4 
  5   return 0;
  6 }
  ```
- [01:11] INSERT mode. Struct in progress (9 lines total):
  ```
  3 typedef struct Node {
  4   int val;|
  5
  6 int main(void) {
  7
  8   return 0;
  9 }
  ```
- [01:30] INSERT mode. Struct complete, first node typed (13 lines):
  ```
  3 typedef struct Node {
  4   int val;
  5   struct Node *next;
  6 } Node;
  7
  8 int main(void) {
  9   Node n1 = {.val=10}|      (matching-brace highlight; no `;` yet)
  10
  11
  12   return 0;
  13 }
  ```
- [01:38] Normal mode (no mode text), 16 lines; typo on line 11 (cursor highlight on `3` of `30`):
  ```
  9   Node n1 = {.val=10};
  10  Node n2 = {.val=20};
  11  Node n1 = {.val=30};     <- typo "n1" (should be n3)
  12-14 blank
  15  return 0;
  16 }
  ```
- [01:55] (k_01m55s) vim tab; typo fixed to `Node n3 = {.val=30};`; added lines 13-14 `n1.next = &n2;` / `n2.next = &n3;` (cursor on `;` of line 14). Status: `"main.c" [New] 18L, 222B written` (CHECKSUM: first save).
- [01:56] (k_01m56s) tab2 `~/lab — -zsh` active showing scrollback from a LATER/finished version of the program (see Shell & outputs): compile errors at main.c:38 and main.c:55, then successful run `10 20 30 / 100 200 300 / sum 600`; prompt `lab $ cc`. Appears to be an edit/overlay or preview of the final result (forward reference).
- [01:59] zsh tab2 cleared: `lab $ `.
- [02:00] Back to vim tab: same as 01:55, status `"main.c" [New] 18L, 222B written`.
- [02:06] INSERT. New line 8 containing `v` (typing `void ...`), line 9 blank, `int main(void) {` now line 10; 20 lines total.
- [02:13] INSERT. `8 void printList(Node *p) {`, line 9 blank (cursor, auto-indented), line 10 blank, `int main(void) {` line 11; 21 lines.
- [02:30] INSERT. 22 lines:
  ```
  8  void printList(Node *p) {
  9    for (; p; p->next)
  10     pr|
  ```
- [02:32] `10     printf("|` (below lines syntax-highlighted all magenta due to unclosed string)
- [02:33] `10     printf("%d |` (red bracket highlighting on main's `{` and node braces: unmatched-brace indicator)
- [02:40] 23 lines:
  ```
  10     printf("%d ", p->val);
  11   |
  ```
- [02:42] `11   printf("\|`
- [02:43] `11   printf("\n")` (matching-paren highlight, no `;` yet)
- [02:44] `11   printf("\n");`, line 12 new empty line with cursor, line 13 blank, `int main(void) {` at 14; 24 lines. Closing `}` of printList not yet typed.
- [03:01] tab2 zsh active: `lab $ cc main.c` typed (not yet run).
- [03:02] After running: two warnings (see Shell & outputs).
- [03:06] vim tab, normal mode: printList closed and `printList(&n1);` added. Status: `"main.c" 25L, 333B written` (CHECKSUM).
- [03:12] zsh tab2 again, same output as 03:02 (the two warnings; no `./a.out` seen).

## File states

### main.c at 00:56 (vim, `"main.c" [New]`)
```
1| #include <stdio.h>
2|
3| int main(void) {
4|
5|   return 0;
6| }
```

### main.c at 01:55 / 02:00 (saved: `"main.c" [New] 18L, 222B written`)
```
1| #include <stdio.h>
2|
3| typedef struct Node {
4|   int val;
5|   struct Node *next;
6| } Node;
7|
8| int main(void) {
9|   Node n1 = {.val=10};
10|  Node n2 = {.val=20};
11|  Node n3 = {.val=30};
12|
13|  n1.next = &n2;
14|  n2.next = &n3;
15|
16|
17|   return 0;
18| }
```
Indent: 2 spaces. (Rendering shows `Node n1` lines ~2px left of `n1.next`/`return`; likely font/syntax-highlight offset, so treat as same 2-space indent; ⟨?⟩ not certain.)

### main.c at END of segment (03:06; saved `"main.c" 25L, 333B written`)
```
1| #include <stdio.h>
2|
3| typedef struct Node {
4|   int val;
5|   struct Node *next;
6| } Node;
7|
8| void printList(Node *p) {
9|   for (; p; p->next)
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
(Line 21 blank with cursor block in the 03:06 frame.) The `for` increment is `p->next` (bug; should be `p = p->next`); compiler warnings confirm; not fixed within this segment, so program as written would loop forever. Note the `printf("\n");` is outside the for (dedented to 2) - the for body is the single un-braced `printf("%d ", p->val);` at indent 4.

### Intermediate: 02:44 (INSERT, 24 lines)
```
8|  void printList(Node *p) {
9|    for (; p; p->next)
10|     printf("%d ", p->val);
11|   printf("\n");
12|   |
13|
14| int main(void) {
15|   Node n1 = {.val=10};  ... (nodes 15-17, links 19-20, blanks 21-22, return 23, } 24)
```

### Chez REPL transcript (as of 00:40; unchanged to 03:12)
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
- [00:00] Left: `lab $ ls` -> (no output).
- [00:20] Right: `lab $ # lisp list`, `lab $ chez` -> `Chez Scheme Version 10.4.1` / `Copyright 1984-2026 Cisco Systems, Inc.` then prompt `> `.
- [01:56] Left tab2 scrollback (verbatim; from a later state of the program):
```
lab $ cc main.c
main.c:38:27: error: no member named 'data' in
      'struct Node'
   38 |     result = f(result, p->data);
      |                        ~  ^
main.c:55:22: error: argument type 'void' is incomplete
   55 |   printf("sum %d\n", apply(n1, add));
      |                      ^
2 errors generated.
lab $ cc main.c
main.c:55:22: error: argument type 'void' is incomplete
   55 |   printf("sum %d\n", apply(n1, add));
      |                      ^
1 error generated.
lab $ cc main.c
lab $ ./a.out
10 20 30
100 200 300
sum 600
lab $ cc
```
  (Forward evidence: later main.c has `result = f(result, p->data);` at line 38 (field named `val` in struct, so `data` is an error) and `printf("sum %d\n", apply(n1, add));` at line 55. `apply` initially void-typed -> error; final run prints `10 20 30`, `100 200 300` (map x10), `sum 600`.)
- [01:59] tab2 cleared: `lab $ `.
- [03:01] `lab $ cc main.c` typed.
- [03:02] and [03:12] (identical):
```
lab $ cc main.c
main.c:9:10: warning: variable 'p' used in loop
      condition not modified in loop body [-Wfor-loop-analysis]
    9 |   for (; p; p->next)
      |          ^
main.c:9:16: warning: expression result unused
      [-Wunused-value]
    9 |   for (; p; p->next)
      |           ~  ^~~~~
2 warnings generated.
lab $ 
```
  (Caret/tilde columns approximate; in frame: `^` under `p` in condition for warning 1; for warning 2 `~` under `p` and `^~~~` under `->next` portion.)

## Observations
- Opening sticky note: `this video help you:` then `- understand linked lis(t)` typed live, then erased. Goal signal: understand linked lists. A second blank sticky is opened via Spotlight (~00:43) but content not captured.
- Lisp first: shell comment `# lisp list`, `chez` (Chez Scheme 10.4.1), `(define my-list '(10 20 30))`, `my-list`, `(append my-list '(40 50))` -> `(10 20 30 40 50)`. Lisp list as the reference model before C.
- C build order: skeleton main -> `typedef struct Node { int val; struct Node *next; } Node;` -> three stack nodes with designated initializers `Node n1 = {.val=10};` -> manual linking `n1.next = &n2; n2.next = &n3;` -> save (`18L, 222B`) -> `void printList(Node *p)` placed above main with `for (; p; p->next)` and `printf("%d ", p->val);` then `printf("\n");` -> `printList(&n1);` call -> save (`25L, 333B`). No malloc/heap yet in this segment.
- Style: 2-space indent; unbraced `for` body indented 4; pointer star on name (`Node *p`, `struct Node *next`); typedef name same as tag; no comments in C; vim with line numbers and syntax highlighting; compiles with bare `cc main.c` (clang default warnings).
- Typo (`Node n1` twice) made and fixed live before first save.
- Real bug: `p->next` as for-increment (no assignment); clang warns twice. Not yet fixed here.
- 01:56 frame shows scrollback of a finished program (errors at lines 38 and 55; final output `10 20 30` / `100 200 300` / `sum 600`) - recording is edited/previewed non-linearly; forward reference to later segments (`apply(n1, add)`, map with x10, field accessed `p->data` mismatch with `val`).
- Lisp-vs-C: Lisp list literal `'(10 20 30)` and `append` are one-liners vs C hand-wiring Node structs and pointers.
