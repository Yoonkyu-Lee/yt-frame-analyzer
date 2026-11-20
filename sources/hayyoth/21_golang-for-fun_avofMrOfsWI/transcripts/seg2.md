# Segment 2 (9:35 - 16:19), video avofMrOfsWI "a bit of Golang for fun"

Setup: macOS, VSCodium window "main.go — lab" (left), Firefox windows (go.dev tour, pkg.go.dev), a yellow Stickies note, webcam "Human" overlay (ignored). The editor does NOT show line numbers (gutter off).

## Timeline
- [09:35] Firefox in front on a YouTube page: a Hayyoth video "Dynamic Array. Conjured in C" (4:31 / 19:34, 3.2K views 2w ago, description "Building a generic dynamic array from scratch in C using malloc and realloc."). Its video frame shows a vim C file (INSERT, 12,7 Top) with `typedef struct { size_t len; size_t cap; int *data; } Slice;` etc. Behind it the VSCodium main.go (partially visible) and a second Firefox window showing the Go tour "slice-len-cap.go".
- [09:36] Firefox tab go.dev/tour/moretypes/7 "Slices" (7/27); example slices.go: `primes := [6]int{2, 3, 5, 7, 11, 13}`, `var s []int = primes[1:4]`, `fmt.Println(s)`.
- [09:37] Second Firefox window: go.dev/tour/moretypes/11 "Slice length and capacity" (11/27); code slice-len-cap.go lines 7-25 (`s = s[:0]`, `s = s[:4]`, `s = s[2:]`, `func printSlice(s []int) { fmt.Printf("len=%d cap=%d %v\n", len(s), cap(s), s) }`). Mouse selecting `len(s)`.
- [09:38-09:39] Back to the tour page 7/27 (slices) window; second window hidden; main.go partially visible left.
- [09:40] VSCodium brought to front, split editor: left main.go (Go), right main.c (C). Terminal at bottom, prompt `lab $` empty. Title "main.go — lab". See File states.
- [09:41] Firefox again front, tour page 7/27; main.go unchanged (comment line reads `// check my previous video for dynam...`).
- [09:53] Tour page go.dev/tour/moretypes/19 "Maps" (19/27): maps.go (Vertex struct with `Lat, Long float64`, `var m map[string]Vertex`, `m = make(map[string]Vertex)`, `m["Bell Labs"] = Vertex{40.68433, -74.39967,}`, `fmt.Println(m["Bell Labs"])`). macOS Dock visible, hovering "Stickies".
- [09:55] A new empty yellow sticky note appears at top-left (cursor in it).
- [10:10] Tour page go.dev/tour/moretypes/2 "Structs" (2/27); sidebar TOC "More types: structs, slices, and maps." list: Pointers, Structs, Struct Fields, Pointers to structs, Struct Literals, Arrays, Slices, Slices are like references to arrays, Slice literals, Slice defaults, Slice length and cap..., Nil slices, Creating a slice with..., Slices of slices, Appending to a slice, Range.
- [10:15] VSCodium front; cursor at end of line `import "fmt"` in main.go; right-hand Firefox shows structs.go (`type Vertex struct { X int; Y int }`, `fmt.Println(Vertex{1, 2})`, lines 5-8 highlighted).
- [12:29] VSCodium now shows Explorer sidebar "LAB": a.out, go.mod, main.c, main.go (1 error badge). main.go content now has `type Point struct` (see File states). Terminal: two `lab $ go run main.go` each printing `33`, then empty prompt. Firefox tour Structs page (2/27) in front, structs.go lines 1-13 with Vertex struct selected.
- [12:30] VSCodium front alone (title dimmed "main.go — lab"). File shows Point struct version.
- [14:18] Explorer now has a new folder `hello` containing `hello.go`; also a.out, go.mod, main.go (1 error). main.go changed to import "example.com/hello". Terminal shows an error. Firefox new empty tab: "Search with DuckDuckGo or enter address", Dock tooltip "Firefox".
- [14:21] main.go: `package main`, `import ( "example.com/hello" )`, `func main() { hello.sayHi() }` (sayHi red-squiggled). Terminal: `lab $ go run main.go` / `# command-line-arguments` / `./main.go:8:8: undefined: hello.sayHi` / `lab $`. Cursor after final `}`. The Point/Println body of main.go from 12:30 is gone (replaced between 12:30 and 14:18, not seen).
- [14:42] Firefox address bar: typing `pkg.go.dev/` with autocomplete suggestions (Go Packages, fs package - io/fs, Standard library - Go Packages, os package, io package, other unrelated suggestions).
- [14:45] pkg.go.dev home page: Go gopher with computer illustration, search box "Search packages or symbols", tip "Search for a package, for example "http" or "command"."
- [14:46-14:47] Scrolled to footer: FAQ ("How can I add a package?", "How can I remove a package?", "How can I add a go badge in my README file?", "How can I use the pkg.go.dev API?"); footer columns Why Go / Get Started (Playground, Tour, Stack Overflow, Help) / Packages (Standard Library, Sub-repositories, About Go Packages, pkg.go.dev API) / About (Download, Blog, Issue Tracker, Release Notes, Brand Guidelines, Code of Conduct) / Connect (Twitter, GitHub, Slack, r/golang, Meetup, Golang Weekly). Pointer over "Standard Library".
- [14:48] pkg.go.dev/std "Standard library": Version go1.27.3 Latest, Published Oct 20, 2026, License BSD-3-Clause, Repository cs.opensource.google/go/go; Directories list starts: archive (tar, zip)...
- [14:50] Directory list: bufio, builtin ("Package builtin provides documentation for Go's predeclared identifiers."), bytes, cmp, compress, container, context, crypto, database, debug, embed...
- [15:18] A yellow sticky note now at right with text: "Go is just C with garbage collector / and great library builtin" (cursor at end after "builtin").
- [15:19-15:37] Scrolling the std directory list: archive, bufio, builtin, bytes, cmp, compress, container, context, crypto, database, debug, embed, encoding, errors, expvar, flag, fmt ("Package fmt implements formatted I/O with functions analogous to C's printf and scanf."), go, hash, html, image, index, io, iter, log...
- [15:44] Clicked fmt: pkg.go.dev/fmt@go1.27.3, Imports: 11, Imported by: 5,643,267; Documentation > Overview (Printing, Explicit argument indexes, Format errors, Scanning). Sticky note now unfocused (no cursor).
- [15:45-15:46] Reading Overview text: "Package fmt implements formatted I/O with functions analogous to C's printf and scanf. The format 'verbs' are derived from C's but are simpler." Printing: "There are four families of printing functions defined by their output destination. Print, Println and Printf write to os.Stdout; Sprint, Sprintln and Sprintf return a string; Fprint, Fprintln and Fprintf write to an io.Writer; and Append, Appendln and Appendf append the output to a byte slice."
- [15:49-15:50] Firefox window resized/moved (dragging).
- [15:53-15:54] Opened the outline dropdown (Outline: Documentation, Source Files; Documentation: Overview, Index, Examples, Constants, Variables, Functions, Types; Examples: Package (Formats), Package (Printers), Errorf, Fprint, Fprintf, Fprintln, Fscanf, Fscanln, GoStringer, Print, Printf, Println, Sprint, Sprintf, Sprintln, Sscanf, Stringer; Functions: Append(b, a)...).
- [15:58-16:01] Chose Println: URL becomes pkg.go.dev/fmt@go1.27.3#example-Println. Page shows func Scan / Scanf / Scanln signatures; then func Println: `func Println(a ...any) (n int, err error)` "Println formats using the default formats for its operands and writes to standard output. Spaces are always added between operands and a newline is appended. It returns the number of bytes written and any write error encountered."
- [16:03-16:05] Reading Scan docs: `func Scan(a ...any) (n int, err error)`, `func Scanf(format string, a ...any) (n int, err error)` ("...The one exception: the verb %c always scans the next rune in the input, even if it is a space (or tab etc.) or newline."), `func Scanln(a ...any) (n int, err error)` ("Scanln is similar to Scan, but stops scanning at a newline and after the final item there must be a newline or EOF."), `func Sprint(a ...any) string`, `func Sprintf(format string, a ...any) string`.
- [16:07] Outline dropdown lists: Fscanln(r, a), Print(a), Printf(format, a), Println(a), Scan(a) (checked), Scanf(format, a), Scanln(a), Sprint(a), Sprintf(format, a), Sprintln(a), Sscan(str, a), Sscanf(str, format, a), Sscanln(str, a); Types: type Formatter, GoStringer, ScanState, Scanner, State, Stringer.
- [16:09] Dropdown closed; pointer on a Firefox toolbar button (history/back area).
- [16:12-16:19] Navigated back to pkg.go.dev/std; scrolling the directory list up and down (maps, math, mime, net, os, path, plugin...; back up to archive/bufio/builtin...). No edits.

## File states
main.go at 09:40-10:15 (split editor left pane; no editor line numbers; pane cut off at right edge so the comment is truncated):
```
 | package main            <- "package" red-squiggled
 |
 | import "fmt"            <- "fmt" red-squiggled
 |
 | // go [name] [type]
 | func main() {
 |     sl := []int{11, 22, 33}
 |     sl = append(sl, 44)
 |     sl = append(sl, 55)
 |     // check my previous video for dynamic arr⟨?⟩   (truncated at pane edge)
 |
 |     for i := 0; i < 5; i++ {
 |         fmt.Println(sl[i])
 |     }
 | }
```
main.c (right pane, 09:40):
```
 | #include <stdio.h>
 |
 | // c [type] [name]
 | int main(void) {
 |     int arr[3] = {11, 22, 33};
 |
 |     for (int i = 0; i < 3; i++) {
 |         printf("%d\n", arr[i]);
 |     }
 | }
```
main.go at 12:30 (Explorer open; top of file scrolled, `package main` not visible; no line numbers):
```
 | ?? (lines above not visible; presumably package main)
 | import "fmt"
 |
 | type Point struct {
 |     x int
 |     y int
 | }
 | // go [name] [type]
 | func main() {
 |     p1 := Point{11, 22}
 |
 |     fmt.Println(p1.x + p1.y)
 | }
```
Output of go run: `33`.

main.go at 14:21 (end-of-segment state):
```
 | package main
 |
 | import (
 |     "example.com/hello"
 | )
 |
 | func main() {
 |     hello.sayHi()
 | }
```
hello/hello.go: exists in Explorer (folder `hello`), contents NOT seen. Explorer at end: hello/hello.go, a.out, go.mod, main.go (error badge 1); main.c is not listed at 14:18.

## Shell & outputs
- [09:40-10:15] Terminal: `lab $` empty prompt.
- [12:29]
```
lab $ go run main.go
33
lab $ go run main.go
33
lab $
```
- [14:21]
```
lab $ go run main.go
# command-line-arguments
./main.go:8:8: undefined: hello.sayHi
lab $
```
Web pages (go.dev/tour/moretypes/7, /11, /19, /2; pkg.go.dev, /std, /fmt@go1.27.3) are described in the Timeline.

## Observations
- Sticky note (typed at 15:18) reads "Go is just C with garbage collector / and great library builtin": the author's thesis while browsing the standard library, reinforced by fmt docs noting its C printf/scanf analogy. A first, empty note was opened at 9:55.
- Comments frame a C-to-Go syntax mapping: `// go [name] [type]` vs `// c [type] [name]`; main.go and main.c kept side by side in a split editor, translating the same array/loop (11, 22, 33) between languages.
- A code comment references his previous video (Dynamic Array. Conjured in C): Go slice append equals a dynamic array.
- Learning order: Go tour (slices, maps, structs) -> own experiments (slice append, Point struct) -> module/package experiment (example.com/hello, `hello.sayHi()` fails: undefined, lowercase name is unexported) -> pkg.go.dev std library tour (fmt, Println/Scan families).
- Works from reference docs in Firefox; runs `go run main.go` in the VSCodium terminal; minimal editor chrome (no line numbers).
