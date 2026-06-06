# Development Setup

## Prerequisites

| Tool | Minimum version | Purpose |
|------|-----------------|---------|
| Rust | stable (edition 2021) | Builds the worker executable |
| cargo | bundled with Rust | Runs builds and tests |

## Build

```bash
cargo build
cargo build --release
```

| Profile | Artifact |
|---------|----------|
| Debug | `target/debug/typio-engine-basic` |
| Release | `target/release/typio-engine-basic` |

## Development Run

```bash
cargo build --release
```

`typio-engine-basic.toml` uses `command = "./target/release/typio-engine-basic"`,
so the release worker must exist before starting the host.

From the `typio-linux` repository:

```bash
LD_LIBRARY_PATH=../libtypio/target/release \
  ./build/src/typio --verbose \
  --engine-dir ../typio-engine-basic
```

See [How to Package the Engine for Distribution](../how-to/package-for-distribution.md)
for production installation.
