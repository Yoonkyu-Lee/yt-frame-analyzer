# Segment 14 (53:32 - 59:20)

Layout throughout: left Terminal window with two tabs ("~/lab — vim main.c" and a shell/a.out tab); right two Terminal windows (front white one: real `ed` session + `cat text.txt`, static all segment); "Human" webcam overlay ignored.

## Timeline

- [53:32] Left window, tab 2 "~/lab — a.out" active: `lab $ cc main.c`, `lab $ ./a.out`, `w`, `on assign ???`, `on saved ???`, `q`, `lab $ `. Right front window (static whole segment): `lab $ ed` / `this is text` / `?` / `a` / `this is text` / `.` / `w` / `?` / `w text.txt` / `13` / `q` / `lab $ cat text.txt` / `this is text` / `lab $ `.
- [53:34] Tab 2 title now "~/lab — -zsh". Ran `lab $ cc main.c` -> 1 warning (-Wformat, see Shell).
- [53:37] vim tab, lines 53-88. Status `"main.c" 96L, 2158B written`, cursor 69,56, 86%. Line 69: `printf("on assign %s - %d\n", path, sizeof(path));`.
- [53:43] Shell tab, same warning output still shown.
- [54:02] vim tab (other tab title "a.out"). Line 69 now `printf("on assign %s - %zu\n", path, strlen(path));` (cursor on the `u` of %zu, 69,36). Status `"main.c" 96L, 2159B written`.
- [54:32] vim lines 45-80, cursor 45,23, 73% (scrolling up).
- [54:33] lines 31-66, 31,7, 50%.
- [54:34] lines 22-57, 22,14, 35%.
- [54:35] lines 19-54, 19,17, 30%.
- [54:36] lines 17-53 (line 18 not yet drawn, mid-scroll), cursor 17,0-1, 26%.
- [54:37] Line 18 shows `char filename[255];` (cursor on `;`, 18,21). No initializer yet.
- [54:52] Line 18 now `char filename[255] = {0};`. View lines 15-50, cursor 15,23. Line 16: `int count_line = 0; // count line in lines`. Webcam: cigarette.
- [54:53] View lines 12-47, ex prompt `:` open at bottom (about to :w).
- [54:55] Shell tab: previous warning output, then `lab $ cc main.c`, `lab $ ./a.out`, `w`, `on assign ??? - 4`, `on saved ???`, program waiting for input.
- [54:57] Shell tab cleared: `lab $ cc m` (typing).
- [55:08] vim lines 12-47, status `"main.c" 96L, 2165B written` (+6 bytes = ` = {0}`), 15,23, 18%.
- [55:27] vim 30-65, cursor 30,43, 48%.
- [55:28] vim 69-96 (Bot), cursor 69,54 (parens of `strlen(path)` highlighted). Lines 92-96: `for (int i = 1; i < count_line; i++)` / `free(lines[i]);` / blank / `return 0;` / `}`.
- [55:29] vim 63-96, 69,54, Bot.
- [55:35] vim 60-95, cursor 60,35, 98%.
- [55:36] vim 58-93, cursor 59,7, 95%.
- [55:44] Debug printf line deleted: 67 `if (*path == '\0') {`, 68 `path = filename;`, 69 `}` (cursor 69,9), 70 blank, 71 `if (strlen(path) == 0) {`. File now 93 lines ending `return 0;` at 93.. (line 93 shown blank at bottom, 96% view).
- [55:45] Closing brace of that if removed too: 67 `if (*path == '\0') {`, 68 `path = filename;`, 69 blank, 70 `if (strlen(path) == 0) {`; cursor 67,1, 98%. (Transiently unbalanced.)
- [56:09] Opening brace removed: 67 `if (*path == '\0')`, 68 `path = filename;`. INSERT mode, line 75: `FILE *fp = fopen(path, "` (typing, 75,33). Rest of old code shown in red (syntax highlight confused by open quote).
- [56:18] Line 75 `FILE *fp = fopen(path, "w");`; blank line 76 (INSERT 76,9); `} else {` at 77.
- [56:22] 76 `if (!fp) {`, 77 typing `per`; `} else {` at 78.
- [56:26] 77 `perror(path);`, 78 blank (INSERT 78,11), `} else {` at 79.
- [56:36] 76 `if (!fp) {` / 77 `perror(path);` / 78 `continue;` / 79 `}` (braces highlighted); `} else {` at 80.
- [56:37] Blank line 80 (INSERT 80,9); `} else {` at 81.
- [56:51] Blank lines 80-81 (INSERT 81,9); `} else {` at 82.
- [57:09] Line 81: `for (int i = 1; i < count_line; i++)`; line 82 blank (INSERT 82,11).
- [57:33] Line 82: `fprintf("%s\n", lines[i]);` (no `fp` arg); line 83 blank (83,9).
- [57:34] Lines 83-84 blank (INSERT 84,9); `} else {` at 85.
- [58:10] `fclose(fp);` appears at line 86 (INSERT 84,9; blank lines 83-85).
- [58:20] Line 84 `if (strlen(filename) == 0)`; line 85 blank (INSERT 85,11); 86 blank; 87 `fclose(fp);`.
- [58:46] Line 84 holds just `/` (INSERT 84,10); 85 `if (strlen(filename) == 0)`; 86 `strncpy(filename, path, sizeof(filename));`; 87 blank; 88 `fclose(fp);`.
- [58:52] Stray `/` removed: 83 blank, 84 `if (strlen(filename) == 0)`, 85 `strncpy(filename, path, sizeof(filename));`, 86 blank, 87 `fclose(fp);`. Normal mode, cursor 84,9.
- [59:08] a.out tab: `lab $ cc main.c`, `lab $ ./a.out`, `w`, `on assign  - 0`, `?`, `w`, `on assign  - 0`, `?`, program waiting. (Binary built from older code that still has the on-assign printf; filename now empty/zero-initialized.)
- [59:11] Tab title "-zsh". `cc main.c` gives 2 warnings on line 82 `fprintf("%s\n", lines[i]);` (verbatim in Shell).
- [59:13] vim: status `"main.c" 106L, 2382B written`, cursor 84,9, 81%. Content same as 58:52 (line 82 still lacks `fp`).
- [59:20] Shell tab cleared: `lab $ ` (empty).

## File states

### main.c at end of segment (59:13; 106L, 2382B written). Lines 1-11 not visible in this segment. Lines 94-106 not visible at that time (inferred, see below).

```
1-11| ?? (not visible this segment)
12| int main(int argc, char **argv) {
13|   char cmd[MAX_LINE_LEN];
14|   char *lines[MAX_LINES];
15|   int current_line = 0;
16|   int count_line = 0; // count line in lines
17| 
18|   char filename[255] = {0};
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
61|       }
62|       else if (strncmp(cmd, "w", 1) == 0) { // save
63|         char *path = cmd + 1;
64|         while (*path == ' ')
65|           path++;
66| 
67|         if (*path == '\0')
68|           path = filename;
69| 
70|         if (strlen(path) == 0) {
71|           puts("?");
72|           continue;
73|         }
74| 
75|         FILE *fp = fopen(path, "w");
76|         if (!fp) {
77|           perror(path);
78|           continue;
79|         }
80| 
81|         for (int i = 1; i < count_line; i++)
82|           fprintf("%s\n", lines[i]);
83| 
84|         if (strlen(filename) == 0)
85|           strncpy(filename, path, sizeof(filename));
86| 
87|         fclose(fp);
88|       } else {
89|         char *endptr;
90|         int num = strtol(cmd, &endptr, 10);
91| 
92|         if (*endptr == '\0' && num > 0 && num <= count_line) {
93|           current_line = num;
94|           puts(lines[current_line]);      [inferred: continuation of pre-edit text; 106L total matches]
95|         } else {                          [inferred]
96|           puts("?");                      [inferred]
97|         }                                 [inferred]
98|       }                                   [inferred]
99|     }                                     [inferred]
100|  }                                       [inferred]
101| 
102|   for (int i = 1; i < count_line; i++)   [inferred]
103|     free(lines[i]);                       [inferred]
104| 
105|   return 0;                               [inferred]
106| }                                         [inferred]
```
Indentation is 2 spaces per level. Lines 94-106 are not directly visible after the edits (they were visible at 55:28-55:36 as lines 84-96 of the earlier 96-line version); exact column of 98-100 braces is `⟨?⟩` (follow the earlier version's 4/2/0 closing pattern).

### Intermediate version of the `w` block (55:08, 96L, 2165B written)

```
62|       else if (strncmp(cmd, "w", 1) == 0) { // save
63|         char *path = cmd + 1;
64|         while (*path == ' ')
65|           path++;
66| 
67|         if (*path == '\0') {
68|           path = filename;
69|           printf("on assign %s - %zu\n", path, strlen(path));
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
89|     }
90|   }
91| 
92|   for (int i = 1; i < count_line; i++)
93|     free(lines[i]);
94| 
95|   return 0;
96| }
```
At 53:37 (96L, 2158B) line 69 was `printf("on assign %s - %d\n", path, sizeof(path));`; line 18 was `char filename[255];` until between 54:37 and 54:52.

### Other files
Right terminal shows only the real-ed transcript (static). No notes file, sticky notes, or other files viewed.

## Shell & outputs

[53:32] left terminal, tab a.out:
```
lab $ cc main.c
lab $ ./a.out
w
on assign ???
on saved ???
q
lab $ 
```
[53:34] (also seen 53:43, 54:55):
```
lab $ cc main.c
main.c:69:47: warning: format specifies type
      'int' but the argument has type 'unsigned long' [-Wformat]
   69 |           printf("on assign %s - %d\n", path, sizeof(path)
);
      |                                  ~~          ^~~~~~~~~~~~~
      |                                  %lu
1 warning generated.
lab $ 
```
[54:55]:
```
lab $ cc main.c
lab $ ./a.out
w
on assign ??? - 4
on saved ???
```
(`???` is how the screen renders the garbage uninitialized filename; program waiting for input.)

[54:57] cleared, typing `lab $ cc m`.

[59:08]:
```
lab $ cc main.c
lab $ ./a.out
w
on assign  - 0
?
w
on assign  - 0
?
```
[59:11]:
```
lab $ cc main.c
main.c:82:19: warning: incompatible pointer
      types passing 'char[4]' to parameter of type 'FILE *' (aka
      'struct __sFILE *') [-Wincompatible-pointer-types]
   82 |           fprintf("%s\n", lines[i]);
      |                   ^~~~~~
/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/usr/include/_s
tdio.h:245:31: note:
      passing argument to parameter here
  245 | int     fprintf(FILE * __restrict, const char * __restrict
...
      |                                ^
main.c:82:27: warning: format string is not a
      string literal (potentially insecure) [-Wformat-security]
   82 |           fprintf("%s\n", lines[i]);
      |                           ^~~~~~~~~
main.c:82:27: note: treat the string as an
      argument to avoid this
   82 |           fprintf("%s\n", lines[i]);
      |                           ^
      |                           "%s",
2 warnings generated.
lab $ 
```
[59:20] screen cleared: `lab $ `.

Right window (static all segment): real ed: `w` with no filename gives `?`; `w text.txt` prints byte count `13`.

## Observations

- Work in this segment is on the `w` (save) command. First debug printfs ("on assign", "on saved") to verify default-filename logic; fixed a `%d` vs `sizeof` warning with `%zu`/`strlen`; discovered `filename` was uninitialized garbage and fixed with `= {0}`.
- Behavior modeled on real `ed` (reference window): `w` without filename and no remembered file prints `?`; `w name` writes and (in real ed) prints byte count.
- Rewrote `w` into real saving: path defaults to `filename`; `strlen(path)==0` -> `?`; `fopen(path, "w")`, `perror(path); continue;` on failure; write loop via `fprintf`; remember filename on first save (`if (strlen(filename) == 0) strncpy(...)`); `fclose(fp)`. Dropped braces on single-statement `if (*path == '\0')`.
- Known bugs visible at segment end: `fprintf("%s\n", lines[i])` lacks the `fp` argument (compiler warnings at 59:11); loop `i < count_line` starting at 1 likely skips the last line; no byte count printed yet.
- Style: 2-space indent, trailing `//` comments, braces omitted for single statements, tight edit-save-compile-run loop with plain `cc main.c` (no -Wall).
- Save checksums: 96L 2158B (53:37) -> 96L 2159B (54:02) -> 96L 2165B (55:08) -> 106L 2382B (59:13).
