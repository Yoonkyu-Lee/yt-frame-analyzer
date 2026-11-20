# Segment 13 (47:45 - 53:32)

Window layout throughout: left Terminal window with tabs "~/lab — vim main.c" and a second tab (title alternates "~/lab — -zsh" / "~/lab — a.out"); right Terminal (front) shows a static earlier session of the real `ed` (unchanged the whole segment); another Terminal behind it. Webcam "Human" ignored.

## Timeline
- [47:45] vim main.c, NORMAL mode, cursor 84,0-1 (96%). Lines 49-84 visible. Line 62 `else if (strncmp(cmd, "w", 1)) { // save` (no `== 0` yet). Lines 63-67: `char *path = cmd + 1;` / `while (*path == ' ') path++; // trim space` / blank / `if (*path == '\0')` / `path = filename;`. Line 68 `} else {` follows directly. Lines 82-83 `for (int i = 1; i < count_line; i++)` / `free(lines[i]);`, line 84 empty (cursor).
- [47:55] INSERT, 68,9: new blank line 68 inserted after `path = filename;`; `} else {` shifted to 69.
- [47:56] Typing `if` at line 69 (69,11); `} else {` now line 70.
- [48:11] Line 69 now `if (strlen(path) == 0) {`; cursor on empty line 70 (70,11), `} else {` on line 71.
- [48:20] Line 70 `puts("?");` typed; cursor on line 71 (71,11); `} else {` line 72.
- [48:30] Typing `conti` on line 71; auto-inserted `}` on line 72; `} else {` line 73.
- [48:36] Lines 69-72: `if (strlen(path) == 0) {` / `puts("?");` / `continue;` / `}`; typing `put` on new line 73 (73,12), `} else {` line 74.
- [48:42] NORMAL, 73,0-1: line 73 empty, line 74 `puts("On saved");` (capital O), `} else {` line 75.
- [49:02] Second tab (title "~/lab — -zsh") terminal history: `lab $ cc main.c`, `lab $ ./a.out`, `lab $ ./a.out hello.txt` -> `file name is hello.txt` / `104`; `lab $ ./a.out abc.txt` -> `abc.txt: No such file or directory` / `file name is abc.txt` / `97`; prompt `lab $ `. `file name is abc.txt` line highlighted (selection). (Old output from earlier segment.)
- [49:11] Tab title now "~/lab — a.out". New `lab $ cc main.c`, `lab $ ./a.out`; ed running, user typed `a` then `hello ` (cursor after "hello ").
- [49:26] Back in vim, NORMAL 74,38 (84%); line 74 is now `printf("on saved %s\n", path);` (cursor on final `;`). Status: `"main.c" 93L, 2082B written`.
- [49:37] a.out tab: session `a` / `hello` / `.` / `w` / `?` / `w what.txt` (selected) / `?` and cursor on an empty line.
- [49:39] vim again, same as 49:26 (`"main.c" 93L, 2082B written`).
- [49:58] INSERT 66,10: trim while converted to braces: line 64 `while (*path == ' ') {` (matching braces highlighted), blank line 65, line 66 `}; // trim space` (the `}` highlighted, pair with `{`). Following: 68 `if (*path == '\0')`, 69 `path = filename;`, 71 `if (strlen(path) == 0) {`, 76 `printf("on saved %s\n", path);`, 77 `} else {`.
- [50:11] INSERT 66,11: line 65 `printf("on trim\n");`, cursor on blank line 66, line 67 `}; // trim space`.
- [50:17] a.out tab: previous output plus `q` typed (`w what.txt` / `?` / `q`).
- [50:33] vim NORMAL 66,17, `"main.c" 96L, 2138B written`. Line 66 `path++;` Loop is: `while (*path == ' ') {` / `printf("on trim\n");` / `path++;` / `}; // trim space`.
- [50:41] a.out tab after `lab $ cc main.c`, `lab $ ./a.out`: `w hello.txt` -> `?`; `w` -> `?`; `w hello.txt` -> `?` (strncmp lacking `== 0`, so branch never taken).
- [50:43] Second tab title "~/lab — -zsh": screen cleared, `lab $ p` typed (partial command).
- [51:03] vim NORMAL 62,40: line 62 now `else if (strncmp(cmd, "w", 1) == 0) { // save` (the `0` highlighted). `"main.c" 96L, 2143B written`.
- [51:10] a.out tab: `lab $ cc main.c`, `lab $ ./a.out`, session: `a` / `hello they` / `w` / (blank) / `.` / `w` / `on saved ???` / `w hello.txt` (cursor right after).
- [51:26] Continued: `w hello.txt` -> `on trim` / `on saved hello.txt`; `w abc.txt` -> `on trim` / `on saved abc.txt`; bare `w` -> `on saved ???` (highlighted by selection); empty prompt line.
- [51:30] vim NORMAL 77,38, `"main.c" 96L, 2143B written`. Cursor on `;` of line 77 `printf("on saved %s\n", path);`.
- [51:37] a.out tab: same output as 51:26.
- [51:39] vim same as 51:30.
- [51:42] vim NORMAL 85,16 (81%), no status message. Frame render shows line numbers 82 `if (*endptr == '\0' && num > 0 && num <= count_line) {`, then 84 `puts(lines[current_line]);`, 85 `} else {` -- line 83 (`current_line = num;`) appears missing in this frame (number jumps 82 -> 84); likely transient scroll/render artifact (in later frames line 83 `current_line = num;` is present).
- [51:43] vim scrolled: top line 53 `on_edit = 1;`, bottom 88 `}`. Cursor 78,14 (86%). File intact (line 83 `current_line = num;` present).
- [52:01] a.out tab: output continues: after `w abc.txt` block: `w` / `on saved ???` / `w` / `on saved ???` / prompt.
- [52:04] vim NORMAL 70,26: cursor on `;` of line 70 `path = filename;`.
- [52:16] vim NORMAL, pending `d` shown in status, cursor 65,8. Top line 53. Lines: 64 `while (*path == ' ') {`, 65 `printf("on trim\n");` (cursor at its start), 66 `path++;`, 67 blank (the `}; // trim space` line already deleted), 68 `if (*path == '\0')`, 69 `path = filename;`, 70 blank, 71 `if (strlen(path) == 0) {`, ... 76 `printf("on saved %s\n", path);`, 77 `} else {`. 88 `}` (one fewer indent).
- [52:17] INSERT 64,31: printf line removed. Line 64 `while (*path == ' ') {` (brace highlighted, cursor right after it), 65 `path++;`, 66 blank, 67 `if (*path == '\0')`, 68 `path = filename;`, 69 blank, 70 `if (strlen(path) == 0) {`, 71 `puts("?");`, 72 `continue;`, 73 `}`, 74 blank, 75 `printf("on saved %s\n", path);`, 76 `} else {`, 87 `}`, 88 `}`.
- [52:25] NORMAL 69,11: line 64 `while (*path == ' ')` (brace removed), 65 `path++;`, 66 blank, 67 `if (*path == '\0') {`, 68 `path = filename;`, 69 `printf("on trim\n");` (cursor on `p`; text pasted/moved from earlier), 70 blank, 71 `if (strlen(path) == 0) {`.
- [52:26] INSERT 70,11: line 69 `printf("on trim\n");`, cursor on new line 70 (empty, indented), blank line 71, `if (strlen(path) == 0) {` line 72; the `if (*path=='\0') {` block not yet closed.
- [52:28] NORMAL 68,9: block closed: 67 `if (*path == '\0') {` / 68 `path = filename;` / 69 `}`; printf deleted.
- [52:29] INSERT 69,11: blank indented line 69 inside block, `}` on line 70, `if (strlen(path) == 0) {` on 72.
- [52:43] a.out tab (old output only): same as 52:01 (`... on saved ???` / prompt cursor).
- [52:44] Second tab "~/lab — -zsh": cleared, `lab $ ` prompt.
- [52:53] vim NORMAL 69,41, `"main.c" 96L, 2139B written`. Line 69 `printf("on assign %s\n", path);` inside `if (*path == '\0') {` block (cursor on `;`).
- [53:08] a.out tab (mouse hovering tab, "x" close button visible): `lab $ cc main.c`, `lab $ ./a.out`, `w` / `on assign ???` / `on saved ???` / cursor.
- [53:12] vim, same as 52:53 (mouse on vim tab, "x" shown).
- [53:32] a.out tab: `w` / `on assign ???` / `on saved ???` / `q` / `lab $ ` (program exited).

## File states
### main.c lines 53-88 as of END of segment (last vim view 53:12; `"main.c" 96L, 2139B written`)
Lines 1-48 and 89-96 not visible in this segment. Lines 49-52 last seen at 52:04-era, unchanged.
```
49|     } else {
50|       if (mode(cmd, "q"))
51|         break;
52|       else if (mode(cmd, "a")) // append
53|         on_edit = 1;
54|       else if (mode(cmd, "p")) { // print
55|         if (count_line > 0 )
56|           puts(lines[current_line]);
57|         else
58|           puts("?");
59|       }
60|       else if (mode(cmd, "d")) { // delete
61|       }
62|       else if (strncmp(cmd, "w", 1) == 0) { // save
63|         char *path = cmd + 1;
64|         while (*path == ' ')
65|           path++;
66| 
67|         if (*path == '\0') {
68|           path = filename;
69|           printf("on assign %s\n", path);
70|         }
71| 
72|         if (strlen(path) == 0) {
73|           puts("?");
74|           continue;
75|         }
76| 
77|         printf("on saved %s\n", path);
78|       } else {
79|         char *endptr;
80|         int num = strtol(cmd, &endptr, 10);
81| 
82|         if (*endptr == '\0' && num > 0 && num <= count_line) {
83|           current_line = num;
84|           puts(lines[current_line]);
85|         } else {
86|           puts("?");
87|         }
88|       }
?? (lines 89-96 not visible; earlier (47:45) the file ended: `    }` / `  }` / blank / `for (int i = 1; i < count_line; i++)` / `  free(lines[i]);` )
```
Indentation notes: columns exact as rendered: `} else {` (line 49) at col 4; `if (mode(cmd, "q"))` col 6; nested body col 8 (relative 2-space steps). Note the mode()/branches at the start of this segment sat at the same indent.

### Intermediate versions
- 47:45 (before edits), lines 62-68:
```
62|       else if (strncmp(cmd, "w", 1)) { // save
63|         char *path = cmd + 1;
64|         while (*path == ' ') path++; // trim space
65| 
66|         if (*path == '\0')
67|           path = filename;
68|       } else {
```
- 49:26 (`"main.c" 93L, 2082B written`):
```
62|       else if (strncmp(cmd, "w", 1)) { // save
63|         char *path = cmd + 1;
64|         while (*path == ' ') path++; // trim space
65| 
66|         if (*path == '\0')
67|           path = filename;
68| 
69|         if (strlen(path) == 0) {
70|           puts("?");
71|           continue;
72|         }
73| 
74|         printf("on saved %s\n", path);
75|       } else {
```
- 50:33 (`"main.c" 96L, 2138B written`):
```
62|       else if (strncmp(cmd, "w", 1)) { // save
63|         char *path = cmd + 1;
64|         while (*path == ' ') {
65|           printf("on trim\n");
66|           path++;
67|         }; // trim space
68| 
69|         if (*path == '\0')
70|           path = filename;
...
```
- 51:03 (`"main.c" 96L, 2143B written`): same as 50:33 but line 62 `... strncmp(cmd, "w", 1) == 0) { // save`.
- 52:53 final (`"main.c" 96L, 2139B written`): as in main listing above (trim-loop braces/comment removed, `on trim` print removed, `on assign` print added).

### Other files
Right terminal (static, unchanged this segment): `lab $ ed` / `this is text` / `?` / `a` / `this is text` / `.` / `w` / `?` / `w text.txt` / `13` / `q` / `lab $ cat text.txt` / `this is text` / `lab $`. No file contents for hello.txt/abc.txt/what.txt seen.

## Shell & outputs
- [49:02] second tab history: `cc main.c`; `./a.out`; `./a.out hello.txt` -> `file name is hello.txt`, `104`; `./a.out abc.txt` -> `abc.txt: No such file or directory`, `file name is abc.txt`, `97`.
- [49:11] `cc main.c`; `./a.out`; typed in editor: `a`, `hello `.
- [49:37] ed session in a.out: `a` / `hello` / `.` / `w` / `?` / `w what.txt` / `?` (input echoed by terminal).
- [50:17] adds `q` (exit).
- [50:41] `cc main.c`; `./a.out`: `w hello.txt` -> `?`; `w` -> `?`; `w hello.txt` -> `?`.
- [50:43] new tab: `lab $ p` (partial; screen cleared).
- [51:10-51:26] `cc main.c`; `./a.out`: `a` / `hello they` / `w` / (empty line) / `.` / `w` -> `on saved ???`; `w hello.txt` -> `on trim` `on saved hello.txt`; `w abc.txt` -> `on trim` `on saved abc.txt`; `w` -> `on saved ???`.
- [52:01] two more bare `w` -> `on saved ???` each.
- [53:08] `cc main.c`; `./a.out`: `w` -> `on assign ???` / `on saved ???`.
- [53:32] `q`, then `lab $ ` (exit).
No compiler errors or warnings seen.

## Observations
- Feature being built: the `w` (save) command with optional path argument, defaulting to global `filename` (from argv). Currently only prints debug messages ("on trim", "on saved %s", "on assign %s"); no actual file writing yet.
- Debug style: printf probes in-code, edit-compile-run loop switching between vim tab and a.out tab; compile with plain `cc main.c`.
- Bug found: `strncmp(cmd, "w", 1)` lacked `== 0` (every `w` gave `?`); fixed at ~51:03. Bare `w` prints `on saved ???` / `on assign ???`: `filename` is not set when no argv given (prints `???`, probably garbage/unset pointer), so the `strlen(path)==0` guard does not trigger.
- Style: 2-space indentation, `// comment` labels (`// save`, `// trim space`), `puts("?")` as ed's generic error response, brace-less single-statement while/if; braces added temporarily for debug then removed.
- Behavior reference: right window shows real `ed` session (`w text.txt` prints byte count `13`; bare `w` with no filename prints `?`).
