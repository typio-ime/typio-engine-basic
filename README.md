# typio-engine-basic

The always-installable keyboard fallback for Typio. It commits printable
Unicode text directly and provides a Shift+Alt compose picker for accented
Latin characters.

## Role in the ecosystem

`typio-engine-basic` is a native Typio engine executable. The host discovers
`typio-engine-basic.toml`, starts the engine command declared by that manifest,
and exchanges Typio Engine Protocol frames over the private engine fd.

| What | Where |
|---|---|
| Engine implementation | [`src/main.rs`](src/main.rs) |
| Manifest | [`typio-engine-basic.toml`](typio-engine-basic.toml) |
| Host framework | [`libtypio`](../libtypio) |

## Build

```bash
cargo build --release
```

The output is `target/release/typio-engine-basic`.

For a development host run, point the host at the repository root:

```bash
typio --engine-dir "$PWD"
```

## Test

```bash
cargo test
```

Unit tests cover the compose state machine.

## Key design points

- **Worker executable** — runs out of process from the daemon.
- **Manifest discovery** — `typio-engine-basic.toml` declares metadata,
  capabilities, and worker argv.
- **Compose rules** — simple static table (`'` + `a` -> `á`, etc.).
- **No runtime dependencies** — no dictionaries, model files, or shared engine
  library.

## License

MIT — see [`LICENSE`](LICENSE).
