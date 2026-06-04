# Development Setup

How to prepare your machine to build, test, and modify `typio-engine-basic`.

## Prerequisites

| Tool | Minimum version | Purpose |
|------|-----------------|---------|
| [Rust](https://rustup.rs/) | stable (edition 2021) | Compiles the engine. |
| cargo | bundled with Rust | Dependency resolution and build orchestration. |
| gcc / clang | any recent | C compiler for linking the `cdylib`. |
| strip | binutils | Optional — for removing debug symbols from release builds. |
| nm | binutils | Optional — for inspecting exported symbols. |

> **Repository layout note:** `typio-engine-basic` has path dependencies on sibling directories (`../libtypio/crates/abi`, `../typio-vet`). Clone the full Typio source tree, or ensure those repos are checked out as siblings on disk, otherwise `cargo` will fail to resolve dependencies.

## Build

### Development build

Compile with debug symbols and no optimisation for fast compile times:

```bash
cargo build
```

Artifact: `target/debug/libtypio_engine_basic.so`

### Production build

Compile an optimised release artifact:

```bash
cargo build --release
```

Artifact: `target/release/libtypio_engine_basic.so`

---

## Apply / Install

Packaged Typio hosts discover system-installed engines from
`<prefix>/<libdir>/typio/engines`. Development directories are explicit runtime
overrides through `--engine-dir` or `TYPIO_ENGINE_DIR`.

### Development (quick iteration)

Copy the debug `.so` into a development engine directory and point the host at
that directory:

```bash
mkdir -p build/engines
cp target/debug/libtypio_engine_basic.so \
    build/engines/
typio --engine-dir "$PWD/build/engines" --list
```

Copy the brand icon into your user icon directory:

```bash
mkdir -p ~/.local/share/icons/hicolor/symbolic/apps
cp data/icons/hicolor/symbolic/apps/typio-engine-basic-symbolic.svg \
    ~/.local/share/icons/hicolor/symbolic/apps/
```

### Production

Copy the release `.so` and icon to system directories:

```bash
# Engine library
mkdir -p /usr/local/lib/typio/engines
cp target/release/libtypio_engine_basic.so \
    /usr/local/lib/typio/engines/
chmod 755 /usr/local/lib/typio/engines/libtypio_engine_basic.so

# Brand icon
mkdir -p /usr/share/icons/hicolor/symbolic/apps
cp data/icons/hicolor/symbolic/apps/typio-engine-basic-symbolic.svg \
    /usr/share/icons/hicolor/symbolic/apps/
chmod 644 /usr/share/icons/hicolor/symbolic/apps/typio-engine-basic-symbolic.svg
```

Use the engine directory that matches the host package prefix. A host installed
with `--prefix=/usr` scans `/usr/lib/typio/engines`; a `/usr/local` host scans
`/usr/local/lib/typio/engines`.

---

## Optional: configure your editor

### rust-analyzer

Most editors with LSP support work out of the box. Ensure `rust-analyzer` is installed and detects the `Cargo.toml` in the project root.

### Clippy

The project uses `typio-vet` to vet the engine against the ABI contract. Run Clippy manually:

```bash
cargo clippy
```

## Next steps

- [Building and testing](building-and-testing.md) — compile the `.so` and run the test suite.
- [Documentation style guide](documentation-style-guide.md) — conventions for writing docs.
