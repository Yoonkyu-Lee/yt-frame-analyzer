# Segment 3 (3:41 - 5:21) - w3m demo: option panel, DuckDuckGo search, GitHub pages, help page, bookmarks

## Timeline
- [03:41] Terminal window running w3m, "Network Settings" of the Option Setting Panel. Status bar: `« ↑ ↓ Viewing <Option Setting Panel>`. Yellow sticky note: "crazy option config / search in page". Cursor on "Prepend https:// to URL automatically".
- [03:42] Scrolled down: end of Network Settings, then "Proxy Settings" section.
- [03:44] Screen shows only `gunzip: unknown compression format`, status `Viewing[SSL] <gfm>` (failed page render).
- [03:46] DuckDuckGo (lite) results for query `github <account> gfm`: GitHub "Golang File Manager" repo, the account's GitHub page, Go packages page. Status `Viewing[SSL] <github <account> gfm at DuckDuckGo>`.
- [03:47] Same page, link 1 underlined, status `1.57kb loaded` (opening link 1).
- [03:48] GitHub repo page loads: "Skip to content", "Navigation Menu", Products/Platform/Build & Ship/Secure/Developers menus rendered as bullet lists with image alt-text like `[platform-4ce65ae0bf273ce2]`. First line shows stray "FFFF".
- [03:50] Scroll: GitHub Universe, Pricing, "(Search or jump to, type / to search)Search/", Sign in / Sign up, "Appearance settings", reload messages, "Dismiss alert", "{{ message }}", breadcrumb list, "Repository navigation".
- [03:52-03:54] Scroll: Actions, Projects, Security and quality, Insights; "More items"; repo header; About "Golang File Manager"; a youtu.be link; Topics; Resources; Stars 4; Watchers 1; Forks 1; "(main branch)"; "Go to file"; "Latest commit".
- [03:57-04:00] History "12 Commits"; Folders and files (.gitignore, LICENSE, README.md, go.mod); README body (Usage, Examples, Install) scrolling down then slightly back up.
- [04:05] Fresh DuckDuckGo lite page with empty search box and prompt `TEXT:`.
- [04:08-04:09] Typed `github linux kernel`; submitted: `lite.duckduckgo.com contacted. Waiting for reply...`.
- [04:10] Results: torvalds/linux, linux/kernel at master, The Linux Kernel Archives.
- [04:12] `Opening socket...` following result 1.
- [04:13] Linux GitHub page loads (same nav menu; stray `8000` at top).
- [04:15-04:17] Scroll: breadcrumb (torvalds / linux), Fork 67.3k, Star 255k, About "Linux kernel source tree", Resources, 254.8k stars. Webcam: he holds a cigarette near his mouth.
- [04:18-04:19] 8.5k watching, 67.3k forks, "(master branch)", "1,506,114 Commits", folder listing (Documentation ... .clippy.toml).
- [04:21-04:26] README: "Linux kernel", "Quick Start", "Essential Documentation"; then "Who Are You?", "For Specific Users", "New Kernel Developer".
- [04:32] Back to top of Linux page, then the w3m help page opened (`Viewing <w3m help page>`, "w3m (WWW-wo-Miru) Version 0.5.6 by A.ITO"). Sticky gets a third line "bookmark".
- [04:33] In help page, bottom prompt `Forward: boo` (typing a forward search, `/`).
- [04:34] `Forward: bookmark`; view jumped to "Bookmark Management" section.
- [04:37] Status back to `Viewing <w3m help page>`; mouse I-beam over "View bookmarks" line.
- [04:39] Back on the Linux GitHub page top.
- [04:45] "Register to my bookmark" form (URL = Linux GitHub URL, Title = page title), [ADD].
- [04:50] Back on the Linux page.
- [04:52-04:53] Status `lite.duckduckgo.com contacted. Waiting for reply...`, then blank DuckDuckGo lite page with `TEXT:`.
- [04:55-04:56] Typed `hayyoth`, cursor on [Search], submitted.
- [04:57] Results for "hayyoth": hayyoth.org, Hayyoth White Paper, hayyoth/hayyoth-lab C framework.
- [04:59] hayyoth.org page (White Paper table).
- [05:00] "Register to my bookmark" with Section `Miscellaneous`, URL `https://hayyoth.org/`, Title `Hayyoth`.
- [05:03] Back on hayyoth.org.
- [05:06] Bottom prompt `Goto URL: https://hayyoth.org/`.
- [05:08] hayyoth.org page, normal status.
- [05:09] "Bookmarks" page: Miscellaneous with Linux GitHub page and Hayyoth.
- [05:17] Linux GitHub page again, now with a raw `{"props": ...}` JSON blob under "Skip to content". Sticky replaced by single line "see bookmark".
- [05:19-05:21] Scrolling Linux page: Universe, breadcrumb, "8.5k watching", repo files list (.clippy.toml ... README), "Repository files navigation", "Linux kernel".

## Texts on screen

### Sticky note (3:41 - ~4:32)
```
crazy option config
search in page
```

### Sticky note (~4:32 - ~5:09)
```
crazy option config
search in page
bookmark
```

### Sticky note (~5:17 - 5:21)
```
see bookmark
```

### w3m Option Setting Panel, Network Settings (3:41)
```
Network Settings

Password file                                      [~/.w3m/passwd        ]
Disable secret file security check                 ( )YES  (*)NO
Password for anonymous FTP (your mail address)     [                     ]
Generate domain part of password for FTP           (*)YES  ( )NO
File for setting form on loading                   [~/.w3m/pre_form      ]
File for preferences for each site                 [~/.w3m/siteconf      ]
User-Agent identification string                   [                     ]
Suppress `Referer:' header                         ( )YES  (*)NO
Exclude pathname and query string from `Referer:'  (*)YES  ( )NO
header when cross domain communication
Accept-Language header                             [en;q=1.0             ]
Accept-Encoding header                             [*                    ]
Accept header                                      [text/html, text/*;q= ]
Treat argument without scheme as URL               (*)YES  ( )NO
Prepend https:// to URL automatically              (*)YES  ( )NO
Default value for open-URL command                 [current URL]
Number of redirections to follow                   [10                   ]
Enable processing of meta-refresh tag              ( )YES  (*)NO
Restrict connections only to localhost             ( )YES  (*)NO
Order of name resolution                           [unspecified]
News server                                        [                     ]
Mode of news server                                [                     ]
Number of news messages                            [50                   ]
                                                   [OK]
« ↑ ↓ Viewing <Option Setting Panel>
```

### w3m Proxy Settings (3:42)
```
Proxy Settings

Use proxy                                  (*)YES  ( )NO
URL of HTTP proxy host                     [          ]
URL of HTTPS proxy host                    [          ]
URL of GOPHER proxy host                   [          ]
URL of FTP proxy host                      [          ]
Domains to be accessed directly (no proxy) [          ]
Check noproxy by network address           (*)YES  ( )NO
Disable cache                              ( )YES  (*)NO
                                           [OK]
```

### Error screen (3:44)
```
gunzip: unknown compression format
« ↑ ↓ Viewing[SSL] <gfm>
```

### DuckDuckGo lite results, query "github <account> gfm" (3:46 - 3:47)
```
DuckDuckGo

[github <account> gfm          ] [Search]
[All Regions     ] [Any Time  ]

 [Next Page >]

1.  GitHub - <account>/gfm: Golang File Manager
    gfm is a command-line file manager written in Go. It provides a simple
    and unified interface to commonly used file management functionalities,
    eliminating the need to remember individual Linux commands.
    github.com/<account>/gfm

2.  <account> - GitHub
    <account> has 51 repositories available. Follow their code on GitHub.
    github.com/<account>/

3.  gfm command - github.com/<account>/gfm - Go Packages
    gfm is a command-line file manager written in Go. It provides a simple
    and unified interface to commonly used file management functionalities,
    eliminating the need to remember individual Linux commands.
```

### GitHub gfm repo page rendered in w3m (3:48 - 4:00)
```
Skip to content

Navigation Menu

(Close navigation menu)
(Toggle navigation)

Sign in

 • Products
    □ Platform
       ☆ [platform-4ce65ae0bf273ce2][platform-dark-ac89fbf373737e]GitHub
         Repository hosting, code review, and collaboration
       ☆ [copilot-e2b89335a66dba00][copilot-dark-2b28403b6080e89]CopilotAI
         coding agent for tasks, projects, and automated work
    □ Build & Ship
       ☆ ActionsCI/CD and automation
       ☆ Code QualityQuality analysis, fixes, and enforcement
       ☆ CodespacesCloud development environments
    □ Secure
       ☆ Code SecurityVulnerability detection and remediation
       ☆ Secret ProtectionSecret scanning and leak prevention
   All Features
 • Developers
    □ Learn
       ☆ Documentation
       ☆ Training
   ...
    □ GitHub Universe
       ☆ [universe-a98cd54821b27569][universe-a98cd54821b27569]GitHub
         UniverseThe event for developers, by developers - talks,
         workshops, and what's next for GitHub.
 • Pricing

(Search or jump to, type / to search)Search/
Sign in
Sign up
Appearance settings
You signed in with another tab or window. Reload to refresh your session. You
signed out in another tab or window. Reload to refresh your session. You
switched accounts on another tab or window. Reload to refresh your session.
Dismiss alert
(Dismiss this message)
{{ message }}

 1. <account>
 2. gfm

Repository navigation

 •
 • Code
 • Issues
 • Pull requests
 • Discussions
 • Actions
 • Projects
 • Security and quality
 • Insights

More items

<account>
gfm
Public

 • NotificationsYou must be signed in to change notification settings
 • Fork1 (1)
 • Star4 (4)You must be signed in to star a repository

About

Golang File Manager

youtu.be/<video-id>?si=<tracking id>

Topics

clifile-managergolang-cligolang-toollinux-tool

Resources

Readme
MIT license
Activity

Stars

4 stars

Watchers

1 watching

Forks

1 fork
Report repository
(main branch)
main
BranchesTags

[____________________]
Go to file
Code
Open more actions menu

Latest commit


History

12 Commits
 12 Commits

Folders and files

   Name       Name        Last commit message          Last commit date
.gitignore   .gitignore
LICENSE      LICENSE
README.md    README.md
go.mod       go.mod
```

### gfm README fragment (3:58 - 4:00)
```
 • Files manger: info, create, delete, move, copy, read, write file
 • Replaces core Linux commands: ls, rm, pwd, mv, mkdir, cat, touch, cp, and
   echo.

Usage

gfm [command] [arg1] [arg2]

Use the commands like their Linux counterparts, with gfm preceding them.

Examples:

# List files in the current directory
gfm ls

# Get the current working directory
gfm pwd

# View the contents of a file
gfm cat abc.txt

# Move a file
gfm mv abc.txt def.txt

# Delete a file
gfm rm abc.txt

Install

Prerequisites: Install Golang

Option 1: Install Binary

go install github.com/<account>/gfm@latest

Option 2: Install and build from source

git clone https://github.com/<account>/gfm.git
cd gfm

# Run code:
# Use go run . + command
go run .

# Build
go build .
```

### DuckDuckGo lite, empty (4:05, 4:52)
```
DuckDuckGo
[__________________________________________] [Search]
TEXT:
```
(4:08: box holds `github linux kernel`, status `« ↑ ↓ Viewing[SSL] <DuckDuckGo Lite: Fast, Lightweight Private Search>`.)

### DuckDuckGo results for "github linux kernel" (4:10)
```
DuckDuckGo

[github linux kernel                ] [Search]
[All Regions     ] [Any Time  ]

 [Next Page >]

1.  GitHub - torvalds/linux: Linux kernel source tree · GitHub
    Linux kernel source tree. Contribute to torvalds/linux development by
    creating an account on GitHub.
    github.com/torvalds/linux

2.  linux/kernel at master · torvalds/linux · GitHub
    Linux kernel source tree. Contribute to torvalds/linux development by
    creating an account on GitHub.
    github.com/torvalds/linux/tree/master/kernel

3.  The Linux Kernel Archives
    Other resources Git Trees Documentation Kernel Mailing Lists Patchwork
    Code of Conduct Reporting Issues Mirrors Linux.com Linux Foundation
    www.kernel.org
```

### GitHub torvalds/linux page in w3m, repo part (4:13 - 4:19)
Same navigation-menu header as above, then:
```
 1. torvalds
 2. linux

 • NotificationsYou must be signed in to change notification settings
 • Fork67.3k (67.3k)
 • Star255k (255k)You must be signed in to star a repository

About
Linux kernel source tree

Resources
Readme
License
Activity

Stars
254.8k stars

Watchers
8.5k watching

Forks
67.3k forks
Report repository
(master branch)
master
BranchesTags

[____________________]
Go to file
Code
Open more actions menu

Latest commit

History
1,506,114 Commits
 1,506,114 Commits

Folders and files
Name | Name | Last commit message | Last commit date
Documentation  LICENSES  arch  block  certs  crypto  drivers  fs  include  init
io_uring  ipc  kernel  lib  mm  net  rust  samples  scripts  security  sound
tools  usr  virt  .clang-format  .clippy.toml  .cocciconfig  .editorconfig
.get_maintainer.ignore  .gitattributes  .gitignore  .mailmap  .pylintrc
.rustfmt.toml  AGENTS.md  COPYING  CREDITS  Kbuild  Kconfig  MAINTAINERS
Makefile  README
View all files

Repository files navigation
 •
 • README
 • License

More items

Linux kernel
```

### Linux README (4:21 - 4:26)
```
Linux kernel
============

The Linux kernel is the core of any Linux operating system. It manages hardware
system resources, and provides the fundamental services for all other software.

Quick Start
-----------

* Report a bug: See Documentation/admin-guide/reporting-issues.rst
* Get the latest kernel: https://kernel.org
* Build the kernel: See Documentation/admin-guide/quickly-build-trimmed-linux.r⟨cut off at window edge⟩
* Join the community: https://lore.kernel.org/

Essential Documentation
-----------------------

All users should be familiar with:

* Building requirements: Documentation/process/changes.rst
* Code of Conduct: Documentation/process/code-of-conduct.rst
* License: See COPYING

Documentation can be built with make htmldocs or viewed online at:
https://www.kernel.org/doc/html/latest/


Who Are You?
============

Find your role below:

* New Kernel Developer: Getting started with kernel development
* Academic Researcher: Studying kernel internals and architecture
* Security Expert: Hardening and vulnerability analysis
* Backport/Maintenance Engineer: Maintaining stable kernels
* System Administrator: Configuring and troubleshooting
* Maintainer: Leading subsystems and reviewing patches
* Hardware Vendor: Writing drivers for new hardware
* Distribution Maintainer: Packaging kernels for distros
* AI Coding Assistant: LLMs and AI-powered development tools


For Specific Users
==================

New Kernel Developer
--------------------

Welcome! Start your kernel development journey here:

* Getting Started: Documentation/process/development-process.rst
* Your First Patch: Documentation/process/submitting-patches.rst
* Coding Style: Documentation/process/coding-style.rst
* Build System: Documentation/kbuild/index.rst
```

### w3m help page (4:32 - 4:37)
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
« ↑ ↓ Viewing <w3m help page>
```
Lower part seen at 4:34-4:37:
```
C-l               Draw the screen anew (REDRAW)
<NOT ASSIGNED>    Restart loading and drawing of images (DISPLAY_IMAGE)
<NOT ASSIGNED>    Stop loading and drawing of images (STOP_IMAGE)

Tab Operations

T                 Open a new tab (with current document) (NEW_TAB)
C-q               Close tab (CLOSE_TAB)
}                 Switch to the next tab (NEXT_TAB)
{                 Switch to the previous tab (PREV_TAB)
ESC-t             Pop up tab selection menu (TAB_MENU)
<NOT ASSIGNED>    Move right along the tab bar (TAB_RIGHT)
<NOT ASSIGNED>    Move left along the tab bar (TAB_LEFT)

Bookmark Management

ESC-b             View bookmarks (VIEW_BOOKMARK)
ESC-a             Add current page to bookmarks (ADD_BOOKMARK)

Searches

/                 Search forward (WHEREIS)
?                 Search backward (SEARCH_BACK)
n                 Continue search forward (SEARCH_NEXT)
N                 Continue search backward (SEARCH_PREV)
C-s               Incremental search forward (ISEARCH)
C-r               Incremental search backward (ISEARCH_BACK)

Forward: bookmark
```

### Bookmark registration forms (4:45; 5:00)
```
Register to my bookmark

New Section: [                                              ]
URL:         [https://github.com/torvalds/linux             ]
Title:       [GitHub - torvalds/linux: Linux kernel source tree · GitHub ]
[ADD]
```
```
Register to my bookmark

Section:     [Miscellaneous]
New Section: [                                              ]
URL:         [https://hayyoth.org/                          ]
Title:       [Hayyoth                                       ]
[ADD]
```

### DuckDuckGo results for "hayyoth" (4:57)
```
DuckDuckGo
[hayyoth                          ] [Search]
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
    Hayyoth Lab We stole Zeus's lightning, trapped it in stone, and taught it
    to think. Hayyoth Lab is a small and fast C framework for building,
```

### hayyoth.org home page in w3m (4:59 - 5:08)
```
Hayyoth


- White Paper -

Understand the Game. Don't Play It Blindly. The Art of Quitting.

Year         Everyone                      I                         Outcome
2018 Studied to get into a    Studied to understand       Earned direct admission
     prestigious university   the world                   to university
2021 Pursued academic         Taught myself               Found a job
     credentials              programming
2023 Wanted a corporate IT    Turned toward software      Freedom and independence
     job                      freedom - Old Git
2025 Wanted long-term job     Left the company again      Remained independent
     security
2025 Rushed toward AI and     Turned to farming           Gained control over my
     technology                                           life and time
2026 Was fascinated by what   Explored human nature,      Gained people's attention
     AI could create          trust, and attention
2026 Chased audience growth   Just do whatever I          Refused to let the
     and algorithmic          want                        algorithm dictate my
     attention                                            direction
Now  ...                      Quit quitting               ...

----------------------------------------------------------------------------

Videos
```
(Blue links: "White Paper", "Studied to understand the world", "Turned toward software freedom - Old Git", "Explored human nature, trust, and attention".) At 5:06 bottom prompt: `Goto URL: https://hayyoth.org/`.

### w3m Bookmarks page (5:09)
```
Bookmarks

Miscellaneous

 • GitHub - torvalds/linux: Linux kernel source tree · GitHub
 • Hayyoth
```

### Linux GitHub page with raw JSON blob (5:17)
```
8000
Skip to content
{"props":
{"color_mode":"dark","logged_in":false,"marketing_page":false,"home_path":"/
","login_path":"/login?return_to=
https%3A%2F%2Fgithub.com%2Ftorvalds%2Flinux","signup_path":"/signup?ref_cta=
Sign+up&ref_loc=header+logged+out&ref_page=
%2F%3Cuser-name%3E%2F%3Crepo-name%3E&source=header-repo&source_repo=
torvalds%2Flinux","signup_enabled":true,"is_signup_controller":false,"show_sear⟨cut⟩
{\"event_type\":\"authentication.click\",\"payload\":{\"location_in_page\":\
"site header menu\",\"repository_id\":null,\"auth_type\":\"SIGN_UP\",\
"originating_url\":\"https://github.com/torvalds/linux\",\"user_id\":null}}
","auth_hydro_click_hmac":"<hex string>
Navigation Menu
...
```

## Code & shell
No shell prompt visible in this segment; everything happens inside w3m. Notable prompts/status lines:
```
TEXT:
Forward: boo      -> Forward: bookmark
Goto URL: https://hayyoth.org/
lite.duckduckgo.com contacted. Waiting for reply...
Opening socket...
1.57kb loaded
```
Keys implied by frames (no keystroke overlay): `/` forward search, ESC-a add bookmark, ESC-b view bookmarks, U goto URL, arrow/space scrolling. The w3m launch command is not visible here.

## Observations
- He demonstrates w3m as a usable daily browser: Option Setting Panel (the sticky "crazy option config"), DuckDuckGo lite searches, opening GitHub repos and reading READMEs in the terminal.
- Sticky notes act as a live agenda: "crazy option config", "search in page", then "bookmark" is added just before he demos it, finally "see bookmark". "Search in page" is shown with `/` in the w3m help page (searching "bookmark"); bookmarks shown by adding the Linux repo and his own site, then opening the Bookmarks page.
- Content choices: a small Go file manager wrapping ls/rm/pwd, the Linux kernel repo (255k stars, ~1.5M commits), and his own site. The site's "Everyone / I / Outcome" table (2018-Now) is a mini life story: studied to understand not for prestige, self-taught programming, software freedom ("Old Git"), left the company, farming for control of life and time, explored human nature/trust/attention, refused to let the algorithm dictate direction, "Just do whatever I want", ending "Quit quitting". Tagline "Understand the Game. Don't Play It Blindly. The Art of Quitting." and "We stole Zeus's lightning, trapped it in stone, and taught it to think."
- The Linux README lists "AI Coding Assistant: LLMs and AI-powered development tools" as a role (visible on screen, not commented in frames).
- Rough edges are left unedited (gunzip error, JSON blob and alt-text noise from GitHub's JS-heavy page), implicitly showing a text browser is workable despite them. Webcam: he looks at the screen, briefly smokes around 4:15-4:18; otherwise static.
