# How to Package the Engine for Distribution

This guide covers building a release-quality `libtypio_engine_basic.so` and verifying that it remains compatible with the Typio host ABI.

## Build a clean release artifact

```bash
cargo clean
cargo build --release
```

The resulting file is:

```
target/release/libtypio_engine_basic.so
```

## Strip debug symbols (optional)

To reduce binary size for distribution:

```bash
strip target/release/libtypio_engine_basic.so
```

Keep an unstripped copy if you need debug symbols later.

## Verify ABI version

`typio-engine-basic` exports:

```c
const TypioAbiVersion *typio_engine_abi_version(void);
```

The returned struct must match the host’s expected ABI version. Check the values at runtime or inspect the source:

```rust
pub const TYPIO_ENGINE_ABI_MAJOR: u16 = 0;
pub const TYPIO_ENGINE_ABI_MINOR: u16 = 1;
```

If the host expects a different version, the engine will be rejected at load time. Coordinate with the host maintainers before bumping the ABI version in this crate.

## Verify exported symbols

```bash
nm -D target/release/libtypio_engine_basic.so | grep -E 'typio_engine|typio_keyboard'
```

You should see at minimum:

```
typio_engine_abi_version
typio_engine_get_info
typio_keyboard_engine_create
```

Missing symbols indicate a build or linking problem.

## What Cargo does (and does not)

`cargo build --release` produces exactly one file:

```
target/release/libtypio_engine_basic.so
```

It **does not** install the `.so` into system directories, and it **does not** copy data files such as icons. Installation of the library and its assets is the responsibility of the packager, the system administrator, or a distribution package.

## Distribution package layout

A complete installation consists of two parts: the shared library and the icon asset.

### Engine library

```
/usr/local/lib/typio/engines/
└── libtypio_engine_basic.so
```

### Icon asset

Install the symbolic icon so the host can resolve `typio-engine-basic` through the icon theme:

```bash
sudo install -Dm644 data/icons/hicolor/symbolic/apps/typio-engine-basic-symbolic.svg \
    /usr/share/icons/hicolor/symbolic/apps/typio-engine-basic-symbolic.svg
```

Alternatively, bundle it next to the `.so` for portable installs:

```
/usr/local/lib/typio/engines/
├── libtypio_engine_basic.so
└── icons/
    └── hicolor/
        └── symbolic/
            └── apps/
                └── typio-engine-basic-symbolic.svg
```

The host scans `<engine-dir>/icons/` and exposes it through the tray’s `IconThemePath`, so panels can resolve the symbolic icon automatically.

No additional run-time dependencies are required.

## See also

- [ABI entry points reference](../reference/abi-entry-points.md)
- [ADR-0001: Use typio-abi crate instead of hand-replicated types](../adr/0001-use-typio-abi-crate.md)
