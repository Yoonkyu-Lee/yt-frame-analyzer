Built `touch`, `cat`, `mkdir`, `mv`, `rm`, and `find` in C, with a Makefile, README, and integration tests.

```powershell
wsl.exe -e make
wsl.exe -e make test
wsl.exe -e ./find . -name '*.c'
wsl.exe -e ./touch --help
```

Tests: 15 passed; one permission test skipped due to Windows mount limitations.