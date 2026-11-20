# Segment 3 (03:30 - 06:11), 40 frames, all viewed

## Timeline
- [03:30] LEFT Terminal, tab `~/lab — vim test.c` active (other tab `-zsh`). INSERT mode, status `10,11`. Sticky note (yellow, top right) contains just "or". Code: `int arr[255] = {0};`, `arr['a'] = 11;` `arr['b'] = 22;` `arr['c'] = 33;`, line 10 being typed: `  printf("` (cursor after the quote), `return 0;` on line 12. No right-hand vim file visible anywhere in this segment.
- [03:40] Switched to `-zsh` tab. Shell: `lab $ cc test.c` / `lab $ ./a.out` / output `99` `20` (does not match visible source; earlier build).
- [03:48] Back in vim. Line 10 now `  printf("%d\n", arr['a']);`. Status: `<ain.c" [New] 13L, 155B written` (truncated; reads like `"main.c" [New] 13L, 155B written`, so saved as main.c though tab title says test.c). Normal mode, cursor 10,27. Sticky still "or".
- [03:56] Code rewritten: `int arr[5] = {0};` (255 -> 5; the "5" highlighted at 4,11), keys still `arr['a']`.. style lines shown as `arr['a'] = 11;` etc., printf removed (11 lines). Status `1 change; befo...31 seconds ago` (undo message). Sticky now EMPTY (the "or" is gone).
- [04:01] Now `arr[1] = 11;` `arr[2] = 22;` `arr[3] = 33;`; INSERT on empty line 10 (`10,3`), 13 lines.
- [04:34] Line 10 `  // can i do this`, line 11 `  arr[7 % 5] = 77;` selected in VISUAL (`-- VISUAL --  18  11,2`). 14 lines.
- [04:36] INSERT, typing `  pri` on line 13 (`13,6`); file 16 lines.
- [04:56] Lines 13-14 `printf("%d\n", arr[2]);` and `printf("%d\n", arr[7 % 5]);`, 17 lines. Status `"main.c" 17L, 215B written`, cursor 14,26 (the `5` highlighted). Zsh tab: `lab $ cc main.c && ./a.out` -> output `11` then prompt. (Odd: source would print 77 twice since 7%5==2; recorded as seen.)
- [05:00] Back to vim main.c (same state, 14,26).
- [05:06] Mouse over macOS Dock, Firefox tooltip.
- [05:07] Firefox opens: "New Tab".
- [05:09] Typing in address bar `wiki`; Firefox Suggest (history) list: The Creation of Adam - Wikipedia; Tower of Babel - Wikipedia (en.wikipedia.org/wiki/Tower_of_Babel); Hash table - Wikipedia (en.wikipedia.org/wiki/Hash_table); Pieter Bruegel the Elder - The Tower of Babel (Vienna); Hash_table_5_0_1_1_1_1_1_LL.svg.png (PNG image); Hash table 5 0 1 1 1 1 1 LL - Hash table - Wikipedia; The Last Supper (Leonardo) - Wikipedia; Vietnamese Last Supper page; File:Hash table 5 0 1 1 1 1 1 LL.svg.
- [05:10] `wikipedia `; [05:11] `wikipedia hashtabl` (suggestion "wikipedia hashtab").
- [05:12] Google results (`www.google.com/search?client=firefox-b-d&q=wikipedia+hashtable`): Wikipedia "Hash table" (en.wikipedia.org > wiki > Hash_table) and "Hash table - Simple English Wikipedia, the free encyclopedia"; "Thinking" (AI mode) loading.
- [05:13-05:14] Opens en.wikipedia.org/wiki/Hash_table. Visible intro: "In computer science, a hash table is a data structure that implements an associative array, also called a dictionary or simply map; an associative array is an abstract data type that maps keys to values. A hash table uses a hash function to compute an index, also called a hash code, into an array of buckets or slots, from which the desired value can be found. During lookup, the key is hashed and the resulting hash indicates where the corresponding value is stored. A map implemented by a hash table is called a hash map." Infobox: Invented 1953; Search/Insert/Delete Theta(1) avg, O(n) worst.
- [05:15-05:21] Fast scroll (about 1 s per screen): collisions/separate chaining/open addressing paragraph; "A small phone book as a hash table" figure; space-time tradeoff; Overview; Load factor; Load factor for open addressing (0.6 to 0.75); Hash function (h : U -> {0,...,m-1}); Integer universe assumption; Hashing by division ("division is the commonly used scheme"); multiplication h(x)=floor(m((xA) mod 1)), golden ratio; String hashing (polynomial rolling hash); Choosing a hash function; prime number sizes; K-independent hashing.
- [05:21] Stops at "Collision resolution": "A search algorithm that uses hashing consists of two parts. The first part is computing a hash function which transforms the search key into an array index. The ideal case is such that no two search keys hash to the same array index. However, this is not always the case and impossible to guarantee for unseen given data. Hence the second part of the algorithm is collision resolution. The two common methods for collision resolution are separate chaining and open addressing." Then "Separate chaining" with the diagram (keys / buckets / entries; John Smith, Lisa Smith, Sam Doe...).
- [05:24] Highlights heading "Collision resolution" (selected); Firefox translate icon popup.
- [05:25-05:26] Window-switch / Mission Control style animation (top-right thumbnail "Screen Recording 2026-10...0.00 PM"). Wallpaper hash-table diagram fully visible. POINTER: mouse on "John Smith" key (05:26); on bucket "152" (05:35-05:36), the red bucket where John Smith and Sandra Dee collide.
- [05:37] Windows restored, "Collision resolution" still highlighted.
- [05:40-05:41] Cursor to Firefox title bar; selection narrows to "Collision".
- [05:42] Firefox moved right; vim test.c visible at left with same 17-line code; mouse on line 8.
- [05:47] Vim in front, mouse at end of line 4; still `"main.c" 17L, 215B written`, 14,26. (Frame 05:47) INSERT mode: blank line inserted at top: cursor `2,1`; file 18 lines (line 2 empty cursor, line 3 empty, `int main(void) {` now line 4).
- [05:48] Line 3 now `//` (INSERT, `3,3`), file 19 lines (line 4 empty, main on 5).
- [06:06] After Esc/removal: line 3 empty, cursor block `3,0-1`, 18 lines (comment text gone). Mouse at line 15-16.
- [06:10-06:11] (first two frames are wallpaper, vim hidden) pointer at left of the arrows between keys and buckets (06:07-06:10 region ~ (285,395)), then on "Sandra Dee 521-9655" entry (06:10), i.e. the chained second entry under John Smith in bucket 152.
- [06:11] Vim shown again, line 3 empty, `3,0-1`; Firefox Wikipedia behind on the right.

Sticky notes: [03:30-03:48] one sticky, text "or" only. From 03:56 on the same sticky is empty. No other text.

## File states
### Left file (tab `~/lab — vim test.c`; vim saves as main.c) at END of segment (06:11)
```
 1| #include <stdio.h>
 2| 
 3| 
 4| int main(void) {
 5|   int arr[5] = {0};
 6| 
 7|   arr[1] = 11;
 8|   arr[2] = 22;
 9|   arr[3] = 33;
10| 
11|   // can i do this
12|   arr[7 % 5] = 77;
13| 
14|   printf("%d\n", arr[2]);
15|   printf("%d\n", arr[7 % 5]);
16| 
17|   return 0;
18| }
```
2-space indentation. Lines 2-3 blank (line 3 is where he typed and removed `//`).

Intermediate versions:
- 03:30 (13 lines): `int arr[255] = {0};` `arr['a'] = 11;` `arr['b'] = 22;` `arr['c'] = 33;` then `printf("` at line 10.
- 03:48: line 10 `printf("%d\n", arr['a']);` saved `13L, 155B`.
- 03:56 (11 lines): `int arr[5] = {0};` with arr['a'] etc., no printf.
- 04:01: `arr[1] = 11; arr[2] = 22; arr[3] = 33;`
- 04:34: `// can i do this` / `arr[7 % 5] = 77;`
- 04:56: plus printf lines; saved `"main.c" 17L, 215B written`.

### Right-hand file
NOT PRESENT in this segment. No second terminal with the hash table C file appears.

## Shell & outputs
- [03:40] `lab $ cc test.c` ; `lab $ ./a.out` ; `99` ; `20`
- [04:56] `lab $ cc main.c && ./a.out` ; `11`
- Vim messages: `<ain.c" [New] 13L, 155B written` (03:48); `1 change; befo...31 seconds ago` (03:56); `"main.c" 17L, 215B written` (04:56 onward).
- No compiler errors/warnings shown.

## Observations
- Learning path: direct-address array with char keys (`arr['a']`, size 255 = ASCII range; wallpaper bucket diagram also ends at 255), shrinks array to 5, uses small integer indices, then tests the core hash idea with a comment "can i do this": `arr[7 % 5] = 77;` (modulo maps an arbitrary key into array range). Verifies with printf of arr[2] and arr[7 % 5].
- Then to the web: Google "wikipedia hashtable" -> en.wikipedia.org/wiki/Hash_table. Firefox history shows earlier visits to Tower of Babel and the Hash_table_5_0_1_1_1_1_1_LL.svg image (the wallpaper diagram's source; Tower of Babel is the wallpaper art). Skims the whole article rapidly, stops at "Collision resolution" and selects it, then points at wallpaper diagram bucket 152 and the chained entries (John Smith -> Sandra Dee): collision/separate chaining is the next concept.
- Returns to vim and begins a comment line above main (`//`), then removes it.
- Origin of right-hand code: no evidence in this segment (not shown; no ChatGPT/AI page opened; only Google + Wikipedia in Firefox). Google AI Mode ("Thinking") appeared on results page but he did not use it.
- Philosophy: hands-on small experiments first, then authoritative reference reading; small numbers (5, 7) to make wraparound visible; doubts expressed as code comments; sticky note used lightly ("or", then cleared).
- Oddities: zsh outputs (`99`/`20`, `11`) don't match displayed source (stale builds); tab titled test.c but file saved as main.c.
