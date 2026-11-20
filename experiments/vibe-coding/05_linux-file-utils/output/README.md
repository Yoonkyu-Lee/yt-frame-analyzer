# Linux file utilities in C

Six standalone Linux programs, implemented with POSIX system calls and the C
standard library. They cover common workflows, rather than every GNU coreutils
option. No external commands are invoked by the utilities.

Build and test from PowerShell with WSL:

```powershell
wsl.exe -e make
wsl.exe -e make test
wsl.exe -e ./touch example.txt
wsl.exe -e ./cat -n example.txt
wsl.exe -e ./mkdir -p demo/subdir
wsl.exe -e ./mv example.txt demo/subdir/
wsl.exe -e ./find demo -type f -name '*.txt'
wsl.exe -e ./rm -r demo
```

Use `./` to select these programs instead of the system versions. Each supports
`--help`; file operands starting with `-` can be passed after `--` (for `find`,
use a path such as `./-name`). Build directly inside Linux with `make` too.
The tests require Python 3 and create temporary fixtures inside this directory.

| Program | Supported features |
| --- | --- |
| `touch` | Create files; update times; `-a`, `-m`, `-c`, `-h`; copy timestamps with `-r FILE`; local time with `-t YYYYMMDDhhmm[.ss]` |
| `cat` | Multiple files, stdin (`-`), binary data; `-n`, `-b`, `-s`, `-v`, `-E`, `-T`, `-A`, `-e`, `-t`, `-u` |
| `mkdir` | Multiple paths; `-p` parents, `-m OCTAL_MODE`, `-v` |
| `mv` | Rename files/directories; multiple sources into a directory; `-f`, `-i`, `-n`, `-v`, `-T`; cross-filesystem fallback |
| `rm` | Multiple paths; `-r`/`-R`, `-f`, `-i`, `-d`, `-v`; refuses `/`, `.` and `..` operands |
| `find` | Multiple starting paths (default `.`); physical symlink handling; glob tests `-name`, `-iname`, `-path`; `-type`, `-size`, `-mtime`, `-mmin`, `-empty`, `-newer`, `-true`, `-false`; `-mindepth`, `-maxdepth`, `-depth`; `-print`, `-print0`, `-prune`, `-delete`; `!`, parentheses, AND, OR |

For `find`, adjacent expressions imply AND, which binds more tightly than OR.
Expressions short-circuit. Unless an explicit print or delete action appears,
matching paths are printed. `-delete` implies depth-first traversal, so `-prune`
has no effect when combined with it. `-size` rounds up to units of 512 bytes
(default or `b`), bytes (`c`), 2-byte words (`w`), KiB (`k`), MiB (`M`), or GiB
(`G`). Numeric tests accept `+N` (greater), `-N` (less), or `N` (equal); age
tests count completed days or minutes from program start.

```sh
./find . -type f \( -name '*.c' -o -name '*.h' \)
./find . -name .git -prune -o -type f -print
./find demo -mindepth 1 -type f -empty -delete
```

`mv` first uses atomic `rename`. On a filesystem boundary, it stages a copy
beside the destination, renames it into place, then removes the source. This
supports regular files, directories, and symlinks and preserves their modes and
timestamps (symlinks preserve timestamps only). Ownership, extended attributes,
ACLs, sparse layout, and hard-link relationships are not preserved in that
fallback; special files are rejected. A failed copy leaves the source and an
existing destination intact. A failure while removing the source can leave
both copies. `-n` and overwrite prompts check the destination before moving;
they do not protect against concurrent destination changes.

Recursive operations are path-based, intended for ordinary local use; they do
not provide race protection against another process replacing directory
components while traversal is in progress. Symlinks encountered during normal
recursive traversal are never followed.

Exit status is 0 for success and 1 for usage or filesystem errors. Errors name
the affected path, and independent operands continue after a failure. Declining
an interactive prompt or skipping an existing destination with `mv -n` succeeds.
On Windows-mounted WSL directories, permissions follow the mount's settings;
full POSIX mode behavior requires a filesystem with Linux permission support.
