# Security Policy: entra-access-graph-engine

## Supported Versions

| Version | Supported |
|---------|-----------|
| Latest  | ✅ Yes    |
| Older   | ❌ No     |

Security fixes are only applied to the latest release.

## Reporting a Vulnerability

**Do NOT open a public GitHub issue for security vulnerabilities.**

Instead, report it privately via [GitHub Security Advisory](https://github.com/9t29zhmwdh-coder/entra-access-graph-engine/security/advisories/new) or contact the maintainer via the GitHub profile.

Include:
- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if any)

A response within **48 hours** is the target, and the issue will be worked on promptly.

## Credential Handling

All Azure credentials (tenant ID, client ID, client secret) are read exclusively from environment variables.
Never commit `.env` files or connection strings to version control.
Use GitHub Actions secrets for the weekly scan workflow.
The tool operates read-only against the Microsoft Graph API and never modifies any Entra ID objects.
