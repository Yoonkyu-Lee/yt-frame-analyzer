# Segment 3 (1:16 - 2:01) - frames k_01m16s ... k_02m01s (37 frames, all viewed)

## Timeline
- [01:16] Terminal window (small, top-left). Tab bar: `...n-conway`, `...-forestfire`, `...grayscott`, `...un-karma` (spinner = process running), `...b — -zsh` (active, last). Prompt `hayyoth-lab $ make run-ants_findf` being typed (cursor after "f"). Behind: raylib "Hayyoth" windows (blue Gray-Scott spots) and an obscured window with orange/cyan swirls on dark (Karman vortex street, running). A second small terminal at bottom shows `hayyoth-lab $ ls lab` output (see Shell).
- [01:18] Command is `make run-ants_findfood`; make prints `./build/lab/ants_findfood.out`, raylib INFO startup lines begin. Active tab title becomes `..._findfood` with spinner.
- [01:19] "Hayyoth" window (black bg) opens: ants_findfood sim. Blue square (nest) near center. Many red squares scattered (food), a few green squares at fixed spots (food sources/markers), yellow squares (ants), faint dark-purple trails (pheromone-like) between green spots and nest.
- [01:20-01:30] Sim runs: purple trails thicken into bright-purple star/arm shapes radiating from the nest (up, up-left, left, lower-left, down, lower-right diagonal, right). Yellow ants cluster on trails; red squares move/vanish in upper area and later pile up along the bottom edge (by 01:29-01:30). Trails fade later (01:30). Webcam: he eats something around 01:25-01:28 (ignore).
- [01:27] stdout line visible in the terminal behind the sim: `step 500  delivered 323 / 1000`.
- [01:31] Sim window closed. Terminal fully visible again: output ends `INFO: Window closed successfully`, then prompt `hayyoth-lab $ `. Mouse I-beam over terminal. Gray-Scott windows still behind.
- [01:32] "Karman vortex street" window (its title text) comes to front: grey disc obstacle at left, orange (upper) and cyan (lower) vortices shedding to the right on dark background. (Tab `...un-karma` still has spinner.)
- [01:33] Karman window gone; tab bar: `...n-conway`, `...-forestfire`, `...grayscott`, `...b — -zsh ...`, active `...b — -zsh`. Gray-Scott "Hayyoth" window in front: bright blue background with cyan-ringed blue spots in near-lattice arrangement; mouse at upper-left.
- [01:34] Forest fire window in front: green speckled trees, dark empty/burnt patches, thin red fire fronts. Behind it Conway's Life (white outlines on black). Tab titles shifting to `...b — -zsh` with "..." markers.
- [01:35] Only the Conway Game of Life window remains (black bg; alive cells drawn as white hollow outlines; gliders, blinkers, small still lifes). Tab titles mostly `...b — -zsh`.
- [01:36] Conway window animating; mouse on its red close button (top-left of window). All 5 tabs `...b — -zsh`.
- [01:37] All sim windows closed; desktop plain gray. Terminal still shows old output; webcam only.
- [01:38] Terminal cleared: `hayyoth-lab $ `.
- [01:42] Browser (Firefox) opens: "New Tab", address bar focused ("Search with Google or enter address"). Dock partially visible with "Terminal" tooltip.
- [01:43] URL being typed: `co` autocompleted to `codeberg.org/`; top suggestion "hayyoth - Dashboard - Codeberg.org". Firefox Suggest also lists YouTube, Hayyoth - YouTube (youtube.com/@hayyoth-org), Reddit, Channel content - YouTube Studio, Google Translate x2, Google Gemini, Hacker News.
- [01:44] `codeberg.org/hayyoth` (suggestion "hayyoth - Codeberg.org"). History suggestions: `codeberg.org/hayyoth/hayyoth-lab` ("hayyoth/hayyoth-lab: a small and fast C framework for building, running, and ob..."), `codeberg.org/hayyoth/hayyoth-lab/src/branch/main/lab/conway.c` ("hayyoth-lab/lab/conway.c at main - hayyoth/hayyoth-lab - Codeberg.org"), `.../src/branch/main/lab` ("hayyoth-lab/lab at main").
- [01:45-01:46] Typing continues to `codeberg.org/hayyoth/hayyoth-lab`.
- [01:47] Enter pressed; "Waiting for codeberg.org...".
- [01:48] Repo page loaded. Tab title "hayyoth/hayyoth-lab: a small an...". `hayyoth / hayyoth-lab`, Watching 1, Star 0, Fork 0. Nav: Code, Issues, Pull requests, Activity, Enable more, Settings. Description: "a small and fast C framework for building, running, and observing computational worlds". "10 commits", "1 branch", "0 tags", "31 KiB". Branch main; clone HTTPS/SSH (SSH selected) `ssh://git@codeberg.org/hayyot...`. Latest commit: `hayyoth` `712a65829f` "update posix support and docs" "last week". Files: `assets` (init), `lab` (fix, add more lab example), `learn` (add learn dir), `src` (update posix support and docs), `.gitignore` (add learn dir), `LICENSE` (init), `Makefile` (update posix support and docs).
- [01:49] Scrolled to README.md: "Hayyoth Lab"; quote "We stole Zeus's lightning, trapped it in stone, and taught it to think."; cover image of three panels (Gray-Scott blue spots | forest fire green/black | Conway white-on-black). "Hayyoth Lab is a small and fast C framework for building, running, and observing computational worlds." "The idea is simple:" + code block starting `simple state / + / simple rules`.
- [01:50] README scrolled (reading only): idea block, prose, "A World in a Few Lines" heading; Conway code begins.
- [01:51] README scroll: Conway example (init tail, rule, view, main).
- [01:52] README scroll: second example (ants) tail: rule end, view, main.
- [01:53] README scroll: "3. Neighborhoods" section.
- [01:54] Small terminal in front of browser; typed `hayyoth-lab $ # Source code` (comment at prompt, cursor at end).
- [02:00] Terminal fullscreen. Tabs all `~/sources/hayyoth-lab — -zsh` (5). Scrollback ends with prior output, then `hayyoth-lab $ `, `hayyoth-lab $ # How it work`, `hayyoth-lab $ e` (typing; cursor after "e").
- [02:01] Screen cleared; `hayyoth-lab $ e` at top (still typing, command unknown).

## File states
No editor/vim frames in this segment. Only README text seen on codeberg (scrolling/reading, not editing).

### README.md (codeberg), seen in pieces
```
# Hayyoth Lab
> We stole Zeus's lightning, trapped it in stone, and taught it to think.
[cover.png: media/branch/main/assets/cover.png]

Hayyoth Lab is a small and fast C framework for building, running, and observing computational worlds.

The idea is simple:

  simple state
       +
  simple rules
       ↓
   emergence

You define a state and the rules that transform it. Hayyoth provides a small amount of machinery to run those rules and observe what happens.

It does not try to define what a computational world should be.

You build the machine, run it, and see what emerges.

Inspired by the philosophy of hayyoth.org.

## A World in a Few Lines
Here is a complete Conway's Game of Life experiment:

#include "hayyoth.h"

typedef unsigned char Cell;

?? (init signature partly hidden: "...ze_t width) {")
  for (size_t y = 0; y < height; y++)
    for (size_t x = 0; x < width; x++)
      cells[y * width + x] = hRand(0, 9) == 0;
}

void rule(void *c[9], void *n[9]) {
  int alive = 0;
  for (int i = 1; i < 9; i++)
    if (c[i])
      alive += *(Cell *)c[i];

  *(Cell *)n[0] = alive == 3 || (alive == 2 && *(Cell *)c[0]);
}

HrdView view(const void *cell) {
  const Cell *c = cell;
  if (*c)
    return (HrdView){.r = 255, .g = 255, .b = 255};

  return (HrdView){.r = 0, .g = 0, .b = 0};
}

int main(void) {
  HcellConf cellConf = {.width = 300, .height = 300, .cellSize = sizeof(Cell)};
  HrdConf rdConf = {.fps = 15, .scale = 2};
  if (!hSetup(cellConf, rdConf))
    return 1;

  hDataInit(init);
  while (hRun()) {
    hCell(rule);
    hRender(view);
?? (closing lines not seen)
```
Second README example (ants; top cut off; Cell has fields `ant`, `food`; `self` variable used):
```
?? 
  Cell *current = n[0];
  Cell *next = n[hRand(1, 8)];

  if (!next || next->ant)
    return;

  *current = (Cell){0};
  *next = *self;
  next->food = 0;
}

static HrdView view(const void *raw) {
  const Cell *c = raw;

  if (c->ant)
    return (HrdView){.r = 255, .g = 80, .b = 80};
  if (c->food)
    return (HrdView){.r = 255, .g = 180, .b = 0};
  return (HrdView){.r = 0, .g = 0, .b = 0};
}

int main(void) {
  HcellConf cellConf = {.width = 80, .height = 80, .cellSize = sizeof(Cell)};
  HrdConf rdConf = {.fps = 15, .scale = 7};
  if (!hSetup(cellConf, rdConf))
    return 1;

  hDataInit(init);

  while (hRun()) {
    hCell(rule);
    hRender(view);
  }?? (next lines not seen)
```
README "3. Neighborhoods" section (partial; preceding text about double buffering, a code block ending `new world state`):
```
The rule never needs to modify the current world.

It calculates what the world should become and writes that result into `next`.

After every cell has been processed:

  current ← next

This prevents one cell from seeing a mixture of old and already-updated neighbors.

---
3. Neighborhoods

`hcellRun()` visits every cell and constructs a 3×3 neighborhood:

  c[5]  c[1]  c[6]

  c[4]  c[0]  c[2]

  c[8]  c[3]  c[7]

The center is always:

  c[0]
```

## Shell & outputs
Bottom small terminal (static through segment 3):
```
hayyoth-lab $ ls lab
ants_findfood.c  conway.c    grayscott.c
ants.c           diffusion.c karma.c
conway_ascii.c   forestfire.c
hayyoth-lab $
```
[01:16] `hayyoth-lab $ make run-ants_findf` (typing) -> [01:18]:
```
hayyoth-lab $ make run-ants_findfood
./build/lab/ants_findfood.out
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
[01:31] output after window closed (tail of listing; earlier lines at 01:19 include Vendor: Apple, Renderer: Apple M2, Version: 4.1 Metal -...):
```
INFO:     > GLSL:     4.10
INFO: GL: VAO extension detected, VAO functions loaded successfully
INFO: GL: NPOT textures extension detected, full NPOT textures supported
INFO: GL: DXT compressed textures supported
INFO: PLATFORM: DESKTOP (GLFW – Cocoa): Initialized successfully
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
INFO: TIMER: Target time per frame: 33.333 milliseconds
INFO: TEXTURE: [ID 3] Texture loaded successfully (100x100 | R8G8B8A8 | 1 mipmaps)
step 500  delivered 323 / 1000
INFO: TEXTURE: [ID 3] Unloaded texture data from VRAM (GPU)
INFO: TEXTURE: [ID 2] Unloaded texture data from VRAM (GPU)
INFO: SHADER: [ID 3] Default shader unloaded successfully
INFO: TEXTURE: [ID 1] Default texture unloaded successfully
INFO: Window closed successfully
hayyoth-lab $
```
The app-level line `step 500  delivered 323 / 1000` is printed once (at step 500) while the sim keeps running. Cell texture is 100x100 for ants_findfood.
[01:54] `hayyoth-lab $ # Source code`
[02:00] `hayyoth-lab $ # How it work` then `hayyoth-lab $ e` (typing)
[02:01] `hayyoth-lab $ e`

## Observations
- Workflow: `make run-<labname>` (Makefile targets per lab/*.c file; binary at `./build/lab/<name>.out`). Lab files: ants_findfood.c, ants.c, conway.c, conway_ascii.c, diffusion.c, forestfire.c, grayscott.c, karma.c (karma = the "Karman vortex street" window). Five terminal tabs each ran a sim simultaneously (conway, forestfire, grayscott, karma, ants_findfood).
- Sims shown: ants_findfood (blue nest, red food, green sources, yellow ants, purple pheromone trails becoming star-shaped highways; prints `step 500  delivered 323 / 1000`); Karman vortex street (fluid, grey cylinder, orange/cyan vorticity); Gray-Scott (blue bg, cyan-ringed spots); forest fire (green trees, red fire fronts, black burnt); Conway (white outlines on black).
- Framework design (README): simple state + simple rules -> emergence. User supplies a Cell type, `init`, `rule(void *c[9], void *n[9])` (c = current 3x3 neighborhood, n = next-state pointers; c[0]/n[0] is the center; ordering c[1] up, c[2] right, c[3] down, c[4] left, c[5] up-left, c[6] up-right, c[7] down-right, c[8] down-left), and `view(const void*) -> HrdView{r,g,b}`. Framework API: `hSetup(HcellConf{width,height,cellSize}, HrdConf{fps,scale})`, `hDataInit(init)`, `hRun()` loop, `hCell(rule)` (step; double-buffered current <- next), `hRender(view)`, `hRand(lo,hi)`. Clear separation of state (opaque bytes of cellSize), rule (pure transform writing into next), view (cell -> color); rendering decoupled from simulation. Prefix naming: `hcell` (state/stepper), `hrd` (render); umbrella header `hayyoth.h`.
- README: "The rule never needs to modify the current world... This prevents one cell from seeing a mixture of old and already-updated neighbors."
- Philosophy signals: "We stole Zeus's lightning, trapped it in stone, and taught it to think."; "It does not try to define what a computational world should be. You build the machine, run it, and see what emerges. Inspired by the philosophy of hayyoth.org." Repo codeberg.org/hayyoth/hayyoth-lab: 10 commits, 31 KiB, dirs assets/lab/learn/src, Makefile, LICENSE.
- Shell comments typed as narration markers: `# Source code` (01:54), `# How it work` (02:00); then starts typing `e...` (likely editor; unknown).
- Env: /Users/ctsotc/sources/hayyoth-lab; Apple M2, raylib 6.2, GLFW/Cocoa, OpenGL 4.1 Metal; browser is Firefox.
