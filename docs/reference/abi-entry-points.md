# ABI Entry Points

`typio-engine-basic` exports three C symbols. The Typio host loads the `.so` dynamically and resolves these symbols at runtime.

## `typio_engine_abi_version`

```c
const TypioAbiVersion *typio_engine_abi_version(void);
```

Returns a pointer to the ABI version this engine was built against. The host uses this to reject incompatible plugins before initialising them.

| Field | Value | Meaning |
|-------|-------|---------|
| `major` | `0` | Breaking-change compatibility boundary. |
| `minor` | `1` | Backward-compatible feature level. |

## `typio_engine_get_info`

```c
const TypioEngineInfo *typio_engine_get_info(void);
```

Returns static metadata about the engine. The host calls this to populate engine-selection UI.

| Field | Value |
|-------|-------|
| `name` | `basic` |
| `display_name` | `Basic` |
| `description` | `Built-in basic keyboard engine that commits printable text directly.` |
| `author` | `Typio` |
| `icon` | `typio-engine-basic` |
| `language` | `und` (undetermined) |
| `type_` | `TypioEngineTypeKeyboard` |

## `typio_keyboard_engine_create`

```c
TypioKeyboardEngine *typio_keyboard_engine_create(void);
```

Allocates and returns a new engine instance. The host must call `base_ops->init` before use and `base_ops->destroy` before freeing the pointer.

On allocation failure returns `NULL`.

The returned struct has the following vtables wired in:

| Vtable | Purpose |
|--------|---------|
| `base.base_ops` | Lifecycle: init, destroy, focus, reset, reload config. |
| `keyboard` | Key processing: `process_key`, `get_mode`, `set_mode`. |

## See also

- [How to package the engine for distribution](../how-to/package-for-distribution.md)
