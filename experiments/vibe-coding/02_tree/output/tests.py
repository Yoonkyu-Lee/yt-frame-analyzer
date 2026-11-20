"""Integration tests; run under WSL with: python3 tests.py ./ctree."""

import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest


PROGRAM = str(Path(sys.argv.pop(1) if len(sys.argv) > 1 else './ctree').resolve())


class TreeTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix='ctree-test-')
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        (self.root / 'branch' / 'nested').mkdir(parents=True)
        (self.root / 'branch' / 'leaf.txt').touch()
        (self.root / 'alpha.txt').touch()
        (self.root / '.hidden').touch()
        (self.root / 'empty').mkdir()

    def run_tree(self, *args, status=0):
        result = subprocess.run(
            [PROGRAM, *args], cwd=self.root, capture_output=True,
            text=True, encoding='utf-8', timeout=10,
        )
        self.assertEqual(result.returncode, status, result.stdout + result.stderr)
        return result.stdout

    def test_default_exact_output(self):
        self.assertEqual(self.run_tree(),
                         '.\n├── alpha.txt\n├── branch\n│   ├── leaf.txt\n'
                         '│   └── nested\n└── empty\n\n3 directories, 2 files\n')

    def test_ascii_and_depth(self):
        self.assertEqual(self.run_tree('--ascii', '-L', '1'),
                         '.\n|-- alpha.txt\n|-- branch\n`-- empty\n'
                         '\n2 directories, 1 file\n')
        self.assertEqual(self.run_tree('-L', '0', '--noreport'), '.\n')

    def test_hidden_and_ignore(self):
        self.assertIn('.hidden', self.run_tree('-a'))
        output = self.run_tree('-a', '-I', '*.txt')
        self.assertIn('.hidden', output)
        self.assertNotIn('alpha.txt', output)
        self.assertNotIn('leaf.txt', output)
        self.assertNotIn('nested', self.run_tree('-I', 'branch'))

    def test_directories_and_sorting(self):
        output = self.run_tree('-d')
        self.assertNotIn('.txt', output)
        self.assertTrue(output.endswith('3 directories\n'))
        output = self.run_tree('--dirsfirst', '-L', '1')
        self.assertLess(output.index('branch'), output.index('empty'))
        self.assertLess(output.index('empty'), output.index('alpha.txt'))

    def test_paths_and_classification(self):
        self.assertIn('./branch/leaf.txt', self.run_tree('-f'))
        executable = self.root / 'run'
        executable.touch()
        executable.chmod(0o755)
        os.mkfifo(self.root / 'pipe')
        output = self.run_tree('-F')
        for label in ('branch/', 'empty/', 'run*', 'pipe|'):
            self.assertIn(label, output)

    def test_links_and_cycles(self):
        (self.root / 'alias').symlink_to('branch', target_is_directory=True)
        (self.root / 'branch' / 'back').symlink_to('..', target_is_directory=True)
        output = self.run_tree()
        self.assertIn('alias -> branch', output)
        self.assertNotIn('[cycle]', output)
        self.assertNotIn('alias', self.run_tree('-d'))
        output = self.run_tree('-l')
        self.assertEqual(output.count('[cycle]'), 2)
        self.assertIn('leaf.txt', self.run_tree('-l', 'alias'))

    def test_broken_link(self):
        (self.root / 'broken').symlink_to('missing')
        self.assertIn('broken -> missing', self.run_tree())
        self.assertIn('[error:', self.run_tree('-l', status=1))

    def test_roots_and_errors(self):
        output = self.run_tree('missing', 'empty', status=1)
        self.assertIn('missing [error:', output)
        self.assertIn('\nempty\n', output)
        self.assertEqual(self.run_tree('alpha.txt'), 'alpha.txt\n\n0 directories, 1 file\n')
        self.assertEqual(self.run_tree('empty'), 'empty\n\n0 directories, 0 files\n')

    def test_unreadable_directory(self):
        if os.geteuid() == 0:
            self.skipTest('root bypasses directory permissions')
        locked = self.root / 'locked'
        locked.mkdir()
        locked.chmod(0)
        try:
            self.assertIn('locked [error:', self.run_tree(status=1))
        finally:
            locked.chmod(0o700)

    def test_filename_controls_and_unicode(self):
        for name in ('line\nbreak', 'escape\x1b', 'back\\slash', '나무'):
            (self.root / name).touch()
        output = self.run_tree()
        for label in ('line\\x0abreak', 'escape\\x1b', 'back\\x5cslash', '나무'):
            self.assertIn(label, output)

    def test_cli_validation(self):
        for value in ('-1', 'abc', '1x', '', '+1', '999999999999999999999999999'):
            self.run_tree('-L', value, status=1)
        self.run_tree('--unknown-option', status=1)
        self.run_tree('-L', status=1)
        self.assertIn('Usage:', self.run_tree('--help'))
        self.assertEqual(self.run_tree('--version'), 'ctree 1.0.0\n')
        (self.root / '-dash').mkdir()
        self.assertTrue(self.run_tree('--', '-dash').startswith('-dash\n'))


if __name__ == '__main__':
    unittest.main()
