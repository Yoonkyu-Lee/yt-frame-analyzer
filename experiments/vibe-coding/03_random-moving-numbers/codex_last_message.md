Built colorful moving digits with glowing trails, resizing, pause, and speed controls. WSL build and terminal tests passed.

```powershell
wsl.exe -e cc -std=c11 -O2 main.c -o main -lm
wsl.exe -e ./main
```

**Space** pauses, **+/-** adjusts speed, **R** remixes, and **Q** quits.