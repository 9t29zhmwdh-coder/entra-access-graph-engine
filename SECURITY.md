# Security Policy: entra-access-graph-engine

## Supported Versions

| Version | Supported |
|---|---|
| 0.1.x | Yes |

## Reporting a Vulnerability

Open a GitHub issue with the label `security`. Describe the vulnerability type and the affected component.
Do not include exploit code in public issues.

I aim to respond within 72 hours and provide a fix within 14 days for confirmed vulnerabilities.

## Credential Handling

All Azure credentials (tenant ID, client ID, client secret) are read exclusively from environment variables.
Never commit `.env` files or connection strings to version control.
Use GitHub Actions secrets for the weekly scan workflow.
The tool operates read-only against the Microsoft Graph API and never modifies any Entra ID objects.
