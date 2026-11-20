# Blind static reader (optional transparency probe)

Prepare: copy a candidate to an isolated folder as `module.<ext>` under a neutral label (keep the
label → candidate map private), plus `QUESTIONS.md`: questions whose answers are **observed by
running the probe** (counts, files left, time to error, chosen mode, behavior under misbehaving
peers), each with an exact answer format.

```
You are a code reader in a measurement of how predictable code is from static reading alone.
Read exactly two files and nothing else: <dir>/QUESTIONS.md and <dir>/module.<ext>.
Use only the Read tool on those two files. Do not compile, run, search the disk, or open any other
file (other versions exist elsewhere; looking at them invalidates the measurement). Infer library
behavior from your own knowledge. Answer every question in the requested format: first a JSON
object, then one line per answer citing the code lines you relied on. Mark uncertain answers with
(guess). Report reading effort 1-5 and the hardest question and why.
```

Grade against the observed facts of **that** candidate. A wrong answer may reveal an oracle blind
spot rather than a reader error — check before scoring (it happened in the pilot).
