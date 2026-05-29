# Building and Testing

## Prerequisites

- [Rust](https://rustup.rs/) toolchain (stable channel, edition 2021)
- A Unix-like environment (Linux) for building the `.so`
- The `typio-abi` crate, expected at `../libtypio/crates/abi`

## Build

```bash
cargo build --release
```

The artifact is `target/release/libtypio_engine_basic.so`.

**Note:** Cargo does not install the library or its data files into system directories. It only produces the build artifact under `target/`. Installation is a separate step performed by the packager or system administrator.

## Install into a Typio host

Copy or symlink the `.so` into the host’s engine directory:

```bash
mkdir -p /usr/local/lib/typio/engines
cp target/release/libtypio_engine_basic.so /usr/local/lib/typio/engines/
```

Also install the symbolic icon so the host can resolve the engine’s brand icon:

```bash
install -Dm644 data/icons/hicolor/symbolic/apps/typio-engine-basic-symbolic.svg \
    /usr/share/icons/hicolor/symbolic/apps/typio-engine-basic-symbolic.svg
```

The exact paths depend on the host’s `TYPIO_ENGINE_PATH` or default search path.

## Run tests

### Unit tests only

The compose state machine and helper functions have unit tests in the `tests` module:

```bash
cargo test tests
```

### Full test suite

The `harness_tests` module exercises the C ABI surface through the `typio-engine-test` mock harness. Ensure the harness crate is available at `../typio-engine-test` and linked as a dev-dependency, then run:

```bash
cargo test
```

## Development workflow

1. Make changes in `src/lib.rs`.
2. Run `cargo test` (or `cargo test tests` if the harness is unavailable).
3. Update relevant docs under `docs/` if user-visible behaviour changed.
4. Update `CHANGELOG.md` under the "Unreleased" heading.
