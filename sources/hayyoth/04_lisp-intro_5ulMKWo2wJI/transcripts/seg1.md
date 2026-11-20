# Segment 1 transcript (00:00 - 04:50), video 5ulMKWo2wJI

All 41 frames in seg1_frames.txt were viewed. Layout: left = Terminal (shell then Chez REPL), right-top = Terminal with vim notes, right-bottom = webcam "Human" (ignored).

## Timeline

- [00:00] Left terminal: empty shell prompt `~ $ `. Right-top vim notes show the full TOC (see File states), no highlight, cursor on `#` of line 1.
- [00:12] Left terminal shows:
  - `~ $ # In this video , i will make you love lisp` (a shell comment)
  - `~ $ chez`
  - `Chez Scheme Version 10.4.1`
  - `Copyright 1984-2026 Cisco Systems, Inc.`
  - blank line, then prompt `> `
- [00:16] Same; the two banner lines are mouse-selected (he points at version/copyright). Notes unchanged.
- [00:17] Selection gone; mouse in blank area. Notes unchanged.
- [00:24] Left terminal cleared; one line `> (` being typed. Notes unchanged, no highlight.
- [00:42] Notes: "- real numbers are always precise." highlighted in VISUAL mode (status `-- VISUAL --  32`). REPL (after clear):
  - `(+ 1 2)` -> 3
  - `(+ 1 2 3)` -> 6
  - `(- 1 2)` -> -1
  - `(- 2 3)` -> -1
  - `(- 2 2)` -> 0
  - (so "some basic operator" was covered before the highlight reached the next line; the highlight at this point is already on "real numbers...")
- [01:19] Still "real numbers are always precise." highlighted (32). New REPL entries:
  - `(/ 2 3)` -> 2/3
  - `(/ 1 1)` -> 1
  - `(/ 4 2)` -> 2
  - `(* (/ 2 3) (/ 3 2))` -> 1
  - typing `(+ (/ 2 3) (/ 1 3)` (closing paren not yet typed)
- [01:24] Highlight moved to "- large number never overflow" (VISUAL 28). `(+ (/ 2 3) (/ 1 3))` -> 1. Empty prompt.
- [01:35] `(* 10 10)` -> 100. Typing `(* 10 1000000000000000000000` (long run of zeros, mid-typing).
- [01:37] `(* 10 1 followed by many zeros)` -> that number with one more zero (fits on one line, about 72 zeros in the input; exact count not reliably countable).
- [01:38] Typing a second `(* 10 1000...)`, same style, closing paren just typed.
- [01:43] Second one done: input wraps to 2 lines, result (1 followed by zeros, one more zero) wraps to 2 lines.
- [01:44] Screen cleared; empty `> `.
- [01:55] After clear:
  - `(expt 2 1)` -> 2
  - `(expt 2 2)` -> 4
  - `(expt 2 3)` -> 8
  - `(expt 2 10)` -> 1024
  - `(expt 2 100)` -> 1267650600228229401496703205376
- [01:57] `(expt 2 1000)` -> 4 rows of digits (see File states b).
- [01:59] Screen full of digits scrolling; bottom of screen ends `...96709376` then `> ` (this is the tail of a huge result; the expression is `(expt 2 10000)` per the 02:01/02:02 frames).
- [02:01] Scrollback view shows the earlier session: `1`, `> (* 10 10)` -> 100, the two big multiplications, `(expt 2 1)` ... `(expt 2 1000)`, then `> (expt 2 10000)` with first rows `19950631168807583848837421626835850838234968318861924548520089498529438830221`, `94663191996168403619459789933112942320912427155649134941378111759378593209632`, `39578557300467937945267652465512660598955205500869181933115425086084606181046`. Mouse/webcam: he gestures; terminal being scrolled by him.
- [02:02] Scrolled further down in the `(expt 2 10000)` output (rows continue).
- [02:03] Bottom of the `(expt 2 10000)` output: last rows `...15947504097321150804981904558034168269497871413160632106863915116817743047925` / `96709376`, then `> `. (Same as 01:59.)
- [02:06] A different huge digit block, last row `...13945703573905374227697796338115683962232084025970251553047343898831 09376`; rows begin `20318111438440400511461399032627336214615066785188007932756348644480372696327`; prompt `> `. The command that produced this is not visible (probably a larger exponent after 10000; unconfirmed).
- [02:07] Mid-scroll through huge digit output (top row `47342275885803162473648253508879276299074813242692461714169521068548818275361`).
- [02:08] Mid-scroll (top row `41762680345281279467292444529191618820953359456671584509414763015370325881092`).
- [02:09] Mid-scroll (top row `71741933750491929536329510111529295183628281919182165167645594651582804898425`).
- [02:10] Back at the bottom of the huge output: same screen as 02:06, ends `...883109376`, prompt `> `.
- [02:14] Same, prompt now displays with a `[` mark at left (new prompt line registered).
- [02:15] Screen cleared: `> ` only.
- [02:17] REPL terminal unfocused. Notes: VISUAL exited; cursor on last char `e` of `variable` (single-char highlight), mouse I-beam beside `function`. No "large number" highlight.
- [02:18] Notes: VISUAL status `3`: the 3-line block `define` / `- variable` / `- function` highlighted (cursor at `d` of define). REPL empty.
- [02:31] "define" block highlighted. REPL (after clear):
  - `(define  x 10)` (two spaces after define, as seen) -> no output
  - `(define y 20)` -> no output
  - `x` -> 10
  - `y` -> 20
  - `(+ x y)` -> 30
  - typing `(- x`
- [02:56] Continues:
  - `(- x y)` -> -10
  - `(define (add x y)` newline `    (+ x y))`
  - `(add 2 3)` -> 5
  - `(add 3 4)` -> 7
  - `(define (sub x y)` newline `    (- x y))`
  - typing `(sub ` (arguments not yet visible)
- [03:06] Screen cleared; typing `(defi`.
- [03:19] After clear:
  - `(define (+ x y)` newline `    (- x y))` -> he redefines built-in `+` as subtraction
  - `(+ 3 4)` -> `-1` (that line and result are mouse-highlighted, he points at them)
  - `> `
- [03:23] Chez exited (shell prompt `~ $ ` below a bare `> ` line). Screen shows `(define (+ x y)`, `(- x y))`, `(+ 3 4)`, `-1`, `>`, `~ $ `.
- [03:25] Screen cleared; `~ $ chez` run again, same banner, `> `. Notes still show the define block highlighted.
- [03:27] Same left screen (terminal unfocused). Notes: VISUAL exited; cursor on the `y` of `easy` in "recursion made easy", mouse I-beam beside it.
- [03:28] Notes: "- recursion made easy" highlighted (VISUAL, 21). REPL: banner and `> ` only.
- [04:01] REPL:
  - `(define (factorial n)` / `    (if (= n 1)` / `        1` / `        (n * (factorial (- n 1)))))` (infix `*`, a bug)
  - typing `(`
- [04:04] `(factorial 3)` ->
  - `Exception: attempt to apply non-procedure 2`
  - `Type (debug) to enter the debugger.`
- [04:40] Recovery: retyped the definition with prefix `*`:
  - `(define (factorial n)` / `    (if (= n 1)` / `        1` / `        (* n (factorial (- n 1)))))`
  - `(factorial 3)` -> 6
  - typing `(fatorial 4` (typo)
- [04:41] `(fatorial 4)` ->
  - `Exception: variable fatorial is not bound`
  - `Type (debug) to enter the debugger.`
- [04:50] Retyped correctly:
  - `(factorial 4)` -> 24
  - `(factorial 10)` -> 3628800
  - `(factorial 100)` -> 3-row number (see below)
  - Notes still highlight "recursion made easy" (VISUAL 21); `factorial` / `loop` sub-items not yet individually highlighted.

## File states

### (a) Notes file (vim; no line numbers shown; only `-- VISUAL --` and a selection count at bottom)

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
```
Text unchanged throughout 00:00-04:50. Window shows 16 lines only; any lines below `- map` (e.g. apply / eval / code is data / other topics mentioned by the task) are not visible in this segment (??). In the first frame the cursor sits on `#`. Selection counts seen in the status line: 32, 28, 3, 21.

### (b) REPL transcript (verbatim as readable; screen clears omitted)

```
~ $ # In this video , i will make you love lisp
~ $ chez
Chez Scheme Version 10.4.1
Copyright 1984-2026 Cisco Systems, Inc.

> (+ 1 2)
3
> (+ 1 2 3)
6
> (- 1 2)
-1
> (- 2 3)
-1
> (- 2 2)
0
> (/ 2 3)
2/3
> (/ 1 1)
1
> (/ 4 2)
2
> (* (/ 2 3) (/ 3 2))
1
> (+ (/ 2 3) (/ 1 3))
1
> (* 10 10)
100
> (* 10 1<~72 zeros, one line>)
1<one more zero than the input>
> (* 10 1<~100+ zeros, wraps to 2 lines>)
1<one more zero than the input, wraps to 2 lines>
> (expt 2 1)
2
> (expt 2 2)
4
> (expt 2 3)
8
> (expt 2 10)
1024
> (expt 2 100)
1267650600228229401496703205376
> (expt 2 1000)
10715086071862673209484250490600018105614048117055336074437503883703510511249
36122493198378815695858127594672917553146825187145285692314043598457757469857
48039345677748242309854210746050623711418779541821530464749835819412673987675
591655439460770629145711964776865421676604298316526243868372056 8069376   [last row approx; ends ...069376]
> (expt 2 10000)
19950631168807583848837421626835850838234968318861924548520089498529438830221
94663191996168403619459789933112942320912427155649134941378111759378593209632
39578557300467937945267652465512660598955205500869181933115425086084606181046
... (about 40 rows, not transcribed) ...
15947504097321150804981904558034168269497871413160632106863915116817743047925
96709376
> <expression not visible; another huge result ending ...883109376>
> (define  x 10)
> (define y 20)
> x
10
> y
20
> (+ x y)
30
> (- x y)
-10
> (define (add x y)
    (+ x y))
> (add 2 3)
5
> (add 3 4)
7
> (define (sub x y)
    (- x y))
> (sub ???            [arguments/result not captured]
> (define (+ x y)
    (- x y))
> (+ 3 4)
-1
>                     [exit chez]
~ $ chez
Chez Scheme Version 10.4.1
Copyright 1984-2026 Cisco Systems, Inc.

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
```
Uncertainties: exact zero counts in the two `(* 10 100...)` tests; exact last row of `(expt 2 1000)` (read as `...5668069376`); the expression behind the second huge output at 02:06/02:10; the middle rows of the `(expt 2 10000)` output not transcribed (only digits seen at the top and bottom). Row splits of `(factorial 100)` are as displayed at 80-ish col width but the digits are read at screenshot resolution (low-confidence on exact characters; the known value of 100! begins 93326215443944152681699238856266700490715968264381621468592963895217599993229915608941463976156518286253697920827223758251185210916864000000000000000000000000).

## Shell & outputs

- [00:12] `~ $ # In this video , i will make you love lisp` (shell comment, no output)
- [00:12] `~ $ chez` -> `Chez Scheme Version 10.4.1` / `Copyright 1984-2026 Cisco Systems, Inc.` then `> `
- [~03:23] Chez exited (shell prompt `~ $ ` visible below `> `); the exit command itself (probably Ctrl-D) is not visible.
- [03:25] `~ $ chez` again, same banner.
- Terminal shows `[` and `]` markers at line edges around entered commands (shell-integration marks).

## Observations

- Stated goal, typed as a shell comment before launching chez: "In this video , i will make you love lisp". Title framing: conjuring the computer's spirits with spells.
- Notes file is a plain, pre-written TOC. He walks it top to bottom, selecting the current topic with vim VISUAL mode. Highlighting is slightly loose: "real numbers are always precise." is selected while he does `+ - / *`.
- Teaching order in this segment:
  1. Basic operators in prefix notation: `(+ 1 2)`, `(+ 1 2 3)` (n-ary), `(- 1 2)`, `(- 2 3)`, `(- 2 2)`.
  2. "Real numbers always precise": `(/ 2 3)` -> `2/3` (exact rationals), `(/ 4 2)` -> 2; shows exactness by turning fractions back into integers: `(* (/ 2 3) (/ 3 2))` -> 1, `(+ (/ 2 3) (/ 1 3))` -> 1.
  3. "Large number never overflow": `(* 10 10)`, then 10 times a 1-followed-by-many-zeros literal (typed twice, bigger second time); then `(expt 2 n)` for n = 1, 2, 3, 10, 100, 1000, 10000 (growing by powers of ten), scrolling the terminal to show how long the digits get.
  4. `define`: variables (`(define x 10)`, `(define y 20)`, `x`, `y`, `(+ x y)`, `(- x y)`), then functions via `(define (add x y) (+ x y))`, `(add 2 3)`, `(add 3 4)`, `(define (sub x y) (- x y))`.
  5. A playful experiment: `(define (+ x y) (- x y))` then `(+ 3 4)` -> -1, showing operators are ordinary names that can be redefined. He then quits Chez and starts `chez` again to reset.
  6. Recursion: `factorial` with `if` and `=`. First attempt with infix `(n * ...)` gives `Exception: attempt to apply non-procedure 2` (first position in a list is treated as the operator); he retypes with prefix `*`. `(factorial 3)` -> 6. A typo `(fatorial 4)` gives `variable fatorial is not bound`; retyped. `(factorial 4)` -> 24, `(factorial 10)` -> 3628800, `(factorial 100)` -> big exact integer, tying back to "large number never overflow".
- Style: minimal examples, geometric growth of inputs (1, 2, 3, 10, 100, 1000, 10000), clears the screen between topics, lets errors and typos be shown and fixed live without comment on screen, uses the mouse to point at lines in the REPL (00:16, 03:19). No on-screen prose beyond the notes and the shell comment. Webcam ignored.
