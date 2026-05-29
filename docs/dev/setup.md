# Development Setup

How to prepare your machine to build, test, and modify `typio-engine-basic`.

## Prerequisites

| Tool | Minimum version | Purpose |
|------|-----------------|---------|
| Rust | stable (edition 2021) | Compiles the engine. |
| cargo | bundled with Rust | Dependency resolution and build orchestration. |
| gcc / clang | any recent | C compiler for linking the `cdylib`. |
| strip | binutils | Optional — for removing debug symbols from release builds. |
| nm | binutils | Optional — for inspecting exported symbols. |

## Repository layout

`typio-engine-basic` depends on crates that live outside its own directory. Clone the full Typio source tree so the relative paths in `Cargo.toml` resolve correctly:

```
typio-project/
├── libtypio/
│   └── crates/
│       └── abi/              <-- typio-abi dependency
├── typio-devlint/            <-- dev-dependency (linting)
├── typio-engine-test/        <-- dev-dependency (test harness)
└── typio-engine-basic/       <-- this crate
```

If your organisation keeps these in separate repositories, ensure they are checked out as siblings on disk.

## Install Rust

If you do not have Rust installed:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

Verify:

```bash
rustc --version   # e.g. rustc 1.80.0
cargo --version   # e.g. cargo 1.80.0
```

## Clone and enter the repository

```bash
cd /path/to/typio-project
cd typio-engine-basic
```

## Verify dependencies resolve

```bash
cargo check
```

If this succeeds, the relative paths to `typio-abi` and other workspace crates are correct.

## Optional: configure your editor

### rust-analyzer

Most editors with LSP support work out of the box. Ensure `rust-analyzer` is installed and detects the `Cargo.toml` in the project root.

### Clippy

The project uses `typio-devlint` for consistent linting. Run Clippy manually:

```bash
cargo clippy
```

## Next steps

- [Building and testing](building-and-testing.md) — compile the `.so` and run the test suite.
- [Documentation style guide](documentation-style-guide.md) — conventions for writing docs.
