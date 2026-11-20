# Segment 2 (0:39 - 1:16) transcript

## Timeline
- [00:39] Terminal window (macOS), 3 tabs: `...way.out • make run-conway`, `...ut • make run-forestfire` (spinner = running), active tab `.../hayyoth-lab — -zsh`. Active prompt: `hayyoth-lab $ make run-` (typing). Right: two raylib windows titled "Hayyoth". Back one = Conway's Game of Life (white cells on black, gliders/blinkers, animating). Front one = forest fire (green trees on black, red burning cells at borders of burnt regions, animating). A second small Terminal window (bottom-center) shows an earlier `hayyoth-lab $ ls lab` output (see File states). Webcam ignored.
- [00:40] Typing: `hayyoth-lab $ make run-gray`.
- [00:41] Still `make run-gray`; forestfire tab spinner gone.
- [00:42] Still `make run-gray`. Small terminal ls output now fully visible (grayscott.c, karma.c).
- [00:43] `make run-graysco`.
- [00:44] `make run-grayscott`.
- [00:45] Enter pressed. Active tab title becomes `...make run-grayscott` (spinner). Terminal fills with raylib INFO log lines (see Shell). New "Hayyoth" raylib window opens: solid pure blue background with a small glowing cyan blob in center (Gray-Scott, initial seed).
- [00:46] Blob becomes a ring/circle (blue center, cyan glowing rim). Window being dragged.
- [00:47] Blob stretches into a horizontal oval/capsule.
- [00:48] Blob splits into two circles (self-replication).
- [00:49] Two circles elongate vertically, about to divide again. Window moved right.
- [00:50] Four spots (2x2). Terminal log fully visible incl. Working Directory path.
- [00:51] 4 spots elongated horizontally (dividing).
- [00:52] 8 spots (2 rows x 4). Mouse over terminal.
- [00:53] New (4th) terminal tab opened. Tabs: `...ake run-conway`, `...ke run-forestfire`, `...un-grayscott` (spinner), active `...-lab — -zsh`. Prompt `hayyoth-lab $ ` empty. Gray-Scott: 8 spots elongating vertically.
- [00:54] 8 vertical capsules; small bottom terminal fully shown (ls lab output).
- [00:55] Typing `hayyoth-lab $ m`. Gray-Scott now 16 spots (4x4).
- [00:56] `make run`; 16 spots; grayscott tab spinner gone.
- [00:57] `make run-k`. 16 spots, corner ones elongated.
- [00:58] `make run-karma`. Gray-Scott ~24 spots (6 columns), still dividing.
- [00:59] Enter pressed: tab title `...e run-karma` (spinner); raylib log starts (see Shell).
- [01:00] New raylib window "Karman vortex street": dark navy background, grey disc (cylinder obstacle) at left, orange (upper) and cyan/blue (lower) wake streaming right, early symmetric stage. Terminal prints `Re = 100   tau = 0.548   (experimental Strouhal around 0.16 - 0.19)`.
- [01:01] Wake longer, still symmetric.
- [01:02] Window dragged right/wider; wake longer.
- [01:03] Window dragged up; wake begins slight waviness.
- [01:04] Gentle sinusoidal waving of wake (instability onset). Gray-Scott beneath: ~10 columns of spots.
- [01:05] Wave amplitude grows; Gray-Scott has more rows/columns of spots.
- [01:06] Larger oscillation, orange/cyan alternating.
- [01:07] Strong vortex shedding: alternating orange and cyan vortices streaming downstream.
- [01:08] Vortices detaching, developed street.
- [01:09] Clear alternating vortex street across window.
- [01:10] Karman run ended/window closed (tab `...e run-karma` no spinner). Terminal in an empty new prompt `hayyoth-lab $ `; Karman window title bar blank, faded vortex image remnants painted over the Gray-Scott window (ghost/redraw artifact).
- [01:11] Tabs: `...n-conway`, `...-forestfire`, `...grayscott`, `...un-karma`, active `...b — -zsh` (5 tabs). Typing `hayyoth-lab $ make `. Karman remnant still painted, fading.
- [01:12] `make run`; tab bar shows `...` overflow marker.
- [01:13] `make run-ant` (spinner at karma tab position).
- [01:14] `make run-ants`.
- [01:15] `make run-ants_`.
- [01:16] `make run-ants_findf` (still typing; not executed within this segment).

## File states
No source files (vim/code) shown in this segment; terminal only. Directory listing seen in the small terminal window:

```
 1| hayyoth-lab $ ls lab
 2| ants_findfood.c  conway.c         grayscott.c
 3| ants.c           diffusion.c      karma.c
 4| conway_ascii.c   forestfire.c
 5| hayyoth-lab $
```
(Three-column layout: col1 ants_findfood.c / ants.c / conway_ascii.c; col2 conway.c / diffusion.c / forestfire.c; col3 grayscott.c / karma.c.)

## Shell & outputs
- [00:39-00:44] Typed: `make run-grayscott` (partials seen: `make run-`, `make run-gray`, `make run-graysco`, `make run-grayscott`). Prompt format `hayyoth-lab $ `.
- [00:45-00:52] Output after Enter (raylib startup log; top of log scrolled off):
```
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
INFO: TIMER: Target time per frame: 16.667 milliseconds
INFO: TEXTURE: [ID 3] Texture loaded successfully (200x200 | R8G8B8A8 | 1 mipmaps)
```
  Active tab title during run: `...make run-grayscott`.
- [00:53-00:58] Second tab: typed `make run-karma` (partials `m`, `make run`, `make run-k`, `make run-karma`).
- [00:59] Output after Enter:
```
hayyoth-lab $ make run-karma
./build/lab/karma.out
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
- [01:00-01:09] Rest of same raylib log (here texture 3 is `(440x120 | R8G8B8A8 | 1 mipmaps)`), then:
```
Re = 100   tau = 0.548   (experimental Strouhal around 0.16 - 0.19)
```
  Window title "Karman vortex street". `make run-karma` echoes only the binary path `./build/lab/karma.out` (no compile output visible; already built).
- [01:10-01:16] New (5th) tab; typed `make run-ants_findf` (partials `make `, `make run`, `make run-ant`, `make run-ants`, `make run-ants_`, `make run-ants_findf`). Not yet executed at 1:16.

## Observations
- Build: Makefile with per-example targets `run-<name>` where `<name>` is the basename of `lab/<name>.c` (run-conway, run-forestfire, run-grayscott, run-karma, run-ants, run-ants_findfood). Binaries go to `./build/lab/<name>.out`. `make run-X` is quiet (prints only the binary path).
- Lab examples: ants_findfood.c, ants.c, conway_ascii.c, conway.c, diffusion.c, forestfire.c, grayscott.c, karma.c.
- raylib 6.2, GLFW desktop backend, Apple M2, OpenGL 4.1 Metal. Home `/Users/ctsotc`, project dir `~/sources/hayyoth-lab`.
- Most sims open a window titled "Hayyoth" (Conway, forest fire, Gray-Scott); karma sets its own title "Karman vortex street", so the framework takes a title parameter.
- Karma: lattice-Boltzmann-style flow (tau = 0.548, Re = 100; prints comparison to experimental Strouhal 0.16-0.19), rendered as vorticity colormap (orange/cyan) around a grey cylinder. Texture 440x120 suggests a 440x120 grid uploaded as a texture each frame; Gray-Scott texture 200x200 => 200x200 grid. Suggests the view renders cell grid to texture scaled to window.
- Gray-Scott: blue background with seeded blob that self-replicates (mitosis) 1 -> 2 -> 4 -> 8 -> 16 -> 24+ spots; cyan rim.
- Conway: white cells on black; forest fire: green trees, red fire, black empty.
- Several sims left running simultaneously in separate windows/tabs.
- Philosophy signals: none in this segment (no shell comments, README or code). It is a gallery run: discrete CA -> reaction-diffusion -> fluid dynamics, same framework.
