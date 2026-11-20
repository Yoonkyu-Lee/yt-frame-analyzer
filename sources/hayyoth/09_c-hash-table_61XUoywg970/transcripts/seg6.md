# Segment 6 (13:23 - 18:25), 39 frames

Note: the left tab title is `~/lab — vim test.c` but vim status messages say `"main.c"` and the shell compiles `main.c`. So the left file is effectively main.c (tab title stale).

## Timeline

- [13:23] LEFT: vim, status `"main.c" 39L, 485B written`, cursor 20,10, Top. RIGHT: vim VISUAL, hash function (lines ~15-22) highlighted. A yellow Sticky is open over the left window; being typed: `now refactor key to string`.
- [13:31] Sticky: `this hash version` / `we just simple ca` (typing).
- [13:54] Sticky:
  ```
  this hash version
  we just simple calculate sum of
  char

  like key "abc"
  = ('a' + 'b' + 'c') %
  ```
- [14:01] Last line now `= ('a' + 'b' + 'c') % size`, new empty line below.
- [14:08] Sticky adds `it simple than chatgpt but` (EVIDENCE of origin: he compares the right-hand hash to ChatGPT).
- [14:18] Sticky full text (top lines scrolled off):
  ```
  this hash version
  we just simple calculate sum of
  char

  like key "abc"
  = ('a' + 'b' + 'c') % size

  it simple than chatgpt but
  "abc" same "cba" or "acb"
  ```
- [14:30] Sticky text erased (empty).
- [14:33] Sticky moved to upper right (over right-hand window), empty, cursor blinking. Right file visible lines 1-23; lines 14-22 (`unsigned long hash...` through `}`) still highlighted. Webcam: cigarette.
- [14:34] Sticky rolled up to just its title bar (stays collapsed until 18:11).
- [14:36] RIGHT scrolled: lines 40-75 visible (VISUAL, cursor at line 40). Mouse I-beam near `if` on line 53. LEFT unchanged (top of file, 20,10).
- [14:40] LEFT scrolled (`75%`, top line 4), no status message. Line 20 `return h % SIZE;` with `h` highlighted (bracket/cursor).
- [14:44] LEFT at bottom (`Bot`), lines 7-39 plus `~` filler; cursor 20,10.
- [15:05] LEFT INSERT, cursor 34,22. Keys changed to strings:
  - 26: `arr[hash("abc")] = (Node) {`
  - 28: `.key = hash("abc"),`
  - 32: `arr[hash("def")].next = &(Node) {`
  - 34: `.key = hash("def"),`
  (Previously `hash(2)`, `hash(7)`.)
- [15:14] LEFT switched to `-zsh` tab; typing `./a` after two `cc main.c`.
- [15:16] Compile errors shown (see Shell).
- [15:19] Back to vim tab: status `"main.c" 42L, 504B written`, cursor 39,0-1, 85%. Blank lines inserted before `return 0;` (lines 37-40 blank, 41 `return 0;`). Keys still `hash("abc")`/`hash("def")`.
- [15:24] LEFT scrolled to top region (`3,3`, 28%), same content (lines 3-37).
- [15:47] LEFT at bottom (41,11 Bot): line 28 now `.key = "abc",`; line 34 still `.key = hash("def"),` (mid-fix).
- [15:53] LEFT on `-zsh` tab again, same error output still on screen (no new run seen).
- [15:56] LEFT vim: `"main.c" 42L, 492B written`, cursor 34,17 Bot. Both now `.key = "abc",` and `.key = "def",`.
- [16:13] LEFT INSERT, 39,29, 77%: new comment lines 38-39:
  ```
  // how we get access
  // get arr "abc" , get arr
  ```
- [16:19] Line 39 completed `// get arr "abc" , get arr "def"` (39,35). RIGHT scrolled lines 34-69, lines 34-40 highlighted (`Entry *e = malloc(sizeof *e);` ... `return;`). Webcam: cigarette.
- [16:20] RIGHT lines 43-78, no highlight; mouse near `e->next` line 52. Lines 74-78 main visible.
- [16:24] RIGHT: lines 49-57 (whole `get`) highlighted.
- [16:25] RIGHT: highlight gone, cursor block line 51; mouse at `key)` of line 50.
- [16:28] RIGHT: lines 53-57 highlighted (from `if (strcmp...` to `}`).
- [16:29] RIGHT: lines 49-57 highlighted again.
- [16:31] RIGHT: status `9 lines yanked`; cursor block at start of line 49. (He yanked `get` but never pastes it in this segment.)
- [16:34] LEFT INSERT, 24,1, 63%: two blank lines inserted between hash and main (22-24 blank; main now at 25).
- [17:24] LEFT: typed his own function header:
  ```
  23| int get(Node *arr, const char *key) {
  24|   |
  ```
  cursor 24,3, 58%.
- [17:28] RIGHT: line 50 `unsigned long i = hash(key);` highlighted.
- [17:30] RIGHT: only `key` in line 50 highlighted.
- [17:48] LEFT: `24|   size_t index = hash(key);`, cursor 25,3, 53%. RIGHT cursor at start of line 50 `unsigned`.
- [18:05] LEFT: header now `int get(const Node *arr, const char *key) {`; line 25 `const Node = arr[index];` (no variable name, as typed), cursor 26,3, 50%.
- [18:11] Sticky unrolled at upper right, empty, cursor; mouse on title bar.
- [18:20] Sticky typing: `if you not understand linked list ,` / `check my `.
- [18:23] Sticky: `if you not understand linked list ,` / `check my previous video`. Dock appears at bottom (Finder, Launchpad, a book icon, Firefox, Terminal, ..., an `exec` icon); tooltip `camera` over a dock icon.
- [18:24] Firefox window opens: tab `New Tab`, address bar `Search with Google or enter address`; dock tooltip `Firefox`. Sticky stays on top.
- [18:25] Typing `y` in Firefox address bar (heading to YouTube).

## File states

### LEFT: main.c (tab "vim test.c") at end of segment (18:25)

Lines 1-7 not on screen late; known from 13:23 (unchanged).

```
 1| #include <stdio.h>
 2| 
 3| #define SIZE 5
 4| 
 5| typedef struct Node {
 6|   char *key;
 7|   int value;
 8|   struct Node *next;
 9| } Node;
10| 
11| size_t hash(const char *key) {
12|   size_t h = 0;
13| 
14|   const char *c = key;
15|   while (*c) {
16|     h += *c;
17|     c++;
18|   }
19| 
20|   return h % SIZE;
21| }
22| 
23| int get(const Node *arr, const char *key) {
24|   size_t index = hash(key);
25|   const Node = arr[index];
26|   ⟨INSERT cursor, empty line, no closing brace yet⟩
27| 
28| int main(void) {
29|   Node arr[SIZE] = {0};
30| 
31|   arr[hash("abc")] = (Node) {
32|     .value = 22,
33|     .key = "abc",
34|     .next = NULL
35|   };
36| 
37|   arr[hash("def")].next = &(Node) {
38|     .value = 77,
39|     .key = "def",
40|     .next = NULL
41|   };
42| 
43|   ?? (below: blank lines, `// how we get access`, `// get arr "abc" , get arr "def"`, blank lines, `return 0;`, `}`; exact numbers not seen after line 42)
```
Indent: 2 spaces.

Intermediate versions:

(a) 13:23-14:44 (39L, 485B), integer keys:
```
23| int main(void) {
24|   Node arr[SIZE] = {0};
25| 
26|   arr[hash(2)] = (Node) {
27|     .value = 22,
28|     .key = hash(2),
29|     .next = NULL
30|   };
31| 
32|   arr[hash(7)].next = &(Node) {
33|     .value = 77,
34|     .key = hash(7),
35|     .next = NULL
36|   };
37| 
38|   return 0;
39| }
```
(b) 15:05: `hash("abc")` / `hash("def")` in both array index and `.key` (compile errors).
(c) 15:19: 42L 504B (blank lines added: 37-40 blank, 41 `return 0;`, 42 `}`).
(d) 15:56: 42L 492B, `.key = "abc",` / `.key = "def",`.
(e) 16:13-16:19: comments at lines 38-39 (`// how we get access`, `// get arr "abc" , get arr "def"`).
(f) 17:24 `int get(Node *arr, const char *key) {` added at 23.
(g) 17:48 `size_t index = hash(key);`.
(h) 18:05 `const Node` in params, `const Node = arr[index];`.

### RIGHT-HAND FILE (second vim, reader)

No file name visible (window title just "Terminal"; status line only `-- VISUAL --`).

```
 1| #include <stdio.h>
 2| #include <stdlib.h>
 3| #include <string.h>
 4| 
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
24| void put(const char *key, in⟨?⟩ (occluded by webcam)
25|     unsigned long i = hash(k⟨?⟩
26| 
27|     for (Entry *e = table[i]; ⟨?⟩
28|         if (strcmp(e->key, ⟨?⟩
29|             e->value = value⟨?⟩
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
```
Indent 4 spaces. Lines after 78 not seen. Line 24-29 right side partially hidden.

## Shell & outputs

- [15:14] zsh tab:
  ```
  lab $ cc main.c && ./a.out
  11
  lab $ cc main.c && ./a.out
  77
  77
  lab $ cc main.c
  lab $ cc main.c
  lab $ ./a
  ```
- [15:16] Result of the last compile:
  ```
  main.c:28:12: error: incompatible integer to pointer
        conversion initializing 'char *' with an expression of
        type 'size_t' (aka 'unsigned long') [-Wint-conversion]
     28 |     .key = hash("abc"),
        |            ^~~~~~~~~~~~
  main.c:34:12: error: incompatible integer to pointer
        conversion initializing 'char *' with an expression of
        type 'size_t' (aka 'unsigned long') [-Wint-conversion]
     34 |     .key = hash("def"),
        |            ^~~~~~~~~~~~
  2 errors generated.
  lab $
  ```
- Vim saves: 13:23 `"main.c" 39L, 485B written`; 15:19 `"main.c" 42L, 504B written`; 15:56 `"main.c" 42L, 492B written`.
- Right pane 16:31: `9 lines yanked`.

## Observations

- Origin of right-hand code: strong evidence ChatGPT. Sticky (14:08/14:18): `it simple than chatgpt but` / `"abc" same "cba" or "acb"`, written while the right-hand djb2 hash is highlighted and compared with his sum-of-chars hash. No file name/website/tab for the right file is visible in this segment. Only browser: Firefox `New Tab`, typing `y` (YouTube) at 18:24-18:25 to show his previous linked-list video.
- Stickies verbatim: (1) `now refactor key to string`; (2) `this hash version / we just simple calculate sum of / char / (blank) / like key "abc" / = ('a' + 'b' + 'c') % size / (blank) / it simple than chatgpt but / "abc" same "cba" or "acb"`; (3) erased; (4) `if you not understand linked list , / check my previous video`.
- Learning style: builds his own minimal version first (sum-of-chars hash, Node with key/value/next, `Node arr[SIZE]` plus chained compound literals), then reads the ChatGPT reference (djb2, Entry, put/get/destroy, malloc'd keys, strcmp/strcpy) in the other window and compares. He spots a weakness in his own hash (anagram collisions) while noting the reference hash is more complex.
- Relation: his `Node{char *key; int value; Node *next}` mirrors `Entry`; he reads right-hand `get` closely (highlights whole function, `unsigned long i = hash(key)`, the strcmp loop; yanks 9 lines but does not paste) and then types his own `get` from scratch with differences (`size_t index`, `const Node *arr` parameter, array of Node values not Entry* pointers, SIZE 5 vs 101, 2-space vs 4-space indent). Typing, not copy-paste.
- Refactor via compiler feedback: int keys -> string keys; `-Wint-conversion` error teaches that `.key` must be the string and `hash()` is only the index; fixed to `.key = "abc"`.
- Process signals: iterative, error-driven; writes plan comments in code (`// how we get access`); uses Stickies as narration; references earlier video in a series (linked list). Wallpaper hash-table diagram not pointed at in this segment.
