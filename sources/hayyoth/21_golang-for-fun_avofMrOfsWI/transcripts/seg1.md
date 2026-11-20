# Segment 1 (0:00 - 9:35) - "a bit of Golang for fun" (Hayyoth)

## Timeline
- [00:00] Desktop (macOS-style, grey background). VSCodium window titled `lab`, empty workspace "LAB", watermark logo, welcome hints "Show All Commands / Open Settings / Toggle Terminal". A yellow sticky note floats on top: "It's been a year since I last wrote any Go code, and I've forgotten almost everything.\n\nIn this video, I'll do a quick refresher on Go and see what I can build with it."
- [00:30] Dock visible (Finder, Launchpad, ..., Firefox tooltip, Terminal, ...). Firefox opens, empty new window with DuckDuckGo search box focused. Sticky note now has an added section after `--`:
  "Learn philosophy of language don't learn syntax\n\nall you need is just go.dev websites"
- [00:33] Firefox URL bar autocompletes `gemini.google.com/` (suggestion list: Google Gemini; shortcuts Telegram Web, YouTube, ChatGPT, Google Translate). Not visited.
- [00:36] Typed search `golang x`. DuckDuckGo results: pkg.go.dev golang.org/x/text; "Go Wiki: X-Repositories - The Go Programming Language" (go.dev/wiki/X-Repositories); "Go (@golang) / X" (x.com/golang); "net module - golang.org/x/net". Sticky note still shown with text cursor at end of "websites" (he was editing it).
- [00:39-00:40] Sticky note hidden; scrolling results, hovering Go (@golang) / X.
- [00:41] Opened `x.com/golang`: Go profile, "Go will make you love programming again. We promise.", go.dev link, Joined February 2010, 0 Following, 213K Followers, 2,274 posts.
- [00:43-00:46] Zooming/scrolling the X page; browser window resized/moved left (URL x.com/golang).
- [00:51] Types `go.dev/` in URL bar (suggestion "The Go Programming Language go.dev").
- [00:53] Loads `go.dev/tour/`.
- [00:54] `go.dev/tour/welcome/1` - "A Tour of Go", "Hello, 世界", editor hello.go:
  `package main / import "fmt" / func main() { fmt.Println("Hello, 世界") }`; page 1/5.
- [00:55-00:56] Browser window shrunk/moved right (go.dev/tour/welcome/1), VSCodium visible behind.
- [01:06] Right-click context menu in VSCodium Explorer on `main.c` (files: `main.c`, `main.go` shown red = errors). Menu: Open to the Side, Open with Five Server, Open With..., Reveal in Finder, Open in Integrated Terminal, Select for Compare, Open Timeline, Cut, Copy, Copy Path, Copy Relative Path, Run Tests, Debug Tests, Rename..., Delete. Title `main.c - lab`.
- [03:20] Split editor: main.go (left) and main.c (right), terminal below. Title `main.go - lab`. Terminal shows `cc main.c` output with a warning/error (see Shell). Go code compares variable declaration styles with C (`var x1 int = 10`, `var x2 = 20`, `x3 := 30`).
- [04:49] Both files rewritten: Go `doubleVar(x int) int` vs C `int doubleVar(int x)`; comments `// go [name] [type]` and `// c [type] [name]` - author's point: Go puts name before type. Title `main.c - lab` (inactive window). Firefox showing tour welcome/1 on the right.
- [05:11] Firefox now at `go.dev/tour/moretypes/6` (Arrays): "The type [n]T is an array of n values of type T." `var a [10]int` "declares a variable a as an array of ten integers." "An array's length is part of its type, so arrays cannot be resized. This seems limiting, but don't worry; Go provides a convenient way of working with arrays." Code box array.go (`var a [2]string; a[0] = "Hello"; a[1] = "World"`).
- [07:01] main.go (left) edited to array demo; C counterpart on right (`int arr[3] = {11, 22, 33};` for loop). Mid-edit: line `var arr [3]int = [3]int{11, 22, 33}` (RHS selected, being deleted) with comment `// go can do this` above `arr := [3]int{11, 22, 33}` (red squiggle).
- [07:16] Terminal: `lab $ go run main.go` -> `11`, `22`, `33` (code reduced to `arr := [3]int{11, 22, 33}`). Firefox on `moretypes/6`, cursor on menu icon.
- [07:25] Firefox `go.dev/tour/moretypes/7` (Slices): `a[low : high]`, "This selects a half-open range which includes the first element, but excludes the last one.", `a[1:4]`. Code slices.go: `primes := [6]int{2, 3, 5, 7, 11, 13}; var s []int = primes[1:4]; fmt.Println(s)`; page 7/27.
- [07:46] main.go now: `sl := []int{11, 22, 33}` then `append(sl, )` with gopls tooltip: `append(slice []Type, elems ...Type) []Type` - "The append built-in function appends elements to the end of a slice. If it has sufficient capacity, the destination is resliced to accommodate the new elements. If it does not, a new underlying array will be allocated. Append returns the updated slice. It is therefore necessary to store the result of append, often in the variable holding the slice itself: slice = append(slice, elem1, elem2) / slice = append(slice, anotherSlice...)".
- [08:23] main.go (left pane partially covered by Firefox) now has `sl = append(sl, 44)`, `sl = append(sl, 55)`, comment `// check my previous video for dynamic array`, then loop `for i := 0; i < 5; i++ { fmt.Println(sl[i]) }`. Terminal cleared (`lab $`). Firefox still at moretypes/7.
- [08:25] Types `youtube.com/@` in Firefox URL bar; suggestions include the channel "Hayyoth - YouTube youtube.com/@hayyoth-org" and its /videos page (other suggestions are YouTube Studio pages and an unrelated channel; omitted).
- [08:28-08:29] Opens `www.youtube.com/@hayyoth-org`: channel Hayyoth, 18.5K subscribers, 20 videos, "The guardian of the craft of manual programming ...more", hayyoth.org. Featured video "I am Human, not AI. This is my story." 15:42, 107,496 views, 8d ago. "Customize channel / Manage videos" visible (owner's view).
- [08:30-08:37] `/@hayyoth-org/videos`; scrolling the video list (titles seen): "What AI cannot fake" 19K 10h ago; "pure Human beats AI slop" 25K 3d ago; "No AI - Just pure JS hand coding to relax - Web Editor & Files Manager" 1:28:00 27K 5d ago; "Pure HTML just works and beats AI Slop" 126K 5d ago; "Rewriting C String Library by Hand - K&R Style" 19:57 12K 7d ago; "macOS - but better - No Apple" 60K 7d ago; "vscode - but better" 5:39 92K 7d ago; "C is Flat. If everything is Flat, you can't go wrong." 30K 8d ago (sticky in thumbnail: "Flat / Understand this, and you won't hate C."); "I am Human, not AI. This is my story." 15:43 107K 8d ago; "No AI. Build ed (the standard text editor) in 100 lines of C" 1:10:04 39K 9d ago; "How we (Human) learn better than AI?" 11:38 27K 12d ago (thumbnail: "AI learn from complexity / Humans learn from simplicity."); "Hash Table. Conjured in C" 33:53 8.7K 12d ago; "Dynamic Array. Conjured in C" 19:35 (3.2K views, 2w ago); "Linked List. Conjured in C" 14:24 4.6K 2w ago.
- [08:37-08:38] Clicks "Dynamic Array. Conjured in C" (watch?v=SOjHybBunAc...), seeks to 15:33 / 19:34. Video frame shows terminal with `vim main.c` and `less man 3 memcpy` tabs; code of `sl_append` (see File states).
- [08:47-08:59] Zoomed view of the C dynamic array code (browser zoom): "HARD VERSION" with `Slice` struct and `Slice |` partially typed at line 12. Window being dragged. A second Firefox window opened ([08:54] blank DuckDuckGo search) and navigated to `go.dev/tour/welcome/1` ([08:55-08:59]) overlapping the first.
- [09:01] Go tour `moretypes/7` "Slices" with TOC menu open (Basics -> Packages, variables, and functions / Flow control statements: for, if, else, switch and defer / More types: structs, slices, and maps -> Pointers, Structs, Struct Fields, Pointers to structs, Struct Literals, Arrays, Slices (bold), Slices are like references to arrays, Slice literals, Slice defaults, Slice length and capacity, Nil slices, Creating a slice with make, Slices of slices).
- [09:06-09:15] Tour `moretypes/11` "Slice length and capacity": "A slice has both a length and a capacity. The length of a slice is the number of elements it contains. The capacity of a slice is the number of elements in the underlying array, counting from the first element in the slice. The length and capacity of a slice s can be obtained using the expressions len(s) and cap(s). You can extend a slice's length by re-slicing it, provided it has sufficient capacity..."; code slice-len-cap.go (see File states); page 11/27.
- [09:30-09:35] Back to the YouTube "Dynamic Array. Conjured in C" window (zoomed, line numbers visible, partly scrolled); paused at 4:31 / 19:34 showing the C source at vim `-- INSERT --  12,7  Top`. Video description: "Building a generic dynamic array from scratch in C using malloc and realloc."

## File states
### main.go (left pane, ~[03:20])
```
1| package main
2|
3| import "fmt"
4|
5| func main() {
6|     var x1 int = 10
7|     // go can do this
8|     var x2 = 20
9|     x3 := 30
10|
11|     fmt.Println(x1 + x2 + x3)
12| }
```
(Editor line numbers are NOT shown in this editor; numbered here by order. Red squiggle under `package main` and `"fmt"` - editor error, likely no go.mod.)

### main.c (right pane, ~[03:20])
```
1| #include <stdio.h>
2|
3| int main(void) {
4|     int x1 = 10;
5|     int x2 = 20;
6|     int x3 = 30;
7|
8|     printf(x1 + x2 + x3);
9| }
```
(Line 8 matches terminal message `main.c:8:12`.)

### main.go [04:49]
```
1| package main
2|
3| import "fmt"
4|
5| func doubleVar(x int) int {
6|     return x * x
7| }
8|
9| // go [name] [type]
10| func main() {
11|     var x1 int = 10
12|
13|     fmt.Println(doubleVar(x1))
14| }
```
### main.c [04:49]
```
1| #include <stdio.h>
2|
3| int doubleVar(int x) {
4|     return x * x;
5| }
6|
7| // c [type] [name]
8| int main(void) {
9|     int x1 = 10;
10|
11|     printf("%d\n", double⟨?⟩   ?? (rest cut off by Firefox window; presumably doubleVar(x1));)
12| }
```

### main.go [07:01 mid-edit]
```
1| package main
2|
3| import "fmt"
4|
5| // go [name] [type]
6| func main() {
7|     var arr [3]int = [3]int{11, 22, 33}      <- RHS "[3]int{11, 22, 33}" selected
8|     // go can do this
9|     arr := [3]int{11, 22, 33}
10|
11|     for i := 0; i < 3; i++ {
12|         fmt.Println(arr[i])
13|     }
14| }
```
### main.go [07:16] (after running)
```
1| package main
2|
3| import "fmt"
4|
5| // go [name] [type]
6| func main() {
7|     arr := [3]int{11, 22, 33}
8|
9|     for i := 0; i < 3; i++ {
10|         fmt.Println(arr[i])
11|     }
12| }
```
### main.c [07:01-07:46]
```
1| #include <stdio.h>
2|
3| // c [type] [name]
4| int main(void) {
5|     int arr[3] = {11, 22, 33};
6|
7|     for (int i = 0; i < 3; i++) {
8|         printf("%d\n", arr[i]);
9|     }
10| }
```

### main.go [07:46] mid-edit
```
1| package main
2|
3| import "fmt"
4|
5| // go [name] [type]
6| func main() {
7|     sl := []int{11, 22, 33}
8|     append(sl, )            <- cursor between ", " and ")"
9|
10|     for i := 0; ⟨?⟩ ... (hidden by tooltip)
11|         fmt.Pri⟨?⟩ ...
12|     }
13| }
```
### main.go [08:23] latest in segment (left ~12 columns hidden by Firefox overlay; reconstructed)
```
1| package main
2|
3| import "fmt"
4|
5| // go [name] [type]
6| func main() {
7|     sl := []int{11, 22, 33}
8|     sl = append(sl, 44)
9|     sl = append(sl, 55)
10|     // check my previous video for dynamic array
11|
12|     for i := 0; i < 5; i++ {
13|         fmt.Println(sl[i])
14|     }
15| }
```
(Visible text at that moment: `e main`, `"fmt"`, `[name] [type]`, `ain() {`, `:= []int{11, 22, 33}`, `= append(sl, 44)`, `= append(sl, 55)`, `check my previous video for dynamic array`, `r i := 0; i < 5; i++ {`, `fmt.Println(sl[i])`. Leading `sl`, `//`, `for` inferred.)

### Go Tour code boxes (not the author's files)
slices.go (moretypes/7):
```
1| package main
2|
3| import "fmt"
4|
5| func main() {
6|     primes := [6]int{2, 3, 5, 7, 11, 13}
7|
8|     var s []int = primes[1:4]
9|     fmt.Println(s)
10| }
```
slice-len-cap.go (moretypes/11, partial):
```
 1| package main
 2|
 3| import "fmt"
 4|
 5| func main() {
 6|     s := []int{2, 3, 5, 7, 11, 13}
 7|     printSlice(s)
 8|
 9|     // Slice the slice to give it zero length.
10|     s = s[:0]
11|     printSlice(s)
12|
13|     // Extend its length.
14|     s = s[:4]
15|     printSlice(s)
16|
17|     // Drop its first two values.
18|     s = s[2:]
19|     printSlice(s)
20| }
?? (lines 21+ not fully visible: func printSlice(s []int) { fmt.Printf("len=%d cap=%d %v\n", len(s), ... } )
```

### Author's earlier video code, "Dynamic Array. Conjured in C" (viewed in browser, NOT written in this video)
At 4:31 of that video (vim, INSERT, 12,7):
```
 1| #include <stdio.h>
 2| #include <stdlib.h>
 3|
 4| /* HARD VERSION */
 5|
 6| typedef struct {
 7|   size_t len; // such as table have 6 person
 8|   size_t cap; // such as table can have max 12 person
 9|   int *data; // array data
10| } Slice;
11|
12| Slice |
13|
14| int main(void) {
15|   size_t cap = 3;
16|   int *arr = malloc(sizeof(int) * cap);
17|   if (!arr) return 1;
18|
19|   arr[0] = 11;
20|   arr[1] = 22;
21|   arr[2] = 33;
22|
23|   for (size_t i = 0; i < cap; i++)
24|     printf("%d ", arr[i]);
25|   puts("");
26|
27|   // how we add 44
28|   cap++;
29|   int *tmp = realloc(arr, sizeof(int) * cap);
30|   if (!tmp) {
```
At 15:33 of that video (later version, lines 29-58 visible):
```
29|     return NULL;
30|   }
31|
32|   return sl;
33| }
34|
35| void sl_append(Slice *sl, void *val) {
36|   // on max size table
37|   // so, we need move to big table
38|   if (sl->len == sl->cap) {
39|     // you don't want call resize every time call append
40|     sl->cap = sl->cap * 2;
41|     int *tmp = realloc(sl->data, sl->elm_size * sl->cap);
42|     // add err handle
43|
44|     sl->data = tmp;
45|   }
46|
47|   //sl->data[sl->len] = val;
48|   memcpy(
49|     sl_get(sl, len),
50|     val,
51|     sl->elm_size
52|   );
53|
54|   sl->len++;
55| }
56|
57| void sl_free(Slice *sl) {
58|   free(sl->data);
```
(Vim tabs: `~/lab - vim main.c` and `~/lab - less man 3 memcpy`; status `52,4  52%`.)

## Shell & outputs
- [03:20] (terminal in VSCodium, prompt `lab $`)
```
lab $ cc main.c
/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/usr/include/_printf.h:34:36: note:
      passing argument to parameter here
   34 | int      printf(const char * __restrict, ...) __printflike(1, 2);
      |                                        ^
main.c:8:12: warning: format string is not a string literal
      (potentially insecure) [-Wformat-security]
    8 |     printf(x1 + x2 + x3);
      |            ^~~~~~~~~~~~~
main.c:8:12: note: treat the string as an argument to avoid this
    8 |     printf(x1 + x2 + x3);
      |            ^
      |            "%s",
1 warning and 1 error generated.
lab $
```
(The first error line, presumably an incompatible integer-to-pointer conversion, is scrolled off above; not visible. Caret positions approximate.)
- [07:16]
```
lab $ go run main.go
11
22
33
lab $
```
- [08:23] terminal cleared: `lab $`.
- Browser-rendered pages are described in the Timeline (X profile, Go Tour pages, DuckDuckGo results, YouTube channel page).

## Observations
- Sticky note thesis (written live at ~0:30): "Learn philosophy of language don't learn syntax / all you need is just go.dev websites". He opts for the official site (go.dev, Tour of Go) over third-party tutorials; a Gemini URL autocompleted but was not used; a search `golang x` led to the official Go X account, then go.dev -> Tour.
- Method: reading A Tour of Go and, side by side in a split VSCodium (main.go left, main.c right), re-expressing each concept in both Go and C to compare philosophies (type-after-name: `// go [name] [type]` vs `// c [type] [name]`; array `[3]int{...}` vs `int arr[3]`; `:=` short declaration; `var x1 int = 10` vs `var x2 = 20` vs `x3 := 30`; slices vs C dynamic array).
- Ordering: variables -> functions (`doubleVar`) -> arrays (moretypes/6) -> slices (moretypes/7, append, len/cap at moretypes/11).
- He relates Go slices to his own earlier C implementation ("Dynamic Array. Conjured in C", "HARD VERSION" `Slice{len, cap, data}`), leaving the comment `// check my previous video for dynamic array`. Showing his channel list (themes: "No AI", "pure Human beats AI slop", hand-coding, "C is Flat") is consistent with the channel's manual-programming ethos.
- Uses `cc main.c` and `go run main.go` from the integrated terminal; editor shows gopls hover docs for `append`. No go.mod visible (red squiggle on `package main` / import).
- No editor save messages seen in this segment (VSCodium, no checksums).
