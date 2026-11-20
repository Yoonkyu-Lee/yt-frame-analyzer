"""Run with: wsl.exe -e python3 test_terminal.py [./main]"""
import fcntl
import os
import pty
import select
import signal
import struct
import subprocess
import sys
import termios
import time

binary = sys.argv[1] if len(sys.argv) > 1 else "./main"


def capture(*args):
    return subprocess.run([binary, *args], capture_output=True, timeout=5)


snapshot = capture("--seed", "42")
assert snapshot.returncode == 0
assert b"\x1b" not in snapshot.stdout
assert any(48 <= c <= 57 for c in snapshot.stdout)
assert capture("--seed", "42").stdout == snapshot.stdout
assert capture("--help").returncode == 0
for args in [("--frames", "0"), ("--seed", "-1"), ("--wat",),
             ("--frames",), ("--seed", "4294967296")]:
    assert capture(*args).returncode != 0, args


def terminal_session(quit_with_signal):
    master, slave = pty.openpty()
    original = termios.tcgetattr(slave)

    def resize(rows, columns):
        fcntl.ioctl(slave, termios.TIOCSWINSZ, struct.pack("HHHH", rows, columns, 0, 0))

    def expect(token):
        data = bytearray()
        deadline = time.monotonic() + 5
        while token not in data and time.monotonic() < deadline:
            if select.select([master], [], [], 0.1)[0]:
                data.extend(os.read(master, 65536))
        assert token in data, f"Did not receive {token!r}; exit={process.poll()}"
        return data

    resize(24, 80)
    process = subprocess.Popen([binary, "--seed", "42"], stdin=slave,
                               stdout=slave, stderr=slave)
    try:
        expect(b"FLOWING")
        os.write(master, b" ")
        expect(b"PAUSED")
        os.write(master, b"+")
        expect(b"1.25x")
        os.write(master, b"-r ")
        expect(b"FLOWING  1.00x")
        previous_size = (24, 80)
        for rows, columns in [(1, 1), (4, 8), (80, 240), (100, 300), (24, 80)]:
            resize(rows, columns)
            size = (min(rows, 80), min(columns, 240))
            expect(b"\x1b[2J" if size != previous_size else b"\x1b[H")
            previous_size = size
            assert process.poll() is None
        if quit_with_signal:
            process.send_signal(signal.SIGINT)
        else:
            os.write(master, b"q")
        expect(b"\x1b[?25h\x1b[?1049l")
        assert process.wait(timeout=5) == 0
        assert termios.tcgetattr(slave) == original, "Terminal settings were not restored"
    finally:
        if process.poll() is None:
            process.kill()
            process.wait()
        os.close(master)
        os.close(slave)


terminal_session(False)
terminal_session(True)
print("PASS: snapshots, arguments, controls, resize, quit, SIGINT, terminal restoration")
