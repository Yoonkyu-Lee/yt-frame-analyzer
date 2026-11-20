# Segment 2 (03:00 - 07:32) - xRZHl1B8wFc "building my own calendar in C - UNIX cal"

Layout: macOS windows, all titled "Terminal". Left = vim editing main.c (line numbers on). Right-top = a Terminal showing a man page (initially the tm struct section of `man 3 localtime`). A third Terminal (middle) appears briefly for compile/run and a Lua REPL. Webcam "Human" ignored.

## Timeline

[03:00] vim, main.c, NORMAL, cursor 9,36. 13 lines: includes stdio.h, time.h; `int main(void) {`; `time_t now_t = time(NULL);`; `struct tm *now = localtime(&now_t);`; three printfs. Line 10 is still the copy-pasted `printf("year = %d\n", now->tm_year);` (mouse I-beam on it). Right-top Terminal shows man text "definition, are contained in the <time.h> include file. The tm structure includes at least the following fields:" with the row `int tm_mon;  /* month of year (0 - 11) */` highlighted.
[03:12] Line 10 now `printf("date = %d\n", now->tm_mday);`. Status: `"main.c" 13L, 255B written`. Third Terminal opened in front:
```
lab $ cc main.c
lab $ ./a.out
year = 126
lab $ lua
Lua 5.5.1  Copyright (C) 1994-2026 Lua.org, PUC-Rio
>  126 + 1900
2026
> 
```
(Lua REPL used as a calculator: tm_year 126 -> 2026. The ./a.out output shown is an earlier run with only the year line.)
[03:13] Lua REPL cleared, only `> ` prompt.
[03:21] Third Terminal after re-run:
```
lab $ cc main.c
lab $ ./a.out
year = 126
month = 10
date = 17
lab $ 
```
"month = 10" row highlighted by mouse selection (today = 17 Nov 2026; tm_mon 0-based).
[03:24] Same; `lab $ 10` typed at the prompt (scratch).
[03:29] Third Terminal buried; vim in front, `"main.c" 13L, 255B written`, cursor 10,36.
[03:31] vim INSERT, cursor 4,1. Blank lines opened between includes and main: file now 15 lines, `int main(void) {` at line 6, lines 4-5 empty.
[03:58] INSERT, cursor 4,68. First helper typed (closing brace wrapped to next row because of window width, matching-brace highlight): `int get_year(const struct tm *date) { return date->tm_year + 1900;` then `}`. main at line 6.
[03:59] Same with the `}` visible on its own row; file 15-16 lines.
[04:01] vim window widened; function on one row: `int get_year(const struct tm *date) { return date->tm_year + 1900; }` (line 4), cursor 5,1 INSERT; main at line 7.
[04:04] NORMAL, cursor 5,1: line 4 duplicated into line 5 (identical get_year); 17 lines, main at line 8.
[04:05] Third duplicate: lines 4,5,6 all `int get_year(...) { return date->tm_year + 1900; }`; cursor 5,2; 18 lines, main at line 9.
[04:18] INSERT, cursor 6,9. Edits:
  4: `int get_year(const struct tm *date) { return date->tm_year + 1900; }`
  5: `int get_month(const struct tm *date) { return date->tm_mon + 1; }`
  6: `int get_(const struct tm *date) { return date->tm_year + 1900; }` (name mid-typing)
[04:20] NORMAL, cursor 6,17. Line 6: `int get_date(const struct tm *date) { return date->tm_year + 1900; }`.
[04:44] VISUAL (5 chars), cursor 6,62. Line 6: `int get_day(const struct tm *date) { return date->tm_mday + 1900; }` (renamed get_date -> get_day; body changed to tm_mday; selection covers ` + 19` of `+ 1900` to delete it).
[04:47] Line 6 final: `int get_day(const struct tm *date) { return date->tm_mday; }`. Status: `"main.c" 18L, 453B written`. A Spotlight-style search box overlays the screen containing `d`.
[04:48] Windows shrink (Mission Control-like animation); Dictionary app opens, query "reverse" (Vietnamese - English tab, "24 found"): "reverse | ri'və:s |", tính từ "đảo, nghịch, ngược lại, trái lại"; examples "in the reverse direction / ngược chiều", "the reverse side / mặt trái", "reverse current / dòng nước ngược"; "in/into reverse order / từ cuối trở lại đầu..."; danh từ "1 (the reverse) điều trái ngược", "it is quite the reverse / hoàn toàn ngược lại", "2 bề trái, mặt trái...". Sidebar: reverse, reverse annuity..., reverse-charge, reverse charges, reverse engineer..., reverse gear, ... reverse repo, reverse split, reverse swing, reverse takeover, reverse transcrip...
[04:49] Dictionary fully drawn, sidebar visible.
[04:50] Search field changed to `d`.
[04:51] Search "date" (20 found): "date | deit |", danh từ: "1 (thực vật học) cây chà là (cũng gọi là date palm)", "2 (thực vật học) quả chà là; trái chà là", "3 ngày tháng; niên hiệu" with examples "what's the date today? / hôm nay ngày bao nhiêu?", "today's date is the sixth of March / hôm nay là ngày 6 tháng 3". Sidebar: date, datable, date-cancel, Date Line, date of record, date palm, date rape, date-rape, date rape drug, date stamp, dateable, dated, dated date, datedly, datedness, Datel, dateless, dateline.
[04:52-04:55] Dictionary window moved and scrolled: "has the date of the meeting been fixed?", "when was the date of the Battle of Waterloo?", "there's no date on this cheque", "that newspaper bears the date 6 March 1999; ...", "date of birth / ngày tháng năm sinh", "4 (thương nghiệp) kỳ; kỳ hạn: pay at fixed dates / at long date / at short date". (Evidently confirming "date" has several meanings -> he chose get_day.)
[04:56] Dictionary closed; empty desktop with window thumbnails at left.
[04:57] Back in vim NORMAL, cursor 13,23, `"main.c" 18L, 453B written`. Lines 4-6 get_year/get_month/get_day as above; main body with 3 printfs unchanged (main at line 9).
[05:02] INSERT, cursor 8,1; lines 7-9 blank, main at line 10 (19 lines).
[05:11] NORMAL, cursor 4,1; back to 18 lines (extra blank removed), main at line 9.
[05:12] VISUAL LINE (2 lines) over lines 4-5 (get_year, get_month); cursor 5,1.
[05:13] NORMAL cursor 7,0-1, 18 lines.
[05:14] INSERT cursor 8,1; 19 lines, main at line 10.
[05:29] INSERT cursor 9,2. New helpers: line 8 `int get_year_tm(int year) { return year - 1900; }`, line 9 `i` (typing). main at line 11, 20 lines.
[05:48] VISUAL LINE (2 lines) on main lines 12-13 (`time_t now_t = time(NULL);` and `struct tm *now = localtime(&now_t);`), cursor 13,38. Line 9 now `int get_month_tm(int month) { return month - 1; }`.
[05:49] NORMAL cursor 12,0-1, message `6 fewer lines`. main reduced to:
```
11| int main(void) {
12|
13|   return 0;
14| }
```
[06:31] INSERT cursor 15,24. main:
```
11| int main(void) {
12|
13|   struct tm date = {0};
14|
15|   date.tm_year = get_ye
16|
17|   return 0;
18| }
```
[07:05] INSERT cursor 19,3, 22 lines:
```
13|   struct tm date = {0};
15|   date.tm_year = get_year_tm(2026);
16|   date.tm_mon = get_month_tm(11);
17|   date.tm_mday = 1;
19|   (cursor)
21|   return 0;
```
[07:07] Focus on right-top Terminal (pager, `:` prompt) showing tm field list again (tm_sec 0-60, tm_min 0-59, tm_hour 0-23, tm_mday 1-31, tm_mon 0-11 highlighted, tm_year year - 1900, tm_wday Sunday = 0, tm_yday 0-365, tm_isdst, `char *tm_zone; /* abbreviation of timezone name */`, `long tm_gmtoff; /* offset from UTC in seconds */`; "The field tm_isdst is non-zero if summer (i.e., Daylight Saving) time is in effect." "The field tm_gmtoff is the offset (in seconds) of the ...").
[07:08] Right-top Terminal at shell: `lab $ man 3 time` / `lab $ man 3 localtime` / `lab $ ` (man quit).
[07:11] Man page reopened: `CTIME(3)   Library Functions Manual   CTIME(3)`. NAME: asctime, asctime_r, ctime, ctime_r, difftime, gmtime, gmtime_r, localtime, localtime_r, mktime, timegm, timelocal - transform binary date and time values. LIBRARY: Standard C Library (libc, -lc). SYNOPSIS: `#include <time.h>`; `extern char *tzname[2];`; `char * asctime(const struct tm *timeptr);`; `char * asctime_r(const struct tm *restrict timeptr, char *restrict buf);`.
[07:13] Scrolled synopsis: `char * ctime(const time_t *clock);`, `ctime_r(const time_t *clock, char *buf);`, `double difftime(time_t time1, time_t time0);`, `struct tm * gmtime(const time_t *clock);`, `gmtime_r(const time_t *clock, struct tm *result);`, `struct tm * localtime(const time_t *clock);`, `localtime_r(const time_t *clock, struct tm *result);`.
[07:16] Scrolled: `time_t mktime(struct tm *timeptr);`, `time_t timegm(struct tm *timeptr);`, `time_t timelocal(struct tm *timeptr);`. DESCRIPTION: "The functions ctime(), gmtime(), and localtime() all take as an argument a time value representing the time in seconds since the Epoch (00:00:00 UTC, January 1, 1970; see time(3)). When encountering an error, these functions return NULL and set errno to an appropriate value. The function localtime() converts the time value pointed at by clock. It returns a pointer to a "struct tm" (described below), which contains the".
[07:20] Same view; mktime/timelocal prototype block highlighted by mouse.
[07:22] Scrolled: "broken-out time information for the value after adjusting for the current time zone (and any other factors such as Daylight Saving Time). Time zone adjustments are performed as specified by the TZ environment variable (see tzset(3)). The function localtime() uses tzset(3) to initialize time conversion information, if tzset(3) has not already been called by the process. After filling in the tm structure, localtime() sets the tm_isdst'th element of tzname to a pointer to an ASCII string containing the time zone abbreviation to be used with localtime()'s return value. The function gmtime() also converts the time value, but makes no time zone adjustment. It returns a pointer to a tm structure (described below). The ctime() function adjusts the time value for the current time zone, in the same manner as localtime(). It returns a pointer to a 26-character string of the".
[07:24] Scrolled: "form: `Thu Nov 24 18:22:48 1986\n\0` All of the fields have constant width. The ctime_r() function provides the same functionality as ctime(), except that the caller must provide the output buffer buf (which must be at least 26 characters long) to store the result. The localtime_r() and gmtime_r() functions provide the same functionality as localtime() and gmtime(), respectively, except the caller must provide the output buffer result. The asctime() function converts the broken-out time in the structure tm (pointed at by *timeptr) to the form shown in the example above. The asctime_r() function provides the same functionality as asctime(), except that the caller".
[07:25] Scrolled: "provides the output buffer buf (which must be at least 26 characters long) to store the result. The functions mktime() and timegm() convert the broken-out time (in the structure pointed to by *timeptr) into a time value with the same encoding as that of the values returned by the time(3) function (that is, seconds from the Epoch, UTC). The mktime() function interprets the input structure according to the current timezone setting (see tzset(3)). The timegm() function interprets the input structure as representing Universal Coordinated Time (UTC). The timelocal() function is like timegm(), but uses the current timezone setting. This makes timelocal() equivalent to mktime(). The original values of the tm_wday and tm_yday components of the structure are ignored. The original values of the other components are not restricted to their normal ranges and will be normalized, if need".
[07:32] Same position; first paragraph (mktime/timegm) highlighted by mouse. vim (left) still INSERT at 19,3 (inactive window). No save visible after 04:47.

## File states

### main.c at end of segment (~07:32), vim INSERT, cursor 19,3, 22 lines (unsaved; last visible save `"main.c" 18L, 453B written`)
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
16|   date.tm_mon = get_month_tm(11);
17|   date.tm_mday = 1;
18|
19|   ⟨cursor on empty/indented line⟩
20|
21|   return 0;
22| }
```
(Line 4 is clipped at the window edge in the last frames; the rest is as shown in earlier frames.)

### Intermediate: main.c at 04:47 (saved, 18L, 453B)
```
 1| #include <stdio.h>
 2| #include <time.h>
 3|
 4| int get_year(const struct tm *date) { return date->tm_year + 1900; }
 5| int get_month(const struct tm *date) { return date->tm_mon + 1; }
 6| int get_day(const struct tm *date) { return date->tm_mday; }
 7|
 8|
 9| int main(void) {
10|   time_t now_t = time(NULL);
11|   struct tm *now = localtime(&now_t);
12|
13|   printf("year = %d\n", now->tm_year);
14|   printf("month = %d\n", now->tm_mon);
15|   printf("date = %d\n", now->tm_mday);
16|
17|   return 0;
18| }
```

### Earlier: main.c at 03:12 (saved, 13L, 255B)
```
 1| #include <stdio.h>
 2| #include <time.h>
 3|
 4| int main(void) {
 5|   time_t now_t = time(NULL);
 6|   struct tm *now = localtime(&now_t);
 7|
 8|   printf("year = %d\n", now->tm_year);
 9|   printf("month = %d\n", now->tm_mon);
10|   printf("date = %d\n", now->tm_mday);
11|
12|   return 0;
13| }
```

## Shell & outputs

- [03:12] `lab $ cc main.c`; `lab $ ./a.out` -> `year = 126`; `lab $ lua` -> `Lua 5.5.1  Copyright (C) 1994-2026 Lua.org, PUC-Rio`; `>  126 + 1900` -> `2026`.
- [03:21] `lab $ cc main.c`; `lab $ ./a.out` -> `year = 126` / `month = 10` / `date = 17`.
- [03:24] typed `10` at prompt (scratch).
- [04:47-04:55] macOS Spotlight -> Dictionary app (Vietnamese - English): "reverse" (24 found), then "date" (20 found).
- [07:08] Right Terminal history: `lab $ man 3 time`, `lab $ man 3 localtime`.
- [07:11-07:32] CTIME(3) man page read (text in timeline).
- Vim messages: `"main.c" 13L, 255B written` (03:12-03:31); `"main.c" 18L, 453B written` (04:47, 04:57); `6 fewer lines` (05:49).

## Observations

- Build order: probe the raw `struct tm` (printf year/month/mday from localtime), verify against known today (126+1900 = 2026, mon 10 = Nov, mday 17), then wrap the tm-convention arithmetic into tiny named helpers.
- Naming: getters `get_year/get_month/get_day` take `const struct tm *` and return human values; inverses `get_year_tm`, `get_month_tm` (suffix `_tm` = into tm convention). He picked `day` over `date` after checking the dictionary (date also means the fruit / appointment).
- Style: 2-space indent, one-line function bodies with spaces inside braces, `const` on read-only pointers, `{0}` zero-init of `struct tm`; copy-paste-then-edit workflow in vim (yank/paste lines, rename).
- References: Lua REPL as calculator; `man 3 time`, `man 3 localtime` (CTIME(3), BSD/macOS style); he highlighted the mktime/timegm paragraph (tm_wday and tm_yday ignored by mktime, other fields normalized) - suggests the plan to build a `struct tm` for the 1st of a month and call mktime() to get the weekday.
- He deleted the original now/localtime/printf test code (`6 fewer lines`) once the helpers existed, and starts constructing an arbitrary date (2026, month 11 -> tm_mon 10, day 1) instead of "now".
