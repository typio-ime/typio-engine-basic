# ADR-0002: Static compose table for two-key sequences

## Status

Accepted

## Context

`typio-engine-basic` needs to support accented characters via compose sequences. A full compose system (like XCompose) supports multi-key sequences, user-defined rules, and locale-specific tables loaded from files.

## Decision

`typio-engine-basic` will use a hard-coded static array (`COMPOSE_RULES`) containing only two-key sequences. There will be no external file loading, no runtime allocation for rule storage, and no multi-key nesting.

## Consequences

- **Positive**: Zero runtime dependencies; no file I/O; no allocation; predictable binary size.
- **Positive**: Simple to test — the entire rule set is visible in source code.
- **Negative**: Users cannot customise rules without recompiling.
- **Negative**: Only a small, curated set of common accents and punctuation is supported.

If a more capable compose system is needed in the future, it should be implemented as a separate engine plugin rather than growing `typio-engine-basic` beyond its fallback/reference role.

## Related

- [`docs/explanation/compose-state-machine.md`](../explanation/compose-state-machine.md)
- [`docs/reference/compose-sequences.md`](../reference/compose-sequences.md)
