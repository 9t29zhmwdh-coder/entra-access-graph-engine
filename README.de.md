<div align="center">
  <img src="RayStudio.png" alt="RayStudio Logo" width="120"/>

  <h1>entra-access-graph-engine</h1>
</div>

> 🇬🇧 [English Version](README.md)

**Alle Entra-ID-Objekte in einen Zugriffsgraphen umwandeln. Eskalationspfade, versteckte Admin-Chains und Risikobewertungen erkennen. Rust, Offline-first, OTLP-ready.**

Ruft Benutzer, Gruppen, Rollen, Applikationen, Service Principals, AppRoleAssignments und DirectoryRoles ueber die Microsoft Graph API ab und erstellt daraus einen gerichteten Zugriffsgraphen. Die Engine erkennt Privilege-Escalation-Pfade, versteckte Admin-Chains (App → SP → Gruppe → GlobalAdmin) und klassifiziert jeden Knoten und jeden Pfad nach Risiko (Low / Medium / High / Critical). Export als JSON, GraphML oder selbstenthaltenem HTML-Report mit interaktivem D3.js-Graphen.

![Rust](https://img.shields.io/badge/Rust-1.78+-orange?logo=rust)
![Platform](https://img.shields.io/badge/Platform-Ubuntu%20%7C%20Windows-lightgrey?logo=linux)
![License](https://img.shields.io/badge/License-MIT-green)
[![Azure Ready](https://img.shields.io/badge/Azure-Entra%20ID%20%7C%20Graph%20API-blue?logo=microsoftazure)](docs/azure_integration.md)
[![CI](https://github.com/9t29zhmwdh-coder/entra-access-graph-engine/actions/workflows/ci.yml/badge.svg)](https://github.com/9t29zhmwdh-coder/entra-access-graph-engine/actions/workflows/ci.yml)

---

## Funktionen

| Funktion | Beschreibung |
|---|---|
| Vollstaendige Entra-ID-Abdeckung | Benutzer, Gruppen, DirectoryRoles, Apps, Service Principals, AppRoleAssignments, OAuth2PermissionGrants |
| Privilege-Chain-Erkennung | BFS bis Tiefe 6 von jedem Hochrisikoknoten, findet alle Eskalationspfade |
| Risikobewertung | Bekannte Role-Template-IDs und Graph-API-Berechtigungen → Critical / High / Medium / Low |
| Drei Exportformate | JSON, GraphML (Gephi/yEd), HTML mit interaktivem D3.js-Graphen |
| Woechentlicher Scan | GitHub-Actions-Workflow fuer geplante Risikoberichte als Artefakte |
| Dry-Run-Modus | `--dry-run` nutzt internen Mock-Graphen fuer CI und Demos ohne Azure-Zugangsdaten |

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
  weekly-scan.yml        Woechentlicher Scan montags 06:00 UTC
```

---

**Autor:** [Rafael Yilmaz](https://github.com/9t29zhmwdh-coder) · **Status:** v0.1.0 · **Zuletzt aktualisiert:** Juni 2026
