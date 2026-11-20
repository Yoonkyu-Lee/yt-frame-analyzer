#!/usr/bin/env python3
"""Intent-density gate: check a git change against the mechanical laws of NORTHSTAR.md.

    python gate.py --base main                 # working tree vs main, note from --note or commits
    python gate.py --base HEAD~1 --head HEAD   # one commit
    python gate.py --base main --note change.txt --budget 30

Checks (FAIL blocks, WARN informs):
  trace       every changed/new source file is listed in TRACE.md            (L1)  FAIL
  intent      change note has `Intent:` naming IDs that exist in INTENT.md   (L1)  FAIL
  budget      net added source lines <= budget, else note needs `Budget:`    (L2)  FAIL
  deps        dependency manifest changed -> LEDGER.md must change too       (L4)  FAIL
  fix         bug-fix note needs `Root cause:` and `Deletion considered:`    (L6)  WARN
  mutation    bug-fix note needs `Mutation:` (test that fails when the fix is reverted)  WARN
  ratchet     bug-fix note needs `Ratchet:` (the check that keeps the class out)   (L13) WARN
  alternatives  new source file or over-budget growth needs `Alternatives:` (L14) WARN
  excuse      added comments that justify a workaround or defer a fix       (L13) WARN
Only the standard library and git are used, so it runs in any project.
"""
import argparse, re, subprocess, sys
from pathlib import Path

SRC = {".c", ".h", ".cc", ".cpp", ".hpp", ".py", ".js", ".ts", ".tsx", ".jsx", ".go", ".rs",
       ".java", ".kt", ".swift", ".rb", ".php", ".cs", ".scala", ".lua", ".sh", ".zig"}
MANIFESTS = {"package.json", "go.mod", "requirements.txt", "pyproject.toml", "Pipfile", "Cargo.toml",
             "Gemfile", "pom.xml", "build.gradle", "build.gradle.kts", "CMakeLists.txt", "vcpkg.json",
             "conanfile.txt", "composer.json", "Package.swift"}
TEST = re.compile(r"(^|/)(tests?|spec|__tests__)/|(^|/)tests?(_[^/]*)?\.[a-z]+$|_test\.[a-z]+$|\.test\.[a-z]+$")
# A comment line (not a C preprocessor line or shebang) whose words excuse or defer a fix.
COMMENT = re.compile(r"^\s*(//|/\*|\*|--|;|#(?!\s*(include|define|if|ifdef|ifndef|endif|else|elif|pragma|undef|error)\b|!))")
# Tuned on the yt-frame-analyzer corpus (111 C/Rust files, 16,730 lines, human and AI written): bare
# "later" and lowercase "todo" only hit feature descriptions and variable names, so they are left out
# and markers count only in upper case. After tuning: 0 hits there.
EXCUSE = re.compile(r"(?i:\b(workaround|hacky?|for now|in case|temporar\w*|quick[- ]?fix|band-?aid|shortcut:))"
                    r"|\b(TODO|FIXME|XXX|HACK)\b")


def git(*args):
    return subprocess.run(["git", *args], check=True, capture_output=True, text=True).stdout


def is_source(path):
    return Path(path).suffix in SRC and not TEST.search(path)


def table_first_column(md):
    """Return first-column cells of markdown tables (skips header and separator rows)."""
    cells = []
    for line in md.splitlines():
        if line.startswith("|") and not re.match(r"^\|\s*-", line):
            cells.append(line.split("|")[1].strip().strip("`"))
    return cells[1:] if cells else []


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--base", required=True)
    ap.add_argument("--head", help="commit to compare; default = working tree")
    ap.add_argument("--note", help="file with the change note; default = commit messages base..head")
    ap.add_argument("--budget", type=int, default=30, help="net added source lines allowed without `Budget:`")
    ap.add_argument("--root", default=".", help="project root holding INTENT.md, TRACE.md, LEDGER.md")
    a = ap.parse_args()
    root = Path(a.root)
    rng = [a.base] + ([a.head] if a.head else [])

    numstat = [l.split("\t") for l in git("diff", "--numstat", *rng).splitlines() if l]
    status = [l.split("\t") for l in git("diff", "--name-status", *rng).splitlines() if l]
    if not a.head:  # include untracked files in a working-tree check
        for f in git("ls-files", "--others", "--exclude-standard").splitlines():
            n = sum(1 for _ in open(root / f, errors="ignore")) if (root / f).is_file() else 0
            numstat.append([str(n), "0", f]); status.append(["A", f])
    changed = {s[-1] for s in status}
    added_files = {s[-1] for s in status if s[0].startswith("A")}

    note = Path(a.note).read_text(encoding="utf-8") if a.note else (
        git("log", "--format=%B", f"{a.base}..{a.head}") if a.head else "")
    trace_md = (root / "TRACE.md").read_text(encoding="utf-8") if (root / "TRACE.md").exists() else ""
    intent_md = (root / "INTENT.md").read_text(encoding="utf-8") if (root / "INTENT.md").exists() else ""
    traced = set(table_first_column(trace_md))
    intent_ids = set(re.findall(r"^\|\s*(I\d+)\s*\|", intent_md, re.M))

    results = []
    def report(level, check, msg): results.append((level, check, msg))

    # trace (L1)
    src_changed = sorted(f for f in changed if is_source(f) and (root / f).exists())
    missing = [f for f in src_changed if f not in traced]
    report("FAIL" if missing else "PASS", "trace",
           f"not in TRACE.md: {', '.join(missing)}" if missing else f"{len(src_changed)} changed source files traced")

    # intent (L1)
    named = set(re.findall(r"\bI\d+\b", " ".join(re.findall(r"^Intent:(.*)$", note, re.M))))
    if not named:
        report("FAIL", "intent", "change note has no `Intent: I<n>` line")
    elif intent_ids and named - intent_ids:
        report("FAIL", "intent", f"unknown intent IDs: {', '.join(sorted(named - intent_ids))}")
    else:
        report("PASS", "intent", f"serves {', '.join(sorted(named))}")

    # budget (L2)
    add = sum(int(x[0]) for x in numstat if x[0] != "-" and is_source(x[2]))
    dele = sum(int(x[1]) for x in numstat if x[1] != "-" and is_source(x[2]))
    net = add - dele
    has_budget = re.search(r"^Budget:\s*\S", note, re.M)
    if net > a.budget and not has_budget:
        report("FAIL", "budget", f"net +{net} source lines > budget {a.budget}; add `Budget:` saying what they buy")
    else:
        report("PASS", "budget", f"+{add} -{dele} (net {net:+d}), budget {a.budget}")

    # deps (L4)
    manif = sorted(f for f in changed if Path(f).name in MANIFESTS)
    mk = [f for f in changed if Path(f).name in ("Makefile", "makefile", "GNUmakefile")
          and re.search(r"^\+.*(LDLIBS|LDFLAGS|-l\w)", git("diff", *rng, "--", f), re.M)]
    dep_changes = manif + mk
    if dep_changes and "LEDGER.md" not in changed:
        report("FAIL", "deps", f"dependency change in {', '.join(dep_changes)} without a LEDGER.md entry")
    else:
        report("PASS", "deps", "no dependency change" if not dep_changes else "recorded in LEDGER.md")

    # fix (L6)
    if re.search(r"\b(fix|bug|crash|leak|regression)\b", note, re.I):
        need = [k for k in ("Root cause:", "Deletion considered:") if k not in note]
        report("WARN" if need else "PASS", "fix",
               f"bug fix note is missing {', '.join(need)}" if need else "root cause and deletion recorded")
        report("WARN" if "Mutation:" not in note else "PASS", "mutation",
               "bug fix note has no `Mutation:` (a test that fails when the fix is reverted)"
               if "Mutation:" not in note else "mutation check recorded")
        # ratchet (L13): the lesson goes into a machine check, not into prose
        report("WARN" if "Ratchet:" not in note else "PASS", "ratchet",
               "bug fix note has no `Ratchet:` (the lint/type/test/import ban that keeps this class of bug out, "
               "or `none: <why>`)" if "Ratchet:" not in note else "ratchet recorded")

    # alternatives (L14): new code that is large or new needs a measured comparison with a baseline way
    src_added = sorted(f for f in added_files if is_source(f))
    if (src_added or net > a.budget) and "Alternatives:" not in note:
        why = f"new source files {', '.join(src_added)}" if src_added else f"net +{net} lines"
        report("WARN", "alternatives", f"{why}; add `Alternatives:` (stdlib/platform/existing code/known method, measured)")
    elif src_added or net > a.budget:
        report("PASS", "alternatives", "alternatives recorded")

    # excuse (L13): comments that justify a workaround become the pattern the next agent copies
    excuses = {}
    for f in src_changed:
        if a.head or f not in added_files or git("ls-files", "--", f).strip():
            lines = [l[1:] for l in git("diff", "-U0", *rng, "--", f).splitlines()
                     if l.startswith("+") and not l.startswith("+++")]
        else:
            lines = (root / f).read_text(encoding="utf-8", errors="ignore").splitlines()
        hits = [l.strip() for l in lines if COMMENT.match(l) and EXCUSE.search(l)]
        if hits:
            excuses[f] = hits
    if excuses:
        first = next(iter(excuses.items()))
        report("WARN", "excuse", f"{sum(map(len, excuses.values()))} comment(s) excuse or defer a fix "
               f"(e.g. {first[0]}: {first[1][0][:60]}); fix the cause, or record it under LEDGER Doubts")
    else:
        report("PASS", "excuse", "no workaround-justifying comments added")

    for level, check, msg in results:
        print(f"{level:4}  {check:7} {msg}")
    ratio = f"{dele / add:.2f}" if add else "n/a"
    print(f"      metrics new files {len(added_files)}, deletion ratio {ratio}")
    return 1 if any(r[0] == "FAIL" for r in results) else 0


if __name__ == "__main__":
    sys.exit(main())
