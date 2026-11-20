#!/usr/bin/env python3
"""Mutation check for a fix: temporarily revert it, run the tests, always restore the file.

    python mutate.py --file src/x.rs --find "<fixed text>" --replace "<old text>" -- cargo test --lib

`--find` must occur exactly once in the file. Exit 0 when the tests FAIL on the mutant (the fix is
guarded), 1 when they pass (the fix is not pinned by any test), 2 on usage errors.
Only the standard library is used.
"""
import argparse, subprocess, sys
from pathlib import Path


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--file", required=True)
    ap.add_argument("--find", required=True)
    ap.add_argument("--replace", required=True)
    ap.add_argument("cmd", nargs=argparse.REMAINDER, help="test command after --")
    a = ap.parse_args()
    cmd = a.cmd[1:] if a.cmd[:1] == ["--"] else a.cmd
    if not cmd:
        print("missing test command after --", file=sys.stderr)
        return 2
    path = Path(a.file)
    original = path.read_bytes()
    text = original.decode("utf-8")
    if text.count(a.find) != 1:
        print(f"--find occurs {text.count(a.find)} times in {path}; it must occur exactly once", file=sys.stderr)
        return 2
    try:
        path.write_bytes(text.replace(a.find, a.replace, 1).encode("utf-8"))
        result = subprocess.run(cmd)
    finally:
        path.write_bytes(original)  # always restore, even on Ctrl+C
    if result.returncode != 0:
        print("MUTANT KILLED: tests fail without the fix (the fix is guarded)")
        return 0
    print("MUTANT SURVIVED: tests pass without the fix (no test pins it)")
    return 1


if __name__ == "__main__":
    sys.exit(main())
