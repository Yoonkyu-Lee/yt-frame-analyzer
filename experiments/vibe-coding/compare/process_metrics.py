"""Summarize how Codex worked on each task from its --json event log.
    python compare/process_metrics.py   (run from vibe-coding/)
Counts: edit rounds (file_change events), files touched, shell commands, build commands,
failed commands, test/sanitizer runs, tokens, wall time."""
import glob, json, os, re

BUILD = re.compile(r"\b(cc|gcc|clang|make)\b")
TEST = re.compile(r"test|sanitize|valgrind|\./")
rows = []
for d in sorted(glob.glob("0*_*")):
    ev = [json.loads(l) for l in open(f"{d}/codex_events.jsonl", encoding="utf-8")]
    items = [e["item"] for e in ev if e.get("type") == "item.completed" and "item" in e]
    edits = [i for i in items if i.get("type") == "file_change"]
    cmds = [i for i in items if i.get("type") == "command_execution"]
    files = {os.path.basename(c["path"]) for i in edits for c in i.get("changes", [])}
    builds = [c for c in cmds if BUILD.search(c.get("command", ""))]
    fails = [c for c in cmds if c.get("exit_code") not in (0, None)]
    tests = [c for c in cmds if TEST.search(c.get("command", "")) and not BUILD.search(c.get("command", ""))]
    flags = sorted({f for c in builds for f in re.findall(r"-W[\w=-]+|-fsanitize=[\w,]+|-std=\w+", c["command"])})
    usage = next((e["usage"] for e in ev if e.get("type") == "turn.completed"), {})
    secs = re.search(r"seconds: (\d+)", open(f"{d}/meta.txt").read()).group(1)
    rows.append((d, len(edits), len(files), len(cmds), len(builds), len(fails), len(tests),
                 usage.get("input_tokens", 0), usage.get("output_tokens", 0), secs, " ".join(flags)))

print("| task | edit rounds | files | commands | builds | failed cmds | runs/tests | in tok | out tok | sec | compiler flags used |")
print("|---|---|---|---|---|---|---|---|---|---|---|")
for r in rows:
    print("| " + " | ".join(str(x) for x in r) + " |")
