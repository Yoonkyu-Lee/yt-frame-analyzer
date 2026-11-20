# Segment 1 (0:00 - 3:00) - Hayyoth, "building my own calendar in C - UNIX cal"

Setup: macOS Terminal.app windows (all titled "Terminal"), prompt `lab $ `, vim with line numbers + syntax colors, C compiled with `cc` (Apple clang). Webcam overlay "Human" ignored.

## Timeline

- [00:00] Single empty Terminal window, prompt `lab $ `.
- [00:02] Runs `cal`. Output: `   November 2026` / `Su Mo Tu We Th Fr Sa` / month grid; 17 (Tue) inverse-video highlighted. Month starts Sunday 1, ends 30.
- [00:07] Screen shows a full-year 2026 calendar in 3 columns x 4 rows (Jan-Mar scrolled off; April/May/June, July/August/September, October/November/December visible), 17 Nov highlighted. The command that produced it (likely `cal -y` or `cal 2026`) is scrolled off / not visible.
- [00:15] Runs `cal 10 2026` -> `    October 2026` single month (Oct 1 = Thursday, 31 days).
- [00:18] Runs `cal 11 2026` -> November 2026, 17 highlighted.
- [00:26] Window now shows vim editing new file: status line `"main.c" [New]`, cursor 4,2. Buffer is the C skeleton (File states). The vim launch command is not visible.
- [00:29] Vim in `-- INSERT --` (4,3). A second Terminal window opened (empty `lab $ `) over the right half.
- [00:31] Second terminal dragged to upper right.
- [00:37] In second terminal a man page is open: TIME(3) "Library Functions Manual". NAME `time - get time of day`; LIBRARY `Standard C Library (libc, -lc)`; SYNOPSIS `#include <time.h>` / `time_t` / `time(time_t *tloc);`; DESCRIPTION "The time() function returns the value of time in seconds since 0 hours, 0 minutes, 0 seconds, January 1, 1970, Coordinated Universal Time, without including leap seconds. If an error occurs, time() returns the value (time_t)-1." (Command `man 3 time` seen later in the 01:32 frame.)
- [00:44-00:45] Scrolls: "The return value is also stored in *tloc, provided that tloc is non-null." then ERRORS: "The time() function may fail for any of the reasons described in gettimeofday(2)."
- [00:50] Back to vim window (insert mode, 4,3), man window behind.
- [01:09] Vim code typed (normal mode, cursor 7,0-1, no save message yet): `time_t now_t = time(NULL);` and `printf("%ld\n", now_t);` (File states v1).
- [01:11] Saved: `"main.c" [New] 9L, 108B written`. A third Terminal window opened (empty `lab $ `).
- [01:14] Runs `cc main.c` -> 2 errors (verbatim in Shell & outputs).
- [01:17] Vim in front (normal mode 7,0-1), still the 9-line file.
- [01:19] Insert mode: new line 2 being typed `#incl` (cursor 2,6); file now 10 lines.
- [01:23] Saved: `"main.c" 10L, 126B written` (cursor 2,17); line 2 is `#include <time.h>`. Terminal behind still shows old error output.
- [01:24] `cc main.c` re-run: no output (success, prompt returns).
- [01:32] Runs `./a.out` -> `1794893823` (output text appears highlighted/selected). In the right terminal: `lab $ man 3 time` then `lab $ m` being typed (next man command).
- [01:36] Man page CTIME(3) opened: NAME lists `asctime, asctime_r, ctime, ctime_r, difftime, gmtime, gmtime_r, localtime, localtime_r, mktime, timegm, timelocal - transform binary date and time values`. SYNOPSIS: `#include <time.h>`, `extern char *tzname[2];`, `char * asctime(const struct tm *timeptr);`, `char * asctime_r(const struct tm *restrict timeptr, char *restrict buf);`.
- [01:38] Scrolled SYNOPSIS: `char * ctime(const time_t *clock);`, `char * ctime_r(const time_t *clock, char *buf);`, `double difftime(time_t time1, time_t time0);`, `struct tm * gmtime(const time_t *clock);`, `struct tm * gmtime_r(const time_t *clock, struct tm *result);`, `struct tm * localtime(const time_t *clock);`, `struct tm * localtime_r(const time_t *clock, struct tm *result);`
- [01:42] The `struct tm * localtime(const time_t *clock);` entry is mouse-highlighted.
- [01:47-01:49] Continues scrolling: `time_t mktime(struct tm *timeptr);`, `time_t timegm(struct tm *timeptr);`, `time_t timelocal(struct tm *timeptr);`, then DESCRIPTION heading.
- [01:50] DESCRIPTION text (partial): "...pointed at by clock. It returns a pointer to a "struct tm" (described below), which contains the broken-out time information for the value after adjusting for the current time zone (and any other factors such as Daylight Saving Time). Time zone adjustments are performed as specified by the TZ environment variable (see tzset(3)). The function localtime() uses tzset(3) to initialize time conversion information, if tzset(3) has not already been called by the process. After filling in the tm structure, localtime() sets the tm_isdst'th element of tzname to a pointer to an ASCII string containing the time zone abbreviation to be used with localtime()'s return value. The function gmtime() also converts the time value, but makes no time zone adjustment. It returns a pointer to a tm structure (described below). The ctime() function adjusts the time value for the..."
- [01:51] More: "The asctime_r() function provides the same functionality as asctime(), except that the caller provides the output buffer buf (which must be at least 26 characters long) to store the result. The functions mktime() and timegm() convert the broken-out time (in the structure pointed to by *timeptr) into a time value with the same encoding as that of the values returned by the time(3) function (that is, seconds from the Epoch, UTC). The mktime() function interprets the input structure according to the current timezone setting (see tzset(3)). The timegm() function interprets the input structure as representing Universal Coordinated Time (UTC). The timelocal() function is like timegm(), but uses the current timezone setting. This makes timelocal() equivalent to mktime(). The original values of the tm_wday and tm_yday components of the structure are ignored. The original..."
- [01:52] "...determined. The mktime() function returns the specified calendar time; if the calendar time cannot be represented, it returns -1; The difftime() function returns the difference between two calendar times, (time1 - time0), expressed in seconds. External declarations, as well as the tm structure definition, are contained in the <time.h> include file. The tm structure includes at least the following fields:" followed by the struct (see Shell & outputs).
- [01:54] struct tm field list fully visible.
- [01:56] Vim window brought to front (cursor 2,17, 10 lines, unchanged); man window behind.
- [01:57] Vim terminal being resized: title bar shows `Terminal — 71×32`; window narrower (status bar shows `2,17  All`).
- [01:58] Vim window dragged up/left.
- [02:00] Dragged further up-left (final position used for the rest of the segment).
- [02:14] Insert mode (6,3): a blank line inserted after line 5; file 11 lines (line 6 empty with cursor, 7 empty, 8 printf, 9 empty, 10 return, 11 `}`).
- [02:26] Line 6 now `  struct tm *now = localtime(&now_t);`; the old `printf("%ld\n", now_t);` line was deleted; normal mode, cursor 7,0-1; file 10 lines (lines 7 and 8 blank, 9 `return 0;`, 10 `}`). Not saved yet.
- [02:27] printf line retyped as `  printf("year = %d\n", now->tm_year);` on line 8; file 11 lines. Saved: `"main.c" 11L, 177B written` (cursor 8,38).
- [02:29] A fourth Terminal window (empty `lab $ `) overlaps at bottom center.
- [02:37] In that terminal: `cc main.c`, `./a.out` -> `year = 126`; then `lua` -> `Lua 5.5.1  Copyright (C) 1994-2026 Lua.org, PUC-Rio` with Lua prompt `> ` (REPL as scratch calculator; nothing typed yet in frame).
- [02:45] Vim back in front, cursor 8,38.
- [02:46] Normal mode: line 8 duplicated -> lines 8 and 9 both `printf("year = %d\n", now->tm_year);`, cursor 9,3, 12 lines.
- [02:49] Insert mode: line 9 being edited to `printf("month|= ...` i.e. `printf("month = %d\n", now->tm_year);` (cursor 9,16); line 10 is now a third `printf("year = %d\n", now->tm_year);` (13 lines total).
- [02:56] Line 9 now `printf("month = %d\n", now->tm_mon);` (cursor 9,36, normal mode); the man window (struct tm fields) is in front.
- [03:00] Vim in front again; man page row `int tm_mon;  /* month of year (0 - 11) */` is mouse-highlighted; vim cursor on the `n` of `tm_mon` (9,36).

## File states

### main.c - skeleton at 00:26 (`"main.c" [New]`, cursor 4,2)
```
1| #include <stdio.h>
2|
3| int main(void) {
4|
5|   return 0;
6| }
```
(Indent is 2 spaces; line 4 is the empty line where cursor sits at col 2, i.e. auto-indented.)

### main.c - 01:09-01:11 (saved `"main.c" [New] 9L, 108B written`)
```
1| #include <stdio.h>
2|
3| int main(void) {
4|   time_t now_t = time(NULL);
5|
6|   printf("%ld\n", now_t);
7|
8|   return 0;
9| }
```

### main.c - 01:23 (saved `"main.c" 10L, 126B written`)
```
 1| #include <stdio.h>
 2| #include <time.h>
 3|
 4| int main(void) {
 5|   time_t now_t = time(NULL);
 6|
 7|   printf("%ld\n", now_t);
 8|
 9|   return 0;
10| }
```

### main.c - 02:27 (saved `"main.c" 11L, 177B written`)
```
 1| #include <stdio.h>
 2| #include <time.h>
 3|
 4| int main(void) {
 5|   time_t now_t = time(NULL);
 6|   struct tm *now = localtime(&now_t);
 7|
 8|   printf("year = %d\n", now->tm_year);
 9|
10|   return 0;
11| }
```

### main.c - 03:00 (END of segment; unsaved edits since 11L, 177B save)
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
10|   printf("year = %d\n", now->tm_year);
11|
12|   return 0;
13| }
```
(Third printf on line 10 is a leftover duplicate, probably to be edited next.)

## Shell & outputs

- [00:02] `lab $ cal`
  ```
     November 2026
  Su Mo Tu We Th Fr Sa
   1  2  3  4  5  6  7
   8  9 10 11 12 13 14
  15 16 [17] 18 19 20 21     (17 inverse-video)
  22 23 24 25 26 27 28
  29 30
  ```
- [00:07] Year view of 2026 (3 months per row). Visible: April (1st on Wed), May (1st Fri), June (1st Mon), July (1st Wed), August (1st Sat), September (1st Tue), October (1st Thu), November (1st Sun), December (1st Tue); each month header centered, 17 Nov highlighted. Command not visible.
- [00:15] `lab $ cal 10 2026`
  ```
      October 2026
  Su Mo Tu We Th Fr Sa
                  1  2  3
   4  5  6  7  8  9 10
  11 12 13 14 15 16 17
  18 19 20 21 22 23 24
  25 26 27 28 29 30 31
  ```
- [00:18] `lab $ cal 11 2026` -> November 2026 as above, 17 highlighted.
- [00:37] man page TIME(3) (see Timeline).
- [01:14] `lab $ cc main.c`
  ```
  main.c:4:3: error: unknown type name 'time_t'; did you mean 'size_t'?
      4 |   time_t now_t = time(NULL);
        |   ^~~~~~
        |   size_t
  /Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/usr/include/sys/_types/_size_t.h:50:32: note: 'size_t' declared here
     50 | typedef __darwin_size_t        size_t;
        |                                ^
  main.c:4:18: error: call to undeclared function 'time'; ISO C99 and later do not support implicit function declarations [-Wimplicit-function-declaration]
      4 |   time_t now_t = time(NULL);
        |                  ^
  2 errors generated.
  ```
- [01:24] `lab $ cc main.c` -> (no output)
- [01:32] `lab $ ./a.out` -> `1794893823`
- [01:32] Right terminal: `lab $ man 3 time` ... `lab $ m` (next man command being typed; page that opens at 01:36 is CTIME(3)).
- [01:54] struct tm fields in the CTIME(3) page:
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
  "The field tm_isdst is non-zero if summer (i.e., Daylight Saving) time is in effect. The field tm_gmtoff is the offset (in seconds) of the ..."
- [02:37] `lab $ cc main.c`, `lab $ ./a.out` -> `year = 126`; `lab $ lua` -> `Lua 5.5.1  Copyright (C) 1994-2026 Lua.org, PUC-Rio` / `> `
- Vim save messages (checksums): `"main.c" [New] 9L, 108B written`; `"main.c" 10L, 126B written`; `"main.c" 11L, 177B written`.

## Observations

- Opens by demonstrating the target: macOS/BSD `cal` (current month with today highlighted, `cal MM YYYY`, full-year 3-column layout). Goal is to clone it.
- Workflow: vim for editing, several Terminal windows (editor / shell / man pages) arranged by hand; reads man pages (`time(3)`, then the ctime(3) family) rather than web docs to learn `time()`, `localtime()` and `struct tm`.
- Incremental, test-as-you-go: first print the raw epoch (`%ld` of `time_t`), hits compile errors (forgot `#include <time.h>`), fixes, then moves to `localtime` and `struct tm`; prints `tm_year` (126 = years since 1900) and starts probing `tm_mon` (0-11, with the man page's note highlighted).
- Naming: `now_t` for the `time_t`, `now` for the `struct tm *`. 2-space indent. Duplicates a printf line with yank/paste to probe fields. Uses Lua REPL as scratch calculator.
- Builds on macOS (Apple clang, MacOSX.sdk path in errors), plain `cc main.c` -> `a.out`.
