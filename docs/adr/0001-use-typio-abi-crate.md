# ADR-0001: Use typio-abi crate instead of hand-replicated types

## Status

Accepted

## Context

Typio engine plugins are loaded dynamically by a C host. They must speak a stable C ABI. The earliest prototype engines copied `#[repr(C)]` struct and enum definitions directly from the C headers into their own Rust source.

## Decision

All Rust engine plugins shall import ABI types from the shared `typio-abi` crate rather than replicating them by hand.

## Consequences

- **Positive**: One authoritative source for ABI types; compile-time breakage when the host ABI changes; less boilerplate in engine code.
- **Positive**: Easier to add new fields or variants — only `typio-abi` changes.
- **Negative**: Engines must be built in a source tree that includes `libtypio/crates/abi` or have `typio-abi` published to a registry.
- **Negative**: Rust engines cannot be built independently without access to the `typio-abi` crate.

## Related

- [`docs/explanation/typio-abi-integration.md`](../explanation/typio-abi-integration.md)
