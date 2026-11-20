"""Static size/shape metrics for human (sources/hayyoth/*/src) vs Codex (vibe-coding/*/output) C code.
    python compare/code_metrics.py   (run from vibe-coding/)"""
import glob, re

PAIRS = {
    "02_tree": "02_c-tree-viewer_tiMHtc6l7xM",
    "03_random-moving-numbers": "03_c-random-moving-numbers_IJe5WtpXx5c",
    "05_linux-file-utils": "05_c-linux-file-utils_gQRmSlcTq9c",
    "07_linked-list": "07_c-linked-list_iz2j9_AwQdQ",
    "08_dynamic-array": "08_c-dynamic-array_SOjHybBunAc",
    "09_hash-table": "09_c-hash-table_61XUoywg970",
}
FUNC = re.compile(r"^[A-Za-z_][\w \*]*?\b(\w+)\s*\([^;]*\)\s*\{?\s*$", re.M)


def metrics(files):
    m = dict(files=len(files), code=0, comment=0, funcs=0, static=0, malloc=0, free=0,
             checks=0, consts=0, asserts=0, tests=0)
    for f in files:
        t = open(f, encoding="utf-8", errors="replace").read()
        for line in t.split("\n"):
            s = line.strip()
            if not s:
                continue
            if s.startswith(("//", "/*", "*")):
                m["comment"] += 1
            else:
                m["code"] += 1
                if "//" in s:
                    m["comment"] += 1
        body = re.sub(r"/\*.*?\*/", "", t, flags=re.S)
        m["funcs"] += len([x for x in FUNC.findall(body) if x not in ("if", "for", "while", "switch", "return", "sizeof")])
        m["static"] += len(re.findall(r"^static\b", body, re.M))
        m["malloc"] += len(re.findall(r"\b(malloc|calloc|realloc|strdup)\s*\(", body))
        m["free"] += len(re.findall(r"\bfree\s*\(", body))
        m["checks"] += len(re.findall(r"if\s*\(\s*!|==\s*NULL|!=\s*0\)|<\s*0\)|perror|errno", body))
        m["consts"] += len(re.findall(r"\bconst\b", body))
        m["asserts"] += len(re.findall(r"\bassert\s*\(", body))
        m["tests"] += 1 if "test" in f.lower() else 0
    return m


cols = ["files", "code", "comment", "funcs", "static", "malloc", "free", "checks", "consts", "asserts", "tests"]
print("| task | who | " + " | ".join(cols) + " |")
print("|---|---|" + "---|" * len(cols))
for v, h in PAIRS.items():
    hf = sorted(glob.glob(f"../../sources/hayyoth/{h}/src/*.c")) + sorted(glob.glob(f"../../sources/hayyoth/{h}/src/*.h"))
    vf = sorted(glob.glob(f"{v}/output/*.c")) + sorted(glob.glob(f"{v}/output/*.h"))
    for who, fs in (("human", hf), ("codex", vf)):
        m = metrics(fs)
        print(f"| {v} | {who} | " + " | ".join(str(m[c]) for c in cols) + " |")
