"""Measure frame-transcription accuracy against ground truth.
The files shown in this video are byte-identical to the public repo codeberg.org/hayyoth/hayyoth-lab
(sizes match vim's open messages once git's CRLF checkout conversion is undone).
For each file: align repo lines with the transcribed lines (difflib), count exact matches,
matches ignoring indentation, and lines missing/garbled. Comments the author added on camera
show up as transcript-only lines and are listed separately.

    python verify/accuracy.py <repo-dir>      (run from the video folder)
"""
import difflib, re, sys

repo = sys.argv[1]
FILES = [("lab/conway.c", "### lab/conway.c"), ("src/hrender.c", "### src/hrender.c"),
         ("src/hcell.c", "### src/hcell.c"), ("src/hcell.h", "### src/hcell.h")]
t = open("transcripts/seg4.md", encoding="utf-8").read().split("## File states")[1]

tot = {"n": 0, "exact": 0, "ws": 0}
for path, head in FILES:
    truth = open(f"{repo}/{path}", encoding="utf-8", newline="").read().replace("\r\n", "\n").split("\n")
    if truth and truth[-1] == "":
        truth = truth[:-1]
    block = re.search(r"```[a-z]*\n(.*?)```", t[t.index(head):], re.S).group(1)
    seen = [m.group(1) for m in re.finditer(r"^\s*\d+\|(?: ?)(.*)$", block, re.M)]
    sm = difflib.SequenceMatcher(a=[l.rstrip() for l in truth], b=[l.rstrip() for l in seen], autojunk=False)
    exact = sum(b.size for b in sm.get_matching_blocks())
    sm2 = difflib.SequenceMatcher(a=[l.strip() for l in truth], b=[l.strip() for l in seen], autojunk=False)
    ws = sum(b.size for b in sm2.get_matching_blocks())
    added = [seen[j] for tag, i1, i2, j1, j2 in sm2.get_opcodes() if tag == "insert" for j in range(j1, j2)]
    wrong = [(truth[i], seen[j]) for tag, i1, i2, j1, j2 in sm2.get_opcodes() if tag == "replace"
             for i, j in zip(range(i1, i2), range(j1, j2))]
    unseen = sum(1 for l in seen if "??" in l)
    print(f"{path:14} truth {len(truth):3}L | exact {exact:3} ({exact/len(truth):5.1%}) | "
          f"ignoring indent {ws:3} ({ws/len(truth):5.1%}) | transcript-only {len(added)} | '??' {unseen}")
    for a, b in wrong[:3]:
        print(f"     truth: {a.strip()[:70]!r}\n     seen : {b.strip()[:70]!r}")
    tot["n"] += len(truth); tot["exact"] += exact; tot["ws"] += ws
print(f"TOTAL          {tot['n']} lines | exact {tot['exact']/tot['n']:.1%} | ignoring indent {tot['ws']/tot['n']:.1%}")
