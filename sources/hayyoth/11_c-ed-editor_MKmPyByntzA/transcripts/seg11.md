# Segment 11 (40:27 - 45:53)

## Timeline
- [40:27] Left Terminal window, tabs "~/lab — vim main.c" and "~/lab — man 3 strncmp" (active). Shows man page STRCMP(3): SYNOPSIS `#include <string.h>`, `int strcmp(const char *s1, const char *s2);`, `int strncmp(const char *s1, const char *s2, size_t n);`. Right Terminal (white) shows the reference real-`ed` session (see Shell). Behind it a browser (beej.us/guide/bgclr/html/split/stdlib.html#man-strtol). Webcam "Human" ignored.
- [40:31] Tab title now "~/lab — less ◂ man 3 strncmp". User mouse-selects the strncmp prototype lines.
- [40:32] Selection spans `int` + `strncmp(const char *s1, const char *s2, size_t n);`.
- [40:33-40:35] Scrolling down in less: DESCRIPTION, RETURN VALUES, SEE ALSO (bcmp, memcmp, strcasecmp, strcoll, strxfrm, wcscmp), STANDARDS ("The strcmp() and strncmp() functions conform to ISO/IEC"). Prompt `:`.
- [40:35] Browser comes to front on the right: beej.us strtol page, Synopsis (`long int strtol(const char * restrict nptr, char ** restrict endptr, int base);`, strtoll, strtoul, strtoull), find-bar contains "strtol". "These convert a string to an integer like atoi(), but they have a few more bells and whistles. Most notable, they can tell you where ... characters, if any, appear."
- [40:36] Browser gone; two Terminal windows on right; left still man strncmp.
- [40:53] Switched to vim main.c tab. Normal mode, cursor 62,0-1, 80%. Lines 37-72 visible. Line 59 `else if (mode(cmd, "d")) { // delete`, 60 `}`, 61 `else if (mode(cmd, "w")) { // save`, 62 empty (cursor), 63 `} else {`.
- [41:01] Back to man strncmp tab.
- [41:02] vim INSERT, 59,23, 80%. Line 59 reads `else if (strncmd(cmd, "d")) { // delete` (typo `strncmd`, replacing `mode` while editing).
- [41:19] Normal, 61,34, 80%. Line 59 restored to `else if (mode(cmd, "d")) { // delete`. Line 61 now `else if (strncmp(cmd, "w", 1)) { // save` (cursor highlights `1`).
- [41:37] INSERT, 63,9, 78%. New line 62 `char *path = cmd + 1;`, line 63 empty cursor, `} else {` now line 64.
- [41:55] Normal, 64,0-1, 76%. Line 63 `while (*path == ' ') path++;`, 64 empty.
- [42:19] Normal, 80,19, 95%. Scrolled to bottom. Line 63 now ends with `// trim space`. End of file: 76 `}`, 77 `}`, 78 empty?? (see state), 79 `for (int i = 1; i < count_line; i++)`, 80 `free(lines[i]);`, 81 empty.
- [42:32] INSERT, 65,9, 93%. Blank line 64, new empty cursor line 65, `} else {` at 66. for-loop now at 80-81.
- [42:44] INSERT, 66,11, 91%. Line 65: `if (*path == '\0')`; line 66 empty indented cursor; `} else {` at 67.
- [43:05] Normal, 29,21, 57%. Scrolled up, lines 29-64.
- [43:06] Normal, 21,26, 40%. Lines 21-56.
- [43:07] Normal, 19,17, 36%. Lines 19-54; `if (argc > 1) {` line 19.
- [43:08] Normal, 18,21, 32%. NEW line 18: `char filename[255];` (above `if (argc > 1) {`); `int on_edit = 0;` now 35, `while (1) {` 36.
- [43:27] INSERT, 33,5, 32%. Blank line inserted after the else-block's closing `}` (line 32): line 33 empty cursor, line 34 `}`; `int on_edit = 0;` now 36.
- [43:28] Left window switched to the man-strncmp tab (STANDARDS section).
- [43:29] Tab title "~/lab — -zsh": shows `lab $ man 3 strncmp` / `lab $ ` (less quit).
- [43:33] Tab "~/lab — less ◂ man 3 strcpy": STRCPY(3). NAME: stpcpy, stpncpy, strcpy, strncpy — copy strings. SYNOPSIS: `char * stpcpy(char * dst, const char * src);`, `char * stpncpy(char * dst, const char * src, size_t len);`, `char * strcpy(char * dst, const char * src);`, `char * strncpy(char * dst, const char * src, size_t len);`. ADMONISHMENT: "YOU SHOULD ALMOST CERTAINLY USE strlcpy() INSTEAD. See "Examples" below."
- [43:54-43:57] Scrolling strcpy man: DESCRIPTION (strncpy copies at most len chars; remainder filled with '\0' if src shorter; otherwise dst is not terminated; strings should not overlap), RETURN VALUES, EXAMPLES (`char chararray[6];` `(void)strncpy(chararray, "abc", sizeof(chararray));`).
- [44:08] EXAMPLES second example: sets chararray to "abcdef": `(void)strncpy(chararray, "abcdefgh", sizeof(chararray));` "Note that it does not NUL terminate chararray because the".
- [44:35] vim main.c INSERT 33,5, 32% (lines 17-52). Line 33 still empty cursor. Chin-on-hand pose.
- [44:38] vim INSERT 33,5, 20%: scrolled up, lines 11-46 visible.
- [44:44] strcpy man page (same EXAMPLES view).
- [44:47] vim again (same as 44:38).
- [44:57] strcpy man page again.
- [45:02] man page scrolled up slightly, ending at heading `EXAMPLES`.
- [45:06] man page scrolled down (EXAMPLES, `char chararray[6];` of the second example at bottom).
- [45:11] vim INSERT 33,5, 20% (same as 44:38).
- [45:23] INSERT 33,36, 20%. Line 33 being typed: `    char *test = strncpy(filename, ` (cursor after space).
- [45:34] INSERT 34,5, 19%. Line 33: `    char *test = strncpy(filename, path, sizeof(path));` ; line 34 empty cursor; `}` now 35.
- [45:49] INSERT 35,5, 19%. Line 34: `    printf("file name is %s\n", filename);`; line 35 cursor.
- [45:50] INSERT 37,3, 18%. Line 35: `    printf("%d\n", *test);`, 36 `  }`, 37 cursor (2-space indent), 38 blank.
- [45:52] INSERT 38,10, 18%. Line 38: `  return ` (typing, incomplete). 37 blank, 39 blank, 40 `  int on_edit = 0;`.
- [45:52-45:53] Left window back to strcpy man tab (same view as 45:06 earlier, scrolled to EXAMPLES).
- [45:53] Left window tab "~/lab — -zsh": `lab $ man 3 strncmp` / `lab $ man 3 strcpy` / `lab $ cc ma` (typing, cursor after "ma"; presumably `cc main.c`).

No vim save message ("... written") seen in this segment. No compiler output seen.

## File states
### main.c, as of 45:52 (last vim frame). Lines 1-10 ?? (not visible this segment)
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
38|   return ⟨incomplete, being typed⟩
39| 
40|   int on_edit = 0;
41|   while (1) {
42|     if (!fgets(cmd, MAX_LINE_LEN, stdin)) break;
43|     cmd[strcspn(cmd, "\r\n")] = 0;
44| 
45|     if (on_edit) {
46|       if (mode(cmd, "."))
?? (lines 47+ not visible at 45:52; last seen at 42:44, below, numbering of that time = final numbering minus 4)
```
Lower part, last seen at 42:44 (numbering of that time; subtract nothing, but in final file add +4):
```
35|   int on_edit = 0;
36|   while (1) {
37|     if (!fgets(cmd, MAX_LINE_LEN, stdin)) break;
38|     cmd[strcspn(cmd, "\r\n")] = 0;
39| 
40|     if (on_edit) {
41|       if (mode(cmd, "."))
42|         on_edit = 0;
43|       else {
44|         current_line++;
45|         lines[current_line] = strdup(cmd);
46|         count_line++;
47|       }
48|     } else {
49|       if (mode(cmd, "q"))
50|         break;
51|       else if (mode(cmd, "a")) // append
52|         on_edit = 1;
53|       else if (mode(cmd, "p")) { // print
54|         if (count_line > 0 )
55|           puts(lines[current_line]);
56|         else
57|           puts("?");
58|       }
59|       else if (mode(cmd, "d")) { // delete
60|       }
61|       else if (strncmp(cmd, "w", 1)) { // save
62|         char *path = cmd + 1;
63|         while (*path == ' ') path++; // trim space
64| 
65|         if (*path == '\0')
66|           ⟨empty line, cursor⟩
67|       } else {
68|         char *endptr;
69|         int num = strtol(cmd, &endptr, 10);
70| 
71|         if (*endptr == '\0' && num > 0 && num <= count_line) {
72|           current_line = num;
73|           puts(lines[current_line]);
74|         } else {
75|           puts("?");
76|         }
77|       }
78|     }
79|   }
80| 
81|   for (int i = 1; i < count_line; i++)
```
(At 42:19: 80 `  for (int i = 1; i < count_line; i++)`... sorry, exact: 79 `for (...)` / 80 `free(lines[i]);` with main's `}` lines 75-77 above; i.e. numbering at 42:19 was one less than at 42:44 since the blank/cursor lines were added. Indent of the `for` loop shown at col 2 (same as `while (1)`). The exact closing-brace structure near end of file is ⟨?⟩; the `}` at 77/78 in the 42:44 view are at cols 4, 2, 0 roughly. Not verified.)

Intermediate versions of the `w` block (see timeline): 40:53 `else if (mode(cmd, "w")) { // save`; 41:19 `else if (strncmp(cmd, "w", 1)) { // save`; 41:37 `char *path = cmd + 1;`; 41:55 `while (*path == ' ') path++;`; 42:19 added `// trim space`; 42:44 `if (*path == '\0')`.

### Other files
Right Terminal static ed reference session (no other files seen). Notes file not visible in this segment.

## Shell & outputs
- [40:27] left window less: `man 3 strncmp` (STRCMP(3)).
- Right terminal (constant throughout, genuine `ed` reference run):
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
(The first `this is text` was typed before `a` hence `?`.)
- [43:29] left shell:
```
lab $ man 3 strncmp
lab $ 
```
- [43:33] `man 3 strcpy` (STRCPY(3) page in less).
- [45:53] left shell:
```
lab $ man 3 strncmp
lab $ man 3 strcpy
lab $ cc ma
```
(typing, no output yet)

## Observations
- Implementing the `w` (save) command: uses `strncmp(cmd, "w", 1)` rather than `mode()` so `w filename` matches (note: strncmp returns 0 on match, so as written the condition is inverted; not yet tested). Parses filename as `char *path = cmd + 1;`, skips spaces (`while (*path == ' ') path++; // trim space`), then branches on `if (*path == '\0')` (no filename; the real ed reference shows `w` with no default filename gives `?`, `w text.txt` prints byte count `13`).
- Reads man 3 strncmp, man 3 strcpy (strncpy semantics, NUL-termination warning, strlcpy admonition), and beej strtol docs.
- Added `char filename[255];` to remember the opened file; wrote scratch test `char *test = strncpy(filename, path, sizeof(path));` with debug printfs and a temporary `return` before the command loop, about to compile with `cc main.c`. Note `sizeof(path)` of a pointer is a pitfall (8 bytes).
- Style: 2-space indent, trailing `//` comments, snake_case, incremental experiments with printf, consults man pages before using functions. Typo `strncmd` corrected quickly.
