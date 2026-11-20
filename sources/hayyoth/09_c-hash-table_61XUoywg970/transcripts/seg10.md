# Segment 10 (27:33 - 31:19), 40 frames, all viewed

Window layout: LEFT Terminal, tabs `~/lab — vim test.c` and `~/lab — -zsh`. RIGHT window is either a Terminal (vim, hash table C file, title "Terminal") or a browser (YouTube) sitting behind it. Webcam "Human" bottom-right (ignored; he is smoking a cigarette in 27:51-28:12 frames). The yellow sticky is only ever a blank yellow strip at the right of the right-hand window (about y=520 in the frame, x 1092-1525); NO sticky text is legible in any frame of this segment. The cyan strip at far left edge (Joh / Lis / Sa / Te) is the wallpaper hash-table diagram (Lisa Smith, John Smith, Sandra, Ted...); he never points at it in this segment.

## Timeline

[27:33] Left: vim INSERT, `35,53`, 94%. Cursor after line 35 comment `// if it not empty -> go to last node and add next`. Left file shows lines 35-69 (see File states). Right: Terminal vim in `-- VISUAL --`, lines 24-47 (the whole `put()`, from `void put(` at 24 through `}` at 47) highlighted. Right file is visible lines 19-54. Mouse arrow at left near line 54.
[27:34] Left scrolled to 38-71 (`Bot`), INSERT `38,44`. Cursor sits on `{` of `int get(...) {` (matching brace highlighted cyan at line 38 end and line 46 `}`). Left file now seen in full tail: line 70 `  return 0;`, 71 `}`. Right unchanged (VISUAL put() still highlighted).
[27:35] Left: `-- (insert) VISUAL --  4  57,1  Bot`: he visual-selects lines 57-60 (`arr[hash("cba")].next = &(Node) {` ... `.next = NULL` partial). Right unchanged.
[27:36] Left back to INSERT `60,14`. Right unchanged.
[27:37] Left: `-- (insert) VISUAL -- 6  56,1`: selection 56-61 (`// because abc and cba have same index` through `};`). He is highlighting his own "cba" collision block (the chained second node) while the right-hand put() remains highlighted (mirror/comparison of chaining).
[27:38] Left scrolled to show 29-63, selection 56-61 still on, `56,1 77%`. Mouse near line 46. Right unchanged.
[27:39] Left scrolled to 26-60, `69%`, selection still on. Mouse arrow on right window at `if` of line 28 (`if (strcmp(e->key, key) == 0) {`): pointing at put()'s existing-key update branch.
[27:40] RIGHT window switches to a BROWSER (YouTube): tab title `The Creation of Chain - Linked L...` (red YouTube icon), URL `www.youtube.com/watch?v=iz2j9_AwQdQ&t=336s`. Page shows (dark-theme vim video, lines 8-37) a linked-list C program (see Right-hand file / browser content). Subtitle-like overlay not seen. Left unchanged (26-60 selection).
[27:46] Browser content scrolled: lines 12-37 visible, yellow strip at y~520. Red progress bar at the bottom of the video, partly obscured text `INSERT` and `5:29 / 14:..` (video timestamp overlay, partly transitioning). Mouse arrow over `void` of line 23 `void printList(Node *p) {`.
[27:47] Browser content lines 9-37; mouse on `p = p->next; // go to last node` (line 19). Red progress bar at bottom, `INSERT` overlay gone.
[27:51] Left: INSERT `43,21 69%`; cursor after `return p->value;` at line 43 (selection cleared; he left VISUAL). Browser dimmed (right window now inactive-looking). Right video unchanged.
[27:53] Left: new blank line inserted at 44 (cursor `44,3` INSERT, 67%): file now has `return 0;` at 46, `}` 47, `int main(void) {` at 49. (Line numbers shifted +1.)
[27:55] Left: normal mode `43,20 69%` (blank line removed again, return at 45); the `;` at line 43 highlighted. (Undo / <Esc>. No mode text.)
[27:57] Left: INSERT `36,3 67%`: a blank line is added inside `set()` at line 36 (below the `// if it not empty -> ...` comment at 35); `}` now on 37, `int get(` on 39, `return 0;` 46... wait: 46 `return 0;`, `}` 47, `int main` 49.
[28:12] Left: line 36 now `  while (p->next) p = p->next;` (copied from video's `append`: `while (p->next) p = p->next; // go to last node`); line 37 blank, `}` at 38; INSERT `37,3 65%`.
[28:29] Left scrolled to 23-57. Line 37 `  p->next = |` (cursor after `= `), INSERT `37,13 57%`. Right browser unchanged (create/append/printList/main).
[28:37] Left: line 37 `  // |` comment being typed, line 38 `  p->next =`; INSERT `37,6 56%`.
[28:53] Left: comment on line 37 (wrapped) reads `  // in this, we need malloc, because return address out` / `of |` (continuation on a wrapped line, typing in progress); `p->next =` pushed to line 38. INSERT `37,61 55%`.
[28:57] Left: comment line deleted; line 37 `  p->next =` again, normal mode `37,5 57%` (cursor on `>`). So the malloc comment was removed.
[29:02] Left: a blank line inserted before `p->next =`: 36 while..., 37 blank (cursor), 38 `  p->next =`. INSERT `37,3 56%`.
[29:10] Same as 29:02 (INSERT `37,3 56%`).
[29:11] FOCUS SWITCH to the right-hand TERMINAL (title "Terminal", active): hash table C file vim, `-- VISUAL --`, lines 19-54 visible, put() lines 24-47 highlighted. Left shows empty line 37 with block cursor (inactive). The browser is behind. Mouse I-beam at line 52 `for (Entry *e = table[i]`. Webcam shows him speaking.
[29:12] Right: VISUAL selection cleared (`-- VISUAL --` mode text gone / normal), cursor `;` on line 35 `if (!e) return;`. Mouse at line 34 near `sizeof`.
[29:14] Right: VISUAL again; selected `*e = malloc(sizeof *e);` on line 34 (from after `Entry` to end of line). Mouse at `Entry` of line 34.
[29:37] Left (typing, insert): line 37 `  Node *newNode = malloc(size|` (right-hand VISUAL selection of line 34 `Entry *e = malloc(sizeof *e);` now highlighted fully). INSERT `37,30 56%`.
[30:12] Left: line 37 `  Node *newNode = malloc(sizeof(Node));`, line 38 blank, line 39 `  p->next =` (cursor `39,3`), `}` 40. INSERT `39,3 55%`.
[30:18] Left: line 38 `  newNode->value = value;`, line 39 `p` (typing, wrongly indented at col 2 ` p`), line 40 `  p->next =`. INSERT `39,3 53%`.
[30:23] Left: 38 `newNode->value = value;`, 39 `newNode->key = key;`, 40 blank (cursor `40,2`), 41 `p->next =`, `}` 42. INSERT `40,2 52%`.
[30:36] Left (normal mode, `69,20 80%`, scrolled to 35-69): complete set() tail: 36 `while (p->next) p = p->next;`, 37 `Node *newNode = malloc(sizeof(Node));`, 38 `newNode->value = value;`, 39 `newNode->key = key;`, 40 `newNode->next = NULL;`, 41 `p->next = newNode;`, 42 `}`. File now has main at 54. Cursor at line 69 `// how we get access`. (He saved/jumped, then runs.)
[30:39] Left window switches to the `-zsh` tab (tab highlights): shell history (see Shell & outputs). Right: put() line 34 still highlighted.
[30:41] Left back to vim: `"main.c" 77L, 1341B written`, `1,1 Top`. Lines 1-35 shown. NOTE: despite tab title `vim test.c`, the written file is `main.c` (title likely stale / file renamed).
[30:42] Left vim INSERT: line 3 `#i|` being typed (cursor `3,3`): he is adding `#include <stdlib.h>` (line 4 blank, `#define SIZE 5` at 5). Lines shift by 1.
[30:48] Left `-zsh` tab: history plus new input line `lab $ cc main.c` (cursor after it, not executed yet in the frame).
[31:00] Left `-zsh` tab: screen cleared (Ctrl-L), prompt `lab $ ./a.out` typed.
[31:03] Left `-zsh`: compile output with errors (see Shell & outputs). `1 warning and 4 errors generated.`
[31:05] Left `-zsh`: scrolled to top of the error output (see below); mouse I-beam at line `const-qualified type 'const Node *'` near `Node`.
[31:13] Left vim tab again (tab shows `x` close icon): `"main.c" 78L, 1361B written`, `3,19 Top`: line 3 is now `#include <stdlib.h>` (the `>` highlighted); 78 lines.
[31:14] Left vim scrolled to 4-38, normal mode `4,0-1 6%`.
[31:15] Left vim scrolled to 10-44, normal `10,19 20%`; shows complete set() (lines 25-43) and an empty line 44. Cursor on `const` at line 27 under the I-beam (the cause of the compile errors).
[31:19] Left `-zsh` tab again, showing the same error output (top portion `77` / `lab $ cc main.c` / errors).

No sticky-note text, no origin text, no `:w` command visible other than the two `written` messages.

## File states

### Right-hand browser (27:40-29:10), YouTube video "The Creation of Chain - Linked L[ist]" (video content, his own earlier linked-list lesson, vim in dark theme)
```
 9| Node *create(int val) {
10|   Node *p = malloc(sizeof Node);        // seen as: Node *p = malloc(sizeof Node);
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
32|   append(|
33|
34|   printList(&n1);
35|
36|   return 0;
37| }
```
(line 8 partially seen as `} Node;`-like at 27:40. Lines 1-7 never seen.) This is a paused video at t=336s, video position overlay `5:29 / 14:..` flashes at 27:46.

### Right-hand terminal file (put() and part of get(), lines 19-54, stable the whole segment)
```
19|         h = h * 33 + (unsigned char)*s++;
20|
21|     return h % SIZE;
22| }
23|
24| void put(const char *key, int value) {
25|     unsigned long i = hash(key);
26|
27|     for (Entry *e = table[i]; e; e = e->next) {
28|         if (strcmp(e->key, key) == 0) {
29|             e->value = value;
30|             return;
31|         }
32|     }
33|
34|     Entry *e = malloc(sizeof *e);
35|     if (!e) return;
36|
37|     e->key = malloc(strlen(key) + 1);
38|     if (!e->key) {
39|         free(e);
40|         return;
41|     }
42|
43|     strcpy(e->key, key);
44|     e->value = value;
45|     e->next = table[i];
46|     table[i] = e;
47| }
48|
49| int *get(const char *key) {
50|     unsigned long i = hash(k??
51|
52|     for (Entry *e = table[i??
53|         if (strcmp(e->key, k??
54|             return &e->value??
```
Indentation 4 spaces. (Right-hand file lines 1-18 and 55+ NOT seen in this segment; the right text is partly occluded by the webcam at the right side, x>1126, for lines 43+ in the bottom half.)
Highlight states: 27:33-29:11 lines 24-47 VISUAL selected; 29:12 cleared; 29:14 `*e = malloc(sizeof *e);` line 34 selected; 29:37 onward line 34 whole `Entry *e = malloc(sizeof *e);` selected (col 5 to end).

### Left file (test.c / main.c), end of segment (as of 31:15 + 30:36 + 31:13 views)
```
 1| #include <stdio.h>
 2| #include <string.h>
 3| #include <stdlib.h>
 4|
 5| #define SIZE 5
 6|
 7| typedef struct Node {
 8|   char *key;
 9|   int value;
10|   struct Node *next;
11| } Node;
12|
13| size_t hash(const char *key) {
14|   size_t h = 0;
15|
16|   const char *c = key;
17|   while (*c) {
18|     h += *c;
19|     c++;
20|   }
21|
22|   return h % SIZE;
23| }
24|
25| void set(Node *arr, const char *key, int value) {
26|   size_t index = hash(key);
27|   const Node *p = &arr[index];
28|
29|   // this index is empty, just put value to it
30|   if (!p->key) {
31|     p->value = value;
32|     p->key = key;
33|     p->next = NULL;
34|   }
35|
36|   // if it not empty -> go to last node and add next
37|   while (p->next) p = p->next;
38|   Node *newNode = malloc(sizeof(Node));
39|   newNode->value = value;
40|   newNode->key = key;
41|   newNode->next = NULL;
42|   p->next = newNode;
43| }
44|
45| int get(const Node *arr, const char *key) {
46|   size_t index = hash(key);
47|   const Node *p = &arr[index];
48|   for (; p; p = p->next)
49|    if (strcmp(p->key, key) == 0)
50|     return p->value;
51|
52|  return 0;
53| }
54|
55| int main(void) {
56|   Node arr[SIZE] = {0};
57|
58|   arr[hash("abc")] = (Node) {
59|     .value = 22,
60|     .key = "abc",
61|     .next = NULL
62|   };
63|   // because abc and cba have same index
64|   arr[hash("cba")].next = &(Node) {
65|     .value = 77,
66|     .key = "cba",
67|     .next = NULL
68|   };
69|
70|   // how we get access
71|   // get arr "abc" , get arr "def"
72|
73|   printf("%d\n", get(arr, "abc"));
74|   printf("%d\n", get(arr, "cba"));
75|
76|
77|   return 0;
78| }
```
Line numbers 4-78 consistent with the 30:36 view (after `#include <stdlib.h>` was added at line 3, "78L, 1361B written"; before that "77L, 1341B written"). Original `set()` was at 24 (before include added), `get` at 38, main 48 at 27:33 and 40-line shifts consistent. `p` declared `const Node *p` at line 27 is the cause of the errors. Indentation: 2 spaces; `if (strcmp` in get has 3-space indent (` if`), `return p->value;` 4; `return 0;` indented 1 space (` return 0;`) at the 27:33 view.

Intermediate versions of set() tail (before the 30:36 final):
- 28:12 : `// if it not empty -> go to last node and add next` / `while (p->next) p = p->next;` / blank
- 28:29 : adds `p->next = ` (incomplete)
- 28:37-28:53: comment lines `//` -> `// in this, we need malloc, because return address out` / `of ` (wrapped) -- later deleted at 28:57
- 29:37 : `Node *newNode = malloc(size`
- 30:12 : `Node *newNode = malloc(sizeof(Node));` + blank + `p->next =`
- 30:18 : `newNode->value = value;`
- 30:23 : `newNode->key = key;`
- 30:36 : `newNode->next = NULL;` / `p->next = newNode;`

## Shell & outputs

[30:39] -zsh tab content (history; earlier commands from before this segment):
```
lab $ cc main.c
lab $ ./a.out
22
zsh: segmentation fault  ./a.out
lab $ cc main.c
lab $ ./a.out
22
77
lab $ ./a.out
```
(cursor block after `./a.out`, not yet executed.)
[30:48] same, last line becomes `lab $ cc main.c`.
[31:00] `lab $ ./a.out` (screen cleared).
[31:03]/[31:05] compile output (after cc main.c; first part scrolled; at 31:05 top shows `77` then `lab $ cc main.c`):
```
main.c:31:14: error: cannot assign to variable 'p' with const-qualified type 'const Node *' (aka 'const struct Node *')
   31 |     p->value = value;
      |     ~~~~~~~~ ^
main.c:27:15: note: variable 'p' declared const here
   27 |   const Node *p = &arr[index];
      |   ~~~~~~~~~~~~~~^~~~~~~~~~~~~~~~~~
main.c:32:12: error: cannot assign to variable 'p' with const-qualified type 'const Node *' (aka 'const struct Node *')
   32 |     p->key = key;
      |     ~~~~~~ ^
main.c:27:15: note: variable 'p' declared const here
   27 |   const Node *p = &arr[index];
main.c:33:13: error: cannot assign to variable 'p' with const-qualified type 'const Node *' (aka 'const struct Node *')
   33 |     p->next = NULL;
      |     ~~~~~~~ ^
main.c:27:15: note: variable 'p' declared const here
   27 |   const Node *p = &arr[index];
main.c:40:16: warning: assigning to 'char *' from 'const char *' discards qualifiers [-Wincompatible-pointer-types-discards-qualifiers]
   40 |   newNode->key = key;
      |                  ^ ~~~
main.c:42:11: error: cannot assign to variable 'p' with const-qualified type 'const Node *' (aka 'const struct Node *')
   42 |   p->next = newNode;
      |   ~~~~~~~ ^
main.c:27:15: note: variable 'p' declared const here
   27 |   const Node *p = &arr[index];
1 warning and 4 errors generated.
lab $
```
(Note in the errors the compiled line numbers 31,32,33,40,42 match a file where set() `const Node *p` is line 27, `p->value` line 31, `newNode->key = key` line 40, `p->next = newNode` line 42, i.e. includes `<stdlib.h>` (line 3).) The `cc main.c` that produced these ran at ~31:01-31:03 after the second `:w` ("main.c" 78L, 1361B written) probably; ordering: first save 77L (30:41) -> typed `#include <stdlib.h>` -> second save 78L 1361B (31:13 frame shows it, but the compile at 31:03 already has line numbers consistent with 78L, so 31:13 is simply a later view of the vim message). Count mismatch: 31:03 errors line numbers match the 78L file.
The `-Wincompatible-pointer-types-discards-qualifiers` warning is about `char *key` in struct vs `const char *key`.

## Observations

- Learning method: he explicitly tries to implement chaining ("set": if bucket empty put value, else walk to last node and append a malloc'd node) in his own `test.c/main.c`, and highlights the right-hand `put()` (27:33-29:37) as the reference. He compares his collision-handling block in `main()` (cba collides with abc, hand-chained via compound literal) to put(), specifically the line 34 `Entry *e = malloc(sizeof *e);` which he then re-types as `Node *newNode = malloc(sizeof(Node));` in his own `set()` (29:37). He does not copy put() wholesale: he adapts the idea (malloc a node, fill value/key/next, link into the chain), with his own names (`newNode`, `arr`, `set`), and his own struct (`Node` with `key,value,next`), plus his own hash (`h += *c`, `% SIZE`, `size_t`), different from right hand's `h * 33 + (unsigned char)*s++`. He does NOT copy the right-hand `strdup`-like `malloc(strlen(key)+1)` + `strcpy` (stores `key` pointer directly, which triggers the const warning).
- He deliberately reuses his own earlier video: at 27:40-29:10 the right window is a browser on YouTube video "The Creation of Chain - Linked L..." (`youtube.com/watch?v=iz2j9_AwQdQ&t=336s`), a dark-vim linked list tutorial containing `create`, `append` (`while (p->next) p = p->next; // go to last node`), `printList`. He copies `while (p->next) p = p->next;` from the `append` in that video into `set()` at 28:12 (also copying the comment idea "go to last node"). So origin evidence this segment: (1) the linked-list append pattern comes from a YouTube video (likely his own earlier "chain" lesson, `t=336s` is where `append` is written); (2) the right-hand terminal file (Entry/put/get) origin is NOT shown in this segment, no ChatGPT/AI window, tab name, or sticky text visible. The right terminal window's title is just "Terminal".
- The webcam shows him smoking in 27:51-28:12, pausing to think while he decides how to write the append.
- Mistakes/learning: he tried a comment `// in this, we need malloc, because return address out of ...` (28:53), reasoning that a local variable's address cannot be returned out of the function (stack vs heap), then deleted it (28:57) and just typed the malloc. Compile exposes `const Node *p` problem (const-qualified pointer being written through); he goes back to vim and looks at line 27 `const` (31:15), meaning the fix is probably removing `const` (not yet done in this segment). The `-Wincompatible-pointer-types-discards-qualifiers` warning (key is `const char *` assigned to `char *key`) also shown.
- File name inconsistency: tab titled `vim test.c` but vim writes `"main.c"`; compile via `cc main.c`.
- Philosophy signals: incremental TDD-ish loop (edit -> save -> cc -> ./a.out), reading reference then re-implementing in smaller words; prints 22 and 77 earlier proves `get` walks chain (from earlier segments; output history shown at 30:39).
- Uncertain: exact sequence between 30:41 `77L` and 31:13 `78L`; frame at 31:19 returns to the error output tab.
