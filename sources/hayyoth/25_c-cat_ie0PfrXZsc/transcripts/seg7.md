# Segment 7 (16:02 - 19:04) - Hayyoth, hand-coding a cat clone in C (ID _ie0PfrXZsc)

## Timeline
- [16:02] Terminal window (vim, file main.c) shows lines 21-50. Status line: `"main.c" 50L, 825B written` (cursor 47,18). `walkcat(path);` at line 47, cursor on `;`. Right side: browser on youtube.com/watch?v=tiMHtc6l7x... (URL truncated) playing his previous video "Build your own X from scratch" (Hayyoth - 1/6); that video shows a terminal with a `walk(const char *path, int indent)` function (opendir/readdir recursion). Yellow Stickies note at bottom center (see Observations).
- [16:03] Mouse hovers the old video; player controls appear (time 18:52 in that video).
- [16:04-16:06] Old video zoomed in; shows his previous `walk` source with line numbers (quoted in File states, reference code).
- [16:10] Stickies note moved over the terminal (top, ~x390-730). vim status `47,18  Bot`. Terminal wider (window to x=785).
- [16:11-16:14] Mouse on `opendir(path)` of the old code (line 14 of reference).
- [16:21] Note line "if dir -> open dir" highlighted (gray).
- [16:24] Note line "check each file in dir" highlighted.
- [16:25] Note collapsed (window-shaded) to a title bar reading `Design:...`.
- [16:26] Collapsed bar dragged to bottom of terminal (~y785).
- [16:28] Bar dragged further down to ~(640-990, 869); stays there for the rest of the segment.
- [16:31] Terminal focused again. Normal mode, `"main.c" 50L, 825B written` still shown, cursor on `;` at 47,18 `Bot`.
- [16:33] INSERT mode. In walkcat after the `if (!S_ISDIR...)` block: blank line 37, line 38 `  /` being typed. Pos 38,4, 90%. walkcat now ends at line 39 `}`; main begins at 41.
- [16:40] Line 38 `  // is folder`, line 39 `  // ` being typed. Pos 39,6, 86%.
- [16:42] Line 39 replaced by `  DIR *dir = opendir(path);`; line 40 `  if ` typed. Pos 40,6, 83%.
- [16:46] Line 40 `  if (!dir) {`, line 41 blank cursor. Pos 41,5, 80%.
- [16:47] Line 41 `    perror(path);`, line 42 `    retur` typed. Pos 42,10, 76%.
- [16:55] Lines 40-43 complete: `if (!dir) {` / `perror(path);` / `return;` / `}`; matching-brace highlight (cyan). Pos 43,4, 74%.
- [16:56] New blank line 44 (indented), pos 44,3, 71%.
- [17:00] Line 45 `  stru` typed (45,7, 68%).
- [17:02] Line 45 `  struct dirent *dp;`, line 46 blank with cursor (46,3, 66%).
- [17:13] Normal mode (no mode text), view scrolled so top is line 26; pos 52,3, 83%. walkcat lines 45-47: `struct dirent *dp;` / blank / `}`.
- [17:21] INSERT. Line 46 `  while ((dp = readdir(dir)) != NULL) {`, line 48 cursor blank, line 49 `  }`. Pos 48,5, 78%.
- [17:22] Line 47 added: `    const char *name = dp->d_name;`. Pos 48,5, 75%.
- [17:23] Normal mode, scrolled (top line 30); cursor 59,5 (on `c` of `const char *path = argv[i];` in main), 87%.
- [17:24] Scrolled again (top line 31); cursor 51,0-1 (empty line 51), 90%.
- [17:25-17:26] Old video scrolled to its `walk` body lines 24-45: `// walk to sub path`, `size_t len = strlen(path) + strlen(name) + 2;`, `char sub_path[len];`, `snprintf(sub_path, len, "%s/%s", path, name);`, `walk(sub_path);`, `closedir(dir); return 0;`. He reads his old code.
- [17:28] INSERT: blank line 50 added after the while's `}` (line 49); closing `}` of walkcat now line 51. Pos 50,3, 88%.
- [17:29] Line 50 `  closed` being typed (50,9).
- [17:33] Line 50 `  closedir(dir);` complete. Normal mode, cursor 50,16 on `;`.
- [17:34] Same; mouse over old video.
- [17:37] Cursor 47,21 (on `name` in `const char *name = dp->d_name;`), normal mode. Old video fully visible.
- [17:51] INSERT: line 48 `    size_t len = strlen(path) + strlen(name) + 2;` and line 49 `    char` typed. Pos 49,9, 85%. closedir now line 51, walkcat `}` at 52, main at 54.
- [17:57] Line 49 `    char sub_path[len];`, line 50 blank cursor (50,5), 83%.
- [18:10] Line 50 `    snprintf(sub_path, len, "%s/%s", path, name);`, line 51 blank, line 52 cursor (52,5), 78%.
- [18:21] Line 52 `    if (dp->d_type == DT_DIR) {`, line 53 blank, line 54 `    } ` cursor after `}` (54,7), 75%. (Reference code had `DT_DIR )` with a space; he writes `DT_DIR)`.)
- [18:23] Line 54 `    } else {`, line 55 blank, line 56 `    }`; cyan highlight on `{` of else, gray on matching `}` -> normal mode. Pos 56,5, 71%.
- [18:26] Terminal window inactive (gray traffic lights); browser window brought forward and shifted down (y~95), old video showing lines 24-45 with line numbers.
- [18:27] Terminal focused again; mouse I-beam over terminal. Webcam: hand over mouth.
- [18:28] Terminal window dragged to center-left (x236-990, y8-750); visible lines 31-60. Browser mostly behind it.
- [18:29] Cursor 47,25 on `dp` in `dp->d_name`, normal mode, 71%.
- [18:45] INSERT: trailing comments added: line 46 `  while ((dp = readdir(dir)) != NULL) { // go into each file`, line 47 `    const char *name = dp->d_name; // get name`, lines 48 `//` and 49 `// ` (cursor 49,39, 68%), pushing len/sub_path lines down to 50-52.
- [18:47] Normal mode: cursor 48,0-1 (empty line 48); the stray `//` lines removed. Comments `// go into each file` and `// get name` still present. 69%.
- [18:48] INSERT: line 49 `    //` typed (49,7), 68%.
- [18:57] Normal mode, cursor 47,5 (on `c` of `const`), 69%. Comments `// go into each file` / `// get name` still shown; no `//` line 49.
- [19:04] View scrolled to lines 38-67: the two trailing comments are GONE (deleted/undone): line 46 `while ((dp = readdir(dir)) != NULL) {` and line 47 `const char *name = dp->d_name;` plain. Cursor 62,34 on `{` of `int main(int argc, char *argv[]) {`, 86%.

Save messages: none new in this segment; only the pre-existing `"main.c" 50L, 825B written` (visible 16:02-16:31). No `:w` seen afterwards, so edits from 16:33 on are unsaved as of 19:04.

## File states

### main.c at start of segment (16:02-16:31; last save `"main.c" 50L, 825B written`)
```
 1| ?? (lines 1-20 not visible)
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
37| }
38| 
39| int main(int argc, char *argv[]) {
40|   if (argc == 1) {
41|     printf("Usage: %s <file_path>...\n", argv[0]);
42|     return 0;
43|   }
44| 
45|   for (int i = 1; i < argc; i++) {
46|     const char *path = argv[i];
47|     walkcat(path);
48|   }
49|   return 0;
50| }
```

### main.c at END of segment (19:04, unsaved edits)
```
 1| ?? (lines 1-25 not visible this segment; cat() ends at line 24 `}` per start state)
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
68| ?? (lines 68+ not visible; earlier state: for loop over argv calling walkcat(path); return 0; })
```
Note: no `return;` after `cat(path);` at lines 35-36 (the `if (!S_ISDIR ...)` block). Intermediate at 18:45: line 46 had trailing `// go into each file` and line 47 `// get name`, later removed.

### Reference code on screen: his previous video's walk() (browser, 16:05-16:06 and 17:25)
```
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
25|     if (strcmp(name, ".") == 0 || strcmp(name, "..") == 0)   (right end cut off: `== 0)` presumably)
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
36|       walk(sub_path);
37|     } else {
38|       print_file(name, 0, indent);
39|     }
40|   }
41| 
42|   closedir(dir);
43|   return 0;
44| }
45| 
```
(Small view at 16:02 also shows its lines 6-10: `printf("%*c", indent * 2, ' ');` / `if (is_dir)` / `printf("\033[0;34m%s\033[0m\n", name);` / `else` / `printf("%s\n", name);`. Characters in the small view are low-res.)

## Shell & outputs
No shell commands executed in this segment. Only vim status line `"main.c" 50L, 825B written` (16:02-16:31). Browser: YouTube watch page for his earlier "Build your own X from scratch" video, at 18:52 of that video; its embedded terminal shows the walk() source above. No compiler output.

## Observations
- Stickies note text (16:02, yellow, bottom center):
  `- check path is file or dir` (bold)
  `if file -> cat` (bold)
  `if dir -> open dir`
  `check each file in dir`
  `    if file -> cat`
  `    if dir -> walk cat again`
  `check my previous video`
  Lines from "if dir -> open dir" onward appeared gray-selected at 16:02 (the not-yet-implemented part); at 16:21 and 16:24 he highlights "if dir -> open dir" then "check each file in dir" as the step he is about to implement. The note's title collapses to `Design:...` at 16:25 and is parked at the bottom of the screen.
- Workflow: design note (pseudo-code plan) first, then implement each line of the plan as code, using comments as plan markers (`// if file -> cat`, `// is folder`). He consults his own earlier video on recursive directory walking and retypes the same structure by hand (opendir -> readdir loop -> sub_path via snprintf "%s/%s" with len = strlen(path)+strlen(name)+2 -> recurse -> closedir) rather than pasting.
- Differences from the reference: `walkcat(path)` with `cat(path)` instead of `walk(path, indent)`/`print_file`; reference skips `.`/`..` via `strcmp` (`// ignore files`) - not yet written here; DT_DIR and else bodies are empty at 19:04; `DT_DIR)` without the space.
- Added trailing comments `// go into each file` and `// get name` at 18:45, removed again by 19:04: he keeps the code comment-light, using comments only as plan markers.
- Style: 2-space indentation, `{` on same line, blank line separating logic groups, `perror(path); return;` on error, VLA `char sub_path[len]`.
- UI: terminal moved to center-left at 18:28 (x236-990) while editing; webcam overlay ignored.
