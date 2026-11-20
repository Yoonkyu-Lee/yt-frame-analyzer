# Segment 3 (09:21 - 10:06) transcript

Note: this segment contains NO Chez REPL typing and no notes-file editing. It is a browser tour of reference books (SICP PDF, then TSPL4 on scheme.com), followed by a desktop-overview (Mission Control-like) shot and the final screen. Webcam window "Human" ignored (face visible, bottom right).

## Timeline
- [09:21] Browser (Firefox) full screen, tab 1 "Structure and Interpretation of ..." URL `web.mit.edu/6.001/6.037/sicp.pdf`, PDF viewer "(1 of 883)", Automatic Zoom, Document outline sidebar open. Title page "Structure and Interpretation of Computer Programs", cover engraving. Outline entries: Unofficial Texinfo Format, Dedication, Foreword, Preface to the Second Edition, Preface to the First Edition, Acknowledgments, Building Abstractions with Procedures, Building Abstractions with Data, Modularity, Objects, and State, Metalinguistic Abstraction, Computing with Register Machines, References, List of Exercises, List of Figures, Index, Colophon.
- [09:22] Scrolled slightly: "SECOND EDITION / Unofficial Texinfo Format 2.andresrabas.6 (as read: "2.andresraba5.6" ⟨?⟩) / Harold Abelson, Gerald Jay Sussman with Julie Sussman, foreword by A⟨lan Perlis⟩ (partly hidden by webcam)".
- [09:23] Mouse on "+" new-tab button.
- [09:24] New tab opened; typing "tsp" in URL bar; Firefox Suggest lists "The Scheme Programming Language, 4th Edition - scheme.com/tspl4/".
- [09:25] URL bar "tspl4"; suggestion scheme.com/tspl4/ at top.
- [09:26] Google search `google.com/search?client=firefox-b-d&q=tspl4` loading; top result "Chez Scheme - https://www.scheme.com > tspl4 - The Scheme Programming Language, 4th Edition - Table of Contents · Section 1.1. Scheme Syntax · Section 1.2 Scheme Naming Conventions · Section 1.3 Typographical and Notational Conventions."
- [09:27] Cursor on result link.
- [09:28] `www.scheme.com/tspl4/` loaded: cover "THE SCHEME PROGRAMMING LANGUAGE, Fourth Edition, R. Kent Dybvig, Illustrations by Jean-Pierre Hébert"; start of Table of Contents (Preface, Ch.1 Introduction 1.1-1.3, Ch.2 Getting Started 2.1-2.8...).
- [09:29]-[09:35] Scrolls through the Table of Contents, down and slightly back up (09:34-09:35 scroll back up to Ch.6-8). Chapters seen: 1 Introduction; 2 Getting Started (2.1 Interacting with Scheme, 2.2 Simple Expressions, 2.3 Evaluating Scheme Expressions, 2.4 Variables and Let Expressions, 2.5 Lambda Expressions, 2.6 Top-Level Definitions, 2.7 Conditional Expressions, 2.8 Simple Recursion, 2.9 Assignment); 3 Going Further (3.1 Syntactic Extension, 3.2 More Recursion, 3.3 Continuations, 3.4 Continuation Passing Style, 3.5 Internal Definitions, 3.6 Libraries); 4 Procedures and Variable Bindings (4.1 Variable References, 4.2 Lambda, 4.3 Case-Lambda, 4.4 Local Binding, 4.5 Multiple Values, 4.6 Variable Definitions, 4.7 Assignment); 5 Control Operations (5.1 Procedure Application, 5.2 Sequencing, 5.3 Conditionals, 5.4 Recursion and Iteration, 5.5 Mapping and Folding, 5.6 Continuations, 5.7 Delayed Evaluation, 5.8 Multiple Values, 5.9 Eval); 6 Operations on Objects (6.1 Constants and Quotation, 6.2 Generic Equivalence and Type Predicates, 6.3 Lists and Pairs, 6.4 Numbers, 6.5 Fixnums, 6.6 Flonums, 6.7 Characters, 6.8 Strings, 6.9 Vectors, 6.10 Bytevectors, 6.11 Symbols, 6.12 Booleans, 6.13 Hashtables, 6.14 Enumerations); 7 Input and Output (7.1 Transcoders ... 7.11 Bytevector/String Conversions); 8 Syntactic Extension (8.1 Keyword Bindings, 8.2 Syntax-Rules Transformers, 8.3 Syntax-Case Transformers, 8.4 Examples); 9 Records (9.1 Defining Records, 9.2 Procedural Interface, 9.3 Inspection); 10 Libraries and Top-Level Programs (10.1 Standard Libraries, 10.2 Defining New Libraries, 10.3 Top-Level Programs, 10.4 Examples); 11 Exceptions and Conditions (11.1 Raising and Handling Exceptions, 11.2 Defining Condition Types, 11.3 Standard Condition Types); 12 Extended Examples (12.1 Matrix and Vector Multiplication, 12.2 Sorting, 12.3 A Set Constructor, 12.4 Word Frequency Counting, 12.5 Scheme Printer, 12.6 Formatted Output, 12.7 A Meta-Circular Interpreter for Scheme, 12.8 Defining Abstract Objects, 12.9 Fast Fourier Transform, 12.10 A Unification Algorithm, 12.11 Multitasking with Engines); then References, Answers to Selected Exercises, Formal Syntax, Summary of Forms, Index (09:34 frame shows bottom of TOC).
- [09:36] Scrolled back up to Ch.6 (09:36 frame at 6.1-7.11).
- [09:37] Cursor highlighting (text-selecting by click) link "Section 6.12. Booleans".
- [09:38] Highlights "Section 6.11. Symbols" (status bar link `www.scheme.com/tspl4/objects.html#./objects:h10`).
- [09:39] Highlights "Section 6.13. Hashtables" (link `.../objects.html#./objects:h13`), page starts loading.
- [09:40] Navigated to `www.scheme.com/tspl4/objects.html#./objects:h13` ("Operations on Objects"); page loading, cover art (Jean-Pierre Hebert drawing) rendering, "Chapter 6. Operations on Objects" intro paragraph.
- [09:41]-[09:42] Cover art drawing continues; scrolls to "Section 6.1. Constants and Quotation" (syntax: constant; returns: constant; examples `3.2 => 3.2`, `#f => #f`, `#\c => #\c`, `"hi" => "hi"`, `#vu8(3 4 5) => #vu8(3 4 5)`).
- [09:43] Jumps to "Section 6.13. Hashtables": text "Hashtables represent sets of associations between arbitrary Scheme values. They serve essentially the same purpose as association lists (see page 165) but are typically much faster when large numbers of associations are involved." Procedures make-eq-hashtable, make-eqv-hashtable, make-hashtable.
- [09:44]-[09:45] Lingers on 6.13 start (make-eq-hashtable, `(define ht1 (make-eq-hashtable))`, `(define ht2 (make-eq-hashtable 32))`); at 09:45 text cursor on "make-eq-hashtable".
- [09:46] Scrolls down: make-hashtable hash equiv?, `(define ht (make-hashtable string-hash string=?))`, hashtable-mutable?, `(hashtable-mutable? (make-eq-hashtable)) => #t`, `(hashtable-mutable? (hashtable-copy (make-eq-hashtable))) => #f`, hashtable-hash-function, hashtable-equivalence-function.
- [09:47] Back to `www.scheme.com/tspl4/` (back button; forward arrow now active), TOC at Ch.5-7.
- [09:48] Scrolls up TOC (Ch.3-6).
- [09:49] Further up (Ch.2-5); highlights "Section 4.7. Assignment" (link `binding.html#./binding:h7`).
- [09:50] Highlights "Section 4.5. Multiple Values" (link `binding.html#./binding:h5`).
- [09:51] Opens `www.scheme.com/tspl4/binding.html#./binding:h5` ("Procedures and Variable Bindings", Chapter 4 intro: "Procedures and variable bindings are the fundamental building blocks of Scheme programs ... define, letrec, let-values, and set!").
- [09:52] "Section 4.5. Multiple Values": let-values / let*-values syntax, `(let-values ([(a b) (values 1 2)] [c (values 1 2 3)]) (list a b c)) => (1 2 (1 2 3))`, `(let*-values ([(a b) (values 1 2)] [(a b) (values b a)]) (list a b)) => (2 1)`. "Section 4.6. Variable Definitions" with define syntax forms.
- [09:55]-[09:59] Scrolls slowly through 4.6 Variable Definitions (define var expr; define var; define (var0 var1 ...) body1 body2 ...; define (var0 . varr); ...; text "In the first form, define creates a new binding of var to the value of expr..."; `(define x 3) x => 3`; `(define f (lambda (x y) ...` ), then scrolls back up to 4.5 (at 09:58-09:59 top shows `(eq? (g) g)) => exception: attempt to reference undefined variable f`; cursor on `(list a b c)) => (1 2 (1 2 3))`).
- [10:00] Same view; text cursor in `(values 1 2)` area of the let-values example (a selection `(a b) (values 1 2)` is later shown highlighted).
- [10:01] Desktop overview / window-shuffle animation (macOS Desktop 2 of 3 in spaces bar: "Desktop 1, Desktop 2, Desktop 3"). Left terminal (Chez REPL) visible; right terminal (notes) behind browser; the browser window shrunk. Dock visible.
- [10:02] Windows tiled: left terminal REPL, browser, right terminal with notes in VISUAL mode; webcam shrunk.
- [10:04] Browser back to full screen; notes terminal (right, "Terminal") now in front over browser. Notes bottom visible with `~` lines; cursor at "level".
- [10:06] Notes last line highlighted in VISUAL (49 characters selected).
- [10:07] (frame 10:06 final) Mac menu bar appears (Terminal, Shell, Edit, View, Window, Help; time "Sat Oct 17 8:59 PM"); mouse at screen-recording/camera indicator in menu bar top (likely about to stop recording). Last frame in list.

## File states

### (a) Notes file (right terminal, vim; only lower part visible; top of file / "# Table of contents" NOT visible in this segment)
Most complete view (frame 10:04/10:06 (window scrolled to end), plus 10:01 showing the earlier list items above):
```
- car, cdr
- loop through list
- apply
- map

code is data
- eval

another feature
- very minimal and fast (f** simple)
- generic data type
- system call
- call c code easy
- recursion very fast
- chez scheme and sbcl common lisp very fast
- repl - code playground very easy
- script language, JIT compiler
- all feature in high level language have in lisp

~
~
~
-- VISUAL --                                   49
```
Notes:
- Line numbers not shown. File ends with "- all feature in high level language have in lisp", then one blank line, then `~` empty-buffer markers.
- At [10:01] a selection partly shows "in lisp" highlighted; at [10:02] "script language, JIT compiler" and "- all feature in high level language have in lisp" are both highlighted in VISUAL mode (status shows "2" = 2 lines selected).
- At [10:04] no selection (normal mode, cursor on "l" of "level"); at [10:06] last line "- all feature in high level language have in lisp" selected in VISUAL (49 chars).
- No new lines appeared compared with the prior segment's end state as far as visible here (the visible block contains only the items above). Items above "- car, cdr" are not visible in this segment.

### (b) REPL transcript of this segment (left terminal, visible at [10:01]-[10:02] only, static)
```
> (system "ls")
Desktop   Downloads Library   Music     notes.md  Public
Documents lab       Movies    notes     Pictures  sources
0
> (system "mkdir abc")
0
> (system "ls")
abc       Documents lab
Desktop   Downloads Libra⟨ry⟩
0
> book to read▯
```
(Right edge of 3rd `ls` output is cut off by overlapping browser window; "notes.md Public..." etc. columns hidden at [10:01]-[10:02].) The last prompt line contains typed text "book to read" with block cursor, evidently typed earlier as a note-to-self/reminder in the REPL (not evaluated; it would be a syntax error if entered). No REPL input occurs during this segment.

## Shell & outputs
- No shell commands run during this segment. The only terminal text seen is the static REPL state above (from earlier in the video: `(system "ls")`, `(system "mkdir abc")`, `(system "ls")` and the pending `book to read`), visible at [10:01]-[10:02].
- Browser URLs visited, in order:
  - [09:21] web.mit.edu/6.001/6.037/sicp.pdf (SICP, 883 pages, "Unofficial Texinfo Format 2.andresraba5.6")
  - [09:26] google.com/search?client=firefox-b-d&q=tspl4
  - [09:28] www.scheme.com/tspl4/
  - [09:40] www.scheme.com/tspl4/objects.html#./objects:h13 (Ch.6, Hashtables)
  - [09:47] back to www.scheme.com/tspl4/
  - [09:51] www.scheme.com/tspl4/binding.html#./binding:h5 (Ch.4, Multiple Values / Variable Definitions)

## Observations
- Recommended references: (1) SICP (Structure and Interpretation of Computer Programs, Abelson and Sussman, Second Edition, shown as PDF at web.mit.edu/6.001/6.037/sicp.pdf), shown first, only briefly (about 3 s of scrolling, with a click on new tab). (2) "The Scheme Programming Language, 4th Edition" by R. Kent Dybvig (scheme.com/tspl4), found via typing "tsp" / "tspl4" into the URL bar and Google; he spends the rest of the segment (09:28 to 10:00) scrolling its table of contents and opening two sections. The REPL line "book to read" (typed in the Chez REPL prompt on the left terminal, visible 10:01-10:02) shows that the "book to read" recommendation was his closing topic.
- He lingers on: TOC Ch.6 "Operations on Objects" (highlighting 6.11 Symbols, 6.12 Booleans, 6.13 Hashtables, then opens 6.13 Hashtables, reading make-eq-hashtable, make-hashtable examples); TOC Ch.4 (highlighting 4.7 Assignment, then 4.5 Multiple Values, opens it and reads 4.5 let-values and 4.6 Variable Definitions/define). Also scrolls past Ch.5.9 Eval and Ch.12.7 "A Meta-Circular Interpreter for Scheme" (visible, not specifically highlighted).
- Closing: after the browser tour, he arranges the windows (macOS multi-space/desktop view at 10:01-10:02), brings the right-hand notes terminal to the front, VISUAL-selects the last bullet "- all feature in high level language have in lisp" (the notes' final topic), and the final frame (10:06) shows the macOS menu bar (Terminal menus, clock "Sat Oct 17 8:59 PM") with mouse pointer at the menu-bar screen-recording indicator, i.e. he is about to stop the recording. No outro title card, no code run at the end.
- Philosophy (notes wording, verbatim on screen): "very minimal and fast (f** simple)", "generic data type", "system call", "call c code easy", "recursion very fast", "chez scheme and sbcl common lisp very fast", "repl - code playground very easy", "script language, JIT compiler", "all feature in high level language have in lisp". Final point: Lisp already has every feature of high-level languages; Lisp is the playground ("code playground very easy").
- Time stamp visible: Sat Oct 17, 8:59 PM (recording clock).
