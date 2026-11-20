# Segment 1 (0:00 - 0:39)

No code editor / vim / browser appears in this segment. Only terminals and simulation windows. Webcam "Human" ignored. No frames exist for 0:01-0:04.

## Timeline
- [00:00] Two macOS Terminal windows. Top ("Terminal", no tab bar yet): prompt `hayyoth-lab $ ` empty. Lower ("Terminal"): `hayyoth-lab $ ls lab` with output (see Shell & outputs). Prompt style is `hayyoth-lab $ ` (custom PS1).
- [00:05] Top terminal shows raylib startup INFO log (a conway sim was launched earlier; no sim window yet visible). Block cursor at bottom.
- [00:06] A raylib window titled "Hayyoth" appears (black bg, white outlined cells): Conway's Game of Life on a random soup, many small still lifes, blinkers, gliders; white on black.
- [00:07-00:08] Hayyoth window dragged right (x ~790-1530); Conway animates.
- [00:09] Top terminal now has a tab bar. Tab 1: `.../hayyoth-lab — conway.out • make run-conway` (spinner = running). Tab 2 (active, new): `~/sources/hayyoth-lab — login`. Prompt empty.
- [00:10] Tab 2 title `~/sources/hayyoth-lab — -zsh`; typing `make`
- [00:11] `make run`
- [00:12] `make run-`
- [00:13] `make run-di`
- [00:14] `make run-diffusio`
- [00:15] Enter: `make run-diffusion` then line `./build/lab/diffusion.out`. Tab 2 title `...lab — diffusion.out • make run-diffusion`. No raylib window for it: diffusion prints a text grid.
- [00:16] Terminal fills with integer grid (diffusion, text mode). Terminal narrow so rows wrap (15 columns shown, remaining 5 wrap to next line). Centre value `35`, ring `7 6 1`.
- [00:17] Grid updated: centre `13`, spreading.
- [00:18] Centre `7`, flatter (`5 4 3 2 1` shoulders).
- [00:19] Terminal being resized wider (title `Terminal — 91×29`); grid shows 18 cols, still wrapping. Tab titles lengthen: `~/sources/hayyoth-lab — conway.out • make run-conway`.
- [00:20] Terminal wider again; 20 columns x 10 rows without wrapping. Centre value `4`.
- [00:21-00:22] Plateau emerging: centre `3`, then mostly `2` in centre, `1` at edges.
- [00:23] Grid is final stable plateau. Shows `^Cmake: *** [run-diffusion] Interrupt: 2` then new prompt `hayyoth-lab $ ` (Ctrl-C pressed). Tab 2 title `...— diffusion.out • make run-diffusion`, no spinner.
- [00:24] typing `make ` ; tab 2 title back to `~/sources/hayyoth-lab — -zsh`.
- [00:25] `make run`
- [00:26] `make run-`
- [00:27] `make run-fores`
- [00:28] `make run-forest`
- [00:29] `make run-forestfire`
- [00:30] Enter: output `./build/lab/forestfire.out`. Tab 2 title `~/sources/hayyoth-lab — forestfire.out • make run-forestfire` (spinner).
- [00:31] New raylib "Hayyoth" window opens over the terminal. Log shows `Screen size: 500 x 500`, `Render size: 500 x 500`, `TIMER: Target time per frame: 33.333 milliseconds`. Forest-fire sim: black bg, mostly green cells (trees), several large roughly round black burnt patches with thin red fire edges.
- [00:32] Same sim: mostly green, a few fire fronts (red) at top edge and bottom; burnt areas mostly regrown (sparse green dots on black).
- [00:33] Window dragged right. Last log line `TEXTURE: [ID 3] Texture loaded successfully (250x250 | R8G8B8A8 | 1 mipmaps)`. Mostly green, few small burn scars with red edges.
- [00:34] Large black burnt regions ringed by thin red fire lines, green elsewhere.
- [00:35] Forest nearly all green, fire only at top-right edge. Window partly behind terminal; old Conway window visible behind.
- [00:36] Terminal window narrowed (log re-wraps); tabs `...hayyoth-lab — conway.out • make run-conway`, `...— forestfire.out • make run-forestfire` (2 tabs). Burn patches with red edges.
- [00:37] Third tab opened: tabs `...way.out • make run-conway`, `...ut • make run-forestfire`, `.../hayyoth-lab — -zsh` (active); prompt `hayyoth-lab $ ` empty. Forest has scattered burn scars.
- [00:38] typing `make r`
- [00:39] `make run-` (unfinished at segment end; forestfire shows large burnt regions with red edges; Conway keeps running behind).

## File states
No source files visible in this segment.

## Shell & outputs
[00:00] lower terminal:
```
hayyoth-lab $ ls lab
ants_findfood.c  conway.c     grayscott.c
ants.c           diffusion.c  karma.c
conway_ascii.c   forestfire.c
hayyoth-lab $
```

[00:05] top terminal log (conway run, tab `make run-conway`, `conway.out`), verbatim:
```
INFO:     > Viewport offsets: 0, 0
INFO: GLAD: OpenGL extensions loaded successfully
INFO: GL: Supported extensions count: 43
INFO: GL: OpenGL device information:
INFO:     > Vendor:   Apple
INFO:     > Renderer: Apple M2
INFO:     > Version:  4.1 Metal - 90.5
INFO:     > GLSL:     4.10
INFO: GL: VAO extension detected, VAO functions loaded successfully
INFO: GL: NPOT textures extension detected, full NPOT textures supported
INFO: GL: DXT compressed textures supported
INFO: PLATFORM: DESKTOP (GLFW - Cocoa): Initialized successfully
INFO: TEXTURE: [ID 1] Texture loaded successfully (1x1 | R8G8B8A8 | 1 mipmaps)
INFO: TEXTURE: [ID 1] Default texture loaded successfully
INFO: SHADER: [ID 1] Vertex shader compiled successfully
INFO: SHADER: [ID 2] Fragment shader compiled successfully
INFO: SHADER: [ID 3] Program shader loaded successfully
INFO: SHADER: [ID 3] Default shader loaded successfully
INFO: RLGL: Render batch vertex buffers loaded successfully in RAM (CPU)
INFO: RLGL: Render batch vertex buffers loaded successfully in VRAM (GPU)
INFO: RLGL: Default OpenGL state initialized successfully
INFO: TEXTURE: [ID 2] Texture loaded successfully (128x128 | GRAY_ALPHA | 1 mipmaps)
INFO: FONT: Default font loaded successfully (224 glyphs)
INFO: SYSTEM: Working Directory: /Users/ctsotc/sources/hayyoth-lab
INFO: TIMER: Target time per frame: 66.667 milliseconds
INFO: TEXTURE: [ID 3] Texture loaded successfully (300x300 | R8G8B8A8 | 1 mipmaps)
```

[00:10-00:15]
```
hayyoth-lab $ make run-diffusion
./build/lab/diffusion.out
```

[00:16-00:23] diffusion.out prints successive integer grids (no header), terminal-redrawn. Snapshots (values read from frames; `⟨?⟩` = not fully certain):
[00:16] (terminal 15 cols wide; wrapped rows; only central rows shown):
```
 . . . 1 1 1 . .            (row 4: ... 0 0 1 1 1 0 0 ...)
 . . 1 6 7 6 1 . .          (row 5)
 . . 1 7 35 7 1 . .         (row 6, centre)
 . . 1 6 7 6 1 . .          (row 7)
 . . . 1 1 1 . . .          (row 8)
```
(all other cells `0`; first rows 1-3 all zeros)
[00:17] centre row `1 2 7 13 7 2 1`, rows around: `2 6 7 6 2`, `1 2 2 2 1`, top `1`.
[00:18] centre row `1 3 5 7 5 3 1`, rows: `1 2 4 5 4 2 1`, `1 1 2 3 2 1 1`, `1 1 1 1 1`.
[00:19] (18 cols) centre row `1 3 4 5 4 3 1`, `1 2 4 4 4 2 1`, `1 1 2 3 2 1 1`, `1 1 1 1 1`, last row gains `1 1 1`.
[00:20] (20 cols x 10 rows), central columns 8-14:
```
r1  0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
r2  0 0 0 0 0 0 0 0 0 1 1 1 0 0 0 0 0 0 0 0
r3  0 0 0 0 0 0 0 0 1 1 1 1 1 0 0 0 0 0 0 0
r4  0 0 0 0 0 0 0 1 1 2 2 2 1 1 0 0 0 0 0 0
r5  0 0 0 0 0 0 0 1 2 3 3 3 2 1 0 0 0 0 0 0
r6  0 0 0 0 0 0 1 1 2 3 4 3 2 1 1 0 0 0 0 0
r7  0 0 0 0 0 0 1 1 2 3 3 3 2 1 1 0 0 0 0 0
r8  0 0 0 0 0 0 0 1 1 2 2 2 1 1 0 0 0 0 0 0
r9  0 0 0 0 0 0 0 1 1 1 1 1 1 1 0 0 0 0 0 0
r10 0 0 0 0 0 0 0 0 1 1 1 1 1 0 0 0 0 0 0 0
```
[00:21-00:30] final stable grid (unchanged after ~00:21):
```
r1  0 0 0 0 0 0 0 0 1 1 1 1 1 0 0 0 0 0 0 0
r2  0 0 0 0 0 0 0 0 1 1 1 1 1 0 0 0 0 0 0 0
r3  0 0 0 0 0 0 0 1 1 1 1 1 1 1 0 0 0 0 0 0
r4  0 0 0 0 0 0 1 1 1 2 2 2 1 1 1 0 0 0 0 0
r5  0 0 0 0 0 0 1 1 2 2 2 2 2 1 1 0 0 0 0 0
r6  0 0 0 0 0 0 1 1 2 2 2 2 2 1 1 0 0 0 0 0
r7  0 0 0 0 0 0 1 1 2 2 2 2 2 1 1 0 0 0 0 0
r8  0 0 0 0 0 0 1 1 1 2 2 2 1 1 1 0 0 0 0 0
r9  0 0 0 0 0 0 0 1 1 1 1 1 1 1 0 0 0 0 0 0
r10 0 0 0 0 0 0 0 1 1 1 1 1 1 1 0 0 0 0 0 0
```
(Row/column positions approximate by eye; the pattern of a flat plateau is certain.)

[00:23]
```
^Cmake: *** [run-diffusion] Interrupt: 2

hayyoth-lab $
```

[00:24-00:30]
```
hayyoth-lab $ make run-forestfire
./build/lab/forestfire.out
```
[00:31-] forestfire log: same INFO block as conway but with
```
INFO:     > Screen size:  500 x 500
INFO:     > Render size:  500 x 500
INFO:     > Viewport offsets: 0, 0
...
INFO: SYSTEM: Working Directory: /Users/ctsotc/sources/hayyoth-lab
INFO: TIMER: Target time per frame: 33.333 milliseconds
INFO: TEXTURE: [ID 3] Texture loaded successfully (250x250 | R8G8B8A8 | 1 mipmaps)
```
[00:37-00:39] new third tab; typed `make r` then `make run-` (command unfinished at segment end).

## Observations
- Build system: Makefile with `run-<name>` targets, one per `lab/*.c`; binaries at `./build/lab/<name>.out`; make echoes `./build/lab/X.out`. Ctrl-C gives `make: *** [run-X] Interrupt: 2`.
- lab/ contents: ants_findfood.c, ants.c, conway.c, conway_ascii.c, diffusion.c, forestfire.c, grayscott.c, karma.c. Both ASCII (terminal) and raylib variants exist (conway_ascii vs conway); diffusion appears terminal-only here (integer grid printed in text).
- One engine, multiple sims: Conway uses 300x300 grid texture at 66.667 ms/frame (15 fps); forestfire uses 250x250 grid texture drawn on a 500x500 screen (2 px per cell) at 33.333 ms/frame (30 fps). Window title is always "Hayyoth". Grid size / frame time are per-sim configuration, consistent with state/rule/view/timer separation (state grid -> texture -> raylib view).
- Platform: Apple M2, raylib via GLFW-Cocoa, OpenGL 4.1 Metal; home dir `/Users/ctsotc/sources/hayyoth-lab`.
- Forest fire render: black empty, green tree, red fire; yields circular burn scars with red fronts and regrowth.
- Diffusion demo: integer-valued point source (35 at centre) spreading and flattening into a plateau of 1s/2s then stalling (integer arithmetic floor stops flow).
- No shell `#` comments, README text, or code visible in this segment; video is silent.
