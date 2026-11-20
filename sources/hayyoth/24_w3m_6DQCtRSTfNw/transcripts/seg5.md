# Segment 5 (07:13 - 08:59) - w3m help, DuckDuckGo search, hayyoth.org, sticky note, Beej's Guide offline

## Timeline
- [07:13] Terminal window running w3m on its "w3m help page" (key list: `H Show help panel (HELP)`, `o Display options setting panel (OPTIONS)`, `r ... VERSION`, `C-k View cookie list (COOKIE)`, `C-h Show browsing history`, `D Display downloads panel`, `m Toggle mouse support`, `C-w Toggle wrapping mode in searches`, `ESC-c Invoke w3m function(s) (COMMAND)`, `ESC-o Set option`, `ESC-k Define a binding...`, `! Execute shell command and display output (SHELL)`, `C-z Suspend w3m to background`, `q Quit with confirmation request (QUIT)`, `Q Quit without confirmation (EXIT)`, then heading "User-defined key bindings"). Status bar: `<w3m help page>`. Yellow sticky note at top right is empty.
- [07:14] Help page scrolled horizontally (key column off-screen); full descriptions visible, e.g. `Define a binding between a key stroke combination and a command (DEFINE_KEY)`.
- [07:15] Terminal now shows a different page: "Beej's Guide to C Programming" (status `Viewing[SSL] <Beej's Guide to C Programming>`), section `26.4 strcat(), strncat()`, with a puts/free code tail, "See Also" links `strcpy(), strncpy(), malloc(), free()`.
- [07:17] Prompt line at bottom: `command [; ...]: ` (w3m command prompt, empty).
- [07:20] A Firefox-style browser window (URL `beej.us/guide/bgclr/html/split/strir...`, "Not Secure") opens over the terminal, still blank/loading.
- [07:22] Browser shows "26 <string.h> String Manipulation" function table (memccpy, memchr, memcmp, memcpy, memmove, memset, memset_explicit, strcat, strchr, strcmp, strcoll, strcpy, strcspn, strdup...). URL now `beej.us/guide/bgclr/html/split/stringref.html#s...`.
- [07:24] Terminal brought to front again; browser behind.
- [07:25] Terminal status: `lite.duckduckgo.com contacted. Waiting for reply...` (he navigated w3m to DuckDuckGo lite).
- [07:26] Page "DuckDuckGo" with search box `[____] [Search]`; prompt `TEXT:h` (typing into the form field).
- [07:27] Field filled with `hayyoth`; status `Viewing[SSL] <DuckDuckGo Lite: Fast, Lightweight Private Search>`.
- [07:28] Submitted; `lite.duckduckgo.com contacted. Waiting for reply...`.
- [07:29] Results page `<hayyoth at DuckDuckGo>`: filters `[All Regions] [Any Time]`, `[Next Page >]`, results 1. Hayyoth (hayyoth.org), 2. Hayyoth White Paper (hayyoth.org/white-paper), 3. Living creatures (Bible) - Wikipedia.
- [07:31] Opened result 1: the hayyoth.org page in w3m (title "Hayyoth", "- White Paper -", tagline, year table with Everyone / I / Outcome columns). Links in blue.
- [07:34] Same page; bottom prompt `command [; ...]: ` again (empty).
- [07:36] Browser window (Firefox, `hayyoth.org`) appears over terminal showing the same page rendered graphically as an HTML table - for comparison with w3m's text rendering.
- [07:38] Terminal in front again, same hayyoth.org page in w3m.
- [07:40] w3m help page again: `******* w3m (WWW-wo-Miru) Version 0.5.6 by A.ITO ********`, `[Japanese] [German]`, `Current keymap file`, section "In-page Navigation" with key table (see below).
- [07:42] Sticky note (top right) gets focus; first typed text: `many ` (cursor after it).
- [08:07] Sticky note content now longer and scrolled: lines `many other features`, blank, `like open image`, `download`, `edit html`, `edit html render`, `open in external editor`, `man` (typing). Top of note scrolled out of view.
- [08:12] Sticky note cleared and retyped: `i open`.
- [08:18] Sticky note: `i often use w3m like this`. Terminal bottom prompt: `Do you want to exit w3m? (y/n)` (he pressed q).
- [08:19] Terminal shows shell: `~ $ w3m` twice then `~ $ ` (w3m exited back to the prompt; earlier runs in history).
- [08:21] Terminal cleared (`~ $`). Browser window in front on hayyoth.org; URL bar selected showing `https://hayyoth.org/` with suggestion list (Google Gemini, Telegram Web, Hayyoth - hayyoth.org, Hayyoth - YouTube, Google Translate).
- [08:22] Typing in URL bar: `beej.us/` with Firefox Suggest list (Beej's Web Page - beej.us; Beej's Guide to C ... beej.us/guide/bgclr/html/..., beej.us/guide/bgclr/, Beej's Guides beej.us/guide/, Google Translate entries).
- [08:23] URL `beej.us` entered, page still hayyoth.org; hovering "Studied to understand the world" shows a youtu.be link in the status bar.
- [08:24] beej.us loaded: "Beej's Home Page" with icon row, sections "Stuff to Learn" (Beej's Guides, My tech blog, Removing stuck Saturn brake rotors) and "Places to Visit" (The Bend Hackers Guild, The Pirate Image Archive, Graffiti Central Archive, ...Pizza Server).
- [08:27] `beej.us/guide/` - "Beej's Guides", "The Guides! Free downloads!" list. Presenter holds a pencil in his mouth on webcam.
- [08:28] Scrolled: C - Tutorial, C - Library Reference, Network Concepts, Git.
- [08:29] Scrolled to Git, Unix Interprocess Communication, Learning Computer Science.
- [08:30]-[08:32] Scrolling the guides list (Python Programming [WIP], GNU Debugger (GDB)...), then back up.
- [08:35] Clicked "Beej's Guide to C - Library Reference": URL `beej.us/guide/bgclr/`.
- [08:36] Scrolled to "What's Here for Readers:" - HTML: HTML, HTML widescreen, HTML single page, HTML single page widescreen, HTML ZIP, HTML ZIP widescreen; PDF: US Letter / A4 variants.
- [08:37] Further scroll: Examples, "What's Here for Translators and Writers".
- [08:38] Mouse hovering on "HTML ZIP".
- [08:42] Clicked HTML ZIP; browser window moves; download spinner in toolbar.
- [08:43] Download button highlighted (blue) in the toolbar.
- [08:44] Downloads panel shows `bgclr.zip - Open File`.
- [08:45] Panel closed, download icon in toolbar.
- [08:46] A Finder-style window "Downloads" opens: folder `bgclr` (33 items) and file `bgclr.zip` (496 KB); sidebar Favorites: <home folder>, Desktop, Documents, Downloads, sources, lab.
- [08:47] Terminal in front again, empty prompt `~ $`.
- [08:52] Commands typed: `cd Downloads`, `ls` (shows `bgclr  bgclr.zip`), `cd bgclr`, `ls` (being typed).
- [08:53] `ls` output (html file list) and `bgclr $ w3m` being typed.
- [08:55] w3m open on local doc: "Beej's Guide to C Programming - Library Reference - Brian "Beej Jorgensen" Hall - v0.9.13, Copyright (c) October 15, 2026" with table of contents.
- [08:56] Scrolled down in contents (2.1.5 Booleans ... 2.9.2 Type Qualifiers).
- [08:57] Scrolled further (4.7 cacosh() ... 5.10 isspace()).
- [08:58] Back at the top of the contents page.
- [08:59] Bottom prompt `Forward: ` (w3m search opened, empty).

## Texts on screen

### Sticky note (top right), versions seen
07:13-07:40: empty.

07:42:
```
many 
```
08:07 (partly scrolled; first lines hidden):
```
?? (earlier lines scrolled out)
many other features

like open image
download
edit html
edit html render
open in external editor
man
```
08:12 (retyped):
```
i open
```
08:18-08:59 (final in this segment):
```
i often use w3m like
this
```

### hayyoth.org page as rendered by w3m (07:31-08:18)
```
Hayyoth

- White Paper -

Understand the Game. Don't Play It Blindly. The Art of Quitting.

Year        Everyone                    I                          Outcome
2018 Studied to get into a      Studied to understand       Earned direct admission
     prestigious university     the world                   to university
2021 Pursued academic           Taught myself               Found a job
     credentials                programming
2023 Wanted a corporate IT      Turned toward software      Freedom and independence
     job                        freedom - Old Git
2025 Wanted long-term job       Left the company again      Remained independent
     security
2025 Rushed toward AI and       Turned to farming           Gained control over my
     technology                                             life and time
2026 Was fascinated by what     Explored human nature,      Gained people's attention
     AI could create            trust, and attention
2026 Chased audience growth     Just do whatever I want     Refused to let the
     and algorithmic attention                              algorithm dictate my
                                                            direction
Now  ...                        Quit quitting               ...
------------------------------------------------------------
Videos
```
(Links in w3m: "- White Paper -", "Studied to understand the world", "Turned toward software freedom", "Old Git", "Explored human nature, trust, and attention". Same table shown graphically in the Firefox window 07:36 and 08:21-08:23.)

### DuckDuckGo Lite results for the search (07:29)
```
DuckDuckGo
[hayyoth______________________] [Search]
[All Regions    ] [Any Time  ]
[Next Page >]

1. Hayyoth
   Hayyoth - White Paper - Understand the Game. Don't Play It Blindly. The Art of Quitting.
   hayyoth.org

2. Hayyoth White Paper
   Hayyoth We stole Zeus's lightning, trapped it in stone, and taught it to think.
   hayyoth.org/white-paper

3. Living creatures (Bible) - Wikipedia
   Ezekiel's "chariot vision", by Matthaeus Merian (1593-1650) Ezekiel's Wheel Ezekiel's encounter with the Merkabah and the Living Creatures The living creatures, living beings, or chayyoth (Hebrew: ⟨?⟩, romanized: ...
```

### w3m help page: In-page Navigation (07:40-08:12)
```
******* w3m (WWW-wo-Miru) Version 0.5.6 by A.ITO ********
[Japanese] [German]

Current keymap file

In-page Navigation

+,C-v,SPC,^[[6~   Scroll down one page (NEXT_PAGE)
-,ESC-v,^[[5~,b   Scroll up one page (PREV_PAGE)
C-f,RIGHT,l       Cursor right (MOVE_RIGHT)
C-b,LEFT,h        Cursor left (MOVE_LEFT)
C-n,DOWN,j        Cursor down (MOVE_DOWN)
C-p,UP,k          Cursor up (MOVE_UP)
<NOT ASSIGNED>    Cursor right. With edge touched, slide (MOVE_RIGHT1)
<NOT ASSIGNED>    Cursor left. With edge touched, slide (MOVE_LEFT1)
<NOT ASSIGNED>    Cursor down. With edge touched, slide (MOVE_DOWN1)
<NOT ASSIGNED>    Cursor up. With edge touched, slide (MOVE_UP1)
K                 Scroll the screen down one line (DOWN)
J                 Scroll the screen up one line (UP)
<                 Shift screen left (SHIFT_LEFT)
>                 Shift screen right (SHIFT_RIGHT)
,                 Shift screen one column left (LEFT)
.                 Shift screen one column right (RIGHT)
C-a,^             Go to the beginning of the line (LINE_BEGIN)
$,C-e             Go to the end of the line (LINE_END)
Z                 Center on cursor column (CENTER_H)
z                 Center on cursor line (CENTER_V)
<NOT ASSIGNED>    Redraw screen with current line at bottom (LINE_BOTTOM)
```

### w3m help page: general commands (07:13)
```
H               Show help panel (HELP)
o               Display options setting panel (OPTIONS)
r               Display the version of w3m (VERSION)
C-k             View cookie list (COOKIE)
C-h             Show browsing history (HISTORY)
<NOT ASSIGNED>  Save browsing history (SAVE_HISTORY)
<NOT ASSIGNED>  Store session (STORE)
D               Display downloads panel (DOWNLOAD_LIST)
<NOT ASSIGNED>  Change the character encoding for the current document (CHARS...
<NOT ASSIGNED>  Change the default character encoding (DEFAULT_CHARSET)
<NOT ASSIGNED>  Display error messages (MSGS)
<NOT ASSIGNED>  Display a message (MESSAGE)
m               Toggle mouse support (MOUSE_TOGGLE)
C-w             Toggle wrapping mode in searches (WRAP_TOGGLE)
ESC-c           Invoke w3m function(s) (COMMAND)
<NOT ASSIGNED>  Set alarm (ALARM)
ESC-o           Set option (SET_OPTION)
<NOT ASSIGNED>  Set environment variable (SETENV)
ESC-k           Define a binding between a key stroke combination and a comma...
<NOT ASSIGNED>  Reload configuration file (REINIT)
!               Execute shell command and display output (SHELL)
C-z             Suspend w3m to background (SUSPEND)
q               Quit with confirmation request (QUIT)
Q               Quit without confirmation (EXIT)

User-defined key bindings
```

### Beej's Guide, string.h page in w3m (07:15-07:25)
```
    if (s != NULL)
        puts(s);  // Hello, world!
    else
        puts("Error duplicating string");

    free(s);
}

See Also
strcpy(), strncpy(), malloc(), free()
--------------------------------------------------
26.4 strcat(), strncat()

Concatenate two strings into a single string

Synopsis

 #include <string.h>

 int strcat(const char *dest, const char *src);

 int strncat(const char *dest, const char *src, size_t n);

Description
```

### Beej's Guide library reference contents in local w3m (08:55-08:59)
```
Prev | Contents | Next
----------------------------------------
Beej's Guide to C Programming

Library Reference

Brian "Beej Jorgensen" Hall

v0.9.13, Copyright (c) October 15, 2026

 * 1 Foreword
     1.1 Audience
     1.2 How to Read This Book
     1.3 Platform and Compiler
     1.4 Official Homepage
     1.5 Email Policy
     1.6 Mirroring
     1.7 Note for Translators
     1.8 Copyright and Distribution
     1.9 Dedication
 * 2 The C Language
     2.1 Background
        2.1.1 Comments
        2.1.2 Separators
        2.1.3 Expressions
        2.1.4 Statements
        2.1.5 Booleans
        2.1.6 Blocks
        2.1.7 Code Examples
     2.2 Operators
        2.2.1 Arithmetic Operators
        2.2.2 Pre- and Post-Increment and -Decrement
        2.2.3 Comparison Operators
        2.2.4 Pointer Operators
        2.2.5 Structure and Union Operators
        2.2.6 Array Operators
        2.2.7 Bitwise Operators
        2.2.8 Assignment Operators
        2.2.9 The sizeof Operator
        2.2.10 Type Casts
        2.2.11 _Alignof Operator
        2.2.12 Comma Operator
     2.3 Type Specifiers
     2.4 Constant Types
     2.5 Composite Types
        2.5.1 struct Types
        2.5.2 union Types
        2.5.3 enum Types
     2.6 Initializers
     2.7 Compound Literals
     2.8 Type Aliases
     2.9 Additional Type-Related Specifiers
        2.9.1 Storage Class Specifiers
        2.9.2 Type Qualifiers
     ?? (2.9.3 - 4.6 not seen in this segment)
     4.7 cacosh(), cacoshf(), cacoshl()
     4.8 casinh(), casinhf(), casinhl()
     4.9 catanh(), catanhf(), catanhl()
     4.10 ccosh(), ccoshf(), ccoshl()
     4.11 csinh(), csinhf(), csinhl()
     4.12 ctanh(), ctanhf(), ctanhl()
     4.13 cexp(), cexpf(), cexpl()
     4.14 clog(), clogf(), clogl()
     4.15 cabs(), cabsf(), cabsl()
     4.16 cpow(), cpowf(), cpowl()
     4.17 csqrt(), csqrtf(), csqrtl()
     4.18 carg(), cargf(), cargl()
     4.19 cimag(), cimagf(), cimagl()
     4.20 CMPLX(), CMPLXF(), CMPLXL()
     4.21 conj(), conjf(), conjl()
     4.22 cproj(), cprojf(), cproj()
     4.23 creal(), crealf(), creall()
 * 5 <ctype.h> Character Classification and Conversion
     5.1 isalnum()  5.2 isalpha()  5.3 isblank()  5.4 iscntrl()  5.5 isdigit()
     5.6 isgraph()  5.7 islower()  5.8 isprint()  5.9 ispunct()  5.10 isspace()
     ?? (rest hidden)
```

### beej.us pages in the Firefox-style browser (08:24-08:38), summary of visible text
```
Beej's Home Page
Stuff to Learn: Beej's Guides / My tech blog / Removing stuck Saturn brake rotors
Places to Visit: The Bend Hackers Guild / The Pirate Image Archive / Graffiti Central Archive / ...Pizza Server

Beej's Guides  - "Like the guides? Here's the tip jar!  Thanks!"
The Guides! Free downloads!
- Beej's Guide to Network Programming
- Beej's Guide to C-Tutorial
- Beej's Guide to C-Library Reference
- Beej's Guide to Network Concepts
- Beej's Guide to Git ("...This isn't a cheat-sheet - the idea is that you'll form a foundation to build toward expert use!")
- Beej's Guide to Unix Interprocess Communication
- Beej's Guide to Learning Computer Science
- Beej's Guide to Python Programming [WIP]
- Beej's Guide to the GNU Debugger (GDB)

Beej's Guide to C Programming - Library Reference (beej.us/guide/bgclr/)
What's Here for Readers:
HTML: HTML | HTML, widescreen | HTML, single page | HTML, single page, widescreen | HTML ZIP | HTML ZIP, widescreen
PDF: US Letter / A4, one sided / two sided, syntax highlighting; US Letter / A4, one / two sided, black and white
Examples: Loose C example downloads (Run make to build.) | ZIP file
What's Here for Translators and Writers: Clone the whole thing from GitHub and follow the README.
```

## Code & shell
Terminal sessions (prompts `~ $`, `Downloads $`, `bgclr $`):
```
~ $ w3m
~ $ w3m
~ $
```
(08:19, shell after quitting w3m; the first two `w3m` lines are earlier history)

```
~ $ cd Downloads
Downloads $ ls
bgclr     bgclr.zip
Downloads $ cd bgclr
bgclr $ ls
assert.html     locale.html     stdio.html
complex.html    math.html       stdlib.html
ctype.html      setjmp.html     stdnoreturn.html
errno.html      signal.html     stringref.html
fenv.html       stdalign.html   tgmath.html
float.html      stdarg.html     the-c-language.html
foreword.html   stdatomic.html  threads.html
index.html      stdbit.html     time.html
inttypes.html   stdbool.html    uchar.html
iso646.html     stddef.html     wchar.html
limits.html     stdint.html     wctype.html
bgclr $ w3m
```
(08:52-08:53. At 08:53 `w3m` is typed with no argument visible; at 08:55 w3m shows the local library reference index, so the argument/file is not visible here.)

w3m prompts seen: `command [; ...]: ` (07:17, 07:34), `TEXT:h` (07:26), `Do you want to exit w3m? (y/n)` (08:18), `Forward: ` (08:59), `lite.duckduckgo.com contacted. Waiting for reply...` (07:25, 07:28).

Key presses are not shown on screen as overlay; they are inferred only from the prompts (command prompt, quit confirmation, search prompt).

## Observations
- Continues the w3m demo. He first shows the built-in w3m help (keybinding reference), then uses w3m for real: a DuckDuckGo Lite search for "hayyoth" (his own brand name), clicks into hayyoth.org and compares w3m's text rendering with the graphical Firefox rendering of the same table, showing the content survives in a text browser.
- He uses the sticky note to list w3m's other capabilities ("many other features like open image, download, edit html, edit html render, open in external editor, man...") and then to state his own use: "i often use w3m like this".
- Demonstrated workflow: download Beej's Guide to C Library Reference HTML ZIP in the graphical browser, `cd` into the extracted folder in the terminal, and read the local HTML in w3m. This ties w3m to learning C from reference docs offline with a terminal-centered, minimal-tools workflow.
- The hayyoth.org table (2018 studied for a prestigious university ... 2025 turned to farming ... 2026 refused to let the algorithm dictate direction ... Now: "Quit quitting") shows his public philosophy: independence, skepticism of credentials and algorithms, self-teaching.
- Style: live terminal use, no slides; webcam only shows him (pencil in mouth at 08:27).
