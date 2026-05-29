# Why `cdylib`

The `Cargo.toml` declares:

```toml
[lib]
crate-type = ["cdylib"]
```

This means the crate compiles to a C-compatible dynamic library (`.so` on Linux) rather than a Rust `rlib` or a static archive.

## The host loads plugins at runtime

The Typio framework discovers engine plugins by scanning a directory for `.so` files. It uses `dlopen`/`dlsym` to:

1. Load the library.
2. Resolve `typio_engine_abi_version` to check compatibility.
3. Resolve `typio_engine_get_info` to read metadata.
4. Resolve `typio_keyboard_engine_create` to instantiate the engine.

A static library (`staticlib`) cannot be loaded this way — it must be linked at build time of the host, which would force every engine into the host binary and eliminate the plugin architecture.

## `cdylib` vs `dylib`

| Crate type | Use case |
|------------|----------|
| `cdylib` | C-compatible ABI; only exported `#[no_mangle]` symbols are visible; minimal symbol table. |
| `dylib` | Rust-native dynamic library; exposes Rust metadata and name-mangled symbols. |

Because the Typio host is written in C and expects plain C symbols, `cdylib` is the correct choice. It also strips unneeded Rust metadata, producing a smaller binary.

## One restriction

`cdylib` crates cannot be directly depended on by other Rust crates via `extern crate` or `use`. If you need to share Rust code between engines, extract it into a standard `rlib` crate and depend on that from the `cdylib` crate.
