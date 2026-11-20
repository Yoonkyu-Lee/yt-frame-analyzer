# Magical moving numbers

A C terminal animation: glowing random digits drift and fall through cyan,
green, violet, and gold trails, with scattered sparks. It runs at about 30 FPS,
adapts to terminal resizing, and restores your terminal when it exits.

From PowerShell in this directory, build and run through WSL:

```powershell
wsl.exe -e cc -std=c11 -O2 -Wall -Wextra -Wpedantic main.c -o main -lm
wsl.exe -e ./main
```

Or build with `wsl.exe -e make`. Requires a POSIX environment and an ANSI terminal;
Windows Terminal with WSL works well. No external libraries are needed beyond the
C system and math libraries.

Controls:

- **Space** pauses or resumes.
- **+ / -** changes speed from 0.25x to 4x.
- **R** remixes the scene.
- **Q**, **Esc**, or **Ctrl+C** exits.

Options:

```powershell
wsl.exe -e ./main --frames 150
wsl.exe -e ./main --no-color
wsl.exe -e ./main --seed 42
wsl.exe -e ./main --help
```

`--frames N` stops after N frames. `--seed N` selects a repeatable random sequence
(interactive timing and resizing affect the result). `--no-color` or the
`NO_COLOR` environment variable disables colors. Redirected output defaults to a
single plain-text snapshot; `--frames` can request multiple snapshots.

The scene uses at most 240 columns and 80 rows to keep rendering lightweight.

Run the terminal integration checks (requires Python 3 in WSL):

```powershell
wsl.exe -e python3 test_terminal.py
```
