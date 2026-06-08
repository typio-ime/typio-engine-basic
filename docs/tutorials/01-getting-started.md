# Getting Started with typio-engine-basic

In this tutorial you will build the `typio-engine-basic` keyboard engine from source, verify it with the test suite, and point a Typio host at its manifest.

## What you need

- A Linux development environment.
- The Rust toolchain installed via [rustup](https://rustup.rs/).
- A Typio host checkout or package for the final verification step.

## Step 1 — Clone the Typio repository

If you have not already done so, obtain the engine repository:

```bash
cd /path/to/typio-project
git clone <typio-engine-basic-repo-url> typio-engine-basic
```

Your working tree should look like this:

```
typio-engine-basic/
├── src/
├── docs/
└── typio-engine-basic.toml
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

The compiled worker is now at:

```
target/release/typio-engine-basic
```

## Step 3 — Run the tests

Run the unit-test suite to confirm the compose state machine behaves correctly:

```bash
cargo test
```

Expected output (truncated):

```
running 2 tests
test tests::picker_exact_sequence ... ok
test tests::picker_search_base_a ... ok
...
```

All tests should pass.

## Step 4 — Verify the Worker

Run a protocol smoke test:

```bash
target/release/typio-engine-basic <<'EOF'
availability
shutdown
EOF
```

Expected output:

```text
AVAILABILITY	READY
END
```

## Step 5 — Verify the host sees the engine

Run the host against the repository root, which contains the manifest:

```bash
typio --engine-dir "$PWD" --verbose
```

The output should include `basic` under available keyboard engines.

## What you have learned

- How to build `typio-engine-basic` as a release engine executable.
- How to run its test suite.
- Where to place the manifest so the Typio host discovers it.

## Next steps

- [Enable compose mode](../how-to/enable-compose-mode.md) to type accented characters.
- Browse the [compose sequences reference](../reference/compose-sequences.md) to see which two-key combinations are supported.
- Read [How compose works](../explanation/compose-state-machine.md) for a deeper understanding of the state machine.
