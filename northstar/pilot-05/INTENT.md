# INTENT — file utilities (pilot)

Owner: experiment condition (drafted by the analysis agent from the original task, see note below)
Source task: "Build Linux file utilities in C: touch, cat, mkdir, mv, rm and find." (vibe-coding/05 PROMPT.md)

> Note: in a real project the intent owner is a human. Here the scope was fixed *before* any
> compression, from the task text plus everyday use of these commands, and frozen. It sits between
> the human video version (very small) and the Codex version (GNU-like). The result of the
> experiment depends on this choice; it is recorded so the comparison is honest.

## Purpose
Small, dependable versions of six file commands for everyday shell use and scripts.

## Intents

| ID | Intent | Test |
|---|---|---|
| C1 | Errors go to stderr as `<prog>: <what> '<path>': <reason>`; remaining operands are still processed; exit status 1 if any operand failed, else 0 | tests/intent.sh `C1_*` |
| C2 | Missing operands print a usage line and exit 1 (except `rm -f` with no operands: exit 0) | `C2_*` |
| I1 | `touch FILE...` creates missing files empty and sets access/modification time of existing files to now **without changing their content** | `I1_*` |
| I2 | `cat [FILE...]` writes files to stdout in order, byte for byte; no operands or `-` reads stdin | `I2_*` |
| I3 | `mkdir [-p] DIR...` creates directories; fails if the path exists; with `-p` creates missing parents and accepts existing directories, but fails if a path component is a file | `I3_*` |
| I4 | `mv SRC DST` renames, replacing an existing file `DST`; `mv SRC... DIR` (or `DST` an existing directory) moves into the directory keeping base names; more than one `SRC` with a non-directory target is an error | `I4_*` |
| I5 | `rm [-r] [-f] PATH...` removes files and symlinks (never follows links); directories only with `-r`, recursively; `-f` ignores missing paths; refuses `.`, `..` and `/` | `I5_*` |
| I6 | `find [PATH...] [-name GLOB] [-type f\|d]` walks paths (default `.`) without following symlinks and prints every path that matches all given tests | `I6_*` |
| P1 | No resource regression on the hot paths vs. the start: `cat` of a large file, `find` and `rm -r` over 50k files | bench in metrics.sh |

## Revision 1 (after the skeptic review, before R4)
The skeptic showed that the 60 tests left these behaviors unpinned, so the compression could drop them unnoticed.
They were always implied by C1 or I1–I6 and are now written down, with tests (`tests/intent.sh`, the block after the I6 tests):
- C1 also covers **write errors**: if stdout cannot be written, `cat` and `find` report it, stop copying, and exit 1.
- C2: a leading `--` ends the (empty) option list.
- I3: an empty operand is an error like any other (no crash, later operands still processed).
- I4–I6: a trailing slash on an operand demands a directory (`rm file/`, `mv file/ x`, `find file/` fail with ENOTDIR).
- I6: `-name` and `-type` may repeat; a path must pass all of them.

## Scale and resource assumptions
- Paths and file counts limited only by the OS; recursion depth = directory depth.

## Trust boundaries
- Command-line arguments (validate options and operand count).
- File system state (anything may fail; report and continue).

## Non-goals
Deliberately not done; code for these is accidental here.
- `--help` and long options, `-v` verbose output, `-i` interactive prompts anywhere
- `touch -a -m -c -h -r -t`; `cat -n -b -s -v -E -T -A -e -t -u`; `mkdir -m -v`
- `mv -f -i -n -v -T`, moving across file systems (report the `EXDEV` error instead), same-file detection
- `rm -d -i -v -R`
- `find` operators (`!`, `-not`, `-a`, `-o`, parentheses), `-iname -path -size -mtime -mmin -empty -newer -print -print0 -delete -prune -mindepth -maxdepth -depth -P`, `-type` other than `f`/`d`
