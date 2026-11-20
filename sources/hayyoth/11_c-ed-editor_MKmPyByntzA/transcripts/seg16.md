# Segment 16 (67:22 - 70:03) - final segment

## Timeline
- [67:22] Left Terminal (tabs "~/lab — vim main.c" active, "~/lab — -zsh"), vim main.c, NORMAL mode, view lines 51-86. Status: `"main.c" 112L, 2585B written` at 86,0-1, 65%. In the `d` branch, lines 60-67 are `else if (mode(cmd, "d")) { // delete` / `if (count_line > 0) {` / `free(lines[current_line]);` / `for (int i = current_line; i < count_line; i++)` / `lines[i] = lines[i + 1];` / `} else` / `puts("?");` / `}` -- NO `count_line--;` yet. Right terminal (vim, `-- INSERT --`, 9,17 All) shows scratch notes buffer (see File states). Webcam overlay ignored.
- [67:28] Same file, `-- INSERT --` at 65,11, 64%. A new empty line 65 has been opened between `lines[i] = lines[i + 1];` (64) and `} else` (now 66). File now 113 lines. About to type `count_line--;`.
- [67:41] Switched to ~/lab — -zsh tab. Shows prior session: `lab $ ls` -> `a.out  ai.c   main.c` (a.out red); `lab $ cc main.c`; `lab $ ./a.out` with ed session (see Shell). Then `lab $ cat line.txt` -> `line 1 / line 3 / line 4 / line 4` (duplicate last line = delete bug: count_line not decremented). Pre-fix run.
- [67:42] Terminal cleared; `lab $ cat line.txt` typed, not executed.
- [67:53] zsh tab title "~/lab — a.out". History: `lab $ cat line.txt`, `lab $ cc main.c`, `lab $ ls` -> `a.out    ai.c     line.txt main.c`, `lab $ rm line.txt`, `lab $ ./a.out` (running, waiting for input). (Recompiled after fix.)
- [68:20] Terminal cleared; `lab $ ./a.out` typed (not yet run), tab "~/lab — -zsh".
- [68:39] Output of the fixed run: `a / line 1 / line 2 / line 3 / line 4 / . / 2 / line 2 / d / w line.txt / q`, then `lab $ cat line.txt` -> `line 1 / line 3 / line 4` (correct).
- [68:42] Same; mouse selecting (gray highlight) the cat output.
- [68:44] Focus on right scratch vim: `-- (insert) VISUAL --`, `4` lines selected (1-4), 1,1 All. Line 9 now `move from 2 to 3` (no cursor block).
- [68:49] Back to vim main.c tab. Save message: `"main.c" 113L, 2609B written` at 65,23, 64%. Line 65 now `count_line--;`. Lines 51-86 visible.
- [68:50-69:02] Scrolling through main.c in normal mode, no edits (cursor positions: 48,7 61%; 41,18 50%; 41,5 6% (lines 6-41); 35,0-1 44%; 103,11 Bot; 85,11 Bot; 84,11 Bot; 77,11 98%; 68,7 87%; 57,11 72%; 50,11 63%; 37,11 46%; 36,11 45%). Gives a full view of the final program, lines 6-113; lines 1-5 never visible.
- [69:04] vim 83,11 (61%), lines 48-83. Cursor on `if(!fp) {` col 11.
- [69:05] Same; macOS Dock appeared, tooltip "Stickies".
- [69:06-69:07] Dock showing/hiding; "Stickies" tooltip; vim cursor hollow (unfocused).
- [69:09] Empty yellow Stickies note opened top-left over the left terminal.
- [69:12] Sticky moved over main.c mid-screen; text typed: `do` (typing "done").
- [69:17] Right terminal (notes vim, lines 15-35) brought to front, red error bar `E211: File "notes.txt" no longer available`. Sticky: `done` / `the idea is that`.
- [69:18] Right terminal bottom message `Type  :qa!  an...s and exit Vim`, cursor 28,3 Bot.
- [69:24] Sticky: `done` / `the idea is that` / (blank) / `note:` / `- it can have ` (typing). Right vim cursor 30,3.
- [69:35] Sticky (scrolled): `the idea is that` / / `note:` / `- it can have bug` / `- it not have full feature of` / `ed`.
- [69:36] Sticky has trailing empty line (scrollbar visible).
- [69:38] Right vim `-- VISUAL --`, `15`, 31,10 Bot, "Print all lines" selected on line 31. Sticky unchanged.
- [69:41] Left terminal tab "~/lab — -zsh" (prior ./a.out session + cat output, empty prompt). Right vim VISUAL, `23`, 31,2 Bot, line 31 highlighted. Sticky closed/hidden.
- [69:53] Left tab titled "~/lab — ed line.txt": ran the real system `ed line.txt`. Output `21`, typed `1,$p` -> line 1/line 3/line 4; `2,$p` -> line 3/line 4. (Real ed range commands, which the clone lacks.)
- [69:54] Mission Control/desktop spaces view: Desktop 1 / Desktop 2 (current) / Desktop 3; windows visible: left terminal, scratch vim window, notes terminal, the sticky.
- [69:55] Back to normal view; sticky over left terminal, scrolled: `note:` / `- it can have bug` / `- it not have full feature of` / `ed` / (blank).
- [70:01] Sticky: `note:` / `- it can have bug` / `- it not have full feature of` / `ed` / (blank) / `you implement your own ` (typing).
- [70:02] Sticky moved over the right notes terminal; same text.
- [70:03] macOS menu bar visible: `Stickies  File  Edit  Font  Color  Window  Help`, clock `Tue Oct 27 2:40 AM`. Final frame; sticky text unchanged ("you implement your own " unfinished).

## File states

### main.c (end of segment; 113 lines; last save `"main.c" 113L, 2609B written` at 68:49)
Lines 1-5 and the MAX_LINES define (used on line 14) NOT visible in this segment. Lines 6-113 verbatim (2-space indentation):
```
1| ?? (lines 1-5 not visible; presumably #includes)
2| ??
3| ??
4| ??
5| ??
6| #define MAX_LINE_LEN 1024
7| 
8| int mode(char *cmd, char *mode) {
9|   return (strcmp(cmd, mode) == 0);
10| }
11| 
12| int main(int argc, char **argv) {
13|   char cmd[MAX_LINE_LEN];
14|   char *lines[MAX_LINES];
15|   int current_line = 0;
16|   int count_line = 0; // count line in lines
17| 
18|   char filename[255] = {0}; // C add trash value to it
19|   if (argc > 1) {
20|     const char *path = argv[1];
21|     FILE *fp = fopen(path, "r");
22|     if (!fp) {
23|       printf("%s: No such file or directory\n", path);
24|     } else {
25|       char line[MAX_LINE_LEN];
26|       while (fgets(line, MAX_LINE_LEN, fp)) {
27|         line[strcspn(line, "\r\n")] = 0;
28|         current_line++;
29|         count_line++;
30|         lines[current_line] = strdup(line);
31|       }
32|     }
33|     strncpy(filename, path, sizeof(filename));
34|   }
35| 
36|   int on_edit = 0;
37|   while (1) {
38|     if (!fgets(cmd, MAX_LINE_LEN, stdin)) break;
39|     cmd[strcspn(cmd, "\r\n")] = 0;
40| 
41|     if (on_edit) {
42|       if (mode(cmd, "."))
43|         on_edit = 0;
44|       else {
45|         current_line++;
46|         lines[current_line] = strdup(cmd);
47|         count_line++;
48|       }
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
61|         if (count_line > 0) {
62|           free(lines[current_line]);
63|           for (int i = current_line; i < count_line; i++)
64|             lines[i] = lines[i + 1];
65|           count_line--;
66|         } else
67|           puts("?");
68|       }
69|       else if (strncmp(cmd, "w", 1) == 0) { // save
70|         char *path = cmd + 1;
71|         while (*path == ' ')
72|           path++;
73| 
74|         if (*path == '\0')
75|           path = filename;
76| 
77|         if (strlen(path) == 0) {
78|           puts("?");
79|           continue;
80|         }
81| 
82|         FILE *fp = fopen(path, "w");
83|         if (!fp) {
84|           perror(path);
85|           continue;
86|         }
87| 
88|         for (int i = 1; i <= count_line; i++)
89|           fprintf(fp, "%s\n", lines[i]);
90| 
91|         if (strlen(filename) == 0)
92|           strncpy(filename, path, sizeof(filename));
93| 
94|         fclose(fp);
95|       } else {
96|         char *endptr;
97|         int num = strtol(cmd, &endptr, 10);
98| 
99|         if (*endptr == '\0' && num > 0 && num <= count_line) {
100|           current_line = num;
101|           puts(lines[current_line]);
102|         } else {
103|           puts("?");
104|         }
105|       }
106|     }
107|   }
108| 
109|   for (int i = 1; i < count_line; i++)
110|     free(lines[i]);
111| 
112|   return 0;
113| }
```
(Blank-line whitespace assumed empty. Line 55 really has `0 )` with a space.)

Intermediate version at 67:22 (112L, 2585B): identical except `count_line--;` absent (`} else` was line 65; later lines one number lower).

### Scratch vim buffer (back-right window, `-- INSERT --` 9,17)
```
1| this is line 1
2| xxxxxxxxxxxxxx // current line - delete it
3| this is line 3
4| this is line 4
5| 
6| 
7| (count line 4)
8| ------------
9| move from 2 to 3
10|   lines[i] = lines[i + 1]
```

### notes.txt (front-right vim, lines 15-35 only visible)
```
15|     - runs via scripts/pipes without a human observer.
16|     - AI love ed
17| * Evolution tree:
18|     - ed -> sed -> ex -> vi -> vim.
19| 
20| ----------------------------------------
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
(Lines 1-14 not visible.)

### line.txt
Fixed run: `line 1` / `line 3` / `line 4`. Pre-fix run: `line 1` / `line 3` / `line 4` / `line 4`.

### Sticky note (final)
```
done
the idea is that

note:
- it can have bug
- it not have full feature of
ed

you implement your own 
```

## Shell & outputs
- [67:41] pre-fix session:
```
lab $ ls
a.out  ai.c   main.c
lab $ cc main.c
lab $ ./a.out
a
line 1
line 2
line 3
line 4
.
1
line 1
2
line 2
3
line 3
2
line 2
d
p
line 3
w line.txt
q
lab $ cat line.txt
line 1
line 3
line 4
line 4
lab $ 
```
(Includes echoed user input; `1`/`2`/`3` replies `line N`; after `d` on line 2, `p` printed `line 3`.)
- [67:53]:
```
lab $ cat line.txt
lab $ cc main.c
lab $ ls
a.out    ai.c     line.txt main.c
lab $ rm line.txt
lab $ ./a.out
```
- [68:39] fixed run:
```
lab $ ./a.out
a
line 1
line 2
line 3
line 4
.
2
line 2
d
w line.txt
q
lab $ cat line.txt
line 1
line 3
line 4
lab $ 
```
- [69:53] real ed:
```
lab $ ed line.txt
21
1,$p
line 1
line 3
line 4
2,$p
line 3
line 4
```
- vim messages: `E211: File "notes.txt" no longer available` [69:17]; `Type  :qa!  an...s and exit Vim` [69:18].
- vim saves: `"main.c" 112L, 2585B written` [67:22]; `"main.c" 113L, 2609B written` [68:49].

## Observations
- Bug found by testing: delete did not decrement count_line, so last line was duplicated in saved output. Fix: `count_line--;` after the shift loop (typed ~67:28-68:49); recompiled with `cc main.c`, removed stale line.txt, re-ran, output correct.
- Test script typed by hand: `a`, 4 lines, `.`, `2`, `d`, `w line.txt`, `q`, then `cat line.txt`.
- Final program is 113 lines. Style: 2-space indent, braceless single-statement bodies, short `// append` `// print` `// delete` `// save` comments, `puts("?")` for all errors (mimics ed), `perror(path)` on fopen failure, strncpy into 255-byte filename, strtol for numeric goto.
- Quirks: lines indexed from 1 (lines[0] unused); final free loop `i < count_line` skips last line (leak/off-by-one); `i`, `$`, `,p`, `1,$p` listed in notes but not implemented.
- Real `ed line.txt` demo with `1,$p` and `2,$p` contrasts with the clone's missing range support.
- Closing sticky: "done / the idea is that / note: - it can have bug - it not have full feature of ed / you implement your own " (unfinished in last frame): toy for educational purposes, viewers encouraged to extend it.
- Clock at end: Tue Oct 27 2:40 AM; Desktop 2 of 3.
