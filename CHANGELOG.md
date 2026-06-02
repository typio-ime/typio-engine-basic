# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
