# Changelog

All notable changes to entra-access-graph-engine will be documented here.
Format based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

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
