# Architecture

## Overview

```
┌────────────────────────────────────────────────────────────────┐
│                    eagraph-cli (binary: eagraph)                │
│                             scan                                 │
└───────────────────────────┬──────────────────────────────────┘
                            │
┌───────────────────────────▼──────────────────────────────────┐
│                         eagraph-core                            │
│                                                                  │
│  graph_client   OAuth2 client credentials flow, paginated       │
│                 Microsoft Graph reads (Users, Groups, Roles,    │
│                 Applications, Service Principals,               │
│                 AppRoleAssignments, DirectoryRoles)             │
│                                                                  │
│  node_builder   maps raw Graph objects into graph nodes         │
│  edge_analyzer  derives edges between nodes (membership,        │
│                 role assignment, app role grant, ownership)     │
│  chain_detector walks the graph for privilege escalation        │
│                 paths and hidden admin chains                   │
│                 (App → SP → Group → GlobalAdmin)                │
│  risk_scorer    classifies each node/path as                    │
│                 Low / Medium / High / Critical                  │
│  exporter       renders JSON, GraphML, or a self-contained      │
│                 HTML report with an interactive D3.js graph     │
│  model          shared domain types used across the crate       │
└───────────────────────────┬──────────────────────────────────┘
                            │ HTTPS (read-only)
┌───────────────────────────▼──────────────────────────────────┐
│                    Microsoft Graph API                          │
│  /users  /groups  /roleManagement/directory/roleAssignments     │
│  /applications  /servicePrincipals  /appRoleAssignments         │
└──────────────────────────────────────────────────────────────┘
```

## Data Flow

1. `eagraph-cli` parses the `scan` command and reads credentials from environment variables (see `.env.example`)
2. `graph_client` performs the OAuth2 client credentials flow and fetches Users, Groups, Roles, Applications, Service Principals, AppRoleAssignments and DirectoryRoles via paginated Graph API calls
3. `node_builder` maps each raw Graph object into a graph node
4. `edge_analyzer` derives edges between nodes from membership, role assignment, app role grant and ownership relationships
5. `chain_detector` walks the resulting directed graph to find privilege escalation paths and hidden admin chains
6. `risk_scorer` classifies every node and path by risk level
7. `exporter` writes the result as JSON, GraphML, or a self-contained HTML report with an interactive D3.js force graph

## Crate Responsibilities

| Crate | Responsibility |
|---|---|
| `eagraph-core` | Graph API client, graph construction, escalation-path detection, risk scoring, report export. |
| `eagraph-cli` | CLI parsing and orchestration. |

## Security Boundary

All network traffic is outbound HTTPS to Microsoft Graph endpoints only, and every call is read-only. No data is forwarded anywhere else. Reports are written to local disk only.
