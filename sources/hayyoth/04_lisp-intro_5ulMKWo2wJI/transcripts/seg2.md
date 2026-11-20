# Segment 2 transcript (04:50 - 09:21)

## Timeline

[04:50] Notes: vim VISUAL line-select on "- recursion made easy" (status "-- VISUAL --  21"). REPL already contains the factorial demo, including two errors and recoveries:
 - First `(define (factorial n) (if (= n 1) 1 (n * (factorial (- n 1)))))` (infix `n *` typo). `(factorial 3)` -> `Exception: attempt to apply non-procedure 2` / `Type (debug) to enter the debugger.`
 - Redefines with `(* n (factorial (- n 1)))`. `(factorial 3)` -> `6`.
 - Typo `(fatorial 4)` -> `Exception: variable fatorial is not bound` / `Type (debug) to enter the debugger.` (He leaves the error as is and retypes.) `(factorial 4)` -> `24`.
 - `(factorial 10)` -> `3628800`. `(factorial 100)` -> a 158-digit number ending in many zeros (93326215443944152681699238856266700490715968264381621468592963895217599993229915608941463976156518286253697920827223758251185210916864000000000000000000000000).
 (The factorial define/typo/error portion happened before 04:50; only its on-screen scrollback is visible here, so exact typing times are unknown.)
[04:52] `(factorial 1000)` is run (prompt line visible at top); prints a ~2568-digit number filling the terminal (starts 40238726007709377354370243392300398571937486421071463254379991042993851239862..., ends in a long run of zeros). Notes unchanged (recursion made easy highlighted, VISUAL 21).
[04:54] Terminal cleared (only `> ` prompt). Notes unchanged.
[05:26] Sub-topic "loop". Typed:
```
> (define (loop i n)
    (display i)
    (newline)
    (if (< i n)
        (loop (+ i 1) n)))
> (loop 2 10)
2
3
4
5
6
7
8
9
10
```
Notes still highlight "recursion made easy" (VISUAL 21).
[05:34] `(loop 10 20)` -> prints 10 through 20, one per line. Notes view has scrolled: top now shows `define / - variable / - function`; the "recursion made easy" highlight is still on; mouse cursor over "list data struct". Notes now additionally visible below: `code is data / - eval / another feature / - very minimal and fast (f** simple)`.
[05:35] Highlight gone (VISUAL ended); a vim block cursor sits on the "r" of "car, cdr". Terminal still shows loop output.
[05:37] Terminal cleared; he has begun typing `> (` (new topic: list data struct, car, cdr).
[06:03] Typed and results:
```
> '(1 2 3)
(1 2 3)
> (define my-list '(10 20 30 40))
> (car my-list)
10
> (cdr my-list)
(20 30 40)
```
[06:49] Starts "loop through list": typing `(define (print-list ls) (display (car ls)) (newline) (if (not (null? (cdr ls))) (print-list` ... (in progress, cut off at `(print-list`).
[07:10] Completed define (see File states). `(print-list my-list)` -> 10 20 30 40 (one per line). `(print-list '(1 2 3))` -> 1 2 3. Notes: cursor on "r" of cdr highlighted (single-char visual); mouse over notes pane.
[07:29] `(apply + '(1 2 3 4))` -> `10`; `(apply + '(1 2 3))` -> `6`; starts typing `(map (lambda (` .
[07:46] `(map (lambda (x) (* x 10)) '(1 2 3 4))` -> `(10 20 30 40)`. Notes pane scrolled: shows `another feature` list now including `- generic data type`, `- system call`, `- call c code easy`. Mouse at "code is data".
[08:07] "code is data": `(define my-code '(+ 1 2 3))`, `(eval my-code)` -> `6`. He has also selected `(apply + '(1 2 3 4))` with the mouse (terminal text highlight) while talking.
[08:12] Typing `(define my-code '(apply + '(1 2 3 4)))` (shown without trailing newline yet).
[08:31] Done: `my-code` -> `(apply + '(1 2 3 4))`; `(eval my-code)` -> `10`. The output `10` line (of `(apply + '(1 2 3 4))`) is mouse-highlighted at 08:31.
[08:32] Mouse cursor over terminal; notes unchanged.
[08:33] Notes pane scrolled to bottom: lines `- repl - code playground very easy`, `- script language, JIT compiler`, `- all feature in high level language have in lisp`, then `~` filler lines (end of file). Cursor block at "co" in "code".
[08:34] Notes scrolled so end-of-file shows the full "another feature" list (see File states). Cursor in "code playground".
[08:35] Scrolled further; same list visible with "another feature" heading.
[08:37] VISUAL line-select on "- very minimal and fast (f** simple)" (VISUAL 36).
[08:38] Highlight on "- generic data type" (VISUAL 19).
[08:39] Mouse over "system call" (single-char selection at the "l"); no VISUAL highlight line.
[08:41] VISUAL line-select on "- system call" (VISUAL 13). Terminal unchanged.
[08:42] Terminal cleared to `> ` prompt. Highlight still on "system call".
[08:46] `(system "ls")` typed and run. Output:
```
Desktop   Downloads Library   Music     notes.md  Public
Documents lab       Movies    notes     Pictures  sources
0
```
(directory names in cyan; notes.md in white; `0` is the exit status return value)
[08:54] `(system "mkdir abc")` -> `0`; `(system "ls")` now lists `abc` first:
```
abc       Documents lab       Movies    notes     Pictures  sources
Desktop   Downloads Library   Music     notes.md  Public
0
```
[08:57] `abc` is mouse-highlighted in the terminal; notes highlight ended; mouse on "recursion very fast".
[08:58] Notes: line-select on "- recursion very fast" (VISUAL 21). Terminal unchanged.
[08:59] Notes: partial visual on "fast" of "chez scheme ... very fast" (VISUAL 5).
[09:00] Notes: line-select on "- chez scheme and sbcl common lisp very fast" (VISUAL 44).
[09:01] Notes: partial select "- code playground very easy" portion of the repl line (VISUAL 27). (No terminal demo for "call c code easy", "recursion very fast", "chez ... fast", "repl" in this segment -- he moves through them by talking only.)
[09:03] Notes: cursor on "in" in "all feature in high level language have in lisp", no VISUAL highlight.
[09:05] Notes: line-select over last two lines: "- script language, JIT compiler" and "- all feature in high level language have in lisp" (VISUAL 2).
[09:11] Terminal has `> book to read` typed (not run, literal text at prompt; the `abc` highlight gone). macOS Dock appears at bottom with tooltip "Firefox" -- he is switching to the browser.
[09:12] Firefox "New Tab" with address bar focused ("Search with Google or enter address"). Webcam still overlaid.
[09:14] Typed `sicp` in address bar; suggestions: "sicp - Search with Google" and Firefox Suggest "Structure and Interpretation of Computer Programs, 2nd ed. - sicp.pdf  web.mit.edu/6.001/6.037/sicp.pdf".
[09:15] Google search results page `www.google.com/search?client=firefox-b-d&q=sicp` loading ("Thinking").
[09:16] Results visible: MIT "Structure and Interpretation of Computer Programs, 2nd ed." (web.mit.edu > sicp, PDF, 883 pages), Wikipedia, sarabander.github.io/sicp (2e), "Welcome to the SICP Web Site" (mitp-content-server.mit.edu ... sicp.zip). Cursor on "Show more".
[09:18] Hovering/clicking the MIT PDF result (status bar `web.mit.edu/6.001/6.037/sicp.pdf`), page loading.
[09:19] Same, cursor moved right; status "Looking up ogs.google.com...".
[09:20] SICP PDF open in Firefox viewer: `web.mit.edu/6.001/6.037/sicp.pdf`, page 1 of 883, "Structure and Interpretation of Computer Programs", "SECOND E[dition] / Unofficial Texinfo Format / Harold Abelson and Gerald Jay Sussman"; Document outline sidebar: Unofficial Texinfo Format, Dedication, Foreword, Preface to the Second Edition, Preface to the First Edition, Acknowledgments, Building Abstractions with Procedures, Building Abstractions with Data, Modularity, Objects, and State, Metalinguistic Abstraction, Computing with Register Machines, References, List of Exercises, List of Figures, Index, Colophon.
[09:21] Same; slightly scrolled. End of segment frames.

## File states

### (a) Notes file (most complete, as visible in this segment; the top part from seg1 not visible here beyond what shows at 04:50; no line numbers shown on screen)
```
# Table of contents

- some basic operator
- real numbers are always precise.
- large number never overflow

define
  - variable
  - function

- recursion made easy
  - factorial
  - loop

list data struct
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
```
(Lines are dash bullets in yellow; sub-bullets of define/recursion indented by 2 spaces. At 04:50 the "list data struct" section appears as shown with `- loop through list`, `- apply`, `- map`. "f**" is as displayed on screen (likely censored by the author in the file itself).)

### (b) REPL transcript (segment 2, verbatim)
```
> (define (factorial n)
    (if (= n 1)
        1
        (n * (factorial (- n 1)))))
> (factorial 3)
Exception: attempt to apply non-procedure 2
Type (debug) to enter the debugger.
> (define (factorial n)
    (if (= n 1)
        1
        (* n (factorial (- n 1)))))
> (factorial 3)
6
> (fatorial 4)
Exception: variable fatorial is not bound
Type (debug) to enter the debugger.
> (factorial 4)
24
> (factorial 10)
3628800
> (factorial 100)
93326215443944152681699238856266700490715968264381621468592963895217599993229
91560894146397615651828625369792082722375825118521091686400000000000000000000
0000
> (factorial 1000)
40238726007709377354370243392300398571937486421071463254379991042993851239862
...(~2568 digits; see frame 04:52; ends with a long run of zeros)
[screen cleared]
> (define (loop i n)
    (display i)
    (newline)
    (if (< i n)
        (loop (+ i 1) n)))
> (loop 2 10)
2
3
4
5
6
7
8
9
10
> (loop 10 20)
10
11
12
13
14
15
16
17
18
19
20
[screen cleared]
> '(1 2 3)
(1 2 3)
> (define my-list '(10 20 30 40))
> (car my-list)
10
> (cdr my-list)
(20 30 40)
> (define (print-list ls)
    (display (car ls))
    (newline)
    (if (not (null? (cdr ls)))
        (print-list (cdr ls))))
> (print-list my-list)
10
20
30
40
> (print-list '(1 2 3))
1
2
3
> (apply + '(1 2 3 4))
10
> (apply + '(1 2 3))
6
> (map (lambda (x) (* x 10)) '(1 2 3 4))
(10 20 30 40)
> (define my-code '(+ 1 2 3))
> (eval my-code)
6
> (define my-code '(apply + '(1 2 3 4)))
> my-code
(apply + '(1 2 3 4))
> (eval my-code)
10
[screen cleared]
> (system "ls")
Desktop   Downloads Library   Music     notes.md  Public
Documents lab       Movies    notes     Pictures  sources
0
> (system "mkdir abc")
0
> (system "ls")
abc       Documents lab       Movies    notes     Pictures  sources
Desktop   Downloads Library   Music     notes.md  Public
0
> book to read      <- text typed at prompt at 09:11, not executed (then he leaves the terminal for Firefox)
```
(Note: the `(define (factorial n)` first attempt is in the 04:50 scrollback; the digit lines of factorial 100 wrap at 77 chars on screen: row 1 `93326215443944152681699238856266700490715968264381621468592963895217599993229`, row2 `915608941463976156518286253697920827223758251185210916864000000000000000000000`... exact wrapping as shown at 04:50 is `...99993229 / 915608941463976156518286253697920827223758251185210916864000000000000000000000 / 0000`; the transcription above may differ slightly in digit grouping.)

## Shell & outputs

- [08:46] `(system "ls")` in Chez REPL -> lists `Desktop Downloads Library Music notes.md Public / Documents lab Movies notes Pictures sources`, return value `0`.
- [08:54] `(system "mkdir abc")` -> `0` (no output), directory abc created.
- [08:54] `(system "ls")` -> now includes `abc`; return `0`.
- No separate shell prompt commands in this segment.
- Browser (Firefox): typed `sicp` -> Google results -> opened `web.mit.edu/6.001/6.037/sicp.pdf` (SICP 2nd ed., 883 pages).

## Observations

- Teaching order in this segment: recursion (factorial with live errors, huge numbers 100, 1000), recursive "loop" counting with display/newline/if, list data structure (quote `'(1 2 3)`, define a list, car, cdr), recursion over a list (`print-list`, with `null?` / `not` base case), apply (`+` over a list), map with lambda, "code is data" (quoted list `'(+ 1 2 3)` then `eval`; then a quoted expression that itself contains a quoted list and `apply`), then a rapid "another feature" tour through the notes (minimal and fast, generic data type, system call, call c code, recursion very fast, chez scheme / sbcl, repl as playground, script language / JIT compiler, all features in high-level languages exist in lisp), ending with a "book to read" pointer: SICP.
- Demonstration style: minimal one-line examples typed live; each notes topic is highlighted in vim VISUAL mode as he works on it; screen cleared (Ctrl-L) between topics. Errors are not hidden: infix `n *` typo -> "attempt to apply non-procedure 2" fixed by prefix `(* n ...)`; misspelled `fatorial` -> "variable fatorial is not bound", then retyped correctly.
- "Code is data" shown by quoting an expression into a variable (`my-code`), printing it as a list, then `eval`ing it; he deliberately changes the quoted code from `'(+ 1 2 3)` (6) to `'(apply + '(1 2 3 4))` (10), tying together apply, quote and eval.
- `(system "ls")` / `(system "mkdir abc")` show the "system call" feature: Scheme drives the shell; return value 0 is the exit code. He did not demo "call c code easy", speed claims, or JIT in the REPL in these frames (only noted in file).
- Notes wording is informal and opinionated: "recursion made easy", "very minimal and fast (f** simple)", "all feature in high level language have in lisp", "repl - code playground very easy". The "f**" is as displayed.
- Ends with typing "book to read" as a literal at the prompt, then opening Firefox and searching "sicp", landing on the MIT PDF of SICP (2nd ed.) -- the recommended book. TSPL/scheme.com not seen in this segment.
- Webcam window "Human" ignored (shown throughout; remains overlaid in browser).
