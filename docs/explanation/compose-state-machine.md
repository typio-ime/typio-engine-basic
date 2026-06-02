# Compose State Machine

`typio-engine-basic` implements a Shift+Alt compose picker. This page explains the state machine that drives it.

## States

The engine has two top-level states, determined by `ComposePicker.active`:

| State | `active` | Meaning |
|-------|----------|---------|
| Raw / Idle | `false` | All printable keys commit directly to the application. |
| Picker Active | `true` | Typed characters build a search buffer; matching candidates are displayed for selection. |

## Transitions

### Idle → Picker Active

Press **Shift+Alt** to toggle the picker on. The engine calls `picker.activate()`, clears the buffer and candidates, and pushes an empty composition to the host.

### Typing in the Picker

Each printable key appends a character to the buffer (max 2 characters) and triggers a search:

1. **One character** — `search_by_base(cp)` finds all rules where the character appears as either the first or second element, producing a broad candidate list.
2. **Two characters** — `search_exact(first, second)` finds rules matching the exact pair, narrowing to typically 0 or 1 candidate.

When candidates exist, the first candidate is auto-selected.

### Candidate Selection (Host-Managed)

The engine sets `host_managed_selection` flags on the composition to delegate selection to the host:

| Flag | Key | Host Action |
|------|-----|-------------|
| `Navigate` | Up/Down/Left/Right | Host updates the selected index and re-renders the panel. |
| `Commit` | Space | Host calls `commit_candidate` with the currently selected index. |
| `IndexPick` | 1–9, 0 | Host calls `commit_candidate` with index 0–8 (1–9) or index 9 (0). |
| `CommitRaw` | Enter / KP_Enter | Host commits the raw preedit buffer text as-is. |

The engine returns `TypioKeyNotHandled` for these keys so the host's candidate guard can intercept them.

Digit keys 0–9 are returned as `NotHandled` **only when candidates are present**. When no candidates exist, digits are appended to the compose buffer like any other printable character (e.g. `^` + `1` produces `¹`).

### Picker → Idle (cancel)

- **Escape**: deactivates the picker, clears the composition.
- **Shift+Alt** (toggle): deactivates the picker.
- **Backspace on empty buffer**: deactivates the picker.
- **Focus out**: triggers `reset`, which deactivates the picker.

### Committing a Candidate

When the host calls `commit_candidate(index)`:

1. The engine retrieves the result character from `picker.candidates[index]`.
2. Deactivates the picker and clears the text cache.
3. Calls `typio_input_context_commit` to commit the character. The host automatically clears the preedit and candidate panel.

## Why a static table?

The `COMPOSE_RULES` array is a compile-time constant. This keeps the engine:

- **Zero-allocation** at runtime — no hash maps, no trees.
- **Self-contained** — no external dictionary files to ship or load.
- **Predictable** — lookup is a small linear scan over ~117 entries, which is faster than a hash map for this data size.

For a more sophisticated compose system (multi-key sequences, user-defined rules, locale-specific tables), a dedicated compose engine plugin would be more appropriate.
