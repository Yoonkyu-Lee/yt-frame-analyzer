# Segment 1 transcript (0:00 - 0:53), video WLih6RtW_hE

## Timeline
- [00:00] Blank dark-gray macOS desktop, mouse cursor center. Orange dot top-right (menu bar recording/mic indicator).
- [00:02] Dock appears at bottom: Finder, Launchpad-like grid, Dictionary/Books-like icon, Firefox-like icon, Terminal (tooltip "Terminal", cursor on it), a green thumbnail, Trash.
- [00:03] Terminal window opened (title "Terminal", no tabs bar). Prompt: `~ $ ` with block cursor. Dock still visible (auto-hide).
- [00:07] Terminal shows (after typing, not seen mid-typing): `~ $ scam start` -> output `starting camera...` / `camera started` / `pid: 1045` / `log: /tmp/scam.log`, then new prompt `~ $ `. Small bracket marks at left/right of the command line (zsh/Terminal prompt marks).
- [00:08] Same; Terminal unfocused (hollow cursor), mouse arrow at center-lower. Dot top-right still orange.
- [00:09-00:11] "Human" webcam window (title `Human` + old-man emoji) appears LARGE (spans x~368-1543, covering most of the right/center, over the terminal). Top-right indicator dot turns green (camera on). Webcam content ignored (man smoking). ENLARGED.
- [00:12] Human window still large; cursor moved to top-right corner (near its edge).
- [00:13] Human window shrinking (resize in progress; slightly smaller, resize arrow icon visible top-right of window).
- [00:14] Human window smaller still (about x368-805, y377-920, portrait), overlaying lower-left of the terminal.
- [00:15] Human window slightly moved (x367-771, y443-920), cursor dragging it.
- [00:16] Human window docked at right (x1127-1531, y325-803), terminal visible fully; cursor at its titlebar.
- [00:17] Same layout; terminal re-focused (solid block cursor), I-beam mouse pointer over terminal.
- [00:19] Typing: `~ $ mkdir my-` 
- [00:20] `~ $ mkdir my-proje`
- [00:21] `~ $ mkdir my-project`
- [00:22] Same, `mkdir my-project` typed, not yet executed.
- [00:23] Executed: new prompt `~ $ ` below; no output (silent success).
- [00:24] Typing `~ $ cd m` (rest typed later; likely `cd my-project`).
- [00:26] Screen cleared; prompt now `my-project $ ` (cwd shown as basename only; no `~`). Prompt format is `<dirname> $ `.
- [00:29] Screen: `my-project $ ls` (empty output, dir empty), then `my-project $ vim mai` (typing `vim main.c`). Screen had been cleared between 26s and 29s (scam/mkdir lines gone; `ls` is first line) - so a `clear` was likely used before `ls`, or ls was first after clear at 0:26.
- [00:32] vim opened, new file. Line numbers on (number column, width 3 + space). Mode `-- INSERT --`. Line 1: `#incl` (typing). Purple `~` for empty lines.
- [00:33] Line 1: `#include`
- [00:34] Line 1: `#include ` (trailing space, cursor after).
- [00:36] Line 1: `#include <stdio.h>` (syntax highlighted: `#include` cyan, `<stdio.h>` magenta).
- [00:40] Lines: 1 `#include <stdio.h>`, 2 (blank), 3 `int main(void) {`, 4 `  ` (cursor, auto/typed 2-space indent). Still INSERT.
- [00:44] Line 4: `  printf("Hello World\n"` (closing paren and semicolon not yet typed).
- [00:46] Line 4 `  printf("Hello World\n");`, line 5 blank, line 6 `  return` typing.
- [00:47] Line 6 `  return 0;`, line 7 cursor (indented 2 spaces) in INSERT.
- [00:49] Line 7 now `}` at column 0 (bracket-match highlight on `{` line 3 and `}` line 7). Normal mode (no mode text). Status line verbatim: `"main.c" [New] 7L, 79B written`. (File saved with :w; vim still open.)
- [00:50] Terminal now has TAB bar: tab 1 `~/my-project — vim main.c`, tab 2 (active) `~/my-project — -zsh`. New tab opened (Cmd-T), prompt `my-project $ `. Window grew taller (to y~910).
- [00:53] In zsh tab: `my-project $ cc main.c` -> no output (no warnings/errors), new prompt `my-project $ `.

## File states
main.c (vim, as of 0:49, saved 7L, 79B):
```
1| #include <stdio.h>
2| 
3| int main(void) {
4|   printf("Hello World\n");
5| 
6|   return 0;
7| }
```
Indent is 2 spaces. Line 5 blank (empty, may contain nothing). Blank line 2 empty. File was written once and not yet changed in this segment.

Intermediate versions: 0:32 `#incl`; 0:36 line1 complete; 0:40 lines 1-4 partial (`int main(void) {` with open brace); 0:44 printf without `);`; 0:47 complete body with `return 0;`; 0:49 closing `}` added and saved.

## Shell & outputs
- [00:07] `~ $ scam start`
  ```
  starting camera...
  camera started
  pid: 1045
  log: /tmp/scam.log
  ```
- [00:23] `~ $ mkdir my-project` (no output)
- [00:24-00:26] `cd my-project` (partially visible: `cd m`; result prompt `my-project $ ` at 0:26)
- [00:29] `my-project $ ls` (empty output) ; `my-project $ vim main.c` (typed: `vim mai...`; vim opens main.c, tab title confirms `vim main.c`)
- [00:50] new tab zsh (`~/my-project — -zsh`)
- [00:53] `my-project $ cc main.c` (no output; produces a.out presumably, not shown)

## Observations
- Prompt is customized: `<cwd basename> $ ` (home shown as `~`). Tab title reveals `-zsh` shell; `cc` used, not gcc/clang name.
- `scam start` is a custom helper that starts the webcam; it prints a pid and log path (/tmp/scam.log). Webcam turns on (indicator turns green at 0:09). The camera window "Human" was shown large at 0:09-0:13, then shrunk and docked right side by 0:16.
- Workflow: mkdir project, cd, ls (empty), vim main.c, type everything character by character, save (:w), second terminal tab for compile (two-tab vim/zsh workflow), cc main.c.
- Style: 2-space indent, `int main(void)`, `return 0;`, blank line between printf and return; hello-world is initial version (later turns into a loop per brief). No comments, no `#` shell comments seen.
- Unreadable/uncertain: exact `cd` command text (only `cd m` seen); whether `clear` was typed between 0:24 and 0:29; line 5 and line 2 whitespace; file contents after 0:53 not covered.
