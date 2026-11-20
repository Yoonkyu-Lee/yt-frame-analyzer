# Change note

Put this in the commit message or PR description (gate.py reads the commit message, or a file
passed with --note).

```
Intent: I3
Root cause: rm fell through to opendir() after unlinking a plain file
Deletion considered: yes — merged file/dir handling into one recursive call (−7 lines)
Budget: +0
Alternatives: (new file or over budget only) <baseline, e.g. nftw(3)>: <its size, bench result> vs ours: <size, result>; <why ours>
Evidence: tests/intent/i3_rm_file.sh, ASan clean
Mutation: reverting the merge makes tests/intent/i3_rm_file.sh fail
Ratchet: <the check that keeps the class out, e.g. the same file/dir fixture now runs for every tool, not only rm>
```

Rules
- `Intent:` is required. It names IDs from INTENT.md.
- `Root cause:`, `Deletion considered:` and `Mutation:` are required for bug fixes. `Mutation:` names
  the test that failed before the fix and fails again when the fixed lines are reverted.
- `Ratchet:` is expected for bug fixes and for any correction you had to give an agent twice: the
  cheapest machine check that keeps the whole class out (lint rule, type, test, import ban, API
  removal). If none exists, write `Ratchet: none — <why>` and add the class to LEDGER Doubts.
- `Budget:` is required when net added lines exceed the gate budget. Say what the new lines buy:
  an intent, a measurement (link it), or a trust boundary.
- `Alternatives:` is expected for a new source file or over-budget growth: the baseline ways
  (existing code, standard library, platform feature, well-known method or tool) with a measured
  size/speed comparison, and why this one won.
- `Evidence:` names what you ran and what it showed (test output, benchmark, screenshot of the real
  program path). "It compiles" is not evidence.
