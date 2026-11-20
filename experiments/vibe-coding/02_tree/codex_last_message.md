Built `ctree` in C with sorted branches, filters, depth limits, ASCII output, symlink cycle detection, and entry counts. All 11 integration tests passed.

Build and run from PowerShell:

```powershell
wsl.exe -e make
wsl.exe -e ./ctree
wsl.exe -e ./ctree --help
```

Run tests with `wsl.exe -e make test`.