<div align="center">
  <img src="RayStudio.png" alt="RayStudio Logo" width="120"/>

  <h1>entra-access-graph-engine</h1>
</div>

[🇬🇧 English Version](README.md)

**Alle Entra-ID-Objekte in einen Zugriffsgraphen umwandeln. Eskalationspfade, versteckte Admin-Chains und Risikobewertungen erkennen. Rust, Offline-first, OTLP-ready.**

Ruft Benutzer, Gruppen, Rollen, Applikationen, Service Principals, AppRoleAssignments und DirectoryRoles über die Microsoft Graph API ab und erstellt daraus einen gerichteten Zugriffsgraphen. Die Engine erkennt Privilege-Escalation-Pfade, versteckte Admin-Chains (App → SP → Gruppe → GlobalAdmin) und klassifiziert jeden Knoten und jeden Pfad nach Risiko (Low / Medium / High / Critical). Export als JSON, GraphML oder selbstenthaltenem HTML-Report mit interaktivem D3.js-Graphen.

[![CI](https://github.com/9t29zhmwdh-coder/entra-access-graph-engine/actions/workflows/ci.yml/badge.svg)](https://github.com/9t29zhmwdh-coder/entra-access-graph-engine/actions) [![CodeQL](https://github.com/9t29zhmwdh-coder/entra-access-graph-engine/actions/workflows/github-code-scanning/codeql/badge.svg)](https://github.com/9t29zhmwdh-coder/entra-access-graph-engine/security/code-scanning) [![OpenSSF Scorecard](https://api.securityscorecards.dev/projects/github.com/9t29zhmwdh-coder/entra-access-graph-engine/badge)](https://securityscorecards.dev/viewer/?uri=github.com/9t29zhmwdh-coder/entra-access-graph-engine)
![Microsoft | Entra ID](https://img.shields.io/badge/Microsoft-Entra_ID-0078d4?logo=microsoftazure&logoColor=white) ![Platform](https://img.shields.io/badge/Platform-Windows_%7C_Ubuntu-lightgrey) ![Rust](https://img.shields.io/badge/Rust-CE422B?logo=rust&logoColor=white) ![AI | Claude Code](https://img.shields.io/badge/AI-Claude_Code-black?logo=anthropic&logoColor=white) ![AI | Copilot](https://img.shields.io/badge/AI-Copilot-black?logo=github&logoColor=white)

> **So läuft das:** Dieses Tool ist ein Kommandozeilenprogramm, keine Desktop-App und kein Server. `eagraph scan` läuft einmal durch und schreibt einen Report (JSON/GraphML/HTML), es gibt keinen Installer und keinen Hintergrundprozess.

![entra-access-graph-engine](docs/screenshot.png)

---

> 🌱 Neu hier? → [Schritt-für-Schritt-Anleitung für Einsteiger](GETTING_STARTED.md)

---

**In der Praxis:** Du bekommst einen HTML-Report mit interaktivem Graphen, der zeigt, welche Konten über welche Ketten Admin-Rechte erreichen können, ganz ohne Azure-Zugangsdaten testbar via `--dry-run`.

## Funktionen

| Funktion | Beschreibung |
|---|---|
| Vollständige Entra-ID-Abdeckung | Benutzer, Gruppen, DirectoryRoles, Apps, Service Principals, AppRoleAssignments, OAuth2PermissionGrants |
| Privilege-Chain-Erkennung | BFS bis Tiefe 6 von jedem Hochrisikoknoten, findet alle Eskalationspfade |
| Risikobewertung | Bekannte Role-Template-IDs und Graph-API-Berechtigungen → Critical / High / Medium / Low |
| Drei Exportformate | JSON, GraphML (Gephi/yEd), HTML mit interaktivem D3.js-Graphen |
| Wöchentlicher Scan | GitHub-Actions-Workflow für geplante Risikoberichte als Artefakte |
| Dry-Run-Modus | `--dry-run` nutzt internen Mock-Graphen für CI und Demos ohne Azure-Zugangsdaten |

---

## Risikolevel

| Level | Beispiele |
|---|---|
| Critical | Global Administrator, Privileged Role Administrator, Apps mit `RoleManagement.ReadWrite.Directory` |
| High | Application Administrator, User Administrator, Exchange Administrator, Apps mit `User.ReadWrite.All` |
| Medium | Alle anderen Directory-Rollen |
| Low | Benutzer, Gruppen, Apps ohne bekannte Hochrisiko-Berechtigungen |

---

## Voraussetzungen

- Rust 1.78+
- Azure App-Registrierung mit **Anwendungs**-Berechtigungen (nicht delegiert): `Directory.Read.All`, `RoleManagement.Read.Directory`, `Application.Read.All`

---

## Schnellstart

```bash
git clone https://github.com/9t29zhmwdh-coder/entra-access-graph-engine.git
cd entra-access-graph-engine
cargo build --release

# Ohne Azure-Zugangsdaten testen
./target/release/eagraph scan --dry-run --format html --output report

# Live-Scan
export AZURE_TENANT_ID=your-tenant-id
export AZURE_CLIENT_ID=your-client-id
export AZURE_CLIENT_SECRET=your-client-secret
./target/release/eagraph scan --format html --output report --min-risk high
```

---

## Deinstallation / Datenbereinigung

Lösche das `target/` Build-Verzeichnis und die generierten Report-Dateien (`report.html`, `.json`, `.graphml`). Es werden keine Zugangsdaten oder Zwischenergebnisse ausserhalb dieser Dateien gespeichert.

---

## Projektstruktur

```
crates/
  eagraph-core/src/
    graph_client.rs      Microsoft Graph API Client (OAuth2, Paginierung)
    node_builder.rs      API-Antwort zu Node/Edge-Modell + Mock-Graph
    edge_analyzer.rs     petgraph DiGraph (EntraGraph)
    chain_detector.rs    BFS Privilege-Chain-Finder (max. Tiefe 6)
    risk_scorer.rs       Rollenvorlagen-IDs und Berechtigungsbasierte Risikobewertung
    exporter.rs          JSON, GraphML, HTML + D3.js Export
    model.rs             Node, Edge, AccessGraph, PrivilegeChain, RiskReport
  eagraph-cli/src/
    main.rs              CLI-Einstiegspunkt (clap)
.github/workflows/
  ci.yml                 Ubuntu + Windows CI
  weekly-scan.yml        Wöchentlicher Scan montags 06:00 UTC
```

---

## Ausgabebeispiele

Siehe [`examples/sample_graph.json`](examples/sample_graph.json) und [`examples/sample_risk_report.json`](examples/sample_risk_report.json).

---

## Azure-Integration

Siehe [`docs/azure_integration.md`](docs/azure_integration.md) für:
- Einrichtung der App-Registrierung und benötigte Berechtigungen
- Konfiguration der GitHub-Actions-Secrets für den wöchentlichen Scan
- KQL-Abfragen für Application Insights

---

## Roadmap

Siehe [ROADMAP.md](ROADMAP.md).

---

**Autor:** [Rafael Yilmaz](https://github.com/9t29zhmwdh-coder) · **Status:** Active · ![version](https://img.shields.io/github/v/release/9t29zhmwdh-coder/entra-access-graph-engine?color=6b7280&style=flat-square) · **Lizenz:** MIT
