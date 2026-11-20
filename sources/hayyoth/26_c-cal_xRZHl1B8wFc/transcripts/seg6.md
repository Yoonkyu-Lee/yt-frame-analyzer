# Segment 6 (21:24 - 25:34) - cal clone in C, adding "highlight today"

Setup: three macOS Terminal windows. Left: vim editing `main.c` (line numbers on). Back-right: a terminal showing a man page (mktime/time.h `struct tm` field list; left part hidden behind vim). Middle (pops up in front at times): shell prompt `lab $`. Bottom-right: webcam "Human" (ignored).

## Timeline

- [21:24] vim, normal mode, cursor 51,15 on `printf("\n");` (line 51). Status: `"main.c" 82L, 1652B written` (cursor `51,15  62%`). Code = print_month without highlighting (File states version A).
- [21:25] Middle terminal appears with earlier history: `lab $ cc main.c`, `lab $ ./a.out` (November grid, no title; last row `29 30` shown mouse-selected), then `lab $ cal` -> real cal output `November 2026` with `17` in inverse video (today = Tue 17).
- [21:26] Terminal cleared; typing `lab $ cc ma`.
- [21:28] `lab $ cc main.c` / `lab $ ./a.out` -> November grid, no title, no highlight.
- [21:37] Ran `lab $ ./a.out 10 2026` -> October 2026 grid (1 under Th).
- [21:41] Ran `lab $ cal 10 2026` (system cal) -> `October 2026` title plus identical grid.
- [21:45-21:48] Mouse-selects his own output of `./a.out 10 2026`, then the system `cal 10 2026` output, to compare; they match except the title line.
- [21:51] Terminal cleared, typing `lab $ ./`.
- [21:52] `lab $ ./a.out` -> November grid, no highlight.
- [21:53] `lab $ cal` -> `November 2026` with `17` inverse. (Goal: make own program highlight today.)
- [21:59] Focus back to vim (cursor 50,3, status still `"main.c" 82L, 1652B written`, `62%`).
- [22:02] vim INSERT: a blank line opened at line 46 inside the `for (int day ...)` loop (cursor 46,5, `61%`); file temporarily 83 lines.
- [22:15] Reverted: blank line gone (`45,5`, `62%`), cursor on `for` at line 45; no save message.
- [22:38] Scrolled to bottom (`82,1  Bot`): main() visible (File states A). Brace-match highlight on `{` of main at line 54. Call is `print_month(year, month);`.
- [22:44] Cursor 47,0-1 (`90%`): main body now shows `print_month(year, month, now);` at line 75. So between 22:38 and 22:44 the call site was changed to pass `now`.
- [22:45] Cursor 33,22 (`62%`). `print_month` signature now `void print_month(int year, int month, const struct tm *now) {` (line 36).
- [22:46] Scrolled up: lines 31-61 visible (`32,0-1  58%`).
- [22:48] Scrolled up further: lines 25-55 visible (`33,22  47%`).
- [22:50] vim INSERT at line 40 (`40,3  45%`): blank lines opened between `first_day` decl and `printf("Su Mo...")` (header printf now at line 42).
- [23:01] INSERT: line 40 is `int month_now = get_month(now);`; line 41 partial `in` (`41,5  44%`).
- [23:09] Lines 40-41: `int month_now = get_month(now);` / `int year_now = get_year(now);`; cursor on blank line 42 in INSERT (`42,3  43%`).
- [23:25] Jumped to top of file (`1,1  Top`), showing includes and helper one-liners (File states).
- [23:27] VISUAL mode selecting lines 6-7 (`2  6,6`).
- [23:28] VISUAL selecting lines 5-7 (`3  5,1`) (looking at the get_year/get_month/get_day trio).
- [23:30] Back in normal mode, cursor 8,0-1 (`Top`).
- [23:31] Cursor 32,0-1 (`1%`), viewport lines 2-32.
- [23:32] Cursor 43,0-1 (`21%`): print_month has `int day_now = get_day(now);` (line 42).
- [23:33] Cursor 45,0-1 (`25%`): line 44 is `printf("Su Mo Tu We Th Fr Sa\n");`.
- [23:34] Cursor 55,15 (`45%`), print_month lines 36-56 visible.
- [23:37] INSERT at line 44 with partial `i` (`44,4  43%`): starting to type the `is_current_month` line (header printf pushed to 46).
- [24:09] Cursor 59,0-1 (`54%`): line 44 reads `int is_current_month = year == year_now && month == month_now;`. print_month ends line 58, main at line 60.
- [24:12] INSERT: blank line 52 inside the for-day loop (`52,5  53%`); old `printf("%2d ", day);` pushed to line 53.
- [24:32] INSERT (`54,5  52%`): typed `if (is_current_month && day == day_now)` + `printf("%2d ", day);` at lines 52-53, cursor on blank line 54.
- [24:36] Normal mode, cursor 55,7 (`51%`): lines 52-55 are `if (is_current_month && day == day_now)` / `printf("%2d ", day);` / `else` / `printf("%2d ", day);` (first branch still a copy of the original printf).
- [25:07] INSERT (`53,43  51%`): line 53 being rewritten as `printf("%s%2d%s ", "\033[7m", day, "` + `);`, partial string; syntax highlighting turns red because the string is unterminated.
- [25:15] Line 53 complete: `printf("%s%2d%s ", "\033[7m", day, "\033[0m");` Normal mode cursor 53,50. Save message: `"main.c" 91L, 1950B written`.
- [25:17] Middle terminal appears with old history (`./a.out` grid; `cal` -> `November 2026` with 17 inverse).
- [25:18] Terminal cleared; typing `lab $ cc ma`.
- [25:21] `lab $ cc main.c` / `lab $ ./a.out` -> grid with `17` in inverse video (no title line); no compiler diagnostics.
- [25:26] `lab $ cal` run below: `November 2026`, same grid, 17 inverse. His output now matches system `cal` except the missing month/year title.
- [25:29] Terminal hidden; vim shows same code (`"main.c" 91L, 1950B written`, cursor 53,50).
- [25:31] Scroll up: cursor 21,22 (`33%`), lines 21-51 visible.
- [25:32] Cursor 21,22 (`31%`), lines 20-50 visible.
- [25:34] Cursor 63,34 (`53%`) on `{` of `int main(int argc, char *argv[]) {` (line 63, brace highlighted); lines 33-63 visible. End of segment.

Mistakes/fixes: [22:02] blank line added in the loop then undone by [22:15] (no save). `print_month` extended with `const struct tm *now`; caller updated.

## File states

### main.c - version A (start of segment, 21:24, `"main.c" 82L, 1652B written`); lines 54-82 as seen at 22:38
```
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
(`printf("   ")` at line 43 is three spaces.) At 22:44 line 75 is `    print_month(year, month, now);`.

### main.c - header/helpers, lines 1-34 (seen 23:25 onward)
```
 1| #include <stdio.h>
 2| #include <time.h>
 3| #include <stdlib.h>
 4| 
 5| int get_year(const struct tm *date) { return date->tm_year + 1900; }
 6| int get_month(const struct tm *date) { return date->tm_mon + 1; }
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
```
Note: `get_day` (line 7) is already present at 23:25; whether it was added earlier or between 22:38-23:25 is not visible (lines 1-32 weren't on screen at 21:24).

### main.c - END of segment (25:15-25:34, `"main.c" 91L, 1950B written`)
```
33|   return date.tm_mday;
34| }
35| 
36| void print_month(int year, int month, const struct tm *now) {
37|   int total_days = days_in_month(year, month);
38|   int first_day = first_weekday(year, month);
39| 
40|   int month_now = get_month(now);
41|   int year_now = get_year(now);
42|   int day_now = get_day(now);
43| 
44|   int is_current_month = year == year_now && month == month_now;
45| 
46|   printf("Su Mo Tu We Th Fr Sa\n");
47| 
48|   for (int i = 0; i < first_day; i++)
49|     printf("   ");
50| 
51|   for (int day = 1; day <= total_days; day++) {
52|     if (is_current_month && day == day_now)
53|       printf("%s%2d%s ", "\033[7m", day, "\033[0m");
54|     else
55|       printf("%2d ", day);
56| 
57|     if ((first_day + day) % 7 == 0 || day == total_days)
58|       printf("\n");
59|   }
60|   printf("\n");
61| }
62| 
63| int main(int argc, char *argv[]) {
64|   ?? (lines 64-91 not visible at end; presumably version A main lines shifted by +9, call `print_month(year, month, now);`)
```
Line count check: 82 + 9 = 91 (matches `91L`).

## Shell & outputs

- [21:25] (history) `lab $ cc main.c`, `lab $ ./a.out`, grid, `lab $ cal` -> `   November 2026`, `Su Mo Tu We Th Fr Sa`, rows 1-7, 8-14, `15 16 17(inverse) 18 19 20 21`, 22-28, `29 30`.
- [21:26] `lab $ cc ma` (typing)
- [21:28] `lab $ cc main.c` (no output); `lab $ ./a.out`:
```
Su Mo Tu We Th Fr Sa
 1  2  3  4  5  6  7
 8  9 10 11 12 13 14
15 16 17 18 19 20 21
22 23 24 25 26 27 28
29 30
```
- [21:37] `lab $ ./a.out 10 2026`
```
Su Mo Tu We Th Fr Sa
              1  2  3
 4  5  6  7  8  9 10
11 12 13 14 15 16 17
18 19 20 21 22 23 24
25 26 27 28 29 30 31
```
- [21:41] `lab $ cal 10 2026`
```
    October 2026
Su Mo Tu We Th Fr Sa
              1  2  3
 4  5  6  7  8  9 10
11 12 13 14 15 16 17
18 19 20 21 22 23 24
25 26 27 28 29 30 31
```
- [21:51] `lab $ ./` (typing) -> [21:52] `lab $ ./a.out` -> November grid, no highlight.
- [21:53] `lab $ cal` -> `   November 2026` + grid, 17 inverse.
- [25:17-25:18] old history shown, then cleared; `lab $ cc ma`.
- [25:21] `lab $ cc main.c` (no diagnostics), `lab $ ./a.out`:
```
Su Mo Tu We Th Fr Sa
 1  2  3  4  5  6  7
 8  9 10 11 12 13 14
15 16 [17 inverse] 18 19 20 21
22 23 24 25 26 27 28
29 30
```
- [25:26] `lab $ cal` -> `   November 2026` / same grid, 17 inverse.
vim save messages: [21:24] `"main.c" 82L, 1652B written`; [25:15] `"main.c" 91L, 1950B written`.

## Observations

- Method: verifies against the real `cal` by running `./a.out 10 2026` and `cal 10 2026` and visually comparing (mouse-selecting each output); system tool is the oracle.
- Goal here: reproduce cal's "highlight today" via ANSI reverse video `\033[7m` ... `\033[0m`. Still missing vs real cal: the centered month/year title line.
- Design: pass `now` (`const struct tm *`) into print_month; derive `month_now/year_now/day_now` through tiny getter helpers (`get_year`, `get_month`, `get_day`) that hide the tm_year+1900 / tm_mon+1 offsets; `is_current_month` computed once outside the loop; named ints preferred over inline conditions.
- Style: 2-space indent, braceless single-statement if/else bodies, blank lines between logical steps, no comments, `get_*_tm` naming for tm-convention conversions.
- Reference: man page of time.h/mktime (struct tm fields, `day of week (Sunday = 0)`, `year - 1900`, `month of year (0 - 11)`) open in a back terminal.
- Workflow: `:w` in vim, switch to terminal, plain `cc main.c` (no flags, no warnings seen), `./a.out`.
- Blank line inserted in the loop at 22:02 was undone by 22:15 without saving.
