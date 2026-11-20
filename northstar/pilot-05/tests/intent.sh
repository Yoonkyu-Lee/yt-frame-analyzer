#!/usr/bin/env bash
# Intent tests for the file utilities (INTENT.md). Tests behavior, not implementation.
#   bash tests/intent.sh <dir-with-binaries>
# Prints FAIL lines and a summary; exit 1 if anything failed.
# Note: `rm /` is deliberately NOT exercised (a broken build could destroy the system).
set -u
B="$(cd "$1" && pwd)"
T="$(mktemp -d)"; cd "$T" || exit 1
pass=0; fail=0
ok()  { pass=$((pass + 1)); }
bad() { fail=$((fail + 1)); echo "FAIL $1${2:+: $2}"; }
is()  { [ "$2" = "$3" ] && ok || bad "$1" "expected [$2] got [$3]"; }
errfmt() { grep -qE "^$2: .* '$3': .+$" "$1"; }   # C1 message shape

# ---------- C1: report, continue, exit 1 ----------
echo hello > a.txt
out=$("$B/cat" missing a.txt 2> e); rc=$?
is C1_cat_continue "hello" "$out"; is C1_cat_status 1 "$rc"
errfmt e cat missing && ok || bad C1_cat_message "$(cat e)"
touch f1; "$B/rm" nope f1 2> e; rc=$?
is C1_rm_continue no "$([ -e f1 ] && echo yes || echo no)"; is C1_rm_status 1 "$rc"
errfmt e rm nope && ok || bad C1_rm_message "$(cat e)"
mkdir have; "$B/mkdir" have fresh 2> e; rc=$?
is C1_mkdir_continue yes "$([ -d fresh ] && echo yes)"; is C1_mkdir_status 1 "$rc"
errfmt e mkdir have && ok || bad C1_mkdir_message "$(cat e)"
"$B/touch" ok1 /nope/x ok2 2> e; rc=$?
is C1_touch_continue yes "$([ -f ok1 ] && [ -f ok2 ] && echo yes)"; is C1_touch_status 1 "$rc"
"$B/find" nope 2> e > /dev/null; is C1_find_status 1 "$?"
errfmt e find nope && ok || bad C1_find_message "$(cat e)"

# ---------- C2: usage on missing operands ----------
for p in touch mkdir rm; do
  out=$("$B/$p" 2>&1); rc=$?; is "C2_${p}_status" 1 "$rc"; [ -n "$out" ] && ok || bad "C2_${p}_message" "no usage text"
done
out=$("$B/mv" onlyone 2>&1); is C2_mv_status 1 "$?"; [ -n "$out" ] && ok || bad C2_mv_message "no usage text"
"$B/rm" -f; is C2_rm_f_empty 0 "$?"

# ---------- I1: touch ----------
"$B/touch" new1 new2; is I1_create "0 0" "$(stat -c %s new1) $(stat -c %s new2)"
echo keep > old.txt; touch -d '2000-01-01 00:00' old.txt
"$B/touch" old.txt; is I1_keep_content keep "$(cat old.txt)"
[ "$(stat -c %Y old.txt)" -gt 1000000000 ] && ok || bad I1_mtime_now "$(stat -c %y old.txt)"
[ "$(stat -c %X old.txt)" -gt 1000000000 ] && ok || bad I1_atime_now "$(stat -c %x old.txt)"
is I1_status 0 "$("$B/touch" old.txt; echo $?)"

# ---------- I2: cat ----------
printf 'one\n' > p1; printf 'two' > p2; head -c 100000 /dev/urandom > bin
is I2_concat "$(printf 'one\ntwo')" "$("$B/cat" p1 p2)"
"$B/cat" bin > bin.out; cmp -s bin bin.out && ok || bad I2_binary "bytes differ"
is I2_stdin_noarg "xyz" "$(printf xyz | "$B/cat")"
is I2_stdin_dash "$(printf 'one\nMIDtwo')" "$(printf MID | "$B/cat" p1 - p2)"
is I2_status 0 "$("$B/cat" p1 > /dev/null; echo $?)"

# ---------- I3: mkdir ----------
"$B/mkdir" d1; is I3_create yes "$([ -d d1 ] && echo yes)"
"$B/mkdir" d1 2> /dev/null; is I3_exists_fails 1 "$?"
"$B/mkdir" -p p/q/r; is I3_parents yes "$([ -d p/q/r ] && echo yes)"
"$B/mkdir" -p p/q; is I3_parents_existing 0 "$?"
touch plain; "$B/mkdir" -p plain/x 2> /dev/null; is I3_parent_is_file 1 "$?"
"$B/mkdir" x/y 2> /dev/null; is I3_no_parent 1 "$?"

# ---------- I4: mv ----------
echo A > ma; echo B > mb
"$B/mv" ma mb; is I4_replace_file "A no" "$(cat mb) $([ -e ma ] && echo yes || echo no)"
mkdir md; echo C > mc; "$B/mv" mc md; is I4_into_dir C "$(cat md/mc 2>/dev/null)"
echo X > mx; echo Y > my; "$B/mv" mx my md; is I4_many_into_dir "X Y" "$(cat md/mx md/my 2>/dev/null | tr '\n' ' ' | sed 's/ $//')"
echo P > m1; echo Q > m2; echo R > mfile
"$B/mv" m1 m2 mfile 2> /dev/null; is I4_many_not_dir 1 "$?"
is I4_many_not_dir_untouched "P Q R" "$(cat m1 m2 mfile | tr '\n' ' ' | sed 's/ $//')"
"$B/mv" ghost there 2> /dev/null; is I4_missing_source 1 "$?"
mkdir mvdir; "$B/mv" mvdir renamed; is I4_rename_dir yes "$([ -d renamed ] && [ ! -e mvdir ] && echo yes)"

# ---------- I5: rm ----------
touch rf; "$B/rm" rf; is I5_file no "$([ -e rf ] && echo yes || echo no)"
mkdir rd; "$B/rm" rd 2> /dev/null; is I5_dir_needs_r "1 yes" "$? $([ -d rd ] && echo yes)"
mkdir -p rt/a/b; touch rt/a/b/c rt/x; "$B/rm" -r rt; is I5_recursive no "$([ -e rt ] && echo yes || echo no)"
"$B/rm" -f ghost; is I5_force_missing 0 "$?"
mkdir keepdir; touch keepdir/k; ln -s keepdir lnk; "$B/rm" lnk
is I5_symlink_only "no yes" "$([ -e lnk ] || [ -L lnk ] && echo yes || echo no) $([ -f keepdir/k ] && echo yes)"
mkdir -p rl; ln -s ../keepdir rl/out; "$B/rm" -r rl
is I5_no_follow_in_tree "no yes" "$([ -e rl ] && echo yes || echo no) $([ -f keepdir/k ] && echo yes)"
mkdir sub; (cd sub && "$B/rm" -r . 2> /dev/null; echo $? > ../rc1; "$B/rm" -r .. 2> /dev/null; echo $? > ../rc2)
is I5_refuse_dots "1 1 yes" "$(cat rc1) $(cat rc2) $([ -d sub ] && [ -f keepdir/k ] && echo yes)"

# ---------- I6: find ----------
mkdir -p ft/s/t; touch ft/a.c ft/b.h ft/s/c.c ft/s/t/d.txt; mkdir -p outside; touch outside/x.c; ln -s ../outside ft/link
sorted() { sort | tr '\n' ' ' | sed 's/ $//'; }
is I6_all "ft ft/a.c ft/b.h ft/link ft/s ft/s/c.c ft/s/t ft/s/t/d.txt" "$("$B/find" ft | sorted)"
is I6_name "ft/a.c ft/s/c.c" "$("$B/find" ft -name '*.c' | sorted)"
is I6_type_d "ft ft/s ft/s/t" "$("$B/find" ft -type d | sorted)"
is I6_type_f "ft/a.c ft/b.h ft/s/c.c ft/s/t/d.txt" "$("$B/find" ft -type f | sorted)"
is I6_name_and_type "ft/s/t/d.txt" "$("$B/find" ft -name 'd*' -type f | sorted)"
is I6_default_dot "./a.c ./s/c.c" "$(cd ft && "$B/find" -name '*.c' | sorted)"
is I6_many_paths "ft/s/c.c outside/x.c" "$("$B/find" ft/s outside -name '*.c' | sorted)"
is I6_status 0 "$("$B/find" ft > /dev/null; echo $?)"

# ---------- added after the skeptic review (README "회의론자") ----------
# stdbuf preloads a library; let ASan builds accept that
export ASAN_OPTIONS="${ASAN_OPTIONS:-}:verify_asan_link_order=0"
"$B/mkdir" -p '' sk_good 2> /dev/null; rc=$?
is I3_empty_operand "1 yes" "$rc $([ -d sk_good ] && echo yes)"
printf x | stdbuf -o0 "$B/cat" > /dev/full 2> /dev/null; is I2_write_error 1 "$?"
timeout 5 "$B/cat" /dev/zero > /dev/full 2> /dev/null; is I2_write_error_stops 1 "$?"
stdbuf -o0 "$B/find" ft > /dev/full 2> /dev/null; is I6_write_error 1 "$?"
is I6_all_tests_name "" "$("$B/find" ft -name 'a*' -name '*.h' | sorted)"
is I6_all_tests_type "" "$("$B/find" ft -type f -type d | sorted)"
fresh() { rm -rf sk_src sk_dst sk_moved; echo s > sk_src; echo d > sk_dst; }
echo keep > sk_file
"$B/rm" sk_file/ 2> /dev/null; rc=$?
is I5_slash_on_file "1 keep" "$rc $(cat sk_file 2> /dev/null)"
fresh; "$B/mv" sk_src/ sk_dst 2> /dev/null; rc=$?
is I4_slash_on_source "1 s d" "$rc $(cat sk_src 2> /dev/null) $(cat sk_dst)"
fresh; "$B/mv" sk_src sk_dst/ 2> /dev/null; rc=$?
is I4_slash_on_target "1 s d" "$rc $(cat sk_src 2> /dev/null) $(cat sk_dst)"
"$B/find" sk_file/ > /dev/null 2>&1; is I6_slash_on_file 1 "$?"
mkdir sk_dd; rc=$(cd sk_dd && "$B/touch" -- > /dev/null 2>&1; echo $?)
is C2_dashdash_touch "1 " "$rc $(ls sk_dd)"
is C2_dashdash_cat "one 0" "$("$B/cat" -- p1 2> /dev/null) $?"
fresh; "$B/mv" -- sk_src sk_moved 2> /dev/null; rc=$?
is C2_dashdash_mv "0 s" "$rc $(cat sk_moved 2> /dev/null)"
is C2_dashdash_find "ft/s/t ft/s/t/d.txt" "$("$B/find" -- ft/s/t 2> /dev/null | sorted)"

echo "intent tests: $pass passed, $fail failed"
cd / && rm -rf "$T"
[ "$fail" -eq 0 ]
