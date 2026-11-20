# Segment 4 (05:34 - 07:38) - "How we (Human) learn better than AI?"

Note: I viewed about 36 of the 51 listed frames (all scene changes and every typing stage; skipped near-duplicates: 05:35, 05:36, 06:20, 06:21, 06:47, 06:48, 06:51, 06:57, 07:01, 07:04, 07:06, 07:08, 07:10, 07:12, 07:14, 07:16, 07:18, 07:21). The yellow note ("How we learn better than AI?...") is collapsed to a title bar the whole time. Webcam "Human" window ignored.

## Timeline
- [05:34] Browser, tab "hayyoth-lab/README" on codeberg.org (hayyoth-lab README.md): C API docs, `hRender()` section, `while (hRun()) { hCell(rule); hRender(view); }`. Cyan sticky note over it reads: "but AI can learn the complex". Yellow note collapsed at top right. Tabs: "The Creation of Chain...", "The Dynamic Array. Co...", "Hash Table, for dummi...", "hayyoth-lab/README", "tree/src/rust_tree/cli.rs", "Google Gemini".
- [05:37] He is closing tabs: the GitHub tab "tree/src/rust_tree/cli.rs" (a Rust `tree` clone, 461 lines) is shown with tooltip while the cursor is on its close button (big Rust CLI source = example of complexity).
- [05:39-05:40] Tabs reduced to 3 YouTube tabs + Google Gemini. Gemini empty prompt page ("Where should w..."), then a pasted README-like text visible in the prompt box ("Or take only the pieces that are useful and build th... / The framework stays small. / The world you build on top of it does not have to be."). Then a Gemini chat: user prompt "# Hayyoth Lab > We stole Zeus's lightning, trapped it in s... to think." (the Hayyoth Lab README pasted). Gemini answers "Here is a complete Forest Fire simulation written for Hayyoth Lab, using ... types, and APIs described in the README" with "The Forest Fire Model", 3 states Empty(0)/Tree(1)/Fire(2). Sticky note moved right/up slightly (same text).
- [06:18-06:22] Gemini finished generating a C program (`lab/forest.c`; tab title "Hayyoth Lab Forest Fire Simulat..."). Scrolled to the end of the code: `main` with HcellConf 250x250, HrdConf fps=15 scale=3, hSetup, hDataInit(init), `while (hRun()) { hCell(rule); hRender(view); }`.
- [06:25] He clicks copy ("Copied to clipboard" toast), then moves to the macOS Dock (tooltip "Terminal").
- [06:26] Terminal window (cream background), empty prompt `~ $`. Desktop wallpaper is the painting "Wanderer above the Sea of Fog". Cyan note unchanged.
- [06:46] Terminal: `hayyoth-lab $ make run-test` compiles `lab/test.c` and runs `./build/lab/test.out`; raylib 6.2 INFO log. Tabs: "~/sources/hayyoth-lab - vim lab/test.c" and "...-lab - test.out < make run-test".
- [06:49] A window "Hayyoth" opens: black grid with sparse green dots (trees) and a few red (fire) cells: the AI-generated forest fire simulation running.
- [06:54] Grid dense with green trees, a few red.
- [06:57-06:59] Simulation continues; fire fronts appear. The cyan note is selected and he types a new text replacing the old: "it work |".
- [06:59] Note: "it work" / blank / "AI love compl|"
- [07:01-07:03] Note: "it work" / "AI love complex" / "you|" (typing).
- [07:05-07:07] Note: "it work / AI love complex / your brain not |".
- [07:09] Simulation window closed; desktop only; note text unchanged.
- [07:11] Terminal reappears with raylib shutdown log ("Window closed successfully") and a new `hayyoth-lab $` prompt. Working directory line shows `/Users/<account>/sources/hayyoth-lab`.
- [07:13] Desktop only again. Note final: "it work / AI love complex / your brain not".
- [07:15] Back in browser on the Gemini chat code; cursor moves toward the Dock.
- [07:17] Dock hover tooltip "Preview"; note unchanged.
- [07:19] Apple Books app, "PDFs" list: "125 Problems in Text Algorithms with Solutions", "The Scheme Programming Language, 4th Edi...", "vdoc.pub_the-practice-of-programming" (Brian W. Kernighan), "Code Biology _ A New Science of Life", "Numerical C".
- [07:22] Opens "vdoc.pdf - Page 1 of 733": cover of "The Practice of Programming", Brian W. Kernighan, Rob Pike; the cover easel reads "Simplicity / Clarity / Generality". Left TOC: Chapter 1 Style ... Chapter 9 Notation, Epilogue, Appendix, Index.
- [07:29] Expands Chapter 2 "Algorithms and Data Structures", selects "2.7. Lists" (page 70 of 733): arrays vs lists, singly-linked list diagram (head -> data 1..4 -> NULL), Exercises 2-5, 2-6.
- [07:31-07:38] Scrolls pages 71-73: C code for `newitem`, `addfront`, `addend`, `lookup`, `apply`, with prose. Note unchanged throughout. He shows a simple classic C book as the "simple" counterpart to AI output.

## Texts on screen
### Cyan sticky note, first version (05:34 - ~06:57)
```
but AI can learn the complex
```

### Cyan sticky note, final version in this segment (typed 06:57 - 07:07, visible to 07:38)
```
it work

AI love complex

your brain not
```
(Ends mid-thought; "your brain not ..." presumably continues in the next segment.)

### Yellow sticky note (collapsed, whole segment)
```
How we learn better than AI?...
```

### Gemini prompt/reply fragments (05:39 - 06:22)
```
# Hayyoth Lab

> We stole Zeus's lightning, trapped it in s⟨?⟩ ... to think.
```
```
Or take only the pieces that are useful and build th⟨?⟩
The framework stays small.
The world you build on top of it does not have to be.
```
```
Here is a complete Forest Fire simulation written for Hayyoth Lab, usin⟨?⟩ types, and APIs described in the README.
The Forest Fire Model
This simulation models three states for each cell:
1. Empty (0): Burned out ash or bare ground. Trees can slowly regrow here.
2. Tree (1): A living tree. It can catch fire if a neighboring cell is burning or via a lightning strike.
3. Fire (2): A burning tree. It turns into empty ground in the next step.
Code (lab/forest.c)
```

### Apple Books PDF list (07:19)
```
125 Problems in Text Algorithms with Solutions - Unknown Author
The Scheme Programming Language, 4th Edi⟨?⟩ - R. Kent Dybvig
vdoc.pub_the-practice-of-programming - Brian W. Kernighan
Code Biology _ A New Science of Life - Unknown Author
Numerical C - Unknown Author
```

## Code & shell
Gemini-generated C (end of file, visible 06:18-07:17):
```c
  return (HrdView){.r = 0, .g = 0, .b = 0};        // Black for empty space
}

int main(void) {
  HcellConf cellConf = {.width = 250, .height = 250, .cell⟨?⟩
  HrdConf rdConf = {.fps = 15, .scale = 3};

  if (!hSetup(cellConf, rdConf))
    return 1;

  hDataInit(init);

  while (hRun()) {
    hCell(rule);
    hRender(view);
  }

  return 0;
}
```
README snippet (05:34):
```c
while (hRun()) {
  hCell(rule);
  hRender(view);
}
```
Terminal (06:46):
```
hayyoth-lab $ make run-test
cc -Wall -Wextra -pedantic -O2 -fPIC -Isrc lab/test.c \
                -Lbuild/src \
                -lhayyoth \
                -o build/lab/test.out \
                -Wl,-rpath,@loader_path/../src
./build/lab/test.out
INFO: Initializing raylib 6.2
INFO: Platform backend: DESKTOP (GLFW)
INFO: Supported raylib modules:
INFO:     > rcore:..... loaded (mandatory)
INFO:     > rlgl:...... loaded (mandatory)
INFO:     > rshapes:... loaded (optional)
INFO:     > rtextures:. loaded (optional)
INFO:     > rtext:..... loaded (optional)
INFO:     > rmodels:... loaded (optional)
INFO:     > raudio:.... loaded (optional)
```
Terminal (07:11, end of run): raylib SHADER/RLGL/TEXTURE/FONT info lines, `INFO: SYSTEM: Working Directory: /Users/<account>/sources/hayyoth-lab`, `INFO: TIMER: Target time per frame: 66.667 milliseconds`, texture 250x250 R8G8B8A8, `INFO: Window closed successfully`, then `hayyoth-lab $`.

Book code (Kernighan & Pike, pages 71-73, shown 07:31-07:38): `newitem` (emalloc, newp->name/value/next), `addfront` (`newp->next = listp; return newp;`), `addend` (loop to end, O(n)), `lookup` (sequential strcmp search), `apply`.

## Observations
- Argument so far: he first wrote "but AI can learn the complex" (conceding AI handles complexity), then demonstrated it: he pasted his own library's README (Hayyoth Lab, a small C cell-simulation framework) into Gemini, and the AI produced a complete forest-fire simulation using that API, which compiled and ran ("it work").
- He replaced the note with "it work / AI love complex / your brain not": AI thrives on complexity, the human brain does not. The AI succeeded here because his API is small and simple (README: "The framework stays small").
- Before this he closed tabs showing a big Rust `tree` clone (cli.rs) as an example of complex software.
- Then he turned to a simple classic C book (The Practice of Programming: "Simplicity, Clarity, Generality") and its basic linked-list chapter: the human learning path is small, clear fundamentals.
- Method: live typing on a sticky note while demonstrating (AI generation, terminal compile and run, then books); no slides, all on the desktop. His library of programming PDFs (Scheme, Numerical C, Code Biology, text algorithms) suggests a self-taught, fundamentals-first study life.
- Privacy: terminal home-directory user name replaced by `<account>`.
