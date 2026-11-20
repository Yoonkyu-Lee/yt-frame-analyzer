# Segment 3 (07:32 - 11:42) - Hayyoth, "building my own calendar in C - UNIX cal"

Setup: vim (left terminal window, title 'Terminal') editing main.c; a second terminal at top right showing a man page (ctime/mktime(3), pager prompt `:`); a third terminal in the middle (prompt `lab $ `) used to compile/run. Webcam window "Human" ignored.

## Timeline

[07:32] vim INSERT mode, main.c 22 lines, cursor 19,3 (empty line 19 inside main). Man page window (mktime/timegm/timelocal paragraph) focused; text "The functions mktime() and timegm() convert the broken-out time ..." highlighted by mouse. Status `-- INSERT --  19,3  All`.
[07:36] vim window focused again; same 22-line file, cursor 19,3 INSERT.
[07:43] Line 19 now `  mktime(date);` and a new empty line 21 (cursor 21,3); file 24 lines. `-- INSERT --  21,3`.
[07:44] Man page scrolled up to SYNOPSIS: `time_t mktime(struct tm *timeptr);`, `timegm`, `timelocal`, then DESCRIPTION (ctime/gmtime/localtime ...).
[07:48] The `time_t mktime(struct tm *timeptr);` prototype highlighted in man page.
[07:49] Vim focused; INSERT 19,10, cursor inside `mktime(date)` (parens match-highlighted). He is about to change `date` to `&date`.
[08:05] Line 19 now `  mktime(&date);`; line 21 is `  printf("first week day: %d\n", date.` (cursor 21,39; file 25 lines). Man page back at DESCRIPTION.
[08:06] Man page at the mktime paragraph ("The original values of the tm_wday and tm_yday components of the structure are ignored. The original values of the other components are not restricted to their normal ranges and will be normalized, if need[ed]").
[08:07] Man page further: "be represented, it returns -1; The difftime() function returns the difference between two calendar times, (time1 - time0), expressed in seconds. External declarations, as well as the tm structure definition, are contained in the <time.h> include file. The tm structure includes at least the following fields:" then the field list.
[08:08] Whole tm field list visible incl. `long tm_gmtoff; /* offset from UTC in seconds */`.
[08:17] Vim line 21 is `  printf("first week day: %d\n", date.tm` (cursor 21,41). `tm_wday;` highlighted in man page (he is looking up the weekday field).
[08:22] A terminal pops forward showing previous-run residue (see Shell). Vim status: `"main.c" 25L, 552B written`.
[08:28] Terminal: cc main.c / ./a.out -> `first week day: 0`.
[08:30] Terminal after running `cal`: November 2026 (today 17 inverse video).
[08:33] Vim focused: line 21 complete `  printf("first week day: %d\n", date.tm_wday);` (cursor 21,47, normal mode, `"main.c" 25L, 552B written`).
[08:38] Terminal: Nov 1 (under Su) highlighted gray by mouse, matching `first week day: 0`.
[08:47] Terminal scrolled with new run: `cal` (Nov), `cc main.c`, `./a.out` -> `first week day: 4`, `cal 10 2026` -> October 2026 starting Thursday. So month literal was changed 11 -> 10 between 08:38 and 08:47 (4 = Thursday matches).
[08:55] Vim: line 16 `  date.tm_mon = get_month_tm(10);`, `10` highlighted, cursor 16,31, `"main.c" 25L, 552B written`.
[08:59] Line 16 now `get_month_tm(9);` (partly visible behind terminal), status `"main.c" 25L, 551B written` (10 -> 9, 1 byte smaller).
[09:00] Terminal cleared; `lab $ cc ` being typed.
[09:07] Terminal: `first week day: 2`, `cal 9 2026` -> September 2026 (Sept 1 = Tuesday = 2, correct).
[09:12] Vim: line 16 `  date.tm_mon = get_month_tm(9);` (cursor 16,30, `9` highlighted), `"main.c" 25L, 551B written`.
[09:14] REFACTOR begins: vim INSERT 11,1; blank lines opened at 10-12, `int main(void) {` pushed to line 13; file 27 lines. `-- INSERT --  11,1`.
[09:25] Function header typed: line 11 `int first_weekday(int year, int month) {`, line 12 empty, line 13 `}` (matching braces highlighted), `int main(void) {` at 15; file 29 lines.
[09:27] VISUAL LINE over main's old body, lines 17-22 (`struct tm date = {0};` ... `date.tm_mday = 1;` + blank 22): `-- VISUAL LINE --  6  22,0-1`.
[09:29] After delete: `7 fewer lines`, cursor 13,1; main now has only `printf("first week day: %d\n", date.tm_wday);` (line 18) and `return 0;`; file 22 lines.
[09:30] Pasted into first_weekday: `7 more lines`, cursor 13,3. Body: blank 12, `struct tm date = {0};` at 13, then `date.tm_year = get_year_tm(2026);` `date.tm_mon = get_month_tm(9);` `date.tm_mday = 1;` blank, `mktime(&date);`, `}` at line 20; main at 22; file 29 lines.
[09:31] Blank line 12 removed: `struct tm date = {0};` directly at line 12 (cursor 12,3); file 28 lines.
[09:46] Parameterized: `date.tm_year = get_year_tm(year);`, `date.tm_mon = get_month_tm(month);`; new indented line 20 in INSERT (20,3) before `}` at 21; file 30 lines.
[09:57] `  return date.tm_wday;` now at line 20; VISUAL LINE selecting lines 11-15 (`-- VISUAL LINE --  5  15,22`).
[09:58] Selection extended to lines 11-21 (`11`, 21,2).
[09:59] `11 lines yanked` (cursor 15,1).
[10:01] Pasted copy: second `int first_weekday(int year, int month) {` at line 23 (`11 more lines`, cursor 23,1, Top).
[10:14] Scrolled to end (Bot, 40,11): second copy renamed `int days_in_month(int year, int month) {` at line 23; body still the copy (mday = 1, `return date.tm_wday;`), `}` line 33; `int main(void) {` at line 34 with no blank line before it; file 41 lines.
[10:27] days_in_month edited: line 27 `date.tm_mon = get_month_tm(month) + 1;`, line 28 `date.tm_mday = 0;`, line 32 still `return date.tm_wday;` cursor on `w` (32,18). Man page `:` prompt visible with `tm_wday` highlighted - he is checking the field.
[10:31] Same state; mouse over line 32.
[10:35] Fixed: line 32 `  return date.tm_mday;`; blank line 34 added; `int main(void) {` at 35; cursor 35,1, 90%.
[10:36] Scrolled: lines 12-42; cursor 41,1 on `return 0;`.
[10:38] main cleaned: `int main(void) {` (35), `  printf("first week day: %d\n", date.tm_wday);` (36, cursor 36,3), two blanks, `  return 0;` (39), `}` (40); file 40 lines.
[11:08] printf rewritten: line 36 `  printf("days of month 11: %d\n", days_in_month(2026, 11));` cursor 36,58. Saved: `"main.c" 40L, 842B written`.
[11:10] Middle terminal brought forward, cleared (`lab $ `).
[11:17] Terminal: cc main.c, ./a.out -> `days of month 11: 30`, then `cal` shows November 2026.
[11:20] Vim same line 36, cursor 36,58, `"main.c" 40L, 842B written`.
[11:26] Vim line 36 text is `printf("days of month 10: %d\n"...` (11 -> 10 edit); terminal still Nov cal with `30` highlighted gray.
[11:35] Terminal: `cal`, `cc main.c`, `./a.out` -> `days of month 10: 31`, `cal 10 2026` -> October 2026.
[11:42] Vim: line 36 `  printf("days of month 10: %d\n", days_in_month(2026, 10));` cursor 36,57 (`0` highlighted); status still shows stale `"main.c" 40L, 842B written`.

## File states

main.c at end of segment (11:42). Lines 1-11 are from earlier frames (view at 11:42 starts at line 12); lines 1-10 last seen at 10:14, unchanged.

```
 1| #include <stdio.h>
 2| #include <time.h>
 3|
 4| int get_year(const struct tm *date) { return date->tm_year + 1900; }
 5| int get_month(const struct tm *date) { return date->tm_mon + 1; }
 6| int get_day(const struct tm *date) { return date->tm_mday; }
 7|
 8| int get_year_tm(int year) { return year - 1900; }
 9| int get_month_tm(int month) { return month - 1; }
10|
11| int first_weekday(int year, int month) {
12|   struct tm date = {0};
13|
14|   date.tm_year = get_year_tm(year);
15|   date.tm_mon = get_month_tm(month);
16|   date.tm_mday = 1;
17|
18|   mktime(&date);
19|
20|   return date.tm_wday;
21| }
22|
23| int days_in_month(int year, int month) {
24|   struct tm date = {0};
25|
26|   date.tm_year = get_year_tm(year);
27|   date.tm_mon = get_month_tm(month) + 1;
28|   date.tm_mday = 0;
29|
30|   mktime(&date);
31|
32|   return date.tm_mday;
33| }
34|
35| int main(void) {
36|   printf("days of month 10: %d\n", days_in_month(2026, 10));
37|
38|
39|   return 0;
40| }
```
Last save seen: `"main.c" 40L, 842B written` (the 11 -> 10 edit keeps the byte count the same; not re-saved in frame).

Intermediate version 08:33-09:12 (before refactor), 25 lines:
```
 1| #include <stdio.h>
 2| #include <time.h>
 3|
 4| int get_year(const struct tm *date) { return date->tm_year + 1900; }
 5| int get_month(const struct tm *date) { return date->tm_mon + 1; }
 6| int get_day(const struct tm *date) { return date->tm_mday; }
 7|
 8| int get_year_tm(int year) { return year - 1900; }
 9| int get_month_tm(int month) { return month - 1; }
10|
11| int main(void) {
12|
13|   struct tm date = {0};
14|
15|   date.tm_year = get_year_tm(2026);
16|   date.tm_mon = get_month_tm(11);   <- 11 at 08:33, 10 at 08:55, 9 from 08:59
17|   date.tm_mday = 1;
18|
19|   mktime(&date);
20|
21|   printf("first week day: %d\n", date.tm_wday);
22|
23|
24|   return 0;
25| }
```
Saves: `"main.c" 25L, 552B written` (month 11 and 10), `"main.c" 25L, 551B written` (month 9).

Man page (macOS ctime(3)) content seen: SYNOPSIS `time_t mktime(struct tm *timeptr);` `time_t timegm(struct tm *timeptr);` `time_t timelocal(struct tm *timeptr);`; tm fields:
```
int tm_sec;     /* seconds (0 - 60) */
int tm_min;     /* minutes (0 - 59) */
int tm_hour;    /* hours (0 - 23) */
int tm_mday;    /* day of month (1 - 31) */
int tm_mon;     /* month of year (0 - 11) */
int tm_year;    /* year - 1900 */
int tm_wday;    /* day of week (Sunday = 0) */
int tm_yday;    /* day of year (0 - 365) */
int tm_isdst;   /* is summer time in effect? */
char *tm_zone;  /* abbreviation of timezone name */
long tm_gmtoff; /* offset from UTC in seconds */
```

## Shell & outputs

[08:22] previous-run residue visible:
```
lab $ cc main.c
lab $ ./a.out
year = 126
month = 10
date = 17
lab $ 10 + 1
```
[08:28]
```
lab $ cc main.c
lab $ ./a.out
first week day: 0
lab $
```
[08:30]
```
lab $ cal
   November 2026
Su Mo Tu We Th Fr Sa
 1  2  3  4  5  6  7
 8  9 10 11 12 13 14
15 16 17 18 19 20 21      (17 in inverse video)
22 23 24 25 26 27 28
29 30
```
[08:47] (month 10 run)
```
lab $ cc main.c
lab $ ./a.out
first week day: 4
lab $ cal 10 2026
    October 2026
Su Mo Tu We Th Fr Sa
             1  2  3
 4  5  6  7  8  9 10
11 12 13 14 15 16 17
18 19 20 21 22 23 24
25 26 27 28 29 30 31
```
[09:00] cleared; `lab $ cc ` being typed.
[09:07] (month 9)
```
lab $ cc main.c
lab $ ./a.out
first week day: 2
lab $ cal 9 2026
   September 2026
Su Mo Tu We Th Fr Sa
       1  2  3  4  5
 6  7  8  9 10 11 12
13 14 15 16 17 18 19
20 21 22 23 24 25 26
27 28 29 30
```
[11:17] (days_in_month 11)
```
lab $ cc main.c
lab $ ./a.out
days of month 11: 30
lab $ cal
   November 2026
(same Nov layout as above)
```
[11:35] (days_in_month 10)
```
lab $ cc main.c
lab $ ./a.out
days of month 10: 31
lab $ cal 10 2026
    October 2026
(same Oct layout as above)
```
No compiler warnings or errors visible in any frame.

## Observations

- Method: reads the macOS man page for mktime/ctime(3) beside the editor, selecting passages with the mouse (mktime paragraph, tm_wday line, tm field table) to justify each step; uses mktime as a normalizer that fills tm_wday and normalizes out-of-range fields.
- Initial bug: `mktime(date)` (struct by value) corrected to `mktime(&date)` before running.
- Verification strategy: each function validated against the system `cal` output (`cal`, `cal 10 2026`, `cal 9 2026`): weekdays 0/4/2 for 1 Nov/Oct/Sep 2026 match where `cal` puts the 1; days_in_month 30 and 31 checked against calendar layouts. Tests by editing month literal in code (11 -> 10 -> 9).
- Design: small bridging helpers between struct tm conventions (`get_year_tm` year-1900, `get_month_tm` month-1) and inverse getters. Refactored the inline experiment into `first_weekday(year, month)` via vim cut/paste (VISUAL LINE, `d`, `p`, `y` of 11 lines, paste to duplicate).
- days_in_month trick: copy of first_weekday edited to `tm_mon = get_month_tm(month) + 1; tm_mday = 0;` - day 0 of next month normalizes via mktime to the last day of this month. Copy-paste mistake (left `return date.tm_wday;`) noticed and changed to `tm_mday` at ~10:35.
- Style: 2-space indent, K&R braces, one-line getters, `int main(void)`, printf in main as throwaway test harness replaced at each step.
