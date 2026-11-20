# Segment 1 (0:00 - 1:48) - "you should try w3m - terminal web browser"

Layout throughout: a macOS-style Terminal window (left), one yellow sticky note (top right), and the "Human" webcam window (bottom right, ignored except where noted).

## Timeline

- [00:00] Terminal is empty: prompt `~ $ ` with a block cursor. Sticky note is blank (yellow).
- [00:03] Terminal shows a man page: `W3M(1)  General Commands Manual  W3M(1)`, NAME / SYNOPSIS / DESCRIPTION / ARGUMENTS, pager prompt `:` at the bottom. (He ran `man w3m`.)
- [00:08] The NAME line `w3m - a text based web browser and pager` is highlighted (selected) with the mouse. Sticky note now reads: "w3m - a text based web browser and pager".
- [00:09] Mouse selects the word "JavaScript" in the sentence "...but it ignores JavaScript and Cascading Style Sheets."
- [00:16] Man page quit. Terminal shows `~ $ man w3m` then a fresh prompt. Sticky note gets a second, bold, line: "ignores JavaScript".
- [00:17] Terminal now shows the tail of the w3m usage/option list (output of `w3m -help` style listing, ending with `-help` / `-version`), prompt `~ $ ` below.
- [00:22] w3m is running on a page titled "Hayyoth" (status bar `Viewing[SSL] <Hayyoth>`): a "White Paper" page with a Year / Everyone / I / Outcome table (text in Texts on screen). Cursor on the "H" of Hayyoth.
- [00:25] Page scrolled down: the end of the table, then a "Videos" list (Opinions on AI, Build your own X from scratch, ...) and a footer link line. Cursor sits on the second bullet "Build your own X from scratch".
- [00:26] Scrolled back to top of the page; cursor moved to the blank line under "Understand the Game. Don't Play It Blindly. The Art of Quitting."
- [00:28] Followed a link to a new page, status bar `<Hayyoth White Paper>`: manifesto text "We stole Zeus's lightning..." with section "I. We Build the Machine".
- [00:29] Scrolled further down the same White Paper page (more manifesto paragraphs).
- [00:31] Back on the first lines of the White Paper page, prompt at bottom: `Do you want to exit w3m? (y/n)` (he pressed q).
- [00:32] Back at the shell. History shows `~ $ w3m hayyoth.org` and a new `~ $ w3m` being typed.
- [00:35] Terminal blank with a hourglass-like cursor icon and message `wingram.org contacted. Waiting for reply...` (he ran `w3m wingram.org`).
- [00:36] Page loaded: `[logo] Wingram.Org`, status bar `Viewing[SSL] <Home - Wingram.Org>` (text in Texts on screen).
- [00:37] Same page, exit prompt `Do you want to exit w3m? (y/n)`.
- [00:38] Back at the shell: history `~ $ w3m hayyoth.org`, `~ $ w3m wingram.org`, new `~ $ w3m ` being typed.
- [00:44] w3m opened on DuckDuckGo Lite: heading `DuckDuckGo`, input `[ ______ ] [Search]`, status bar `<DuckDuckGo Lite: Fast, Lightweight Private Search>`. (He ran `w3m lite.duckduckgo.com/lite`, confirmed in the later history.)
- [00:45] Typing into the form field in the bottom prompt: `TEXT:hel`.
- [00:47] Field filled with `hello world`; cursor moved onto the `[Search]` button.
- [00:48] Search results page for `hello world` (status `<hello world at DuckDuckGo>`), with sponsored results 1 and 2 (Udemy, Codefinity) and result 3 "Hello, world - Wikipedia".
- [00:52] Exit prompt `Do you want to exit w3m? (y/n)` shown over the results (webcam: he now has a cigarette in his mouth).
- [00:53] Back on the results page, same view (he answered n).
- [00:55] Scrolled down; cursor on result number `3.`; results 3 and 4 visible.
- [00:57] Opened result 3: Wikipedia page `<Hello, world - Wikipedia>`, starting with "Jump to content", Main menu, Navigation links, search box.
- [00:59] Scrolled down to the article lead ("From Wikipedia, the free encyclopedia ... Traditional first example of a computer programming language").
- [01:00] Scrolled further: article lead paragraph and start of the "History" section with the Brian Kernighan "hello, world" image alt text.
- [01:03] Exited w3m. Shell history: `~ $ w3m hayyoth.org`, `~ $ w3m wingram.org`, `~ $ w3m lite.duckduckgo.com/lite`, then empty prompt.
- [01:04] Typing `e` at an empty prompt (alias e = vim).
- [01:06] vim opens `.zshrc` (status `".zshrc" 18L, 552B  1,1  All`).
- [01:07] In INSERT mode on line 5 (blank line below the PATH export); an empty line is being opened.
- [01:11] Typed line 5: `export WWW_HOME="htt` (still typing).
- [01:17] Line complete: `export WWW_HOME="https://lite.duckduckgo.com/lite"`; he is at the vim `:` command prompt (about to save/quit).
- [01:19] Back at shell: `~ $ e .zshrc` and new prompt.
- [01:20] Terminal now has two tabs (both `~ - zsh`); a new tab is active with an empty prompt (new shell to pick up the new rc).
- [01:22] In the new tab, typing `w3m` at the prompt.
- [01:23] w3m opens straight to DuckDuckGo Lite (no URL argument needed), cursor in the search field.
- [01:24] Typing in the bottom prompt: `TEXT:hello`.
- [01:25] Field shows `hello`.
- [01:26] Search submitted: `lite.duckduckgo.com contacted. Waiting for reply...`
- [01:27] Results for `hello` (status `<hello at DuckDuckGo>`): 1. Hello (Adele song) - Wikipedia, 2. Adele - Hello (Official Music Video) - YouTube.
- [01:32] Opened result 1: Wikipedia `<Hello (Adele song) - Wikipedia>`, top of page (Jump to content / Main menu ...).
- [01:33] Scrolled to the article infobox ("2015 single by Adele", "Hello", Released, Recorded, Studio, Genre, Length ...).
- [01:35] Scrolled further down the infobox (Label, Songwriters, Producer, singles chronology, "Music video").
- [01:38] The sticky note is being edited live: it is now being retyped; text so far `multio|` (typing cursor visible); old text gone. Terminal unchanged.
- [01:45] Sticky note reads "multiple pages". Terminal now shows a tab bar at the top: `[ Hello (Adele song) - Wikipedia ][ Hello (Adele song) - Wikipedia ]` (second tab bold/underlined = active), i.e. he opened a new w3m tab with the same page.
- [01:47] Second tab switched to a new DuckDuckGo Lite page: tab bar `[ Hello (Adele song) - Wikipedia ][DuckDuckGo Lite: Fast, Lightweight Pri]`, empty search field.
- [01:48] Typing in the bottom prompt in the second tab: `TEXT:` (empty so far).

## Texts on screen

### Sticky note (0:00 - 0:08) - blank
(empty yellow note)

### Sticky note (0:08 - 0:16)
```
w3m - a text based web
browser and pager
```

### Sticky note (0:16 - ~1:35)
```
w3m - a text based web
browser and pager

ignores JavaScript
```
(second line "ignores JavaScript" is rendered in bold)

### Sticky note (~1:38 - 1:48)
Old text cleared; typed live ("multio" at 1:38), final form:
```
multiple pages
```

### man page W3M(1) (0:03 - 0:16, scrolled to top only)
```
W3M(1)                    General Commands Manual                    W3M(1)

NAME
       w3m - a text based web browser and pager

SYNOPSIS
       w3m [OPTION]... [ file | URL ]...

DESCRIPTION
       w3m is a text based browser which can display local or remote web
       pages as well as other documents.  It is able to process HTML tables
       and frames but it ignores JavaScript and Cascading Style Sheets.  w3m
       can also serve as a pager for text files named as arguments or passed
       on standard input, and as a general purpose directory browser.

       w3m organizes its content in buffers or tabs, allowing easy navigation
       between them.  With the w3m-img extension installed, w3m can display
       inline graphics in web pages.  And whenever w3m's HTML rendering
       capabilities do not meet your needs, the target URL can be handed over
       to a graphical browser with a single command.

       For help with runtime options, press "H" while running w3m.

ARGUMENTS
       When given one or more command line arguments, w3m will handle targets
       according to content type.  For web, w3m gets this information from
       HTTP headers; for relative or absolute file system paths, it relies on
       filenames.
:
```
(words "w3m", "file", "URL" are underlined in the man page.)

### w3m option listing (visible 0:17, 0:32, 0:38, 1:03; scrolled)
```
    -dump_head      dump response of HEAD request into stdout
    -dump_source    dump page source into stdout
    -dump_both      dump HEAD and source into stdout
    -dump_extra     dump HEAD, source, and extra information into stdout
    -post file      use POST method with file content
    -header string  insert string as a header
    +<num>          goto <num> line
    -num            show line number
    -no-proxy       don't use proxy
    -4              IPv4 only (-o dns_order=4)
    -6              IPv6 only (-o dns_order=6)
    -insecure       use insecure SSL config options
    -no-mouse       don't use mouse
    -cookie         use cookie (-no-cookie: don't use cookie)
    -cookie-jar file use file instead of default cookie file
    -graph          use DEC special graphics for border of table and menu
    -no-graph       use ASCII character for border of table and menu
    -s              squeeze multiple blank lines
    -W              toggle search wrap mode
    -X              don't use termcap init/deinit
    -title[=TERM]   set buffer name to terminal title string
    -o opt=value    assign value to config option
    -show-option    print all config options
    -config file    specify config file
    -debug          use debug mode (only for debugging)
    -reqlog         write request logfile
    -help           print this usage message
    -version        print w3m version
```

### Page "Hayyoth" in w3m (0:22 - 0:26)
Title line `Hayyoth`, link `- White Paper -`, then:
```
Understand the Game. Don't Play It Blindly. The Art of Quitting.

Year       Everyone                    I                          Outcome
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
2026 Chased audience growth     Just do whatever I          Refused to let the
     and algorithmic            want                        algorithm dictate my
     attention                                              direction
Now  ...                        Quit quitting               ...

--------------------------------------------------------------------------------
Videos

 * Opinions on AI
 * Build your own X from scratch
 * x - but better
 * Learn C
 * Data Structures in C
 * Golang
 * Lisp Language
 * Web Development

--------------------------------------------------------------------------------
Youtube - Codeberg - Email
--------------------------------------------------------------------------------
```
(Link text shown in blue/purple: "- White Paper -", "Studied to understand the world", "Turned toward software freedom - Old Git", "Explored human nature, trust, and attention", all video items, and "Youtube", "Codeberg", "Email". Status bar: `<< ^ v Viewing[SSL] <Hayyoth>`.)

### Page "Hayyoth White Paper" (0:28 - 0:31, parts seen)
Top of page:
```
Hayyoth

We stole Zeus's lightning, trapped it in stone, and taught it to think.

--------------------------------------------------------------------------------

I. We Build the Machine

A program takes form within the mind, while computers bring it into reality.
Together we summon the computer spirits, using spells to call them forth.

Engineers build the structure, craftsmen create the art.
Children love playing with it, adults love controlling it.


But no hand alone commands the whole, for systems grow beyond their makers.
Rules shape the hands that once shaped them, and power teaches power how to
endure.

Beliefs give the system a purpose, fear gives its walls a reason.
Faith can make a cage feel holy, and good intentions can sharpen the blade.


Time is stolen, minds are misled, money keeps pouring fuel upon the fire.
The cruel may make the Machine worse, but even the kind may feed it
unknowingly.
```
Scrolled-down portion (0:29, screen starts mid-section, so heading of that section not visible: ?? (heading above)):
```
One ruler leaves, another takes his place, one engineer falls, another builds.
The faces change, the structure remains, for those within it learn its hidden
laws.


Some seek gold, some seek glory, some seek power, some seek a better world.
Yet those who believe they must save the world may forget to ask who guards
the savior.

The wise do not merely flee the Machine; they learn the forces that make it
turn.
They question the hands, the rules, the purpose, and leave a door for being
wrong.


For evil is not always born in evil hearts, nor does good forever live in good
intentions.
A system can shape the righteous into tyrants, and a believer can become the
servant of his belief.

So build, but remember what you build. Believe, but remember you may be wrong.
Hold power, but let another question you, for no mind should become the law of
the world.


The lost remain blind inside, the greedy keep feeding the fire.
The faithful keep guarding what they believe, and the system quietly teaches
?? (continues below frame)
```

### Page "Home - Wingram.Org" (0:36 - 0:37)
```
[logo] Wingram.Org

  * Wingram.Org
      [] About

Wingram.Org

Wingram on the way we find the Truth.

--------------------------------------------------------------------------------

About

  * Wingram Philosophy
  * Wingram Father
  * Sites - Contact

--------------------------------------------------------------------------------

Don't forget bookmark this site.

You can forget this site, but you can't forget an Ideology

Keyboard Shortcuts

(Close)

Keys     Action
?? (table continues below the visible area)
```

### DuckDuckGo Lite start page (0:44 - 0:47, again 1:23 - 1:25 and 1:47)
```
                              DuckDuckGo

              [______________________________]  [Search]
```
Status bar: `<DuckDuckGo Lite: Fast, Lightweight Private Search>`.

### DuckDuckGo results for "hello world" (0:48 - 0:55)
```
DuckDuckGo

[hello world                          ] [Search]
[All Regions     ] [Any Time  ]

 [Next Page >]

1.  Programming Languages Courses - Udemy(TM) Official Site (Sponsored link -
    more info)
    Shop thousands of high-quality on-demand online courses. Start learning
    today. Join millions of learners from around the world already learning
    on Udemy.
    udemy.com

2.  Free Python Lessons Online - Learn Coding. Get a Dream Job (Sponsored
    link - more info)
    Open up new career options in tech! "Intro to Coding" Self-Paced Online
    Course. Run and Submit Code from Your Browser - No Installation Required
    codefinity.com

3.  Hello, world - Wikipedia
    A " Hello, world " program is usually a simple computer program that
    outputs a message similar to "Hello, world". The text may be printed out
    or displayed on a screen (often a console). A small piece of code in most
    general-purpose programming languages, this program is used to illustrate
    a language's basic syntax. Such a program is often the first written by a
    student of a new programming ...
    en.wikipedia.org/wiki/Hello,_world

4.  Hello World Program: First Program While Learning Programming
    The Hello World program is the first step in learning programming. The
    task is to complete the given code to print "Hello World" as output.
    Example: Output: Hello World Explanation: Prints the first message
    traditionally used to begin learning programming. Let's see how to write
    the Hello World program in different programming languages.
```
Status bar: `<hello world at DuckDuckGo>`.

### Wikipedia "Hello, world" page (0:57 - 1:00)
Top (0:57):
```
Jump to content
[ ] Main menu
Main menu
(Move Main menu to sidebar)move to sidebar (Hide Main menu)hide
Navigation

  * Main page
  * Contents
  * Current events
  * Random article
  * About Wikipedia
  * Contact us

Contribute

  * Help
  * Learn to edit
  * Community portal
  * Recent changes
  * Upload file
  * Special pages

[enwik] Wikipedia The Free Encyclopedia
Search
[____________________]
Search

[ ] Appearance
```
Later (0:59 - 1:00):
```
Print/export
  * Download as PDF
  * Printable version

In other projects
  * Wikimedia Commons
  * Wikiversity
  * Wikidata item

Appearance
(Move Appearance to sidebar)move to sidebar (Hide Appearance)hide
From Wikipedia, the free encyclopedia
Traditional first example of a computer programming language
"Hello, World" and "Helloworld" redirect here. For the Artemis II image, see
Hello, World (photograph). For other uses, see Hello World (disambiguation).

A "Hello, world" program is usually a simple computer program that outputs a
message similar to "Hello, world". The text may be printed out or displayed on
a screen (often a console). A small piece of code in most general-purpose
programming languages, this program is used to illustrate a language's basic
syntax. Such a program is often the first written by a student of a new
programming language,^[1] but it can also be used as a sanity check to ensure
that the computer software intended to compile or run source code is correctly
installed, and that its operator understands how to use it.

History

[edit]

[250px-Hello_World_Brian_Kernig]
A program to output "hello, world", handwritten in the C language and signed
by Brian Kernighan (1978)

While several small test programs have existed since the development of
programmable computers, the tradition of using the phrase "Hello, world" as a
test message was influenced by an example program in the 1978 book The C
Programming Language,^[2] with likely earlier use in BCPL. The example program
from the book prints "hello, world", and was inherited from a 1974 Bell
Laboratories internal memorandum by Brian Kernighan, Programming in C: A
?? (continues below frame)
```

### DuckDuckGo results for "hello" (1:27)
```
DuckDuckGo

[hello                                ] [Search]
[All Regions     ] [Any Time  ]

 [Next Page >]

1.  Hello (Adele song) - Wikipedia
    " Hello " is a song recorded by British singer-songwriter Adele, released
    on 23 October 2015 by XL Recordings as the lead single from her third
    studio album, 25 (2015). Written by Adele and the album's producer, Greg
    Kurstin, "Hello" is a piano ballad with soul influences (including
    guitar) and lyrics that discuss themes of nostalgia and regret.
    en.wikipedia.org/wiki/Hello_(Adele_song)

2.  Adele - Hello (Official Music Video) - YouTube
    Listen to "Easy On Me" here: http://Adele.lnk.to/EOMPre-order Adele's new
    album "30" before its release on November 19: https://www.adele.comShop
    the "Adele...
    www.youtube.com/watch?v=YQHsXMglC9A
```

### Wikipedia "Hello (Adele song)" infobox (1:33 - 1:48)
```
From Wikipedia, the free encyclopedia
2015 single by Adele

                      "Hello"
A dark black-and-white image of Adele holding a telephone to her ear
                  Single by Adele
                  from the album 25
     Released      23 October 2015 (2015-10-23)
     Recorded      2015
      Studio       Metropolis (London)^[1]
      Genre        Soul^[2]
      Length       4:56 (album version)
                   4:15 (radio edit)
                     * XL
      Label          * Columbia

                     * Adele Adkins
   Songwriters       * Greg Kurstin

     Producer      Greg Kurstin
                  Adele singles chronology
"Skyfall" "Hello" "When We Were Young"
(2012)    (2015)  (2016)

                      Music video
"Hello" on YouTube
```
(Before the infobox at 1:32: `Get shortened URL`, `Switch to legacy parser`, `Print/export: Download as PDF, Printable version`, `In other projects: Wikidata item`.)

## Code & shell

Shell commands typed or visible (prompt is `~ $ `, zsh):
```
~ $ man w3m
~ $ w3m hayyoth.org
~ $ w3m wingram.org
~ $ w3m lite.duckduckgo.com/lite
~ $ e                 # alias e="vim"
~ $ e .zshrc
~ $ w3m               # in a new terminal tab, after WWW_HOME was set
```
w3m status/prompts quoted:
- `Do you want to exit w3m? (y/n)`
- `wingram.org contacted. Waiting for reply...`
- `lite.duckduckgo.com contacted. Waiting for reply...`
- `TEXT:hel`, `TEXT:hello` (form-field entry line at the bottom)
- Status bar form: `<< ^ v Viewing[SSL] <page title>`
- Tab bar (1:45 - 1:48): `[ Hello (Adele song) - Wikipedia ][ ... ]`

`.zshrc` as shown in vim (".zshrc" 18L, 552B), final state at 1:17 (line numbers not shown; the ones in the status bar are cursor position 5,1 / 5,21):
```
export CLICOLOR=1
export LSCOLORS=Gxfxcxdxbxegedabagacad

export PATH="$PATH:$HOME/go/bin"
export WWW_HOME="https://lite.duckduckgo.com/lite"

PROMPT="%1~ $ "
#PROMPT="%B%F{blue}%1~ $ %f%b"

alias e="vim"
alias cc="clang -Wall -Wextra -pedantic -Wno-unused-parameter -Wno-unused-varia⟨cut off by window width⟩
alias l="ls -a"
alias ll="du -sh * | sort -h"
alias web="open -a Firefox"
alias hay="open -a Firefox https://www.youtube.com/<channel handle>"
alias cam="scam start"

alias crawl='wget --recursive --page-requisites --adjust-extension --convert-li⟨cut off⟩
```
The `export WWW_HOME=...` line (line 5) is the only new line he typed: first `export WWW_HOME="htt` at 1:11, finished at 1:17 as `export WWW_HOME="https://lite.duckduckgo.com/lite"`. Before the edit (1:06) there was only a blank line there (file had 18 lines). The tilde `~` filler lines below the file are vim's end-of-buffer markers.

## Observations

- He is teaching that w3m is a text-mode web browser and pager, and he builds the point on a single sticky note: first the man-page NAME line, then the one fact he wants remembered, "ignores JavaScript". Later the note is erased and replaced by "multiple pages" when he demonstrates tabs. So the sticky note is a running one-or-two-line summary, kept deliberately tiny.
- Method: demonstrate rather than explain. He reads `man w3m` (primary source, selecting the line with the mouse and copying it to the note), then immediately browses real sites in the terminal: his own site (hayyoth.org, shown rendering as plain readable text with the table intact and the links highlighted), another site (wingram.org), then DuckDuckGo Lite and Wikipedia. The point is visible: pages work without JavaScript or CSS and still read fine.
- The "Hayyoth" page contains the author's own life table ("Everyone / I / Outcome", 2018 to "Now ... Quit quitting") and a "White Paper" manifesto ("We stole Zeus's lightning, trapped it in stone, and taught it to think ... The wise do not merely flee the Machine; they learn the forces that make it turn ... for no mind should become the law of the world"). Philosophy visible: understand the system rather than blindly play it or blindly flee it; do whatever he wants rather than let an algorithm dictate direction; "The Art of Quitting" / "Quit quitting"; learn C, data structures in C, Go, Lisp (his video list). He dogfoods: his own site is readable in the simplest possible browser. The wingram.org page tagline reads "Wingram on the way we find the Truth."
- Workflow customization: he edits `.zshrc` in vim (alias `e`) and sets `export WWW_HOME="https://lite.duckduckgo.com/lite"` so that a bare `w3m` opens DuckDuckGo Lite directly, turning w3m into a one-command search tool. He tests it in a new terminal tab (typing only `w3m`), searching "hello" and opening the Adele "Hello" Wikipedia article. His other aliases (open -a Firefox, clang with strict warnings, `crawl` using wget, `ls -a`) show a terminal-first, small-tools setup.
- Keys/UI shown: the bottom prompt `TEXT:` for typing into form fields, a cursor moved onto `[Search]` to submit, `q` leading to the `Do you want to exit w3m? (y/n)` prompt (answered n once at 0:52 to stay in the page), scrolling with the keyboard, and a second w3m tab (tab bar with page titles) to show "multiple pages". The key presses themselves are not visible, only their effects.
- Tone: unhurried, plain, one terminal window plus one note, no slides; the webcam shows him calmly talking (cigarette in mouth from about 0:52, not relevant to content).
