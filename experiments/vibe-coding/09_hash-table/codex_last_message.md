Built a C11 string-to-int hash table with insert/update, lookup, deletion, collision handling, and automatic resizing. Includes a demo, tests, and README.

Build and run through WSL:

```powershell
wsl.exe -e make
wsl.exe -e ./main
wsl.exe -e make test
```

All tests passed, including AddressSanitizer and UndefinedBehaviorSanitizer checks.