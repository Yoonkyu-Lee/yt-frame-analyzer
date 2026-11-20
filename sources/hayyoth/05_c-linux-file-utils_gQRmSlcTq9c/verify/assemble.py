"""Assemble rm.c and find.c from transcripts + frame-verified fixes, then report sizes vs on-screen checksums.
    python verify/assemble.py   (run from the video folder)"""
import re


def numbered(block):
    return {int(m.group(1)): m.group(2) for m in re.finditer(r"^\s*(\d+)\|(?: ?)(.*)$", block, re.M)}


def block_after(text, head):
    i = text.index(head)
    return re.search(r"```\n(.*?)```", text[i:], re.S).group(1)


def write(path, lines):
    s = "\n".join(lines) + "\n"
    open(path, "w", newline="", encoding="utf-8").write(s)
    return s


# ---- rm.c: 41:56 body (lines 3-67) + header seen at 43:30 + main seen at 43:29 ----
t12 = open("transcripts/seg12.md", encoding="utf-8").read()
body = numbered(block_after(t12, "### rm.c, version at 41:56"))
main = numbered(block_after(t12, "### rm.c final"))
rm81 = ["#include <stdio.h>", "#include <sys/stat.h>"] + [body[i] for i in range(3, 68)] + [main[i] for i in range(69, 83)]
s = write("history/rm_v1_4156.c", rm81)
print(f"rm   v1 (41:56): {s.count(chr(10))}L {len(s.encode())}B   screen 81L 1562B")
# 43:28: debug printf inserted as line 10 (inside the lstat error block, 4-space indent)
rm82 = rm81[:9] + ['    printf("on this case\\n");'] + rm81[9:]
s = write("src/rm.c", rm82)
print(f"rm   v2 (43:28): {s.count(chr(10))}L {len(s.encode())}B   screen 82L 1592B")

# ---- find.c: 51:35 state; lines 16 and 44 verified on 51:39 / 51:31 frames; main verified on 47:55 frame ----
t14 = open("transcripts/seg14.md", encoding="utf-8").read()
f = numbered(block_after(t14, "### find.c (open"))
lines = [f[i] for i in range(1, 56)]
lines[15] = '      printf("%s\\n", path);'
lines[43] = '        printf("%s\\n", new_path);'
lines += [
    "",
    "int main(int argc, char *argv[]) {",
    "  if (argc < 3) {",
    '    printf("Useage: find [patter] [path]...\\n");',
    "    return 0;",
    "  }",
    "",
    "  const char *pattern = argv[1];",
    "  for (int i = 2; i < argc; i++) {",
    "    const char *path = argv[i];",
    "    if (walkfind(path, pattern))",
    "      return 1;",
    "  }",
    "",
    "  return 0;",
    "}",
]
s = write("src/find.c", lines)
assert "⟨" not in s and "??" not in s
print(f"find final      : {s.count(chr(10))}L {len(s.encode())}B   (no save message on screen after edits)")
