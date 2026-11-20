#!/usr/bin/env python3
"""Integration tests for the local binaries; run on Linux/WSL with make test."""
import os
from pathlib import Path
import shutil
import stat
import subprocess
import tempfile
import time
import unittest

ROOT = Path(__file__).resolve().parent


class Utilities(unittest.TestCase):
    def setUp(self):
        self.fixture = tempfile.TemporaryDirectory(prefix=".fileutils-test-", dir=ROOT)
        self.addCleanup(self.fixture.cleanup)
        self.work = Path(self.fixture.name)

    def run_tool(self, tool, *args, data=None, status=0, env=None):
        result = subprocess.run(
            [str(ROOT / tool), *map(str, args)], cwd=self.work,
            input=data, stdout=subprocess.PIPE, stderr=subprocess.PIPE, env=env,
        )
        self.assertEqual(result.returncode, status,
                         f"{tool} {args}: {result.stderr!r}")
        return result.stdout

    def put(self, name, content=b""):
        path = self.work / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(content)
        return path

    def paths(self, *args):
        return set(self.run_tool("find", *args).decode().splitlines())

    def test_help_and_usage(self):
        for tool in ("touch", "cat", "mkdir", "mv", "rm", "find"):
            with self.subTest(tool=tool):
                self.assertIn(b"Usage:", self.run_tool(tool, "--help"))
                self.run_tool(tool, "--unsupported-option", status=1)
        for tool in ("touch", "mkdir", "mv", "rm"):
            self.run_tool(tool, status=1)

    def test_touch_create_reference_and_selective_times(self):
        self.run_tool("touch", "one", "two")
        self.assertEqual((self.work / "one").read_bytes(), b"")
        os.utime(self.work / "one", ns=(1_200_000_000_123456789, 1_300_000_000_987654321))
        reference = (self.work / "one").stat()
        self.run_tool("touch", "-r", "one", "two")
        copied = (self.work / "two").stat()
        self.assertEqual(copied.st_atime_ns, reference.st_atime_ns)
        self.assertEqual(copied.st_mtime_ns, reference.st_mtime_ns)
        self.run_tool("touch", "-a", "two")
        self.assertEqual((self.work / "two").stat().st_mtime_ns, copied.st_mtime_ns)
        self.assertGreater((self.work / "two").stat().st_atime_ns, copied.st_atime_ns)
        self.run_tool("touch", "-m", "-r", "one", "two")
        self.assertEqual((self.work / "two").stat().st_mtime_ns, reference.st_mtime_ns)
        self.run_tool("touch", "-c", "absent")
        self.assertFalse((self.work / "absent").exists())
        self.run_tool("touch", "-t", "202402291234.56", "dated")
        tm = time.localtime((self.work / "dated").stat().st_mtime)
        self.assertEqual(tuple(tm[:6]), (2024, 2, 29, 12, 34, 56))
        self.run_tool("touch", "-t", "202302291234", "invalid", status=1)
        self.run_tool("touch", "missing/child", "after-error", status=1)
        self.assertTrue((self.work / "after-error").exists())

    def test_touch_symlinks_and_no_truncation(self):
        file = self.put("file", b"keep me")
        link = self.work / "link"
        link.symlink_to("file")
        os.utime(file, (1000000000, 1000000000))
        before = file.stat().st_mtime_ns
        self.run_tool("touch", "-h", "link")
        self.assertEqual(file.stat().st_mtime_ns, before)
        self.run_tool("touch", "link")
        self.assertGreater(file.stat().st_mtime_ns, before)
        self.assertEqual(file.read_bytes(), b"keep me")
        (self.work / "dangling").symlink_to("created")
        self.run_tool("touch", "dangling")
        self.assertTrue((self.work / "created").is_file())
        self.run_tool("touch", "--", "-dash")
        self.assertTrue((self.work / "-dash").is_file())

    def test_cat_binary_stdin_and_errors(self):
        data = bytes(range(256)) * 600
        self.put("binary", data)
        self.assertEqual(self.run_tool("cat", "binary"), data)
        self.assertEqual(self.run_tool("cat", data=b"stdin\0"), b"stdin\0")
        self.put("a", b"A")
        self.assertEqual(self.run_tool("cat", "a", "-", "a", data=b"B"), b"ABA")
        self.assertEqual(self.run_tool("cat", "missing", "a", status=1), b"A")
        self.run_tool("cat", ".", status=1)
        self.run_tool("cat", "--", "-missing", status=1)

    def test_cat_formatting_and_file_boundaries(self):
        self.put("a", b"a\n\n")
        self.put("b", b"\n\nb\n\n")
        self.assertEqual(self.run_tool("cat", "-ns", "a", "b"),
                         b"     1\ta\n     2\t\n     3\tb\n     4\t\n")
        self.assertEqual(self.run_tool("cat", "-bn", data=b"\nX\n\nY"),
                         b"\n     1\tX\n\n     2\tY")
        self.assertEqual(self.run_tool("cat", "-A", data=b"\t\0\r\n\x7f\x80\xff"),
                         b"^I^@^M$\n^?M-^@M-^?")
        self.put("partial", b"part")
        self.put("rest", b"ial\nnext")
        self.assertEqual(self.run_tool("cat", "-n", "partial", "rest"),
                         b"     1\tpartial\n     2\tnext")
        # Compare combinations against the installed Linux cat for tricky byte cases.
        system_cat = shutil.which("cat")
        if system_cat:
            sample = b"\n\nA\0\t\r\n\n\nZ\xff\n"
            for option in ("-n", "-b", "-s", "-A", "-v", "-bsE", "-nsT"):
                expected = subprocess.check_output([system_cat, option], input=sample)
                self.assertEqual(self.run_tool("cat", option, data=sample), expected)

    def test_mkdir_parents_existing_and_errors(self):
        self.run_tool("mkdir", "-p", "a/b/c", "other")
        self.assertTrue((self.work / "a/b/c").is_dir())
        self.run_tool("mkdir", "-p", "a/b/c", "a//b/./c/")
        self.run_tool("mkdir", "a", status=1)
        self.put("file")
        self.run_tool("mkdir", "-p", "file/sub", "after", status=1)
        self.assertTrue((self.work / "after").is_dir())
        self.run_tool("mkdir", "-m", "invalid", "bad", status=1)
        self.run_tool("mkdir", "--", "-dash")
        self.assertTrue((self.work / "-dash").is_dir())

    def test_mkdir_modes_when_supported(self):
        probe = self.put("permission-probe")
        probe.chmod(0o600)
        if stat.S_IMODE(probe.stat().st_mode) != 0o600:
            self.skipTest("filesystem does not implement POSIX chmod modes")
        self.run_tool("mkdir", "-m", "751", "mode")
        self.assertEqual(stat.S_IMODE((self.work / "mode").stat().st_mode), 0o751)
        old_umask = os.umask(0o077)
        try:
            self.run_tool("mkdir", "-p", "-m", "755", "parents/final")
        finally:
            os.umask(old_umask)
        self.assertEqual(stat.S_IMODE((self.work / "parents").stat().st_mode), 0o700)
        self.assertEqual(stat.S_IMODE((self.work / "parents/final").stat().st_mode), 0o755)

    def test_mv_rename_overwrite_and_interactive(self):
        self.put("source", b"source")
        self.run_tool("mv", "source", "dest")
        self.assertFalse((self.work / "source").exists())
        self.put("source", b"new")
        self.run_tool("mv", "-n", "source", "dest")
        self.assertEqual((self.work / "dest").read_bytes(), b"source")
        self.assertTrue((self.work / "source").exists())
        self.run_tool("mv", "-i", "source", "dest", data=b"n\n")
        self.assertTrue((self.work / "source").exists())
        self.run_tool("mv", "-i", "source", "dest", data=b"y\n")
        self.assertEqual((self.work / "dest").read_bytes(), b"new")
        self.run_tool("mv", "dest", "dest", status=1)
        self.put("another", b"other")
        self.run_tool("mv", "-nf", "another", "dest")
        self.assertEqual((self.work / "dest").read_bytes(), b"other")

    def test_mv_directories_multiple_sources_and_links(self):
        self.put("tree/child", b"child")
        self.run_tool("mv", "tree", "renamed")
        self.assertTrue((self.work / "renamed/child").is_file())
        self.run_tool("mkdir", "bucket")
        self.put("a", b"a")
        (self.work / "link").symlink_to("a")
        self.run_tool("mv", "a", "link", "bucket")
        self.assertTrue((self.work / "bucket/link").is_symlink())
        self.assertEqual((self.work / "bucket/link").read_bytes(), b"a")
        self.run_tool("mv", "renamed", "renamed/inside", status=1)
        self.run_tool("mv", "missing", "bucket/a", "bucket", status=1)
        self.run_tool("mv", "bucket/a", "bucket/link", "not-a-dir", status=1)
        self.run_tool("mv", "-T", "bucket/a", "renamed", status=1)

    def test_rm_recursive_symlinks_and_protection(self):
        self.put("tree/deep/file", b"x")
        self.put("outside/keep", b"safe")
        (self.work / "tree/link").symlink_to("../outside", target_is_directory=True)
        self.run_tool("rm", "tree", status=1)
        self.run_tool("rm", "-r", "tree")
        self.assertFalse((self.work / "tree").exists())
        self.assertEqual((self.work / "outside/keep").read_bytes(), b"safe")
        self.run_tool("rm", "-f", "absent")
        self.run_tool("rm", "-f")
        self.run_tool("rm", "absent", status=1)
        self.run_tool("rm", "-d", "outside", status=1)
        self.run_tool("mkdir", "empty")
        self.run_tool("rm", "-d", "empty")
        # Do not pass -r in root protection tests, even when the guard is broken.
        self.run_tool("rm", "-d", "/", status=1)
        self.run_tool("rm", "-d", ".", status=1)
        self.run_tool("rm", "-d", "outside/..", status=1)
        self.assertTrue((self.work / "outside/keep").exists())

    def test_rm_prompts_continue_and_dangling_link(self):
        self.put("file", b"keep")
        self.run_tool("rm", "-i", "file", data=b"no\n")
        self.assertTrue((self.work / "file").exists())
        self.run_tool("rm", "-i", "file", data=b"yes\n")
        self.assertFalse((self.work / "file").exists())
        self.put("tree/a")
        self.put("tree/b")
        self.run_tool("rm", "-ri", "tree", data=b"y\nn\nn\n")
        self.assertTrue((self.work / "tree/a").exists())
        self.assertTrue((self.work / "tree/b").exists())
        (self.work / "dangling").symlink_to("absent")
        self.run_tool("rm", "dangling")
        self.assertFalse((self.work / "dangling").is_symlink())
        self.put("ok")
        self.run_tool("rm", "missing", "ok", status=1)
        self.assertFalse((self.work / "ok").exists())

    def make_find_tree(self):
        self.put("tree/a.c", b"abc")
        self.put("tree/A.H")
        self.put("tree/sub/b.c", b"12345")
        self.put("tree/sub/x.txt", b"hello")
        (self.work / "tree/empty").mkdir()
        (self.work / "tree/link").symlink_to("sub", target_is_directory=True)
        (self.work / "tree/broken").symlink_to("missing")

    def test_find_names_types_depth_and_nul(self):
        self.make_find_tree()
        self.assertEqual(self.paths("tree", "-name", "*.c"), {"tree/a.c", "tree/sub/b.c"})
        self.assertEqual(self.paths("tree", "-iname", "*.h"), {"tree/A.H"})
        self.assertEqual(self.paths("tree", "-type", "l"), {"tree/link", "tree/broken"})
        self.assertEqual(self.paths("tree", "-maxdepth", "0"), {"tree"})
        self.assertEqual(self.paths("tree", "-mindepth", "2", "-maxdepth", "2"),
                         {"tree/sub/b.c", "tree/sub/x.txt"})
        self.assertEqual(self.paths("tree", "-path", "*/sub/*"), {"tree/sub/b.c", "tree/sub/x.txt"})
        self.put("tree/new\nline")
        printed = self.run_tool("find", "tree", "-type", "f", "-print0")
        self.assertIn(b"tree/new\nline\0", printed)
        self.assertEqual(printed.count(b"\0"), 5)
        self.assertEqual(self.paths("tree/a.c", "tree/A.H"), {"tree/a.c", "tree/A.H"})
        self.assertIn(".", self.paths("-maxdepth", "0"))

    def test_find_expression_precedence_pruning_and_short_circuit(self):
        self.make_find_tree()
        self.assertEqual(self.paths("tree", "(", "-name", "*.c", "-o", "-name", "*.H", ")",
                                    "-a", "!", "-path", "*/sub/*"), {"tree/a.c", "tree/A.H"})
        self.assertEqual(self.paths("tree", "-name", "sub", "-prune", "-o", "-type", "f", "-print"),
                         {"tree/a.c", "tree/A.H"})
        self.assertEqual(self.paths("tree", "-false", "-a", "-print"), set())
        self.assertEqual(self.paths("tree", "-true", "-o", "-print"), set())
        self.assertEqual(self.paths("tree", "-name", "sub", "-o", "-name", "*.c", "-type", "f"),
                         {"tree/sub", "tree/a.c", "tree/sub/b.c"})
        self.assertEqual(self.paths("tree", "-not", "-true"), set())
        order = self.run_tool("find", "tree", "-depth").decode().splitlines()
        self.assertLess(order.index("tree/sub/b.c"), order.index("tree/sub"))
        self.assertEqual(order[-1], "tree")

    def test_find_sizes_times_empty_and_delete(self):
        self.make_find_tree()
        self.assertEqual(self.paths("tree", "-type", "f", "-size", "+3c"),
                         {"tree/sub/b.c", "tree/sub/x.txt"})
        self.assertEqual(self.paths("tree", "-empty"), {"tree/A.H", "tree/empty"})
        self.assertEqual(self.paths("tree", "-type", "f", "-size", "1"),
                         {"tree/a.c", "tree/sub/b.c", "tree/sub/x.txt"})
        old = time.time() - 3 * 86400 - 5
        os.utime(self.work / "tree/a.c", (old, old))
        self.assertEqual(self.paths("tree", "-mtime", "+1"), {"tree/a.c"})
        self.assertEqual(self.paths("tree", "-mmin", "+100"), {"tree/a.c"})
        self.assertEqual(self.paths("tree", "-type", "f", "-newer", "tree/a.c"),
                         {"tree/A.H", "tree/sub/b.c", "tree/sub/x.txt"})
        self.run_tool("find", "tree", "-type", "f", "-empty", "-delete")
        self.assertFalse((self.work / "tree/A.H").exists())
        self.run_tool("find", "tree/sub", "-delete")
        self.assertFalse((self.work / "tree/sub").exists())
        self.assertTrue((self.work / "tree/a.c").exists())

    def test_find_invalid_expressions_and_missing_paths(self):
        self.put("ok")
        for expression in (("-name",), ("(", "-true"), ("-true", "-o"),
                           ("-size", "3z"), ("-maxdepth", "-1"), ("-type", "q"),
                           ("-true", ")"), ("-mtime", "+"), ("-size", "99999999999999999999999")):
            with self.subTest(expression=expression):
                self.run_tool("find", "ok", *expression, status=1)
        self.assertEqual(self.run_tool("find", "missing", "ok", status=1), b"ok\n")

    def test_cross_filesystem_move_fallback(self):
        # Inject EXDEV only on the initial rename. This exercises the entire
        # fallback without creating fixtures outside the project directory.
        shim_source = self.put("exdev.c", br'''
#define _GNU_SOURCE
#include <errno.h>
#include <stdlib.h>
#include <string.h>
#include <sys/syscall.h>
#include <unistd.h>
int rename(const char *a, const char *b) {
    const char *source = getenv("FILEUTILS_TEST_EXDEV");
    if (source && !strcmp(a, source)) { errno = EXDEV; return -1; }
    return (int)syscall(SYS_renameat, -100, a, -100, b);
}
ssize_t read(int fd, void *buffer, size_t size) {
    if (getenv("FILEUTILS_TEST_READ_FAIL")) { errno = EIO; return -1; }
    return syscall(SYS_read, fd, buffer, size);
}
''')
        library = self.work / "exdev.so"
        subprocess.run(["cc", "-shared", "-fPIC", str(shim_source), "-o", str(library)], check=True)
        # ld.so splits LD_PRELOAD on whitespace, so use a path relative to cwd.
        env = dict(os.environ, LD_PRELOAD="./exdev.so", FILEUTILS_TEST_EXDEV="source")
        self.put("source/sub/file", b"preserved")
        (self.work / "source/link").symlink_to("sub/file")
        self.run_tool("mv", "source", "dest", env=env)
        self.assertFalse((self.work / "source").exists())
        self.assertEqual((self.work / "dest/sub/file").read_bytes(), b"preserved")
        self.assertEqual(os.readlink(self.work / "dest/link"), "sub/file")
        source = self.put("source", b"replacement")
        os.utime(source, ns=(1_200_000_000_123456700, 1_300_000_000_987654300))
        original = source.stat()
        self.put("existing", b"old")
        self.run_tool("mv", "source", "existing", env=env)
        self.assertEqual((self.work / "existing").read_bytes(), b"replacement")
        self.assertEqual((self.work / "existing").stat().st_mtime_ns, original.st_mtime_ns)
        self.assertFalse(source.exists())
        # A failed copy must not destroy the destination or source.
        self.put("source", b"keep on failure")
        bad_env = dict(env, FILEUTILS_TEST_READ_FAIL="1")
        self.run_tool("mv", "source", "existing", env=bad_env, status=1)
        self.assertEqual(source.read_bytes(), b"keep on failure")
        self.assertEqual((self.work / "existing").read_bytes(), b"replacement")
        self.assertFalse(list(self.work.glob(".fileutils-mv-*")))


if __name__ == "__main__":
    unittest.main(verbosity=2)
