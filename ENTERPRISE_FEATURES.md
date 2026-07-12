# Enterprise Features

This document lists features planned for the Enterprise Edition of this
project, licensed separately under
[LICENSE.COMMERCIAL](LICENSE.COMMERCIAL). See [COMMERCIAL.md](COMMERCIAL.md)
for the licensing model.

## Status

No Enterprise features have shipped yet. This list is a forward-looking plan,
not a changelog of existing functionality: everything currently in this
repository is part of the Community Edition and remains MIT-licensed. See the
repository's own [ROADMAP.md](ROADMAP.md), "Dual-Licensing Readiness"
section, for the prerequisites that need to land first.

## Planned

- Multi-tenant scanning: analyzing multiple customer Entra ID tenants from a
  single install, for MSPs and consultancies.
- SIEM export (Sentinel/Splunk): turnkey integration of attack-path findings
  into an organization's existing security stack.
- PIM (Privileged Identity Management) analysis: surfacing eligible/active
  role assignments and time-bound privilege escalation paths.
- Automated remediation suggestions: concrete, actionable steps to close
  identified attack paths, beyond reporting them.

## Not planned

The core graph-building and chain-detection engine (Microsoft Graph client,
node/edge model, BFS chain detector, risk scorer, CLI) stays in the Community
Edition permanently. Dual-licensing governs only new, enterprise-shaped
capabilities such as the ones listed above, not the tool's standalone
usefulness for a single tenant.
