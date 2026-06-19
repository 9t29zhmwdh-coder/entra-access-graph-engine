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
- [ ] SIEM export: Microsoft Sentinel (KQL-ready), Splunk CIM
- [ ] PIM (Privileged Identity Management) eligible assignment analysis
- [ ] Comparison mode: diff two snapshots to detect new escalation paths

---

**Author:** [Rafael Yilmaz](https://github.com/9t29zhmwdh-coder) · **Status:** v0.1.0 · **Last Updated:** June 2026
