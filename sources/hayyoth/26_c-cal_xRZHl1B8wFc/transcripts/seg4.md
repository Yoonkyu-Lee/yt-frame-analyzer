# Segment 4 (11:42 - 17:15) - Hayyoth, "building my own calendar in C - UNIX cal"

Layout throughout: three macOS-style Terminal windows. Left = vim editing `main.c` (window title "Terminal"). Back-right = a man page (time.h / tm struct field list) left static. Middle-front = a scratch terminal (`lab $` prompt) used first for a vim notes buffer, then for compile/run. Webcam overlay "Human" ignored.

## Timeline

[11:42] main.c, 40 lines, normal mode, cursor 36,57. Status: `"main.c" 40L, 842B written`. main is still the test harness: `int main(void) {` / `printf("days of month 10: %d\n", days_in_month(2026, 10));` / `return 0;`. Back scratch terminal shows tail of earlier output `25 26 27 28 29 30 31` and `lab $ `.
[11:49] INSERT, line 36 of main emptied (printf deleted), cursor 36,3. Still `int main(void) {`.
[12:11] main signature changed to `int main(int argc, char *argv[]) {`. New lines `int year = 0, month = 0;` (line 36) and `switch (argc) {` (line 38); INSERT at 39,5.
[12:14] Normal mode; `switch (argc) {` closed with `}` (lines 38-40), view scrolled (top = line 27). Status `39,0-1`. File now 44 lines.
[12:17] A new empty scratch Terminal window (`lab $ `) raised in front of the editor.
[12:24] In that scratch terminal vim started with no file (splash "VIM - Vi IMproved version 9.2.1200 by Bram Moolenaar et al.", "Help poor children in Uganda!"); `-- INSERT --`, `0,1`, line-number gutter on.
[12:25] First notes line being typed: `Use c` (-> `Use cal`).
[12:52] Notes buffer (INSERT, 3,46): line 1 `Use cal`; 2 blank; 3 `- cal           --> current month and current ` (typing); 4 `- cal 2026`; 5 `- cal 10 2026`; 6 `- cal 5 2026`.
[12:55] Line 3 finished: `... current year` (INSERT 3,50). Focus back in main.c (39,0-1, INSERT shown in other window).
[13:07] Editor INSERT: switch body typed: `case 1:` / `break;` / `case 2:` / `break;l` (stray `l` after `break;` at 42,14, a typo). File 47 lines.
[13:18] Typo gone; three cases (`case 1:` + blank line + `break;`, `case 2: break;`, `case 3: break;`), `}` at line 46, `return 0;` at 49, `}` at 50. Normal mode cursor 39,7.
[13:20] INSERT at 38,3: blank lines opened above `switch` (switch now at line 40). File 52 lines.
[13:28] Typed `time_t now_t = time(NULL);` (line 38) and started `s` on line 39 (39,4). switch at 41.
[13:53] Line 39 `struct tm *now = localtime(&now_t);`. `case 1:` has `year = get_year(now);` (line 43), cursor on blank line 44 (44,7). File 54 lines.
[14:07] `case 1:` has `year = get_year(now);` and `month = get_month(now);`; cursor blank line 47 in `case 2:` (47,7). File 55 lines.
[14:09] Middle scratch terminal raised: notes buffer unchanged (3,50, INSERT); he consults it.
[14:14] Editor again; scratch terminal status bar `-- (insert) VISUAL --   11   4,1` (line 4 of notes selected).
[14:20] Editor jumped to top (1,1 "Top"): header `#include <stdio.h>` / `#include <time.h>` / blank / helper one-liners (see file states).
[14:21] INSERT at 3,3: typing `#i` on a new line 3 (new include being added; text not yet visible, presumably `<stdlib.h>` for atoi). Everything below shifts by one line.
[14:26] View ~40%: `int main(int argc, char *argv[]) {` now at line 36 (was 35), `time_t now_t = time(NULL);` line 39, `struct tm *now = localtime(&now_t);` line 40, cursor 41,0-1.
[14:27] Bottom of file (56,1). `case 2:` has `year =` (line 48) in progress; brace highlight on main's `{` at line 36.
[14:45] `case 2:` line 48 `year = atoi(argv[1]);`, INSERT on blank line 49 (49,7).
[15:04] Line 49 comment typed: `// note: in production use strtol and validation year`. Cursor blank line 52 in `case 3:` (52,7).
[15:06] Scratch terminal peek: notes line 4 `- cal 2026` highlighted (visual, `11  4,1`).
[15:10] Scratch terminal `-- (insert) VISUAL --   14   5,1` (line 5 selected). Editor still INSERT at 52,7.
[15:23] `case 3:`: `month = atoi(argv[1]);` and `year = atoi(argv[` in progress (53,24).
[15:36] `case 3:` complete (`month = atoi(argv[1]);`, `year = atoi(argv[2]);`, `break;`). After the switch added `if (month) {` / blank / `} else {` / blank / `}` (lines 57-61). Cursor 65,1.
[15:37] View scrolled (top = line 47), same code.
[15:56] INSERT: line 58 `printf("year = %d, month = %d\n", year, month);`; line 60 `printf("` in progress (60,13).
[16:05] Saved: `"main.c" 65L, 1331B written`. Scratch terminal in front with notes buffer (line 5 `- cal 10 2026` visual-selected).
[16:07] Notes buffer quit attempt: `E37: No write since last change (add ! to override)` / `Press ENTER or type command to continue` (buffer scrolled so lines 2-6 visible).
[16:08] Out of vim; shell shows `lab $ e` then fresh `lab $ `.
[16:14] `cc main.c` (no diagnostics), `./a.out` -> `year = 2026, month = 11` (output highlighted with mouse).
[16:18] `./a.out 2025` -> `year = 2025`.
[16:28] `./a.out 2020` -> `year = 2020`; `./a.out 10 2026` -> `year = 2026, month = 10`.
[16:29] He mouse-selects the output from `lab $ ./a.out` through `year = 2026, month = 10`.
[16:30] Back in editor, normal mode, cursor 60,32 (end of line 60 printf), status still `"main.c" 65L, 1331B written`.
[16:31] Cursor 46,12 (on `break;`), view top = 46.
[16:32] Cursor 28,32 (on `month` in `date.tm_mon = get_month_tm(month) + 1;`), 79%.
[16:33] Cursor 26,0-1 (blank line in days_in_month), 70%.
[16:35] INSERT at 36,1: blank lines opened between end of days_in_month (line 34 `}`) and main; main now at line 38. 66%.
[16:38] Normal, cursor 50,7. The `// note: in production use strtol ...` comment is GONE (case 2 is `year = atoi(argv[1]);` then `break;`); main at line 38.
[16:54] New function above main (normal, 38,1): `void print_month(int year, int month) {` (line 36) / blank / `}` (line 38); main now at line 40.
[17:10] INSERT inside print_month: `int total_days = days_in_month` with message `-- Keyword completion (^N^P) The only match` (word completed via ctrl-n/p).
[17:14] `int total_days = days_in_month(` (args not yet typed), normal mode view at 43%, cursor 17,19.
[17:15] View 24%, cursor 10,33 (on `return` in `get_month_tm`). End of segment. No save message after the 16:05 write; print_month edits are unsaved.

## File states

### main.c as saved at 16:05 (`"main.c" 65L, 1331B written`)
Line 3 is a newly added include whose text was never visible in this segment ⟨?⟩ (probably `#include <stdlib.h>`). Lines 4-35 helper content seen at 14:20/14:21 views.
```
 1| #include <stdio.h>
 2| #include <time.h>
 3| ⟨?⟩ (typing `#i...` at 14:21; probably #include <stdlib.h>)
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
35| 
36| int main(int argc, char *argv[]) {
37|   int year = 0, month = 0;
38| 
39|   time_t now_t = time(NULL);
40|   struct tm *now = localtime(&now_t);
41| 
42|   switch (argc) {
43|     case 1:
44|       year = get_year(now);
45|       month = get_month(now);
46|       break;
47|     case 2:
48|       year = atoi(argv[1]);
49|       // note: in production use strtol and validation year
50|       break;
51|     case 3:
52|       month = atoi(argv[1]);
53|       year = atoi(argv[2]);
54|       break;
55|   }
56| 
57|   if (month) {
58|     printf("year = %d, month = %d\n", year, month);
59|   } else {
60|     printf("year = %d\n", year);
61|   }
62| 
63| 
64|   return 0;
65| }
```

### main.c at end of segment (~17:15, unsaved)
Changes vs. saved version: comment line 49 removed (seen gone at 16:38), two blank lines inserted before main then `print_month` added (so main starts at line 40 and everything after shifts +2 from the comment removal and +4 from print_month net; exact lower part not visible).
```
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
37|   int total_days = days_in_month(
38| }
39| 
40| int main(int argc, char *argv[]) {
41|   int year = 0, month = 0;
42| 
43|   time_t now_t = time(NULL);
44|   struct tm *now = localtime(&now_t);
45| 
46|   switch (argc) {
47|     case 1:
48|       year = get_year(now);
49|       month = get_month(now);
50|       break;
51|     case 2:
52|       year = atoi(argv[1]);
53|       break;
54|     case 3:
55|       month = atoi(argv[1]);
?? (lines 56-end not visible at 17:14; presumably year = atoi(argv[2]); break; } if (month) {printf...} else {printf...} return 0; })
```
Lines 1-23 unchanged from saved version (seen at 17:15: line 10 `int get_month_tm(int month) { return month - 1; }`, 12-22 first_weekday).

### Intermediate: main.c at 11:42 (saved 40L, 842B), lines 12-40
```
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
(At this stage the header was 2 includes, helpers at lines 4-6 and 8-9, first_weekday at 11.)

### Scratch notes buffer (unnamed vim buffer in the middle terminal)
```
1| Use cal
2| 
3| - cal           --> current month and current year
4| - cal 2026
5| - cal 10 2026
6| - cal 5 2026
```

## Shell & outputs

Middle terminal (`lab $` prompt):
- [12:17] empty `lab $ `; [12:24] vim launched with no file.
- [16:07] vim: `E37: No write since last change (add ! to override)` / `Press ENTER or type command to continue`.
- [16:08]
```
lab $ e
lab $ 
```
- [16:14]
```
lab $ cc main.c
lab $ ./a.out
year = 2026, month = 11
lab $ 
```
- [16:18] `lab $ ./a.out 2025` / `year = 2025`
- [16:28]
```
lab $ ./a.out 2020
year = 2020
lab $ ./a.out 10 2026
year = 2026, month = 10
lab $ 
```
No compiler warnings or errors displayed. Back-right man-page terminal is static: text about calendar times (time1 - time0, expressed in seconds), "The tm structure includes at least the following fields:" tm_sec (seconds 0-60), tm_min (0-59), tm_hour (0-23), tm_mday (day of month 1-31), tm_mon (month of year 0-11), tm_year (year - 1900), tm_wday (day of week, Sunday = 0), tm_yday (day of year 0-365), tm_isdst (summer time), tm_zone (timezone abbreviation), tm_gmtoff (offset from UTC in seconds).
Vim save messages in segment: `"main.c" 40L, 842B written` (11:42), `"main.c" 65L, 1331B written` (16:05, visible through 16:30).

## Observations

- He is cloning `cal`'s CLI and wrote the spec in a throwaway vim buffer: `cal` (current month and current year), `cal 2026`, `cal 10 2026`, `cal 5 2026`; he reads/selects those lines while writing the argc switch.
- Design: `switch (argc)`: 1 -> now (get_year/get_month on localtime), 2 -> year only via atoi(argv[1]), 3 -> month year via argv[1], argv[2]. `if (month)` distinguishes single-month vs whole-year output; month = 0 means year-only. Stubbed with debug printfs and verified with four runs (no args, 2025, 2020, `10 2026`).
- He types a comment `// note: in production use strtol and validation year` acknowledging atoi's weakness, then deletes it within ~1.5 minutes (keeps code minimal).
- Helper style: tiny one-line accessors hiding tm's 1900-based year / 0-based month; days_in_month via the mktime day-0-of-next-month trick; 2-space indent, case labels indented 2, bodies 4.
- Build order: helpers -> main test harness -> argument parsing -> print_month (started: `int total_days = days_in_month(`). Uses vim keyword completion (ctrl-n/p).
- Minor slips: stray `l` after `break;` (13:07), stray `e` typed in shell (16:08).
