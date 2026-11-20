# Segment 7 (18:25 - 20:18), 40 frames

## Timeline
- [18:25] Left Terminal (`~/lab — vim test.c`, tab 2 `~/lab — zsh` clipped), vim INSERT, `26,3  50%`, cursor on empty line 26 inside `get()`. A Firefox window ("New Tab") is open on top, overlapping the right terminal; address bar being typed: `y`. Yellow sticky (top right) verbatim: "if you not understand linked list , check my previous video". Right-hand terminal visible only at bottom: lines 75-78 `put("apple", 10);` `put("banana", 20);` `put("orange", 30);` `put("apple", 99);`.
- [18:26] URL bar autocomplete: `youtube.com/@hayyoth-org` (suggestion "Hayyoth - YouTube"; Firefox Suggest also lists Tsoding, Gabe Sampson, Hayyoth/videos, Computerphile, Brett Codes, Mot Soi Tranh, Magicalbat, runofff).
- [18:27] Loading `www.youtube.com/@hayyoth-org`.
- [18:28-18:30] Firefox window moved left/resized (tab "youtube.com/@hayyoth-org"); page loading dark, then `?themeRefresh=1`. Status bar: "Read fonts.gstatic.com", "Read www.google.com", "Transferring data from www.youtube.com...".
- [18:31] Channel page "Hayyoth - YouTube": "@hayyoth-org • 64 subscribers • 8 videos", "The guardian of the craft of manual programming ...more", hayyoth.org, "Customize channel", "Manage videos"; tabs Videos/Posts. Videos: "Dynamic Array", "Linked List". On webcam he holds a cigarette.
- [18:32-18:34] Scrolled: "The Dynamic Array. Conjured in C" (19:35, 53 views, 1d ago); "The Creation of Chain - Linked List. Conjured in C" (14:24, 57 views, 2d ago). Hover link `www.youtube.com/watch?v=iz2j9_AwQdQ&t=336s`.
- [18:36] Firefox hidden; both terminals visible. Left unchanged. Right terminal (title `Terminal`, window at x~770): vim lines 43-78 of the hash-table file (see File states). Right-hand vim cursor block at line 50 (`unsigned`). Sticky unchanged. Mouse near left line 23.
- [18:41-18:42] Mouse on macOS Dock hovering Firefox ("Firefox" tooltip). Left tab `~/lab — vim test.c` active at 18:41.
- [18:43] Firefox "New Tab" reappears. [18:44] types `youtube.com/@hayyoth-org` again (autocomplete). [18:45] loading.
- [18:47] Channel page again (Hayyoth, 64 subs, 8 videos). [18:48] scrolled, hovering Linked List video (same watch URL t=336s). [18:49] clicked: `www.youtube.com/watch?v=iz2j9_AwQdQ&t=336s`, player 5:36 / 14:24.
- [18:50] Firefox maximized; tab "The Creation of Chain - Linked ..."; "58 views 2 days ago"; description "Linked List data structure. Lisp makes it look easy, but in C, you have to build the whole world from scratch!"; channel Hayyoth 64 subscribers, buttons Analytics, Edit video (he is the owner). Sticky still on top.
- [18:51-18:53] Video plays at 5:36-5:37; goes full screen ("youtube.com is now full screen"). Video shows HIS OWN earlier linked-list video: left Terminal `~/lab — vim main.c`, right Terminal running Chez Scheme (`lab $ # lisp list`, `lab $ chez`, `Chez Scheme Version 10.4.1`, `Copyright 1984-2026 Cisco Systems, Inc.`, `> (define my-list '(10 20 30))`, `> my-list`, `(10 20 30)`, `> (append my-list '(40 50))`, `(10 20 30 40 50)`). Scrub preview at 11:06 (18:52).
- [18:53-18:57] Fullscreen paused at 5:38-5:39: `append(` typed at line 32 of his linked-list main.c (see File states). Scrub previews at 4:08 (18:55) and 5:57 (18:56). Mouse over line 20 `p->next = create(val);` (18:57).
- [19:10-19:13] exits fullscreen to normal Firefox page (still 5:39 / 14:23). [19:14-19:15] window shrinks and slides right over the right-hand terminal. [19:16-19:18] video zoomed (browser/theater zoom) to read code; mouse on line 19 `while (p->next) p = p->next; // go to last node` (19:16), then on `for` in printList line 24 (19:19).
- [19:19] Firefox slid to the right, left terminal in focus (26,3); mouse I-beam over `hash(key)` / `arr[index]` lines 24-25 (comparing his get() with the traversal).
- [19:21-19:22] Firefox on right shows video code lines 9-37 (`append(` at 32). Left terminal in focus.
- [19:56] Left unchanged (26,3).
- [20:16] LEFT FILE EDITED: line 25 now `const Node p = arr[index];` (was `const Node = arr[index];`), new line 26: `for (; p; p = p->next)`, line 27 empty indented (`27,4  46%`). Following lines shift +1 (main now line 29). Firefox still behind showing his linked-list video.
- [20:18] Line 26 now `for (; p; p = p.next)`; line 27: ` if (p.key ⟨cursor box⟩`, status `27,14  46%`. Right-hand terminal brought to front (vim lines 43-78), cursor block still on line 50 `unsigned`. In the last frame (20:18) the sticky note has been dragged lower (title bar y~520) uncovering right-hand lines 43-57 fully; mouse over line 54 `return &e->value;`. Last frame: right-hand get() fully readable.

No vim save messages, no shell commands, no compiler output in this segment.

## File states

### Left file: test.c (as of 20:18; only lines 8-42 seen; lines 1-7 not seen in this segment)
```
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
25|   const Node p = arr[index];
26|   for (; p; p = p.next)
27|    if (p.key ⟨?⟩            <- cursor 27,14; text after "p.key " not yet typed
28|
29| int main(void) {
30|   Node arr[SIZE] = {0};
31|
32|   arr[hash("abc")] = (Node) {
33|     .value = 22,
34|     .key = "abc",
35|     .next = NULL
36|   };
37|
38|   arr[hash("def")].next = &(Node) {
39|     .value = 77,
40|     .key = "def",
41|     .next = NULL
42|   };
```
Indent 2 spaces; line 27 indent is 1 space (` if`). Intermediate at 20:16: line 26 `for (; p; p = p->next)` (arrow), changed to `p.next` by 20:18. get() has no closing brace yet.

Earlier version (18:25-19:56): 23 `int get(const Node *arr, const char *key) {`, 24 `size_t index = hash(key);`, 25 `const Node = arr[index];`, 26 empty (cursor 26,3), 27 blank, 28 `int main(void) {`, 29 `Node arr[SIZE] = {0};`, 30 blank, 31-35 abc init, 36 blank, 37-41 def init, 42 blank.

### Right-hand file (vim in second Terminal; lines 43-78, unchanged all segment)
```
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
Lines 43-48 and 49-57 right portion were partly hidden by the sticky until 20:18 (e.g. earlier seen `int *get(const char *key` , `unsigned long i = ha`, `for (Entry *e = tabl`). Cursor block on line 50 col 5. Indent 4 spaces.

### His own linked-list video (browser; NOT the right-hand file) main.c at 5:39 of that video
```
 6|   struct Node *next;
 7| } Node;
 8|
 9| Node *create(int val) {
10|   Node *p = malloc(sizeof Node);
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
32|   append(
33|
34|   printList(&n1);
35|
36|   return 0;
37| }
```

## Shell & outputs
None. Only browser URLs: `youtube.com/@hayyoth-org`, `https://www.youtube.com/watch?v=iz2j9_AwQdQ&t=336s` (title "The Creation of Chain - Linked List. Conjured in C"). Chez output in his old video: `Chez Scheme Version 10.4.1`.

## Observations
- Single sticky, unchanged throughout: "if you not understand linked list , check my previous video". It explains the browser trip: he shows his own channel (Hayyoth, 64 subs, 8 videos) and his earlier Linked List (and Dynamic Array) video, jumping to t=336s where he writes append/printList traversal. This is a prerequisite pointer for viewers.
- He then returns to the left test.c and immediately writes bucket-chain traversal in get() modelled on that printList loop: `const Node p = arr[index]; for (; p; p = p->next)` then (typo/changes) `p.next`, `if (p.key ...` (about to compare keys). So he derives his get() from his own linked-list knowledge. At 20:18 he brings the right-hand terminal forward, moves the sticky, and mouses over its get() lines 53-54 (`strcmp(e->key, key) == 0`, `return &e->value;`), apparently to check how to compare keys. No visual highlighting mode in this segment.
- Origin evidence for right-hand code: none new. No ChatGPT/AI window or site. Only external page is his own YouTube channel. Right file style (4-space indent, `unsigned long`, `Entry`, `table`, `strcpy`, `destroy`) differs from his 2-space `Node`/`arr`/`size_t` style. Its window title is just "Terminal"; no filename visible.
- Philosophy signals: treats prior videos as prerequisites; builds hash table by composing earlier structure (linked list chain per bucket); uses reference code only to cross-check.
- Potential bugs in his code: `const Node p` is a value not pointer, `p.next` vs `p->next`; get() lacks closing brace so far; wallpaper not pointed at in this segment.
