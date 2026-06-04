# Getting Started with typio-engine-basic

In this tutorial you will build the `typio-engine-basic` keyboard engine from source, verify it with the test suite, and install it so a Typio host can load it.

## What you need

- A Linux development environment.
- The Rust toolchain installed via [rustup](https://rustup.rs/).
- The Typio source tree available locally, because this crate depends on `typio-abi` and optionally on `typio-engine-test`.

## Step 1 — Clone the Typio repository

If you have not already done so, obtain the full Typio tree. `typio-engine-basic` expects `libtypio` and `typio-engine-test` as sibling directories:

```bash
cd /path/to/typio-project
git clone <typio-repo-url> .
```

Your working tree should look like this:

```
typio-project/
├── libtypio/
│   └── crates/
│       └── abi/
├── typio-engine-basic/   <-- you are here
└── typio-engine-test/
```

## Step 2 — Build the engine

```bash
cd typio-engine-basic
cargo build --release
```

You will see output ending with:

```
Compiling typio-engine-basic v0.1.0 (...)
Finished `release` profile [...]
```

The compiled plugin is now at:

```
target/release/libtypio_engine_basic.so
```

## Step 3 — Run the tests

Run the unit-test suite to confirm the compose state machine behaves correctly:

```bash
cargo test tests
```

Expected output (truncated):

```
running 12 tests
test tests::compose_none_for_plain_key ... ok
test tests::compose_commit_sequence ... ok
...
```

If you also have the `typio-engine-test` harness set up, run the full suite:

```bash
cargo test
```

All tests should pass.

## Step 4 — Install the plugin

Cargo only compiles the engine; it does **not** install anything to system
directories. For this tutorial, copy the `.so` into an explicit development
engine directory:

```bash
mkdir -p build/engines
cp target/release/libtypio_engine_basic.so build/engines/
```

Also install the symbolic icon so the host can display the engine’s brand icon:

```bash
sudo install -Dm644 data/icons/hicolor/symbolic/apps/typio-engine-basic-symbolic.svg \
    /usr/share/icons/hicolor/symbolic/apps/typio-engine-basic-symbolic.svg
```

Packaged hosts discover system-installed engines from
`<prefix>/<libdir>/typio/engines`. Development directories are explicit runtime
overrides through `--engine-dir` or `TYPIO_ENGINE_DIR`.

## Step 5 — Verify the host sees the engine

Run the host against the development directory:

```bash
typio --engine-dir "$PWD/build/engines" --list
```

The output should include `basic` under available keyboard engines.

## What you have learned

- How to build `typio-engine-basic` as a release `.so`.
- How to run its test suite.
- Where to place the plugin so the Typio host discovers it.

## Next steps

- [Enable compose mode](../how-to/enable-compose-mode.md) to type accented characters.
- Browse the [compose sequences reference](../reference/compose-sequences.md) to see which two-key combinations are supported.
- Read [How compose works](../explanation/compose-state-machine.md) for a deeper understanding of the state machine.
