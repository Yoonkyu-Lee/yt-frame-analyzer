"""Extract a file state block from a transcript and compare to the on-screen vim checksum.
usage: python verify/extract.py <seg.md> "<heading substring>" <out> [lines bytes]"""
import re, sys
seg, head, out = sys.argv[1:4]
t = open(seg, encoding="utf-8").read().split("## File states")[1]
i = t.index(head)
block = re.search(r"```[a-z]*\n(.*?)```", t[i:], re.S).group(1)
rows = [l for l in block.rstrip("\n").split("\n")]
code = [re.sub(r"^\s*\d+\|( ?)", "", l, count=1) for l in rows if re.match(r"^\s*\d+\|", l)]
bad = [l for l in rows if not re.match(r"^\s*\d+\|", l)]
s = "\n".join(code) + "\n"
open(out, "w", newline="", encoding="utf-8").write(s)
msg = f"{out}: {s.count(chr(10))}L {len(s.encode())}B"
if len(sys.argv) > 5:
    L, B = int(sys.argv[4]), int(sys.argv[5])
    msg += f"  (screen {L}L {B}B, delta {s.count(chr(10))-L:+d}L {len(s.encode())-B:+d}B)"
print(msg)
if bad: print("   non-numbered lines dropped:", [b[:60] for b in bad][:4])
if "⟨?⟩" in s or "??" in s: print("   !! uncertain markers present")
