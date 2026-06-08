# Typio ABI Integration

`typio-engine-basic` no longer links the Rust `typio-abi` crate or exports a
C ABI plugin. The historical ADR that introduced `typio-abi` remains in the
record for the old plugin implementation. The current engine is a native engine
executable declared by `typio-engine-basic.toml`.

See [Engine manifest](../reference/engine-manifest.md) for the current
contract.
