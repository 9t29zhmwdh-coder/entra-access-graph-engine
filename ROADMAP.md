<div align="center">
  <img src="RayStudio.png" alt="RayStudio Logo" width="120"/>

  <h1>Roadmap</h1>
</div>

> 🇩🇪 [Deutsche Version](ROADMAP.de.md)

## v0.1 (Current)

- [x] GraphClient with Microsoft Graph API (OAuth2 client credentials, automatic pagination)
- [x] NodeBuilder: Users, Groups, DirectoryRoles, Applications, ServicePrincipals
- [x] Edge types: MemberOf, AppRoleAssignment, RoleAssignment, OAuthPermissionGrant, ServicePrincipalOf
- [x] EdgeAnalyzer with petgraph DiGraph (directed)
- [x] PrivilegeChainDetector (BFS, max depth 6)
- [x] RiskScorer with 10 well-known Entra role template IDs and Graph API permission names
- [x] GraphExporter: JSON, GraphML, HTML + D3.js interactive force graph
- [x] Dry-run mode with built-in mock graph
- [x] CI on Ubuntu + Windows (cargo check, clippy -D warnings, cargo test)
- [x] Weekly GitHub Actions scan with artifact upload

## v0.2

- [ ] Conditional Access policy gap detection (roles without CA coverage)
- [ ] Guest account risk detection (external users in privileged groups)
- [ ] Managed Identity assignment chain detection
- [ ] SARIF output format for GitHub Advanced Security integration
- [ ] Configurable risk rule file (YAML/TOML)

## v0.3

- [ ] Multi-tenant support (scan multiple tenants in one run)
- [ ] Azure Monitor / Application Insights OTLP export for risk metrics
- [ ] Microsoft Defender for Identity integration (alert correlation)
- [ ] Automated remediation suggestions (least-privilege recommendations)

## Future

- [ ] Live graph viewer (Tauri desktop app with real-time Graph API polling)
- [ ] PIM (Privileged Identity Management) eligible assignment analysis
- [ ] Comparison mode: diff two snapshots to detect new escalation paths

## Dual-Licensing Readiness

Assessed 2026-07-11 as a Dual-Licensing candidate (Community MIT + Commercial/Enterprise tier): identity attack-path analysis for Entra ID/Active Directory is one of the most established commercial security categories (BloodHound Enterprise sells exactly this capability), and this project's own roadmap already lists several classic enterprise differentiators. Not ready yet; blocked on:

- [ ] No multi-tenant support yet (v0.3 item above): MSPs and consultancies scanning multiple customer tenants are a natural Commercial-tier audience
- [x] SIEM export shipped 2026-07-13, as an Enterprise-only feature in the private [entra-access-graph-engine-enterprise](https://github.com/9t29zhmwdh-coder/entra-access-graph-engine-enterprise) companion repository (Sentinel + Splunk), not in this Community repository, gated behind a signed license file
- [ ] No PIM analysis or automated remediation suggestions yet (v0.3/Future items above): still roadmap entries, not implemented
- [ ] No server or API component to gate a Commercial tier against: today this is a pure local CLI with no persistence layer

Once multi-tenant support (v0.3) lands, revisit: candidate Enterprise-only features would be multi-tenant scanning, PIM analysis, and automated remediation suggestions, alongside the SIEM export that has already shipped, with the core graph-building and chain-detection engine (Microsoft Graph client, node/edge model, BFS chain detector, risk scorer, CLI) staying Community/MIT.

---

**Author:** [Rafael Yilmaz](https://github.com/9t29zhmwdh-coder) · **Status:** v0.1.0 · **Last Updated:** June 2026
