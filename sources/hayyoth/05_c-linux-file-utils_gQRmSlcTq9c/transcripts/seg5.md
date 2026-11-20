# Segment 5 (16:30 - 21:13) - mv.c (edited in tab titled `~/lab — vim touch.c`)

NOTE: the vim tab is always titled `~/lab — vim touch.c` but the buffer is `mv.c` (save messages say "mv.c"; title is stale). Tabs: [1] `~/lab — vim touch.c`, [2] `~/lab — less ◂ man strchr` (shown as `...lab — less ◂ man strchr` when focused), [3] `~/lab — -zsh` (prompt `lab $ `). Tab 3 is `...— less ◂ man strchr` at 16:30 and 16:42, `-zsh` at 16:43+, `...ss ◂ man 3 snprintf` at 20:22-20:25 (then `...less ◂ man 3 snprintf` unfocused at 20:30, back to `-zsh` at 20:38).
notes.txt (right top) is constant for the whole segment: vim VISUAL mode, second `mv` row line highlighted: `if target is dir , move`; line number indicator `25`. No edits to notes.txt.

## Timeline
- [16:30] Tab 3: `man strchr` open (STRCHR(3) Library Functions Manual; NAME `strchr, strrchr, strchrnul — locate character in string`; SYNOPSIS `#include <string.h>`; `char *strchr(const char *s, int c);` etc.). less prompt `:`.
- [16:38] Tab 1 vim (NORMAL), lines 20-57 visible. Status `"mv.c" 57L, 1150B written`. Line 51 being edited: `const char *name = strchr(path,█"/");` (cursor block between `,` and space-before `"/"`; second arg is still the string `"/"`). Lines 50 `printf("path: %s\n", path);` and 52 `printf("name: %s\n", name);` are debug prints.
- [16:42] Tab 3 man strchr again, `int ` of `strchr(const char *s, int c)` highlighted: he points out 2nd param is `int` (char), not a string.
- [16:43] Tab 3 `-zsh`: `lab $ man strchr` typed (cursor at end, not yet executed).
- [16:51] Shell:
  ```
  lab $ cc mv.c -o mv
  lab $ ./mv ~/lab/my-xyz.txt abc
  path: /Users/ctsotc/lab/my-xyz.txt
  name: /Users/ctsotc/lab/my-xyz.txt
  lab $ █
  ```
  (name printed as the whole path - the result of the earlier `strchr(path, "/")`-style version; exact source at compile time not visible.)
- [16:55] vim: line 51 is `const char *name = strchr(path, '/█');` - char literal now (cursor inside quotes). Status still `"mv.c" 57L, 1150B written` (stale message).
- [17:01] Shell same as 16:51.
- [17:03]-[17:10] Tab 2 `man strchr` (less) scrolled down: DESCRIPTION, RETURN VALUES ("The functions strchr() and strrchr() return a pointer to the located character, or NULL if the character does not appear in the string."; "strchrnul() returns a pointer to the terminating '\0' if the character does not appear in the string."), SEE ALSO at bottom (17:10).
- [17:14] vim NORMAL: line 51 `const char *name = strchr(path, '/');`, cursor on final `}` (line 57); `"mv.c" 57L, 1150B written`.
- [17:19] Tab 2 man page scrolled back so SYNOPSIS visible; `strrchr` in `strrchr(const char *s, int c);` highlighted (decides to use strrchr = last occurrence).
- [17:20] Shell still shows only the first run.
- [17:24] Shell now has two runs:
  ```
  lab $ cc mv.c -o mv
  lab $ ./mv ~/lab/my-xyz.txt abc
  path: /Users/ctsotc/lab/my-xyz.txt
  name: /Users/ctsotc/lab/my-xyz.txt
  lab $ cc mv.c -o mv
  lab $ ./mv ~/lab/my-xyz.txt abc
  path: /Users/ctsotc/lab/my-xyz.txt
  name: /my-xyz.txt
  lab $ █
  ```
  (second run is the strchr(path,'/')... result `/my-xyz.txt` = last slash, i.e. it printed from last slash, so the strrchr version; compiled between 17:14 and 17:24; the 17:34 save message confirms the +1 byte `r`.)
- [17:34] vim: line 51 `const char *name = str█chr(path, '/');` cursor highlight on the `r` of `strrchr`; `"mv.c" 57L, 1151B written` (+1 byte vs 1150B = the added `r`).
- [17:42] Shell unchanged; `/my-xyz.txt` (after `name: `) highlighted by mouse: points at the leading slash that must be removed.
- [17:51] vim: line 51 `const char *name = strrchr(path, '/');`, cursor at `n` of `name` on line 52; `"mv.c" 57L, 1151B written`.
- [17:53] INSERT: new empty line 52 opened under the strrchr line; `printf("name: %s\n", name);` pushed to 53, `}` to 54, `return 0;` at 57.
- [17:59] INSERT: line 52 typed `if (!name) name = path;`; new empty line 53 with cursor; printf name at 54; `}` 55.
- [18:07] NORMAL: the `printf("path: %s\n", path);` line deleted; lines now: 47 `const char *path = argv[i];`, 48 `// move to dir , like rename but get new path`, 49 `// new path = dir + "/" + file name`, 50 `const char *name = strrchr(path, '/');`, 51 `if (!name) name = path;`, 52 `name++;`, 53 `printf("name: %s\n", name);`, 54 `}`, 55-56 blank, 57 `return 0;`. Cursor block on first `/` of line 48. (`name++;` typed in between, line 52, no comment yet.)
- [18:08] The two comment lines 48-49 also deleted: 47 path, 48 `const char *name = strrchr(path, '/');`, 49 `if (!name) name = path;`, 50 `name++;`, 51 `printf("name: %s\n", name);`, 52 `}`, 55 `return 0;`, 56 `}`, then `~`.
- [18:15] Shell (full history shown):
  ```
  lab $ cc mv.c -o mv
  lab $ ./mv ~/lab/my-xyz.txt abc
  path: /Users/ctsotc/lab/my-xyz.txt
  name: /Users/ctsotc/lab/my-xyz.txt
  lab $ cc mv.c -o mv
  lab $ ./mv ~/lab/my-xyz.txt abc
  path: /Users/ctsotc/lab/my-xyz.txt
  name: /my-xyz.txt
  lab $ ./mv my-xyz.txt abc
  path: my-xyz.txt
  name: (null)
  lab $ █
  ```
  (`name: (null)` highlighted = strrchr returned NULL when no `/`; motivates `if (!name) name = path;`.)
- [18:26] Shell additionally:
  ```
  lab $ cc mv.c -o mv
  lab $ ./mv my-xyz.txt abc
  name: y-xyz.txt
  lab $ ./mv ~/lab/my-xyz.txt abc█        (typed, not run)
  ```
  Bug: with no slash, `name++` skips first char -> `y-xyz.txt`.
- [18:36] vim: line 50 `name++; // remove slash` (cursor on the `h`); status `"mv.c" 56L, 1085B written`. Line 49 `if (!name) name = path;` unchanged (bug stays).
- [18:40] INSERT: lines after printf (52, 53 blank, cursor on 54) opened inside the for loop; `}` 55, blank 56-57, `return 0;` 58, `}` 59. View scrolled, `~` filler.
- [18:41] Same, scrolled to top line 38. Still INSERT, cursor line 54.
- [19:14] INSERT: `// make new path` (line 54); line 55 `size_t path_len = strlen(dir) + strlen(name` (incomplete); `}` 56, `return 0;` 59, `}` 60.
- [19:37] INSERT: 55 `size_t path_len = strlen(dir) + strlen(name) + 2;`, 56 `char new_path[path_len];`, 57 `snprintf(new_path, "%s/%s` (incomplete); `}` 58, `return 0;` 61, `}` 62.
- [20:06] Command line `:w` typed (NORMAL). Lines: 57 `snprintf(new_path, "%s/%s", dir, name);`, 58 blank, 59 `printf("new_path %s\n", new_path);`, 60 `}`, 63 `return 0;`, 64 `}`.
- [20:11] Shell (the earlier history plus):
  ```
  lab $ ./mv ~/lab/my-xyz.txt abc
  name: my-xyz.txt
  lab $ █
  ```
  (from the 18:26 typed command; name now correct for absolute path.)
- [20:13] Shell:
  ```
  lab $ cc mv.c -o mv
  mv.c:57:24: error: incompatible pointer to
        integer conversion passing 'char[6]' to parameter of type
        'unsigned long' [-Wint-conversion]
     57 |     snprintf(new_path, "%s/%s", dir, name);
        |                        ^~~~~~~
  1 error generated.
  lab $ █
  ```
- [20:22] Tab 3 now `man 3 snprintf` (PRINTF(3)): NAME `printf, fprintf, sprintf, snprintf, asprintf, dprintf, vprintf, vfprintf, vsprintf, vsnprintf, vasprintf, vdprintf — formatted output conversion`; SYNOPSIS `#include <stdio.h>`, `int snprintf(char * restrict str, size_t size, const char * restrict format, ...);`.
- [20:24],[20:25] Page scrolled (SYNOPSIS reading; mouse I-beam near `sprintf`).
- [20:30] Back to vim tab: `"mv.c" 64L, 1275B written` (the `:w` from 20:06); line 59 `printf("new_path %s\n", new_path)` with `;` highlighted; line 57 still lacks size arg.
- [20:38] Shell: history ends with the compile error then `lab $ man 3 snprintf` then new prompt.
- [20:40] Shell cleared: `lab $ █`.
- [20:48] Shell:
  ```
  lab $ ./mv ~/lab/my-xyz.txt abc
  name: my-xyz.txt
  new_path abc/my-xyz.txt
  lab $ █
  ```
  (a `new_path abc/my-xyz.txt` line highlighted by mouse; compile of the fix not on screen.)
- [20:58] Shell:
  ```
  lab $ ./mv ~/lab/my-xyz.txt abc
  name: my-xyz.txt
  new_path abc/my-xyz.txt
  lab $ ./mv my-xyz.txt abc
  name: y-xyz.txt
  new_path abc/y-xyz.txt
  lab $ █
  ```
  `new_path abc/y-xyz.txt` highlighted: bug still visible.
- [21:07] vim: line 57 now `snprintf(new_path, path_len, "%s/%s", dir, name);` (cursor in `path_len`); `"mv.c" 64L, 1285B written` (+10 bytes = `path_len, `). View top line 38.
- [21:13] vim NORMAL; view scrolled to top line 37 (cursor block on empty line 37); file now 65 lines (extra blank line after printf new_path: 59 printf, 60 blank, 61 `}`, 62-63 blank, 64 `return 0;`, 65 `}`). No save message shown (unsaved).

## File states

### notes.txt (right top; unchanged during segment; visible columns only, right edge cut off by window)
```
  | Build linux files utils in C
  |
  +-------+---------------------+------------------------------------------
  |Command|  Function use       |           Algorithm
  +-------+---------------------+------------------------------------------
  |touch  | fopen               |
  +-------+---------------------+
  |cat    | fopen, fgetc, putchar|          get path and do
  +-------+---------------------+
  |mkdir  | mkdir               |
  +-------+---------------------+------------------------------------------
  |mv     | stat, rename        | if target is file or not exit, rename
  |       |                     | if target is dir , move      <- VISUAL highlighted (line 25)
  +-------+---------------------+------------------------------------------
  |rm     | lstat, unlink, rmdir| walk path, if file remove
  |       | opendir, readdir    | if dir, open child, if dir walk path
  +-------+---------------------+------------------------------------------
  |find   | lstat, strstr       | walk path, if match print
  |       | opendir, readdir    | if dir, open child, if dir walk path
  +-------+---------------------+------------------------------------------
  -- VISUAL --                                                      25
```
(tree row not visible in this window; column borders are approximate.)

### mv.c - version at 17:14-17:24 (57L, 1150B -> 1151B at 17:34); lines 20-57 visible; lines 1-19 ?? (not visible)
```
20|     //
21|     // if error is file not exit
22|     if (errno == ENOENT && argc == 2) {
23|       // do rename
24|       const char *path = argv[1];
25|       if (rename(path, target)) {
26|         perror(path);
27|         return 1;
28|       }
29| 
30|       return 0;
31|     }
32| 
33|     perror(target);
34|     return 1;
35|   }
36| 
37| 
38|   // if dir not exit, return error
39|   // usecase 2
40|   const char *dir = target;
41|   if (!S_ISDIR(st.st_mode)) {
42|     fprintf(stderr, "%s is not dir or not exit\n", dir);
43|     return 1;
44|   }
45| 
46|   for (int i = 1; i < argc; i++) {
47|     const char *path = argv[i];
48|     // move to dir , like rename but get new path
49|     // new path = dir + "/" + file name
50|     printf("path: %s\n", path);
51|     const char *name = strchr(path, '/');   // at 17:51: strrchr(path, '/')
52|     printf("name: %s\n", name);
53|   }
54| 
55| 
56|   return 0;
57| }
```
Indentation approximate: pixel offsets show lines 20-35 at 4/6/8/10 cols (22 `if` at 4, 23-25 at 6, 26-27 at 8, 28 `}` at 6, 31 `}` at 4, 35 `}` at 2) and lines 38-57 start at col 2 with 2-space nesting; there is a `stat`/`if` block opened before line 20 (?? lines 1-19 not visible).

### mv.c - FINAL as of 21:13 (65L; last saved 64L, 1285B at 21:07), lines 37-65 visible
```
37| 
38|   // if dir not exit, return error
39|   // usecase 2
40|   const char *dir = target;
41|   if (!S_ISDIR(st.st_mode)) {
42|     fprintf(stderr, "%s is not dir or not exit\n", dir);
43|     return 1;
44|   }
45| 
46|   for (int i = 1; i < argc; i++) {
47|     const char *path = argv[i];
48|     const char *name = strrchr(path, '/');
49|     if (!name) name = path;
50|     name++; // remove slash
51|     printf("name: %s\n", name);
52| 
53| 
54|     // make new path
55|     size_t path_len = strlen(dir) + strlen(name) + 2;
56|     char new_path[path_len];
57|     snprintf(new_path, path_len, "%s/%s", dir, name);
58| 
59|     printf("new_path %s\n", new_path);
60| 
61|   }
62| 
63| 
64|   return 0;
65| }
```
Lines 1-36: lines 20-36 as in the version above; ?? lines 1-19 never visible.

Checksums / intermediate saves:
- 16:38-17:20: `"mv.c" 57L, 1150B written` (strchr(path,"/") -> strchr(path,'/'))
- 17:34-17:51: `57L, 1151B written` (strrchr)
- 18:36: `56L, 1085B written` (path printf removed, 2 comments removed, `if (!name) name = path;`, `name++; // remove slash`)
- 20:30: `64L, 1275B written` (new_path block, snprintf missing size)
- 21:07: `64L, 1285B written` (snprintf(new_path, path_len, ...))

## Shell & outputs
Prompt `lab $ ` in ~/lab; user dir in output `/Users/ctsotc/lab/`.
- 16:43 `man strchr` typed.
- 16:51/17:24: `cc mv.c -o mv`; `./mv ~/lab/my-xyz.txt abc` -> `path: /Users/ctsotc/lab/my-xyz.txt` / `name: /Users/ctsotc/lab/my-xyz.txt`; again `cc mv.c -o mv`; `./mv ~/lab/my-xyz.txt abc` -> `path: /Users/ctsotc/lab/my-xyz.txt` / `name: /my-xyz.txt`.
- 18:15: `./mv my-xyz.txt abc` -> `path: my-xyz.txt` / `name: (null)`.
- 18:26: `cc mv.c -o mv`; `./mv my-xyz.txt abc` -> `name: y-xyz.txt`; then `./mv ~/lab/my-xyz.txt abc` typed.
- 20:11: `./mv ~/lab/my-xyz.txt abc` -> `name: my-xyz.txt`.
- 20:13: `cc mv.c -o mv` -> `mv.c:57:24: error: incompatible pointer to integer conversion passing 'char[6]' to parameter of type 'unsigned long' [-Wint-conversion]` / `   57 |     snprintf(new_path, "%s/%s", dir, name);` / `      |                        ^~~~~~~` / `1 error generated.` (message wraps in the terminal after "pointer to" and "'unsigned long'").
- 20:38: `man 3 snprintf` (from history); 20:40 screen cleared.
- 20:48: `./mv ~/lab/my-xyz.txt abc` -> `name: my-xyz.txt` / `new_path abc/my-xyz.txt`.
- 20:58: `./mv my-xyz.txt abc` -> `name: y-xyz.txt` / `new_path abc/y-xyz.txt`.
Man pages: `man strchr` (STRCHR(3), whole page to SEE ALSO), `man 3 snprintf` (PRINTF(3), SYNOPSIS only).

## Observations
- Still on mv.c, usecase 2 (move into directory = notes row "if target is dir , move"). rename-only branch (ENOENT && argc == 2) already done before 16:30. This segment builds basename extraction and `new_path = dir + "/" + name`; the actual rename() of new_path not yet written at 21:13.
- Print-driven debugging: `printf("path: ...")`, `printf("name: ...")`, `printf("new_path %s\n", ...)`; tests with absolute (`~/lab/my-xyz.txt`) and relative (`my-xyz.txt`) paths with target `abc`. Edge case bug found (relative path without slash -> `name++` drops first char, `y-xyz.txt`), seen at 18:26 and again 20:58, NOT fixed as of 21:13 (line 49-50 unchanged).
- Man pages used for exact signatures: strchr (int c, char not string; NULL return; strrchr for last occurrence) and snprintf (size argument) after a clang `-Wint-conversion` error. He reads SYNOPSIS/RETURN VALUES only; highlights with mouse/search.
- Error handling style: `perror(path)`/`perror(target)` + `return 1;`; `fprintf(stderr, "%s is not dir or not exit\n", dir)` (consistent "exit" for "exist" typo in comments and messages).
- Style: 2-space indent, C99 `for (int i = 1; ...)`, VLA `char new_path[path_len]`, `size_t path_len = strlen(dir) + strlen(name) + 2;` (slash + NUL), one-line `if (!name) name = path;`, informal lowercase comments (`// make new path`, `name++; // remove slash`, `// usecase 2`). Compile cmd: `cc mv.c -o mv`.
- Tab title `vim touch.c` is stale: one long-lived vim session reused for every utility file.
- Gap: mv.c lines 1-19 never visible in this segment.
