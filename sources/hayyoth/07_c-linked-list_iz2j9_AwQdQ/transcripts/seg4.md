# Segment 4 transcript (10:14 - 14:23), iz2j9_AwQdQ

Tabs (left terminal): `~/lab — vim main.c` and `~/lab — -zsh`. Right-top terminal = Chez REPL (prompt `> `). Webcam ignored.

## Timeline
- [10:14] vim main.c, INSERT. Lines 16-50 visible. Right REPL already shows an earlier session: `my-list` -> `(10 20 30)`; `(map (lambda (x) (* x 2)) my-list)` -> `(20 40 60)`; `(map (lambda (x) (* x 10)) my-list)` -> `(100 200 300)`; `(apply + my-list)` -> `60`; `(apply - my-list)` -> `-40`. Cursor at empty line 41. A DUPLICATE `void map(Node *p, int (*f)(int)) {` block (lines 37-40) has just been copied after x10 (37 `void map(...) {`, 38 `for (; p; p = p->next)`, 39 `p->val = f(p->val);`, 40 `}`); main now starts at line 42.
- [10:28] Duplicate being turned into apply: line 37 `int apply(Node *p, int (*f)(int, int)) {`, line 38 empty (cursor, INSERT), lines 39-41 still old for / `p->val = f(p->val);` / `}`. main at 43.
- [10:44] Normal mode (matching-brace highlight on `{` line 37 / `}` line 40). Body: 38 `  int result = p->val;`, 39 `  for (p = p->next; p; p = p->next)`, 40 `}`.
- [10:45] INSERT. Line 40 empty indented (cursor), `}` at 41; main at 43.
- [11:01] INSERT. Line 40 `    result = f(result, p->val);`, 41 empty, 42 cursor (about to type return), 43 `}`; main at 45.
- [11:05] Normal mode, cursor on `{` of main (line 45). apply complete: 42 `  return result;`, 43 `}`. main lines 45-61 in full (see File states, intermediate 2): map/printList pairs, `// add free`, `return 0;`.
- [11:07] `-- VISUAL LINE --` (count 4): lines 53-55 selected.
- [11:08] VISUAL LINE count 5: lines 53-57 selected.
- [11:09] Normal mode, status `5 fewer lines` (deleted 53-57). File 56 lines: 51 `printList(head);`, 52 (cursor, blank), 53 blank, 54 `// add free`, 55 `return 0;`, 56 `}`. Vim `~` filler visible.
- [11:18] INSERT. Blank lines 45-46 inserted between apply `}` (43) and main (47); cursor on 45. File 58 lines. Lines 54, 55 both blank.
- [11:45] INSERT. Line 45 `int add(int x, int y) { return x + y; }` written; main at 47. Line 55 being typed: `  printf("sum = %d\n", a` (cursor after `a`). 56 blank, 57 `// add free`, 58 `return 0;`, 59 `}`.
- [11:54] Switched to tab `~/lab — -zsh` (vim tab dimmed). Shell (scrollback of an older run):
  ```
  lab $ cc main.c
  lab $ ./a.out
  10 20 30
  lab $ cc main.c
  lab $ ./a.out
  10 20 30
  20 40 60
  200 400 600
  lab $ 
  ```
  (last run's lines highlighted/selected.)
- [11:55] zsh cleared; `lab $ c` being typed.
- [12:03] Back to vim tab (mouse over tab title). Normal mode. Status: `"main.c" 59L, 963B written`. Line 55 `  printf("sum = %d\n", apply(head, add));` (cursor on trailing `;`). 59 lines.
- [12:08] Line 45 duplicated: lines 45 and 46 both `int add(int x, int y) { return x + y; }` (cursor on `i` of line 46). main at 48; printf at 56; 60 lines.
- [12:22] zsh tab:
  ```
  lab $ cc main.c
  lab $ ./a.out
  10 20 30
  sum = 60
  lab $ 
  ```
- [12:33] vim tab: line 46 `int sub(int x, int y) { return x - y; }`; line 56 `  printf("sub = %d\n", apply(head, sub));` (cursor on `b` of `sub`). Status: `"main.c" 60L, 1003B written`. Mouse over right REPL window.
- [12:36-12:38] Right REPL window dragged (title-bar drag) to a new position (~885,85 then); vim unchanged, same save message. At 12:38 vim scrolled to show lines 30-60; tab title shows `...` marker at right of tab.
- [12:39] vim scrolled to top: lines 1-35 shown (full header; see File states). Matching-brace highlight on x10 braces (cursor line 35).
- [12:40] scrolled, lines 28-60 visible.
- [12:41] scrolled, lines 34-60 visible (tildes after 60).
- [12:42-12:43] REPL window dragged left/up to ~(285,48), covering part of vim; then ~(270,46).
- [12:44] REPL window resized larger; mid-clear: `>` at top, blank gap, old history still at lower lines.
- [12:46] REPL fully cleared: just `> ` with cursor.
- [13:19] REPL fresh session:
  ```
  > (define my-list '(10 1.5 "Hello"))
  > my-list
  (10 1.5 "Hello")
  > (cons my-list)
  Exception: incorrect argument count in call (cons my-list)
  Type (debug) to enter the debugger.
  > 
  ```
- [13:21] REPL cleared again, `> ` only.
- [13:35] REPL:
  ```
  > (car my-list)
  10
  > (cdr my-list)
  (1.5 "Hello")
  > (cdr (cdr my-list))
  ("Hello")
  > 
  ```
- [14:08] REPL (continues, same screen):
  ```
  > (cons "hayy" my-list)
  ("hayy" 10 1.5 "Hello")
  > (define my-code '(+ 1 1 1 1))
  > my-code
  (+ 1 1 1 1)
  > 
  ```
- [14:23] REPL final frame (macOS menu bar visible: `Terminal Shell Edit View Window Help`, clock `Wed Oct 21 1:55 AM`):
  ```
  > (eval my-code)
  4
  > ; this very hard to implement in C 
  ```
  (cursor after trailing space; comment typed, not yet entered.)

## File states
Indentation: 2 spaces; for-bodies 4. Lines 1-35 first seen in full at [12:39]; 36-60 at [12:41]. At [12:39] the mouse cursor partly hides `val)` on line 18 (read as `int val) {`, consistent with earlier frames).

### Final main.c (as of 12:39-14:23; last vim status `"main.c" 60L, 1003B written`)
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
37| int apply(Node *p, int (*f)(int, int)) {
38|   int result = p->val;
39|   for (p = p->next; p; p = p->next)
40|     result = f(result, p->val);
41| 
42|   return result;
43| }
44| 
45| int add(int x, int y) { return x + y; }
46| int sub(int x, int y) { return x - y; }
47| 
48| int main(void) {
49|   Node *head = create(10);
50| 
51|   append(head, 20);
52|   append(head, 30);
53| 
54|   printList(head);
55| 
56|   printf("sub = %d\n", apply(head, sub));
57| 
58|   // add free
59|   return 0;
60| }
```
(Colouring: `//` comments cyan; `NULL`, numbers, format strings magenta.)

### Intermediate 1: [10:14] (duplicate map, before apply)
```
34| int x2(int x) { return x * 2;}
35| int x10(int x) { return x * 10;}
36| 
37| void map(Node *p, int (*f)(int)) {
38|   for (; p; p = p->next)
39|     p->val = f(p->val);
40| }
41| 
42| int main(void) {
43|   Node *head = create(10);
44| 
45|   append(head, 20);
46|   append(head, 30);
47| 
48|   printList(head);
49| 
50|   map(head, x2);
```

### Intermediate 2: [11:05] (apply done, main with map demo; 61 lines)
```
37| int apply(Node *p, int (*f)(int, int)) {
38|   int result = p->val;
39|   for (p = p->next; p; p = p->next)
40|     result = f(result, p->val);
41| 
42|   return result;
43| }
44| 
45| int main(void) {
46|   Node *head = create(10);
47| 
48|   append(head, 20);
49|   append(head, 30);
50| 
51|   printList(head);
52| 
53|   map(head, x2);
54|   printList(head);
55| 
56|   map(head, x10);
57|   printList(head);
58| 
59|   // add free
60|   return 0;
61| }
```
Then lines 53-57 deleted (`5 fewer lines`).

### Intermediate 3: [12:03] (add + sum version; 59 lines, `"main.c" 59L, 963B written`)
```
45| int add(int x, int y) { return x + y; }
46| 
47| int main(void) {
48|   Node *head = create(10);
49| 
50|   append(head, 20);
51|   append(head, 30);
52| 
53|   printList(head);
54| 
55|   printf("sum = %d\n", apply(head, add));
56| 
57|   // add free
58|   return 0;
59| }
```
(At 11:45 line 55 was mid-typing: `  printf("sum = %d\n", a`.)

### Intermediate 4: [12:08] line 45 duplicated (60 lines)
45 and 46 both `int add(int x, int y) { return x + y; }`; main at 48; line 56 `printf("sum = %d\n", apply(head, add));`. Then 46 edited to `sub` and the printf to `"sub = %d\n"` / `sub`, saved as `"main.c" 60L, 1003B written`.

### REPL transcript, final session after clear (right terminal)
```
> (define my-list '(10 1.5 "Hello"))
> my-list
(10 1.5 "Hello")
> (cons my-list)
Exception: incorrect argument count in call (cons my-list)
Type (debug) to enter the debugger.
> (car my-list)
10
> (cdr my-list)
(1.5 "Hello")
> (cdr (cdr my-list))
("Hello")
> (cons "hayy" my-list)
("hayy" 10 1.5 "Hello")
> (define my-code '(+ 1 1 1 1))
> my-code
(+ 1 1 1 1)
> (eval my-code)
4
> ; this very hard to implement in C 
```
(Between the two screens at 13:19 and 13:35 the REPL was cleared; the screen at 13:35 begins with `(car my-list)`.)

Earlier REPL session (visible 10:14-12:44, before clear):
```
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
```
(Scrollback above `my-list` not visible in this segment.)

## Shell & outputs
- [11:54] zsh scrollback: `lab $ cc main.c` / `lab $ ./a.out` -> `10 20 30`; `lab $ cc main.c` / `lab $ ./a.out` -> `10 20 30` / `20 40 60` / `200 400 600`. Prompt format `lab $ `; no warnings shown.
- [12:22] `lab $ cc main.c`, `lab $ ./a.out` -> `10 20 30` then `sum = 60`.
- No compile errors/warnings seen. The `sub` version was saved (12:33) but its compile/run output was NOT seen in this segment (expected `sub = -40`, matching Scheme `(apply - my-list)` -> -40; unverified).
- Vim messages (checksums): `5 fewer lines` (11:09); `"main.c" 59L, 963B written` (12:03); `"main.c" 60L, 1003B written` (12:33-12:38).

## Observations
- Design: `apply` is a left fold (reduce) over the list: `result` starts at the first node's val, loop begins at `p = p->next` (reuses/mutates parameter p), `result = f(result, p->val)`. Binary function pointer `int (*f)(int, int)`. Built by copying `map`, renaming and changing return type/signature. Mirrors Scheme `(apply + my-list)` = 60 and `(apply - my-list)` = -40 (10-20-30), which is what `sub` yields.
- `map` mutates in place (`p->val = f(p->val)`); Scheme `map` returns a new list.
- He deleted the map demo calls (x2, x10, printLists) from main to make room for the apply demos; the x2/x10/map definitions remain.
- Style: 2-space indent, braceless single-statement for bodies, one-line functions (`{ return x + y; }`), x2/x10 without trailing space before `}` (`return x * 2;}`). `// add free` TODO left in main; the list is never freed. `create` aborts via `if (!p) exit(1);`.
- Final REPL part contrasts Lisp: heterogeneous list `'(10 1.5 "Hello")` (C list is int-only); deliberate error `(cons my-list)` -> `Exception: incorrect argument count in call (cons my-list)`; car/cdr; `(cons "hayy" my-list)` -> `("hayy" 10 1.5 "Hello")`; code as data: `(define my-code '(+ 1 1 1 1))` then `(eval my-code)` -> `4`.
- Closing comment verbatim at 14:23: `; this very hard to implement in C ` (no "is"). Point: homoiconicity / eval (code-as-data) is what C cannot easily do.
- Window handling: REPL window repositioned and enlarged over vim ~12:36-12:44 and cleared twice (12:44-12:46, 13:21) to start the fresh heterogeneous-list demo. Last frame 14:23 shows macOS clock `Wed Oct 21 1:55 AM`.
