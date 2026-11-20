"""Re-derive leading indentation of a C file from brace structure, in Hayyoth's style
(2 spaces per level, K&R braces, brace-less single-statement bodies indented one extra level).
Transcription agents often misjudge leading spaces from screenshots; the code text is right but
the indent is off by one. This recomputes indentation only — line content is untouched.

    python tools/reindent.py <in.c> [out.c]     (prints a diff summary; writes out.c if given)
    python tools/reindent.py --selftest <file.c>...   (exact-match check on files known to be correct)

Limitations: multi-line expressions / initializers keep their transcribed continuation indent
relative to the statement start; comments and preprocessor lines follow the current depth.
"""
import re, sys

CTRL = re.compile(r"^(if|for|while|else if)\b.*\)\s*(//.*)?$|^else\s*(//.*)?$")


def reindent(text, unit=2):
    out, depth, pending, cont_base = [], 0, 0, None
    lines = text.split("\n")
    for raw in lines:
        s = raw.strip()
        if not s:
            out.append("")                      # blank lines carry no indent (screen can't show it)
            continue
        if s.startswith("#"):
            out.append(s); continue
        lead = depth - (1 if s.startswith("}") else 0)
        level = lead + pending
        # continuation lines (inside an unclosed '(' or after '=' with '{' initializer handled by depth)
        if cont_base is not None or s.startswith("//"):
            out.append(raw.rstrip("\n"))        # keep transcribed indent for wrapped expressions and comment lines
        else:
            out.append(" " * (unit * max(level, 0)) + s)
        code = re.sub(r'"(\\.|[^"\\])*"|\'(\\.|[^\'\\])*\'|//.*$', "", s)
        opens, closes = code.count("{"), code.count("}")
        parens = code.count("(") - code.count(")")
        if cont_base is None and parens > 0:
            cont_base = parens
        elif cont_base is not None:
            cont_base += parens
            if cont_base <= 0:
                cont_base = None
        depth += opens - closes
        if CTRL.match(code.strip()) and not code.rstrip().endswith("{") and not code.rstrip().endswith(";"):
            pending = 1 if pending == 0 else pending + 1
        elif not code.rstrip().endswith("{"):
            pending = 0
        else:
            pending = 0
    return "\n".join(out)


if __name__ == "__main__":
    if sys.argv[1] == "--selftest":
        ok = True
        for p in sys.argv[2:]:
            t = open(p, newline="", encoding="utf-8").read()
            r = reindent(t)
            same = r == t
            ok &= same
            print(("OK   " if same else "DIFF ") + p)
            if not same:
                for a, b in zip(t.split("\n"), r.split("\n")):
                    if a != b: print("   orig:", repr(a)); print("   new :", repr(b)); break
        sys.exit(0 if ok else 1)
    t = open(sys.argv[1], newline="", encoding="utf-8").read()
    r = reindent(t)
    changed = sum(1 for a, b in zip(t.split("\n"), r.split("\n")) if a != b)
    print(f"{sys.argv[1]}: {changed} lines re-indented, {len(t.encode())}B -> {len(r.encode())}B")
    if len(sys.argv) > 2:
        open(sys.argv[2], "w", newline="", encoding="utf-8").write(r)
