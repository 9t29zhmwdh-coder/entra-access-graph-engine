# Changelog

All notable changes to entra-access-graph-engine will be documented here.
Format based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [1.0.0] - 2026-07-17

First stable release: a real release pipeline now builds and attaches
`eagraph` binaries for Linux, macOS, and Windows to every GitHub
Release, the prerequisite for a 1.0 release per this portfolio's own
SemVer discipline.

### Added
- Release workflow (`release.yml`) that cross-compiles `eagraph` for Linux/macOS/Windows on every `v*` tag push and attaches the binaries to a GitHub Release. Previously there was no prebuilt binary; users had to build from source.

## [0.1.8] - 2026-07-17

### Changed
- CI: added an explicit `permissions: contents: read` block to the workflow(s) that were missing one (CodeQL `actions/missing-workflow-permissions`), narrowing the default GITHUB_TOKEN scope.

## [0.1.7] - 2026-07-12

### Fixed

- Removed em-dashes and en-dashes across the repo (GETTING_STARTED.md, `.github/workflows/weekly-scan.yml`, `crates/eagraph-core/src/chain_detector.rs`). Swiss German orthography rule.

## [0.1.6] - 2026-07-12

### Added

- Dual-Licensing skeleton: LICENSE.COMMERCIAL, COMMERCIAL.md, and ENTERPRISE_FEATURES.md, documenting the licensing model for a future Enterprise Edition ahead of any actual feature split. The existing MIT LICENSE and all currently released code are unchanged; nothing in this repository is restricted by this addition.

## [0.1.5] - 2026-07-11

### Added

- Documented Dual-Licensing readiness assessment in ROADMAP.md.

## [0.1.4] - 2026-07-11

### Fixed

- Updated actions/checkout and actions/upload-artifact to their latest major versions in CI, since GitHub is deprecating the Node.js 20 runtime and older action versions were being forced onto Node 24 and crashing during post-run cleanup.

## [0.1.3] - 2026-07-10

### Fixed

- Changed the language-switch link from a blockquote to plain text
