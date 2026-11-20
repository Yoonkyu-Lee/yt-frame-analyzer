#!/usr/bin/env bash
# Run one vibe-coding task with Codex in an isolated empty directory (no access hints to sources/hayyoth/).
#   bash run_codex.sh <task-dir> <isolated-workdir>
# Produces in <task-dir>: output/ (files Codex wrote, binaries excluded), codex_last_message.md,
# codex_events.jsonl (full event log: commands, compiles, edits), meta.txt
set -u
task="$1"; work="$2"
here="$(cd "$(dirname "$0")" && pwd)"
tdir="$here/$task"
mkdir -p "$work"
prompt="$(cat "$tdir/PROMPT.md")"
start=$(date +%s)
codex exec --skip-git-repo-check --ephemeral \
  --dangerously-bypass-approvals-and-sandbox \
  -C "$work" --json -o "$tdir/codex_last_message.md" \
  "$prompt" > "$tdir/codex_events.jsonl" 2> "$tdir/codex_stderr.txt"
rc=$?
end=$(date +%s)
mkdir -p "$tdir/output"
# copy sources, scripts and docs only (compiled binaries and .o files stay in the workdir)
for f in "$work"/*.c "$work"/*.h "$work"/*.py "$work"/*.sh "$work"/Makefile "$work"/*.md "$work"/.gitignore; do
  [ -f "$f" ] && cp "$f" "$tdir/output/"
done
{
  echo "codex: $(codex --version 2>/dev/null)"
  echo "model: $(grep -E '^model' ~/.codex/config.toml | tr '\n' ' ')"
  echo "exit: $rc"
  echo "seconds: $((end - start))"
  echo "workdir: $work"
} > "$tdir/meta.txt"
echo "$task done rc=$rc in $((end - start))s"
