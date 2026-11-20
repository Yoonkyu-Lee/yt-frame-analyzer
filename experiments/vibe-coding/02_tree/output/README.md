# ctree

A small Unix `tree`-style directory viewer written in C11 for Linux/WSL.
It sorts entries by name, draws Unicode branches, displays symbolic link
targets, and reports the number of displayed directories and files.
It has no external C dependencies.

From PowerShell in this directory:

```powershell
wsl.exe -e cc -std=c11 -O2 -Wall -Wextra -Wpedantic main.c -o ctree
wsl.exe -e ./ctree
wsl.exe -e ./ctree -a -L 2 --dirsfirst .
wsl.exe -e ./ctree --help
```

Or build and run the integration tests using WSL's `make` and Python 3:

```powershell
wsl.exe -e make test
```

Examples inside WSL:

```sh
./ctree --ascii -L 2 /path/to/project
./ctree -d --dirsfirst .
./ctree -a -I '*.o' .
./ctree -l -F first-directory second-directory
```

Hidden entries are omitted unless `-a` is used. `-I` accepts one shell glob
matched against each basename; excluded directories are not traversed.
Depth zero prints only the root, depth one includes its immediate children.
Sorting uses byte order and is independent of the current locale.

Symbolic links are not followed by default. `-l` follows them, including a
symlink supplied as a root, and marks ancestor loops with `[cycle]`.
Without `-l`, directory symlinks count as files and are excluded by `-d`.
Counts include only displayed entries and exclude directory roots.
Explicit file roots count as files. Known cycles are informational;
unreadable paths, broken links when following, and other filesystem errors
are printed inline and produce a nonzero exit status. Other roots are
still processed. Filename control bytes and backslashes are escaped as
`\xNN` to keep the output readable; UTF-8 names are preserved.

Use `--` before paths beginning with a hyphen. Full-path output (`-f`)
preserves the root path as supplied; it does not resolve it to an absolute path.
