# Segment 7 (24:32 - 26:49)

## Timeline
- [24:32] Three Terminal windows. Left window (tab "~/lab — vim main.c" + second tab) shows `lab $ ` prompt. A new front Terminal window (middle) shows `lab $ ed` being typed (real system `ed`). Right window = notes file in vim (status "26,3 Bot"; partial text only: "pipes without a human observer.", "> vi -> vim.", dashes line, "the current line", "he current line", "line", "ine", "s", "a file", "o buffer").
- [25:05] Left window now has tabs "~/lab — vim main.c" and "~/lab — -zsh" (second tab active, empty `lab $ ` prompt). Middle window blank/behind.
- [25:06] Switched to vim main.c tab (mouse over tab). main.c shows lines 26-55; cursor 35,41 Bot (on "t" of "// print").
- [25:07] Middle Terminal in front showing the REAL `ed` demo: `lab $ ed` / `a` / `this is line 1` / `this is line 2` / `this is line 3` / `.` / `p` / `this is line 3`.
- [25:08] Middle window hidden again; vim main.c lines 26-55, cursor 35,41 Bot, no changes.
- [25:09] vim scrolled: lines 24-55, cursor 24,20 Bot (on `;` of `on_edit = 0;`).
- [25:10] vim scrolled up: lines 18-53, cursor 18,13 (on `{` of `while (1) {`), 89%.
- [25:14] Left window switched to -zsh tab, empty `lab $ `.
- [25:17] Left window: `lab $ ./a.out` running, tab title now "~/lab — a.out"; middle ed window forward showing same ed session.
- [25:18] Left window only: `lab $ ./a.out` running (spinner on tab), empty line below.
- [25:19] ed window in front (moved right, x~812); left shows `lab $ ./a.out`.
- [25:22] a.out receiving input: `a` / `this is ` (typing "this is line 1").
- [25:34] a.out session (left): `lab $ ./a.out` / `a` / `this is line 1` / `this is line 2` / `this is line 3` / `.` / `p` / `this is line 3` — identical to real ed session beside it.
- [25:42] Real ed continues: after `p` -> `this is line 3`, typed `2` -> `this is line 2`; `1` -> `this is line 1`; `2` -> `this is line 2`; `p` -> `this is line 2`; `3` -> `this is line 3`; `p` -> `this is line 3`. a.out (left) still ends at `p` / `this is line 3`.
- [25:44] vim main.c tab in front (lines 18-53, cursor 18,13, 89%); ed window behind-right with same output.
- [25:45] vim scrolled up: lines 16-51, cursor 17,18 (on `;` of `int on_edit = 0;`), 78%.
- [25:47] vim scrolled down: lines 20-55, cursor 52,19 Bot (on `;` of `free(lines[i]);`).
- [25:49] vim `-- INSERT --`: new line 46 inserted inside the `else { // number` block, containing a lone `/` (starting a comment); file now 56 lines. Cursor 46,15, 95%.
- [26:09] Comment finished: line 46 `// get num and go to line` (cursor 46,33). Mac Dock visible, mouse over Firefox icon.
- [26:10] Firefox window opened over editor (blank page, URL bar "Search with DuckDuckGo or enter address").
- [26:11] Typing URL `beej.us/` with autocomplete dropdown (Beej's Web Page; Beej's Guide to C Programming Library beej.us/guide/bgclr/html/split/; stdio.html#man-gets; stdlib.html#man-strto...; also unrelated YouTube/Proton history items).
- [26:12] URL `beej.us/b`.
- [26:13] URL `beej.us/guide/` (autocomplete).
- [26:14] URL `beej.us/guide/bgclr/`.
- [26:16] Page loaded: "Beej's Guide to C Programming—Library Reference" (second volume, beta-quality doc).
- [26:17] Firefox find bar open with `fgets` on same page.
- [26:18] Find bar closed; mouse over "HTML" link (status: beej.us/guide/bgclr/html/split/).
- [26:19] Page beej.us/guide/bgclr/html/split/: "Beej's Guide to C Programming — Library Reference — Brian "Beej Jorgensen" Hall — v0.9.13, Copyright © October 15, 2026"; contents list start.
- [26:20] Find bar `fgets` open on contents page.
- [26:21] Typed `strto` in find bar; contents scrolled to 9.4 `strtoimax() strtoumax()` highlighted (section 9 `<inttypes.h>`).
- [26:23] Find `strtol`: contents at 24 `<stdlib.h>`: 24.3 atoi(), atol(), atoll(); 24.4 strtod(), strtof(), strtold(); 24.5 strtol(), strtoll(), strtoul(), strtoull(); 24.6 rand() ... 24.9 calloc(), malloc(); 24.10 free() ... Above: 23.15 gets(), fgets(); 23.17 puts(), fputs().
- [26:25] Mouse on strtol() link (status: beej.us/guide/bgclr/html/split/stdlib.html#man-strtol).
- [26:26] stdlib.html loading: "24 <stdlib.h> Standard Library Functions" function table (_Exit(), abort(), abs(), aligned_alloc(), at_quick_exit(), atexit(), atof(), atoi(), bsearch(), calloc(), div(), exit(), free()...).
- [26:27] Section 24.5 strtol(), strtoll(), strtoul(), strtoull() "Convert a string to an integer". Synopsis `#include <stdlib.h>`; `long int strtol(const char * restrict nptr, char ** restrict endptr, int base);` plus strtoll/strtoul/strtoull variants. Description: "like atoi(), but a few more bells and whistles".
- [26:33] Scrolled a bit; Description text ("it'll set it to point at the NUL at the end of the string. If things don't go well, they'll set endptr to point at the first character where things have gone awry... 103z2!").
- [26:39] Top of stdlib.html again; find bar contains `e`.
- [26:42] Section 24.4 strtod(), strtof(), strtold() with find `endptr` highlighted.
- [26:43] Same, scrolled slightly (Description: "Set endptr to NULL if you don't care about where the first invalid character is.").
- [26:44] Description + example code visible.
- [26:45] Example: `if (*endptr == '\0') { printf("What a perfectly-formed number!\n"); } else { printf("I found badness in your number: \"%s\"\n", endptr); }`; also `NAN(foobar_3490)`.
- [26:49] Firefox pushed behind; vim main.c in front, unchanged: line 46 comment `// get num and go to line`, cursor 46,33, 95%. Firefox partly visible at right showing the endptr example.

## File states

### main.c (end of segment; lines 1-16 not visible in this segment)
```
1-15| ?? (lines 1-15 not visible)
16|
17|   int on_edit = 0;
18|   while (1) {
19|     if (!fgets(cmd, MAX_LINE_LEN, stdin)) break;
20|     cmd[strcspn(cmd, "\r\n")] = 0;
21|
22|     if (on_edit) {
23|       if (mode(cmd, "."))
24|         on_edit = 0;
25|       else {
26|         current_line++;
27|         lines[current_line] = strdup(cmd);
28|         count_line++;
29|       }
30|     } else {
31|       if (mode(cmd, "q"))
32|         break;
33|       else if (mode(cmd, "a")) // append
34|         on_edit = 1;
35|       else if (mode(cmd, "p")) { // print
36|         if (count_line > 0 )
37|           puts(lines[current_line]);
38|         else
39|           puts("?");
40|       }
41|       else if (mode(cmd, "d")) { // delete
42|       }
43|       else if (mode(cmd, "w")) { // save
44|       }
45|       else { // number
46|         // get num and go to line
47|         puts("?");
48|       }
49|     }
50|   }
51|
52|   for (int i = 1; i < count_line; i++)
53|     free(lines[i]);
54|
55|   return 0;
56| }
```
Indentation approximate: the screen shows `int on_edit`/`while`/`for`/`return` at ~2 spaces (inside main), body 4, nested 6, and so on, in 2-space steps (the file uses 2-space indentation, `else {` block bodies at +2). Exact absolute columns are inferred from pixel offsets (marked approximate).

Earlier version (before 25:49 insert): lines 45-49 were `else { // number` / `puts("?");` / `}` / `}` / `}` with `for` at 51, `free(lines[i]);` at 52, `return 0;` at 54, `}` at 55 (55 lines total). The only edit in this segment is the added line 46 comment.

No vim save message (`written`) appeared in this segment.

## Shell & outputs
- [24:32] middle terminal: `lab $ ed`
- [25:07] real ed (system) session:
```
lab $ ed
a
this is line 1
this is line 2
this is line 3
.
p
this is line 3
```
- [25:17] left terminal: `lab $ ./a.out` (tab title "~/lab — a.out"); no compile command in this segment.
- [25:22-25:34] typed into a.out:
```
lab $ ./a.out
a
this is line 1
this is line 2
this is line 3
.
p
this is line 3
```
- [25:42] real ed session continued:
```
lab $ ed
a
this is line 1
this is line 2
this is line 3
.
p
this is line 3
2
this is line 2
1
this is line 1
2
this is line 2
p
this is line 2
3
this is line 3
p
this is line 3
```
(No `?` replies seen. Real ed prints no prompt; bare number prints that line and makes it current.)

## Observations
- The programmer demonstrates real `ed` side by side with their a.out to define target behaviors: `a`, text, `.`, `p` (prints current line), then bare line numbers (`2`, `1`, `3`) which move the current line and print it. This motivates the next feature: the `// number` branch, with the comment "get num and go to line" typed at 25:49-26:09.
- Planned parsing via C library number conversion: consulted Beej's Guide to C Programming Library Reference (beej.us/guide/bgclr/): searched fgets, strto*, strtol() (24.5, endptr/base) and strtod() (24.4), focusing on the `endptr` check (`*endptr == '\0'`) for validating the whole string as a number. (Earlier man page "man 3 strncmp" noted in task context is not in this segment.)
- Style: 2-space indent, `else if (mode(cmd, "x"))` chain with trailing `// name` comments, empty brace placeholders for `d` and `w`, `puts("?")` as the error response, `strdup` into `lines[]`, `count_line`/`current_line`/`on_edit` as locals in main, final `for` loop frees lines starting at index 1 (lines[] 1-indexed, index 0 unused; note loop `i < count_line` would miss the last line if count_line == last index).
- Workflow: uses Firefox (DuckDuckGo default, Firefox Suggest history) for docs rather than man pages in this stretch.
