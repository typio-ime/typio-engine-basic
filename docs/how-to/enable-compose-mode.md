# How to Use Compose Mode

By default `typio-engine-basic` commits keys directly to the application. The compose picker provides access to extended Latin characters (e.g. `á`, `ñ`, `ü`) via two-key sequences.

## Activate the picker

Press **Shift+Alt** to toggle the compose picker on. The same shortcut toggles it off.

## Verify

1. Focus a text field.
2. Press **Shift+Alt**. An empty pre-edit area should appear.
3. Type `'` (apostrophe). A candidate list of accented characters appears (á, é, í, ó, ú, …).
4. Press a letter key (e.g. `a`) to narrow to a single candidate (`á`), then press **Space** to commit it. Alternatively, press a **number key** (1–9, 0) to pick a candidate by index directly from the list.

## Selection keys

| Key | Action |
|-----|--------|
| Up/Down/Left/Right | Navigate through candidates |
| Space | Commit the currently selected candidate |
| Enter | Commit the raw preedit buffer text as-is |
| 1–9 | Commit candidate at index 0–8 |
| 0 | Commit candidate at index 9 (10th item) |
| Escape | Cancel and close the picker |
| Backspace | Remove last character from buffer; close picker if empty |

## See also

- [Compose sequences reference](../reference/compose-sequences.md) — full table of supported combinations.
- [How compose works](../explanation/compose-state-machine.md) — explanation of the state machine and host-managed selection.
