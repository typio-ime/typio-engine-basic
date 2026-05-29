# typio-engine-basic

The always-installable keyboard fallback for the Typio input method
framework. Commits printable Unicode text directly, with optional
two-key compose sequences for accented characters.

## Role in the ecosystem

`typio-engine-basic` is a **reference engine plugin**. It demonstrates the
correct way to build a Rust engine against the shared `typio-abi` crate
rather than replicating C ABI types by hand.

```
┌─────────────────┐      use typio_abi::*;       ┌──────────────────┐
│   libtypio      │ ◄──────────────────────────── │  typio-engine-   │
│  (host/runtime) │   loads .so at runtime        │     basic        │
└─────────────────┘                               └──────────────────┘
         ▲                                                 │
         │                                                 │
         └─────────────────────────────────────────────────┘
              depends on typio-abi for shared types
```

| What | Where |
|---|---|
| Engine implementation | [`src/lib.rs`](src/lib.rs) |
| ABI types (shared) | [`typio-abi`](../libtypio/crates/abi) |
| Test harness | [`typio-engine-test`](../typio-engine-test) |
| Host framework | [`libtypio`](../libtypio) |

## Build

```bash
cargo build --release
```

The output is `target/release/libtypio_engine_basic.so`, which the Typio
host discovers under `<libdir>/typio/engines/`.

## Test

```bash
cargo test
```

Unit tests cover the compose state machine. Integration tests (in the
`harness_tests` module) exercise the full C ABI surface through the shared
[`typio-engine-test`](../typio-engine-test) mock harness.

## Key design points

- **Zero hand-replicated ABI types** — all `#[repr(C)]` structs, enums,
  and constants come from `use typio_abi::*;`.
- **`cdylib`** — compiled as a loadable plugin, not an executable or
  static library.
- **Compose rules** — simple static table (`'` + `a` → `á`, etc.). No
  external dictionaries.
- **Config-driven** — reads `engines.basic.compose` from the host config
  to enable/disable compose mode.

## License

MIT — see [`../libtypio/LICENSE`](../libtypio/LICENSE).
