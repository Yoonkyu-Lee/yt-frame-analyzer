"""Append edge-case probes to src/main.c (into a temp copy) and print the generated C for compiling.
    python verify/edge_test.py > /tmp/t.c   (run from the video folder)
Probes: (1) lookup of a missing key that hashes to an EMPTY bucket, (2) node count in the bucket of
"abc" after a single set() — the author's set() falls through after filling an empty slot."""
src = open("src/main.c", encoding="utf-8").read()
i = src.rindex("  return 0;")
probe = (
    '  int n = 0;\n'
    '  for (Node *q = &arr[hash("abc")]; q; q = q->next) n++;\n'
    '  printf("nodes in bucket of abc: %d\\n", n);\n'
    '  fflush(stdout);\n'
    '  printf("missing key in empty bucket -> %d\\n", get(arr, "d"));\n'
)
print(src[:i] + probe + src[i:], end="")
