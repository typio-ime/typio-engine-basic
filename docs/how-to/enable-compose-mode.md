# How to Enable Compose Mode

By default `typio-engine-basic` commits keys directly and does not perform composition. This guide shows how to turn on two-key compose sequences (for example `'` + `a` → `á`).

## Set the configuration key

Add the following to your Typio host configuration:

```ini
[engines.basic]
compose = true
```

The exact file and format depend on the host. Common locations:

| Host | Typical config path |
|------|---------------------|
| `libtypio` default | `~/.config/typio/typio.conf` |
| System-wide | `/etc/typio/typio.conf` |

## Restart or reload the engine

After saving the config, either:

- Restart the Typio host / input-method daemon, or
- Trigger a config reload if the host supports live reloading.

## Verify

1. Focus a text field.
2. Press `'` (apostrophe) once. You should see an underlined `'` in the pre-edit area.
3. Press `a`. The pre-edit disappears and `á` is committed.

If nothing happens, check the host logs for config-parsing errors and confirm the engine was re-initialised after the config change.

## Disable

Set `compose = false` (or remove the key) and reload:

```ini
[engines.basic]
compose = false
```

## See also

- [Compose sequences reference](../reference/compose-sequences.md) — full table of supported combinations.
- [How compose works](../explanation/compose-state-machine.md) — explanation of the pre-edit and commit flow.
