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

## The one exception: `TypioAbiVersion`

At the time of writing, `TypioAbiVersion` is defined locally in `src/lib.rs`:

```rust
#[repr(C)]
pub struct TypioAbiVersion {
    pub major: u16,
    pub minor: u16,
}
```

This is a temporary mirror. Once `typio-abi` exports this type, the local definition will be removed and the engine will import it directly.

## For engine authors

If you are writing a new Typio engine in Rust, depend on `typio-abi` rather than replicating types. See [ADR-0001: Use typio-abi crate instead of hand-replicated types](../adr/0001-use-typio-abi-crate.md) for the original decision record.
