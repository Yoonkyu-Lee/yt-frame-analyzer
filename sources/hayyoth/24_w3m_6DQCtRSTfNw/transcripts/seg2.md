# Segment 2 (1:48 - 3:41) - w3m demo: tabs, help page, option panel

## Timeline
- [01:48] Terminal window running w3m (tab bar at top). Tab 1 "Hello (Adele song) - Wikipedia", tab 2 "DuckDuckGo Lite: Fast, Lightweight Pri...". Page shows DuckDuckGo search box with empty field and [Search]. Prompt line "TEXT:" (input mode). Sticky note (right): "multiple pages". Webcam overlay only.
- [01:50] Typed "hayyoth" in the search field. Status bar: "<< ^ v Viewing[SSL] <DuckDuckGo Lite: Fast, Lightweight Private Search>".
- [01:51] Submitted; status "lite.duckduckgo.com contacted. Waiting for reply...".
- [01:52] Results page, tab title "hayyoth at DuckDuckGo". Results: 1. Hayyoth (hayyoth.org); 2. Hayyoth White Paper (hayyoth.org/white-paper); 3. hayyoth/hayyoth-lab: A small and fast C framework for building, running ...
- [01:53] A third tab opened (T): tab bar "[Hello (Adele song) - Wik][ hayyoth at DuckDuckGo ][ ...new tab ]"; new tab is again a DuckDuckGo Lite page copy.
- [01:55] New tab shows empty DuckDuckGo search; "TEXT:" prompt.
- [01:56] Typed "linus torvalds" in the new tab's search field, cursor on [Search].
- [02:02] Results for "linus torvalds" (tab 3 title "linus torvalds at DuckDuckGo"): Zero-click info (Wikipedia summary), 1. Official site.
- [02:04] Switches tabs with mouse hover on tab bar: back to tab 2 "hayyoth at DuckDuckGo" (pointer over the tab bar).
- [02:05] Tab 1 shown: Wikipedia "Hello (Adele song)" page rendered as text.
- [02:06] Tab 2 again; [02:07] tab 3 (linus torvalds) again. Demonstrates switching between multiple tabs.
- [02:13] Opened w3m help page (key H). "w3m (WWW-wo-Miru) Version 0.5.6 by A.ITO", sections In-page Navigation etc. Tab 4 titled "w3m help page".
- [02:26 - 02:35] Scrolls through the help page: In-page navigation, Hyperlink operations, Buffer operations, Tab operations, Miscellaneous, line-edit keys, Popup menu mode, Searches, Dictionary lookup, Mark operations. Sticky note at 02:26 changed to "good help / many shortcuts".
- [02:35] A scroll back to top then down; [02:38] File/Stream operations section visible.
- [02:43] Typed "Goto URL:" prompt: `file:///$LIB/w3mhelp.cgi?version=w3m%2F0.5.6&lang=e` (being edited); then at [02:44] a URL on hayyoth.org is contacted ("hayyoth.org contacted. Waiting for reply...").
- [02:53] hayyoth.org page loaded: "Hayyoth - White Paper - Understand the Game. Don't Play It Blindly. The Art of Quitting." with table (see Texts).
- [02:54] Pressed q: prompt "Do you want to exit w3m? (y/n)". Sticky note now: "crazy option config".
- [02:55] Back in shell: `~ $ w3m` (previous command) and new `~ $ w3m` typed. Terminal now plain, no tabs.
- [02:57] Launched w3m with no args: opens DuckDuckGo Lite directly (homepage), cursor at top.
- [03:01] Opened Option Setting Panel (key o): "Option Setting Panel (w3m version w3m/0.5.6)". Display Settings list visible.
- [03:02] Scrolled further (webcam: he is smoking, irrelevant) through display settings.
- [03:04] Scrolled to Color Settings, Miscellaneous Settings.
- [03:06] Directory Settings, External Program Settings.
- [03:07] Network Settings.
- [03:09] Proxy settings, SSL settings.
- [03:10] Left the option panel; back to DuckDuckGo start page.
- [03:16] Typed "gi" at TEXT: prompt; [03:17] field now "github <account> gfm" (see privacy note), cursor on first char.
- [03:18] Submit: "lite.duckduckgo.com contacted. Waiting for reply...".
- [03:20] Results for the search (github repo of a Go file manager "gfm").
- [03:21] Clicked result 1: "Opening socket..."
- [03:25] Resulting page shows only "gunzip: unknown compression format" (status "Viewing[SSL] <gfm>") - failed to load GitHub page (encoding problem).
- [03:26] Back in the Option Setting Panel (top). 
- [03:34] Pressed / and Forward: search prompt in page.
- [03:36] Searched "accept"; scrolled to Network Settings, cursor at "Accept-Language header" row. Sticky note adds second entry "search in page". Prompt shows "Forward: accept".
- [03:37] Cursor on "Accept-Encoding header" row (value "gzip, compress, bzip").
- [03:41] Edit prompt "TEXT:gzip, compress, bzip, bzip2, deflate" being typed in the Accept-Encoding field (editing the value); [03:43 equivalent final frame] Accept-Encoding field shows only "*" after edit (value replaced with "*" - last frame in segment).

## Texts on screen

### Sticky note (right), 01:48 - 02:26
```
multiple pages
```

### Sticky note, 02:26 - 02:54
```
good help
many shortcuts
```

### Sticky note, 02:54 - 03:34
```
crazy option config
```

### Sticky note, 03:34 - end of segment
```
crazy option config

search in page
```

### Search results page for "hayyoth", tab 2 (01:52 - 02:07)
```
DuckDuckGo
[hayyoth                                ] [Search]
[All Regions     ] [Any Time  ]
 [Next Page >]

1.  Hayyoth
    Hayyoth - White Paper - Understand the Game. Don't Play It Blindly. The
    Art of Quitting.
    hayyoth.org

2.  Hayyoth White Paper
    Hayyoth We stole Zeus's lightning, trapped it in stone, and taught it to
    think.
    hayyoth.org/white-paper

3.  hayyoth/hayyoth-lab: A small and fast C framework for building, running
    ...
```

### Search results page "linus torvalds", tab 3 (02:02 - 02:07)
```
Zero-click info: Linus Torvalds
Linus Benedict Torvalds is a Finnish and American software engineer who is the
creator and lead developer of the Linux kernel since 1991. He also created the
distributed version control system Git. Torvalds was one of the recipients of
the 2012 Millennium Technology Prize "in recognition of his creation of a new
open source operating system for computers leading to the widely used Linux
kernel". More at "Wikipedia"

1.  Official site
    Linus Torvalds
    torvalds-family.blogspot.com
```

### Wikipedia text rendering, tab 1 (02:05)
```
From Wikipedia, the free encyclopedia
2015 single by Adele

            "Hello"
A dark black-and-white image of Adele holding a telephone to her ear
          Single by Adele
          from the album 25
 Released     23 October 2015 (2015-10-23)
 Recorded     2015
 Studio       Metropolis (London)^[1]
 Genre        Soul^[2]
 Length       4:56 (album version)
              4:15 (radio edit)
 Label        . XL
              . Columbia
              . Adele Adkins
 Songwriters  . Greg Kurstin
 Producer     Greg Kurstin
          Adele singles chronology
"Skyfall" "Hello" "When We Were Young"
(2012)    (2015)  (2016)
      Music video
```

### hayyoth.org page rendered in w3m (02:53 - 02:54)
```
Hayyoth

- White Paper -

Understand the Game. Don't Play It Blindly. The Art of Quitting.

Year  Everyone                      I                          Outcome
2018  Studied to get into a         Studied to understand      Earned direct admission
      prestigious university        the world                  to university
2021  Pursued academic              Taught myself              Found a job
      credentials                   programming
2023  Wanted a corporate IT         Turned toward software     Freedom and independence
      job                           freedom - Old Git
2025  Wanted long-term job          Left the company again     Remained independent
      security
2025  Rushed toward AI and          Turned to farming          Gained control over my
      technology                                               life and time
2026  Was fascinated by what        Explored human nature,     Gained people's attention
      AI could create               trust, and attention
2026  Chased audience growth        Just do whatever I         Refused to let the
      and algorithmic               want                       algorithm dictate my
      attention                                                direction
Now   ...                           Quit quitting              ...
```
(Colored/linked cells in the original: "Studied to understand the world", "Turned toward software freedom - Old Git", "Explored human nature, trust, and attention", "White Paper".)

### w3m help page (02:13 - 02:38; read in the order displayed)
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
<NOT ASSIGNED>    Redraw screen with current line at top (LINE_TOP)
ESC-g             Go to the specified line (GOTO_LINE)
ESC-<,^[[1~,g     Go to the first line (BEGIN)
ESC->,G,^[[4~     Go to the last line (END)
w                 Move to the next word (NEXT_WORD)
W                 Move to the previous word (PREV_WORD)
[                 Move to the first hyperlink (LINK_BEGIN)
]                 Move to the last hyperlink (LINK_END)
TAB               Move to the next hyperlink (NEXT_LINK)
C-u,ESC-TAB,^[[Z  Move to the previous hyperlink (PREV_LINK)
ESC-m             Pop up menu to navigate between hyperlinks (MOVE_LIST_MENU)
<NOT ASSIGNED>    Move right to the next hyperlink (NEXT_RIGHT)
<NOT ASSIGNED>    Move left to the next hyperlink (NEXT_LEFT)
<NOT ASSIGNED>    Move downward to the next hyperlink (NEXT_DOWN)
<NOT ASSIGNED>    Move upward to the next hyperlink (NEXT_UP)
<NOT ASSIGNED>    Move right or downward to the next hyperlink (NEXT_RIGHT_DOWN)
<NOT ASSIGNED>    Move left or upward to the next hyperlink (NEXT_LEFT_UP)
<NOT ASSIGNED>    Move to the next image hyperlink (NEXT_IMAGE)
<NOT ASSIGNED>    Move to the previous image hyperlink (PREV_IMAGE)
(                 Cancel the last cursor movement (UNDO)
)                 Cancel the last undo (REDO)
<NOT ASSIGNED>    Move cursor to the top line on the screen (CURSOR_TOP)
<NOT ASSIGNED>    Move cursor to the middle line on the screen (CURSOR_MIDDLE)
<NOT ASSIGNED>    Move cursor to the bottom line on the screen (CURSOR_BOTTOM)

Hyperlink Operations

C-j,C-m           Follow current hyperlink in a new buffer (GOTO_LINK)
C-t               Follow current hyperlink in a new tab (TAB_LINK)
ESC-C-j,ESC-C-m,a Save hyperlink target (SAVE_LINK)
I                 Display image in viewer (VIEW_IMAGE)
ESC-I             Save inline image (SAVE_IMAGE)
<NOT ASSIGNED>    Submit form (SUBMIT)
c                 Show current address (PEEK)
u                 Show target address (PEEK_LINK)
i                 Show image address (PEEK_IMG)
<NOT ASSIGNED>    Pop up accesskey menu (ACCESSKEY)

File/Stream Operations

U                 Open specified document in a new buffer (GOTO)
ESC-u             Go to relative address (GOTO_RELATIVE)
<NOT ASSIGNED>    Open specified document in a new tab (TAB_GOTO)
<NOT ASSIGNED>    Open relative address in a new tab (TAB_GOTO_RELATIVE)
V                 Open local file in a new buffer (LOAD)
@                 Execute shell command and display output (READ_SHELL)
#                 Execute shell command and display output (PIPE_SHELL)
|                 Pipe current buffer through a shell command and display outpu...

Buffer Operations

B                 Close current buffer and return to the one below in stack (BA...
<NOT ASSIGNED>    Switch to the next buffer (NEXT)
<NOT ASSIGNED>    Switch to the previous buffer (PREV)
C-_               Return to the homepage (specified HTTP_HOME or WWW_HOME varia...
s                 Pop up buffer-stack menu (SELECT_MENU)
<NOT ASSIGNED>    Display buffer-stack panel (SELECT)
v                 Toggle between HTML shown or processed (VIEW)
ESC-s             Save document source (SAVE)
S                 Save rendered document (SAVE_SCREEN)
E                 Edit local source (EDIT)
ESC-e             Edit rendered copy of document (EDIT_SCREEN)
R                 Load current document anew (RELOAD)
<NOT ASSIGNED>    Re-render document (RESHAPE)
C-l               Draw the screen anew (REDRAW)
<NOT ASSIGNED>    Restart loading and drawing of images (DISPLAY_IMAGE)
<NOT ASSIGNED>    Stop loading and drawing of images (STOP_IMAGE)

Tab Operations

T                 Open a new tab (with current document) (NEW_TAB)
C-q               Close tab (CLOSE_TAB)
}                 Switch to the next tab (NEXT_TAB)
{                 Switch to the previous tab (PREV_TAB)
ESC-t             Pop up tab selection menu (TAB_MENU)
?? (a few lines between "Tab Operations" and "Searches" not seen: scrolled past)

Searches

/                 Search forward (WHEREIS)
?                 Search backward (SEARCH_BACK)
n                 Continue search forward (SEARCH_NEXT)
N                 Continue search backward (SEARCH_PREV)
C-s               Incremental search forward (ISEARCH)
C-r               Incremental search backward (ISEARCH_BACK)

Dictionary Lookup

ESC-w             Execute dictionary command (see README.dict) (DICT_WORD)
ESC-W             Execute dictionary command for word at cursor (DICT_WORD_AT)

Mark Operations

C-@               Set/unset mark (MARK)
ESC-n             Go to the next mark (NEXT_MARK)
ESC-p             Go to the previous mark (PREV_MARK)
"                 Mark all occurences of a pattern (REG_MARK)

Miscellaneous

^[[28~,^[[2~,^[[E,^[[L   Pop up menu (MENU)
H                 Show help panel (HELP)
o                 Display options setting panel (OPTIONS)
r                 Display the version of w3m (VERSION)
C-k               View cookie list (COOKIE)
C-h               Show browsing history (HISTORY)
<NOT ASSIGNED>    Save browsing history (SAVE_HISTORY)
<NOT ASSIGNED>    Store session (STORE)
D                 Display downloads panel (DOWNLOAD_LIST)
<NOT ASSIGNED>    Change the character encoding for the current document (CHARS...
<NOT ASSIGNED>    Change the default character encoding (DEFAULT_CHARSET)
<NOT ASSIGNED>    Display error messages (MSGS)
<NOT ASSIGNED>    Display a message (MESSAGE)
m                 Toggle mouse support (MOUSE_TOGGLE)
C-w               Toggle wrapping mode in searches (WRAP_TOGGLE)
ESC-c             Invoke w3m function(s) (COMMAND)
<NOT ASSIGNED>    Set alarm (ALARM)
ESC-o             Set option (SET_OPTION)

(line-edit mode keys, in the help page)
C-f   Cursor forward
C-b   Cursor backward
C-h   Delete previous character
C-d   Delete character
C-k   Delete everything after cursor
C-u   Delete everything before cursor
C-a   Cursor to the beginning of the line
C-e   Cursor to the end of the line
C-p   Fetch the previous string from the history list
C-n   Fetch the next string from the history list
C-o   Edit with external editor
SPC,TAB  Try to complete filename
RET   Accept input line

Popup Menu Mode

RET,RIGHT,SPC     Select item
C-c,^[[28~,^[[2~  Close menu
C-h,DEL,LEFT      One selection step backward
C-n,DOWN,j        Move to the next item
C-p,UP,k          Move to the previous item
K                 Scroll up one item
J                 Scroll down one item
C-a               Move to the first item
C-e               Move to the final item
C-f,C-v,^[[6~     Go to the next page
C-b,ESC-v,^[[5~   Go to the previous page
/,C-s             Search forward
?,C-r             Search backward
n                 Search for the next match
N                 Search for the previous match
C-z               Suspend
```

### Option Setting Panel (w3m/0.5.6) (03:01 - 03:41)
```
Option Setting Panel
(w3m version w3m/0.5.6)

[External Viewer Setup]

Display Settings

Tab width in characters                          [8]
Indent for HTML rendering                        [4]
Max chars per line (0: unset) Needs restart!     [0]
Number of pixels per character (4.0...32.0)      [8]
Render frames automatically                      ( )YES (*)NO
Use _self as default target                      ( )YES (*)NO
Open link on new tab if target is _blank or _new ( )YES (*)NO
Open download list panel on new tab              ( )YES (*)NO
Display link URL automatically                   ( )YES (*)NO
Display link numbers                             ( )YES (*)NO
Display decoded URL                              ( )YES (*)NO
Display current line information                 ( )YES (*)NO
Display column number in line information        ( )YES (*)NO
Use external program for directory listing       (*)YES ( )NO
URL of directory listing command                 [file:///$LIB/dirlist]
Enable dictionary lookup through CGI             (*)YES ( )NO
URL of dictionary lookup command                 [file:///$LIB/w3mdict]
Dictionary command prompt                        [(dictionary)!]
Display file names in multi-column format        ( )YES (*)NO
Use ASCII equivalents to display entities        ( )YES (*)NO
Character type for border of table and menu     [charset specific]
Display table borders, ignore value of BORDER    ( )YES (*)NO
Disable center alignment                         ( )YES (*)NO
Fold lines in TEXTAREA                           ( )YES (*)NO
Fold lines in PRE                                ( )YES (*)NO
Display INS, DEL, S and STRIKE element           [use tag]
Display link name for images lacking ALT         (*)YES ( )NO
Display unseen objects (e.g. bgimage tag)        (*)YES ( )NO
Display inline images                            ( )YES (*)NO
Display pseudo-ALTs for inline images with no ALT (*)YES ( )NO
or TITLE string
Fold lines of plain text file                    ( )YES (*)NO
Show line numbers                                ( )YES (*)NO
Show search string                               (*)YES ( )NO
Number of seconds to display user messages       [2]
Move cursor to top line when going to label      ( )YES (*)NO
Move cursor to top line when moving to next page ( )YES (*)NO
                                                 [OK]
--------------------
Color Settings

Display with color                               (*)YES ( )NO
Use high-intensity colors                        ( )YES (*)NO
Color of normal character                        [terminal]
Color of anchor                                  [blue]
Color of image link                              [green]
Color of form                                    [red]
Color of mark                                    [cyan]
Color of background                              [terminal]
Enable coloring of active link                   ( )YES (*)NO
Color of currently active link                   [cyan]
Use visited link color                           ( )YES (*)NO
Color of visited link                            [magenta]
                                                 [OK]
--------------------
Miscellaneous Settings

Number of remembered lines when used as a pager  [10000]
Use URL history                                  (*)YES ( )NO
Number of remembered URL                         [100]
?? (a few lines not seen)

Directory Settings

Directory corresponding to / (document root)     [ ]
Directory corresponding to /~user                [ ]
Directory corresponding to /cgi-bin              [ ]
Index file for directories                       [ ]
Directory for temporary files                    [ ]
Default directory for downloads                  [ ]
                                                 [OK]
--------------------
External Program Settings

List of mime.types files                         [~/.mime.types, /opt/...]
List of mailcap files                            [~/.w3m/mailcap, /opt...]
List of urimethodmap files                       [~/.w3m/urimethodmap,...]
Editor                                           [/usr/bin/vi]
How to call Mailer for mailto URLs with options  [ignore options and use only the address]
Mailer                                           [ ]
External browser                                 [/usr/bin/firefox]
2nd..9th external browser                        [ ] (all empty)
Run external viewer in the background            (*)YES ( )NO
Use LESSOPEN                                     ( )YES (*)NO
                                                 [OK]

Network Settings

Password file                                    [~/.w3m/passwd]
Disable secret file security check               ( )YES (*)NO
Password for anonymous FTP (your mail address)   [ ]
Generate domain part of password for FTP         (*)YES ( )NO
File for setting form on loading                 [~/.w3m/pre_form]
File for preferences for each site               [~/.w3m/siteconf]
User-Agent identification string                 [ ]
Suppress `Referer:' header                       ( )YES (*)NO
Exclude pathname and query string from `Referer:'
header when cross domain communication           (*)YES ( )NO
Accept-Language header                           [en;q=1.0]
Accept-Encoding header                           [gzip, compress, bzip]
Accept header                                    [text/html, text/*;q=...]
Treat argument without scheme as URL             (*)YES ( )NO
Prepend https:// to URL automatically            (*)YES ( )NO
Default value for open-URL command               [current URL]
Number of redirections to follow                 [10]
Enable processing of meta-refresh tag            ( )YES (*)NO
Restrict connections only to localhost           ( )YES (*)NO
Order of name resolution                         [unspecified]
News server                                      [ ]
Mode of news server                              [ ]
Number of news messages                          [50]
                                                 [OK]
--------------------
Proxy Settings

Use proxy                                        (*)YES ( )NO
URL of HTTP proxy host                           [ ]
URL of HTTPS proxy host                          [ ]
URL of GOPHER proxy host                         [ ]
URL of FTP proxy host                            [ ]
Domains to be accessed directly (no proxy)       [ ]
Check noproxy by network address                 (*)YES ( )NO
Disable cache                                    ( )YES (*)NO
                                                 [OK]
--------------------
SSL Settings

List of forbidden SSL methods (2: SSLv2, 3:
SSLv3, t: TLSv1.0, 5: TLSv1.1, 6: TLSv1.2, 7:    [2, 3, t, 5]
```
Final Accept-Encoding edit (03:41): prompt "TEXT:gzip, compress, bzip, bzip2, deflate" then field shown as "[*" (value replaced by "*").

### Failed page load (03:21 - 03:25)
```
gunzip: unknown compression format
```
(status bar: Viewing[SSL] <gfm>)

## Code & shell

Shell:
```
~ $ w3m
~ $ w3m
```
(first line is the previously typed command visible in scrollback; second is the new launch at 02:55; launching with no URL opens the DuckDuckGo Lite homepage at 02:57.)

w3m prompts/keys seen:
- Search prompt "TEXT:" (type text in a form field), then Enter/submit.
- "Goto URL: file:///$LIB/w3mhelp.cgi?version=w3m%2F0.5.6&lang=e..." (pressing U / the help-url prompt, edited to point at a hayyoth.org URL).
- "Do you want to exit w3m? (y/n)" (q).
- "Forward: " search prompt (/), typed "accept".
- Status bar format: `<< ^ v Viewing[SSL] <page title>`.
- Mouse support: hovering/clicking the tab bar switches tabs.
- Keys demonstrated: T (new tab), H (help), o (option panel), q (quit), / (search).

## Observations
- The author is demonstrating w3m features one at a time and labels each with a yellow sticky note: "multiple pages" (tabs), "good help / many shortcuts" (H help page with long key list), "crazy option config" (o panel with huge number of settings), then "search in page" (/ search to find "accept").
- Tabs: he opens several tabs (Wikipedia Hello, hayyoth search, linus torvalds search) and switches among them with the mouse.
- He searches his own name/project ("hayyoth", then lands on hayyoth.org/white-paper) as a demo: the white-paper table is a compact life/philosophy statement: where "Everyone" studies for credentials, jobs, security, AI, attention, and "I" studied to understand the world, taught himself programming, left the company, turned to farming, explored human nature/trust/attention, just does whatever he wants, concluding "Quit quitting". Theme: refusing the herd path, independence, control over time, not letting the algorithm decide his direction.
- He also searches for a "github ... gfm" Go file manager (a public repo) and the GitHub page fails with "gunzip: unknown compression format", i.e. the Accept-Encoding header; he then goes to the option panel and edits the Accept-Encoding field to fix it (live debugging as a demonstration of "crazy option config"). Privacy: the GitHub account name in the search string and results is written as <account>.
- Style: pure terminal, no slides; notes appear when he moves to a new feature; casual (smoking on the webcam, not relevant). He shows the terminal as a minimal, keyboard-driven, text-first tool, consistent with preference for simplicity and small tools (he even surfaces his own C framework repo "hayyoth-lab: a small and fast C framework").
