#!/usr/bin/env python3
"""Independent checker for this project's non-interlaced RGB8/filter-zero PNGs."""
import argparse
import hashlib
import struct
from pathlib import Path
import zlib


def decode(data: bytes) -> tuple[int, int, bytes]:
    if not data.startswith(b"\x89PNG\r\n\x1a\n"):
        raise ValueError("not a PNG signature")
    at, width, height = 8, 0, 0
    payload = bytearray()
    ended = False
    while at < len(data):
        if at + 12 > len(data):
            raise ValueError("truncated chunk")
        size = struct.unpack_from(">I", data, at)[0]
        kind = data[at + 4:at + 8]
        end = at + 8 + size
        if end + 4 > len(data):
            raise ValueError("truncated payload")
        body = data[at + 8:end]
        checksum = struct.unpack_from(">I", data, end)[0]
        if zlib.crc32(kind + body) != checksum:
            raise ValueError(f"bad {kind!r} CRC")
        if kind == b"IHDR":
            if width or at != 8 or len(body) != 13:
                raise ValueError("invalid IHDR placement/length")
            width, height, depth, color, compression, filtering, interlace = struct.unpack(">IIBBBBB", body)
            if not width or not height or (depth, color, compression, filtering, interlace) != (8, 2, 0, 0, 0):
                raise ValueError("expected nonempty non-interlaced RGB8")
        elif kind == b"IDAT":
            if not width:
                raise ValueError("IDAT before IHDR")
            payload.extend(body)
        elif kind == b"IEND":
            if body or end + 4 != len(data):
                raise ValueError("invalid IEND or trailing bytes")
            ended = True
        else:
            raise ValueError(f"unexpected chunk {kind!r}")
        at = end + 4
    if not ended or not payload:
        raise ValueError("missing image data or IEND")
    inflater = zlib.decompressobj()
    raw = inflater.decompress(payload) + inflater.flush()
    if not inflater.eof or inflater.unused_data:
        raise ValueError("incomplete or trailing zlib stream")
    stride = width * 3 + 1
    if len(raw) != height * stride:
        raise ValueError("decompressed size disagrees with dimensions")
    rows = []
    for y in range(height):
        row = raw[y * stride:(y + 1) * stride]
        if row[0] != 0:
            raise ValueError("checker supports filter-zero rows only")
        rows.append(row[1:])
    return width, height, b"".join(rows)


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("path", type=Path)
    args = parser.parse_args()
    try:
        width, height, pixels = decode(args.path.read_bytes())
    except (OSError, ValueError, zlib.error) as error:
        parser.exit(1, f"{error}\n")
    print(f"{args.path}: {width}x{height}; {len(pixels)} RGB bytes; sha256={hashlib.sha256(pixels).hexdigest()}")


if __name__ == "__main__":
    main()
