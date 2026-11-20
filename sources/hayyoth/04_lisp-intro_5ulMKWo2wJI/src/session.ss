;;; REPL session reconstructed from "Intro Lisp, we conjure the spirits of the computer with our spells"
;;; (Hayyoth, https://youtu.be/5ulMKWo2wJI). Chez Scheme 10.4.1, typed live at the `>` prompt.
;;; Expressions are in the order typed; `;=>` is the result shown on screen.
;;; Deliberate/accidental errors from the video are kept as comments so the file runs top to bottom:
;;;   chez --script session.ss

;; ~ $ # In this video , i will make you love lisp

;;; - some basic operator (prefix notation, n-ary)
(+ 1 2)                          ;=> 3
(+ 1 2 3)                        ;=> 6
(- 1 2)                          ;=> -1
(- 2 3)                          ;=> -1
(- 2 2)                          ;=> 0

;;; - real numbers are always precise. (exact rationals)
(/ 2 3)                          ;=> 2/3
(/ 1 1)                          ;=> 1
(/ 4 2)                          ;=> 2
(* (/ 2 3) (/ 3 2))              ;=> 1
(+ (/ 2 3) (/ 1 3))              ;=> 1

;;; - large number never overflow (bignums)
(* 10 10)                        ;=> 100
;; (* 10 1000…000) twice with ~72 and ~100+ zeros (exact zero counts not readable)
(expt 2 1)                       ;=> 2
(expt 2 2)                       ;=> 4
(expt 2 3)                       ;=> 8
(expt 2 10)                      ;=> 1024
(expt 2 100)                     ;=> 1267650600228229401496703205376
(expt 2 1000)                    ;=> 302 digits, …5668069376
(expt 2 10000)                   ;=> 3011 digits, …96709376
(expt 2 100000)                  ;=> …883109376  (expression off-screen; identified from the last digits)

;;; define - variable / - function
(define x 10)
(define y 20)
x                                ;=> 10
y                                ;=> 20
(+ x y)                          ;=> 30
(- x y)                          ;=> -10
(define (add x y)
  (+ x y))
(add 2 3)                        ;=> 5
(add 3 4)                        ;=> 7
(define (sub x y)
  (- x y))

;; Operators are just names — he redefines + as subtraction, then restarts chez:
;; (define (+ x y)
;;   (- x y))
;; (+ 3 4)                       ;=> -1

;;; - recursion made easy - factorial
;; first attempt was infix:  (n * (factorial (- n 1)))
;;   (factorial 3) => Exception: attempt to apply non-procedure 2
(define (factorial n)
  (if (= n 1)
      1
      (* n (factorial (- n 1)))))
(factorial 3)                    ;=> 6
;; (fatorial 4)                  ;=> Exception: variable fatorial is not bound   (typo)
(factorial 4)                    ;=> 24
(factorial 10)                   ;=> 3628800
(factorial 100)                  ;=> 158 digits, 933262154439441…000
(factorial 1000)                 ;=> 2568 digits

;;; - loop (recursion instead of a loop construct)
(define (loop i n)
  (display i)
  (newline)
  (if (< i n)
      (loop (+ i 1) n)))
(loop 2 10)                      ; prints 2..10
(loop 10 20)                     ; prints 10..20

;;; list data struct - car, cdr
'(1 2 3)                         ;=> (1 2 3)
(define my-list '(10 20 30 40))
(car my-list)                    ;=> 10
(cdr my-list)                    ;=> (20 30 40)

;;; - loop through list
(define (print-list ls)
  (display (car ls))
  (newline)
  (if (not (null? (cdr ls)))
      (print-list (cdr ls))))
(print-list my-list)             ; prints 10 20 30 40
(print-list '(1 2 3))            ; prints 1 2 3

;;; - apply / - map
(apply + '(1 2 3 4))             ;=> 10
(apply + '(1 2 3))               ;=> 6
(map (lambda (x) (* x 10)) '(1 2 3 4))  ;=> (10 20 30 40)

;;; code is data - eval
(define my-code '(+ 1 2 3))
(eval my-code)                   ;=> 6
(define my-code '(apply + '(1 2 3 4)))
my-code                          ;=> (apply + '(1 2 3 4))
(eval my-code)                   ;=> 10

;;; another feature - system call
(system "ls")                    ;=> prints home dir listing, returns 0
(system "mkdir abc")             ;=> 0
(system "ls")                    ;=> listing now includes abc, returns 0

;; > book to read                (typed at the prompt, not evaluated — then SICP and TSPL4 in the browser)
