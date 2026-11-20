# Segment 5 of 5 (04:06 - 05:37) - "vscode - but better" (VSCodium)

Frames viewed: 51 (k_04m06s .. k_05m37s). Webcam overlay "Human" ignored except where noted.

## Timeline
- [04:06] VSCodium window "cat.c - xv6-riscv" (macOS, light theme). Explorer lists xv6 user files (logstress.c, ls.c, mkdir.c, printf.c, rm.c, sh.c, stressfs.c, sync.c, ulib.c, umalloc.c, user.h, user.ld, usertests.c, usys.pl, wc.c, zombie.c, .clang-format, .dir-locals.el, .editorconfig, .gdbinit.tmpl-riscv, .gitignore, LICENSE, Makefile, README, test-xv6.py). Bottom panel open with tabs DEBUG CONSOLE / TERMINAL / PORTS. Right-click context menu on the panel tab bar: Configure Keybinding, Hide 'Debug Console', Hide Badge, Move To >, Problems, Output, Debug Console (checked, highlighted), Terminal (checked), Ports (checked), Panel Position >, Align Panel >, Show Icons, Hide Panel (cmd J). He unticks Debug Console.
- [04:07] Debug Console tab gone; tabs now TERMINAL, PORTS. Terminal prompt "xv6-riscv $".
- [04:08] Context menu again, "Ports" highlighted (checked); he unticks Ports.
- [04:09] Only TERMINAL tab left. Cursor on the "+" (new terminal) button; toolbar: "zsh", +, v, split, trash, "...", maximize, X.
- [04:10] Context menu on the toolbar: Configure Keybinding, Hide 'New Terminal', Focus Terminal (checked, highlighted), New Terminal (checked), Split Terminal (checked), Kill Terminal (checked), Scroll to Previous Command, Scroll to Next Command, Clear Terminal, Run Active File, Run Selected Text, Start Dictation, Reset Menu. He unticks Focus Terminal.
- [04:12-04:13] "zsh" label is gone. Menu reopened; Split Terminal highlighted (being unticked).
- [04:14] Split icon gone; toolbar now: +, v, trash, ..., maximize, X.
- [04:15] Menu reopened; Kill Terminal highlighted (being unticked).
- [04:16] Trash icon gone. Toolbar now only: +, v, "...", maximize, X. Panel is stripped to a bare terminal.
- [04:20] Panel hidden; editor scrolled down: end of cat() `}` then `int main(int argc, char *argv[])` and the start of main.
- [04:21-04:23] Scrolled back to top of cat.c: `char buf[512];`, cat(), start of main. Editor now full height, cursor after the closing `}` of `if (n < 0)`.
- [04:22] Command palette opened with ">". Items: "Preferences: Open User Settings (JSON)" (recently used), "View: Toggle Editor Sticky Scroll", "View: Toggle Tree Sticky Scroll", "Preferences: File Icon Theme", "Preferences: Color Theme" (cmd K cmd T), "File: Open Folder...", "Accounts: Manage Accounts", "Accounts: Manage Extension Account Preferences...", "Accounts: Manage Trusted Extensions For Account", "Add Cursor Above", "Add Cursor Below".
- [04:24] Quick Open (cmd P) shows: "cat.c user" (recently opened) and "settings.json ~/Library/Application Support/VSCodium/User", i.e. the user settings.json had been opened earlier.
- [04:25] Palette typed "> the": Preferences: File Icon Theme (recently used), Preferences: Color Theme, Authentication: Remove Dynamic Authentication Providers, Close Other Windows, Preferences: Browse Color Themes in Marketplace, Preferences: Product Icon Theme, Preferences: Toggle between Light/Dark Themes, Preferences: Try New Default Themes, Search Editor: Open New Search Editor to the Side, Terminal: Create New Terminal in Editor Area to the Side, Terminal: Kill the Active Terminal in Editor Area.
- [04:27] Picker "Select Color Theme for High Contrast Light Mode" (list below under Texts on screen).
- [04:28-04:37] Arrowing through the list with live preview: Light+ highlighted (04:28); Quiet Light previewed - purple title bar, green highlight line, bold red strings (04:29); Light Modern (04:30); Light (VSCodium) (04:31); Light 2026 (04:33, `\n` turns red); Light (VSCodium) again (04:34-04:35); Light Modern (04:37). The picker is then closed and the editor goes back to the original look (04:37-04:38). No visible theme change.
- [04:38-04:39] Mouse in explorer; sidebar scrolling (hand cursor near wc.c).
- [04:40] Explorer scrolled to top: .github, kernel, mkfs, user (expanded) with cat.c selected, dorphan.c, echo.c, forktest.c, forphan.c, grep.c, grind.c, init.c, kill.c, ln.c, logstress.c, ls.c, mkdir.c, printf.c, rm.c, sh.c, stressfs.c, sync.c, ulib.c, umalloc.c, user.h, user.ld.
- [04:41] Palette "> ins": Convert Indentation to Spaces; Developer: Inspect Editor Tokens and Scopes; Extensions: Disable All Installed Extensions; ...for this Workspace; Extensions: Install Extensions; Install from VSIX...; Install Missing Dependencies; Install Specific Version of Extension...; Extensions: Show Installed Extensions; File: Reset Active Editor Read-only in Session; File: Set Active Editor Read-only in Session.
- [04:42] "> install code" yields only "Toggle Source Code in Disassembly View" (similar commands).
- [04:43-04:49] Retyped "> install " / "> install v" / "> install": list includes "Shell Command: Install 'codium' command in PATH" and "Shell Command: Uninstall 'codium' command from PATH". At 04:48 the Install item is highlighted.
- [04:49] Modal dialog: "Shell command 'codium' successfully installed in PATH." with OK button.
- [04:50] Dialog fading out (OK pressed).
- [04:51] VSCodium window closed; empty grey desktop with only the webcam.
- [04:52] macOS dock appears at the bottom (Finder, Launchpad, a files-like app, a browser, Terminal (tooltip "Terminal"), a magnifier-like app, an "exec" app, a document thumbnail, Trash). He clicks Terminal.
- [04:53] New Terminal window opens (white, prompt "~ $").
- [04:58] Terminal window dragged to the left.
- [04:59] Terminal shows: `~ $ cd lab`, `lab $ ls` -> `a.out  main.c`, `lab $ rm *` -> zsh asks "sure you want to delete all 2 files in <account path>/lab [yn]?". He is clearing the lab folder.
- [05:02] Terminal cleared; typing `lab $ c` ...
- [05:03] Terminal shows `lab $ codium .` then a new prompt `lab $` (launches VSCodium on the current folder).
- [05:09] New VSCodium window titled "lab" opens on the now-empty folder (explorer header LAB, empty). Watermark (VSCodium coral logo) and hints "Show All Commands shift cmd P", "Open Settings cmd ,", "Go to File cmd P". UI is very bare: no tabs, no activity bar, no status bar.
- [05:11] main.c created: `#include <stdio.h>` and cursor on next line. Explorer shows main.c.
- [05:12] Typing `int main(void)`.
- [05:13] `int main(void) {` with auto-closed `}` and caret on indented line.
- [05:20 frame k_05m11s..k_05m13s done; next list frames:] k_05m12s shows `int main(void) {` / caret / `}`; k_05m13s shows `    printf` being typed.
- [05:20] Full program present: `printf("Hello World\n");`, blank line, `return 0;`. Terminal panel opened at bottom, empty with a solid blue block cursor.
- [05:35] Explorer now lists a.out and main.c; editor has new line `// Look good now, beauty, clean` between printf and return. Terminal shows `lab $ cc main.c`, `lab $ ./a.out`, `Hello World`, `lab $`. Window slightly offset (a zoom/pan transition under way).
- [05:36] Same content, window shifted further (zoom-out animation).
- [05:37] Final frame: macOS menu bar becomes partly visible at the top (VSCodium, File, Edit, Selection, View, Go, Run, Terminal, Window, Help; clock text cut off). Same lab/main.c state. Video ends.

## Texts on screen

### Command palette (04:22)
```
Preferences: Open User Settings (JSON)      recently used
View: Toggle Editor Sticky Scroll
View: Toggle Tree Sticky Scroll
Preferences: File Icon Theme
Preferences: Color Theme                    cmd K cmd T
File: Open Folder...
Accounts: Manage Accounts                   other commands
Accounts: Manage Extension Account Preferences...
Accounts: Manage Trusted Extensions For Account
Add Cursor Above
Add Cursor Below
```

### Quick Open recent list (04:24)
```
cat.c  user                      recently opened
settings.json  ~/Library/Application Support/VSCodium/User
```

### Color theme picker (04:27-04:37)
```
Select Color Theme for High Contrast Light Mode
high contrast themes:  Dark High Contrast (Default High Contrast)
                       Light High Contrast (Default High Contrast Light)
light themes:          Light 2026 (Default Light)
                       Light (VSCodium) (Visual Studio Light)
                       Light Modern
                       Light+
                       Quiet Light
                       Solarized Light
dark themes:           Dark 2026 (Default Dark)
                       Abyss
                       Dark (VSCodium) (Visual Studio Dark)  ?? (list continues)
```

### Panel and terminal toolbar context menus (04:06-04:16)
```
[Panel tab menu]
Configure Keybinding
Hide 'Debug Console'
Hide Badge
Move To >
-----
Problems
Output
v Debug Console     <- unticked at 04:06
v Terminal
v Ports             <- unticked at 04:08
-----
Panel Position >
Align Panel >
Show Icons
Hide Panel   cmd J

[Terminal toolbar menu]
Configure Keybinding
Hide 'New Terminal'
-----
v Focus Terminal    <- unticked at 04:10
v New Terminal
v Split Terminal    <- unticked at 04:13
v Kill Terminal     <- unticked at 04:15
Scroll to Previous Command
Scroll to Next Command
Clear Terminal
Run Active File
Run Selected Text
Start Dictation
-----
Reset Menu
```
Net effect: Debug Console, Ports, Focus Terminal (zsh label), Split Terminal and Kill Terminal controls are removed; only New Terminal (+/v), "...", maximize and close remain.

### Palette results for "install" (04:44-04:49)
```
> install
Extensions: Disable All Installed Extensions
Extensions: Disable All Installed Extensions for this Workspace
Extensions: Install Extensions
Extensions: Install from VSIX...
Extensions: Install Missing Dependencies
Extensions: Install Specific Version of Extension...
Extensions: Show Installed Extensions
Shell Command: Install 'codium' command in PATH      <- chosen
Shell Command: Uninstall 'codium' command from PATH
Developer: Install Extension from Location...
```

### Dialog (04:49-04:50)
```
Shell command 'codium' successfully installed in PATH.
[ OK ]
```

### Terminal session (04:59-05:03)
```
~ $ cd lab
lab $ ls
a.out  main.c
lab $ rm *
zsh: sure you want to delete all 2 files in <account path>/lab [yn]?
...
lab $ codium .
lab $
```

### Welcome hints in empty "lab" window (05:09)
```
Show All Commands    shift cmd P
Open Settings        cmd ,
Go to File           cmd P
```

### Integrated terminal in lab (05:35-05:37)
```
lab $ cc main.c
lab $ ./a.out
Hello World
lab $
```

## Code & shell

### cat.c (xv6-riscv user/cat.c), 04:06-04:49; no line numbers shown in the editor
```
char buf[512];

void
cat(int fd)
{
  int n;

  while ((n = read(fd, buf, sizeof(buf))) > 0) {
    if (write(1, buf, n) != n) {
      fprintf(2, "cat: write error\n");
      exit(1);
    }
  }
  if (n < 0) {
    fprintf(2, "cat: read error\n");
    exit(1);
  }
}

int
main(int argc, char *argv[])
{
  int fd, i;

  if (argc <= 1) {
    cat(0);
    exit(0);
  }

  for (i = 1; i < argc; i++) {
    if ((fd = open(argv[i], O_RDONLY)) < 0) {
      fprintf(2, "cat: cannot open %s\n", argv[i]);
      exit(1);
```
(rest of main is below the visible edge: ?? (not visible))

### main.c in the new "lab" folder, final version (05:35-05:37); no line numbers shown
```
#include <stdio.h>

int main(void) {
    printf("Hello World\n");
    // Look good now, beauty, clean
    return 0;
}
```
Intermediate states: 05:11 only `#include <stdio.h>` plus blank line; 05:12 `int main(void)`; 05:13 `int main(void) {` and `}`; 05:20 (k_05m13s) `    printf`; later frame (k_05m20s) full program with `printf("Hello World\n");`, an empty line, `return 0;`.

## Observations
- This final segment is a clean-up and "look how simple it is" demonstration, not an install walkthrough. For the first ~45 s he strips VSCodium's bottom panel: removes the Debug Console and Ports tabs, hides Focus Terminal / Split Terminal / Kill Terminal buttons from the terminal toolbar (the "zsh" label vanishes too), then hides the panel. No reason is written on screen for these removals; the visible intent is fewer buttons and less UI noise.
- He browses the colour theme picker (Light+, Quiet Light, Light (VSCodium), Light 2026, Light Modern) with live preview but ends without a visible change.
- The command palette (shift cmd P) is his central tool; he shows Sticky Scroll toggles and "Open User Settings (JSON)". The user settings.json under ~/Library/Application Support/VSCodium/User was opened earlier, i.e. he configures by editing JSON.
- Key demonstration: "Shell Command: Install 'codium' command in PATH". He then closes the editor completely, opens the system Terminal, goes into a throwaway `lab` folder, deletes everything with `rm *` (zsh confirmation), and launches the editor from the shell with `codium .`. Workflow: terminal first, editor opened on demand from the current directory.
- In the fresh "lab" window the UI is almost empty: no tabs, activity bar, status bar or line numbers, just explorer, editor and the three welcome hints. He writes Hello World in C live, compiles with `cc main.c`, runs `./a.out` in the integrated terminal ("Hello World"), and adds the on-screen comment "// Look good now, beauty, clean" as the punchline that the stripped-down setup looks good, beautiful and clean.
- Conclusion: minimalism and control. Fewer UI elements, an open-source build without telemetry (VSCodium), palette plus shell launch instead of menus, plain C and `cc`. There is no closing slide or sticky note; the video ends on the editor with the comment line while the screen zooms out and the macOS menu bar appears. The webcam shows him talking calmly throughout, changing only in head position.
