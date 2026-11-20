# Segment 1 (0:00 - 1:04) - 38 frames viewed

## Timeline
- [00:00] Full-screen wallpaper (hash-table diagram: keys John Smith / Lisa Smith / Sam Doe / Sandra Dee / Ted Baker; buckets 000,001,002,:,151,152(red),153,154,:,253,254,255; entries Lisa Smith 521-8976, John Smith 521-1234 -> Sandra Dee 521-9655 chained, Ted Baker 418-4165, Sam Doe 521-50xx; tower drawing). A collapsed yellow Stickies window (title bar only) sits over the diagram at about y=560; cursor on it. Webcam bottom-right. Cursor does not point at diagram entries.
- [00:01] Sticky is expanded: blank yellow note, text cursor in it.
- [00:08] Sticky rolled up, showing its text: `i try to understand hashtable` (lowercase).
- [00:10]-[00:11] Sticky dragged down to the bottom of the screen (y=815, then y=907), still rolled up. It stays at y=907 for the rest of the segment.
- [00:12] macOS Dock appears; cursor on Firefox icon (tooltip "Firefox"). Dock icons: Finder, Launchpad, Dictionary, Firefox, Terminal, (stone-like app), QuickTime, (copy icon), "exec", folder, Trash.
- [00:13] Firefox window (tab "New Tab"). Address bar typing `chatgpt.com/`; Firefox Suggest list: ChatGPT chatgpt.com, YouTube, Hayyoth - YouTube (youtube.com/@hayyoth-org), YouTube Studio channel content, Google Gemini (gemini.google.com), Google Translate, Channel analytics - YouTube Studio, Google Gemini (gemini.google.com/app), Home - Canva, YouTube Studio (history shows he also uses Gemini).
- [00:14] chatgpt.com loads: "Hey, hayyoth. Ready to dive in?" Prompt box "Ask ChatGPT". Logged in (avatar "HA").
- [00:17] Typing in the ChatGPT prompt: `hash table i` (in progress).
- [00:18] Prompt submitted: user bubble `hash table in C`. URL chatgpt.com/c/local-chatgpt%3A81717aeb-9a2b-4cf5-acd4-7efb... (loading). ORIGIN EVIDENCE: the C code he later reads is generated here by ChatGPT.
- [00:19] Tab title `hash table in C`; URL chatgpt.com/c/6adb06a3-c714-83ec-adfa-c27ed0b4fa8b. Answer streaming: "A hash table in ..."
- [00:20] Browser window moved/resized (left edge x~190). Tab title `Hash table in C`. Reply: "A hash table in C is a data structure that stores key-value pairs and lets you insert, search, and delete values efficiently. Average time complexity is O(1) for these operations. For example, you could store a mapping like this:" Table Key/Value: "apple" 10, "banana" 20, "orange" 30. "C doesn't have a built-in hash table, so you implement one yourself or use a library."
- [00:21]-[00:23] Scrolling; heading "1. How a hash table works".
- [00:25] He selects "key-value pairs" in the answer (tooltip "Ask ChatGPT" appears).
- [00:26] "1. How a hash table works": Step 1 - Key `"apple"` (The key you want to store.) -> Step 2 - Hash function `hash("apple") = 42` (An example hash value.)
- [00:27] Step 3 - Convert to array index: index = 42 mod 8 = 2; "The hash table has 8 slots, so the key maps to slot 2."
- [00:28] Step 4 - Store the entry: slots 0 "-", 1 "-", 2 `"apple" -> 10` (green), 3 "-", 4,5,6,7.
- [00:29] "2. Two common collision-handling methods. A collision happens when two different keys map to the same index." Separate chaining: "Each array slot holds a linked list or another collection of entries. When multiple keys map to the same index, they share that bucket. - Easy to grow beyond the number of buckets. - Requires extra memory for nodes or collections." The thumbnail beside it is the SAME diagram as his wallpaper (keys/buckets/entries, John Smith, Lisa Smith...).
- [00:30] Open addressing (e.g. linear probing): "All entries are stored directly in the array. If a slot is occupied, the algorithm searches for another available slot. - Often cache-friendly. - Needs capacity management and careful deletion" (streaming).
- [00:31] Scrolled up a bit: "The important idea is that a hash table uses an array for fast access, but a hash function determines where each key belongs."
- [00:33] "3. A simple hash table in C using separate chaining. This example maps string keys to integer values. It demonstrates insertion and lookup, with linked lists handling collisions." Code block (label "C") scrolling, showing put().
- [00:34] Code block: strcpy/e->value/e->next/table[i] = e lines, then `int *get(const char *key) {` (streaming; "return &e" partial).
- [00:35] Code block: destroy() tail and `int main(void) {` / `put("apple", 10);` (streaming).
- [00:36] Below code: "Bash" block header, "Output:", Plain text block: `99` / `20`. Heading "4. Understand..." streaming.
- [00:37] main() tail in ChatGPT: `if (value)` / `printf("%d\n", *value);  // 99` / `value = get("banana");` / `if (value)` / `printf("%d\n", *value);  // 20` / `destroy();` / `}`. Then "Compile and run:" and a Bash block (command text hidden behind input box, UNREADABLE).
- [00:40] Cursor on the yellow (minimise) traffic-light button of the Firefox window.
- [00:41] Firefox gone; wallpaper again; Dock visible, cursor near Firefox icon; Dock has an extra item (minimised-window thumbnail) next to the folder. Sticky still at bottom.
- [00:42] A fresh Terminal window (title "Terminal", beige theme), no tab bar, prompt `~ $ ` empty.
- [00:48] In this Terminal vim is open showing the pasted code: lines 1 and 2 empty (cursor on line 2, status `2,0-1`, message `92 more lines`), `3| #include <stdio.h>` ... lines 3-21 visible. So he pasted ChatGPT's code from clipboard into a new vim buffer (two leading blank lines at that moment).
- [00:49] Leading blank lines removed: `1| #include <stdio.h>`; status `1,1  Top`; lines 1-21 visible.
- [00:52] Terminal now shows tab bar: tabs `~/lab — vim test.c` and `~/lab — -zsh` (active) and `+`. On -zsh tab: `lab $ cc t` (typing; probably `cc test.c`).
- [00:57] Back on vim tab `~/lab — vim test.c`; 1,1 Top; lines 1-21. So the file is named test.c in ~/lab.
- [00:58] Terminal maximised to full screen; vim shows lines 1-38. Webcam overlay over lower right. Sticky visible at bottom.
- [01:01] Scrolled: lines 13-50 visible.
- [01:02] Lines 19-56 visible (cursor line 19).
- [01:03] Lines 46-83 visible (cursor line 46).
- [01:04] Lines 52-89 visible (cursor line 52); line 89 empty. He is scrolling/reading through the pasted file (no edits).
- No vim save messages (no "written"), no compiler output, no VISUAL-mode highlighting, and no other sticky notes in this segment.

## File states

### Left/main file: ~/lab/test.c (vim, tab "~/lab — vim test.c") as of 01:04
ChatGPT's code pasted (not edited in observed frames apart from removing two leading blank lines at 00:49). Lines 1-89 seen; 90+ NOT seen. Indent = 4 spaces.
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
??| (90+ unseen in vim; ChatGPT's version ends with `    destroy();` and `}` - inferred from the browser frame at 00:37)
```
Initial paste at 00:48 had two empty lines before `#include` (so `#include <stdio.h>` was line 3); removed by 00:49.

### Right-hand file
NOT PRESENT in this segment. No second terminal/vim file. The only code is the ChatGPT code in test.c (and in the browser).

## Shell & outputs
- [00:42] new Terminal: `~ $ ` (empty).
- [00:52] -zsh tab, ~/lab: `lab $ cc t` (partial, not executed in this segment).
- ChatGPT's expected output (from its answer, not his run): `99` then `20`.
- No `written` messages and no compiler errors in this segment.

## Observations
- Origin of code: unambiguously ChatGPT. At 00:13-00:18 he opens Firefox, goes to chatgpt.com (account "hayyoth"), prompt `hash table in C` (a very short prompt). Conversation "Hash table in C", URL chatgpt.com/c/6adb06a3-c714-83ec-adfa-c27ed0b4fa8b. Answer: definition (key-value pairs, avg O(1)), apple/banana/orange example, 4-step "how it works" (key -> hash("apple")=42 -> index = 42 mod 8 = 2 -> slot 2), two collision methods (separate chaining; open addressing/linear probing), then "3. A simple hash table in C using separate chaining" code, compile and run, output 99 / 20, and "4. Understand..." section.
- The wallpaper diagram is the same image ChatGPT embeds as its "Separate chaining" illustration (keys/buckets/entries; John Smith -> Sandra Dee chained in bucket 152). He is studying separate chaining.
- Sticky note (verbatim): `i try to understand hashtable` (only note in this segment); created 00:01, rolled up by 00:08, dragged to the screen bottom.
- He selects the phrase "key-value pairs" (00:25).
- Workflow: ask ChatGPT, skim explanation (scrolling 00:20-00:37), minimise browser, open new Terminal, vim, paste code as test.c in ~/lab, delete leading blank lines, open second tab (zsh) to compile (`cc t...`), maximise and scroll through code to read. No code is typed by him in this segment (copy/paste of AI output). Contrary to the brief, no right-hand reference file yet; the learning here starts with the top-down AI explanation (array, hash function, index = hash mod slots, collisions) before reading the code.
- Thesis from sticky: "i try to understand hashtable" - learning by reading a working AI-generated program.
- Webcam "Human" stays bottom-right (ignored).
