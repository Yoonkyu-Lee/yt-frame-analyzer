# Segment 4 (05:21 - 07:13) - w3m demo: bookmarks, author's page, searching Beej's Guide, option panel, help page

Screen layout throughout: macOS-style "Terminal" window (w3m running), yellow sticky note top right, webcam "Human" window bottom right (face only; occasionally he smokes / covers mouth / looks down while typing).

## Timeline
- [05:21] w3m shows the GitHub torvalds/linux page, rendered as text: file list in two columns (`.clippy.toml .cocciconfig .editorconfig .get_maintainer.ignore .gitattributes .gitignore .mailmap .pylintrc .rustfmt.toml AGENTS.md COPYING CREDITS Kbuild Kconfig MAINTAINERS Makefile README`), red `View all files`, "Repository files navigation" with bullets README / License, red `More items`, `Linux kernel`. Status bar: `« ↑ ↓ Viewing[SSL] <GitHub - torvalds/linux: Linux kernel source tree · GitHub`. Sticky note: `see bookmark`.
- [05:22] Page jumped to its top: junk raw JSON props text (`8000`, `Skip to content`, `{"props":{"color_mode":"dark","logged_in":false,...`), then `Navigation Menu`, `(Close navigation menu)`, `(Toggle navigation)`, `Sign in`, `Products` > `Platform` > `[platform-...]GitHub Repository hosting, code review, and collaboration`, `[copilot-...]CopilotAI coding agent for tasks, projects, and automated work`, `Build & Ship`. Shows how noisy a JS-heavy site is in w3m.
- [05:23] Bookmark view (key not visible, presumably ESC-b): `Bookmarks` / `Miscellaneous` with two entries: `GitHub - torvalds/linux: Linux kernel source tree · GitHub` and `Hayyoth` (cursor on Hayyoth). Status `Viewing <Bookmarks>`.
- [05:24] Opened the Hayyoth bookmark: his own site/white paper page (full table in Texts section). Status `Viewing[SSL] <Hayyoth>`.
- [05:25] Scrolled down: `Videos` list (Opinions on AI, Build your own X from scratch, x - but better, Learn C, Data Structures in C, Golang, Lisp Language, Web Development) and footer links `Youtube - Codeberg - Email`.
- [05:28] Pressed a key to open a URL prompt; status shows `Opening socket...` (key icon top-left = SSL).
- [05:29] DuckDuckGo Lite loaded (`DuckDuckGo` heading, empty search box, `[Search]`). Status `<DuckDuckGo Lite: Fast, Lightweight Private Search>`.
- [05:30] Pressed Enter on the input: prompt `TEXT:` at bottom (w3m form-field editing).
- [05:33] Search field now contains `beej guide C`; cursor moved onto `[Search]` button.
- [05:34] Enter pressed: `lite.duckduckgo.com contacted. Waiting for reply...`
- [05:35] Results page `<beej guide C at DuckDuckGo>`: result 1 and 2 "Beej's Guide to C Programming" (text in Texts section).
- [05:40] Followed result: `Opening socket...`
- [05:43] Beej's Guide to C Programming index page (v0.11.3, Copyright November 7, 2026) with table of contents.
- [05:45] Scrolled down the TOC (3 Variables and Statements ... 3.3 Flow Control).
- [05:48] Cursor on `2.2 Hello, World!` link, Enter pressed, `Opening socket...`
- [05:50] Section 2 text: end of intro ("everything else in C is just memorizing another way ... Pointers are the weird bit"), then `2.2 Hello, World!` with the C program. Webcam: he is smoking.
- [05:51 - 05:56] Paging down through the explanation of the Hello World program (comments, #include, preprocessor "octothorpe", stdio.h, main()). Text in Texts section.
- [05:56] Went back to the TOC (cursor on `2.2 Hello, World!`).
- [05:57] Pressed `/` : prompt `Forward: `
- [05:58] Typed `string` in the Forward prompt; page jumped to section 7 Strings in the TOC (cursor on `7 Strings`).
- [06:01] Back at DuckDuckGo Lite empty search page (new search; cursor in the box).
- [06:02] Typing into field: `TEXT:beej`.
- [06:06] Field now `beej guide bgclr`, cursor on `[Search]`.
- [06:07] Results for `beej guide bgclr` (result 1: v0.9.13, Copyright October 15, 2026; result 2: translators note).
- [06:10] `Opening socket...` then Beej's Guide "Library Reference" page (v0.9.13, Copyright October 15, 2026) with TOC.
- [06:14] Pressed `/`, typed `st` in the `Forward:` prompt.
- [06:15] Prompt now `Forward: string.h`; jumped to `26 <string.h> String Manipulation` in the TOC (cursor on the `s` of string.h).
- [06:16] Enter confirms search (status bar back to `Viewing[SSL] <Beej's Guide to C Programming>`).
- [06:17] Enter on the link: `Opening socket...`
- [06:19] Chapter `26 <string.h> String Manipulation` function table (memccpy ... strndup) loaded; he covers his mouth with his hand.
- [06:21] Pressed `/` again: `Forward: ` prompt.
- [06:24] Typed `strcat`; page jumped to end of strncpy's "See Also" and `26.3 strdup(), strndup()`.
- [06:25] Pressed `n`-style repeat (prompt still shows `Forward: strcat`); page now at `26.4 strcat(), strncat()` with synopsis.
- [06:35] Same page; mouse cursor (I-beam) appeared at middle; sticky note updated: added `easy search and navigation` below `see bookmark`. Status bar normal.
- [06:50] Sticky note now EMPTY (cleared). Screen switched to w3m `Option Setting Panel` (w3m version w3m/0.5.6), Display Settings. (Key `o`, not visible.)
- [06:51] Pressed `/` in the panel: `Forward: `.
- [06:54] Typed `browser`; panel jumped to `External Program Settings` with `External browser [/usr/bin/firefox]`.
- [06:59] Edited the External browser field: it now reads `open` (macOS `open` command replacing /usr/bin/firefox). Status `Viewing <Option Setting Panel>`.
- [07:02] Back on the strcat page of Beej's Guide (cursor on 'e' of "single"), i.e. the option panel was left/closed.
- [07:05] w3m help page opened (`H`): `******* w3m (WWW-wo-Miru) Version 0.5.6 by A.ITO ********`, In-page Navigation key list.
- [07:06] `Forward: ` prompt on the help page.
- [07:07] Typed `command`; jumped to "File/Stream Operations" and "Buffer Operations" sections (`Forward: command`).
- [07:09] Further down: `View bookmarks (VIEW_BOOKMARK)`, Searches, Dictionary Lookup, Mark Operations, Miscellaneous (`Forward: command`).
- [07:11] Bottom of help: `H` Show help panel ... `Q` Quit without confirmation; "User-defined key bindings" (empty). Cursor on `C` of `COMMAND`.
- [07:13] Same; status bar `Viewing <w3m help page>`; mouse cursor near `C-w`.

## Texts on screen

### Sticky note (05:21 - ~06:25)
```
see bookmark
```

### Sticky note (~06:35)
```
see bookmark

easy search and navigation
```

### Sticky note (06:50 - 07:13)
```
(empty)
```

### Bookmarks page (05:23)
```
Bookmarks

Miscellaneous

 • GitHub - torvalds/linux: Linux kernel source tree · GitHub
 • Hayyoth
```

### Hayyoth page, "White Paper" (05:24 - 05:30)
```
Hayyoth


- White Paper -

Understand the Game. Don't Play It Blindly. The Art of Quitting.

Year         Everyone                    I                          Outcome
2018 Studied to get into a        Studied to understand      Earned direct admission
     prestigious university       the world                  to university
2021 Pursued academic             Taught myself              Found a job
     credentials                  programming
2023 Wanted a corporate IT        Turned toward software     Freedom and independence
     job                          freedom - Old Git
2025 Wanted long-term job         Left the company again     Remained independent
     security
2025 Rushed toward AI and         Turned to farming          Gained control over my
     technology                                              life and time
2026 Was fascinated by what       Explored human nature,     Gained people's attention
     AI could create              trust, and attention
2026 Chased audience growth       Just do whatever I         Refused to let the
     and algorithmic              want                       algorithm dictate my
     attention                                               direction
Now  ...                          Quit quitting              ...
```
(In the "I" column the links "Studied to understand the world", "Turned toward software freedom - Old Git", "Explored human nature, trust, and attention" are blue links. "- White Paper -" is also a link.)

```
────────────────────────────────────────

Videos

 • Opinions on AI
 • Build your own X from scratch
 • x - but better
 • Learn C
 • Data Structures in C
 • Golang
 • Lisp Language
 • Web Development

────────────────────────────────────────
Youtube - Codeberg - Email
────────────────────────────────────────
```
(Bullet items and the three footer words are links.)

### DuckDuckGo Lite search page (05:29 - 05:34)
```
                DuckDuckGo

   [beej guide C                          ] [Search]
```

### DuckDuckGo results `beej guide C` (05:35 - 05:40)
```
DuckDuckGo

[beej guide C                          ] [Search]
[All Regions     ] [Any Time  ]

 [Next Page >]

1.  Beej's Guide to C Programming
    27.11 Unicode Encodings and C 27.11.1 UTF-8 27.11.2 UTF-16, UTF-32,
    char16_t, and char32_t 27.11.3 Multibyte Conversions 27.11.4 Third-Party
    Libraries 28 Exiting a Program 28.1 Normal Exits 28.1.1 Returning From
    main () 28.1.2 exit () 28.1.3 Setting Up Exit Handlers with atexit ()
    28.2 Quicker Exits with quick_exit () 28.3 Nuke it from Orbit: _Exit ()
    beej.us/guide/bgc/html/split/index.html

2.  Beej's Guide to C Programming
    Beej's Guide to C Programming This is the first volume of Beej's Guide to
    C, the tutorial. Click here for the library reference second volume.
    Please keep in mind this is a beta-quality document. If you have
    corrections, I'd love to hear them, but understand that even I might not
    have read that part of the guide yet. There are glaring errors!
```
(Words "Beej's Guide", "C", "guide" bold in snippet.)

### DuckDuckGo results `beej guide bgclr` (06:07 - 06:10)
```
[beej guide bgclr                      ] [Search]
[All Regions     ] [Any Time  ]
 [Next Page >]

1.  Beej's Guide to C Programming
    Brian "Beej Jorgensen" Hall v0.9.13, Copyright © October 15, 2026 1
    Foreword 1.1 Audience 1.2 How to Read This Book 1.3 Platform and Compiler
    1.4 Official Homepage 1.5 Email Policy 1.6 Mirroring 1.7 Note for
    Translators 1.8 Copyright and Distribution 1.9 Dedication 2 The C
    Language 2.1 Background 2.1.1 Comments 2.1.2 Separators 2.1.3 Expressions
    beej.us/guide/bgclr/html/split/

2.  Beej's Guide to C Programming
    If you want to translate the guide into another language, write me at
    <mail address> and I'll link to your translation from the main page. Feel
    free to add your name and contact info to the translation.
    beej.us/guide/bgclr/html/index-wide.html
```

### Beej's Guide to C Programming, index (05:43 - 05:48)
```
Prev | Contents | Next
────────────────────────────────────────
Beej's Guide to C Programming

Brian "Beej Jorgensen" Hall

v0.11.3, Copyright © November 7, 2026

 • 1 Foreword
    □ 1.1 Audience
    □ 1.2 How to Read This Book
    □ 1.3 Platform and Compiler
    □ 1.4 Official Homepage
    □ 1.5 Email Policy
    □ 1.6 Mirroring
    □ 1.7 Note for Translators
    □ 1.8 Copyright and Distribution
    □ 1.9 Dedication
 • 2 Hello, World!
    □ 2.1 What to Expect from C
    □ 2.2 Hello, World!
    □ 2.3 Compilation Details
    □ 2.4 Building with gcc
    □ 2.5 Building with clang
    □ 2.6 Building from IDEs
    □ 2.7 C Versions
 • 3 Variables and Statements
    □ 3.1 Variables
       ☆ 3.1.1 Variable Names
       ☆ 3.1.2 Variable Types
       ☆ 3.1.3 Boolean Types
    □ 3.2 Operators and Expressions
       ☆ 3.2.1 Arithmetic
       ☆ 3.2.2 Ternary Operator
       ☆ 3.2.3 Pre-and-Post Increment-and-Decrement
       ☆ 3.2.4 The Comma Operator
       ☆ 3.2.5 Conditional Operators
       ☆ 3.2.6 Boolean Operators
       ☆ 3.2.7 The sizeof Operator
    □ 3.3 Flow Control
       ☆ 3.3.1 The if-else statement
```
Later (05:57 - 05:58) TOC portion seen after searching "string":
```
    □ 5.5 The NULL Pointer
    □ 5.6 A Note on Declaring Pointers
    □ 5.7 sizeof and Pointers
 • 6 Arrays
    □ 6.1 Easy Example
    □ 6.2 Getting the Length of an Array
    □ 6.3 Array Initializers
    □ 6.4 Out of Bounds!
    □ 6.5 Multidimensional Arrays
    □ 6.6 Arrays and Pointers
       ☆ 6.6.1 Getting a Pointer to an Array
       ☆ 6.6.2 Passing Single Dimensional Arrays to Functions
       ☆ 6.6.3 Changing Arrays in Functions
       ☆ 6.6.4 Passing Multidimensional Arrays to Functions
 • 7 Strings
    □ 7.1 String Literals
    □ 7.2 String Variables
    □ 7.3 String Variables as Arrays
    □ 7.4 String Initializers
    □ 7.5 Getting String Length
    □ 7.6 String Termination
    □ 7.7 Copying a String
 • 8 Structs
    □ 8.1 Declaring a Struct
    □ 8.2 Struct Initializers
    □ 8.3 Passing Structs to Functions
    □ 8.4 The Arrow Operator
    □ 8.5 Copying and Returning structs
```

### Beej's Guide, section 2.1 end / 2.2 Hello, World! (05:50)
```
using operators you've likely never seen before.

It's especially insidious because once you grok^24 pointers, they're suddenly
easy. But up until that moment, they're slippery eels.

Everything else in C is just memorizing another way (or sometimes the same
way!) of doing something you've done already. Pointers are the weird bit. And,
arguably, even pointers are variations on a theme you're probably familiar
with.

So get ready for a rollicking adventure as close to the core of the computer
as you can get without assembly, in the most influential computer language of
all time^25. Hang on!

2.2 Hello, World!

This is the canonical example of a C program. Everyone uses it. (Note that the
numbers to the left are for reader reference only, and are not part of the
source code.)

 /* Hello world program */

 #include <stdio.h>

 int main(void)
 {
     printf("Hello, World!\n");  // Actually do the work here
 }
```

### Beej's Guide, Hello World explanation (05:51 - 05:55)
```
}

We're going to don our long-sleeved heavy-duty rubber gloves, grab a scalpel,
and rip into this thing to see what makes it tick. So, scrub up, because here
we go. Cutting very gently...

Let's get the easy thing out of the way: anything between the digraphs /* and
*/ is a comment and will be completely ignored by the compiler. Same goes for
anything on a line after a //. This allows you to leave messages to yourself
and others, so that when you come back and read your code in the distant
future, you'll know what the heck it was you were trying to do. Believe me,
you will forget; it happens.

Now, what is this #include? GROSS! Well, it tells the C Preprocessor to pull
the contents of another file and insert it into the code right there.

Wait-what's a C Preprocessor? Good question. There are two stages^26 to
compilation: the preprocessor and the compiler. Anything that starts with
pound sign, hash symbol, or "octothorpe", (#) is something the preprocessor
operates on before the compiler even gets started. Common preprocessor
directives, as they're called, are #include and #define. More on that later.

Before we go on, why would I even begin to bother pointing out that a pound
sign is called an octothorpe? The answer is simple: I think the word
octothorpe is so excellently funny, I have to gratuitously spread its name
around whenever I get the opportunity. Octothorpe. Octothorpe, octothorpe,
octothorpe.
```
```
So anyway. After the C preprocessor has finished preprocessing everything, the
results are ready for the compiler to take them and produce assembly code^27,
machine code^28, or whatever it's about to do. Machine code is the "language"
the CPU understands, and it can understand it very rapidly. This is one of the
reasons C programs tend to be quick.

Don't worry about the technical details of compilation for now; just know that
your source runs through the preprocessor, then the output of that runs
through the compiler, then that produces an executable for you to run.

What about the rest of the line? What's <stdio.h>? That is what is known as a
header file. It's the dot-h at the end that gives it away. In fact it's the
"Standard I/O" (stdio) header file that you will grow to know and love. It
gives us access to a bunch of I/O functionality^29. For our demo program,
we're outputting the string "Hello, World!", so we in particular need access
to the printf() function to do this. The <stdio.h> file gives us this access.
Basically, if we tried to use printf() without #include <stdio.h>, the
compiler would have complained to us about it.

How did I know I needed to #include <stdio.h> for printf()? Answer: it's in
the documentation. If you're on a Unix system, man 3 printf and it'll tell you
right at the top of the man page what header files are required. Or see the
reference section in this book. :-)

Holy moly. That was all to cover the first line! But, let's face it, it has
been completely dissected. No mystery shall remain!
```
```
So take a breather...look back over the sample code. Only a couple easy lines to
go.

Welcome back from your break! I know you didn't really take a break; I was
just humoring you.

The next line is main(). This is the definition of the function main();
everything between the squirrelly braces ({ and }) is part of the function
definition.

(How do you call a different function, anyway? The answer lies in the printf()
line, but we'll get to that in a minute.)

Now, the main function is a special one in many ways, but one way stands above
the rest: it is the function that will be called automatically when your
program starts executing. Nothing of yours gets called before main(). In the
case of our example, this works fine since all we want to do is print a line
and exit.

Oh, that's another thing: once the program executes past the end of main(),
down there at the closing squirrelly brace, the program will exit, and you'll
be back at your command prompt.

So now we know that that program has brought in a header file, stdio.h, and
declared a main() function that will execute when the program is started. What
are the goodies in main()?
```

### Beej's Guide Library Reference index (06:10 - 06:14)
```
Prev | Contents | Next
────────────────────────────────────────
Beej's Guide to C Programming

Library Reference

Brian "Beej Jorgensen" Hall

v0.9.13, Copyright © October 15, 2026

 • 1 Foreword
    □ 1.1 Audience
    □ 1.2 How to Read This Book
    □ 1.3 Platform and Compiler
    □ 1.4 Official Homepage
    □ 1.5 Email Policy
    □ 1.6 Mirroring
    □ 1.7 Note for Translators
    □ 1.8 Copyright and Distribution
    □ 1.9 Dedication
 • 2 The C Language
    □ 2.1 Background
       ☆ 2.1.1 Comments
       ☆ 2.1.2 Separators
       ☆ 2.1.3 Expressions
       ☆ 2.1.4 Statements
       ☆ 2.1.5 Booleans
```
TOC part after searching `string.h` (06:15 - 06:17):
```
    □ 24.14 exit(), quick_exit(), _Exit()
    □ 24.15 getenv()
    □ 24.16 system()
    □ 24.17 bsearch()
    □ 24.18 qsort()
    □ 24.19 abs(), labs(), llabs()
    □ 24.20 div(), ldiv(), lldiv()
    □ 24.21 mblen()
    □ 24.22 mbtowc()
    □ 24.23 wctomb()
    □ 24.24 mbstowcs()
    □ 24.25 wcstombs()
    □ 24.26 memalignment()
 • 25 <stdnoreturn.h> Macros for Non-Returning Functions
 • 26 <string.h> String Manipulation
    □ 26.1 memcpy(), memccpy(), memmove()
    □ 26.2 strcpy(), strncpy()
    □ 26.3 strdup(), strndup()
    □ 26.4 strcat(), strncat()
    □ 26.5 strcmp(), strncmp(), memcmp()
    □ 26.6 strcoll()
    □ 26.7 strxfrm()
    □ 26.8 strchr(), strrchr(), memchr()
    □ 26.9 strspn(), strcspn()
    □ 26.10 strpbrk()
    □ 26.11 strstr()
    □ 26.12 strtok()
    □ 26.13 memset(), `memset_explicit
```

### Chapter 26 <string.h> function table (06:19 - 06:21)
```
26 <string.h> String Manipulation

   Function    Description
memccpy()      Copy a region of memory to another, stopping at a specified
               character.
memchr()       Find the first occurrence of a character in memory.
memcmp()       Compare two regions of memory.
memcpy()       Copy a region of memory to another.
memmove()      Move a (potentially overlapping) region of memory.
memset()       Set a region of memory to a value.
memset_explicit Set a region of memory to a value in a manner that cannot be
()             optimized away.
strcat()       Concatenate (join) two strings together.
strchr()       Find the first occurrence of a character in a string.
strcmp()       Compare two strings.
strcoll()      Compare two strings accounting for locale.
strcpy()       Copy a string.
strcspn()      Find length of a string not consisting of a set of characters.
strdup()       Duplicate a string on the heap.
strerror()     Return a human-readable error message for a given code.
strlen()       Return the length of a string.
strncat()      Concatenate (join) two strings, length-limited.
strncmp()      Compare two strings, length-limited.
strncpy()      Copy two strings, length-limited.
strndup()      Duplicate a string on the heap, length-limited.
```

### Library reference: strncpy example tail / strdup / strcat (06:24 - 07:02)
```
    // leave room for the terminator:
    strncpy(dest+len, src, sizeof(dest)-len-1);

    // remember that sizeof() returns the size of the array in bytes
    // and a char is a byte:
    dest[sizeof(dest)-1] = '\0'; // terminate

    // dest is now:       v null terminator
    // I like hockey hocke
    // 0123456789012345678901234 5
 }

See Also

memcpy(), strcat(), strncat()

────────────────────────────────────────

26.3 strdup(), strndup()

Duplicate a string on the heap

Synopsis

New in C23!

 #include <string.h>
```
(The digit ruler line reads `// 01234567890123456789012345` on screen.)
```
    if (s != NULL)
        puts(s);  // Hello, world!
    else
        puts("Error duplicating string");

    free(s);
 }

See Also

strcpy(), strncpy(), malloc(), free()

────────────────────────────────────────

26.4 strcat(), strncat()

Concatenate two strings into a single string

Synopsis

 #include <string.h>

 int strcat(const char *dest, const char *src);

 int strncat(const char *dest, const char *src, size_t n);

Description
```

### w3m Option Setting Panel (06:50 - 06:59)
```
          Option Setting Panel
        (w3m version w3m/0.5.6)

[External Viewer Setup]

Display Settings

Tab width in characters                          [8        ]
Indent for HTML rendering                        [4        ]
Max chars per line (0: unset) Needs restart!     [0        ]
Number of pixels per character (4.0...32.0)      [8        ]
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
Dictionary command prompt                        [(dictionary)!   ]
Display file names in multi-column format        ( )YES (*)NO
Use ASCII equivalents to display entities        ( )YES (*)NO
Character type for border of table and menu      [charset specific   ]
```
```
Directory for temporary files          [                    ]
Default directory for downloads        [                    ]
                                       [OK]
──────────────────

External Program Settings

List of mime.types files               [~/.mime.types, /opt/
List of mailcap files                  [~/.w3m/mailcap, /opt
List of urimethodmap files             [~/.w3m/urimethodmap,
Editor                                 [/usr/bin/vi         ]
How to call Mailer for mailto URLs     [ignore options and use only the address]
with options
Mailer                                 [                    ]
External browser                       [/usr/bin/firefox    ]     <- at 06:54
                                       [open                ]     <- at 06:59 (edited)
2nd external browser ... 9th external browser   (all empty)
Run external viewer in the background  (*)YES ( )NO
Use LESSOPEN                           ( )YES (*)NO
                                       [OK]
```

### w3m help page (07:05 - 07:13)
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
```
M                 Display using an external browser (EXTERN)
ESC-M             Display target using an external browser (EXTERN_LINK)
L                 Show all URLs referenced (LIST)
ESC-l             Pop up menu for hyperlinks to browse to (LIST_MENU)
<NOT ASSIGNED>    Pop up link element menu (LINK_MENU)
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
```
```
ESC-b             View bookmarks (VIEW_BOOKMARK)
ESC-a             Add current page to bookmarks (ADD_BOOKMARK)

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
```
```
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
<NOT ASSIGNED>    Set environment variable (SETENV)
ESC-k             Define a binding between a key stroke combination and a comma...
<NOT ASSIGNED>    Reload configuration file (REINIT)
!                 Execute shell command and display output (SHELL)
C-z               Suspend w3m to background (SUSPEND)
q                 Quit with confirmation request (QUIT)
Q                 Quit without confirmation (EXIT)

User-defined key bindings
```

### Prompts / status bar strings seen
```
« ↑ ↓ Viewing[SSL] <GitHub - torvalds/linux: Linux kernel source tree · GitHub
« ↑ ↓ Viewing <Bookmarks>
« ↑ ↓ Viewing[SSL] <Hayyoth>
Opening socket...
« ↑ ↓ Viewing[SSL] <DuckDuckGo Lite: Fast, Lightweight Private Search>
TEXT:            (form field editing prompt; seen as "TEXT:" and "TEXT:beej")
lite.duckduckgo.com contacted. Waiting for reply...
« ↑ ↓ Viewing[SSL] <beej guide C at DuckDuckGo>
« ↑ ↓ Viewing[SSL] <Beej's Guide to C Programming>
Forward: string / Forward: string.h / Forward: strcat / Forward: browser / Forward: command
« ↑ ↓ Viewing <Option Setting Panel>
« ↑ ↓ Viewing <w3m help page>
```

## Code & shell
No shell commands typed in this segment; everything happens inside w3m. Key presses inferred from prompts/status (keys themselves are not displayed):
- `ESC-b` (view bookmarks), Enter on a link to follow it, Enter on a form field to edit (`TEXT:` prompt), Enter on `[Search]` to submit.
- `/` then text then Enter: the `Forward:` incremental text search prompt, used to jump around the Beej TOC (`string`, `string.h`, `strcat`), the option panel (`browser`) and the help page (`command`).
- `o` opens the Option Setting Panel; `H` opens the help page (`Viewing <w3m help page>`).
- Option edit shown: External browser `/usr/bin/firefox` changed to `open`.
- C code visible (in the Beej's Guide pages): Hello World program, strncpy example tail, strdup example tail, strcat/strncat prototypes (see Texts section).

## Observations
- Demonstrates w3m as a real daily-driver text browser: a bookmark list that holds the Linux kernel GitHub page (which renders as noisy JS junk) and his own site; searching with DuckDuckGo Lite; reading documentation (Beej's Guide to C, both the tutorial volume and the Library Reference volume).
- The sticky note is the "slide": `see bookmark` while showing bookmarks, then he adds `easy search and navigation` as the takeaway for the Beej's Guide + `/` search demo, then clears it before moving to configuration.
- Key teaching points: (1) bookmark pages and jump to them; (2) search from inside the terminal; (3) `/` forward search to jump inside long docs/TOC (e.g. `string.h`, `strcat`) - "easy search and navigation"; (4) w3m is configurable: `o` option panel, set External browser to `open` (macOS) so pressing `M` opens a page in the GUI browser; (5) `H` help lists all keys; he uses `/` search on the help page too.
- Content choices reveal his philosophy: his own page is a "White Paper" titled "Understand the Game. Don't Play It Blindly. The Art of Quitting." with a year/Everyone/I/Outcome table (study to understand not to get credentials; teach yourself programming; left corporate IT twice; software freedom/Old Git; farming; "Just do whatever I want"; "Quit quitting"). He keeps his video series list there: Opinions on AI, Build your own X from scratch, x - but better, Learn C, Data Structures in C, Golang, Lisp Language, Web Development. He recommends Beej's Guide as the C learning resource and reads it in the terminal; the Beej passage he scrolls through stresses that pointers are "the weird bit" and everything else in C is variations on things already known.
- He uses the screen live: types searches (typos like `bgclr` guessing the library reference URL), pages with space, and searches rather than scrolls. Webcam changes are only incidental (smoking, covering mouth, looking down while typing).
- Privacy note: a translator contact e-mail in a Beej search snippet was redacted as `<mail address>`.
