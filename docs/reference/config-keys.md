# Configuration Keys

The engine reads the following keys from the host configuration at initialisation time.

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `engines.basic.compose` | `bool` | `false` | Enable two-key compose sequences for accented characters and special punctuation. |

## Notes

- The engine reads the config once during `basic_init`. Live config reload is not implemented; changing the value requires re-creating the engine instance (typically by restarting the host).
- Unknown keys are ignored.
- The engine does not expose any other tunable parameters.
