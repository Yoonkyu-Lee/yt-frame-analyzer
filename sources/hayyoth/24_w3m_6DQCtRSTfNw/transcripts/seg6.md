# Segment 6 (8:59 - 10:58) - w3m video, Hayyoth

## Timeline
- [08:59] Terminal running w3m on the local copy of "Beej's Guide to C Programming - Library Reference" (v0.9.13, Copyright October 15, 2026). TOC shown. Bottom prompt `Forward: ` (w3m search). Sticky note: "i often use w3m like this".
- [09:01] Search typed `Forward: stdio`; cursor lands on `23 <stdio.h> Standard I/O Library` in the TOC.
- [09:02] Status line `<< ^ v Viewing <Beej's Guide to C Programming>`; cursor on link `23.6 fflush()`.
- [09:03] Link followed: end of an fclose() example, "See Also fopen()", then `23.6 fflush()` section (see Texts).
- [09:05] Sticky note being edited: "y".
- [09:08] Sticky note: "yeah, offline".
- [09:09] w3m quit; terminal shows earlier commands: `cd Downloads`, `ls`, `cd bgclr`, `ls` (list of the guide's *.html files), `w3m index.html`. Prompt `bgclr $`.
- [09:10] Terminal cleared.
- [09:12] A Firefox window (beej.us/guide/bgclr/) briefly overlaps the terminal: download list (HTML, PDF variants, Examples, "What's Here for Translators and Writers"). Shows where the offline copy came from.
- [09:13] Firefox gone, terminal empty.
- [09:17] vim on `.zshrc` (19L, 603B): his shell config with aliases (see Code & shell).
- [09:19] Last line (alias crawl=...) selected in VISUAL mode.
- [09:28] Selection cleared. Sticky note changes to: "crawl all webpages to work offline".
- [09:30] Shell: `cd`, `e .zshrc`, prompt `~ $`.
- [09:35] `cd Downloads`, `crawl wingram.org` - wget output starts (301 Moved Permanently to https).
- [09:37] wget awaiting HTTPS response.
- [09:38] 200 OK, saving wingram.org/index.html; "Loading robots.txt; please ignore errors."
- [09:40] robots.txt fetched and removed.
- [09:42] about/wingram/index.html saved.
- [09:44] about/father/ requested.
- [09:45] about/father/index.html saved.
- [09:47] about/sites/index.html saved; FINISHED summary: 5 files, 26K; links converted in 4 files. Prompt `Downloads $`.
- [09:48] Screen cleared.
- [09:51] `ls` -> bgclr, bgclr.zip, wingram.org; `cd wingram.org`; `ls` -> about, index.html.
- [09:56] w3m opens local wingram.org index.html (Wingram.Org home page).
- [09:58] "Wingram Philosophy" page.
- [09:59] Back on home page; cursor on "Wingram Philosophy" link.
- [10:01] "Wingram Father" page.
- [10:11] w3m closed; `ls about` -> father sites wingram; `w3m index.html`. Sticky note: "crazy fast" / blank line / "no wifi".
- [10:12] Terminal cleared.
- [10:20] `cd`, `cd .w3m`, `ls` -> bookmark.html config cookie history. Sticky note being retyped: "ev".
- [10:29] vim on `history` ("history" 31L, 1450B). Sticky note: "everything storage in files".
- [10:30] Scrolled to the end of history (cursor line 27, Bot): last lines are local file URLs for bgclr and wingram.org pages.
- [10:31] Cursor on line 28, back at Top view.
- [10:33] Shell: `e history`, then typing `e`.
- [10:36] vim on `config` ("config" 167L, 2992B), top.
- [10:37] Scrolled down in config (18%).
- [10:38] Back to top of config, cursor line 28.
- [10:40] Shell: `e history`, `e config`, typing `e `.
- [10:41] vim on `bookmark.html` (11L, 330B).
- [10:44] Shell shows `e bookmark.html` done.
- [10:46] `w3m bookmark.html` renders "Bookmarks / Miscellaneous" with two links.
- [10:48] "Hayyoth" link selected; status "hayyoth.org contacted. Waiting for reply..."
- [10:49] Hayyoth page loads over SSL: "White Paper" table (see Texts).
- [10:51] Scrolled to bottom: Videos list and footer links.
- [10:52] Back to top.
- [10:54] `q` pressed: "Do you want to exit w3m? (y/n)".
- [10:55] Exited; shell shows `w3m bookmark.html` as last command; mouse pointer over sticky note.
- [10:58] macOS menu bar visible (Stickies app active). Sticky note: "try it". Terminal idle at `.w3m $`.

## Texts on screen

### Sticky note sequence (right side)
```
9:00-9:03    i often use w3m like this
9:05         y
9:08-9:19    yeah, offline
9:28-10:01   crawl all webpages to work offline
10:11-10:12  crazy fast

             no wifi
10:20        ev            (being typed)
10:29-10:55  everything storage in files
10:58        try it
```

### w3m: Beej's Guide TOC (8:59)
```
Prev | Contents | Next
Beej's Guide to C Programming
Library Reference
Brian "Beej Jorgensen" Hall
v0.9.13, Copyright (c) October 15, 2026
 * 1 Foreword
     - 1.1 Audience
     - 1.2 How to Read This Book
     - 1.3 Platform and Compiler
     - 1.4 Official Homepage
     - 1.5 Email Policy
     - 1.6 Mirroring
     - 1.7 Note for Translators
     - 1.8 Copyright and Distribution
     - 1.9 Dedication
 * 2 The C Language
     - 2.1 Background
         * 2.1.1 Comments
         * 2.1.2 Separators
         * 2.1.3 Expressions
         * 2.1.4 Statements
         * 2.1.5 Booleans
Forward:
```
TOC lower part at 9:01-9:02:
```
    20.1 Example
    20.2 _Bool?
 * 21 <stddef.h> A Few Standard Definitions
    21.1 ptrdiff_t / 21.2 size_t / 21.3 max_align_t / 21.4 wchar_t / 21.5 offsetof
 * 22 <stdint.h> More Integer Types
    22.1 Specific-Width Integers / 22.2 Other Integer Types / 22.3 Macros
    22.4 Other Limits / 22.5 Macros for Declaring Constants
 * 23 <stdio.h> Standard I/O Library
    23.1 remove()  23.2 rename()  23.3 tmpfile()  23.4 tmpnam()  23.5 fclose()
    23.6 fflush()  23.7 fopen()  23.8 freopen()  23.9 setbuf(), setvbuf()
    23.10 printf(), fprintf(), sprintf(), snprintf()
    23.11 scanf(), fscanf(), sscanf()
    23.12 vprintf(), vfprintf(), vsprintf(), vsnprintf()
    23.13 vscanf(), vfscanf(), vsscanf()
```

### w3m: 23.6 fflush() page (9:03-9:10)
```
    if (fp == NULL) {
        printf("Error opening file\n");
    } else {
        printf("Opened file just fine!\n");
        fclose(fp);  // All done!
    }
 }

See Also

fopen()

------------------------------------------------------------

23.6 fflush()

Process all buffered I/O for a stream right now

Synopsis

 #include <stdio.h>

 int fflush(FILE *stream);

Description

When you do standard I/O, as mentioned in the section on the setvbuf()
function, it is usually stored in a buffer until a line has been entered or
```

### Firefox page seen briefly (9:12), beej.us/guide/bgclr/
```
HTML
HTML, widescreen
HTML, single page
HTML, single page, widescreen
HTML ZIP
HTML ZIP, widescreen
PDF:
US Letter, one sided, syntax highlighting
US Letter, two sided, syntax highlighting
A4, one sided, syntax highlighting
A4, two sided, syntax highlighting
US Letter, one sided, black and white
US Letter, two sided, black and white
A4, one sided, black and white
A4, two sided, black and white
Examples:
Loose C example downloads (Run make to build.)
ZIP file
What's Here for Translators and Writers:
Clone the whole thing from GitHub and follow the README.
```

### Local wingram.org pages rendered in w3m (9:56-10:01)
Home:
```
[logo] Wingram.Org
 * Wingram.Org
     - About
Wingram.Org
Wingram on the way we find the Truth.
------------------------------------------
About
 * Wingram Philosophy
 * Wingram Father
 * Sites - Contact
------------------------------------------
Don't forget bookmark this site.
You can forget this site, but you can't forget an Ideology
Keyboard Shortcuts
(Close)
Keys     Action
```
Wingram Philosophy:
```
[logo] Wingram.Org
 * Wingram Philosophy
     - Why, how and what we need to find?
     - How do we find the Truth?
     - Faith gives strength
Wingram Philosophy
Before we can answer, we need the question
------------------------------------------
Why, how and what we need to find?
 * What's Life and Life Purpose?
 * How and Why to Live?
 * What's Universe, what created it?
 * What question need to find?
How do we find the Truth?
 * Knowledge is all we have
 * Philosophy help we continue
 * Science is good method
 * Computer is great tool
Faith gives strength
```
Wingram Father:
```
[logo] Wingram.Org
 * Wingram Father
     - I had a strange dream
     - How I help you?
     - I quit anything
     - What i do today?
Wingram Father
I'm just one person who wants to make something good for the world
Wingram Father
------------------------------------------
I had a strange dream
I found myself standing before an old, mysterious man. He offered me a choice:
a lot of money or an Book. The Book was encrypted and he didn't tell me what
was in it, he said I had to find out for myself. I was very curious about what
was hidden inside the Book, and I chose the Book.
How I help you?
 * I don't go to school to teach students knowledge
 * I don't go to church to teach people right and wrong
 * I go out on the street to help a random person change their mind
```

### ~/.w3m/bookmark.html in vim (10:41)
```
<html><head><title>Bookmarks</title></head>
<body>
<h1>Bookmarks</h1>
<h2>Miscellaneous</h2>
<ul>
<li><a href="https://github.com/torvalds/linux">GitHub - torvalds/linux: Linux ⟨?⟩
<li><a href="https://hayyoth.org/">Hayyoth</a>
<!--End of section (do not delete this comment)-->
</ul>
</body>
</html>
```
Rendered in w3m (10:46):
```
Bookmarks
Miscellaneous
 * GitHub - torvalds/linux: Linux kernel source tree · GitHub
 * Hayyoth
```

### hayyoth.org page rendered in w3m (10:49-10:52)
```
Hayyoth

- White Paper -

Understand the Game. Don't Play It Blindly. The Art of Quitting.

Year   Everyone                    I                          Outcome
2018   Studied to get into a       Studied to understand      Earned direct admission
       prestigious university      the world                  to university
2021   Pursued academic            Taught myself              Found a job
       credentials                 programming
2023   Wanted a corporate IT       Turned toward software     Freedom and independence
       job                         freedom - Old Git
2025   Wanted long-term job        Left the company again     Remained independent
       security
2025   Rushed toward AI and        Turned to farming          Gained control over my
       technology                                             life and time
2026   Was fascinated by what      Explored human nature,     Gained people's attention
       AI could create             trust, and attention
2026   Chased audience growth      Just do whatever I         Refused to let the
       and algorithmic             want                       algorithm dictate my
       attention                                              direction
Now    ...                         Quit quitting              ...
------------------------------------------------------------
Videos
 * Opinions on AI
 * Build your own X from scratch
 * x - but better
 * Learn C
 * Data Structures in C
 * Golang
 * Lisp Language
 * Web Development
------------------------------------------------------------
Youtube - Codeberg - Email
------------------------------------------------------------
```
(w3m status line: `<< ^ v Viewing[SSL] <Hayyoth>`; at 10:54 "Do you want to exit w3m? (y/n)".)

## Code & shell

### ~/.zshrc in vim (9:17-9:28), 19L, 603B; long lines cut off at window edge
```
export CLICOLOR=1
export LSCOLORS=Gxfxcxdxbxegedabagacad

export PATH="$PATH:$HOME/go/bin"
export WWW_HOME="https://lite.duckduckgo.com/lite"

PROMPT="%1~ $ "
#PROMPT="%B%F{blue}%1~ $ %f%b"

alias e="vim"
alias cc="clang -Wall -Wextra -pedantic -Wno-unused-parameter -Wno-unused-varia⟨?⟩
alias l="ls -a"
alias ll="du -sh * | sort -h"
alias web="open -a Firefox"
alias hay="open -a Firefox https://www.youtube.com/@hayyoth-org"
alias cam="scam start"

alias crawl='wget --recursive --page-requisites --adjust-extension --convert-li⟨?⟩
```
(Vim status in VISUAL mode: "-- VISUAL --  2  19,0-1  All". Rest of the crawl alias is off-screen: ?? (...))

### Shell sessions
9:09:
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
bgclr $ w3m index.html
```
9:30-9:47:
```
bgclr $ cd
~ $ e .zshrc
~ $ cd Downloads
Downloads $ crawl wingram.org
Prepended http:// to 'wingram.org'
--2026-11-08 10:25:55--  http://wingram.org/
Resolving wingram.org (wingram.org)... 104.21.94.109, 172.67.222.185
Connecting to wingram.org (wingram.org)|104.21.94.109|:80... connected.
HTTP request sent, awaiting response... 301 Moved Permanently
Location: https://wingram.org/ [following]
--2026-11-08 10:25:57--  https://wingram.org/
Connecting to wingram.org (wingram.org)|104.21.94.109|:443... connected.
HTTP request sent, awaiting response... 200 OK
Length: unspecified [text/html]
Saving to: 'wingram.org/index.html'

wingram.org/index.h     [ <=>                ]   5.57K  --.-KB/s    in 0s

2026-11-08 10:25:58 (11.8 MB/s) - 'wingram.org/index.html' saved [5706]

Loading robots.txt; please ignore errors.
--2026-11-08 10:26:00--  https://wingram.org/robots.txt
Reusing existing connection to wingram.org:443.
HTTP request sent, awaiting response... 200 OK
Length: 1248 (1.2K) [text/plain]
Saving to: 'wingram.org/robots.txt.tmp'

wingram.org/robots. 100%[===================>]   1.22K  --.-KB/s    in 0s

2026-11-08 10:26:00 (5.02 MB/s) - 'wingram.org/robots.txt.tmp' saved [1248/1248]

Removing wingram.org/robots.txt.tmp.
--2026-11-08 10:26:02--  https://wingram.org/about/wingram/
Reusing existing connection to wingram.org:443.
HTTP request sent, awaiting response... 200 OK
Length: unspecified [text/html]
Saving to: 'wingram.org/about/wingram/index.html'

wingram.org/about/w     [ <=>                ]   6.46K  --.-KB/s    in 0s

2026-11-08 10:26:03 (15.3 MB/s) - 'wingram.org/about/wingram/index.html' saved [6619]

--2026-11-08 10:26:05--  https://wingram.org/about/father/
Reusing existing connection to wingram.org:443.
HTTP request sent, awaiting response... 200 OK
Length: unspecified [text/html]
Saving to: 'wingram.org/about/father/index.html'

wingram.org/about/f     [ <=>                ]   7.18K  --.-KB/s    in 0s

2026-11-08 10:26:05 (23.7 MB/s) - 'wingram.org/about/father/index.html' saved [7352]

--2026-11-08 10:26:07--  https://wingram.org/about/sites/
Reusing existing connection to wingram.org:443.
HTTP request sent, awaiting response... 200 OK
Length: unspecified [text/html]
Saving to: 'wingram.org/about/sites/index.html'

wingram.org/about/s     [ <=>                ]   5.87K  --.-KB/s    in 0s

2026-11-08 10:26:07 (13.0 MB/s) - 'wingram.org/about/sites/index.html' saved [6013]

FINISHED --2026-11-08 10:26:07--
Total wall clock time: 12s
Downloaded: 5 files, 26K in 0.002s (13.9 MB/s)
Converting links in wingram.org/index.html... 20.
7-13
Converting links in wingram.org/about/father/index.html... 22.
8-14
Converting links in wingram.org/about/wingram/index.html... 20.
7-13
Converting links in wingram.org/about/sites/index.html... 20.
5-15
Converted links in 4 files in 0.005 seconds.
Downloads $
```
9:51-10:11:
```
Downloads $ ls
bgclr       bgclr.zip   wingram.org
Downloads $ cd wingram.org
wingram.org $ ls
about       index.html
wingram.org $ ls about
father  sites   wingram
wingram.org $ w3m index.html
wingram.org $
```
10:20-10:55:
```
wingram.org $ cd
~ $ cd .w3m
.w3m $ ls
bookmark.html config        cookie        history
.w3m $ e history
.w3m $ e config
.w3m $ e bookmark.html
.w3m $ w3m bookmark.html
.w3m $
```

### ~/.w3m/history in vim (10:29), "history" 31L, 1450B (local user name replaced)
First 27 lines as visible, then (10:30) the last lines:
```
https://w3m.sourceforge.net/
file:///Users/<account>/Downloads/wingram.org/wingram.org/index.html
file:///Users/<account>/Downloads/wingram.org/wingram.org/about/wingram/index.html
file:///Users/<account>/Downloads/hayyoth.org/index.html
file:///Users/<account>/Downloads/wingram.org/about/sites/index.html
http://ww19.sourceforget.net/
https://w3m.sourceforge.net/w3m
https://hayyoth.org/white-paper
https://wingram.org/
https://en.wikipedia.org/wiki/Hello,_world
https://en.wikipedia.org/wiki/Hello,_world#bodyContent
https://en.wikipedia.org/wiki/Hello_(Adele_song)
https://en.wikipedia.org/wiki/Hello_(Adele_song)#bodyContent
file:///Users/<account>/hayyoth.org
https://github.com/codegram01/gfm
https://github.com/codegram01/gfm#start-of-content
file:///$LIB/w3mbookmark
https://github.com/torvalds/linux
https://github.com/torvalds/linux#start-of-content
https://beej.us/guide/bgc/html/split/index.html
https://beej.us/guide/bgc/html/split/hello-world.html#hello-world-1
https://beej.us/guide/bgclr/html/split/
https://beej.us/guide/bgclr/html/split/stringref.html#stringref
https://lite.duckduckgo.com/lite
https://lite.duckduckgo.com/lite/
https://hayyoth.org/
file:///Users/<account>/Downloads/bgclr/index.html
file:///Users/<account>/Downloads/bgclr/stdio.html#man-fflush
file:///Users/<account>/Downloads/wingram.org/index.html
file:///Users/<account>/Downloads/wingram.org/about/wingram/index.html
file:///Users/<account>/Downloads/wingram.org/about/father/index.html
```
(The scrolled bottom view at 10:30 showed the final five lines: the bgclr index, bgclr stdio.html#man-fflush, then the three wingram.org pages.)

### ~/.w3m/config in vim (10:36-10:38), "config" 167L, 2992B (first ~52 lines seen)
```
tabstop 8
indent_incr 4
maxcols 0
pixel_per_char 8
frame 0
target_self 0
open_tab_blank 0
open_tab_dl_list 0
display_link 0
display_link_number 0
decode_url 0
display_lineinfo 0
display_column_number 0
ext_dirlist 1
dirlist_cmd file:///$LIB/dirlist.cgi
use_dictcommand 1
dictcommand file:///$LIB/w3mdict.cgi
dictprompt (dictionary)!
multicol 0
alt_entity 0
graphic_char 0
display_borders 0
disable_center 0
fold_textarea 0
fold_pre 0
display_ins_del 1
ignore_null_img_alt 1
view_unseenobject 1
display_image 0
pseudo_inlines 1
fold_line 0
show_lnum 0
show_srch_str 1
message_delay 2
label_topline 0
nextpage_topline 0
color 1
high-intensity 0
basic_color terminal
anchor_color blue
image_color green
form_color red
mark_color cyan
bg_color terminal
active_style 0
active_color cyan
visited_anchor 0
visited_color magenta
pagerline 10000
use_history 1
history 100
save_hist 1
confirm_qq 1
exit_on_last 0
```
(remaining ~115 lines not seen)

## Observations
- The segment shows w3m as an offline documentation reader: he searches (`Forward: stdio`) inside a locally stored copy of Beej's C library reference and jumps to fflush(). The sticky note "yeah, offline" makes the point: local HTML files, no network.
- He then shows the other half of the workflow: a self-made `crawl` alias (wget recursive mirror with page-requisites, adjust-extension, convert-links) to download a whole small site (wingram.org, 5 files, 12 s) and read it from disk in w3m. Sticky notes: "crawl all webpages to work offline", "crazy fast", "no wifi".
- Final theme "everything storage in files": he opens w3m's own state (history, config, bookmark.html) in vim to show they are plain text / HTML files, then renders bookmark.html in w3m and follows the Hayyoth bookmark to his own White Paper page ("Understand the Game. Don't Play It Blindly. The Art of Quitting."), a table contrasting "Everyone" vs "I" from 2018 to Now, ending with "Quit quitting". Closing sticky: "try it".
- Philosophy visible: simple tools, plain files, offline-first, owning your data, doing what you want instead of following the crowd or the algorithm (quitting jobs, farming, refusing algorithmic attention). Heavy keyboard / terminal / vim use (alias e="vim").
- Method: sticky notes typed live and rewritten as he talks; demonstration by running commands in a terminal and editing config files in vim. Webcam overlay ("Human") otherwise not relevant.
- Privacy: the local OS user name in the history file paths was replaced with <account>.
