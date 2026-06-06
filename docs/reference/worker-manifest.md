# Worker Manifest

| Item | Value |
|------|-------|
| Manifest file | `typio-engine-basic.toml` |
| Worker command | `./target/release/typio-engine-basic` |
| Engine name | `basic` |
| Engine type | `keyboard` |
| Required capabilities | `preedit`, `candidates` |
| Optional capabilities | None |

## Worker Requests

| Request | Response |
|---------|----------|
| `init` | `OK`, `END` |
| `deactivate` | `OK`, `END` |
| `focus-in` | `OK`, `END` |
| `focus-out` | Optional `CLEAR`, `OK`, `END` |
| `reset` | Optional `CLEAR`, `OK`, `END` |
| `reload-config` | `OK`, `END` |
| `availability` | `AVAILABILITY\tREADY`, `END` |
| `list-modes` | `MODE ...`, `END` |
| `get-active-mode` | `ACTIVE_MODE ...`, `END` |
| `set-active-mode` | `OK`, `END` |
| `process-key` | `RESULT ...`, optional `COMMIT` or `COMPOSITION`, `END` |
| `commit-candidate` | `COMMIT ...`, `OK`, `END` |
| `shutdown` | Process exit |
