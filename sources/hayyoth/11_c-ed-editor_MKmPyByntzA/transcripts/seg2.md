# Segment 2 (1:29 - 5:09)

## Timeline

Layout throughout: left Terminal window (vim / shell), right Terminal window (vim showing notes file, "ed text editor"), a yellow Stickies note in the middle, "Human" webcam bottom right (ignored). A ChatGPT browser window is partly visible behind (URL chatgpt.com/c/6adf65b9-0e50-83ec-9a97...). Window titles are just "Terminal" (no tab titles visible).

- [01:29] Left vim shows ChatGPT-generated C code (file ~549 lines), lines 341-376, cursor 341,5 (66%). Content: `if (strcmp(cmd, "$") == 0) {` ... `b->cur = b->len; print_line(b, b->cur); return 1;`, then `/* Address followed by a print command, e.g. 3p */` `if (isdigit((unsigned char)*cmd)) {` with `parse_number(arg, &n)`, saved char / `*cmd = '\0'` trick, `while (isspace(...)) cmd++;`. Right terminal: notes file lines 1-18 visible, cursor 18,35 Top. Sticky note behind reads "...his is chatgpt code" (left part hidden by overlapping window).
- [01:30] Scrolling the ChatGPT code: lines 409-444, 79%. Shows `strcmp(cmd, "a") == 0` branch with `position`, `edit_lines(b, position)`, `w` command with `isspace`, `strlen(arg) >= 4096`, `realloc(*filename, strlen(arg) + ...`.
- [01:31] Lines 443-478, 86%: `fprintf(stderr, "Error: out of memory\n"` , `strcpy(*filename, arg);`, `write_file(b, *filename);`, comment `/* Read a file into the buffer, replacing its conten...`, `strncmp(cmd, "ed", 2) == 0`.
- [01:32] Lines 511-546, 99%: end of main: `if (argc == 2) { filename = malloc(strlen(argv[1]) + 1); ... load_file(&b, filename) ... free_buffer(&b); return EXIT_FAILURE;`, `while (running) { cmd = read_line(stdin); ... running = execute(&b, cmd, &filename); free(cmd); }`, `free(filename); free_buffer(&b);`.
- [01:33] End of file: lines 544-549 + tildes, cursor 544,0-1, Bot. `return EXIT_SUCCESS;` `}` at 548-549.
- [01:34] Scrolled back: 530-549, cursor 530,5 Bot. Mouse pointer over sticky note.
- [01:41] Sticky note now expanded/being typed: 
  ```
  this is chatgpt code
  550 line
  too complica|
  ```
  (typing "too complicated"). Left vim unchanged (530-549).
- [02:10] Sticky note blank (text cleared). Left vim: `:q` typed on command line (lines 530-549, indentation now shows `    while (running) {` at col 4 -> line 530 `    while (running) {`).
- [02:11] Left terminal at shell: 
  ```
  lab $ ls
  ai.c      notes.txt
  lab $ e ai.c
  
  lab $ 
  ```
  (`e` appears to be a shell alias for vim; screen cleared after vim exit.) Right terminal still same notes file.
- [02:13] Left terminal window slightly widened (now ~x 88-807). Same content.
- [02:15] Left terminal cleared: only `lab $ ` prompt. Right terminal notes file lines 1-21 visible: line 18 `- ed -> sed -> ex -> vi -> vim.`, line 20 dashes, line 21 `[num] - go to line`. Cursor 18,35 Top (line 18 end).
- [02:16] ChatGPT browser window brought to front: table of commands: `,p` Print all lines; `1,$p` Print all lines; `w` Save to the current filename; `w file.txt` Save to a specified file; `ed file.txt` Read a file, replacing the current buffer; `q` Quit; `h` Show help. Note text: "Note: This is a simplified ed-style editor, not a complete POSIX ed implementation. The ed path command replaces the buffer with the file's contents. It uses only standard C library facilities and requires no external libraries." Footer "ChatGPT can make mistakes. Check important info."
- [02:17] ChatGPT window hidden again; sticky note partly visible; right terminal notes lines 1-21.
- [02:19] Right notes scrolled: lines 9-29 (57%, cursor 29,26). New lines: 22 `  $ - last line`, 24 `i - Insert text before the current line`, 25 `a - Append text after the current line`, 26 `  . - Stop edit`, 28 `d - Delete the current line`, 29 `p - Print the current line`.
- [02:20] Notes scrolled to end: lines 15-35 (Bot, cursor 35,31). Adds 30 `  ,p - Print all lines`, 31 `  1,$p - Print all lines`, 32 `w - Write the buffer to a file`, 33 `q - Quit the editor`, 34 blank, 35 `ed [path] - Read file to buffer`.
- [02:35] Left terminal: running the (system) `ed`:
  ```
  lab $ ed
  a
  hello they
  my name is hayyiot|
  ```
  (typing, "hayyiot" typo/mid-typing, later "hayyoth"). Right: cursor 21,18 on line 21 ("line" e highlighted).
- [02:41] Left: `hello they` / `my name is hayyoth` / `.` ; new empty yellow sticky note being focused (cursor in it).
- [02:45] Sticky note text starting ". is end..." (partly hidden). Mouse in right terminal.
- [02:48] Right terminal in `-- VISUAL --` mode with line 26 (`  . - Stop edit`) selected, status `15  26,1 Bot`.
- [02:53] Left: after `.`, typing `w`. Sticky note: `. is end edit`.
- [02:57] Left: `w hello.txt` -> output `30`.
- [03:04] Left: `q` typed, back to `lab $ `. Sticky note: `save file`.
- [03:07] `lab $ cat hello.txt ` typed.
- [03:10] cat output: `hello they` / `my name is hayyoth`; then `lab $ `. Output text selected (highlight). Sticky blank.
- [03:11] `lab $ ed ` typed.
- [03:17] `lab $ ed hello.txt` -> `30` (system ed prints byte count). Sticky note text: `quit`.
- [03:24] Typed `1` at ed prompt. Sticky: `[num] -> go to line` (partly).
- [03:31] `1` -> `hello they`; `2` -> `my name is hayyoth`; `p` -> `my name is hayyoth`. Sticky: `[num] -> go to line`.
- [03:38] Sticky note moved lower (y~585-780): text `p -> print current line`. Right terminal back in `-- VISUAL --`. Left: continued: `1` -> `hello they`, `p` -> `hello they`.
- [03:42] Same left state (`1`, `hello they`, `p`, `hello they`). Sticky shows "... print current line".
- [03:46] After another `p` -> `hello they`. Sticky note being retyped: `d `.
- [03:50] Sticky: `d -> delete` (partly). Left: `hello they` selected (after `p`); cursor below. (d command presumably typed; not visible.)
- [03:57] Left: `lab $ cat ` typed (screen cleared, new session).
- [04:02] `lab $ cat hello.txt` -> `my name is hayyoth` (only second line remains -> `d` on line 1 then `w` was done). Sticky: `d -> delete`.
- [04:06] Dialog "If you don't save this note, its contents will be lost. Are you sure you want to discard this Stickies note?" with Save... / Delete Note / Cancel; mouse on Delete Note.
- [04:07] Sticky note gone; left terminal cleared `lab $ `.
- [04:09] Left Terminal vim opened: `"main.c" [New]`, 4,2 All. Contents (vim template/auto?):
  ```
  1 #include <stdio.h>
  2
  3 int main(void) {
  4   |
  5   return 0;
  6 }
  ```
  (indent is 2 spaces apparently). Right notes: lines 15-35, VISUAL leftover. Webcam: programmer touching ear.
- [04:32] Insert mode, 5,3 All: 
  ```
  1 #include <stdio.h>
  2
  3 int main(void) {
  4
  5   |
  6
  7   return 0;
  8 }
  ```
  Right terminal notes now normal mode, cursor 35,1.
- [04:43] Insert mode 3,1:
  ```
  1 #include <stdio.h>
  2
  3 |
  4
  5 int main(void) {
  6
  7   while (1) {
  8
  9   }
  10
  11   return 0;
  12 }
  ```
- [04:51] Insert mode 4,1:
  ```
  1 #include <stdio.h>
  2
  3 #define MAX_LINE 1024
  4 |
  5
  6 int main(void) {
  7
  8   while (1) {
  9
  10  }
  11
  12  return 0;
  13 }
  ```
- [05:09] Insert mode 3,3: programmer is renaming/adding defines:
  ```
  1 #include <stdio.h>
  2
  3 //|
  4 #define MAX_LINEs 1024
  5 #define MAX_LINE_LEN 1024
  6
  7 int main(void) {
  8
  9   while (1) {
  10
  11  }
  12
  13  return 0;
  14 }
  ```
  (webcam shows him with a cigarette in mouth). No save yet.

## File states

### main.c (new file, end of segment 2, 05:09, unsaved, vim INSERT mode at 3,3)
```
 1| #include <stdio.h>
 2| 
 3| //⟨cursor⟩
 4| #define MAX_LINEs 1024
 5| #define MAX_LINE_LEN 1024
 6| 
 7| int main(void) {
 8| 
 9|   while (1) {
10| 
11|   }
12| 
13|   return 0;
14| }
```
Indent is 2 spaces (vim default template); no :w message seen in this segment other than `"main.c" [New]` at open.

Intermediate (05:09 predecessor, 04:51):
```
 1| #include <stdio.h>
 2| 
 3| #define MAX_LINE 1024
 4| 
 5| 
 6| int main(void) {
 7| 
 8|   while (1) {
 9| 
10|   }
11| 
12|   return 0;
13| }
```
Initial (04:09):
```
 1| #include <stdio.h>
 2| 
 3| int main(void) {
 4|   
 5|   return 0;
 6| }
```

### Notes file (right terminal; filename not shown; probably notes.txt), as of end of segment (35 lines)
```
 1| 
 2| ------------- ed text editor ------------
 3| 
 4|     "ed is the standard text editor"
 5|     Runs everywhere, even on a microwave
 6| 
 7| * Created by Ken Thompson in 1971
 8|     - Used to write early Unix versions.
 9| * Not TUI , pure CLI editor
10|     - line-oriented editor
11| * Can be built using only C standard library
12|     - no termios, curses, or TUI dependencies.
13|     - Present everywhere
14| * Works headless:
15|     - runs via scripts/pipes without a human observer.
16|     - AI love ed
17| * Evolution tree:
18|     - ed -> sed -> ex -> vi -> vim.
19| 
20| ---------------------------------------------
21| [num] - go to line
22|   $ - last line
23| 
24| i - Insert text before the current line
25| a - Append text after the current line
26|   . - Stop edit
27| 
28| d - Delete the current line
29| p - Print the current line
30|   ,p - Print all lines
31|   1,$p - Print all lines
32| w - Write the buffer to a file
33| q - Quit the editor
34| 
35| ed [path] - Read file to buffer
```
(Line 1 blank, line 2 header dashes: `------------- ed text editor ------------`; line 20 dashes ~41 chars long; the dash widths are approximate. The "-" characters in lines 8+ render as en/em-like dashes in the font; treated as `-`.)

### ChatGPT-generated ai.c (550 lines; only fragments seen, lines 341-376, 409-478, 511-549)
```
341|     }
342| 
343|     if (strcmp(cmd, "$") == 0) {
344|         if (b->len == 0) {
345|             puts("?");
346|             return 1;
347|         }
348| 
349|         b->cur = b->len;
350|         print_line(b, b->cur);
351|         return 1;
352|     }
353| 
354|     /* Address followed by a print command, e.g. 3p */
355|     if (isdigit((unsigned char)*cmd)) {
356|         arg = cmd;
357| 
358|         while (isdigit((unsigned char)*cmd))
359|             cmd++;
360| 
361|         {
362|             char saved = *cmd;
363|             *cmd = '\0';
364| 
365|             if (!parse_number(arg, &n)) {
366|                 *cmd = saved;
367|                 puts("?");
368|                 return 1;
369|             }
370| 
371|             *cmd = saved;
372|         }
373| 
374|         while (isspace((unsigned char)*cmd))
375|             cmd++;
376| 
```
```
409|             return 1;
410|         }
411| 
412|         edit_lines(b, position);
413|         return 1;
414|     }
415| 
416|     if (strcmp(cmd, "a") == 0) {
417|         if (b->len == 0)
418|             position = 0;
419|         else if (b->cur >= 1 && b->cur <= b->len)
420|             position = b->cur;
421|         else {
422|             puts("?");
423|             return 1;
424|         }
425| 
426|         edit_lines(b, position);
427|         return 1;
428|     }
429| 
430|     if (cmd[0] == 'w' &&
431|         (cmd[1] == '\0' || isspace((unsigned char)cmd[1]⟨line cut off⟩
432|         arg = cmd + 1;
433| 
434|         while (isspace((unsigned char)*arg))
435|             arg++;
436| 
437|         if (*arg != '\0') {
438|             if (strlen(arg) >= 4096) {
439|                 puts("?");
440|                 return 1;
441|             }
442| 
443|             *filename = realloc(*filename, strlen(arg) + ⟨cut off⟩
444|             if (!*filename) {
445|                 fprintf(stderr, "Error: out of memory\n"⟨cut off⟩
446|                 return 0;
447|             }
448| 
449|             strcpy(*filename, arg);
450|         }
451| 
452|         if (!*filename) {
453|             puts("?");
454|             return 1;
455|         }
456| 
457|         write_file(b, *filename);
458|         return 1;
459|     }
460| 
461|     /* Read a file into the buffer, replacing its conten⟨cut off⟩
462|     if (strncmp(cmd, "ed", 2) == 0 &&
463|         (cmd[2] == '\0' || isspace((unsigned char)cmd[2]⟨cut off⟩
464|         arg = cmd + 2;
465| 
466|         while (isspace((unsigned char)*arg))
467|             arg++;
468| 
469|         if (*arg == '\0') {
470|             puts("?");
471|             return 1;
472|         }
473| 
474|         if (strlen(arg) >= 4096) {
475|             puts("?");
476|             return 1;
477|         }
478| 
```
```
511|     }
512| 
513|     if (argc == 2) {
514|         filename = malloc(strlen(argv[1]) + 1);
515| 
516|         if (!filename) {
517|             fprintf(stderr, "Error: out of memory\n");
518|             return EXIT_FAILURE;
519|         }
520| 
521|         strcpy(filename, argv[1]);
522| 
523|         if (!load_file(&b, filename)) {
524|             free(filename);
525|             free_buffer(&b);
526|             return EXIT_FAILURE;
527|         }
528|     }
529| 
530|     while (running) {
531|         cmd = read_line(stdin);
532| 
533|         if (!cmd) {
534|             if (feof(stdin))
535|                 break;
536| 
537|             fprintf(stderr, "Error: unable to read comma⟨cut off⟩
538|             break;
539|         }
540| 
541|         running = execute(&b, cmd, &filename);
542|         free(cmd);
543|     }
544| 
545|     free(filename);
546|     free_buffer(&b);
547| 
548|     return EXIT_SUCCESS;
549| }
```
(This is the discarded ChatGPT sample, shown only to be rejected. Indent 4 spaces.)

### Test file hello.txt (created with system `ed`, 30 bytes)
```
1| hello they
2| my name is hayyoth
```
Later (after `d` on line 1 and write): only `my name is hayyoth`.

## Shell & outputs

- [02:11] `lab $ ls` -> `ai.c      notes.txt`; `lab $ e ai.c` (opens vim; `e` is likely an alias for vim; later quit with `:q` at 02:10 earlier in video order... note the `:q` frame at 02:10 precedes the shell frame at 02:11).
- [02:35-03:04] System `ed` session (prompt-less):
  ```
  lab $ ed
  a
  hello they
  my name is hayyoth
  .
  w hello.txt
  30
  q
  lab $ 
  ```
- [03:07-03:10] `lab $ cat hello.txt` -> `hello they` / `my name is hayyoth`.
- [03:17] `lab $ ed hello.txt` -> `30`
  then typed commands/outputs [03:24-03:46]:
  ```
  1
  hello they
  2
  my name is hayyoth
  p
  my name is hayyoth
  1
  hello they
  p
  hello they
  p
  hello they
  ```
  (then, between 03:50 and 03:57, presumably `d`, `w`, `q`; not captured.)
- [04:02] `lab $ cat hello.txt` -> `my name is hayyoth`
- [04:09] vim opened on `main.c`: status `"main.c" [New]`.
- No compiler commands in this segment.

## Observations

- Opening (1:29-1:41): programmer scrolls through a 549-line ChatGPT-generated ed clone (ai.c) and writes on a sticky note "this is chatgpt code / 550 line / too complicated" -- the premise is to reject it and hand-write a minimal version ("100 lines"). Quits with `:q`.
- Shell prompt is `lab $ `; `e` is a custom alias/command for opening vim (`e ai.c`). Directory contains ai.c and notes.txt.
- Notes file (notes.txt) is his spec: command list [num], $, i, a, ., d, p, ,p, 1,$p, w, q, `ed [path]`. Also "Works headless" and "AI love ed" points.
- He demos the real system `ed` to learn its behavior, with sticky notes labeling each: ". is end edit", "save file", "quit", "[num] -> go to line", "p -> print current line", "d -> delete". Observed real-ed behavior: `w file` prints byte count (30), `ed file` prints byte count on open, a line number prints that line and sets current, `p` prints current line, `d` deletes current line.
- Then starts main.c in vim: begins with a template (#include <stdio.h>, int main(void) { return 0; }), 2-space indent, adds `while (1) { }` command-loop skeleton first, then `#define MAX_LINE 1024`, then changes to `MAX_LINEs 1024` and `MAX_LINE_LEN 1024` (two defines: max number of lines and max line length), with a `//` comment started above (content unseen).
- Style: K&R-ish brace on same line, 2-space indent, short names, uses vim insert mode, mostly typing from scratch.
