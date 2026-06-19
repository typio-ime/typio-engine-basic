# ADR-0001: Use typio-abi crate instead of hand-replicated types

## Status

Superseded — the engine no longer exports a C ABI plugin and therefore no
longer links `typio-abi`. The decision is preserved for the historical
record; see [Engine ABI Integration](../explanation/typio-abi-integration.md)
for the current contract.

## Context

Typio engine plugins were originally loaded dynamically by a C host. They
had to speak a stable C ABI. The earliest prototype engines copied
`#[repr(C)]` struct and enum definitions directly from the C headers into
their own Rust source.

## Decision

All Rust engine plugins shall import ABI types from the shared `typio-abi`
crate rather than replicating them by hand.

## Consequences

- **Positive**: One authoritative source for ABI types; compile-time breakage when the host ABI changes; less boilerplate in engine code.
- **Positive**: Easier to add new fields or variants — only `typio-abi` changes.
- **Negative**: Engines must be built in a source tree that includes `libtypio/crates/abi` or have `typio-abi` published to a registry.
- **Negative**: Rust engines cannot be built independently without access to the `typio-abi` crate.

## Supersession

`typio-engine-basic` was reworked into a native engine executable that
speaks the Typio Engine Protocol (fd-3 framed IPC) directly. It no longer
exports a C ABI vtable, so it has no reason to link `typio-abi`. The
"single source of truth for ABI types" benefit still applies to engines
that *do* ship a C ABI plugin (rime, mozc, sherpa, whisper) — those
continue to consume `typio-engine-abi.pc`.

## Related

- [`docs/explanation/typio-abi-integration.md`](../explanation/typio-abi-integration.md)

