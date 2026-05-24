# Changelog

## Unreleased

## [0.0.2](https://github.com/RustUse/use-stats/compare/use-stats-v0.0.1...use-stats-v0.0.2) - 2026-05-24

### Changed

- Add CI, devcontainer, tooling and workspace configs

### Added

- Added the initial `use-stats` multi-crate workspace scaffold.
- Added focused crates for averages, variance, standard deviation,
  percentiles, z-scores, correlation, and distribution helpers.
- Added the `use-stats` facade crate with a shared prelude and focused-crate
  reexports.

### Tooling

- Added `release-plz` configuration and publish-readiness workflows for the
  focused-first `use-stats` release flow.