Built a C11 linked list library with insertion, deletion, lookup, reversal, and memory cleanup, plus a demo, tests, and README.

All 10,000 test steps passed, including AddressSanitizer and UndefinedBehaviorSanitizer checks.

Run from this directory:

```powershell
wsl.exe -e cc -std=c11 -Wall -Wextra -Wpedantic main.c list.c -o main
wsl.exe -e ./main
```