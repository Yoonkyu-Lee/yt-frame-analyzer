# Segment 2 (01:04 - 03:30), 40 frames, all viewed

NOTE: No right-hand vim window appears in this segment. The origin of the hash-table code is revealed instead: a ChatGPT conversation titled "Hash table in C" in a browser (see Observations).

## Timeline
- [01:04] Terminal window large, tabs `~/lab — vim test.c` (active) and `~/lab — -zsh`. vim test.c shows lines 52-89: tail of `get()`, `destroy()`, `main()` (a full finished hash-table program, ~91 lines). Block cursor at line 52 col 1. Small sticky note at bottom-center (partly under webcam): `i try to understand hashtable`.
- [01:05] Scrolled: lines 61-91 visible, `~` filler after 91. Line 90 `destroy();`, line 91 `}`. Sticky same.
- [01:06] Scrolled back: lines 31-68; cursor on line 61.
- [01:08] Terminal unfocused (grey title buttons). Sticky is being dragged to mid-screen (collapsed bar) with text `i try to understand hashtable`. Cursor (hollow box) at line 61.
- [01:09] Sticky expanded to a large yellow note: `i try to understand hashtable` (text selected/highlighted).
- [01:18] Wallpaper visible: hash-table diagram (keys John Smith, Lisa Smith, Sam Doe, Sandra Dee, Ted Baker; bucket 254 -> Sam Doe 521-503x; 255 X) plus a Tower-of-Babel-like illustration. Terminal window now small and moved, showing test.c lines 45-65. Sticky now BLANK (text deleted). He does not point at the diagram.
- [01:19] Mouse moves to Dock ("Terminal" tooltip). Dock icons: Finder, Launchpad, a book app, a browser, Terminal, ?, QuickTime, ?, "exec".
- [01:20] Browser window in front: tab "Hash table in C", URL `chatgpt.com/c/6adb06a3-c714-83ec-adfa-c27ed0b4fa8b` (ChatGPT, avatar "HA"). Shows a code block labelled "C" with the end of main(): `if (value)` / `printf("%d\n", *value);  // 99` / `value = get("banana");` / `if (value)` / `printf("%d\n", *value);  // 20` / `destroy();` / `}`. Then text "Compile and run:" and a "Bash" code block. Input box "Ask ChatGPT". Blank sticky overlaid.
- [01:21] New tab ("New Tab"), address bar focused; status "Read chatgpt.com".
- [01:22] Typed `hayyoth.org`; status "Looking up hayyoth.org...".
- [01:23] Page loaded: tab "Hayyoth", `hayyoth.org`, logo (triangle with dot), heading "Hayyoth", tagline "The guardian of the craft of manual programming.", links "Youtube - Telegram - Codeberg - Email". Blank sticky.
- [01:24] Same; sticky moved to the right, blank.
- [01:26] Browser devtools opened, Console tab (tabs: Inspector, Console, Debugger, Network, Storage), empty.
- [01:27] Typed `const` with autocomplete (console, const, constructor, ConstantSourceNode); earlier output `ReferenceError: c is not defined` (from typing `c`).
- [01:28] Console shows `const` typed, no output.
- [01:43] Console:
  ```
  >> const myobj = {
       "name": "Cong",
       "age": 20
     }
  <- undefined
  >> myobj
  <- Object { name: "Cong", age: 20 }
  ```
- [01:44] Typing `myobj.n` with autocomplete list (__defineGetter__ ... age, constructor, hasOwnProperty, isPrototypeOf, name, propertyIsEnumerable, toLocaleString, toString, valueOf).
- [01:45] `myobj.name` -> `"Cong"`.
- [01:47] Typing `myobj.` autocomplete open.
- [01:48] `myobj.age` -> `20`.
- [01:50] `myobj.` autocomplete again.
- [01:51] `myobj.name =` being typed.
- [01:53] `myobj.name = "Nguyen"` -> `"Nguyen"`.
- [01:55] `myobj` typed, no result yet.
- [01:56] `myobj` -> `Object { name: "Nguyen", age: 20 }`.
- [02:01] Sticky (right side) now reads: `hashtable look like this in js` (being typed).
- [02:10] Back on ChatGPT tab (same URL, "Hash table in C"), same code block. Sticky text: `code simple version in C`. Mouse near copy icons of the code block.
- [02:11] Terminal over wallpaper showing test.c lines 45-65, status `61,1   62%`. Sticky: `code simple version in C`.
- [02:15] vim now status `"main.c" [New]   4,2   All`:
  ```
  1 #include <stdio.h>
  2
  3 int main(void) {
  4
  5   return 0;
  6 }
  ```
  Tab title still `~/lab — vim test.c`.
- [02:16] Terminal moved up-left; same content; more wallpaper visible.
- [02:18] Terminal resized taller (title "Terminal — 60×35"); same content `4,2 All`.
- [02:19] Mac menu bar visible (`Fri Oct 23 2:05 PM`). Same vim.
- [02:20] Control Center opened briefly (Wi-Fi "cong nguyen 2", Bluetooth Off, AirDrop Off, Focus...). Irrelevant.
- [02:21] Control Center closed; vim unchanged.
- [02:40] INSERT mode:
  ```
  4   int arr[5] = {0};
  6   arr[1] = 1|
  ```
  status `-- INSERT --  6,13  All`. Sticky: `code simple version in C`.
- [02:50] Lines 6-8 `arr[1] = 11;` `arr[2] = 22;` `arr[3] = 33;`; 10 `return 0;`, 11 `}`. Status `8,14 All`. Sticky being edited/dragged.
- [02:51] Same; sticky moved center-right, still `code simple version in C`.
- [03:02] Sticky text: `is this hashtable ?` / `this is hashtable ` (cursor at end).
- [03:12] Sticky text: `or`. Mouse I-beam near line 6.
- [03:30] INSERT mode, vim:
  ```
  4   int arr[255] = {0};
  6   arr['a'] = 11;
  7   arr['b'] = 22;
  8   arr['c'] = 33;
  10   printf("|
  ```
  status `-- INSERT --  10,11  All`. Sticky: `or`.

No `:w` save messages visible except initial `"main.c" [New]`.

## File states

### Left file (end of segment, ~03:30); buffer is main.c (tab title says test.c)
```
 1| #include <stdio.h>
 2|
 3| int main(void) {
 4|   int arr[255] = {0};
 5|
 6|   arr['a'] = 11;
 7|   arr['b'] = 22;
 8|   arr['c'] = 33;
 9|
10|   printf("⟨unfinished⟩
11|
12|   return 0;
13| }
```
2-space indentation. `arr[5]` -> `arr[255]` and numeric -> char indices happened between 03:12 and 03:30 (intermediate edits not seen).

Intermediate (02:50-03:12):
```
 1| #include <stdio.h>
 2|
 3| int main(void) {
 4|   int arr[5] = {0};
 5|
 6|   arr[1] = 11;
 7|   arr[2] = 22;
 8|   arr[3] = 33;
 9|
10|   return 0;
11| }
```
Initial skeleton (02:15): lines 1-6 `#include <stdio.h>`, blank, `int main(void) {`, blank, `  return 0;`, `}`.

Earlier in segment (01:04-02:11) `vim test.c` held the finished hash-table program (91 lines, 4-space indent). Visible parts:
```
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
Lines 1-30 never visible in this segment (?? struct Entry, SIZE, hash(), start of put()).

### Right-hand file
Not shown. The ChatGPT page shows the tail of main() identical to test.c lines 82-91.

### Browser console JS (01:43-01:56)
```
const myobj = {"name": "Cong", "age": 20}
myobj.name             -> "Cong"
myobj.age              -> 20
myobj.name = "Nguyen"  -> "Nguyen"
myobj                  -> Object { name: "Nguyen", age: 20 }
```

## Shell & outputs
- No zsh commands visible.
- [01:27] browser console: `ReferenceError: c is not defined`.
- [02:15] vim message: `"main.c" [New]`.

## Observations
- ORIGIN EVIDENCE (strong): at 01:20 and 02:10 the browser shows a ChatGPT conversation (`chatgpt.com/c/6adb06a3-c714-83ec-adfa-c27ed0b4fa8b`, tab "Hash table in C") whose C code block ends with the same main() as test.c lines 82-91, followed by "Compile and run:" and a Bash block. So the finished hash-table C code (Entry/put/get/destroy) came from ChatGPT and was in test.c. Sticky `code simple version in C` (02:10) reads like the prompt or goal: a simple C hash table.
- Sticky notes verbatim, chronological: `i try to understand hashtable` (01:04-01:09); blank (01:18-01:56); `hashtable look like this in js` (02:01); `code simple version in C` (02:10-02:51); `is this hashtable ?` / `this is hashtable ` (03:02); `or` (03:12-03:30). Sticky is used as live narration.
- Learning method: after showing the AI-generated reference, he starts a fresh buffer (`main.c [New]`) and rebuilds from first principles. Starts with a JS analogy: object `{name: "Cong", age: 20}` accessed and mutated by key ("hashtable look like this in js"); then in C builds from an array: `int arr[5] = {0}; arr[1]=11; arr[2]=22; arr[3]=33;` asks "is this hashtable?", then pivots ("or") to char indices: `int arr[255] = {0}; arr['a']=11; arr['b']=22; arr['c']=33;` and begins a printf. This is a direct-address table, the stepping stone toward hashing a key to an index.
- He opens his own site hayyoth.org (tagline "The guardian of the craft of manual programming.", links Youtube/Telegram/Codeberg/Email) only to get a page with a JS console. Name used: "Cong"/"Nguyen" (wifi "cong nguyen 2").
- Philosophy signals: tagline "craft of manual programming"; reads generated code, then rewrites manually from simpler analogies; minimal tools (vim, terminal).
- Wallpaper diagram visible but not pointed at. Menu bar date: Fri Oct 23, 2:05 PM.
- Tab title stayed "vim test.c" while vim status showed main.c (stale title or reused tab). The `-zsh` tab was unused here.
