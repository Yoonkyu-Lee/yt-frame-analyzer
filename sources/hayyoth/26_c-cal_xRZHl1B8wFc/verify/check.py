"""Compare ./a.out MONTH YEAR against Python's calendar for many months.
Expected format (author's): header, 3 spaces per leading blank, "%2d " per day,
newline after Saturday or the last day, then one blank line. Today's highlight
is excluded by testing months other than the current one."""
import calendar, subprocess, sys, datetime
exe = sys.argv[1]
today = datetime.date.today()
fails = n = 0
for year in list(range(1970, 2041)) + [1900, 1901, 2100, 2400]:
    for month in range(1, 13):
        if (year, month) == (today.year, today.month):
            continue
        first = (calendar.weekday(year, month, 1) + 1) % 7   # Sunday = 0
        days = calendar.monthrange(year, month)[1]
        out = "Su Mo Tu We Th Fr Sa\n" + "   " * first
        for d in range(1, days + 1):
            out += "%2d " % d
            if (first + d) % 7 == 0 or d == days:
                out += "\n"
        out += "\n"
        got = subprocess.run([exe, str(month), str(year)], capture_output=True, text=True).stdout
        n += 1
        if got != out:
            fails += 1
            if fails <= 3:
                print("MISMATCH", year, month); print(repr(got)); print(repr(out))
print(f"{n} months checked, {fails} mismatches")
