# Engine Icons

`typio-engine-basic` provides a single symbolic icon asset that **must be installed separately** by the packager or system administrator. Cargo does not copy data files; the icon is shipped as a source asset only.

## Icon name

The engine reports:

```
typio-engine-basic
```

via `TypioEngineInfo.icon`. The host resolves this name against the current icon theme.

## Source asset

| File | Path | Purpose |
|------|------|---------|
| Symbolic SVG | [`data/icons/hicolor/symbolic/apps/typio-engine-basic-symbolic.svg`](../../data/icons/hicolor/symbolic/apps/typio-engine-basic-symbolic.svg) | Adaptive monochrome icon for symbolic/icon-only themes. |

## Installation paths

### System-wide (packaged)

```text
/usr/share/icons/hicolor/
└── symbolic/apps/
    └── typio-engine-basic-symbolic.svg
```

### Bundled (next to the `.so`)

```text
lib/typio/engines/
├── libtypio_engine_basic.so
└── icons/
    └── hicolor/
        └── symbolic/
            └── apps/
                └── typio-engine-basic-symbolic.svg
```

The host scans `<engine-dir>/icons/` and exposes it through the tray’s `IconThemePath`, so panels can resolve the symbolic icon automatically.

## See also

- [`libtypio` Engine Icon Reference](https://github.com/typio/libtypio/blob/main/docs/reference/engine-icons.md)
