# Segment 1 (00:00 - 03:09)

## Timeline
Layout: LEFT = macOS Terminal (2 tabs, both `~/lab`), RIGHT TOP = second Terminal window with notes.txt in vim, RIGHT BOTTOM = webcam "Human" (ignored).

- [00:00-00:01] Start. Left terminal: `lab $ ls` -> `notes.txt touch.c`. A notes.txt window (vim, normal mode, cursor on `B` of line 1) floats over it, status line `"notes.txt" 43L, 1893B written`. Title line `Build linux files utils in C` plus design table (see File states). Both tabs titled `~/lab — zsh`.
- [00:02-00:04] notes.txt window dragged to the top right (right edge of table cut off by screen). Webcam overlaps its lower-right.
- [00:07] Left terminal: typing `lab $ e tou` (`e` is evidently an alias/function for vim, since next frame tab is `vim touch.c`). notes.txt: `-- VISUAL --`, `touch` (command cell, row 1) highlighted, ruler `5`.
- [00:09] Tab 1 title `~/lab — vim touch.c`. touch.c opened: `"touch.c" 6L, 70B` (pre-existing template: include stdio, empty main, `return 0;`). notes.txt still VISUAL on `touch`.
- [00:23] INSERT mode, touch.c: line 4 `if (argc == 1) {`, line 5 `printf("Useage: touch [path]...\n")` (no `;` yet; auto-paired parens highlighted).
- [00:29] touch.c 12 lines: usage block complete (`;`, `return 0;`, `}`), blank line 8, cursor on empty indented line 9 (INSERT). notes.txt VISUAL now highlights `fopen` in the touch row (mouse I-beam there).
- [00:38] Line 9: `for (int i = 1; i < argc; i++) {` typed, cursor line 10 in INSERT. 13 lines.
- [00:50] Switched to tab 2 (`~/lab — zsh`), empty prompt `lab $ `.
- [00:53] Tab 2 title `~/lab — less ◂ man fopen`: `FOPEN(3) Library Functions Manual`; NAME `fopen, fdopen, freopen, fmemopen — stream open functions`; LIBRARY `Standard C Library (libc, -lc)`; SYNOPSIS `#include <stdio.h>`, `FILE * fopen(const char * restrict path, const char * restrict mode);` and fdopen/freopen/fmemopen; DESCRIPTION with modes "r" (Open for reading ... Fail if the file does not exist) and "w" (Open for writing ... Truncate the file to zero). Section (3). Pager prompt `:`.
- [01:01] Back in vim: line 10 `const char *path = argv[i];` typed, cursor line 11, INSERT. 14 lines.
- [01:20] Lines 11-15 typed: `FILE *fp = fopen(path, "w");`, `if (!fp) {`, `perror(path);`, `return 1;`, `}`. 18 lines. INSERT.
- [01:31] Tab 2 (man fopen) again: the `fopen` synopsis block (FILE * / fopen(...) / mode) highlighted by mouse selection (grey box).
- [01:32] Tab 2 at shell: `lab $ man fopen` then new prompt (man quit).
- [01:41] Shell: `cc touch.c -o touch`, `ls`, `./touch abc.txt`, `ls` (see Shell). No compiler output.
- [01:52] `./touch def.txt xyz.txt`, `ls` shows both created.
- [01:56] Vim: touch.c with line 16 `fclose(fp);` added; saved `"touch.c" 20L, 326B written`. `w` in `"w"` shows a highlighted cursor cell.
- [01:58] VISUAL LINE on line 1 (ruler `1`).
- [01:59] Back to NORMAL, cursor line 1 col 0.
- [02:00] VISUAL LINE over lines 1-20 (ruler `20`) (`ggVG`).
- [02:01] Status `20 lines yanked`.
- [02:03] New file: `"cat.c" [New]` (empty buffer, line 1). Tab title still `vim touch.c` (title not refreshed; likely `:e cat.c`).
- [02:04] Pasted: 21 lines, cursor on empty line 1, content on lines 2-21; status `20 more lines`.
- [02:05] Leading blank line removed: 20 lines, cursor on line 3 col 0. Content still the touch code.
- [02:12] Edit: line 5 `Useage: cat [path]...\n` (cursor on the `t` of `cat`); `"w"` still on line 11. notes.txt window active (title bar dark): cat row `fopen` highlighted, mouse on `fgetc`.
- [02:15] Line 11 now `FILE *fp = fopen(path, "r");`. notes.txt VISUAL highlights `putchar` (ruler `7`). Vim status line: `Type  :qa!  and press <Ent...n all changes and exit Vim` (Ctrl-C pressed in normal mode).
- [02:21] INSERT: new line 16 containing `o` (being typed), 21 lines, `fclose(fp);` on 17.
- [02:32] INSERT: line 17 `char c;`, line 18 `while ((c = fgetc(fp))` (parens highlighted), lines 19-20 blank, line 21 `fclose(fp);`. 25 lines.
- [02:44] Tab 2 (shell): old history visible (`man fopen`, `cc touch.c -o touch`, ...). Typing `lab $ c`. The cat compile is not visible between 02:32 and 02:44/02:55.
- [02:54] Shell shows lower part of notes.txt (rm / find / tree rows, Helper section) - output of presumably `cat notes.txt` (his own tool or system cat; not visible). Prompt `lab $ `.
- [02:55] Typing `lab $ ./`.
- [02:58] `lab $ ./cat cat.c` prints cat.c (see Shell). Custom `cat` works.
- [03:00] Terminal scrolled up with grey mouse selection over the output; shows tail of Helper notes, `lab $`, `lab $ ./cat cat.c`, output.
- [03:03] Shell cleared: `lab $ `.
- [03:04] Vim tab: cat.c, `"cat.c" [New] 25L, 395B written`. Cursor blank line 20. `!= EOF` now visible, EOF in red.
- [03:06] notes.txt window active; VISUAL on mkdir row (`|mkdir  | mkdir  ` highlighted; ruler `22`).
- [03:08] Left vim: VISUAL LINE all 25 lines (`ggVG`), ruler `25`.
- [03:09] Left vim NORMAL then `:` prompt (empty) - presumably `:e mkdir.c` next (same yank/new-file flow as touch -> cat).

## File states

### notes.txt (43L, 1893B at 00:00) - top-right vim window
Line numbers not shown (no `set number`). Column positions approximate.
```
Build linux files utils in C

+---------+------------------------+--------------------------------------+
|Command  |  Function use          |            Algorithm                 |
+---------+------------------------+--------------------------------------+
|touch    | fopen                  |                                      |
+---------+------------------------+                                      +
|cat      | fopen, fgetc, putchar  |         get path and do              |
+---------+------------------------+                                      +
|mkdir    | mkdir                  |                                      |
+---------+------------------------+--------------------------------------+
|mv       | stat, rename           | if target is file or not exit, rename|
|         |                        | if target is dir , move              |
+---------+------------------------+--------------------------------------+
|rm       | lstat, unlink, rmdir   | walk path, if file remove            |
|         | opendir, readdir       | if dir, open child, if dir walk path |
+---------+------------------------+--------------------------------------+
|find     | lstat, strstr          | walk path, if match print            |
|         | opendir, readdir       | if dir, open child, if dir walk path |
+---------+------------------------+--------------------------------------+
```
Typos verbatim: `not exit` (for "not exist"). Dash counts approximate.
Lower part (seen at 02:54 in narrow terminal, wrapped):
```
|tree    |                        | watch my previous video              |
+---------+------------------------+--------------------------------------+

Helper
- stat : check path, if file is link, follow link
- lstat: check path, if file is link, return link

struct stat st;
stat(path, &st) == -1   // have error
- errno == ENOENT       // not have file

S_ISDIR(st.st_mode)     // is dir
```
VISUAL highlight progression: 00:07 `touch` (cmd cell, 5 chars) -> 00:29 `fopen` (touch row) -> 02:12 `fopen` (cat row) -> 02:15 `putchar` (cat row, 7 chars) -> 03:06 mkdir row (`|mkdir  | mkdir  `, line 22).

### touch.c - final state at 01:56 (`"touch.c" 20L, 326B written`)
```
 1| #include <stdio.h>
 2|
 3| int main(int argc, char *argv[]) {
 4|   if (argc == 1) {
 5|     printf("Useage: touch [path]...\n");
 6|     return 0;
 7|   }
 8|
 9|   for (int i = 1; i < argc; i++) {
10|     const char *path = argv[i];
11|     FILE *fp = fopen(path, "w");
12|     if (!fp) {
13|       perror(path);
14|       return 1;
15|     }
16|     fclose(fp);
17|   }
18|
19|   return 0;
20| }
```
2-space indent; "Useage" misspelling.
Intermediates: 00:09 template 6L/70B (include, blank, main signature, blank, `  return 0;`, `}`). 00:23 usage block in progress. 00:38 `for` header. 01:01 `path` line. 01:20 fopen/perror block (no fclose). 01:56 final.

### cat.c - state at 03:04 (`"cat.c" [New] 25L, 395B written`)
```
 1| #include <stdio.h>
 2|
 3| int main(int argc, char *argv[]) {
 4|   if (argc == 1) {
 5|     printf("Useage: cat [path]...\n");
 6|     return 0;
 7|   }
 8|
 9|   for (int i = 1; i < argc; i++) {
10|     const char *path = argv[i];
11|     FILE *fp = fopen(path, "r");
12|     if (!fp) {
13|       perror(path);
14|       return 1;
15|     }
16|
17|     char c;
18|     while ((c = fgetc(fp)) != EOF)
19|       putchar(c);
20|
21|     fclose(fp);
22|   }
23|
24|   return 0;
25| }
```
Intermediate 02:32: lines 17-18 `char c;` / `while ((c = fgetc(fp))`.

## Shell & outputs
```
[00:00] lab $ ls
        notes.txt touch.c
[00:07] lab $ e tou        (typing; tab becomes `vim touch.c`)
[01:32] lab $ man fopen    (less; FOPEN(3))
[01:41] lab $ cc touch.c -o touch
        lab $ ls
        notes.txt touch     touch.c          (touch in red = executable)
        lab $ ./touch abc.txt
        lab $ ls
        abc.txt   notes.txt touch     touch.c
[01:52] lab $ ls
        lab $ ./touch def.txt xyz.txt
        lab $ ls
        abc.txt   def.txt   notes.txt touch     touch.c   xyz.txt
[02:44] lab $ c            (typing)
[02:54] output of (presumably) cat of notes.txt: lower half incl. Helper section
[02:55] lab $ ./
[02:58] lab $ ./cat cat.c
        #include <stdio.h>

        int main(int argc, char *argv[]) {
          if (argc == 1) {
            printf("Useage: cat [path]...\n");
            return 0;
          }

          for (int i = 1; i < argc; i++) {
            const char *path = argv[i];
            FILE *fp = fopen(path, "r");
            if (!fp) {
              perror(path);
              return 1;
            }

            char c;
            while ((c = fgetc(fp)) != EOF)
              putchar(c);

            fclose(fp);
          }

          return 0;
        }
[03:03] shell cleared: lab $
```
No compiler errors or warnings seen. `cc cat.c -o cat` not visible (happened between 02:32 and 02:55) but binary `./cat` ran at 02:58.

## Observations
- Build order: touch -> cat -> mkdir next (row highlighted at 03:06), following the notes.txt table top to bottom. He highlights the current row/function cell in vim VISUAL mode as a progress pointer.
- Workflow: vim in tab 1, man/shell in tab 2. Prompt `lab $ `. `e` opens vim. Compile `cc file.c -o name`, test `./name args`, verify with `ls`.
- Reuse: new util starts by yank-all (`ggVG`, `y`) of previous file, open new file (probably `:e cat.c`), `p`, delete leading blank line, then edit name/mode (`touch`->`cat`, `"w"`->`"r"`). Shared skeleton: usage check `argc == 1` printing `Useage: <cmd> [path]...\n` and `return 0`; loop `i = 1..argc`; `const char *path = argv[i]`.
- Error handling: `perror(path); return 1;` on failure; fclose result unchecked. cat uses `char c` with `!= EOF` (latent bug, verbatim).
- Man pages: only `man fopen` (section 3), read at SYNOPSIS/mode-letters level; also selected the synopsis with mouse.
- Style: 2-space indent, `if (!fp)`, same-line braces, blank line before `return 0`. "Useage" misspelling and "not exit" typo retained.
- notes.txt includes a "Helper" cheat sheet for later utils: stat follows links, lstat returns link; `struct stat st; stat(path, &st) == -1 // have error`; `errno == ENOENT // not have file`; `S_ISDIR(st.st_mode) // is dir`. Last table row `tree | | watch my previous video`.
- Philosophy: design table with algorithm first, then implement one util at a time, test immediately.
