# Control prompt (given to Codex with r0 only; no INTENT.md, no tests)

This directory contains C implementations of the file utilities touch, cat, mkdir, mv, rm and find (common.c/common.h are shared helpers). Make the codebase as minimal as possible while keeping it working. Edit the files in place. You cannot compile in this environment.

Run: `codex exec --skip-git-repo-check --ephemeral -s workspace-write --json` (gpt-6.1-sol, reasoning high).
