<div align="center">
  <img src="../RayStudio.png" alt="RayStudio Logo" width="120"/>

  <h1>Azure Integration Guide</h1>
</div>

> 🇩🇪 [Deutsche Version](azure_integration.de.md)

## App Registration Setup

1. Navigate to **Entra ID > App registrations > New registration**
2. Name: `entra-access-graph-engine` (or similar)
3. Supported account types: **Accounts in this organizational directory only**
4. After registration, go to **API permissions > Add a permission > Microsoft Graph > Application permissions**
5. Add the following permissions:

| Permission | Reason |
|---|---|
| `Directory.Read.All` | Read users, groups, and directory objects |
| `RoleManagement.Read.Directory` | Read directory role assignments |
| `Application.Read.All` | Read application registrations and service principals |
| `AppRoleAssignment.Read.All` | Read app role assignments |

6. Click **Grant admin consent**
7. Under **Certificates & secrets**, create a new client secret

## Environment Variables

```bash
export AZURE_TENANT_ID="your-tenant-id"        # Found under Entra ID > Overview
export AZURE_CLIENT_ID="your-client-id"        # Found under App registration > Overview
export AZURE_CLIENT_SECRET="your-secret-value" # Created under Certificates & secrets
```

## GitHub Actions Weekly Scan

Add the following secrets to your fork under **Settings > Secrets > Actions**:

| Secret | Value |
|---|---|
| `AZURE_TENANT_ID` | Your Entra ID tenant ID |
| `AZURE_CLIENT_ID` | Your app registration client ID |
| `AZURE_CLIENT_SECRET` | Your client secret value |

The weekly scan runs every Monday at 06:00 UTC. You can also trigger it manually via **Actions > Weekly Risk Scan > Run workflow**.

## KQL: Query Risk Reports in Application Insights

If you forward logs to Azure Monitor, use these KQL queries:

```kql
// All critical privilege chains detected this week
customEvents
| where name == "PrivilegeChain"
| where customDimensions["risk_level"] == "critical"
| project timestamp, customDimensions["path"], customDimensions["description"]
| order by timestamp desc
```

```kql
// Risk node count over time
customMetrics
| where name in ("eagraph.critical_nodes", "eagraph.high_nodes")
| summarize avg(value) by name, bin(timestamp, 1d)
| render timechart
```

---

**Author:** [Rafael Yilmaz](https://github.com/9t29zhmwdh-coder) · **Status:** v0.1.0 · **Last Updated:** June 2026
