# Segment 2 transcript (0:53 - 1:47)

## Timeline
Window: macOS Terminal, 2 tabs: tab 1 `~/my-project — vim main.c`, tab 2 `~/my-project — -zsh`. Prompt is `my-project $ `. Webcam "Human" window ignored.

- [00:53-00:56] zsh tab active. Screen: `my-project $ cc main.c` then empty prompt `my-project $ ` (cc printed nothing, so it compiled with no warnings). Frames 0:53, 0:54, 0:55, 0:56 near-identical.
- [00:57] Typed `./a.out ` at the second prompt (not yet executed). Shell-integration bracket marks `[` / `]` at left/right edges of command lines.
- [00:59-01:01] After running:
  ```
  my-project $ cc main.c
  my-project $ ./a.out
  Hello World
  my-project $ 
  ```
  (old hello-world binary). Frames 0:59, 1:00, 1:01 identical.
- [01:02] Mouse clicked tab 1 (`vim main.c`). Vim normal mode, cursor on `}` line 7 (brace-match highlight). File has 7 lines (see File states).
- [01:03-01:04] Normal mode; block cursor on the `\` of `\n` in line 4 (mouse I-beam nearby). No edits.
- [01:05] INSERT mode (`-- INSERT --`). New line inserted after line 4; cursor on new line 5 at 2-space indent. File now 8 lines (line 6 blank, 7 `  return 0;`, 8 `}`).
- [01:10-01:17] (frames 1:10, 1:11, 1:12, 1:13, 1:14, 1:15, 1:16, 1:17) INSERT mode. Line 5 is `  while (1) {`; line 6 blank with cursor at 4-space indent; line 7 empty; line 8 `  return 0;`; line 9 `}`. No change across these frames. No closing brace for the while exists yet.
- [01:19] Line 6 being typed: `    printf("` (cursor right after opening quote).
- [01:24] Line 6: `    printf("No AI --- "` (cursor after closing quote; `);` not yet typed).
- [01:27] Normal mode, file now 10 lines; line 6 `    printf("No AI --- ");`, line 7 `  }`, line 8 blank, line 9 `  return 0;`, line 10 `}`. Cursor on line 7 `}` (matching `{` on line 5 highlighted). The `);` and the closing `  }` were typed between 1:24 and 1:27 (not captured).
- [01:30] Cursor on line 4 col 3 (`p` of printf). Status line: `"main.c" 10L, 123B written` (CHECKSUM).
- [01:31] Switched to zsh tab: history (`cc main.c`, `./a.out`, `Hello World`) and new prompt `my-project $ cc m` (typing).
- [01:32] `my-project $ cc main.c` entered; cursor on next line (command running). Tab title momentarily reads `...extra -pedantic -Wno-unused-parameter main.c`: `cc` is an alias/wrapper that expands to extra warning flags (start truncated with `...`; likely `-Wall -Wextra -pedantic -Wno-unused-parameter`, the part before "extra" is not visible, UNCERTAIN). No compiler output visible.
- [01:35] `./a.out` is running (command line scrolled off). Tab title `~/my-project — a.out` with busy spinner. Screen full of `No AI --- ` repeats: 28 rows x 9 repeats. Last line partial: `No AI --- No AI --- No AI --- No AI --- No AI --- No` plus block cursor.
- [01:37] Same, last row complete; scrolling continues. Spinner shown.
- [01:38] Same; mouse I-beam at about (787,290) over output.
- [01:39] Same; last row ends `No AI ` plus cursor; I-beam at about (805,475).
- [01:40] Same; last row complete.
- [01:44] Ctrl-C pressed: tab title back to `~/my-project — -zsh`, mouse arrow hovering tab 2 close button. Output shows `^C` inline mid-row, subsequent rows shifted by one column, then a new prompt `my-project $ ` with cursor. Scrollbar thumb visible bottom-right.
- [01:46] Same final screen; mouse I-beam near the window title bar.
- [01:47] Same final screen; mouse arrow at top of screen over the macOS menu bar (a green menu-bar item visible). Segment ends.

## File states

### main.c at 1:02 (opened in vim, before editing)
```
1| #include <stdio.h>
2| 
3| int main(void) {
4|   printf("Hello World\n");
5| 
6|   return 0;
7| }
```

### main.c at 1:05 (insert mode, new blank line)
```
1| #include <stdio.h>
2| 
3| int main(void) {
4|   printf("Hello World\n");
5|   ⟨cursor⟩
6| 
7|   return 0;
8| }
```

### main.c at 1:10-1:17
```
1| #include <stdio.h>
2| 
3| int main(void) {
4|   printf("Hello World\n");
5|   while (1) {
6|     ⟨cursor⟩
7| 
8|   return 0;
9| }
```

### main.c at 1:24
```
5|   while (1) {
6|     printf("No AI --- "⟨cursor⟩
```

### main.c FINAL (1:27-1:30, saved: `"main.c" 10L, 123B written`)
```
 1| #include <stdio.h>
 2| 
 3| int main(void) {
 4|   printf("Hello World\n");
 5|   while (1) {
 6|     printf("No AI --- ");
 7|   }
 8| 
 9|   return 0;
10| }
```
Indentation: 2 spaces for body, 4 inside the while. The string is `No AI ` + three dashes + trailing space; displayed dashes look like long dashes but a byte count shows plain ASCII hyphens. Byte check: 19+1+17+27+14+26+4+1+12+2 = 123, which matches `123B` exactly (line 1 `#include <stdio.h>`=19 with newline; blank=1; `int main(void) {`=17; `  printf("Hello World\n");`=27; `  while (1) {`=14; `    printf("No AI --- ");`=26 with string of 10 ASCII bytes; `  }`=4; blank=1; `  return 0;`=12; `}`=2). So the string is exactly `No AI --- ` (10 ASCII bytes, no `\n`), and each of those lines has no trailing whitespace. (Lines 2 and 8 blank shown as empty; line 5 cursor-blank earlier was removed/overwritten by `while`.)

## Shell & outputs
- [00:53] `my-project $ cc main.c` -> no output.
- [00:57] `my-project $ ./a.out`
- [00:59] output: `Hello World`, then prompt returns.
- [01:31-01:32] `my-project $ cc main.c` (typed `cc m` first). Tab title at 1:32: `...extra -pedantic -Wno-unused-parameter main.c`. No warnings/errors displayed.
- [~01:34, not visible] `./a.out` run; at 1:35 tab title `~/my-project — a.out` with spinner.
- [01:35-01:40] Output pattern: `No AI --- ` (N o space A I space - - - space) repeated with no newline; the terminal is about 90 columns wide so each row holds exactly 9 repeats (rows read `No AI --- No AI --- ... No AI ---`, with the trailing space at the wrap edge). 28 rows visible, scrolling continuously.
- [01:44] `^C`. The interrupt row reads: `No AI --- No AI --- No AI --- No AI --- ^CNo AI --- No AI --- No AI --- No AI --- No AI --` and subsequent rows start with `- No AI --- ...` (shifted one column because ^C takes 2 columns, no newline emitted), last partial row `- No AI --- No AI --- No AI --- No AI --- No AI --- No AI --- No`, then new prompt `my-project $ ` on the next line. No "Terminated" message.

## Observations
- The program is a deliberate infinite loop printing "No AI --- " with no newline, flooding the screen as a visual statement matching the video title; no `\n`, `fflush`, or `break`; stopped manually with Ctrl-C.
- Style: 2-space indent, K&R braces, `int main(void)`, retained `return 0;` after the infinite loop (unreachable), blank line before return, leftover `Hello World\n` printf kept. Closing brace typed after the body (no auto-pairing visible).
- Workflow: edit in vim tab, write (`"main.c" 10L, 123B written`), click zsh tab, `cc main.c` (a `cc` alias/wrapper with strict warning flags per tab title), then `./a.out`. First run (old code) printed `Hello World`.
- No shell `#` comments or other on-screen text. Mouse moves to the menu bar at the very end (1:47), likely to stop recording.
- Not captured in keyframes: the `);` and closing `}` typing (1:24-1:27), the `:w` keystrokes (1:27-1:30), the `./a.out` entry (1:32-1:35), and exact `cc` alias expansion.
