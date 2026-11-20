# Segment 11 of 11 (31:19 - 33:51): final fixes, final compile, closing sticky

Frames viewed: all 40 (k_31m19s ... k_33m51s). Left terminal tabs: `~/lab — vim test.c` (title never changes) and `~/lab — -zsh` (once it reads `...Wno-unused-parameter main.c` while compiling). NOTE: vim's own messages say `"main.c"`, so the file on disk is main.c although the tab title says "vim test.c". Menu bar at 33:51: app `Stickies` frontmost, clock `Fri Oct 23 2:36 PM`.

## Timeline

- [31:19] Left window on `-zsh` tab; scrollback shows `77` (earlier program output) then `lab $ cc main.c` with compile errors (verbatim in Shell section). Right window: vim VISUAL mode, lines 19-54 visible, line 34 `Entry *e = malloc(sizeof *e);` highlighted (stays highlighted until 33:04). A thin empty yellow bar (a rolled-up/collapsed Stickies note, no readable text) sits across the right editor at y~520 from 31:19 until 33:27.
- [31:20] Left tab title is `...Wno-unused-parameter main.c`; screen shows `lab $ cc main.c` and a cursor, no output yet (cc appears to be a wrapper/alias with flags; tab title shows the tail of the real command line).
- [31:21] Compile finished: 2 warnings only (see Shell).
- [31:28] Left switched to vim tab. Message `"main.c" 78L, 1355B written` at 32,3 (20%). In set(), line 27 is `Node *p = &arr[index];` (the `const` was removed from `p`). Cursor on `p->key = key;` (line 32).
- [31:29] Cursor 47,3 (27%): get() starts line 45; line 47 `const Node *p = &arr[index];` (const kept in get).
- [31:44] Left back on zsh tab showing the 2-warning output.
- [31:53] Vim tab: cursor on line 30 `if (!p->key) {` (30,3, 27%).
- [32:00] Vim: `#define SIZE 5` at line 5, cursor on the `5` (5,14, 9%). struct Node at lines 7-11 still has `char *key;` (line 8).
- [32:05] Zsh tab cleared: `lab $ `.
- [32:08] Vim: line 8 now `const char *key;` (cursor 8,8). Message `"main.c" 78L, 1361B written` (6 bytes more = "const ").
- [32:10] Cursor on `main` at line 55 (55,8, 46%); lines 21-55 visible; get() at 45-53 with odd indentation.
- [32:11] Cursor 68,4 (76%): main() lines 55-68 visible (original hand-built-nodes version, see File states).
- [32:12] Cursor 73,8 (90%): lines 70-74 are `// how we get access`, `// get arr "abc" , get arr "def"`, blank, two printf lines.
- [32:15] `-- INSERT --` at 58,3 (88%): new blank line 58 inserted in main body above `arr[hash("abc")] = (Node) {` (which moves to 59).
- [32:18] INSERT 58,7: line 58 is `  set(` (being typed).
- [32:19] INSERT 56,7 (47%): cursor moved onto line 56 after `Node`; no visible text change.
- [32:20] INSERT 53,3 (68%): cursor after `}` of get(); line 58 still `  set(`.
- [32:22] INSERT 53,3 (95%): end of file visible: lines 74-75 printf abc / printf cba, blank line 76, 77. I-beam over line 62.
- [32:31] `-- VISUAL LINE --`, `5` lines (63,5; 95%): lines 59-63 selected (the `arr[hash("abc")] = (Node) {...};` block). Line 58 now reads `  set(arr, "abc", 22);`.
- [32:32] Message `6 fewer lines` (59,3, Bot): block deleted. Now line 58 `set(arr, "abc", 22);`, 59 starts the cba block `arr[hash("cba")].next = &(Node) {`, return 0 at 72, `}` 73. (The `// because abc and cba have same index` comment was also removed.)
- [32:33] INSERT at 59,3: blank line 59 added; cba block now at 60-64.
- [32:42] INSERT 60,6: line 59 `set(arr, "cba", 77);`, line 60 `set` (being typed), cba block 61-65.
- [32:50] VISUAL LINE `5` lines (65,5): line 60 `set(arr, "def", 33);`; lines 61-65 (cba block) selected.
- [32:51] VISUAL LINE `8` lines (68,22): selection extended to lines 61-68 (cba block, blank, and the comments `// how we get access` / `// get arr "abc" , get arr "def"`).
- [32:52] Message `8 fewer lines` (62,1, Bot): deleted. File now 67 lines: set x3 at 58-60, blank 61, printf abc 62, printf cba 63, blank 64-65, `return 0;` 66, `}` 67.
- [33:04] Zsh tab: `lab $ cc main.c` then `lab $ ` : clean compile, no output.
- [33:12] Vim: added line 64 `printf("%d\n", get(arr, "def"));`. Message `"main.c" 67L, 1184B written` at 65,0-1 (Bot). Mouse arrow now over right window.
- [33:13] Right window activated, scrolled to lines 52-87 (destroy() visible); left dimmed.
- [33:14] Right scrolled to 67-91 (end of main).
- [33:17] Left window active again, scrolled to 46-67, cursor 65,0-1.
- [33:23] Left scrolled to lines 16-50 (50,1; 46%). Tab strip shows `...` next to the vim tab title. Pointer near line 29.
- [33:24] Left shows lines 10-44 (44,0-1; 28%). Right at 67-91, cursor on blank line 79.
- [33:25] Right scrolled to top: lines 4-39 (`#define SIZE 101`, `struct Entry`, `Entry *table[SIZE];`, djb2 `hash`).
- [33:26] Right scrolled to 16-51; cursor on blank line 36.
- [33:27] Right scrolled to 25-60; the rolled-up yellow sticky now sits at line 44 with the mouse arrow on it.
- [33:28] Sticky dragged to centre (x 472-905, y ~345-360), still rolled up (tiny corner boxes).
- [33:29] Sticky expanded (yellow note y 345-633); text being typed: `alklk` (first stray typing).
- [33:40] Sticky text: `all work , we now understand` / `hashtable and chat` (cursor).
- [33:49] Sticky text: `all work , we now understand` / `hashtable and chatgpt code` / `you implement refactor of it for` / `comp` (cursor).
- [33:51] Final sticky (last frame): `all work , we now understand hashtable and chatgpt code you implement refactor of it for complex ` (displayed wrapped as: "all work , we now understand / hashtable and chatgpt code / you implement refactor of it for / complex |"), trailing space after "complex", cursor after it.

## Sticky notes (verbatim)
1. Rolled-up yellow bar, 31:19-33:28: no text visible.
2. Intermediate: `alklk`
3. Intermediate: `all work , we now understand hashtable and chat`
4. Intermediate: `all work , we now understand hashtable and chatgpt code you implement refactor of it for comp`
5. FINAL / closing: `all work , we now understand hashtable and chatgpt code you implement refactor of it for complex `
(Cyan sticky fragments at the left screen edge: "Joh", "Lis", "Sa", "San", "Te"; older notes, offscreen, unreadable.)

## File states

### LEFT file (vim; messages say main.c, tab says test.c): state at END (~33:24). Lines 1-4 never visible in this segment.
```
1| ??
2| ??
3| ??
4| ??
5| #define SIZE 5
6|
7| typedef struct Node {
8|   const char *key;
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
27|   Node *p = &arr[index];
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
58|   set(arr, "abc", 22);
59|   set(arr, "cba", 77);
60|   set(arr, "def", 33);
61|
62|   printf("%d\n", get(arr, "abc"));
63|   printf("%d\n", get(arr, "cba"));
64|   printf("%d\n", get(arr, "def"));
65|
66|   return 0;
67| }
```
Indentation as seen: 2 spaces; lines 49 (3 spaces), 50 (4), 52 (1) are oddly indented. Hidden lines 1-4 presumably includes (#include lines), not seen.

Notes on the code as written: in set() the `if (!p->key) {...}` block has no `return`, so for an empty slot execution falls through to `while (p->next)...` and appends a second node. set() has no duplicate-key update. get() uses `strcmp(p->key, key)` on an empty array slot whose key is NULL if the slot is empty.

#### Intermediate versions
- v1 (before 31:28): struct `char *key;` and set() `const Node *p = &arr[index];` (main.c:27) -> compile errors.
- v2 (31:28, `78L, 1355B written`): set() `Node *p = &arr[index];`. Struct still `char *key;` -> 2 warnings (main.c:32:12, main.c:40:16).
- v3 (32:08, `78L, 1361B written`): struct field `const char *key;`.
- v4 main() before replacing (seen 32:11-32:15), lines 55-~78:
```
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
77|   return 0;   (inferred: blank lines then return; exact numbering partly unseen)
78| }
```
  Then 32:15-32:52 he inserted the three `set(...)` calls and deleted the hand-built nodes and the comments.
- v5 (33:12, `"main.c" 67L, 1184B written`): the final text above.

### RIGHT file (ChatGPT-style hash table; static during this segment, only scrolled). Lines 1-3 never visible.
```
1| ??
2| ??
3| ??
4| ??
5| #define SIZE 101
6|
7| typedef struct Entry {
8|     char *key;
9|     int value;
10|     struct Entry *next;
11| } Entry;
12|
13| Entry *table[SIZE];
14|
15| unsigned long hash(const char *s) {
16|     unsigned long h = 5381;
17|
18|     while (*s)
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
4-space indent. Whole file is 91 lines (vim shows `~` after). Some lines were partly hidden behind the webcam / sticky (e.g. right parts of lines 50, 52-54); content inferred from the full-width frames at 31:19 and 33:13-33:17. The `// 99` and `// 20` comments are the only comments.

## Shell & outputs

- [31:19] scrollback (exact line breaks reflowed to the window width):
```
77
lab $ cc main.c
main.c:31:14: error: cannot assign to variable 'p' with const-qualified type 'const Node *' (aka 'const struct Node *')
   31 |     p->value = value;
      |     ~~~~~~~~ ^
main.c:27:15: note: variable 'p' declared const here
   27 |   const Node *p = &arr[index];
      |   ~~~~~~~~~~~~~^~~~~~~~~~~~~~
main.c:32:12: error: cannot assign to variable 'p' with const-qualified type 'const Node *' (aka 'const struct Node *')
   32 |     p->key = key;
      |     ~~~~~~ ^
main.c:27:15: note: variable 'p' declared const here
   27 |   const Node *p = &arr[index];
      |   ~~~~~~~~~~~~~^~~~~~~~~~~~~~
main.c:33:13: error: cannot assign to variable 'p' with const-qualified type 'const Node *' (aka 'const struct Node *')
   33 |     p->next = NULL;
      |     ~~~~~~~ ^
main.c:27:15: note: variable 'p' declared const here
   27 |   const Node *p = &arr[index];
      |   ~~~~~~~~~~~~~^~~~~~~~~~~~~~
main.c:40:16: warning: assigning to 'char *' from 'const char *' discards qualifiers [-Wincompatible-pointer-types-discards-qualifiers]
   40 |   newNode->key = key;
      |                ^ ~~~
main.c:42:11: error: cannot assign to variable 'p' with const-qualified type 'const Node *' (aka 'const struct Node *')
   42 |   p->next = newNode;
      |   ~~~~~~~ ^
```
  (output continues below the visible area; the frame ends here)
- [31:20] `lab $ cc main.c` running (tab title `...Wno-unused-parameter main.c`).
- [31:21] (also visible 31:44)
```
lab $ cc main.c
main.c:32:12: warning: assigning to 'char *' from 'const char *' discards qualifiers [-Wincompatible-pointer-types-discards-qualifiers]
   32 |     p->key = key;
      |            ^ ~~~
main.c:40:16: warning: assigning to 'char *' from 'const char *' discards qualifiers [-Wincompatible-pointer-types-discards-qualifiers]
   40 |   newNode->key = key;
      |                ^ ~~~
2 warnings generated.
lab $ 
```
- [32:05] screen cleared: `lab $ `
- [33:04]
```
lab $ cc main.c
lab $ 
```
  Clean compile. No program execution or output is visible in any frame of this segment after 33:04 (no `./a.out`).
- Vim messages: [31:28] `"main.c" 78L, 1355B written`; [32:08] `"main.c" 78L, 1361B written`; [32:32] `6 fewer lines`; [32:52] `8 fewer lines`; [33:12] `"main.c" 67L, 1184B written`.

## Observations

- Relation of the two files: the right file is a chained hash table (`Entry`, global `table[SIZE]`, SIZE 101, djb2 hash `5381`, `*33`, `put` that updates existing keys, `get` returning `int *`/NULL, `destroy`, strdup-style key copy). His own file is a simplified hand reproduction: SIZE 5, struct `Node`, hash = sum of bytes (`h += *c`), `set(Node *arr, ...)` with the first node embedded in the array and collisions chained through `next`, `get` returning int, "abc"/"cba" colliding (same byte sum) and "def" added. He mirrors the structure (struct with key/value/next, hash % SIZE, strcmp search loop, malloc for new node) but does not copy it: 2-space indent vs 4, `sizeof(Node)` vs `sizeof *e`, no malloc NULL-check, no key copy, no update-on-duplicate, no destroy.
- ORIGIN EVIDENCE: the only source evidence in this segment is the closing sticky: "we now understand hashtable and chatgpt code". So the right-hand code is, in his words, "chatgpt code". No ChatGPT/browser window is visible in this segment.
- Philosophy signals: final sticky note, verbatim: `all work , we now understand hashtable and chatgpt code you implement refactor of it for complex ` (reads as: all works; we now understand the hash table and the ChatGPT code; you (viewer) implement a refactor of it for a more complex version). Workflow: understand AI code by rebuilding a simpler version by hand, driven by compiler errors (const-correctness fixes: removed `const` from `p` in set(), added `const` to the struct key field).
- Final cleanups before the end: removed hand-built node literals in main(), replaced by `set(...)` calls; removed two explanatory comments (`// how we get access`, `// get arr "abc" , get arr "def"`), removed `// because abc and cba have same index`.
- Recording date from menu bar: Fri Oct 23, 2:36 PM.
- Last frame is 33:51 with the sticky still being edited (trailing space after "complex"); no frames show a program run of the final binary.
