# Segment 8 of 20 (27:16 - 31:26) - VBORE_zH5oc

Note: this segment is NOT the JS web editor. The author has started a side project, "go-filemanager" (Go language), in a Codium-style editor (VSCodium/Codium on macOS, launched with `codium .`). Webcam overlay "Human" ignored.

## Timeline
- [27:16] Editor window titled `main.go — go-filemanager`. Explorer sidebar "GO-FILEMANAGER" with single file `main.go`. Editor is empty (cursor on line 1). Bottom panel: TERMINAL tab, prompt `go-filemanager $ `.
- [27:39] `main.go` now contains (skeleton, typed by hand): `package main`, blank, `func main()  {` (two spaces before `{`), body line `    fmt` (incomplete), `}`. Bottom panel switched to OUTPUT tab (dropdown "Go") showing Go extension log lines (timestamps 2026-10-30 20:56:50 / 20:57:04): "Installing 1 tool at the configured GOBIN: /Users/<user>/go/bin", "gopls", "Tools environment: GOPATH=/Users/<user>/go, GOBIN=/Users/<user>/go/bin, GOTOOLCHAIN=auto". i.e. the Go extension is trying to install gopls.
- [27:40-27:41] Switches to a separate macOS Terminal window (Dock tooltip "Terminal"). Visible history: `~ $ cd lab` / `lab $ ls` -> `web-editor` (cyan, a directory) / `lab $ mkdir go-filemanager` / `lab $ cd go-filemanager` / `go-filemanager $ codium .` / `go-filemanager $ `.
- [27:55] Same Terminal now shows: `go-filemanager $ cd` / `~ $ e .zshrc` / `~ $ gopls` / `zsh: command not found: gopls` / `~ $ `. (He edited .zshrc with an `e` command to fix PATH for gopls, then tested `gopls`; still not found.)
- [27:56] Back to Codium; `fmt` selected (highlighted) in main.go; OUTPUT panel still shows gopls install log.
- [29:27] (Jump ahead ~1m30s.) Codium: sidebar now shows `go.mod` and `main.go`. main.go is a complete small program (see File states). Terminal tab: `go-filemanager $ go run .` -> `30`, then fresh prompt. (`import "fmt"` has a red squiggle under `fmt`.)
- [29:28-29:29] Switches to Terminal window (still `go-filemanager $ cd` / `~ $ e .zshrc` / `~ $ gopls` / `zsh: command not found: gopls` / `~ $ `), then empty desktop, Dock with tooltip "Stickies".
- [29:31] Opens Firefox (Dock tooltip "Firefox"), blank window with URL bar "Search with DuckDuckGo or enter address".
- [29:32] Types `pkg` in URL bar; Firefox Suggest list shows public items "Go Packages - Go Packages pkg.go.dev", "fs package - io/fs - Go Packages pkg.go.dev/io/fs@go1.27.3#ReadFile", "io package - io - Go Packages pkg.go.dev/io@go1.27.3", plus unrelated history (private mail inbox, spreadsheets, YouTube titles incl. "No AI, 52 min. Build all Linux Files utils in ..." - the author's earlier video).
- [29:34] Navigates `http://pkg.go.dev` ("Waiting for pkg.go.dev..."). [29:35] pkg.go.dev home loads: Go gopher image, "Search packages or symbols", Tip text, "Take the 2026 Go Developer Survey" banner.
- [29:39-29:42] Scrolls down to FAQ ("Frequently asked questions: How can I add a package? ...") and footer (Why Go / Get Started / Packages / About / Connect columns). Hovers "Standard Library" (status bar pkg.go.dev/std).
- [29:45-29:46] Clicks Standard Library -> `pkg.go.dev/std` "Standard library", Version go1.27.3 Latest, Published Oct 20, 2026, License BSD-3-Clause.
- [29:47-29:53] Scrolls package list: archive (tar, zip), bufio, builtin, bytes, cmp, compress, container, context, crypto, database, debug, embed, encoding, errors, expvar, flag, fmt ("Package fmt implements formatted I/O with functions analogous to C's printf and scanf."), go, hash, html, image, index, io ("Package io provides basic interfaces to I/O primitives."), iter, log, maps, math, mime, net, os... Hovers fmt link (pkg.go.dev/fmt@go1.27.3). Expands `io`: sub-packages `fs` ("Package fs defines basic interfaces to a file system.") and `ioutil` ("Package ioutil implements some I/O utility functions.").
- [29:55] Opens `pkg.go.dev/io/fs@go1.27.3`: "fs package standard library", Version go1.27.3 Latest, Published Oct 20, 2026, License BSD-3-Clause, Imports: 8, Imported by: 47,376. Documentation > Overview: Path Names, Testing.
- [29:56-30:01] Reads Overview text (Path Names: "Path names are UTF-8-encoded, unrooted, slash-separated sequences of path elements, like "x/y/z". Path names must not contain an element that is "." or ".." or the empty string, except for the special case that the name "." may be used for the root directory. Paths must not start or end with a slash: "/x" and "x/" are invalid."; Testing: "See the testing/fstest package for support with testing implementations of file systems."). Firefox window is dragged wider (29:59-30:01) until it is full screen.
- [30:02] Full-width layout with left "Documentation" nav (Overview, Index, Examples, Constants, Variables, Functions, Types, Source Files). Index lists: Variables; func FormatDirEntry(dir DirEntry) string; func FormatFileInfo(info FileInfo) string; func Glob(fsys FS, pattern string) (matches []string, err error); func ReadFile(fsys FS, name string) ([]byte, error); func ReadLink(fsys FS, name string) (string, error); func ValidPath(name string) bool; func WalkDir(fsys FS, root string, fn WalkDirFunc) error; type DirEntry (func FileInfoToDirEntry(info FileInfo) DirEntry; func ReadDir(fsys FS, name string) ([]DirEntry, error)); type FS (func Sub(fsys FS, dir string) (FS, error)); ...
- [30:04-30:05] Expands Functions in nav (URL #pkg-functions). Reads FormatDirEntry (added in go1.21.0; sample output `d subdir/` / `- hello.go`), FormatFileInfo (sample `-rw-r--r-- 100 1970-01-01 12:00:00 hello.go`), Glob.
- [30:07-30:09] Clicks ReadFile(fsys, name) (#ReadFile): "func ReadFile(fsys FS, name string) ([]byte, error). ReadFile reads the named file from the file system fsys and returns its contents. A successful call returns a nil error, not io.EOF. ..." Then ReadLink (added in go1.25.0), ValidPath.
- [30:13] Jumps to `type FS` (#FS): `type FS interface { // Open opens the named file. ... Open(name string) (File, error) }` - "An FS provides access to a hierarchical file system. The FS interface is the minimum implementation required of the file system..." Types nav: DirEntry, FS (Sub), File, FileInfo, FileMode, GlobFS, PathError, ReadDirFS, ReadDirFile...
- [30:16] Scrolls to `type DirEntry interface` with doc comments: Name() string, IsDir() bool, Type() FileMode, Info() (FileInfo, error).
- [30:17-30:20] Scrolls back up through Functions, Index (type FileInfo: Lstat, Stat; type FileMode: IsDir, IsRegular, Perm, String, Type; type GlobFS; type PathError; ReadDirFS, ReadDirFile, ReadFileFS, ReadLinkFS, StatFS, SubFS, WalkDirFunc), Examples (Glob, ReadFile, ValidPath, WalkDir), Constants ("This section is empty."), Variables (ErrInvalid = errInvalid() // "invalid argument", ErrPermission // "permission denied", ErrExist // "file already exists", ErrNotExist // "file does not exist"; var ErrClosed = errClosed()).
- [30:21-30:26] Opens the ReadFile "Example" (#example-ReadFile) and selects (highlights) the entire example code, evidently to copy. Output shown: `Hello, World!`.
- [30:27] Window switch (collapsed windows view). [30:28] Back in Codium: still the 10/20 program (cursor on the Println line).
- [31:11] Codium main.go now holds the pasted ReadFile example plus two hand-typed comment lines (see File states). Window title still `main.go — go-filemanager`.
- [31:12-31:14] Firefox full-screen, new tab; types `zig ` in URL bar; suggestions include ziglang.org, ziglang.org/download/, YouTube items (the author's own Zig videos: "Building cat by hand - 1 Project, 4 Languages", "Zig Memory Layout is Better Than C", "(8) Integers, Bits & Overflow in Zig", "(9) Ziglings Final Part - Handcoding Zig", "(1) Slices, Tagged Unions & Loop Expressions", "No AI, 52 min. Build all Linux Files utils in C by ...") plus a Google Translate item. Searches DuckDuckGo "zig".
- [31:15] DuckDuckGo results: first "Home ⚡ Zig Programming Language ziglang.org - Use Zig as a zero-dependency, drop-in C/C++ compiler that supports cross-compilation out-of-the-box. Leverage zig build to create a consistent development environment across all platforms." with sub-links Documentation, Devlog, Download, Learn, News, Zig Software Foundation; then Wikipedia "Zig (programming language) - Wikipedia"; then ziglang.org/download.
- [31:17-31:18] Clicks first result -> ziglang.org: "Zig is a general-purpose programming language and toolchain for maintaining robust, optimal and reusable software." Sections "A Simple Language" (No hidden control flow. No hidden memory allocations. No preprocessor, no macros.), "Comptime", "Maintain it with Zig"; code sample index.zig (`const std = @import("std"); const parseInt = std.fmt.parseInt; test "parse integers" { ... }`). Nav: Download & Documentation, Introduction, News, Zig Software Foundation, Devlog, Source, Community.
- [31:21-31:22] Clicks "Zig Software Foundation" -> ziglang.org/zsf/: Mission Statement, "ZSF is a 501(c)(3) non-profit corporation.", Board Members list (President, Secretary, Treasurer), ZSF Board Meeting Minutes, Sponsorship.
- [31:25-31:26] Clicks the president's name -> that person's personal blog: "President and Lead Developer of Zig Software Foundation." and a long "Blog Posts" list (e.g. "2025 Oct 29 - Zig's New Async I/O (Text Version)", "2018 Sep 15 - String Matching based on Compile Time Perfect Hashing in Zig", "2017 Jan 30 - Zig Programming Language Blurs the Line Between Compile-Time and Run-Time"). He scrolls the list. Segment ends there.
- No editor save messages visible in this segment.

## File states

### main.go (Codium, project go-filemanager) at ~27:39 (earliest skeleton)
```
1| package main
2|
3| func main()  {
4|     fmt
5| }
```

### main.go at 29:27 / 30:28 (intermediate version, "10 + 20" sample)
```
1| package main
2|
3| import "fmt"
4|
5| func main()  {
6|     var a = 10;
7|     b := 20;
8|
9|     fmt.Println(a + b);
10| }
```
(Gutter line numbers are hidden in the editor; numbers above are derived. Output of `go run .` was `30`. Semicolons present; two spaces between `main()` and `{`. Line 8 blank, and the Println line shows the current-line highlight.)

### main.go at 31:11 (END of segment; pasted ReadFile example from pkg.go.dev/io/fs plus two comment lines)
```
1| package main
2|
3| import (
4|     "fmt"
5|     "io/fs"
6|     "log"
7|     "testing/fstest"
8| )
9|
10| func main() {
11|     fsys := fstest.MapFS{
12|         "hello.txt": {
13|             Data: []byte("Hello, World!\n"),
14|         },
15|     }
16|     // what the heck fsys, it look like zig
17|     // many year ago , i remember not have this
18|     data, err := fs.ReadFile(fsys, "hello.txt")
19|     if err != nil {
20|         log.Fatal(err)
21|     }
22|
23|     fmt.Print(string(data))
24|
25| }
```
(Derived line numbers; gutter not visible. Red squiggles under each import path at that moment. Line 17 is the highlighted current line.)

### go.mod
Appears in the explorer from 29:27 on; contents never shown.

### Terminal (macOS Terminal) at 27:41
```
~ $ cd lab
lab $ ls
web-editor
lab $ mkdir go-filemanager
lab $ cd go-filemanager
go-filemanager $ codium .
go-filemanager $
```

## Shell & outputs
- [27:41] `cd lab`; `ls` -> `web-editor`; `mkdir go-filemanager`; `cd go-filemanager`; `codium .` (no output).
- [27:55] `cd` (to home), `e .zshrc` (opens zshrc for editing), `gopls` -> `zsh: command not found: gopls`.
- [29:27] (Codium integrated terminal) `go-filemanager $ go run .` -> `30`.
- Codium OUTPUT panel (Go) at 27:39:
  `2026-10-30 20:56:50.174 [info]   gopls`
  `2026-10-30 20:57:04.416 [info] Tools environment: GOPATH=/Users/<user>/go, GOBIN=/Users/<user>/go/bin, GOTOOLCHAIN=auto`
  `2026-10-30 20:57:04.416 [info] Installing 1 tool at the configured GOBIN: /Users/<user>/go/bin`
  `2026-10-30 20:57:04.416 [info]   gopls`
- Browser pages read: pkg.go.dev home, pkg.go.dev/std (go1.27.3, published Oct 20, 2026), pkg.go.dev/io/fs@go1.27.3 (Overview, Index, Functions, type FS, type DirEntry, Variables, ReadFile Example), DuckDuckGo "zig", ziglang.org, ziglang.org/zsf/, the Zig president's personal blog.
- ReadFile example as shown on pkg.go.dev (Output: `Hello, World!`):
```
package main

import (
	"fmt"
	"io/fs"
	"log"
	"testing/fstest"
)

func main() {
	fsys := fstest.MapFS{
		"hello.txt": {
			Data: []byte("Hello, World!\n"),
		},
	}

	data, err := fs.ReadFile(fsys, "hello.txt")
	if err != nil {
		log.Fatal(err)
	}

	fmt.Print(string(data))

}
```

## Observations
- Not part of the JS web editor build: a new "go-filemanager" side project in a fresh dir under `~/lab/` (sibling of `web-editor`), exploring Go for a file manager (cf. his earlier video "Build all Linux Files utils in C").
- Workflow: type a minimal Go skeleton by hand, fight tooling (gopls not on PATH; edits .zshrc), then learn the API by reading official docs (pkg.go.dev -> std -> io -> fs) rather than asking AI, consistent with the "No AI" theme. He goes straight to `io/fs` (the file-system abstraction) and reads ReadFile/FS/DirEntry.
- Learns by copying the official example into main.go and adding his own comments in broken English: "what the heck fsys, it look like zig" / "many year ago , i remember not have this" - he finds `fsys` (an FS interface value passed explicitly, like an allocator/IO handle) reminiscent of Zig's explicit-context style. He then looks up Zig, the Zig Software Foundation and its lead developer's blog, apparently comparing/recalling Zig's philosophy (no hidden control flow, no hidden allocations).
- Toy program style: C-like semicolons and double space before `{` (non-gofmt), `var a = 10; b := 20;` showing both declaration forms.
- Frame sampling gaps (27:56 -> 29:27; 30:28 -> 31:11) hid the typing; go.mod was created in the first gap (probably `go mod init`).
