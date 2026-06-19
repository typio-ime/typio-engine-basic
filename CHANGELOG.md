# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.1] - 2026-06-19

### Changed

- ADR-0001 ("Use typio-abi crate") marked **Superseded**. The engine no
  longer ships a C ABI plugin and therefore no longer links `typio-abi`;
  the historical decision is preserved with a supersession note pointing
  at the current contract in `docs/explanation/typio-abi-integration.md`.

## [0.3.0] - 2026-06-13

### Added

- Declare supported languages in the manifest (`languages = ["en"]`) for
  language-first switching (typio-linux ADR-0031). The legacy single
  `language` key is kept for older hosts.

### Changed

- Engine traffic now uses Typio Engine Protocol on the private engine fd instead
  of the previous stdin/stdout protocol. The manifest declares
  `protocol = "typio-engine-protocol"`.
- `MODE` / `ACTIVE_MODE` lines now carry a trailing `salience` field after
  `is_active`. The `compose` mode is static and Latin, so it reports `0`
  (quiet) and stays silent on incidental focus, per the framework default.

## [0.2.0] - 2026-06-06

### Changed

- **Converted the engine to a native engine executable.** The package now
  builds `typio-engine-basic` and ships `typio-engine-basic.toml`; it no
  longer exports a C ABI `cdylib`.
- **Adopted the private-worker package layout.** Install the worker under
  `<libexecdir>/typio/engines` and the manifest under
  `<datadir>/typio/engines`.

## [0.1.4] - 2026-06-04

### Fixed

- **Current Typio host compatibility.** `typio_engine_abi_version()` now reports
  the `TypioAbiVersion` type and version constants from `typio-abi` instead of
  a stale local mirror. Current hosts now read the plugin ABI as `0.2` and load
  `libtypio_engine_basic.so` successfully.
- Updated developer and packaging docs to use explicit development engine
  directories (`--engine-dir` / `TYPIO_ENGINE_DIR`) and the system
  `<prefix>/<libdir>/typio/engines` install location.

## [0.1.3] - 2026-06-02

### Fixed

- **Digit keys 0–9 now select candidates by index (ADR-0012).** The compose
  picker sets `TypioHostSelIndexPick` in `host_managed_selection` so the host
  intercepts digit keys when candidates are present. Keys 1–9 commit candidate
  at index 0–8; key 0 commits candidate at index 9. When no candidates exist,
  digits are still usable as compose buffer input (e.g. `^` + `1` → `¹`).
- Removed redundant `typio_input_context_clear` call in `commit_candidate`;
  `typio_input_context_commit` already clears preedit and candidates atomically.

## [0.1.2] - 2026-06-02

### Changed

- **Adopt `COMMIT_RAW` flag and exclude Space/Enter from preedit input.**
  The compose picker now sets `NAVIGATE | COMMIT | COMMIT_RAW` so that
  Space commits the selected candidate (host-managed) and Enter commits
  the raw preedit buffer (also host-managed). `picker_process_key` no
  longer swallows Space as a printable character, letting the host
  intercept it correctly.

## [0.1.1] - 2026-06-02

### Changed

- **Adopted granular `host_managed_selection` flags (ADR-0012).**
  The compose picker now sets
  `TYPIO_HOST_SEL_NAVIGATE | TYPIO_HOST_SEL_COMMIT` instead of the old
  blanket `true`. Digits and space are no longer intercepted by the host
  and can be used as compose trigger characters (e.g. `^1` → `¹`).
- Removed dead code: `select_up`, `select_down`, and the
  `picker_navigation` test that exercised engine-side navigation no longer
  needed because arrow keys are host-managed.

## [0.1.0] - 2026-06-02

### Changed

- Bumped version to 0.1.0.

## [0.0.3] - 2026-06-01

### Changed

- Adopted keyboard-domain ABI status names.
- Corrected engine installation path in setup docs.
- Removed redundant setup documentation sections.

## [0.0.2] - 2026-05-30

### Changed

- Renamed dev-dependency `typio-devlint` to `typio-vet`.
- Updated developer setup documentation.

## [0.0.1] - 2026-05-29

### Added

- Initial release of `typio-engine-basic`.
- Basic compose input engine built as a `cdylib` for the Typio platform.
- Static compose table for common compose sequences.
- Integration with `typio-abi` for engine ABI compatibility.
- Documentation: ADRs, developer guides, tutorials, and reference docs.
- Project icon and branding assets.
