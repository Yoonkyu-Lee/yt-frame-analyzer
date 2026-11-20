# Segment 8 (17:50 - 19:54) transcription

Note: Terminal window has two tabs: "~/lab -- vim README.md" (stale title; the buffer actually edited is main.c, status line shows "main.c" written) and "~/lab -- zsh". Shell prompt is `lab $ `. Browser is Firefox-like with 4 tabs: "The Open Group Base S...", "Adding Color to Your O...", "c printf indent - Google", "printf - Output indentat..." (Stack Overflow).

## Timeline

- [17:50] Browser over terminal. Google results for query `c printf indent` (URL www.google.com/search?client=firefox-b-d&q=c+printf+indent+). Top hit Stack Overflow "printf - Output indentation in C" (stackoverflow.com/questions/67089739/output-indentation-in-c). Also CPlusPlus.com "Printf" result. At the left edge, the terminal behind shows the earlier compile output: `lab $ cc ...`, `main.c:6...`, `6 |`, `|`, `1 warnin...`, `lab $ `.
- [17:51] Opened the SO question: "Output indentation in C", asks how to print aligned columns "Process No.(Size)  Block No.(Size)" with variable spacing; code `printf("%d(%d)%d(%d)",processNo,processSize,blockNo,blockSize)`. "Where to put the spacing value?"
- [17:52-17:53] Scrolling; Google sign-in popup (Vietnamese UI). Comment by Gerhardh: print first part into a buffer and adjust by strlen. Top (accepted, 3 votes) answer: print each field into a string with a left-justified fixed-width specifier (`-n%s`), sample code with `printf("%-22s%-22s\n", procField, blockField)`.
- [17:54-17:55] Reading that answer's code (snprintf into procField/blockField, `"%-22s%-22s\n"`). Not used by him.
- [17:56-17:57] Scrolled to second answer (2 votes): "printf has two seldom-used features": (1) return value = number of chars printed: `int width = printf("Process No.(Size)    ");` / `int used  = printf("%d(%d)", procNum[i], procSize[i]);` -> spaces needed = width - used. (2) variable field width specifier: "A field width or precision, or both, may be indicated by an asterisk '*' ... an int argument supplies the field width or precision."
- [17:58-18:00] Reads: `printf("%*s", width - used, "");` (asterisk width with empty string prints blanks); note if width - used < 1 set to 1; "Putting it all together" code with `#define max(a,b) ((a) > (b) ? (a) : (b))`. Pointer rests on `"%*s"` at 18:00 (this is the idiom he then applies as `%*c`).
- [18:03] Back in vim (main.c; view lines 9-48). Status: `"main.c" 56L, 1066B written`. Cursor highlighted on the `3` in line 38 `print_file(name, 0, 3);`. Line 29 `print_file(name, 1, 3);`.
- [18:04] Scrolled up, lines 3-42. Line 5 `void print_file(const char *name, int is_dir, int indent) {`, line 6 `printf("%.*c", indent * 2, ' ');` (the buggy `%.*c` version). walk is `int walk(const char *path) {` (no indent param), `walk(sub_path);` at 36. Literal `3` passed as indent.
- [18:07] zsh tab shows `lab $ cc main.c` result:
  `main.c:6:13: warning: precision used with 'c' conversion specifier, resulting in undefined behavior [-Wformat]` with source line `    6 |   printf("%.*c", indent * 2, ' ');` and `~~^~` marker, `1 warning generated.`
- [18:10] zsh: `cc main.c` again (silent) then `./a.out` -> flat list with ~6-space indent (see Shell section).
- [18:13] Vim: line 6 now `printf("%*c", indent * 2, ' ');` (the `.` removed; cursor on the `*`). Status `"main.c" 56L, 1065B written` (1 byte shorter). Lines 29/38 still `3`.
- [18:15] Scrolled; lines 10-49 visible: main at lines 46-49: `int main(int argc, char **argv) {`, `const char *path = "."; // default path is current dir`, blank, `if (argc > 1) {` (cursor on `>`). Lines 42-44 `closedir(dir); return 0; }`.
- [18:23] Lines 17-56 visible: main complete: 49 `if (argc > 1) {`, 50 `path = argv[1];`, 51 `}`, 52 blank, 53 `int err = walk(path);`, 54 blank, 55 `return err;`, 56 `}`. Cursor on closing `}` line 56 with matching `{` at 46 highlighted. Calls still `print_file(name, 1, 3)`.
- [18:28] Lines 8-47. CHANGE: line 13 `int walk(const char *path, int indent) {` (indent param added). Calls still `print_file(name, 1, 3);`, `print_file(name, 0, 3);`, `walk(sub_path);`. Cursor on `s` of `%s` in line 8.
- [18:29] Top of file, lines 1-40; same as above, cursor at end of `<stdio.h>`.
- [18:31] Lines 3-42, cursor on `;` of `closedir(dir);`. Same content.
- [18:32] Lines 6-45, cursor on `{` of line 13 (matching brace highlighted). Same content.
- [19:01] zsh tab shows old screen (compile warning + flat `./a.out` output identical to 18:10).
- [19:02] zsh cleared; typing `lab $ cc mai`.
- [19:09] Vim lines 6-45. `"main.c" 56L, 1102B written`. CHANGES: line 29 `print_file(name, 1, indent);`, line 36 `walk(sub_path, indent + 1);` (cursor on the `1`), line 38 `print_file(name, 0, indent);`.
- [19:12] Lines 1-40, cursor on `>` of line 3. Same as 19:09.
- [19:16] zsh after `cc main.c` / `./a.out`: nested output (abc blue with 1 leading space, def.txt 2 spaces, `.main.c.swp`, `README.md`, `a.out`, `main.c` each with 1 leading space). Top-level items get a leading space because `%*c` with width 0 still prints the ' ' char.
- [19:18] Vim top (lines 1-40), cursor on the `' '` argument in line 6.
- [19:20] Vim INSERT mode: new line 6 being typed `  if` (just "if"); old line 6 shifted to 7; walk now at line 14.
- [19:26] Line 6 now `  if (indent)`, line 7 `    printf("%*c", indent * 2, ' ');` (4-space indent, cursor on `p`). Message line: `Type  :qa!  and press <Enter> to abandon all changes and exit Vim` (he pressed Ctrl-C in normal mode).
- [19:28] INSERT mode again: an empty line inserted at line 8 after the printf (cursor on it); `if (is_dir)` now line 9, walk header at line 15. Purpose unclear (maybe adding a blank line/else?).
- [19:30] zsh tab: previous output visible (with leading spaces) and typing `lab $ cc ma`.
- [19:36] zsh cleared: `lab $ `.
- [19:41] zsh: large tree output tail (command not visible; program run on a Go repo with .git, probably `./a.out <path>`). Tail shown in Shell section. Top-level entries (e.g. `main.go`) now at column 0, i.e. the `if (indent)` guard works.
- [19:43-19:47] Scrolling up through the output: `.git/objects/XX` two-hex-char dirs with 38-char hash files beneath, e.g. `76` -> `5a6a55b9b5e50f1efc243a9933ec8546395310`, `8ed0e45895e07e6c216e6c9bf7fd6fae929392`; `2e` `78` `13` `7a`...; `HEAD`, `info/exclude`, `logs`...; earlier: dirs `91 06 99 0a d4 a7 dd dc a9 d2 aa b7 db c4`, `2a 9f 6b 00`; further up: `cache/peers.go`, `config/config.go`, `auth/auth.go`, `peer/peer.go`, `env/env.go`, `format/format.go`, `dialogs/dialogs.go`, `client/client.go`, `media/media.go`, then top-level `go.sum`, `README.md`, `.gitignore`, `.git` (col 0), `config`, `objects` -> `95`, `50`, `68`, `03`, `93`, `94`, `34`...; so the target is a Go project. Raw readdir order (unsorted).
- [19:47-19:51] Scrolled back to bottom (same tail as 19:41).
- [19:51] zsh cleared, typing `lab $ #`.
- [19:54] `lab $ # Bravo` (macOS menu bar visible: Fri Oct 16 10:56 AM). End of segment.

## File states

### main.c

Version at 19:12 (all lines seen except main body from the 18:23 frame; line 53 as of 18:23):
```
 1| #include <stdio.h>
 2| #include <dirent.h>
 3| #include <string.h>
 4| 
 5| void print_file(const char *name, int is_dir, int indent) {
 6|   printf("%*c", indent * 2, ' ');
 7|   if (is_dir)
 8|     printf("\033[0;34m%s\033[0m\n", name);
 9|   else
10|     printf("%s\n", name);
11| }
12| 
13| int walk(const char *path, int indent) {
14|   DIR *dir = opendir(path);
15|   if (!dir) {
16|     perror("opendir");
17|     return 1;
18|   }
19| 
20|   struct dirent *dp;
21|   while ((dp = readdir(dir)) != NULL) {
22|     const char *name = dp->d_name;
23| 
24|     // ignore files
25|     if (strcmp(name, ".") == 0 || strcmp(name, "..") == 0)
26|       continue;
27| 
28|     if (dp->d_type == DT_DIR ) {
29|       print_file(name, 1, indent);
30| 
31|       // walk to sub path
32|       size_t len = strlen(path) + strlen(name) + 2;
33|       char sub_path[len];
34|       snprintf(sub_path, len, "%s/%s", path, name);
35| 
36|       walk(sub_path, indent + 1);
37|     } else {
38|       print_file(name, 0, indent);
39|     }
40|   }
41| 
42|   closedir(dir);
43|   return 0;
44| }
45| 
46| int main(int argc, char **argv) {
47|   const char *path = "."; // default path is current dir
48| 
49|   if (argc > 1) {
50|     path = argv[1];
51|   }
52| 
53|   int err = walk(path);      // as seen at 18:23; probably later changed to walk(path, 0) -- ?? not seen
54| 
55|   return err;
56| }
```
(Note the `DT_DIR )` has a space before `)`. File was 56L; main after 18:23 not seen again. Since the 19:16 build compiled and ran nested, main must call walk with two args by then -- ?? not seen.)

Version at 19:28 (top part, end of segment):
```
 1| #include <stdio.h>
 2| #include <dirent.h>
 3| #include <string.h>
 4| 
 5| void print_file(const char *name, int is_dir, int indent) {
 6|   if (indent)
 7|     printf("%*c", indent * 2, ' ');
 8| ⟨empty line, INSERT mode, cursor here⟩
 9|   if (is_dir)
10|     printf("\033[0;34m%s\033[0m\n", name);
11|   else
12|     printf("%s\n", name);
13| }
14| 
15| int walk(const char *path, int indent) {
16|   DIR *dir = opendir(path);
17|   if (!dir) {
18|     perror("opendir");
19|     return 1;
20|   }
21| 
22|   struct dirent *dp;
23|   while ((dp = readdir(dir)) != NULL) {
24|     const char *name = dp->d_name;
25| 
26|     // ignore files
27|     if (strcmp(name, ".") == 0 || strcmp(name, "..") == 0)
28|       continue;
29| 
30|     if (dp->d_type == DT_DIR ) {
31|       print_file(name, 1, indent);
32| 
33|       // walk to sub path
34|       size_t len = strlen(path) + strlen(name) + 2;
35|       char sub_path[len];
36|       snprintf(sub_path, len, "%s/%s", path, name);
37| 
38|       walk(sub_path, indent + 1);
39|     } else {
40|       print_file(name, 0, indent);
?? (lines 41+ not visible; same as above shifted by +2)
```
The 19:41 output (top-level names at column 0) shows the guard compiled and worked; whether the empty line 8 stayed is not known.

Intermediate versions:
- 18:04: line 6 `printf("%.*c", indent * 2, ' ');` (warning); `int walk(const char *path) {`; `print_file(name, 1, 3)`, `print_file(name, 0, 3)`, `walk(sub_path);`.
- 18:13: `%.*c` -> `%*c`.
- 18:28: walk gets `int indent`.
- 19:09: calls use `indent`, recursion `indent + 1`.
- 19:26: `if (indent)` guard added in print_file.

No README.md or Makefile content visible in this segment (the vim tab title says README.md but the buffer is main.c).

## Shell & outputs

- [18:07] `lab $ cc main.c`
```
main.c:6:13: warning: precision used with 'c' conversion specifier, resulting in undefined behavior [-Wformat]
    6 |   printf("%.*c", indent * 2, ' ');
      |             ~~^~
1 warning generated.
```
  ("warning:" magenta; format string and marker green.)
- [18:10] `lab $ cc main.c` (silent), `lab $ ./a.out`:
```
      abc          (abc in blue)
      def.txt
      .main.c.swp
      README.md
      a.out
      main.c
```
  all with a 6-space indent (indent hard-coded 3).
- [19:16] `lab $ cc main.c`, `lab $ ./a.out`:
```
 abc          (blue, 1 leading space)
  def.txt     (2 spaces)
 .main.c.swp
 README.md
 a.out
 main.c
```
  Unsorted readdir order; `.main.c.swp` is vim's swap file. Directories blue (`\033[0;34m`), files default color.
- [19:30] typing `lab $ cc ma`.
- [19:41] output of program on a Go repo (command not visible), tail:
```
  info          (blue)
    exclude
  logs          (blue)
    HEAD
    refs        (blue)
      heads     (blue)
        main
      remotes   (blue)
        origin  (blue)
          main
  description
  hooks         (blue)
    commit-msg.sample
    pre-rebase.sample
    sendemail-validate.sample
    pre-commit.sample
    applypatch-msg.sample
    fsmonitor-watchman.sample
    pre-receive.sample
    prepare-commit-msg.sample
    post-update.sample
    pre-merge-commit.sample
    pre-applypatch.sample
    pre-push.sample
    update.sample
    push-to-checkout.sample
  refs          (blue)
    heads
      main
    tags
      v0.0.1
      v0.0.2
    remotes
      origin
        main
  index
  COMMIT_EDITMSG
main.go
lab $
```
  (approximate leading spaces: 2 per level; `.git` children start at 2 spaces, `.git` and `main.go` at column 0.)
- [19:43-19:47] scrollback of same output as in Timeline.
- [19:51] `lab $ #`; [19:54] `lab $ # Bravo`.

## Observations

- Final stage polishes printing: researched printf indentation on Stack Overflow (found `%*s` variable-width idiom, second answer), implemented as `printf("%*c", indent * 2, ' ')`; first mistaken as `%.*c` (clang -Wformat: "precision used with 'c' conversion specifier"), fixed by deleting the dot.
- Indent threaded as a parameter: `walk(path, indent)`, recursion `indent + 1`, `print_file(name, is_dir, indent)`. First tested with hard-coded `3`, then wired through.
- Quirk: `%*c` with width 0 still prints one char, so he adds `if (indent)` guard (19:20-19:26) to put top-level entries in column 0.
- Style: 2-space indent, K&R braces, braceless single-statement ifs, lowercase `//` comments, one tiny helper `print_file`, VLA for sub_path, `perror("opendir")` + `return 1`, main returns walk's error code. Three headers only. No Makefile; builds `cc main.c`, runs `./a.out`.
- Blue directories via ANSI `\033[0;34m%s\033[0m`. No branch glyphs (`|--`), just indentation + color; no sorting; hidden files shown; only `.` and `..` skipped.
- Ends by running the tree on a real Go repo (with .git) and typing a comment `# Bravo` in the shell.
