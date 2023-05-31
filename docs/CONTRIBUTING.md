# Contributing and verification

Use Rust 1.88 or newer. Runtime rendering depends on `num` and `termios`; Naga is a development-only shader validator. The crate and binary forbid unsafe source code, but that does not audit transitive dependencies. Linux and macOS are the supported CI targets; no Windows support is claimed for the termios-backed keyboard adapter.

Before proposing a change:

```sh
cargo fmt --check
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo test --locked --all-targets --all-features
cargo test --locked --doc
cargo build --locked --release
python3 -m unittest discover -s tests -p 'test_*.py'
```

Test behavior before optimizing it. Serial rendering is the CPU oracle; compare complete orbit results, not timing output or an aggregate checksum alone. Include a failing reproducer for floating-point edge cases. Exercise PNGs with multiple DEFLATE blocks and verify CRC/zlib data using `scripts/verify_png.py`; that checker intentionally supports only this project's RGB8/filter-zero subset.

Keep output files under `renders/` or outside the source tree. Existing files must never be replaced or removed by export failure cleanup. Atomic no-clobber publication currently requires hard-link support on the destination filesystem. Recipe files are whitespace-tokenized numeric settings, not shell programs: they are UTF-8, capped at 64 KiB, cannot load other recipes and cannot request file output or interactive mode. Later command-line arguments override earlier loaded settings.

These limits are guardrails, not a sandbox. Do not expose rendering directly to untrusted remote requests without a process memory/time budget: valid dimensions and iteration caps can still consume substantial CPU and memory. Cancellation occurs between rows only in the progress-callback API. GPU code is a source-validated reference until hardware execution is implemented and independently measured.

Git dates in the reconstructed series are intentionally synthetic; don't use them as release timestamps, development chronology evidence or benchmark provenance. No CI run is implied until the workflow is actually executed by a host. Local checks do not certify another operating system.
