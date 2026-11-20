# Segment 7 (25:34 - 28:30) - final segment

## Timeline
Windows: left = vim editing `main.c` in a Terminal window; back-right = Terminal showing a man page (`struct tm` fields; static the whole segment); middle = shell Terminal (prompt `lab $ `) that appears at 26:52. Webcam "Human" ignored.

- [25:34] vim normal mode, cursor 63,34, 53%. `main.c` lines 33-63 visible: end of `get_day` (`return date.tm_mday;` / `}`), full `print_month` (lines 36-61), blank 62, line 63 `int main(int argc, char *argv[]) {` (cursor on the `{`).
- [25:35] Cursor 62,0-1 (56%); window scrolled so line 35 is on top. Line 63 `int main(int argc, char *argv[]) {`, line 64 `  int year = 0, month = 0;`.
- [25:36] `-- INSERT --` at 63,1: two blank lines (62, 63) opened above main; main now at line 65.
- [25:51] Typed `void print_year(int year, const struct tm *now) {` at line 63; line 64 blank, line 65 `}` (cursor 65,1, 53%). Matching `{` highlighted cyan.
- [25:52] Scrolled (73,1, 68%): print_year at 63-65, blank 66, `int main(...)` 67, `int year = 0, month = 0;` 68, blank 69, `time_t now_t = time(NULL);` 70, `struct tm *now = localtime(&now_t);` 71, `switch (argc) {` 73, `case 1:` 74, `year = get_year(now);` 75.
- [26:06] INSERT (66,4, 66%): print_year body starting: line 64 `  for (int month = 1; month <= 12; month++) {`, 65 blank, 66 `  }`, 67 `}`. main now at 69.
- [26:29] INSERT (66,5, 65%): line 65 `    printf("-------- %2d --------\n", month);` typed; cursor on empty line 66; 67 `  }`, 68 `}`. main at 70.
- [26:40] Normal mode (84,11, 79%): line 66 `    print_month(year, month, now);` added. switch: case 1 (`year = get_year(now); month = get_month(now); break;`), case 2 (`year = atoi(argv[1]); break;`), `case 3:` at line 84 being written.
- [26:41] (92,10, 91%) switch finished: `case 3:` -> `month = atoi(argv[1]); year = atoi(argv[2]); break;`, closing `}` line 88, blank 89, `if (month) {` 90, `print_month(year, month, now);` 91, `} else {` 92.
- [26:42] (97,11, Bot) else-branch stub: line 93 `    printf("year = %d\n", year);`, 94 `  }`, 95 and 96 blank, 97 `  return 0;`, 98 `}`.
- [26:52] Else-branch replaced: line 93 now `    print_year(year, now);`. Vim message: `"main.c" 98L, 2128B written`. A new shell Terminal window (middle) opened; typing `lab $ cc`.
- [26:57] Shell: `lab $ cc main.c`, `lab $ ./a.out` -> current-month calendar for Nov 2026 (header `Su Mo Tu We Th Fr Sa`, no title line), 17 shown in inverse video.
- [26:59] Shell output shows year-run blocks `-------- 11 --------` (17 highlighted) and `-------- 12 --------`, ends with `lab $ `. (This is the result of running a year; the command itself scrolled/typed between frames, likely `./a.out 2026`, confirmed at 27:06.)
- [27:01] Shell window title becomes `Terminal - 60x28` (window being resized/dragged); 27:02 window moved up (top at y~90); 27:04 moved left. Content: months 9-12 at bottom of output.
- [27:05]-[27:10] Scrolling up/down the output: months 1-12 of 2026 as stacked blocks. Visible history at 27:06-27:07: `lab $ cc main.c`, `lab $ ./a.out` (month), `lab $ ./a.out 2026`.
- [27:13] Another year output (months 9-12) with no highlighted day (Sep 1 under Tu... see below) ending `lab $ `.
- [27:15]-[27:20] Scrolling through that year; at 27:17 a `lab $ ./a.out 2025` line is visible right after the previous year's output (previous run's Nov block still has 17 highlighted). Output for 2025 has no highlight.
- [27:20] Cleared; ran system `cal`: `lab $ cal` -> `   November 2026` title, grid, 17 highlighted.
- [27:23]-[27:27] `lab $ cal 2026` -> real UNIX cal year layout: centered `2026` title, month names (January ... December) over 3-column rows. 17 highlighted in November.
- [27:27]-[27:28] Mouse drag-selection across the year output (grey highlight), then deselected.
- [27:29] Typed `lab $ "` (double quote).
- [27:30] zsh shows continuation prompt `dquote> ` - he uses it as a notepad.
- [27:40] `dquote> In default cal it print by three column`
- [27:43] `dquote> you do that`
- [27:44] blank `dquote>` then typing `w`.
- [27:56] `dquote> we can update print_month not print to terminal`; typing `pri`.
- [28:09] `dquote> print_month add output to buffer ,`
- [28:24] `dquote> then print_year get buffer and print output to terminal`
- [28:25]-[28:30] Mouse to top; macOS menu bar shows `Terminal Shell Edit View Window Help`, clock `Tue Nov 17 1:04 PM`. Video ends with dquote notes on screen. No code edits after the 26:52 save.

## File states

### main.c (final as of 28:30; last save `"main.c" 98L, 2128B written` at 26:52)
Lines 1-32 are not visible in this segment. Lines 33-98:
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
63| void print_year(int year, const struct tm *now) {
64|   for (int month = 1; month <= 12; month++) {
65|     printf("-------- %2d --------\n", month);
66|     print_month(year, month, now);
67|   }
68| }
69|
70| int main(int argc, char *argv[]) {
71|   int year = 0, month = 0;
72|
73|   time_t now_t = time(NULL);
74|   struct tm *now = localtime(&now_t);
75|
76|   switch (argc) {
77|     case 1:
78|       year = get_year(now);
79|       month = get_month(now);
80|       break;
81|     case 2:
82|       year = atoi(argv[1]);
83|       break;
84|     case 3:
85|       month = atoi(argv[1]);
86|       year = atoi(argv[2]);
87|       break;
88|   }
89|
90|   if (month) {
91|     print_month(year, month, now);
92|   } else {
93|     print_year(year, now);
94|   }
95|
96|
97|   return 0;
98| }
```
Uncertain: exact space count in the string on line 49 (looks like 3 spaces).

Intermediate versions:
- 25:35-25:36: main at lines 63/65 (before print_year inserted).
- 26:06: print_year body only `for (...) {` with empty body; main at line 69.
- 26:29: header printf added; main at line 70.
- 26:42: else branch was `printf("year = %d\n", year);` stub, replaced at 26:52 by `print_year(year, now);`.

Back-right terminal (static): man page text about `<time.h>` and `struct tm` fields: tm_sec (seconds 0-60), tm_min, tm_hour, tm_mday (1-31), tm_mon (0-11), tm_year (year - 1900), tm_wday (Sunday = 0), tm_yday (0-365), tm_isdst, tm_zone, tm_gmtoff (offset from UTC in seconds).

## Shell & outputs
- [26:52] `lab $ cc` (typing) -> [26:57] `lab $ cc main.c`
- [26:57] `lab $ ./a.out` ->
```
Su Mo Tu We Th Fr Sa
 1  2  3  4  5  6  7
 8  9 10 11 12 13 14
15 16 17 18 19 20 21     (17 in inverse video)
22 23 24 25 26 27 28
29 30
```
- [27:06] `lab $ ./a.out 2026` -> twelve stacked blocks, each `-------- N --------`, `Su Mo Tu We Th Fr Sa`, grid. Jan 2026: `1 2 3` under Th Fr Sa then 4-10, 11-17, 18-24, 25-31. Feb: 1-7 (Sun start) ... 22-28. Mar: 1-7 ... 29 30 31. Apr: `1 2 3 4` starting Wed ... Nov has 17 inverse; Dec starts Tu (`1 2 3 4 5`).
- [27:17] `lab $ ./a.out 2025` -> same stacked format, no highlighted day (Sep starts Mon; Oct starts Wed `1 2 3 4`; Nov starts Sat `1`; Dec starts Mon `1 2 3 4 5 6`).
- [27:20] `lab $ cal` ->
```
   November 2026
Su Mo Tu We Th Fr Sa
 1  2  3  4  5  6  7
 8  9 10 11 12 13 14
15 16 17 18 19 20 21     (17 inverse)
22 23 24 25 26 27 28
29 30
```
- [27:24] `lab $ cal 2026` -> title `2026`, then 4 rows x 3 columns: January/February/March, April/May/June, July/August/September, October/November/December, each with `Su Mo Tu We Th Fr Sa` headers; Nov 17 highlighted.
- [27:29]-[28:25] notepad typed at zsh `dquote>` prompt (not executed):
```
lab $ "
dquote> In default cal it print by three column
dquote> you do that
dquote>
dquote> we can update print_month not print to terminal
dquote> print_month add output to buffer ,
dquote> then print_year get buffer and print output to terminal
dquote>
dquote>
```
- vim: `"main.c" 98L, 2128B written` (26:52).

## Observations
- Final tool: `print_year` loops 1-12, prints `-------- %2d --------` separator and calls `print_month`; `main` uses `switch (argc)` for `cal`, `cal YEAR`, `cal MONTH YEAR`; `if (month)` prints a month else the year. `atoi` with no validation.
- Build/test loop: `cc main.c`, `./a.out`, `./a.out 2026`, `./a.out 2025`; compared against system `cal` and `cal 2026`.
- He identifies the remaining gap: real `cal` prints a year in three columns with month-name titles; his is single-column with numeric separators. He closes the video by writing the plan in the shell's `dquote>` prompt: make `print_month` write into a buffer instead of the terminal, then have `print_year` take those buffers and print them side by side - left as an exercise ("you do that"). No code written for it.
- No compiler errors or warnings in this segment. Style: 2-space indent, K&R braces, `case` indented inside `switch`, stub-then-replace development (the `printf("year = %d\n", year)` stub before wiring `print_year`).
