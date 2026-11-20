# Segment 5 (17:15 - 21:24) - "building my own calendar in C - UNIX cal" (Hayyoth)

Layout throughout: left window "Terminal" running vim on `main.c`; behind it a second "Terminal" showing a man page (tm struct fields; static, never changes); a third small "Terminal" (shell prompt `lab $`) is raised on top intermittently for `cal` / compile / run. Webcam overlay "Human" ignored.

## Timeline
- [17:15] vim normal mode, cursor 10,33 (24%). Lines 10-40 visible: `get_month_tm`, `first_weekday`, `days_in_month`, `print_month` (unfinished: `int total_days = days_in_month(`), start of `main`.
- [17:16] Scrolled up (7,33, 16%): line 7 `int get_day(const struct tm *date) { return date->tm_mday; }`, lines 9-10 `get_year_tm` / `get_month_tm`.
- [17:18] Scrolled to 25-55 (25,23, 64%): main body: `int year = 0, month = 0;`, `time_t now_t = time(NULL);`, `struct tm *now = localtime(&now_t);`, `switch (argc)` case 1 (`year = get_year(now); month = get_month(now);`), case 2 (`year = atoi(argv[1]);`), case 3 start (`month = atoi(argv[1]);`).
- [17:19] 22-52, cursor 25,23 (56%); line 37 still `int total_days = days_in_month(`.
- [17:27] INSERT. Line 37 completed `int total_days = days_in_month(year, month);`; new line 38 being typed `in` (38,5; main now at line 41).
- [17:40] Normal mode, line 38 `int first_day =` (unfinished); cursor 16,18 (39%) - looking at earlier code.
- [17:41] Cursor 11,0-1 (26%) next to `first_weekday` (line 12).
- [17:44] VISUAL mode (count "2") highlighting the signature `int first_weekday(int year, int month)` on line 12 (to recall the name).
- [17:47] Back at 38,18 normal mode; line 38 `int first_day =`.
- [17:55] INSERT: line 38 `int first_day = first_weekday(year, month);`, new blank line 39 (39,2).
- [17:57] Scrolled 24-54; two blank lines (39,40) after line 38; cursor 53,2 normal (57%).
- [18:03] INSERT, line 40 `  printf("` typed. Small Terminal raised with `lab $ cal` being typed.
- [18:04] Real `cal` run for reference (output below), today (17) highlighted.
- [18:08] Terminal lowered; line 40 `  printf("`, cursor 40,11.
- [18:10] Mode line `-- INSERT (paste) --`: line 40 now `  printf("Su Mo Tu We Th Fr Sa\n")` (parens highlighted), cursor 40,35 - header text pasted from the cal output.
- [18:12] `;` present: `  printf("Su Mo Tu We Th Fr Sa\n");`; blank line 41 (41,3 INSERT).
- [18:45] Normal mode, cursor 41,1 on closing `}`, matching `{` of line 36 highlighted.
- [18:46] INSERT again 41,3.
- [18:47] Extra blank line opened: cursor 42,3 INSERT (54%); closing `}` at 43.
- [19:05] Line 42 `  for (int i = 0; i < first_day; i++)`; body line 43 empty, cursor 43,5 INSERT.
- [19:11] Line 43 `    printf("   ");` (three spaces in the quotes); cursor 45,3 INSERT.
- [19:34] Line 45 `  for (int day = 1; day <= total_days; day++) {`; cursor 46,5.
- [19:50] Line 46 `    printf("%2d ", day);`; blank 47 (47,5), `  }` at 48, `}` at 49.
- [19:51] Same, scrolled (27-57 visible, 55%).
- [19:58] Scrolled to end of file (Bot, 55,0-1): end of main: `if (month) {` / `printf("year = %d, month = %d\n", year, month);` / `} else {` / `printf("year = %d\n", year);` / `}` / two blank lines / `return 0;` / `}` (78 lines).
- [20:01] The debug printf line deleted; `if (month) {` then `} else {` directly (77 lines), cursor 70,3.
- [20:02] INSERT at line 71: `    pri` typed (71,8), file 78 lines.
- [20:09] Line 71 now `    print_month(year, month);`. Saved: `"main.c" 78L, 1558B written`. Small Terminal raised with empty `lab $` prompt.
- [20:13] Terminal: `lab $ cc main.c`, `lab $ ./a.out` -> header then all days printed on one line (soft-wrapped), no newlines (output below).
- [20:20] Editor normal mode, cursor 70,14 on `if (month) {`, matching `}` highlighted; status line still the 78L save message.
- [20:21] Scrolled to lines 43-73, cursor 43,18 (89%).
- [20:22] Lines 39-69, cursor 40,29 (80%).
- [20:24] INSERT at blank line 47 inside the day loop (47,5, 79%).
- [20:25] Typed `    if` on line 48 (48,7).
- [20:27] `    if (` (48,9, 71%). [20:28] same, scrolled (65%).
- [20:55] Line 48 `    if ((first_day + day) % 7 == 0 || day == total_days)`; blank line 49, cursor 49,7 INSERT (64%).
- [21:03] Line 49 `      printf("\n");`. Saved: `"main.c" 81L, 1636B written`. Terminal raised still showing the old (pre-change) run.
- [21:04] New terminal screen: `lab $ cc ma` being typed.
- [21:07] After `cc main.c` / `./a.out`: weeks now break correctly (output below). No compiler errors or warnings shown.
- [21:10] `lab $ cal` run for side-by-side comparison; real cal additionally has the `November 2026` title and highlighted 17.
- [21:16] Editor normal mode, 49,19 (64%), status `"main.c" 81L, 1636B written`.
- [21:18] INSERT: after day loop's `}` (line 50), new line 51 `  prin` (51,7, 62%).
- [21:20] `  printf("` on line 51, blank line 52 below with cursor (52,6).
- [21:22] `  printf("\` (51,12).
- [21:24] `  printf("\n");` complete; saved: `"main.c" 82L, 1652B written`. Cursor 51,15.

## File states
`main.c` at end of segment (21:24). Lines 1-6 and 8 not visible in this segment. Numbers from screens: print_month at 36-52, main at 54 (consistent with 82L).

```
1| ?? (lines 1-6 not visible in this segment; get_year(now)/get_month(now) helpers presumably there)
7| int get_day(const struct tm *date) { return date->tm_mday; }
8|
9| int get_year_tm(int year) { return year - 1900; }
10| int get_month_tm(int month) { return month - 1; }
11|
12| int first_weekday(int year, int month) {
13|   struct tm date = {0};
14|
15|   date.tm_year = get_year_tm(year);
16|   date.tm_mon = get_month_tm(month);
17|   date.tm_mday = 1;
18|
19|   mktime(&date);
20|
21|   return date.tm_wday;
22| }
23|
24| int days_in_month(int year, int month) {
25|   struct tm date = {0};
26|
27|   date.tm_year = get_year_tm(year);
28|   date.tm_mon = get_month_tm(month) + 1;
29|   date.tm_mday = 0;
30|
31|   mktime(&date);
32|
33|   return date.tm_mday;
34| }
35|
36| void print_month(int year, int month) {
37|   int total_days = days_in_month(year, month);
38|   int first_day = first_weekday(year, month);
39|
40|   printf("Su Mo Tu We Th Fr Sa\n");
41|
42|   for (int i = 0; i < first_day; i++)
43|     printf("   ");
44|
45|   for (int day = 1; day <= total_days; day++) {
46|     printf("%2d ", day);
47|
48|     if ((first_day + day) % 7 == 0 || day == total_days)
49|       printf("\n");
50|   }
51|   printf("\n");
52| }
53|
54| int main(int argc, char *argv[]) {
55|   int year = 0, month = 0;
56|
57|   time_t now_t = time(NULL);
58|   struct tm *now = localtime(&now_t);
59|
60|   switch (argc) {
61|     case 1:
62|       year = get_year(now);
63|       month = get_month(now);
64|       break;
65|     case 2:
66|       year = atoi(argv[1]);
67|       break;
68|     case 3:
69|       month = atoi(argv[1]);
70|       year = atoi(argv[2]);
71|       break;
72|   }
73|
74|   if (month) {
75|     print_month(year, month);
76|   } else {
77|     printf("year = %d\n", year);
78|   }
79|
80|
81|   return 0;
82| }
```
(Lines 54-82 reconstructed from the 20:09 screen, where `int main` was at 50 and the `switch` at 56, shifted by the +4 lines added later to print_month; consistent with 82L total. Not all of 73-82 were on screen at 21:24 - marked reconstructed.)

Intermediate version at 20:09 (`"main.c" 78L, 1558B written`): print_month ended with the day loop containing only `printf("%2d ", day);` + blank line + `  }` + `}` (lines 45-49), `main` at line 51; `if (month) {` at 70, `print_month(year, month);` at 71.

## Shell & outputs
- [18:03-18:04] `lab $ cal`:
```
   November 2026
Su Mo Tu We Th Fr Sa
 1  2  3  4  5  6  7
 8  9 10 11 12 13 14
15 16 17 18 19 20 21
22 23 24 25 26 27 28
29 30
```
(17 in reverse video; then blank line and `lab $`)
- [20:13] `lab $ cc main.c` / `lab $ ./a.out`:
```
Su Mo Tu We Th Fr Sa
 1  2  3  4  5  6  7  8  9 10 11 12 13 14 15 16 17 18 19 20
21 22 23 24 25 26 27 28 29 30 %
```
(second row is terminal soft-wrap; trailing `%` is the shell's no-newline marker)
- [21:03] Terminal still shows the same old output (grey selected cell on "7" is a selection artifact).
- [21:07] `lab $ cc main.c` / `lab $ ./a.out`:
```
Su Mo Tu We Th Fr Sa
 1  2  3  4  5  6  7
 8  9 10 11 12 13 14
15 16 17 18 19 20 21
22 23 24 25 26 27 28
29 30
lab $
```
- [21:10] `lab $ cal` again below it: `   November 2026` + same grid, 17 highlighted, then blank line, `lab $`.
- vim save messages (checksums): `"main.c" 78L, 1558B written` (20:09), `"main.c" 81L, 1636B written` (21:03), `"main.c" 82L, 1652B written` (21:24).

## Observations
- Incremental build-and-compare: header, then leading padding loop, then day loop, then row-break condition, then trailing newline; each step compiled with `cc main.c` and run against the real `cal` as reference/spec (header text copied from cal's output).
- Debug printf in main's `if (month)` is replaced by the real `print_month(year, month)` call; the else branch (year only) is still a stub `printf("year = %d\n", year);`.
- Style: 2-space indent, unbraced single-statement `for`/`if` bodies, small helpers (`get_year_tm`, `get_month_tm`), `struct tm date = {0}` + `mktime` normalization trick (day 0 of next month = last day) for `days_in_month`, `tm_wday` for first weekday.
- Row-break condition `(first_day + day) % 7 == 0 || day == total_days` relies on Sunday = 0 and day starting at 1.
- Not yet done at end of segment: month/year title line, current-day highlight, year view, trailing-space matching (his rows end with a space, real cal's do not).
- The man page window in the background is a static reference for `struct tm` fields (never scrolled in this segment).
