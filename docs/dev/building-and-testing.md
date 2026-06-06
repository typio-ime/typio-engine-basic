# Building and Testing

## Build

```bash
cargo build --release
```

| Artifact | Path |
|----------|------|
| Worker executable | `target/release/typio-engine-basic` |
| Manifest | `typio-engine-basic.toml` |

## Test

```bash
cargo test
```

## Worker Smoke Test

```bash
target/release/typio-engine-basic <<'EOF'
availability
shutdown
EOF
```

```text
AVAILABILITY	READY
END
```

## Development Workflow

1. Make changes in `src/main.rs`.
2. Run `cargo test`.
3. Run `cargo build --release`.
4. Start the host with `typio --engine-dir "$PWD"`.
5. Update docs and `CHANGELOG.md` when behavior changes.
