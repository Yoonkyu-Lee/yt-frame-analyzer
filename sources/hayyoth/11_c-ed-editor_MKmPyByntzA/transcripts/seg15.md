# Segment 15 (59:20 - 67:22)

Layout throughout: left Terminal window (two tabs: "~/lab — vim main.c" and a shell tab "~/lab — -zsh"), right Terminal window (in front of another Terminal behind it), and the "Human" webcam overlay (ignored). The right terminal initially shows an old ed session; from ~63:43 it is reused as a scratch vim buffer.

## Timeline

- [59:20] Left window on shell tab "~/lab — -zsh", prompt `lab $ ` empty. Right terminal shows earlier ed session (see Shell & outputs).
- [59:21] Shell: `lab $ cc main.c` -> warning: `main.c:82:19: warning: incompatible pointer types passing 'char[4]' to parameter of type 'FILE *' (aka 'struct __sFILE *') [-Wincompatible-pointer-types]`, pointing at line 82 `fprintf("%s\n", lines[i], fp);`, with a note from stdio.h:245:31. `1 warning generated.`
- [59:32] Switched to tab "~/lab — vim main.c". Normal mode, cursor 82,38 (on `p` of `fp`). Status line: `"main.c" 106L, 2386B written` (82,38  81%). Lines 58-93 visible. Line 82 still the buggy `fprintf("%s\n", lines[i], fp);`.
- [59:38] Back on shell tab; same compile warning still displayed.
- [59:39] Shell cleared; only `lab $ cc main.c` then fresh `lab $ ` (clean compile, presumably after fixing the fprintf argument order).
- [59:43] Shell: `lab $ ls` -> `a.out      count.txt main.c    text.txt` / `ai.c      hello.txt notes.txt` (a.out red). Typing `lab $ rm count.txt ` (not shown executed).
- [60:07] Shell: `lab $ ./a.out count.txt` run: `p` -> `this is line 3`; `a`, `this is line 4`, `.`, `w`, `q`. Then `lab $ cat count.txt` -> `this is line 1 / this is line 2 / this is line 3` (line 4 NOT saved: loop `i < count_line` skips last line).
- [60:17] Tab title "~/lab — a.out count.txt". Second `cat count.txt` (same 3 lines), then new `lab $ ./a.out count.txt` running, awaiting input.
- [60:31] vim main.c tab, normal mode. `"main.c" 106L, 2386B written`, cursor 82,22, 81%. Line 82 now `fprintf(fp, "%s\n", lines[i]);` (args fixed; cursor on the space before `"%s\n"`).
- [60:32] Scrolled: lines 64-99 visible, 90%, cursor 82,22. Line 99 `}`.
- [60:33] Scrolled further: lines 67-102 visible, 94%. Lines 100 `}`, 101 blank, 102 `for (int i = 1; i < count_line; i++)` (stray leftover fragment at file bottom).
- [60:49] Shell tab (title "a.out count.txt"): the run continued: `a`, `this is line 4`, `.`, then `p` -> `this is line 4`; program waiting.
- [60:56] Shell cleared, typing `lab $ ca` (about to `cat count.txt`).
- [61:00] `lab $ cat count.txt` -> `this is line 1 .. this is line 4` (4 lines; "this is line 4" highlighted by mouse selection).
- [61:01] vim main.c tab in INSERT mode, cursor 81,28 (line 81 `for (int i = 1; i < count_line; i++)`, cursor right after `<`), 94%. Lines 67-102 visible.
- [61:04] Shell: same cat output with line 4 highlighted.
- [61:21] Shell tab title "~/lab — a.out". Screen: `lab $ cat count.txt` (4 lines), `lab $ cc main.c`, `lab $ ./a.out`; typed `a`, `hello 1`, `hello 2`, `hello 3`, `.`, `w` -> `?` (no filename, expected), typing `w he` (in progress).
- [61:33] `w hello-here.txt`, `p` -> `hello 3`, `q`; `lab $ cat hello-here.txt` -> `hello 1 / hello 2 / hello 3` (all saved; `<=` fix works). Prompt `lab $ `.
- [61:38] Shell cleared, typing `lab $ l`.
- [61:40] vim main.c normal mode: `"main.c" 106L, 2387B written` (81,28  94%). Line 81 now `for (int i = 1; i <= count_line; i++)` (cursor on the `=`).
- [61:43] Scrolled up: lines 59-94 visible, cursor 59,7, 82%.
- [61:44] Cursor 57,12 (on `else` at line 57), lines 57-92, 80%.
- [61:45] Cursor 45,23, lines 45-80, 62%. Looking at the `d` handler (lines 60-61, empty body `{ // delete` / `}`).
- [62:01] INSERT mode: line 60 `else if (mode(cmd, "d")) { // delete`; new line 61 contains a comment being typed `// |` (cursor 61,37); `}` at 62. Rest shifted +1 (save handler at 63). 61%.
- [62:27] INSERT, cursor 63,11. Delete handler:
  - line 60 `else if (mode(cmd, "d")) { // delete`
  - line 61 `if (count_line > 0) {`
  - line 62 empty
  - line 63 cursor (empty)
  - line 64 `}`
  60%.
- [62:29] Cursor 64,11: line 62 empty, line 63 `} else`, line 64 cursor (else body next). 59%.
- [63:11] Delete handler: `if (count_line > 0) {` / `free(lines[current_line]);` / (line 63 cursor) / `} else` / `puts("?");` / `}`. 58%, cursor 63,11.
- [63:43] Typing `for (int i = ` on line 63 (cursor 63,24). Right terminal cleared, shows `lab $ e` being typed (launching vim).
- [63:44] Right terminal now in vim (splash screen: "VIM - Vi IMproved", "version 9.2.1200", "by Bram Moolenaar et al.", "Vim is open source and freely distributable", "Help poor children in Uganda!", ...), line 1, `-- INSERT --`, 0,1 All.
- [63:45] Right vim buffer: lines 1, 2, 3 empty, INSERT 3,1.
- [64:05] Right vim buffer: `1 this is line 1`, `2 this is line 2 // current line – delete it`, `3 this is line 3`, `4 this is line 4`. Cursor 2,42.
- [65:22] Right vim, INSERT 10,7 (content in File states below): line 2 now `xxxxxxxxxxxxxx // current line – delete it`; lines 7-10: `(count line 4)`, `------------`, `move from 2 to 4`, `  line` (typing).
- [66:04] Right vim: line 9 `move from 2 to 3` (changed from 4), line 10 `  lines[i] = lines[i + 1]`, INSERT 9,17. Left vim: line 63 now `for (int i = 1; i < count_line; i++)`, cursor on blank line 64 (64,13), INSERT, 57%; save handler now at line 68.
- [66:27] Left vim delete handler complete (`i = 1` changed to `i = current_line`, body `lines[i] = lines[i + 1];`). Cursor 86,0-1 (empty line 86), 65%.
- [66:29] Shell tab "~/lab — -zsh": `lab $ ls` -> `a.out  ai.c   main.c` (only 3 files; differs from earlier ls).
- [66:56] Shell tab "~/lab — a.out": `lab $ ls`, `lab $ cc main.c` (clean), `lab $ ./a.out`; typed `a`, `line 1`, `line 2`, `line 3`, `line 4`, `.`, `1` -> `line 1`, `2` -> `line 2`, `3` -> `line 3`, `2` -> `line 2`, `d` (waiting).
- [67:16] `p` -> `line 3`, `w line.txt`, `q`; `lab $ cat line.txt` -> `line 1 / line 3 / line 4 / line 4` (mouse selection highlighting). Delete works but last line duplicated (count_line not decremented).
- [67:17]-[67:20] Same screen, mouse selection toggling; no changes.
- [67:22] Back to vim main.c tab: `"main.c" 112L, 2585B written`, cursor 86,0-1, 65%. Visible lines 51-86.

## File states

### main.c (end of segment; lines 45-86 verbatim from 67:22 frame)

Indentation unit appears to be 2 spaces (columns inferred from pixel positions).

```
45|      current_line++;
46|      lines[current_line] = strdup(cmd);
47|      count_line++;
48|    }
49|  } else {
50|    if (mode(cmd, "q"))
51|      break;
52|    else if (mode(cmd, "a")) // append
53|      on_edit = 1;
54|    else if (mode(cmd, "p")) { // print
55|      if (count_line > 0 )
56|        puts(lines[current_line]);
57|      else
58|        puts("?");
59|    }
60|    else if (mode(cmd, "d")) { // delete
61|      if (count_line > 0) {
62|        free(lines[current_line]);
63|        for (int i = current_line; i < count_line; i++)
64|          lines[i] = lines[i + 1];
65|      } else
66|        puts("?");
67|    }
68|    else if (strncmp(cmd, "w", 1) == 0) { // save
69|      char *path = cmd + 1;
70|      while (*path == ' ')
71|        path++;
72|
73|      if (*path == '\0')
74|        path = filename;
75|
76|      if (strlen(path) == 0) {
77|        puts("?");
78|        continue;
79|      }
80|
81|      FILE *fp = fopen(path, "w");
82|      if (!fp) {
83|        perror(path);
84|        continue;
85|      }
86|
?? (lines 87-112 not visible at end of segment)
```

Remainder as seen before the 6-line insertion (old numbering; final line number = old + 6):

```
old 81|      for (int i = 1; i <= count_line; i++)   (was `<` until 61:40)
old 82|        fprintf(fp, "%s\n", lines[i]);
old 83|
old 84|      if (strlen(filename) == 0)
old 85|        strncpy(filename, path, sizeof(filename));
old 86|
old 87|      fclose(fp);
old 88|    } else {
old 89|      char *endptr;
old 90|      int num = strtol(cmd, &endptr, 10);
old 91|
old 92|      if (*endptr == '\0' && num > 0 && num <= count_line) {
old 93|        current_line = num;
old 94|        puts(lines[current_line]);
old 95|      } else {
old 96|        puts("?");
old 97|      }
old 98|    }
old 99|  }
old100|}
old101|
old102|for (int i = 1; i < count_line; i++)   <- stray fragment at file bottom
?? (old lines 103-106 never visible)
```

File length at 67:22: 112L, 2585B. Lines 1-44 not visible in this segment.

Intermediate version, 62:01 delete handler: line 61 `//` comment placeholder (`// |`), replaced by 62:27 with `if (count_line > 0) {` structure.

### Right terminal scratch vim buffer (unnamed, unsaved; end of segment)

```
 1| this is line 1
 2| xxxxxxxxxxxxxx // current line – delete it
 3| this is line 3
 4| this is line 4
 5|
 6|
 7| (count line 4)
 8| ------------
 9| move from 2 to 3
10|   lines[i] = lines[i + 1]
```
(Dash in line 2 looks like an en dash; x-count approximate.)

### Test files (from cat output)
- count.txt: `this is line 1`, `this is line 2`, `this is line 3` (60:07); later 4 lines incl. `this is line 4` (61:00).
- hello-here.txt: `hello 1`, `hello 2`, `hello 3`.
- line.txt: `line 1`, `line 3`, `line 4`, `line 4`.
- text.txt: `this is text`.

## Shell & outputs

Right terminal (static, 59:20-63:43):
```
lab $ ed
this is text
?
a
this is text
.
w
?
w text.txt
13
q
lab $ cat text.txt
this is text
lab $
```

Left terminal shell:
- [59:21]
```
lab $ cc main.c
main.c:82:19: warning: incompatible pointer
      types passing 'char[4]' to parameter of type 'FILE *' (aka
      'struct __sFILE *') [-Wincompatible-pointer-types]
   82 |         fprintf("%s\n", lines[i], fp);
      |                 ^~~~~~
/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/usr/include/_s
tdio.h:245:31: note:
      passing argument to parameter here
  245 | int      fprintf(FILE * __restrict, const char * __restric
...
      |                                                          ^
1 warning generated.
lab $
```
- [59:39] `lab $ cc main.c` (no output).
- [59:43] `lab $ ls` -> `a.out      count.txt main.c    text.txt` / `ai.c      hello.txt notes.txt`; typing `rm count.txt `.
- [60:07] `lab $ ./a.out count.txt`; input `p` -> `this is line 3`; `a`; `this is line 4`; `.`; `w`; `q`. `lab $ cat count.txt` -> `this is line 1`, `this is line 2`, `this is line 3`.
- [60:17] second `cat count.txt` (same), then `lab $ ./a.out count.txt` (waiting).
- [60:49] in that run: `p` -> `this is line 3`, `a`, `this is line 4`, `.`, `p` -> `this is line 4`.
- [61:00] `lab $ cat count.txt` -> `this is line 1`..`this is line 4`.
- [61:21] `lab $ cc main.c`; `lab $ ./a.out`: `a`, `hello 1`, `hello 2`, `hello 3`, `.`, `w` -> `?`, `w he...`.
- [61:33] `w hello-here.txt`, `p` -> `hello 3`, `q`; `lab $ cat hello-here.txt` -> `hello 1`, `hello 2`, `hello 3`.
- [66:29] `lab $ ls` -> `a.out  ai.c   main.c`.
- [66:56] `lab $ cc main.c` (clean), `lab $ ./a.out`: `a`, `line 1`, `line 2`, `line 3`, `line 4`, `.`, `1` -> `line 1`, `2` -> `line 2`, `3` -> `line 3`, `2` -> `line 2`, `d`.
- [67:16] `p` -> `line 3`, `w line.txt`, `q`; `lab $ cat line.txt` -> `line 1`, `line 3`, `line 4`, `line 4`.

Vim save messages (checksums): `"main.c" 106L, 2386B written` (59:32, 60:31); `"main.c" 106L, 2387B written` (61:40); `"main.c" 112L, 2585B written` (67:22).

## Observations

- Bugs found by testing: (1) `fprintf("%s\n", lines[i], fp)` wrong argument order, fixed to `fprintf(fp, "%s\n", lines[i])`; (2) save loop `i < count_line` dropped the last line, fixed with `<=` (hello-here.txt proved it). So lines[1..count_line] are used, lines[0] unused.
- Test workflow: edit in vim, `cc main.c`, `./a.out [file]`, `cat` the output file; small test files count.txt, hello-here.txt, line.txt.
- Implemented `d` (delete): guard `count_line > 0`, `free(lines[current_line])`, shift loop `for (int i = current_line; i < count_line; i++) lines[i] = lines[i + 1];`. Designed with a scratch vim buffer in the right terminal (sketch: "(count line 4) / move from 2 to 3 / lines[i] = lines[i + 1]"). First draft `i = 1` corrected to `i = current_line`.
- Remaining bug at end: count_line not decremented after delete, so last line duplicated in saved file (line.txt shows `line 4` twice); current_line not adjusted. Likely fixed next segment.
- Style: 2-space indentation, trailing `// append`, `// print`, `// delete`, `// save` comments, same-line braces, braceless single-statement `if`/`for`, errors via `puts("?")` and `perror(path)`.
- Stray `for (int i = 1; i < count_line; i++)` line remains at file bottom (old line 102).
- Real `ed` used in right terminal as reference (`w text.txt` -> `13`).
