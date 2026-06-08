# How to Package the Engine for Distribution

## Build a Release Worker

```bash
cargo build --release
```

## Install the Worker

```bash
install -Dm755 target/release/typio-engine-basic \
  <prefix>/<libexecdir>/typio/engines/typio-engine-basic
```

## Install the Manifest

```bash
install -Dm644 /dev/stdin \
  <prefix>/<datadir>/typio/engines/typio-engine-basic.toml <<'EOF'
name = "basic"
type = "keyboard"
display_name = "Basic"
description = "Basic keyboard engine with Shift+Alt compose picker."
author = "Typio"
icon = "typio-engine-basic"
language = "und"
command = "<prefix>/<libexecdir>/typio/engines/typio-engine-basic"
args = []
required = ["preedit", "candidates"]
optional = []
EOF
```

## Install the Icon

```bash
install -Dm644 data/icons/hicolor/symbolic/apps/typio-engine-basic-symbolic.svg \
  <prefix>/share/icons/hicolor/symbolic/apps/typio-engine-basic-symbolic.svg
```

## Installed Layout

```text
<prefix>/
├── <libexecdir>/
│   └── typio/
│       └── engines/
│           └── typio-engine-basic
└── share/
    ├── typio/
    │   └── engines/
    │       └── typio-engine-basic.toml
    └── icons/
        └── hicolor/
            └── symbolic/
                └── apps/
                    └── typio-engine-basic-symbolic.svg
```

## Verification

```bash
<prefix>/<libexecdir>/typio/engines/typio-engine-basic <<'EOF'
availability
shutdown
EOF
```

```text
AVAILABILITY	READY
END
```

## See Also

- [Engine manifest](../reference/engine-manifest.md)
