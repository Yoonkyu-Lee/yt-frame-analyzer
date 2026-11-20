# Segment 12 (45:53 - 47:45)

## Timeline
Layout throughout: left Terminal window with two tabs ("~/lab — vim main.c", "~/lab — -zsh"); right Terminal window (white) showing a prior real `ed` session; webcam "Human" overlay (ignored).

- [45:53] Left window, tab "~/lab — -zsh" active. History: `lab $ man 3 strncmp`, `lab $ man 3 strcpy`, typing `lab $ cc ma` (cursor after "ma").
- [45:54] Ran `cc main.c`. Compiler warning at main.c:33:49 about 'strncpy' call / -Wsizeof-pointer-memaccess (verbatim in Shell section); code line 33 at that time: `char *test = strncpy(filename, path, sizeof(path));`. "1 warning generated."
- [46:08] Switched to tab "~/lab — vim main.c" (normal mode). Visible lines 11-46 of main.c (File states, version A). Status: `"main.c" 90L, 2051B written`, cursor `37,0-1`, 18%. Line 33 still `sizeof(path)`. Lines 34-35 are debug printfs.
- [46:14] Back to -zsh tab, same compile warning still on screen.
- [46:15] Screen cleared: only `lab $ cc main.c` then blank line (compiling / no output yet).
- [46:29] zsh history: `lab $ cc main.c`, `lab $ ./a.out`, `lab $ ./a.out hello.txt` -> `file name is hello.txt` / `104`; typing `lab $ ./a.out abc.tx`.
- [46:33] Ran `./a.out abc.txt` -> `abc.txt: No such file or directory`, `file name is abc.txt`, `97` (the "file name is abc.txt" line is highlighted by mouse selection). (104/97 are first chars 'h'/'a' from `printf("%d\n", *test)`.)
- [46:35] Back in vim. Line 33 now `char *test = strncpy(filename, path, sizeof(filename));` (sizeof(path) -> sizeof(filename); cursor on last "e", `33,56`). Status: `"main.c" 90L, 2055B written`. Mouse I-beam near line 19.
- [46:42] VISUAL LINE mode selecting lines 34-35 (the two debug printf lines). Status `-- VISUAL LINE --   2   35,27   18%`.
- [46:44] Lines 34-35 deleted. Now line 33 `char *test = strncpy(filename, path, sizeof(filename));`, 34 `}`, 35 blank, 36 `return 0;`, 37 blank, 38 `int on_edit = 0;`. Status `33,3  19%` (normal mode, no message).
- [46:53] INSERT mode on line 33: `char *test = ` removed, line now `strncpy(filename, path, sizeof(filename));`. macOS Dock popped up (tooltip "Terminal"); mouse near Dock.
- [46:54] A Firefox window opened over the right side (new tab page, "Search with DuckDuckGo or enter address"). vim still INSERT, `33,5`.
- [46:55] Typing URL `beej.us/`; autocomplete dropdown (Beej's Web Page; Firefox Suggest: Beej's Guide to C Programming Library, etc.; also YouTube/Proton mail suggestions).
- [46:56] URL `beej.us/b`.
- [46:57] URL `beej.us/guide/`.
- [46:58] URL `beej.us/guide/bgclr/` (autocompleted).
- [47:00] Page loaded: "Beej's Guide to C Programming—Library Reference" ("second volume of Beej's Guide to C, the library reference"; links HTML, HTML widescreen, HTML single page...).
- [47:01] Firefox window moved/enlarged left (covers vim); same page.
- [47:03] Navigated to beej.us/guide/bgclr/html/split/: "Beej's Guide to C Programming, Library Reference, Brian "Beej Jorgensen" Hall, v0.9.13, Copyright © October 15, 2026" with TOC (1 Foreword ... 2 The C Language ...).
- [47:04] Find-in-page bar opened, typed "str" (highlight in "Distribution").
- [47:05] Find text "strcpy": TOC section 26 `<string.h>` String Manipulation: 26.1 memcpy(), memccpy(), memmove(); 26.2 strcpy(), strncpy() (highlighted); 26.3 strdup(), strndup(); 26.4 strcat(), strncat(); 26.5 strcmp(), strncmp(), memcmp(); 26.6 strcoll(); 26.7 strxfrm(); 26.8 strchr(), strrchr(), memchr(); 26.9 strspn(), strcspn(); 26.10 strpbrk(); 26.11 strstr(); 26.12 strtok(); 26.13 memset(), memset_explicit; 26.14 strerror(); 26.15 strlen().
- [47:07] Opened section 26.2 strcpy(), strncpy() (URL .../split/stringref.html#man-strcpy). Synopsis: `char *strcpy(char *dest, char *src);` `char *strncpy(char *dest, char *src, size_t n);`. Description: "strncpy() is just like strcpy(), except only the first n characters are actually copied. Beware that if you hit the limit, n before you get a NUL terminator on the src string, your dest string won't be NUL-terminated. Beware! BEWARE!"; "You can terminate the string yourself by sticking the '\0' in there yourself".
- [47:09]-[47:11] Scrolling: example `char s[10]; char foo = "My hovercraft is full of eels."; strncpy(s, foo, 9); s[9] = '\0';`. Return Value: "Both functions return dest for your convenience, at no extra charge."
- [47:12]-[47:25] Scrolling the Example (lines 1-26): `#include <string.h>`, `int main(void)`, `char *src = "hockey hockey ..."; char dest[20]; int len; strcpy(dest, "I like "); len = strlen(dest); strncpy(dest+len, src, sizeof(dest)-len-1); dest[sizeof(dest)-1] = '\0'; // terminate`. See Also: memcpy(), strcat(), strncat(). Section 26.3 strdup(), strndup() ("New in C23!") briefly visible at 47:21. Scrolled back up to Example 47:24-47:25. Reading only; vim behind remained INSERT 33,5.
- [47:32] Firefox gone; vim (INSERT, `33,5`, 19%), line 33 `strncpy(filename, path, sizeof(filename));`. Unchanged from 46:53 (no NUL-terminator line added visible).
- [47:36] Vim normal mode scrolled: lines 15-50. Cursor `50,4`, 26%.
- [47:37] Scrolled to lines 20-55, cursor `55,4` 36%. Line 54 `else if (mode(cmd, "a")) // append`, 55 `on_edit = 1;`.
- [47:40] `return 0;` and a blank line removed: lines 34 `}`, 35 blank, 36 a whitespace-only/empty line with cursor (`36,2`, 37%), 37 `int on_edit = 0;`. Line 55 `else if (mode(cmd, "p")) { // print`.
- [47:41] Line 36 now `int on_edit = 0;` (cursor `36,3`); `return 0;` gone. 54 `else if (mode(cmd, "p")) { // print`, 55 `if (count_line > 0 )`.
- [47:43] Scrolled to 23-58, `58,3`, 44%: print branch 54-58.
- [47:44] Scrolled to 36-71, `71,0-1`, 70%: `d` stub (60-61), `w` save branch, start of numeric-line branch.
- [47:45] Scrolled to 49-84, `84,0-1`, 96%: end of loop and `for (...) free(lines[i]);` (82-83). No save message in this part.

## File states

### main.c as of 46:08 (version A, lines 11-46 visible; saved `"main.c" 90L, 2051B written`)
Indentation is 2 spaces per level.
```
11|
12| int main(int argc, char **argv) {
13|   char cmd[MAX_LINE_LEN];
14|   char *lines[MAX_LINES];
15|   int current_line = 0;
16|   int count_line = 0; // count line in lines
17|
18|   char filename[255];
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
33|     char *test = strncpy(filename, path, sizeof(path));
34|     printf("file name is %s\n", filename);
35|     printf("%d\n", *test);
36|   }
37|
38|   return 0;
39|
40|   int on_edit = 0;
41|   while (1) {
42|     if (!fgets(cmd, MAX_LINE_LEN, stdin)) break;
43|     cmd[strcspn(cmd, "\r\n")] = 0;
44|
45|     if (on_edit) {
46|       if (mode(cmd, "."))
```
(At 46:35 only line 33 changed: `sizeof(filename)`; saved 2055B.)

### main.c at END of segment (47:45)
Lines 1-10 never visible in this segment. Lines 85+ not visible.
```
?? (lines 1-10 not visible)
11|
12| int main(int argc, char **argv) {
13|   char cmd[MAX_LINE_LEN];
14|   char *lines[MAX_LINES];
15|   int current_line = 0;
16|   int count_line = 0; // count line in lines
17|
18|   char filename[255];
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
62|       else if (strncmp(cmd, "w", 1)) { // save
63|         char *path = cmd + 1;
64|         while (*path == ' ') path++; // trim space
65|
66|         if (*path == '\0')
67|           path = filename;
68|       } else {
69|         char *endptr;
70|         int num = strtol(cmd, &endptr, 10);
71|
72|         if (*endptr == '\0' && num > 0 && num <= count_line) {
73|           current_line = num;
74|           puts(lines[current_line]);
75|         } else {
76|           puts("?");
77|         }
78|       }
79|     }
80|   }
81|
82|   for (int i = 1; i < count_line; i++)
83|     free(lines[i]);
84|
?? (lines 85+ not visible)
```
Notes: `return 0;` (formerly line 38) deleted at 47:40-47:41. `w` branch has no write code yet. `strncmp(cmd, "w", 1)` has no `!` as shown. Line 55 has a space before `)`: `if (count_line > 0 )`. Exact brace/indent of lines 59-61 as in frame at 47:44.

No other files (notes file, text.txt) edited. text.txt content (from earlier real-ed session on right terminal): `this is text`.

## Shell & outputs
Left terminal, tab -zsh (prompt `lab $ `):
- [45:53] history: `man 3 strncmp`, `man 3 strcpy`, then typing `cc ma`
- [45:54] `cc main.c` output (line breaks as wrapped on screen):
```
main.c:33:49: warning: 'strncpy' call
      operates on objects of type 'const char' while the size is
      based on a different type 'const char *'
      [-Wsizeof-pointer-memaccess]
   33 |     char *test = strncpy(filename, path, sizeof(path));
      |                                  ~~~~     ^~~~
main.c:33:49: note: did you mean to provide
      an explicit length?
   33 |     char *test = strncpy(filename, path, sizeof(path));
      |                                              ^~~~
1 warning generated.
lab $
```
(caret/tilde positions approximate)
- [46:15] screen cleared: `lab $ cc main.c` (running, no output yet)
- [46:29]
```
lab $ cc main.c
lab $ ./a.out
lab $ ./a.out hello.txt
file name is hello.txt
104
lab $ ./a.out abc.tx_
```
- [46:33]
```
lab $ ./a.out abc.txt
abc.txt: No such file or directory
file name is abc.txt
97
lab $
```
(`./a.out` with no args produced no output shown.)

Right terminal (static throughout; earlier real ed session):
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

vim messages: [46:08] `"main.c" 90L, 2051B written`; [46:35] `"main.c" 90L, 2055B written`.

## Observations
- Fixing compiler warning for `strncpy(filename, path, sizeof(path))`: sizeof(pointer) wrong, changed to `sizeof(filename)` (46:35, +4 bytes). Verified by running a.out with hello.txt / abc.txt using debug `printf("%d\n", *test)`.
- Then removed debug printfs and `char *test =`, and consulted Beej's Guide to C Programming Library Reference (section 26.2 strcpy/strncpy; skimmed 26.3 strdup/strndup) re: strncpy NUL termination. No NUL-terminator line visibly added in this segment.
- Removed stray `return 0;` before the command loop.
- Style: 2-space indent, `//` trailing comments naming command ("append", "print", "delete", "save", "trim space"), `puts("?")` as ed's error reply, `mode(cmd, "x")` helper for matching, strtol for numeric line addressing, free loop at end. `d` and `w` still stubs.
- Programmer uses man pages (`man 3 strncmp`, `man 3 strcpy`) and Beej's guide; real `ed` session on right used as behavior reference (`w` with no filename gives `?`, `w text.txt` prints byte count 13).
