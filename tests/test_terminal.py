"""Real POSIX pseudoterminal smoke tests for navigation and cleanup."""
import os
from pathlib import Path
import select
import subprocess
import time
import unittest

if os.name == "posix":
    import pty
    import termios

BIN = Path(os.environ.get("MANDELBROT_BIN", str(Path(__file__).parents[1] / "target/release/mandelbrot")))


@unittest.skipUnless(os.name == "posix" and BIN.is_file(), "requires a built POSIX release binary")
class TerminalTests(unittest.TestCase):
    def exercise(self, quit_key, navigate):
        master, slave = pty.openpty()
        before = termios.tcgetattr(slave)
        captured = bytearray()
        process = None
        try:
            process = subprocess.Popen([str(BIN), "--interactive", "--color", "--width", "16", "--height", "8", "--threads", "2"], stdin=slave, stdout=slave, stderr=slave)

            def frame(number):
                end = time.monotonic() + 8
                while captured.count(b"q quit") < number:
                    self.assertLess(time.monotonic(), end, f"missing frame {number}: {captured[-500:]!r}")
                    if select.select([master], [], [], 0.1)[0]:
                        captured.extend(os.read(master, 65536))

            frame(1)
            if navigate:
                for number, key in enumerate(b"kpuy][rj", start=2):
                    os.write(master, bytes([key]))
                    frame(number)
            os.write(master, quit_key)
            self.assertEqual(process.wait(timeout=8), 0)
            while select.select([master], [], [], 0.1)[0]:
                data = os.read(master, 65536)
                if not data:
                    break
                captured.extend(data)
            self.assertIn(b"\x1b[48;2;", captured)
            self.assertIn(b"\x1b[?25h\x1b[?1049l", captured)
            after = termios.tcgetattr(slave)
            # Darwin can set the transient PENDIN retype flag when restoring canonical mode.
            mask = ~getattr(termios, "PENDIN", 0)
            before[3] &= mask
            after[3] &= mask
            self.assertEqual(after, before)
        finally:
            if process is not None and process.poll() is None:
                process.terminate()
                process.wait(timeout=5)
            os.close(master)
            os.close(slave)

    def test_navigation_palette_and_quit_restore_terminal(self):
        self.exercise(b"q", True)

    def test_standalone_escape_is_not_swallowed(self):
        self.exercise(b"\x1b", False)

    def test_control_c_is_a_clean_guarded_exit(self):
        self.exercise(b"\x03", False)


if __name__ == "__main__":
    unittest.main()
