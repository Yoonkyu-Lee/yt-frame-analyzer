# Segment 8 (19:04 - 21:05) - C `cat` clone, Hayyoth

## Timeline
Layout at start: a macOS Terminal window titled "Terminal" (vim, line numbers on) at upper left; a browser window behind it on the right showing a YouTube page (URL bar `www.youtube.com/watch?v=tiMHtc6l7x...`) with the author's earlier-video code (a `walk(...)`/`print_file(...)` tree-printer) as reference; a yellow Sticky titled "Design:" at bottom; webcam window "Human" bottom right (ignored except where it covers the screen).

- [19:04] vim, normal mode, cursor 62,34 (`{` of main highlighted), 86%. Visible lines 38-67 (see File states). Sticky "Design:..." collapsed to a thin bar at bottom.
- [19:08] vim now `-- INSERT --`, cursor 54,7 (inside empty `if (dp->d_type == DT_DIR) {` body, line 54 empty). Sticky expanded at bottom of screen; "check each file in dir" highlighted.
- [19:10] Sticky dragged up over the editor/browser. Text: "Design:" / "- get path" / "- check path is file or dir" / "if file -> cat" / "if dir -> open dir" / "check each file in dir" (highlighted) / "    if file -> cat" / "    if dir -> walk cat again".
- [19:11-19:12] Sticky scrolled down: it ends with a last line "check my previous video". Cursor in sticky text.
- [19:14] Nested "if file -> cat" highlighted in Sticky; vim still INSERT 54,7; line 54 still empty.
- [19:25] vim out of INSERT (no mode shown), cursor 54,6. Line 56 now reads `cat(sub_path);` in the else branch (line 54 still empty; 55 `} else {`, 56 `cat(sub_path);`, 57 `}`).
- [19:29] Sticky highlight moves to "if dir -> walk cat again". Cursor block at 54,6 (line 54 empty).
- [19:42] Line 54 now: `walkcat(sub_path);` cursor 54,24 (on the `;`). The dir branch calls a new function `walkcat` recursively; else branch calls `cat(sub_path)`.
- [19:43-20:04] Webcam window enlarged to near-full-screen over the editor, author moves around and shows the balcony (rain/wind outside), then returns to the desk. No code change visible. Underlying vim state unchanged (cursor 54,24, 86%) as seen at 19:43 and 20:04.
- [20:06] Webcam resized to a window at right-middle; editor unchanged. Mouse over Sticky line "check my previous video".
- [20:12] Webcam smaller bottom right; browser right side shows `... walk to sub path / size_t len = strlen(path) + strlen(name) + 2; / char sub_path[len]; / snprintf(sub_path, len, "%s/%s", path, name);`. Sticky hidden. Editor unchanged (54,24).
- [20:13] Terminal window moved left a bit. Same content, cursor 54,24.
- [20:14] Terminal window moved further left/down. Cursor 55,12 on `{` of `} else {` (matching `}` at line 57 highlighted cyan). Browser shows earlier code: `// ignore files / if (strcmp(name, ".") == 0 || strcmp(name, "..") == ... / continue; / if (dp->d_type == DT_DIR ) { / print_file(name, 1, indent); / // walk to sub path / size_t len = strlen(path) + strlen(name) + 2; / char sub_path[len]; / snprintf(sub_path, len, "%s/%s", path, name); / walk(sub_path); / } else { / print_file(nam... / } } / closedir(dir); / return 0; / }`.
- [20:24] vim scrolled up: top line 27 `struct stat st;`, cursor 27,17, 60%.
- [20:25] Scrolled: top line 26 `void walkcat(const char *path) {`, cursor 26,32, 58%. **Save message: `"main.c" 73L, 1260B written`**.
- [20:26] Scrolled up more: top line 10, cursor 10,12, 20%.
- [20:27] Top of file visible: lines 1-30, cursor 1,18 ("Top"). Author reviews the `cat` function.
- [20:28] Browser (right) scrolled: reference shows `printf("\033[0;34m%s\033[0m\n", name); else printf("%s\n", name); } int walk(const char *path, int indent) { DIR *dir = opendir(path); if (!dir) { perror("opendir"); return 1; } struct dirent *dp; while ((dp = readdir(dir)) != NULL) { const char *name ...`.
- [20:29] Browser scrolled up showing YouTube header "YouTube VN", a nested Terminal screenshot titled `~/lab -- vim README.md`, then code `printf("%*c", indent * 2, ' '); if (is_dir) printf("\033[0;34m%s\033[0m\n", name); else printf("%s\n", name); } int walk(const char ...`.
- [20:30] Browser at `else printf("%s\n", name); } int walk(const char *path, int indent) {...`, mouse I-beam near end of line 7 of main.c. Editor unchanged (1,18).
- [20:31] vim `-- INSERT --`, cursor 3,1. Blank line inserted at line 3 (now: 1 `#include <stdio.h>`, 2 `#include <sys/stat.h>`, 3 empty cursor, 4 empty, 5 `#define BUFFER_SIZE 1024`; everything below +1).
- [20:35] INSERT, cursor 2,1. Now: 1 `#include <stdio.h>`, 2 (empty, cursor), 3 `#include <sys/stat.h>`, 4 `#include <dirent.h>`, 5 empty, 6 `#define BUFFER_SIZE 1024`. cat now at line 8 (+2 vs 20:27). Author added `#include <dirent.h>`.
- [20:42] Normal mode, scrolled: top line 13, cursor 41,19, 26%. walkcat at line 28.
- [20:44] INSERT, cursor 28,1, 25%: lines 27, 28 (cursor), 29 empty; `void walkcat(const char *path) {` now at line 30. Author opened blank lines between cat and walkcat to add a helper.
- [20:52] INSERT, cursor 30,2: helper stub typed: `int is_ignore(const char *path) {` (line 28), empty line 29, `}` (line 30; matching braces highlighted cyan); walkcat now at line 32.
- [21:01] A second Terminal window (shell) appears on top: prompt `lab $ ` (empty). In vim behind (INSERT, cursor 26,2, 24%): line 29 now `  // ignore '.' file`; line 26 `}` has a cursor box after it.
- [21:05] In the shell: `cd test-data`, then `ls -a` (output in Shell section). Editor unchanged (INSERT, 26,2).

## File states
### main.c at ~19:04 (lines 38-67 visible)
```
38|   // is folder
39|   DIR *dir = opendir(path);
40|   if (!dir) {
41|     perror(path);
42|     return;
43|   }
44| 
45|   struct dirent *dp;
46|   while ((dp = readdir(dir)) != NULL) {
47|     const char *name = dp->d_name;
48| 
49|     size_t len = strlen(path) + strlen(name) + 2;
50|     char sub_path[len];
51|     snprintf(sub_path, len, "%s/%s", path, name);
52| 
53|     if (dp->d_type == DT_DIR) {
54| 
55|     } else {
56| 
57|     }
58|   }
59|   closedir(dir);
60| }
61| 
62| int main(int argc, char *argv[]) {
63|   if (argc == 1) {
64|     printf("Usage: %s <file_path>...\n", argv[0]);
65|     return 0;
66|   }
67| 
```
(2-space indent. Line 67 last visible; file is 73 lines at the 20:25 save.)

### main.c as of 20:25 save (`"main.c" 73L, 1260B written`), assembled from frames 20:14-20:27
```
 1| #include <stdio.h>
 2| #include <sys/stat.h>
 3| 
 4| #define BUFFER_SIZE 1024
 5| 
 6| void cat(const char *path) {
 7|   printf("\n=========== %s ===========\n", path);
 8| 
 9|   FILE *fp = fopen(path, "r");
10|   if (!fp) {
11|     perror(path);
12|     return;
13|   }
14| 
15|   char buffers[BUFFER_SIZE];
16|   size_t nitems;
17| 
18|   while ((nitems = fread(buffers, 1, BUFFER_SIZE, fp)))
19|     fwrite(buffers, 1, nitems, stdout);
20| 
21|   printf("\n");
22| 
23|   fclose(fp);
24| }
25| 
26| void walkcat(const char *path) {
27|   struct stat st;
28|   if (lstat(path, &st)) {
29|     perror(path);
30|     return;
31|   }
32| 
33|   if (!S_ISDIR(st.st_mode)) {
34|     // if file -> cat
35|     cat(path);
36|   }
37| 
38|   // is folder
39|   DIR *dir = opendir(path);
40|   if (!dir) {
41|     perror(path);
42|     return;
43|   }
44| 
45|   struct dirent *dp;
46|   while ((dp = readdir(dir)) != NULL) {
47|     const char *name = dp->d_name;
48| 
49|     size_t len = strlen(path) + strlen(name) + 2;
50|     char sub_path[len];
51|     snprintf(sub_path, len, "%s/%s", path, name);
52| 
53|     if (dp->d_type == DT_DIR) {
54|       walkcat(sub_path);
55|     } else {
56|       cat(sub_path);
57|     }
58|   }
59|   closedir(dir);
60| }
61| 
62| int main(int argc, char *argv[]) {
63|   if (argc == 1) {
64|     printf("Usage: %s <file_path>...\n", argv[0]);
65|     return 0;
66|   }
67| 
68| ?? (lines 68-73 not visible in this segment)
```
Notes: no `<dirent.h>` or `<string.h>` include at the save; line 36 `}` follows `cat(path);` with no `return;` visible.

### main.c at END of segment (21:05), partial; edits after the save are unsaved (no write message seen)
```
 1| #include <stdio.h>
 2| ?? (blank line, empty at 20:35)
 3| #include <sys/stat.h>
 4| #include <dirent.h>
 5| 
 6| #define BUFFER_SIZE 1024
 7| 
 8| void cat(const char *path) {
 9|   printf("\n=========== %s ===========\n", path);
10| 
11|   FILE *fp = fopen(path, "r");
12|   if (!fp) {
13|     perror(path);
14|     return;
15|   }
16| 
17|   char buffers[BUFFER_SIZE];
18|   size_t nitems;
19| 
20|   while ((nitems = fread(buffers, 1, BUFFER_SIZE, fp)))
21|     fwrite(buffers, 1, nitems, stdout);
22| 
23|   printf("\n");
24| 
25|   fclose(fp);
26| }
27| 
28| int is_ignore(const char *path) {
29|   // ignore '.' file
30| }
31| 
32| void walkcat(const char *path) {
33|   struct stat st;
34|   if (lstat(path, &st)) {
35|     perror(path);
36|     return;
37|   }
38| 
39|   if (!S_ISDIR(st.st_mode)) {
40|     // if file -> cat
41|     cat(path);
42|   }
43| ?? (lines 43+ not visible; same as the 20:25 version shifted by +6)
```
(The 20:31 frame had line 2 `#include <sys/stat.h>` and line 3 blank; by 20:35 sys/stat.h is at line 3 and dirent.h at line 4 with an empty line 2 - so a line was inserted above sys/stat.h. At 21:01 lines 1-12 are hidden behind the shell window.)

## Shell & outputs
- [21:01] New Terminal window (separate from vim), prompt: `lab $ `
- [21:05] Commands:
```
lab $ cd test-data
test-data $ ls -a
```
Output (mouse cursor covers the first column's first entries, which are presumably `.` and `..`):
```
             .vscode      index.js   run.html   style.css
.            CNAME        LICENSE    run.js
.secret      index.html   README.md  src
test-data $
```
(`.vscode` and `src` shown in cyan, i.e. directories.) No compile/run happened in this segment.

## Observations
- Build order: design notes first (Sticky "Design:"), then fill the recursion step so main.c matches the Sticky. The dir-walk code is copied from the author's previous video (browser reference: `walk(path, indent)` with opendir/readdir, `strcmp(name, ".")` filter, `snprintf(sub_path, len, "%s/%s", path, name)`), replacing `print_file`/`walk` with `cat`/`walkcat`.
- Sticky text (exact): "Design: / - get path / - check path is file or dir / if file -> cat / if dir -> open dir / check each file in dir / (indented) if file -> cat / (indented) if dir -> walk cat again / check my previous video". The last line is a reminder to reuse the previous video's code.
- After looking at the reference's "ignore files" block (`strcmp(name, ".") == 0 || strcmp(name, "..") == 0 -> continue`), he realises the loop will recurse into `.` and `..`; he starts an `int is_ignore(const char *path)` helper (comment `// ignore '.' file`) and uses `ls -a` in a `test-data` directory (dotfiles `.vscode`, `.secret`) to show `.`/`..` entries.
- `<dirent.h>` added after the save; `<string.h>` (strlen) not seen being added.
- Style: 2-space indent, vim, small helper functions, short `//` comments.
- Roughly 20 s (19:43-20:04) of the segment is webcam-only (author away from desk showing the balcony); no screen activity.
