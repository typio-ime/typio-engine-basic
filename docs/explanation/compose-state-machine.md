# Compose State Machine

`typio-engine-basic` implements a minimal two-key compose system. This page explains the state machine that drives it.

## States

The compose logic has two conceptual states, tracked by `BasicCompose.active`:

| State | `active` | Meaning |
|-------|----------|---------|
| Idle | `false` | Waiting for the first key of a compose sequence. |
| Pending | `true` | First key received; waiting for the second key. |

There is no deeper nesting — this engine supports exactly two-key sequences.

## Transitions

### Idle → Pending

When `compose_enabled` is true and a printable key arrives:

1. `BasicCompose::process_key` calls `can_start_compose` against the static `COMPOSE_RULES` table.
2. If the codepoint appears as the first element of any rule, the engine enters the Pending state, stores the codepoint in `first`, and returns `ComposeResult::Consume`.
3. The keyboard callback sees `Consume`, builds a `TypioComposition` containing the first key as pre-edit text, and calls `typio_input_context_set_composition`.

### Pending → Commit (success)

When a second key arrives while Pending:

1. `process_key` searches `COMPOSE_RULES` for a tuple `(first, second, result)`.
2. If a match is found, the engine returns to Idle and yields `ComposeResult::Commit(result)`.
3. The keyboard callback clears the pre-edit and commits the result codepoint.

### Pending → Commit (failure / no rule)

If the second key does not form a known sequence:

1. The engine yields `ComposeResult::Cancel(first)`.
2. The keyboard callback clears the pre-edit, commits the first key as a literal character, and then commits the second key as a literal character.

This behaviour ensures the user never loses input — even a "failed" compose commits both keystrokes.

### Pending → Idle (cancel)

If the user presses Escape while Pending:

1. The keyboard callback detects `typio_key_event_is_escape`.
2. It calls `compose.cancel()`, which returns `Some(first)`.
3. The pre-edit is cleared. No text is committed.

If a non-printable key (e.g. Backspace, Arrow) is pressed while Pending:

1. The engine cancels the compose, commits the first key as a literal, and returns `TypioKeyCommitted`.
2. The non-printable key is then handled by the host or another layer.

## Why a static table?

The `COMPOSE_RULES` array is a compile-time constant. This keeps the engine:

- **Zero-allocation** at runtime — no hash maps, no trees.
- **Self-contained** — no external dictionary files to ship or load.
- **Predictable** — lookup is a small linear scan over ~80 entries, which is faster than a hash map for this data size.

For a more sophisticated compose system (multi-key sequences, user-defined rules, locale-specific tables), a dedicated compose engine plugin would be more appropriate.
