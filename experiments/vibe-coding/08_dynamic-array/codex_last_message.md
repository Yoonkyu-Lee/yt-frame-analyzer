Built a generic C dynamic array using `malloc` and `realloc`, with automatic growth, insert/remove, access, reserve, and shrink operations. Includes a demo, README, and tests.

Build and run from this directory:

```powershell
wsl.exe -e make
wsl.exe -e ./main
wsl.exe -e make test
```

All tests passed, including AddressSanitizer and UndefinedBehaviorSanitizer checks.