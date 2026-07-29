# Changelog

All notable changes to entra-access-graph-engine will be documented here.
Format based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [1.0.6] - 2026-07-29

### Security

- The release workflow no longer grants `contents: write` for its whole run. The permission moves to the one job that publishes the release, and everything else runs with `contents: read`. OpenSSF Scorecard scores the Token-Permissions check 0 out of 10 whenever any workflow holds a top-level write permission, regardless of how little of the run needs it, so this single line was what held the check at zero.

---

## [1.0.5] - 2026-07-29

### Changed

- `reqwest` updated from 0.12 to 0.13. The `rustls-tls` feature no longer exists in 0.13 and is replaced by `rustls`, so the automated dependency update could not build: it can raise a version number but not rename a feature.
- The `form` feature is enabled explicitly. In 0.13 `RequestBuilder::form` sits behind its own feature; in 0.12 the method was always available, so the build failed only after the version bump.

### Security

- TLS now trusts the operating system's certificate store rather than a bundled root set. The `rustls` feature in 0.13 pulls in `rustls-platform-verifier`, where 0.12 resolved roots independently of the host. A machine that trusts an internal certificate authority, which is the normal case behind a corporate proxy, now works without extra configuration. The other side of that is real and worth naming: the trust decision moves to the machine the tool runs on, so a tampered local certificate store is enough to intercept the connection.
- The rustls crypto provider changes from `ring` to `aws-lc-rs`, which is what the `rustls` feature selects in 0.13.

---

## [1.0.4] - 2026-07-29

### Changed

Dependency and workflow updates merged since 1.0.3:

- chore(ci): bump the actions group across 1 directory with 2 updates

---

## [1.0.3] - 2026-07-29

### Changed

- CodeQL moved from GitHub's default setup to an advanced setup with a committed `.github/workflows/codeql.yml`. The default setup decides on its own when to run and skips pull requests that touch no code of a given language, so a dependency pull request changing only `Cargo.lock` reported `skipping` on the required `Analyze` checks and could never be merged. The workflow runs on every pull request regardless of what changed and uses the `security-extended` query suite, which the default setup does not allow choosing. Required checks are unchanged.
- The Cargo group in `.github/dependabot.yml` is limited to `minor` and `patch` updates. Without that limit a major bump lands inside a grouped pull request that reads as routine, which is how a breaking change slips in unreviewed.

---

## [1.0.2] - 2026-07-28

### Added

- `.github/dependabot.yml`, covering GitHub Actions and Cargo with grouped weekly updates. The file was missing, and without it there are no version updates at all: repository security alerts only fire for disclosed vulnerabilities. Follows `engineering-standards` v0.10.0.

### Fixed

- 10 action references were pinned to a mutable tag or branch rather than a commit SHA, `dtolnay/rust-toolchain@stable` among them. A branch HEAD can be moved to point at different code at any time without the workflow file changing, which is exactly what `standards/ci-cd.md` section 2 exists to prevent. All are now pinned to SHAs with the version in the comment. Pinned at their current versions, not upgraded: a major bump belongs in its own reviewed PR, and Dependabot will now propose one.
- `actions/checkout` pins were inconsistent across workflows. All now use v7.0.1 with the full version in the comment, per `standards/ci-cd.md` section 2.

## [1.0.1] - 2026-07-20

### Changed

- OpenSSF Scorecard workflow and badge.
- `copilot-instructions.md` for consistent AI-assisted contributions.
- Unified the EN/DE language-switch link format and restored missing sections in the German README.
- Split the README's security/CI badges onto their own line, separate from the platform/tech/AI badges (they were rendering as a single merged line).

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
