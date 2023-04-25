import importlib.util
from pathlib import Path
import struct
import unittest
import zlib

spec = importlib.util.spec_from_file_location("checker", Path(__file__).parents[1] / "scripts/verify_png.py")
checker = importlib.util.module_from_spec(spec)
spec.loader.exec_module(checker)


def chunk(name, payload):
    return struct.pack(">I", len(payload)) + name + payload + struct.pack(">I", zlib.crc32(name + payload))


class VerifyPngTests(unittest.TestCase):
    def test_independently_compressed_rgb_pixels_decode(self):
        data = b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", struct.pack(">IIBBBBB", 2, 1, 8, 2, 0, 0, 0))
        data += chunk(b"IDAT", zlib.compress(b"\0\x01\x02\x03\xff\0\x80")) + chunk(b"IEND", b"")
        self.assertEqual(checker.decode(data), (2, 1, b"\x01\x02\x03\xff\0\x80"))
        corrupted = bytearray(data)
        corrupted[20] ^= 1
        with self.assertRaises(ValueError):
            checker.decode(bytes(corrupted))
        with self.assertRaises(ValueError):
            checker.decode(data + b"trailing")
        with self.assertRaises(ValueError):
            checker.decode(data[:-3])


if __name__ == "__main__":
    unittest.main()
