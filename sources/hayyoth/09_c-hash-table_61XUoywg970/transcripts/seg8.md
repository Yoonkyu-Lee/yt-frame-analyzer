# Segment 8 (20:18 - 24:07), 40 frames

## Timeline
Layout throughout: LEFT Terminal (tabs `~/lab — vim test.c`, `~/lab — -zsh`), RIGHT Terminal with vim (another C file, in VISUAL mode almost the whole segment), yellow Sticky, webcam. NOTE: the left tab title says `vim test.c`, but vim status messages and `cc main.c` show the file is actually `main.c` (tab title stale or file renamed earlier).

- [20:18] Left: vim INSERT, cursor 27,14 (46%). Line 27 half-typed: ` if (p.key ▯` (cursor box). Right: vim NORMAL, lines 43-78; cursor box on `u` of line 50 `unsigned long i = hash(key);`; mouse pointer near line 54. Sticky (verbatim): "if you not understand linked list , check my previous video".
- [20:23] Right: VISUAL, highlights from after `for` on line 52 through line 54: `(Entry *e = table[i]; e; e = e->next)` / `if (strcmp(e->key, key) == 0)` / `return &e->value;` (the get() lookup loop). Highlight persists until 24:07.
- [20:36] Left: line 27 now ` if (strcmp(p.key, key) == 0)`, cursor on new line 28 (INSERT 28,5, 43%). main now starts at line 30.
- [20:49] Left: line 28 `return p.value;`, cursor 29,2 (41%).
- [20:54] Left: cursor 30,2 (38%), blank line 29-30.
- [20:56] Left: line 30 ` return 0;` (1-space indent), INSERT 31,1 (36%).
- [20:59] Left: NORMAL (no mode text), cursor 48,1 (68%). get() closes at line 31 `}`. Line 48 shows `// how we get access` (blue comment, block cursor at col 1).
- [21:03] Left: top of file, INSERT, cursor 2,1 (Top). Line 1 `#include <stdio.h>`, line 2 blank (cursor), 3 blank, 4 `#define SIZE 5`. He is about to add `#include <string.h>`.
- [21:07] Left switches to `-zsh` tab: `lab $ ▮`.
- [21:08] `lab $ cc main.c` -> 2 compiler errors (verbatim in Shell section).
- [21:14] Back to vim tab: line 2 now `#include <string.h>` (final `>` highlighted); `"main.c" 55L, 765B written`, cursor 2,19 Top.
- [21:16] Left scrolled to 11-45, NORMAL, cursor 44,16 (50%). Still `const Node p = arr[index];` (26), `for (; p; p = p.next)` (27), `p.key` (28), `p.value` (29). get signature on line 24 `int get(const Node *arr, const char *key) {`.
- [21:41] zsh tab: the same 2-error output still on screen (same as 21:08).
- [21:43] Vim: FIX applied: line 26 `const Node *p = &arr[index];`, 27 `for (; p; p = p->next)`, 28 `if (strcmp(p->key, key) == 0)`, 29 `return p->value;` (cursor on `>` of `->`, 29,14). Status: `"main.c" 55L, 770B written`.
- [21:44] Left scrolled (12-46), cursor 46,14 on `NULL` (55%).
- [21:45] Left at bottom of file (Bot, cursor 53,0-1). main tail now: `48` blank, `49   // how we get access`, `50   // get arr "abc" , get arr "def"`, 51-53 blank (cursor on 53), `54   return 0;`, `55 }`.
- [22:10] Left INSERT 53,31 (90%): added `52   printf("%d\n", get(arr, "abc"));` and `53   printf("%d\n", get(arr, "def"));` (cursor right after `def`).
- [22:11] Left NORMAL, cursor 57,1 (Bot): file is 57 lines; `56   return 0;`, `57 }`; lines 54-55 blank.
- [22:14] zsh tab: old error block still above, then a second `lab $ cc main.c` with NO output (clean compile, no warnings), new prompt.
- [22:15] zsh cleared: only `lab $ ▮`.
- [22:23] Vim: `"main.c" 57L, 840B written`, cursor 55,0-1 (Bot). Mouse I-beam at end of line 53.
- [22:39] Left VISUAL (15 chars selected, cursor 43,33): highlights `.next = &(Node)` in line 43 `arr[hash("def")].next = &(Node) {`. (He is explaining the chaining assignment.)
- [23:08] Left: "def" changed to "cba": line 43 `arr[hash("cba")].next = &(Node) {`, line 45 `.key = "cba",`, line 53 `printf("%d\n", get(arr, "cba"));` (cursor 53,30 on `a`, 840B written; lines 49-50 comments still say `"def"`). Status `"main.c" 57L, 840B written`.
- [23:09] zsh tab (verbatim in Shell section): `cc main.c`, `./a.out`, `22`, `zsh: segmentation fault  ./a.out`. The text `zsh: segmentation fault  ./a` is highlighted by the mouse.
- [23:17] Back to vim tab (tab hover shows `x` close button at left of tab); same code as 23:08, mouse I-beam at line 38; `"main.c" 57L, 840B written` still on status.
- [23:28] Left INSERT: new line 42 comment typed in place of the blank line: `  // because abc and cba have same index` (cursor 42,41). File still 57 lines in later frames (comment replaced the blank line 42). Mouse pointer moving left-bottom toward the desktop.
- [23:29] Full-screen wallpaper (both terminals hidden; Mission Control / space switch). Diagram visible: keys (John Smith, Lisa Smith, Sam Doe, Sandra Dee, Ted Baker) -> buckets (000, 001, 002, :, 151, 152 [red], 153, 154, :, 253, 254, 255) -> entries (Lisa Smith 521-8976; John Smith 521-1234 -> Sandra Dee 521-9655 chained; Ted Baker 418-4165; Sam Doe 521-50..). Pointer near buckets 153/154. Top-right: desktop file thumbnail "Screen Recording 2026-10...0.00 PM".
- [23:36] Wallpaper still; pointer now on the chained entry "Sandra Dee" (below John Smith), i.e. he points at the collision chain of bucket 152 (red).
- [23:37] Terminals slide back in (transition, windows offset ~10px left). Left shows line 42 comment, cursor box after `index`, INSERT 42,41. Behind the right terminal a browser window top bar is partly visible, with a tab title that begins with something unreadable `⟨?⟩`.
- [23:38] Left NORMAL 42,40 (Bot), last `x` of `index` highlighted.
- [23:40] Right window active, scrolled to end of the file (lines 76-91 plus `~`): VISUAL highlight on `put(` and `20)` of line 76. Lines 80-90 as in File states. Right file has 91 lines.
- [23:44] Right: highlight on `get("apple");` in line 80. Left unchanged.
- [23:45] Left scrolled to 20-54 (86%), cursor 42,40; right as 23:44.
- [23:46] Right scrolled to lines 58-91 (destroy 59-72, main 74-91), highlight remains in line 80.
- [23:47] Right scrolled to lines 34-69 (rest of put(), get(), destroy()), cursor box at line 69.
- [23:49] Left VISUAL, 5 lines selected (cursor 43,7): from `hash("cba")` on line 43 through `};` on 47. Mouse I-beam at line 41.
- [23:50] Left VISUAL selection grown to lines 37-47 (11 lines, cursor 37,1), covering abc and cba initializers and the comment. Mouse pointer over the sticky text ("understand").
- [23:52] Sticky text is DELETED: sticky now empty with a blinking caret (title-bar controls visible). Right shows lines 34-69. Left selection unchanged.
- [24:05] Sticky collapsed (rolled up) into a thin yellow title bar over right line 53. Webcam window `Human` fully visible. Left selection 37-47 persists.
- [24:07] Right: VISUAL selection now over lines 61-69 (`Entry *e = table[i];` ... closing `}` of the while). Sticky still collapsed. Left unchanged.

## File states

### Left file (tab says test.c; saved as main.c), as of 24:07
Lines 1-11 seen at 21:03/21:14, 12-57 at 23:28-24:07. Indent 2 spaces; lines 28-29 and 31 have odd indents as rendered.
```
 1| #include <stdio.h>
 2| #include <string.h>
 3|
 4| #define SIZE 5
 5|
 6| typedef struct Node {
 7|   char *key;
 8|   int value;
 9|   struct Node *next;
10| } Node;
11|
12| size_t hash(const char *key) {
13|   size_t h = 0;
14|
15|   const char *c = key;
16|   while (*c) {
17|     h += *c;
18|     c++;
19|   }
20|
21|   return h % SIZE;
22| }
23|
24| int get(const Node *arr, const char *key) {
25|   size_t index = hash(key);
26|   const Node *p = &arr[index];
27|   for (; p; p = p->next)
28|    if (strcmp(p->key, key) == 0)
29|     return p->value;
30|
31|  return 0;
32| }
33|
34| int main(void) {
35|   Node arr[SIZE] = {0};
36|
37|   arr[hash("abc")] = (Node) {
38|     .value = 22,
39|     .key = "abc",
40|     .next = NULL
41|   };
42|   // because abc and cba have same index
43|   arr[hash("cba")].next = &(Node) {
44|     .value = 77,
45|     .key = "cba",
46|     .next = NULL
47|   };
48|
49|   // how we get access
50|   // get arr "abc" , get arr "def"
51|
52|   printf("%d\n", get(arr, "abc"));
53|   printf("%d\n", get(arr, "cba"));
54|
55|
56|   return 0;
57| }
```
Indent of line 28 is 3 spaces (1 deeper than `for`+2? it renders ` if` one space past `for`'s body indent) and line 29 is 4 spaces; line 31 ` return 0;` is 1 space. Exact counts from rendering, not guaranteed.

Intermediate versions:
- 20:18: get() = `int get(const Node *arr, const char *key) {` / `size_t index = hash(key);` / `const Node p = arr[index];` / `for (; p; p = p.next)` / ` if (p.key ▯` (line 27).
- 20:36-20:56: line 27 ` if (strcmp(p.key, key) == 0)`, 28 `return p.value;`, then ` return 0;`.
- 21:14 (`"main.c" 55L, 765B written`): + `#include <string.h>`; still `const Node p = arr[index];`, `p.next`, `p.key`, `p.value`.
- 21:43 (`"main.c" 55L, 770B written`): `const Node *p = &arr[index];`, `p->next`, `p->key`, `p->value`.
- 22:23 (`"main.c" 57L, 840B written`): + two printf lines (abc, def).
- 23:08: "def" -> "cba" (lines 43, 45, 53).
- 23:28: comment line 42 added.

### Right-hand file (second terminal vim, file name never shown), as seen this segment
Lines 43-78 at 20:18; 76-91 at 23:40; 58-91 at 23:46; 34-69 at 23:47. 4-space indent. Lines 1-33 not visible.
```
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
50|     unsigned long i = hash(key);
51|
52|     for (Entry *e = table[i]; e; e = e->next)
53|         if (strcmp(e->key, key) == 0)
54|             return &e->value;
55|
56|     return NULL;
57| }
58|
59| void destroy(void) {
60|     for (int i = 0; i < SIZE; i++) {
61|         Entry *e = table[i];
62|
63|         while (e) {
64|             Entry *next = e->next;
65|             free(e->key);
66|             free(e);
67|             e = next;
68|         }
69|
70|         table[i] = NULL;
71|     }
72| }
73|
74| int main(void) {
75|     put("apple", 10);
76|     put("banana", 20);
77|     put("orange", 30);
78|     put("apple", 99);
79|
80|     int *value = get("apple");
81|
82|     if (value)
83|         printf("%d\n", *value);  // 99
84|
85|     value = get("banana");
86|
87|     if (value)
88|         printf("%d\n", *value);  // 20
89|
90|     destroy();
91| }
```
Right get() returns `int *` (NULL if missing); left get() returns `int` (0 if missing).

## Shell & outputs
- [21:08] (also visible 21:41, 22:14)
```
lab $ cc main.c
main.c:27:3: error: statement requires expression of scalar type ('Node' (aka 'struct Node') invalid)
   27 |   for (; p; p = p.next)
      |   ^        ~
main.c:27:15: error: cannot assign to variable 'p' with const-qualified type 'const Node' (aka 'const struct Node')
   27 |   for (; p; p = p.next)
      |                 ~ ^
main.c:26:14: note: variable 'p' declared const here
   26 |   const Node p = arr[index];
      |   ~~~~~~~~~~~^~~~~~~~~~~~~~
2 errors generated.
lab $
```
(Line-wrapping and exact `^`/`~` columns approximate.)
- [22:14] second `lab $ cc main.c` -> no output (clean). [22:15] screen cleared.
- [23:09]
```
lab $ cc main.c
lab $ ./a.out
22
zsh: segmentation fault  ./a.out
lab $
```
  (First printf prints 22, second lookup of "cba" segfaults.)
- Vim save messages: `"main.c" 55L, 765B written` (21:14); `"main.c" 55L, 770B written` (21:43); `"main.c" 57L, 840B written` (22:23, 23:08, 23:17).

## Observations
- Stickies: only one note, verbatim "if you not understand linked list , check my previous video" (visible 20:18-23:50). At 23:52 its text is deleted (empty, caret), and at 24:05-24:07 it is collapsed to a title bar. No new note text in this segment.
- Origin evidence for the right-hand code: none new. The right terminal's vim never shows a filename. A browser-like window sits behind the right terminal (sidebar/translate icons top right, tab title unreadable `⟨?⟩`). The "Screen Recording 2026-10...0.00 PM" thumbnail at 23:29 is just the macOS recording file.
- Wallpaper use: at 23:29-23:36 he points at bucket 153/154 then at the John Smith -> Sandra Dee chain under red bucket 152 to illustrate collision/chaining, tying to his "abc and cba have same index" comment (char-sum hash: 294 % 5 = 4 for both).
- Learning method: reads the right get() (highlights the for/strcmp loop 52-54), then re-derives his own array-of-Node version of get() with p walking `next`; compiles after each chunk; fixes const/struct-vs-pointer errors by switching to `const Node *p = &arr[index]` and `->`; adds `#include <string.h>`. Writes printf tests, then deliberately changes "def" to "cba" to force a same-bucket collision, and runs it: first lookup prints 22, second segfaults (not resolved in this segment; no further run). Later (23:40-24:07) he skims the right file's main() (put/get/destroy usage, lines 76-91) and destroy() (61-69), highlighting parts, and clears/collapses the sticky.
- Design contrast: left = fixed array of Node with first node stored inline in the bucket and chain via `next` (compound literals); right = array of `Entry *` heads with malloc'd entries, put/get/destroy. He borrows idioms from the right file rather than copying it.
- Philosophy signals: small steps, compile often, read errors; use of his own diagram as mental model; points viewers to prior linked-list video as prerequisite.
- Uncertainties: whitespace on left lines 28-31; exact `^`/`~` columns in compiler output; the tab title `test.c` vs actual `main.c`.
