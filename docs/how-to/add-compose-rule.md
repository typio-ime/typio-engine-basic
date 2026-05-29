# How to Add a Compose Rule

The compose table in `typio-engine-basic` is a static Rust slice. Adding a new two-key sequence requires editing `src/lib.rs` and rebuilding.

## Prerequisites

- You have built the engine from source (see [Getting started](../tutorials/01-getting-started.md)).
- You know the Unicode codepoint of the character you want to produce.

## Step 1 — Locate the table

Open `src/lib.rs` and find `COMPOSE_RULES`:

```rust
const COMPOSE_RULES: &[(u32, u32, u32)] = &[
    // Acute accent (')
    (b'\'' as u32, b'A' as u32, 0x00C1),
    ...
];
```

Each tuple is `(first_key, second_key, result_codepoint)`.

## Step 2 — Add your rule

Choose the appropriate comment group (or create one) and insert a new line. For example, to add `'` + `y` → `ý` (if it were missing):

```rust
    (b'\'' as u32, b'y' as u32, 0x00FD),
```

Rules are searched linearly, so order does not affect correctness, but keep related accents grouped for readability.

## Step 3 — Add a unit test

In the `tests` module at the bottom of `src/lib.rs`, add a test that exercises the new rule:

```rust
#[test]
fn compose_my_new_rule() {
    let mut compose = BasicCompose::new();
    compose.process_key(b'\'' as u32);
    let result = compose.process_key(b'y' as u32);
    assert!(matches!(result, ComposeResult::Commit(0x00FD)));
}
```

## Step 4 — Build and test

```bash
cargo test tests
cargo build --release
```

## Step 5 — Update the reference docs

Add the new sequence to [`docs/reference/compose-sequences.md`](../reference/compose-sequences.md) so users can discover it.

## Constraints

- Only two-key sequences are supported by this engine.
- Both keys must be single Unicode codepoints.
- The result must fit in a single Unicode scalar value (this engine does not commit multi-codepoint strings from compose rules).

## See also

- [Compose sequences reference](../reference/compose-sequences.md)
- [How compose works](../explanation/compose-state-machine.md)
