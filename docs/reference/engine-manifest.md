# Engine Manifest

| Item | Value |
|------|-------|
| Manifest file | `typio-engine-basic.toml` |
| Protocol | `typio-engine-protocol` |
| Engine command | `./target/release/typio-engine-basic` |
| Engine name | `basic` |
| Engine type | `keyboard` |
| Required capabilities | `preedit`, `candidates` |
| Optional capabilities | None |

## Engine Requests

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
| `set-active-mode` | `OK`, optional `ACTIVE_MODE ...`, `END` |
| `process-key` | `RESULT ...`, optional `COMMIT` or `COMPOSITION`, optional `ACTIVE_MODE ...`, `END` |
| `commit-candidate` | `COMMIT ...`, `OK`, `END` |
| `shutdown` | Process exit |

## Mode lines

`MODE` and `ACTIVE_MODE` carry the same tab-separated fields (all text fields
hex-encoded):

```
<prefix>\t<id>\t<label>\t<display_label>\t<icon>\t<profile_id>\t<profile_label>\t<description>\t<is_active>\t<salience>
```

`is_active` is `0`/`1`. `salience` is the trailing on-focus auto-reveal hint —
`0` = quiet, `1` = notable — and is optional (omitted ⇒ quiet). An engine with an
internal mode that flips reports the change by appending an `ACTIVE_MODE` line to
the reply of the request that caused it (`process-key`, `set-active-mode`,
`reset`, `focus-in`); the framework de-duplicates and notifies the host. The
basic engine's single `compose` mode is static and quiet, so it reports mode
only on explicit `list-modes`/`get-active-mode`.
