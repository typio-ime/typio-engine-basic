# typio-abi Integration

All C ABI types used by `typio-engine-basic` come from the `typio-abi` crate:

```rust
use typio_abi::*;
```

This includes `#[repr(C)]` structs, enums, constants, and vtable definitions such as `TypioEngineInfo`, `TypioKeyboardEngineOps`, and `TypioKeyEvent`.

## The alternative: hand-replicated types

An engine plugin *could* copy the C header definitions into its own source:

```rust
#[repr(C)]
pub struct TypioKeyEvent {
    pub struct_size: usize,
    pub type_: u32,
    // ...
}
```

This was common in early Typio experiments, but it causes problems:

- **Drift** — when the host adds a field or changes an enum discriminant, every hand-replicated engine silently breaks.
- **Duplication** — every engine repeats the same boilerplate.
- **Review burden** — reviewers must verify that the Rust structs exactly match the C headers.

## The shared-crate approach

`typio-abi` is a small Rust crate that lives inside the `libtypio` source tree. It is the single source of truth for ABI types.

Benefits:

- **One update** — when the ABI changes, only `typio-abi` is modified.
- **Compile-time checking** — Rust engines that depend on `typio-abi` fail to compile if they use outdated field names or types.
- **IDE support** — autocomplete and go-to-definition work across the project boundary.

## ABI version

`TypioAbiVersion` and the current `TYPIO_ENGINE_ABI_MAJOR` /
`TYPIO_ENGINE_ABI_MINOR` constants come from `typio-abi`:

```rust
static TYPIO_ENGINE_ABI_VERSION_STATIC: TypioAbiVersion = TypioAbiVersion {
    major: TYPIO_ENGINE_ABI_MAJOR,
    minor: TYPIO_ENGINE_ABI_MINOR,
};
```

The plugin exports `typio_engine_abi_version()` by returning a pointer to this
static value. Do not mirror the struct locally; field width or version changes
must stay centralized in `typio-abi`.

## For engine authors

If you are writing a new Typio engine in Rust, depend on `typio-abi` rather than replicating types. See [ADR-0001: Use typio-abi crate instead of hand-replicated types](../adr/0001-use-typio-abi-crate.md) for the original decision record.
